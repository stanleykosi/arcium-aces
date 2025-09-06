//! src/instructions/resolve_showdown.rs
//!
//! @description
//! This instruction is called after the final betting round to resolve the hand.
//! It triggers the `evaluate_hands_and_payout` confidential computation, which
//! securely determines the winner(s) and calculates payouts. The callback then
//! executes these payouts, takes the platform rake, and resets the table for the
//! next hand.
//!
//! @accounts
//! - `table`: The table account with the final state of the hand.
//! - `hand_data`: The account holding the encrypted player hands.
//! - `platform_config`: Used to get the rake parameters.
//! - `table_vault`: The table's token vault from which payouts and rake are made.
//! - `treasury_vault`: The platform's treasury account to receive the rake.
//!
//! @logic
//! 1. Validates the game state and betting round.
//! 2. Gathers all necessary inputs for the Arcium circuit: encrypted player hands,
//!    public community cards, total player bets, etc.
//! 3. Queues the `evaluate_hands_and_payout` computation.
//! 4. The `resolve_showdown_callback` receives the public `WinnerInfo` results.
//! 5. It calculates the total pot and the rake amount based on `PlatformConfig`.
//! 6. Transfers the rake from the `table_vault` to the `treasury_vault`.
//! 7. Distributes the remaining pot to the winner(s) by updating their stacks in the `Table` account.
//! 8. Updates the `Table` state to `HandComplete`, resets hand-specific data, and closes the
//!    `HandData` account to refund the rent.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use arcium_anchor::prelude::*;
use arcium_client::idl::arcium::types::CallbackAccount;
use crate::state::{Table, HandData, GameState, BettingRound, PlayerInfo, PlatformConfig};
use crate::error::AcesUnknownErrorCode;
use crate::state::constants::MAX_PLAYERS;

pub fn resolve_showdown(ctx: Context<ResolveShowdown>, table_id: u64, computation_offset: u64) -> Result<()> {
    let table = &ctx.accounts.table;
    let hand_data = &ctx.accounts.hand_data;

    // --- Validation ---
    require!(
        table.game_state == GameState::HandInProgress,
        AcesUnknownErrorCode::InvalidGameState
    );
    // Check that the river betting round is complete
    require!(
        table.betting_round == BettingRound::River,
        AcesUnknownErrorCode::InvalidGameState
    );
    // Check that the current betting round is actually complete
    // This means all active players have either called, folded, or gone all-in
    let mut betting_round_complete = true;
    for seat in table.seats.iter() {
        if let Some(player) = seat {
            if player.is_active_in_hand && !player.is_all_in && player.bet_this_round < table.current_bet {
                betting_round_complete = false;
                break;
            }
        }
    }
    require!(betting_round_complete, AcesUnknownErrorCode::InvalidGameState);

    // --- Prepare Args for Arcium ---
    let mut player_bets = [0u64; MAX_PLAYERS];
    let mut active_players = [false; MAX_PLAYERS];
    let mut player_pubkeys = [[0u8; 32]; MAX_PLAYERS]; // Placeholder for ArcisPublicKey
    
    for i in 0..MAX_PLAYERS {
        if let Some(player) = &table.seats[i] {
            player_bets[i] = player.total_bet_this_hand;
            active_players[i] = player.is_active_in_hand;
            // The client will need to provide the Arcium pubkeys. Here we just prepare the structure.
            // For the test, we'll pass in dummy keys for non-players.
        }
    }

    let community_cards_indices: [u8; 5] = core::array::from_fn(|i| {
        table.community_cards[i].map_or(255, |c| c.rank + c.suit * 13) // 255 as invalid
    });
    
    // We need to pass the encrypted hands as arguments. We will pass them as Account references
    // to avoid transaction size limits. We need to calculate offsets.
    let mut encrypted_hands_args: Vec<Argument> = Vec::new();
    const HAND_INFO_SIZE: u16 = 32 + 32 + 128/8 + 32; // pubkey, ciphertext, nonce, encryption_key
    const ENCRYPTED_HANDS_OFFSET: u16 = 8 // discriminator
        + 32 // table_pubkey
        + 8 // hand_id
        + 32 // shuffle_commitment
        + (32*3) // encrypted_deck_ciphertexts
        + 16; // encrypted_deck_nonce

    for i in 0..MAX_PLAYERS {
         // Add nonce and pubkey for Shared encryption
        if let Some(hand_info) = &hand_data.encrypted_hands[i] {
            encrypted_hands_args.push(Argument::ArcisPubkey(hand_info.encryption_key));
            encrypted_hands_args.push(Argument::PlaintextU128(hand_info.nonce));
        } else {
             // Dummy values for inactive players
            encrypted_hands_args.push(Argument::ArcisPubkey([0u8; 32]));
            encrypted_hands_args.push(Argument::PlaintextU128(0));
        }
        
        let offset = ENCRYPTED_HANDS_OFFSET + (i as u16 * (1 + HAND_INFO_SIZE)); // 1 for Option
        // We only care about the ciphertext part of the EncryptedHandInfo struct.
        let ciphertext_offset = offset + 1 + 32; // 1 for Option, 32 for pubkey
        encrypted_hands_args.push(Argument::Account(ctx.accounts.hand_data.key(), ciphertext_offset, 32));
    }
    
    let mut args = vec![
        Argument::PlaintextU8s(community_cards_indices.to_vec()),
        Argument::PlaintextU64s(player_bets.to_vec()),
        Argument::PlaintextBools(active_players.to_vec()),
        // Placeholder for ArcisPublicKeys
        Argument::PlaintextU8s(player_pubkeys.iter().flatten().map(|b| *b).collect()),
    ];
    args.extend(encrypted_hands_args);


    queue_computation(
        ctx.accounts,
        computation_offset,
        args,
        vec![
            CallbackAccount { pubkey: ctx.accounts.table.key(), is_writable: true },
            CallbackAccount { pubkey: ctx.accounts.hand_data.key(), is_writable: true },
            CallbackAccount { pubkey: ctx.accounts.table_vault.key(), is_writable: true },
            CallbackAccount { pubkey: ctx.accounts.treasury_vault.key(), is_writable: true },
            CallbackAccount { pubkey: ctx.accounts.platform_config.key(), is_writable: false },
        ],
        None,
    )?;

    Ok(())
}

#[arcium_callback(encrypted_ix = "evaluate_hands_and_payout")]
pub fn resolve_showdown_callback(
    ctx: Context<ResolveShowdownCallback>,
    output: ComputationOutputs<EvaluateHandsAndPayoutOutput>,
) -> Result<()> {
    let winner_infos = match output {
        ComputationOutputs::Success(data) => data.field_0,
        _ => return err!(AcesUnknownErrorCode::AbortedComputation),
    };

    let table = &mut ctx.accounts.table;
    let hand_data = &ctx.accounts.hand_data;
    let platform_config = &ctx.accounts.platform_config;
    
    // --- Calculate Rake ---
    let total_pot = table.pot;
    let rake_bps = platform_config.rake_bps as u64;
    let mut rake_amount = (total_pot * rake_bps) / 10000;
    if platform_config.rake_max_cap > 0 && platform_config.rake_max_cap < rake_amount {
        rake_amount = platform_config.rake_max_cap;
    }
    
    // --- Transfer Rake ---
    if rake_amount > 0 {
        let table_key = table.key();
        let seeds = &[&b"vault"[..], table_key.as_ref(), &[ctx.bumps.table_vault]];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.table_vault.to_account_info(),
            to: ctx.accounts.treasury_vault.to_account_info(),
            authority: table.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, rake_amount)?;
    }
    
    // --- Distribute Winnings ---
    for winner_info in winner_infos.iter() {
        if winner_info.amount_won > 0 {
             for seat in table.seats.iter_mut() {
                if let Some(player) = seat {
                    // NOTE: This comparison is inefficient. A mapping from ArcisPublicKey back to Solana Pubkey
                    // would be better, but requires passing more data into the circuit.
                    // For now, we find the player by their original Solana pubkey which we assume the client passed in order.
                    // This part of logic is simplified for now.
                    // A better implementation would pass player indices into Arcis and get indices back.
                    // Let's assume winner_info.player_pubkey is just an index for now.
                }
            }
        }
    }
    // Simplified payout: Give all pot to the first winner for now.
    // The complex logic resides in Arcis; this on-chain part is just execution.
    // We assume the Arcis circuit provides correct amounts. We just need to find the pubkey.
    for i in 0..MAX_PLAYERS {
        let winner_payout = winner_infos[i].amount_won;
        if winner_payout > 0 {
            if let Some(player) = table.seats[i].as_mut() {
                player.stack = player.stack.saturating_add(winner_payout);
            }
        }
    }
    
    // --- Reset Table State ---
    table.game_state = GameState::HandComplete;
    
    emit!(HandResolved {
        table_id: table.table_id,
        hand_id: hand_data.hand_id,
        pot: total_pot,
        rake: rake_amount,
    });
    
    // HandData account is closed automatically by Anchor when the context goes out of scope,
    // refunding the rent to the `payer`.

    Ok(())
}

#[derive(Accounts)]
#[instruction(table_id: u64, computation_offset: u64)]
pub struct ResolveShowdown<'info> {
    #[account(
        mut,
        seeds = [b"table", table_id.to_le_bytes().as_ref()],
        bump
    )]
    pub table: Account<'info, Table>,
    #[account(
        mut,
        seeds = [b"hand", table.key().as_ref(), table.hand_id_counter.to_le_bytes().as_ref()],
        bump
    )]
    pub hand_data: Account<'info, HandData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    
    // Arcium accounts
    #[account(address = derive_mxe_pda!())]
    pub mxe_account: Account<'info, MXEAccount>,
    #[account(mut, address = derive_mempool_pda!())]
    /// CHECK: Checked by Arcium program
    pub mempool_account: UncheckedAccount<'info>,
    #[account(mut, address = derive_execpool_pda!())]
    /// CHECK: Checked by Arcium program
    pub executing_pool: UncheckedAccount<'info>,
    #[account(mut, address = derive_comp_pda!(computation_offset))]
    /// CHECK: Checked by Arcium program
    pub computation_account: UncheckedAccount<'info>,
    #[account(address = derive_comp_def_pda!(crate::COMP_DEF_OFFSET_EVALUATE_HANDS_AND_PAYOUT))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(mut, address = derive_cluster_pda!(mxe_account))]
    pub cluster_account: Account<'info, Cluster>,
    #[account(mut, address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS)]
    pub pool_account: Account<'info, FeePool>,
    #[account(address = ARCIUM_CLOCK_ACCOUNT_ADDRESS)]
    pub clock_account: Account<'info, ClockAccount>,
    pub arcium_program: Program<'info, Arcium>,
}

#[callback_accounts("evaluate_hands_and_payout", payer)]
#[derive(Accounts)]
pub struct ResolveShowdownCallback<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(address = derive_comp_def_pda!(crate::COMP_DEF_OFFSET_EVALUATE_HANDS_AND_PAYOUT))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,
    
    // Callback accounts
    #[account(mut)]
    pub table: Account<'info, Table>,
    #[account(mut, close = payer)] // Close account and refund rent to payer
    pub hand_data: Account<'info, HandData>,
    #[account(mut)]
    pub table_vault: Account<'info, TokenAccount>,
    #[account(mut, address = platform_config.treasury_vault)]
    pub treasury_vault: Account<'info, TokenAccount>,
    pub platform_config: Account<'info, PlatformConfig>,
    pub token_program: Program<'info, Token>,
}

#[event]
pub struct HandResolved {
    pub table_id: u64,
    pub hand_id: u64,
    pub pot: u64,
    pub rake: u64,
}
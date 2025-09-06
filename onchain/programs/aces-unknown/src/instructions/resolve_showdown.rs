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
    
    for i in 0..MAX_PLAYERS {
        if let Some(player) = &table.seats[i] {
            player_bets[i] = player.total_bet_this_hand;
            active_players[i] = player.is_active_in_hand;
        }
    }

    let community_cards_indices: [u8; 5] = core::array::from_fn(|i| {
        table.community_cards[i].map_or(255, |c| c.rank + c.suit * 13) // 255 as invalid
    });
    
    // Pack player hands for Arcium
    let mut player_hands_ciphertexts = [[0u8; 32]; MAX_PLAYERS];
    let mut player_hands_nonces = [0u128; MAX_PLAYERS];
    let mut player_hands_encryption_keys = [[0u8; 32]; MAX_PLAYERS];
    
    for i in 0..MAX_PLAYERS {
        if let Some(encrypted_hand) = &hand_data.encrypted_hands[i] {
            player_hands_ciphertexts[i] = encrypted_hand.ciphertext;
            player_hands_nonces[i] = encrypted_hand.nonce;
            player_hands_encryption_keys[i] = encrypted_hand.encryption_key;
        }
    }
    
    let args = vec![
        Argument::PlaintextU64s(player_bets.to_vec()),
        Argument::PlaintextBools(active_players.to_vec()),
        Argument::PlaintextU8s(community_cards_indices.to_vec()),
        Argument::PlaintextU8s(player_hands_ciphertexts.iter().flat_map(|arr| arr.iter().cloned()).collect()),
        Argument::PlaintextU128s(player_hands_nonces.to_vec()),
        Argument::PlaintextU8s(player_hands_encryption_keys.iter().flat_map(|arr| arr.iter().cloned()).collect()),
    ];

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
    let platform_config = &ctx.accounts.platform_config;
    
    // --- Calculate Rake ---
    let total_pot = table.pot;
    let rake_bps = platform_config.rake_bps as u64;
    let mut rake_amount = (total_pot * rake_bps) / 10000;
    if platform_config.rake_max_cap > 0 {
        rake_amount = std::cmp::min(rake_amount, platform_config.rake_max_cap);
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
            // Find the winning player in the seats array
            // We use the player index from the winner info to find the correct player
            if let Some(player) = table.seats[winner_info.player_index].as_mut() {
                player.stack = player.stack.saturating_add(winner_info.amount_won);
            }
        }
    }
    
    // --- Reset Table State ---
    table.game_state = GameState::HandComplete;
    
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
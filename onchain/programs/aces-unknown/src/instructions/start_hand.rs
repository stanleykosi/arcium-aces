//! src/instructions/start_hand.rs
//!
//! @description
//! This instruction begins a new hand of poker. It is responsible for validating that
//! the game can start, collecting blinds from the appropriate players, creating a new
//! `HandData` account to store confidential information for the duration of the hand,
//! and queuing a confidential computation on Arcium to shuffle the deck and deal
//! hole cards. The corresponding callback populates the `HandData` account and
//! officially starts the first betting round.
//!
//! @accounts
//! - `table`: The poker table account where the hand is being started.
//! - `payer`: The player initiating the transaction. Any player can start a hand.
//! - `hand_data`: A new account initialized to store encrypted hand details.
//! - Arcium-related accounts for queuing the `shuffle_and_deal` computation.
//!
//! @logic
//! 1. Validates game state (`WaitingForPlayers` or `HandComplete`) and player count (>= 2).
//! 2. Rotates the dealer button to the next active player.
//! 3. Identifies the small blind (SB) and big blind (BB) positions based on standard poker rules.
//! 4. Deducts blind amounts from the SB and BB players' stacks and adds them to the pot.
//! 5. Prepares inputs for the Arcium `shuffle_and_deal` circuit, including player public keys.
//! 6. Calls `queue_computation` to start the confidential shuffle and deal process.
//! 7. The `start_hand_callback` receives the encrypted results, populates the `HandData`
//!    account, sets the game state to `HandInProgress`, and sets the turn to the first player to act.

use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;
use arcium_client::idl::arcium::types::CallbackAccount;
use arcium_client::idl::arcium::accounts::Cluster;
use arcium_client::idl::arcium::ID_CONST;
use crate::state::{Table, HandData, GameState, EncryptedHandInfo};
use crate::error::AcesUnknownErrorCode;
use crate::state::constants::MAX_PLAYERS;
use crate::ID;

/// Instruction logic for starting a new hand.
pub fn start_hand(ctx: Context<StartHand>, _table_id: u64, computation_offset: u64, arcium_pubkeys: [u8; 32]) -> Result<()> {
    // Extract keys before any mutable borrows
    let table_key = ctx.accounts.table.key();
    let hand_data_key = ctx.accounts.hand_data.key();
    
    let table = &mut ctx.accounts.table;

    // --- Validation ---
    msg!("start_hand: entering, table_id=%{}", _table_id);
    require!(
        table.game_state == GameState::WaitingForPlayers || table.game_state == GameState::HandComplete,
        AcesUnknownErrorCode::InvalidGameState
    );
    require!(
        table.player_count >= 2,
        AcesUnknownErrorCode::NotEnoughPlayers
    );

    // --- Reset Table for New Hand ---
    table.pot = 0;
    table.current_bet = 0;
    table.community_cards = [None; 5];
    table.hand_id_counter = table.hand_id_counter.checked_add(1).unwrap();
    table.last_aggressor_position = 0; // Reset for new hand

    for seat in table.seats.iter_mut() {
        if let Some(player) = seat {
            player.is_active_in_hand = true;
            player.is_all_in = false;
            player.bet_this_round = 0;
            player.total_bet_this_hand = 0;
        }
    }
    
    // --- Rotate Dealer Button ---
    let mut next_dealer_pos = (table.dealer_position + 1) % MAX_PLAYERS as u8;
    while table.seats[next_dealer_pos as usize].is_none() {
        next_dealer_pos = (next_dealer_pos + 1) % MAX_PLAYERS as u8;
    }
    table.dealer_position = next_dealer_pos;
    msg!("start_hand: dealer rotated to {}", table.dealer_position);

    // --- Identify Blinds ---
    let (sb_pos, bb_pos, first_to_act_pos) = find_blinds_and_first_actor(table)?;
    
    // --- Collect Blinds ---
    // Extract table values first to avoid borrow conflicts
    let small_blind = table.small_blind;
    let big_blind = table.big_blind;
    msg!("start_hand: blinds sb={}, bb={}", small_blind, big_blind);
    
    // Small Blind
    let sb_player = table.seats[sb_pos as usize].as_mut().unwrap();
    let sb_amount = std::cmp::min(small_blind, sb_player.stack);
    sb_player.stack -= sb_amount;
    sb_player.total_bet_this_hand += sb_amount;
    sb_player.bet_this_round += sb_amount;
    table.pot += sb_amount;

    // Big Blind
    let bb_player = table.seats[bb_pos as usize].as_mut().unwrap();
    let bb_amount = std::cmp::min(big_blind, bb_player.stack);
    bb_player.stack -= bb_amount;
    bb_player.total_bet_this_hand += bb_amount;
    bb_player.bet_this_round += bb_amount;
    table.pot += bb_amount;

    table.current_bet = big_blind;
    msg!("start_hand: blinds collected, pot={}", table.pot);
    
    // --- Queue Arcium Computation ---
    msg!("start_hand: preparing args for queue_computation, players={}", table.player_count);
    // Use a more memory-efficient approach to avoid stack overflow
    let mut args = Vec::with_capacity(32 + MAX_PLAYERS); // Pre-allocate with reasonable capacity
    
    // Add arcium pubkey as individual u8 values
    for byte in arcium_pubkeys.iter() {
        args.push(Argument::PlaintextU8(*byte));
    }
    
    // Add active players mask as individual bool values
    for i in 0..MAX_PLAYERS {
        let is_active = table.seats[i].is_some();
        args.push(Argument::PlaintextBool(is_active));
    }

    // Drop the mutable borrow before calling queue_computation
    let _ = table;
    
    msg!("start_hand: calling queue_computation with computation_offset={} ", computation_offset);
    queue_computation(
        ctx.accounts,
        computation_offset,
        args,
        vec![
            CallbackAccount { pubkey: table_key, is_writable: true },
            CallbackAccount { pubkey: hand_data_key, is_writable: true },
        ],
        None,
    )?;
    msg!("start_hand: queue_computation dispatched");

    // Re-borrow table after queue_computation
    let table = &mut ctx.accounts.table;
    table.turn_position = first_to_act_pos;
    table.turn_started_at = Clock::get()?.unix_timestamp;

    Ok(())
}

/// Helper function to find blind and first actor positions.
fn find_blinds_and_first_actor(table: &Account<Table>) -> Result<(u8, u8, u8)> {
    let mut active_indices = [0u8; MAX_PLAYERS];
    let mut num_active = 0;
    for i in 0..MAX_PLAYERS {
        if table.seats[i].is_some() {
            active_indices[num_active] = i as u8;
            num_active += 1;
        }
    }
    let dealer_idx_in_active = active_indices[..num_active].iter().position(|&p| p == table.dealer_position).unwrap();
    
    if num_active == 2 { // Heads-up case
        let sb_pos = table.dealer_position;
        let bb_pos = active_indices[(dealer_idx_in_active + 1) % num_active];
        Ok((sb_pos, bb_pos, sb_pos)) // Dealer (SB) acts first pre-flop
    } else { // 3+ players
        let sb_pos = active_indices[(dealer_idx_in_active + 1) % num_active];
        let bb_pos = active_indices[(dealer_idx_in_active + 2) % num_active];
        let first_to_act_pos = active_indices[(dealer_idx_in_active + 3) % num_active];
        Ok((sb_pos, bb_pos, first_to_act_pos))
    }
}


/// Callback logic for the `start_hand` instruction.
// #[arcium_callback(encrypted_ix = "shuffle_and_deal")]
pub fn shuffle_and_deal_callback(
    ctx: Context<StartHandCallback>,
    output: ComputationOutputs<ShuffleAndDealOutput>,
) -> Result<()> {
    let results = match output {
        ComputationOutputs::Success(data) => data.field_0,
        _ => return err!(AcesUnknownErrorCode::AbortedComputation),
    };

    let encrypted_deck = results.field_0;
    let shuffle_commitment = results.field_1;
    let encrypted_hands_from_arcium = results.field_2;

    // --- Update HandData Account ---
    let hand_data = &mut ctx.accounts.hand_data;
    hand_data.table_pubkey = ctx.accounts.table.key();
    hand_data.hand_id = ctx.accounts.table.hand_id_counter;
    hand_data.shuffle_commitment = shuffle_commitment.ciphertexts[0];
    hand_data.encrypted_deck_ciphertexts = encrypted_deck.ciphertexts;
    hand_data.encrypted_deck_nonce = encrypted_deck.nonce;
    
    // Process encrypted hands more efficiently to reduce stack usage
    for i in 0..MAX_PLAYERS {
        if let Some(player_info) = &ctx.accounts.table.seats[i] {
            let arcium_hand = &encrypted_hands_from_arcium[i];
            hand_data.encrypted_hands[i] = Some(EncryptedHandInfo {
                player: player_info.pubkey,
                ciphertext: arcium_hand.ciphertexts[0],
                nonce: arcium_hand.nonce,
                encryption_key: arcium_hand.encryption_key,
            });
        } else {
            hand_data.encrypted_hands[i] = None;
        }
    }

    // --- Update Table Account ---
    let table = &mut ctx.accounts.table;
    table.game_state = GameState::HandInProgress;
    
    // Emit event for clients
    // Event data can be large, so we may need to be selective.
    // For now, let's just confirm the hand has started.
    emit!(HandStarted {
        table_id: table.table_id,
        hand_id: table.hand_id_counter,
    });

    Ok(())
}

#[queue_computation_accounts("shuffle_and_deal", payer)]
#[derive(Accounts)]
#[instruction(table_id: u64, computation_offset: u64)]
pub struct StartHand<'info> {
    #[account(
        mut,
        seeds = [b"table", table_id.to_le_bytes().as_ref()],
        bump
    )]
    pub table: Account<'info, Table>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + HandData::INIT_SPACE,
        seeds = [b"hand", table.key().as_ref(), table.hand_id_counter.to_le_bytes().as_ref()],
        bump
    )]
    pub hand_data: Account<'info, HandData>,
    
    // Arcium accounts
    #[account(
        address = derive_mxe_pda!()
    )]
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
    #[account(address = derive_comp_def_pda!(crate::COMP_DEF_OFFSET_SHUFFLE_AND_DEAL))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(mut)]
    pub cluster_account: Account<'info, Cluster>,
    #[account(mut, address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS)]
    pub pool_account: Account<'info, FeePool>,
    #[account(address = ARCIUM_CLOCK_ACCOUNT_ADDRESS)]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

#[callback_accounts("shuffle_and_deal", payer)]
#[derive(Accounts)]
pub struct StartHandCallback<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(address = derive_comp_def_pda!(crate::COMP_DEF_OFFSET_SHUFFLE_AND_DEAL))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,
    
    // Callback accounts specified in `queue_computation`
    #[account(mut)]
    pub table: Account<'info, Table>,
    #[account(mut)]
    pub hand_data: Account<'info, HandData>,
}

#[event]
pub struct HandStarted {
    pub table_id: u64,
    pub hand_id: u64,
}
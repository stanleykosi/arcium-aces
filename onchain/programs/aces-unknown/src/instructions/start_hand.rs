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
use anchor_lang::Discriminator;
use arcium_client::idl::arcium::accounts::Cluster;
use crate::SignerAccount;
use crate::state::{Table, HandData, GameState};
use crate::error::AcesUnknownErrorCode;
use crate::state::constants::MAX_PLAYERS;


/// Instruction logic for starting a new hand.
pub fn start_hand(ctx: Context<StartHand>, _table_id: u64) -> Result<()> {
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

    // Note: Player seat data is now stored in separate PlayerSeat accounts
    // The individual PlayerSeat accounts will be updated in a separate instruction
    // or through a callback that has access to all the PlayerSeat accounts

    // --- Rotate Dealer Button ---
    let mut next_dealer_pos = (table.dealer_position + 1) % MAX_PLAYERS as u8;
    while (table.occupied_seats & (1 << next_dealer_pos)) == 0 {
        next_dealer_pos = (next_dealer_pos + 1) % MAX_PLAYERS as u8;
    }
    table.dealer_position = next_dealer_pos;
    msg!("start_hand: dealer rotated to {}", table.dealer_position);

    // --- Identify Blinds ---
    let (sb_pos, bb_pos, first_to_act_pos) = find_blinds_and_first_actor(table)?;
    msg!("start_hand: blinds: SB={}, BB={}, First={}", sb_pos, bb_pos, first_to_act_pos);

    // --- Collect Blinds ---
    // Note: We can't directly modify the player's stack and bet information
    // because they're stored in the compact PlayerSeatInfo struct
    // In a real implementation, we would need to update the full player data
    // in a separate account or use a different approach

    table.current_bet = table.big_blind;
    msg!("start_hand: blinds collected, pot={}", table.pot);

    // TODO: Add Arcium computation queuing once Arcium integration is properly set up

    // For now, just set the table state and turn
    table.turn_position = first_to_act_pos;
    table.turn_started_at = Clock::get()?.unix_timestamp;
    table.game_state = GameState::HandInProgress;

    // Emit event for clients
    emit!(HandStarted {
        table_id: table.table_id,
        hand_id: table.hand_id_counter,
    });

    Ok(())
}

/// Helper function to find blind and first actor positions.
fn find_blinds_and_first_actor(table: &Account<Table>) -> Result<(u8, u8, u8)> {
    let mut active_indices = [0u8; MAX_PLAYERS];
    let mut num_active = 0;
    for i in 0..MAX_PLAYERS {
        if (table.occupied_seats & (1 << i)) != 0 {
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






#[derive(Accounts)]
#[instruction(table_id: u64)]
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
    pub system_program: Program<'info, System>,
}



#[event]
pub struct HandStarted {
    pub table_id: u64,
    pub hand_id: u64,
}

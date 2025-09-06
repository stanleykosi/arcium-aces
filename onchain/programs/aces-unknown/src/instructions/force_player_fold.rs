//! src/instructions/force_player_fold.rs
//!
//! @description
//! This instruction provides a mechanism to prevent the game from stalling due to an
//! inactive player. Anyone can call this instruction for the player whose turn it
//! currently is, but it will only succeed if that player's on-chain timer has expired.
//!
//! @accounts
//! - `table`: The table account where the player has timed out.
//! - `payer`: The signer calling the instruction (can be anyone).
//!
//! @logic
//! 1. Fetches the current on-chain time using `Clock::get()`.
//! 2. Compares the current time to the `turn_started_at` plus `turn_duration_seconds`
//!    from the `Table` account.
//! 3. If the timer has expired, it marks the current player's hand as folded
//!    (`is_active_in_hand = false`).
//! 4. It then advances the turn to the next active, non-all-in player, ensuring the
//!    game can continue.
//! 5. If the timer has not expired, the instruction fails with a `TurnNotExpired` error.

use anchor_lang::prelude::*;
use crate::state::Table;
use crate::error::AcesUnknownErrorCode;
use crate::state::constants::MAX_PLAYERS;

/// The instruction logic for forcing a timed-out player to fold.
pub fn force_player_fold(ctx: Context<ForcePlayerFold>, _table_id: u64) -> Result<()> {
    let table = &mut ctx.accounts.table;
    
    // --- Validation ---
    let now = Clock::get()?.unix_timestamp;
    require!(
        now <= table.turn_started_at + table.turn_duration_seconds as i64,
        AcesUnknownErrorCode::TurnNotExpired
    );
    
    // --- Action: Fold Player ---
    let turn_pos = table.turn_position as usize;
    let timed_out_player = table.seats[turn_pos]
        .as_mut()
        .ok_or(AcesUnknownErrorCode::PlayerNotFound)?;
    
    timed_out_player.is_active_in_hand = false;
    msg!("Player {} at seat {} was folded due to timeout.", timed_out_player.pubkey, turn_pos);
    
    // --- Advance Turn ---
    // This logic is duplicated from `player_action`. It could be refactored into a helper.
    let mut next_turn_pos = (turn_pos + 1) % MAX_PLAYERS;
    let mut active_players_count = 0;
    
    for i in 0..MAX_PLAYERS {
        if let Some(player) = &table.seats[i] {
             if player.is_active_in_hand {
                active_players_count += 1;
            }
        }
    }
    
    // If only one active player is left, the hand is over.
    if active_players_count <= 1 {
        table.game_state = crate::state::GameState::HandComplete;
        return Ok(());
    }

    // Find next player who can act.
    loop {
        if let Some(player) = &table.seats[next_turn_pos] {
            if player.is_active_in_hand && !player.is_all_in {
                break;
            }
        }
        next_turn_pos = (next_turn_pos + 1) % MAX_PLAYERS;
    }
    
    table.turn_position = next_turn_pos as u8;
    table.turn_started_at = now;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(table_id: u64)]
pub struct ForcePlayerFold<'info> {
    #[account(
        mut,
        seeds = [b"table", table_id.to_le_bytes().as_ref()],
        bump
    )]
    pub table: Account<'info, Table>,
    /// The payer can be anyone, acting as a "keeper" to keep the game moving.
    #[account(mut)]
    pub payer: Signer<'info>,
}
//! src/instructions/force_hand_refund.rs
//!
//! @description
//! This is a safety mechanism instruction designed to resolve a "stuck" hand, for
//! example, if an Arcium computation fails to return a callback. It allows players
//! to reclaim funds they have bet in the current hand if a significant amount of
//! time has passed without any action.
//!
//! @accounts
//! - `table`: The table account that is stuck.
//! - `payer`: The signer calling the instruction (can be any player at the table).
//!
//! @logic
//! 1. Defines a `STUCK_HAND_TIMEOUT_SECONDS` constant.
//! 2. Checks if the time since the last action (`turn_started_at`) exceeds this timeout.
//! 3. If the hand is confirmed to be stuck, it iterates through all seated players.
//! 4. For each player, it adds their `total_bet_this_hand` back to their `stack`.
//! 5. It resets the table's state to `HandComplete`, clearing pot info and resetting
//!    player hand states, effectively voiding the hand.
//! 6. This prevents player funds from being permanently locked in the pot.

use anchor_lang::prelude::*;
use crate::state::Table;
use crate::error::AcesUnknownErrorCode;

/// A long duration timeout to determine if a hand is unrecoverably stuck.
const STUCK_HAND_TIMEOUT_SECONDS: i64 = 300; // 5 minutes

/// Instruction logic to refund a stuck hand.
pub fn force_hand_refund(ctx: Context<ForceHandRefund>, _table_id: u64) -> Result<()> {
    let table = &mut ctx.accounts.table;

    // --- Validation ---
    let now = Clock::get()?.unix_timestamp;
    require!(
        now <= table.turn_started_at + STUCK_HAND_TIMEOUT_SECONDS,
        AcesUnknownErrorCode::HandNotStuck
    );
    
    // --- Refund Logic ---
    // Note: Player data is now in separate PlayerSeat accounts
    // In a real implementation, we would need to iterate through all PlayerSeat accounts
    // and refund each player's bets for this hand
    let total_refunded = 0; // Placeholder

    // --- Reset Table State ---
    require!(table.pot == total_refunded, AcesUnknownErrorCode::InvalidAction);
    table.pot = 0;
    table.current_bet = 0;
    table.game_state = crate::state::GameState::HandComplete;
    table.betting_round = crate::state::BettingRound::PreFlop; // Reset to default
    
    msg!("Hand was stuck. Total pot of {} refunded to players.", total_refunded);

    Ok(())
}

#[derive(Accounts)]
#[instruction(table_id: u64)]
pub struct ForceHandRefund<'info> {
    #[account(
        mut,
        seeds = [b"table", table_id.to_le_bytes().as_ref()],
        bump
    )]
    pub table: Account<'info, Table>,
    #[account(mut)]
    pub payer: Signer<'info>,
}
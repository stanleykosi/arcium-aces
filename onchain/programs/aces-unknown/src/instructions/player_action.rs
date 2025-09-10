//! src/instructions/player_action.rs
//!
//! @description
//! This instruction is the core of player interaction during a hand. It handles all
//! standard poker actions: Fold, Check, Call, Bet, and Raise. It performs extensive
//! validation to ensure the action is legal within the game's rules and current state.
//!
//! @accounts
//! - `table`: The poker table account where the action is taking place.
//! - `player`: The signer performing the action.
//!
//! @logic
//! 1. Verifies that the game is in progress and it's the correct player's turn.
//! 2. Checks the on-chain turn timer to prevent players from taking too long.
//! 3. Based on the `PlayerAction` enum provided, it validates and executes the move:
//!    - **Fold**: Marks the player as inactive for the rest of the hand.
//!    - **Check**: Allowed only if there is no current bet to call.
//!    - **Call**: Matches the `current_bet`.
//!    - **Bet**: Makes the first bet in a round.
//!    - **Raise**: Increases the `current_bet`.
//! 4. Updates the player's stack, their bet amounts, and the table's pot.
//! 5. Determines the next player to act and updates `turn_position`. If the betting
//!    round is complete, this is handled by advancing to the next stage (e.g., dealing cards).
//! 6. If the action concludes a betting round, prepares the table for the next action
//!    (dealing community cards or resolving the showdown).

use anchor_lang::prelude::*;
use crate::state::{Table, PlayerAction, GameState, PlayerSeat};
use crate::error::AcesUnknownErrorCode;
use crate::state::constants::MAX_PLAYERS;

/// The instruction logic for a player taking an action during a betting round.
pub fn player_action(ctx: Context<PlayerActionAccounts>, _table_id: u64, action: PlayerAction) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let player_signer_key = ctx.accounts.player.key();
    let turn_pos = table.turn_position as usize;

    // --- Validation ---
    require!(
        table.game_state == GameState::HandInProgress,
        AcesUnknownErrorCode::InvalidGameState
    );
    
    // Verify the player seat belongs to the correct player and table
    let player_seat = &ctx.accounts.player_seat;
    require!(
        player_seat.player_pubkey == player_signer_key,
        AcesUnknownErrorCode::NotPlayersTurn
    );
    require!(
        player_seat.table_pubkey == table.key(),
        AcesUnknownErrorCode::PlayerNotFound
    );
    require!(
        player_seat.seat_index as usize == turn_pos,
        AcesUnknownErrorCode::NotPlayersTurn
    );
    
    // Check turn timer - time should NOT be expired
    let now = Clock::get()?.unix_timestamp;
    require!(
        now <= table.turn_started_at + table.turn_duration_seconds as i64,
        AcesUnknownErrorCode::TurnTimerExpired
    );
    
    // Extract values we need before mutable borrow
    let current_bet = table.current_bet;
    let big_blind = table.big_blind;
    let pot = table.pot;
    
    // Extract table values first to avoid borrow conflicts
    let last_aggressor_position = table.last_aggressor_position;
    
    // Now we can borrow mutably
    let current_player = &mut ctx.accounts.player_seat;
    
    // --- Action Handling ---
    let mut pot_delta = 0u64;
    let mut new_current_bet = current_bet;
    let mut new_last_aggressor = last_aggressor_position;
    
    match action {
        PlayerAction::Fold => {
            current_player.is_active_in_hand = false;
        }
        PlayerAction::Check => {
            require!(
                current_player.bet_this_round == current_bet,
                AcesUnknownErrorCode::InvalidAction
            );
        }
        PlayerAction::Call => {
            let call_amount = current_bet - current_player.bet_this_round;
            require!(call_amount > 0, AcesUnknownErrorCode::InvalidAction);
            
            let actual_call = std::cmp::min(call_amount, current_player.stack);
            current_player.stack -= actual_call;
            current_player.bet_this_round += actual_call;
            current_player.total_bet_this_hand += actual_call;
            pot_delta = actual_call;

            if current_player.stack == 0 {
                current_player.is_all_in = true;
            }
        }
        PlayerAction::Bet { amount } => {
            require!(current_bet == 0, AcesUnknownErrorCode::InvalidAction);
            require!(amount >= big_blind, AcesUnknownErrorCode::BetTooSmall);
            require!(amount <= current_player.stack, AcesUnknownErrorCode::InsufficientFunds);
            
            current_player.stack -= amount;
            current_player.bet_this_round += amount;
            current_player.total_bet_this_hand += amount;
            pot_delta = amount;
            new_current_bet = amount;
            new_last_aggressor = turn_pos as u8;

            if current_player.stack == 0 {
                current_player.is_all_in = true;
            }
        }
        PlayerAction::Raise { amount } => {
            let min_raise = current_bet * 2;
            require!(current_bet > 0, AcesUnknownErrorCode::InvalidAction);
            require!(amount >= min_raise, AcesUnknownErrorCode::BetTooSmall);
            require!(amount <= current_player.stack + current_player.bet_this_round, AcesUnknownErrorCode::InsufficientFunds);

            let amount_to_add = amount - current_player.bet_this_round;
            current_player.stack -= amount_to_add;
            current_player.bet_this_round = amount;
            current_player.total_bet_this_hand += amount_to_add;
            pot_delta = amount_to_add;
            new_current_bet = amount;
            new_last_aggressor = turn_pos as u8;

            if current_player.stack == 0 {
                current_player.is_all_in = true;
            }
        }
    }
    
    // Update table fields after releasing the borrow
    table.pot = pot + pot_delta;
    table.current_bet = new_current_bet;
    table.last_aggressor_position = new_last_aggressor;
    
    // --- Advance Turn or End Round ---
    // Check for end-of-hand conditions (e.g., only one player left)
    // Note: In a real implementation, we would need to check all PlayerSeat accounts
    // to count active players. For now, we'll use a placeholder.
    let active_players_count = 2; // Placeholder - should be calculated from PlayerSeat accounts
    if active_players_count <= 1 {
        // Hand is over, proceeds to showdown/payout
        // The frontend will call `resolve_showdown`
        table.game_state = GameState::HandComplete; // Or a specific pre-resolve state
        return Ok(());
    }

    // Find the next player
    let mut next_turn_pos = (turn_pos + 1) % MAX_PLAYERS;
    loop {
        if (table.occupied_seats & (1 << next_turn_pos)) != 0 {
            // Note: In a real implementation, we would need to check the PlayerSeat account
            // to verify the player is active in the hand
            // For now, we'll assume the player is active
            break;
        }
        next_turn_pos = (next_turn_pos + 1) % MAX_PLAYERS;
    }
    
    // Check if the betting round is over
    if next_turn_pos as u8 == table.last_aggressor_position {
        // Round is over. The next step will be triggered by a `deal_community_cards` call.
        // We can signal this by setting a specific state or just let the client logic handle it.
        // For now, we'll just stop advancing the turn. The client will see the state
        // and know to call the next instruction.
        msg!("Betting round is complete.");
    } else {
        table.turn_position = next_turn_pos as u8;
        table.turn_started_at = now;
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(table_id: u64)]
pub struct PlayerActionAccounts<'info> {
    #[account(
        mut,
        seeds = [b"table", table_id.to_le_bytes().as_ref()],
        bump
    )]
    pub table: Account<'info, Table>,
    #[account(mut)]
    pub player: Signer<'info>,
    
    /// The player's seat account.
    #[account(
        mut,
        seeds = [b"player_seat", table.key().as_ref(), player_seat.seat_index.to_le_bytes().as_ref()],
        bump = player_seat.bump,
    )]
    pub player_seat: Account<'info, PlayerSeat>,
}
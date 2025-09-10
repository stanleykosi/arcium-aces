//! src/instructions/leave_table.rs
//!
//! @description
//! This instruction allows a seated player to leave a poker table and cash out their
//! remaining chip stack. It includes safety checks to ensure a player cannot leave
//! while a hand is actively being played.
//!
//! @accounts
//! - `table`: The `Table` account the player is leaving.
//! - `player`: The signer leaving the table.
//! - `player_token_account`: The player's token account to receive the cashed-out chips.
//! - `table_vault`: The table's token vault from which the chips are transferred.
//! - `player_seat`: The player's seat account to be closed.
//!
//! @logic
//! 1. Verifies that the game is not currently in progress (`GameState::HandInProgress`).
//! 2. Retrieves the player's current chip stack from their PlayerSeat account.
//! 3. Signs with the table's PDA seeds to authorize a transfer from the `table_vault`.
//! 4. Transfers the player's stack from the `table_vault` back to their `player_token_account`.
//! 5. Closes the player's PlayerSeat account and refunds rent to the player.
//! 6. Decrements the `player_count` on the `Table` account.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::{GameState, Table, PlayerSeat};
use crate::error::AcesUnknownErrorCode;

/// The instruction logic for a player to leave a table.
pub fn leave_table(ctx: Context<LeaveTable>, table_id: u64) -> Result<()> {
    let table = &mut ctx.accounts.table;

    // --- Validation ---
    require!(
        table.game_state != GameState::HandInProgress,
        AcesUnknownErrorCode::CannotLeaveMidHand
    );

    let player_key = ctx.accounts.player.key();
    let player_seat = &ctx.accounts.player_seat;
    
    // Verify the player seat belongs to the correct player and table
    require!(
        player_seat.player_pubkey == player_key,
        AcesUnknownErrorCode::PlayerNotFound
    );
    require!(
        player_seat.table_pubkey == table.key(),
        AcesUnknownErrorCode::PlayerNotFound
    );
    
    let cash_out_amount = player_seat.stack;

    if cash_out_amount > 0 {
        // --- Token Transfer ---
        let table_key = table.key();
        let seeds = &[&b"vault"[..], table_key.as_ref(), &[ctx.bumps.table_vault]];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.table_vault.to_account_info(),
            to: ctx.accounts.player_token_account.to_account_info(),
            authority: table.to_account_info(), // The table PDA is the authority
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, cash_out_amount)?;
    }

    // --- State Update ---
    table.occupied_seats &= !(1 << player_seat.seat_index);
    table.player_count -= 1;

    // TODO: Handle dealer button and turn adjustments if the leaving player affects them.
    // This logic can be complex and depends on house rules (e.g., dead button).
    // For now, we leave it simple.

    msg!(
        "Player {} left Table #{} with {} chips.",
        player_key,
        table_id,
        cash_out_amount
    );
    Ok(())
}

/// The context struct for the `leave_table` instruction.
#[derive(Accounts)]
#[instruction(table_id: u64)]
pub struct LeaveTable<'info> {
    /// The table account being left.
    #[account(
        mut,
        seeds = [b"table", table_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub table: Account<'info, Table>,

    /// The player leaving the table.
    #[account(mut)]
    pub player: Signer<'info>,

    /// The player's token account to receive their cashed-out stack.
    #[account(
        mut,
        constraint = player_token_account.mint == table.token_mint,
        constraint = player_token_account.owner == player.key()
    )]
    pub player_token_account: Account<'info, TokenAccount>,

    /// The table's token vault.
    #[account(
        mut,
        seeds = [b"vault", table.key().as_ref()],
        bump,
    )]
    pub table_vault: Account<'info, TokenAccount>,

    /// The player's seat account to be closed.
    #[account(
        mut,
        close = player,
        seeds = [b"player_seat", table.key().as_ref(), player_seat.seat_index.to_le_bytes().as_ref()],
        bump = player_seat.bump,
    )]
    pub player_seat: Account<'info, PlayerSeat>,

    // System programs
    pub token_program: Program<'info, Token>,
}
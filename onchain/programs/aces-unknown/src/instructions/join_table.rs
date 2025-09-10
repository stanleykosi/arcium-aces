//! src/instructions/join_table.rs
//!
//! @description
//! This instruction allows a player to join an existing poker table. It handles
//! creating a new PlayerSeat account for the player, transferring the player's 
//! buy-in to the table's vault, and updating the on-chain table state.
//!
//! @accounts
//! - `table`: The `Table` account the player wishes to join.
//! - `player`: The signer joining the table.
//! - `player_token_account`: The player's token account from which the buy-in is paid.
//! - `table_vault`: The table's token vault where the buy-in is transferred.
//!
//! @logic
//! 1. Checks if the table is already full (`player_count >= MAX_PLAYERS`).
//! 2. Checks if the player is already seated at the table to prevent duplicate entries.
//! 3. Transfers the specified `buy_in` amount from the player to the table's vault.
//! 4. Creates a new `PlayerSeat` account for the player.
//! 5. Increments the `player_count` on the `Table` account.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::{Table, PlayerSeat};
use crate::error::AcesUnknownErrorCode;
use crate::state::constants::MAX_PLAYERS;

/// The instruction logic for a player to join a table.
pub fn join_table(ctx: Context<JoinTable>, table_id: u64, seat_index: u8, buy_in: u64) -> Result<()> {
    let table = &mut ctx.accounts.table;

    // --- Validation ---
    require!(
        table.player_count < MAX_PLAYERS as u8,
        AcesUnknownErrorCode::TableFull
    );
    require!(
        buy_in >= table.big_blind * 20, // Must have at least minimum buy-in
        AcesUnknownErrorCode::InsufficientBuyIn
    );
    require!(
        seat_index < MAX_PLAYERS as u8,
        AcesUnknownErrorCode::InvalidSeatIndex
    );
    require!(
        (table.occupied_seats & (1 << seat_index)) == 0,
        AcesUnknownErrorCode::SeatOccupied
    );

    let player_key = ctx.accounts.player.key();

    // --- Token Transfer ---
    let cpi_accounts = Transfer {
        from: ctx.accounts.player_token_account.to_account_info(),
        to: ctx.accounts.table_vault.to_account_info(),
        authority: ctx.accounts.player.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, buy_in)?;

    // --- Create PlayerSeat Account ---
    let player_seat = &mut ctx.accounts.player_seat;
    player_seat.table_pubkey = table.key();
    player_seat.seat_index = seat_index;
    player_seat.player_pubkey = player_key;
    player_seat.stack = buy_in;
    player_seat.is_active_in_hand = false;
    player_seat.is_all_in = false;
    player_seat.bet_this_round = 0;
    player_seat.total_bet_this_hand = 0;
    player_seat.bump = ctx.bumps.player_seat;

    // --- Update Table ---
    table.occupied_seats |= 1 << seat_index;
    table.player_count += 1;

    msg!("Player {} joined Table #{} at seat {}", player_key, table_id, seat_index);
    Ok(())
}

/// The context struct for the `join_table` instruction.
#[derive(Accounts)]
#[instruction(table_id: u64, seat_index: u8)]
pub struct JoinTable<'info> {
    /// The table account to be joined.
    #[account(
        mut,
        seeds = [b"table", table_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub table: Account<'info, Table>,

    /// The player joining the table.
    #[account(mut)]
    pub player: Signer<'info>,

    /// The player's token account for the table's currency.
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

    /// The player's seat account to be created.
    #[account(
        init,
        payer = player,
        space = 8 + PlayerSeat::INIT_SPACE,
        seeds = [b"player_seat", table.key().as_ref(), seat_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub player_seat: Account<'info, PlayerSeat>,

    // System programs
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
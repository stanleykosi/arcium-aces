//! src/instructions/join_table.rs
//!
//! @description
//! This instruction allows a player to join an existing poker table. It handles
//! finding an empty seat, transferring the player's buy-in to the table's vault,
//! and updating the on-chain table state to reflect the new player.
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
//! 3. Finds the first available empty seat (`None`) in the `seats` array.
//! 4. Transfers the specified `buy_in` amount from the player to the table's vault.
//! 5. Creates a new `PlayerInfo` struct and places it in the empty seat.
//! 6. Increments the `player_count` on the `Table` account.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::{PlayerInfo, Table};
use crate::error::AcesUnknownErrorCode;
use crate::state::constants::MAX_PLAYERS;

/// The instruction logic for a player to join a table.
pub fn join_table(ctx: Context<JoinTable>, _table_id: u64, buy_in: u64) -> Result<()> {
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

    let player_key = ctx.accounts.player.key();
    let mut empty_seat_index: Option<usize> = None;

    // Check if player is already seated and find an empty seat
    for (i, seat) in table.seats.iter().enumerate() {
        if let Some(player_info) = seat {
            require!(
                player_info.pubkey != player_key,
                AcesUnknownErrorCode::AlreadySeated
            );
        } else if empty_seat_index.is_none() {
            empty_seat_index = Some(i);
        }
    }
    
    let seat_idx = empty_seat_index.unwrap(); // Should always find a seat due to player_count check

    // --- Token Transfer ---
    let cpi_accounts = Transfer {
        from: ctx.accounts.player_token_account.to_account_info(),
        to: ctx.accounts.table_vault.to_account_info(),
        authority: ctx.accounts.player.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, buy_in)?;

    // --- State Update ---
    let new_player = PlayerInfo {
        pubkey: player_key,
        stack: buy_in,
        is_active_in_hand: false,
        is_all_in: false,
        bet_this_round: 0,
        total_bet_this_hand: 0,
    };
    table.seats[seat_idx] = Some(new_player);
    table.player_count += 1;

    msg!("Player {} joined Table #{}", player_key, _table_id);
    Ok(())
}

/// The context struct for the `join_table` instruction.
#[derive(Accounts)]
#[instruction(table_id: u64)]
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

    // System programs
    pub token_program: Program<'info, Token>,
}
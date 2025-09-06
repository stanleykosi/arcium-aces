//! src/instructions/create_table.rs
//!
//! @description
//! This instruction allows a player to create a new poker table. It initializes a `Table`
//! account with the specified parameters (blinds, token type) and a dedicated SPL token
//! vault to hold all chips for that table. The creator is automatically seated and
//! their initial buy-in is transferred to the vault.
//!
//! @accounts
//! - `table`: The new `Table` account, initialized via PDA.
//! - `creator`: The player creating the table, who pays for the account initialization.
//! - `token_mint`: The SPL token mint to be used for this table's currency.
//! - `creator_token_account`: The creator's token account from which the buy-in is paid.
//! - `table_vault`: A new token account (PDA) that will hold all player chips for this table.
//!
//! @logic
//! 1. Validates that the big blind is greater than the small blind.
//! 2. Validates that the initial buy-in meets a minimum requirement (e.g., 20 big blinds).
//! 3. Initializes the `Table` account with game parameters.
//! 4. Initializes the `table_vault` token account, with the table PDA as its authority.
//! 5. Transfers the `buy_in` amount from the creator's token account to the `table_vault`.
//! 6. Creates a `PlayerInfo` struct for the creator and adds them to the first seat.
//! 7. Sets the game state to `WaitingForPlayers`.

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
use crate::state::{BettingRound, GameState, PlayerInfo, Table, PlatformConfig};
use crate::error::AcesUnknownErrorCode;
use crate::state::constants::MAX_PLAYERS;

/// The instruction logic for creating a new poker table.
pub fn create_table(
    ctx: Context<CreateTable>,
    table_id: u64,
    small_blind: u64,
    big_blind: u64,
    buy_in: u64,
) -> Result<()> {
    // --- Validation ---
    require!(big_blind > small_blind, AcesUnknownErrorCode::InvalidStakes);
    // A common rule is a minimum buy-in of 20 big blinds.
    require!(buy_in >= big_blind * 20, AcesUnknownErrorCode::InsufficientBuyIn);

    // --- Token Transfer ---
    let cpi_accounts = Transfer {
        from: ctx.accounts.creator_token_account.to_account_info(),
        to: ctx.accounts.table_vault.to_account_info(),
        authority: ctx.accounts.creator.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, buy_in)?;

    // --- State Initialization ---
    let table = &mut ctx.accounts.table;
    table.table_id = table_id;
    table.creator = ctx.accounts.creator.key();
    table.admin = ctx.accounts.platform_config.admin;
    table.game_state = GameState::WaitingForPlayers;
    table.betting_round = BettingRound::PreFlop; // Default state
    table.small_blind = small_blind;
    table.big_blind = big_blind;
    table.token_mint = ctx.accounts.token_mint.key();
    table.turn_duration_seconds = 30; // Default turn duration

    // Seat the creator at the first position
    let mut seats = Vec::with_capacity(MAX_PLAYERS);
    let creator_info = PlayerInfo {
        pubkey: ctx.accounts.creator.key(),
        stack: buy_in,
        is_active_in_hand: false,
        is_all_in: false,
        bet_this_round: 0,
        total_bet_this_hand: 0,
    };
    seats.push(Some(creator_info));
    // Fill the rest of the seats with None
    for _ in 1..MAX_PLAYERS {
        seats.push(None);
    }
    table.seats = seats;

    table.player_count = 1;
    table.dealer_position = 0; // Creator starts as the dealer
    table.turn_position = 0;

    msg!("Table #{} created by {}", table_id, table.creator);
    Ok(())
}

/// The context struct for the `create_table` instruction.
#[derive(Accounts)]
#[instruction(table_id: u64)]
pub struct CreateTable<'info> {
    /// The new table account being created.
    /// It's a PDA seeded with "table" and the `table_id`.
    #[account(
        init,
        payer = creator,
        space = 8 + Table::INIT_SPACE,
        seeds = [b"table", table_id.to_le_bytes().as_ref()],
        bump
    )]
    pub table: Account<'info, Table>,

    /// The creator of the table, who also pays for the account initialization.
    #[account(mut)]
    pub creator: Signer<'info>,

    /// The global platform configuration account.
    pub platform_config: Account<'info, PlatformConfig>,
    
    /// The SPL token mint for the table's currency.
    pub token_mint: Account<'info, Mint>,

    /// The creator's token account for the specified mint.
    #[account(
        mut,
        constraint = creator_token_account.mint == token_mint.key()
    )]
    pub creator_token_account: Account<'info, TokenAccount>,
    
    /// The table's token vault, a PDA to hold all player chips.
    /// The authority is the table account itself, ensuring program-controlled transfers.
    #[account(
        init,
        payer = creator,
        token::mint = token_mint,
        token::authority = table,
        seeds = [b"vault", table.key().as_ref()],
        bump,
    )]
    pub table_vault: Account<'info, TokenAccount>,

    // System programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
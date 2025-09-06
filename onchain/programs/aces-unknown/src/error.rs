//! src/error.rs
//!
//! @description
//! This module defines the custom error codes for the Aces Unknown on-chain program.
//! Using custom errors with descriptive messages is crucial for a good developer
//! and user experience, as it allows clients to understand exactly why an
//! instruction failed.
//!
//! @dependencies
//! - `anchor_lang`: Provides the `error_code` macro for defining custom errors.

use anchor_lang::prelude::*;

#[error_code]
pub enum AcesUnknownErrorCode {
    // ========================================
    // Admin & Table Management Errors
    // ========================================
    #[msg("Unauthorized: Signer is not the platform admin.")]
    Unauthorized,

    #[msg("Invalid Stakes: Big blind must be greater than small blind.")]
    InvalidStakes,

    #[msg("Invalid Buy-in: Buy-in amount is insufficient.")]
    InsufficientBuyIn,

    #[msg("Invalid Token Mint: The provided token mint is not supported or invalid.")]
    InvalidTokenMint,

    #[msg("Table is full. Cannot join.")]
    TableFull,

    #[msg("Player is already seated at this table.")]
    AlreadySeated,

    #[msg("Player not found at this table.")]
    PlayerNotFound,

    #[msg("Cannot leave the table while a hand is in progress.")]
    CannotLeaveMidHand,

    // ========================================
    // Gameplay Errors
    // ========================================
    #[msg("Invalid Game State: The action is not valid in the current game state.")]
    InvalidGameState,

    #[msg("Cannot start hand. The game is already in progress.")]
    CannotStartHand,

    #[msg("Not enough players to start a hand.")]
    NotEnoughPlayers,

    #[msg("It is not this player's turn to act.")]
    NotPlayersTurn,

    #[msg("Turn timer has expired.")]
    TurnTimerExpired,

    #[msg("Invalid Action: The attempted move is not allowed.")]
    InvalidAction,

    #[msg("Invalid Bet Amount: The bet or raise amount is not valid.")]
    InvalidBetAmount,

    #[msg("Bet is too small. Must be at least the minimum raise.")]
    BetTooSmall,

    #[msg("Insufficient funds to perform this action.")]
    InsufficientFunds,

    // ========================================
    // Arcium & Computation Errors
    // ========================================
    #[msg("The Arcium computation was aborted or failed.")]
    AbortedComputation,

    #[msg("The Arcium cluster is not set.")]
    ClusterNotSet,

    #[msg("Arcium computation timed out.")]
    ComputationTimeout,
}
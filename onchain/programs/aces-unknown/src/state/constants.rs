//! src/state/constants.rs
//!
//! @description
//! This module defines shared constants used across the on-chain program.
//! Centralizing these constants makes the program easier to configure and maintain.
//!
//! Key Constants:
//! - MAX_PLAYERS: The maximum number of players allowed at a single poker table.
//!                This is set to 6 for "6-max" No-Limit Texas Hold'em games.

// The maximum number of players allowed at a poker table.
pub const MAX_PLAYERS: usize = 6;
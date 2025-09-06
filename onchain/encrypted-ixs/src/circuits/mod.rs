//! src/circuits/mod.rs
//!
//! @description
//! This module serves as the entry point for all Arcis circuit definitions.
//! It declares the individual circuit files as submodules, organizing the
//! confidential logic of the application.
//!
//! @modules
//! - `shuffle_and_deal`: Contains the circuit for securely shuffling the deck
//!   and dealing encrypted hole cards to players.
//! - `reveal_community_cards`: Contains the circuit for revealing the flop, turn, and river.
//! - `evaluate_hands_and_payout`: Contains the circuit for resolving the showdown,
//!   evaluating hands, and calculating pot distribution.

pub mod shuffle_and_deal;
pub mod reveal_community_cards;
pub mod evaluate_hands_and_payout;
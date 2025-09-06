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

pub mod shuffle_and_deal;
//! src/lib.rs
//!
//! @description
//! This is the main library entry point for the `encrypted-ixs` crate, which contains
//! all the confidential game logic for the Aces Unknown poker platform. All code
//! within this crate is designed to be compiled into MPC circuits and executed on
//! the Arcium network.
//!
//! The library is structured into modules:
//! - `types`: Defines the core data structures for cards, decks, and hands.
//! - `circuits`: Contains the actual `#[instruction]` definitions for confidential computations.
//!
//! @dependencies
//! - `arcis_imports`: Provides all necessary macros, types, and functions for Arcis development.

// Import all necessary items from the Arcis framework.
use arcis_imports::*;

// Make the types module public so its contents can be used by other modules in this crate.
pub mod types;

// Define the `circuits` module, which will contain all the encrypted instructions.
// The `#[encrypted]` attribute tells the Arcis compiler to process this module.
#[encrypted]
mod circuits {
    // Import Arcis framework items and our custom types into the circuit module's scope.
    use arcis_imports::*;
    use crate::types::*;

    // Encrypted instructions for game logic (e.g., shuffle_and_deal, reveal_community_cards)
    // will be added here in subsequent implementation steps.
}
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
//! - `logic`: Contains reusable business logic for complex tasks like hand evaluation.
//! - `circuits`: A directory containing the actual `#[instruction]` definitions for
//!   confidential computations.
//!
//! @dependencies
//! - `arcis_imports`: Provides all necessary macros, types, and functions for Arcis development.

// Import all necessary items from the Arcis framework.
use arcis_imports::*;

// Make the types, logic, and circuits modules public so their contents can be used by the program.
pub mod types;
pub mod logic;
pub mod circuits;

// The `#[encrypted]` attribute tells the Arcis compiler to process this module and
// compile all public functions within it into MPC circuits.
#[encrypted]
mod confidential_instructions {
    // Import Arcis framework items and our custom types/logic into this module's scope.
    use arcis_imports::*;
    use crate::types::*;
    use crate::logic::*;

    // By using `use`, we bring the instructions into this `#[encrypted]`
    // module, making them visible to the Arcis compiler. All future circuits will be
    // imported and exposed here.
    use crate::circuits::shuffle_and_deal::shuffle_and_deal;
    use crate::circuits::reveal_community_cards::reveal_community_cards;
    use crate::circuits::evaluate_hands_and_payout::evaluate_hands_and_payout;
}
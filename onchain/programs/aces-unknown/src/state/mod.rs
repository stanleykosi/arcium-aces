//! src/state/mod.rs
//!
//! @description
//! This module serves as the central hub for all on-chain account state definitions.
//! It aggregates and exports the various structs that define the structure of
//! the program's accounts, such as `PlatformConfig`, `Table`, and `HandData`.
//!
//! By organizing state definitions here, we maintain a clean and modular codebase.

// Declare each state file as a public submodule.
pub mod platform_config;
pub mod table;
pub mod hand_data;
pub mod card;
pub mod constants;

// Re-export the contents of each submodule for easy access from other parts of the program.
pub use platform_config::*;
pub use table::*;
pub use hand_data::*;
pub use card::*;
pub use constants::*;
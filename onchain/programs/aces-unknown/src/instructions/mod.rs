//! src/instructions/mod.rs
//!
//! @description
//! This module acts as the central aggregator for all instruction-specific modules
//! in the Aces Unknown on-chain program. It follows the barrel pattern, exporting
//! the contents of each submodule. This organization keeps the main `lib.rs` file
//! clean and makes the instruction logic modular and easier to navigate.
//!
//! Each submodule corresponds to a single on-chain instruction and contains the
//! instruction logic function as well as its associated `Context` struct.

// Platform and table management instructions
pub mod create_table;
pub mod join_table;
pub mod leave_table;
pub mod update_rake_params;

// Hand lifecycle instructions
pub mod start_hand;
pub mod deal_community_cards;
pub mod resolve_showdown;

// Re-export all public items from the submodules.
pub use create_table::*;
pub use join_table::*;
pub use leave_table::*;
pub use update_rake_params::*;
pub use start_hand::*;
pub use deal_community_cards::*;
pub use resolve_showdown::*;
//! src/lib.rs
//!
//! @description
//! This is the main entry point for the Aces Unknown on-chain program.
//! It defines the program's instructions, state accounts, and custom errors.
//! The program orchestrates the public aspects of the poker game on Solana,
//! such as managing tables and player actions, while delegating all confidential
//! logic (card shuffling, dealing, showdowns) to the Arcium network.
//!
//! The program is built using the Anchor framework for Solana and the Arcis
//! framework for confidential computations on Arcium.

use anchor_lang::prelude::*;

// Import the state module, which contains all account struct definitions.
pub mod state;
use state::*;

declare_id!("ACESUnKnOwn111111111111111111111111111111111");

#[program]
pub mod aces_unknown {
    use super::*;

    /// Placeholder instruction to initialize the program.
    /// This will be replaced with specific instructions for platform setup,
    /// table creation, and game actions in subsequent implementation steps.
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

/// Context for the placeholder `initialize` instruction.
#[derive(Accounts)]
pub struct Initialize {}
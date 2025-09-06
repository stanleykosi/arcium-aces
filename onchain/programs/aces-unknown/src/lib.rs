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

// Import local modules.
pub mod state;
pub mod error;
pub mod instructions;

// Make their contents available for the program.
use state::*;
use error::*;
use instructions::*;

declare_id!("ACESUnKnOwn111111111111111111111111111111111");

#[program]
pub mod aces_unknown {
    use super::*;

    /// Initializes a `PlatformConfig` singleton account with the deployer as the admin.
    /// This should be called once after the program is deployed.
    pub fn initialize_platform_config(ctx: Context<InitializePlatformConfig>) -> Result<()> {
        ctx.accounts.platform_config.admin = ctx.accounts.admin.key();
        // Set default rake: 5% with a cap of 3 Big Blinds (example, can be updated)
        ctx.accounts.platform_config.rake_bps = 500; // 5.00%
        // Cap is set later based on table currency, this is a placeholder
        ctx.accounts.platform_config.rake_max_cap = 0;
        Ok(())
    }

    /// Instruction for the platform admin to update rake parameters.
    pub fn update_rake_params(
        ctx: Context<UpdateRakeParams>,
        new_rake_bps: u16,
        new_rake_max_cap: u64,
    ) -> Result<()> {
        instructions::update_rake_params(ctx, new_rake_bps, new_rake_max_cap)
    }

    /// Instruction for a player to create a new poker table.
    pub fn create_table(
        ctx: Context<CreateTable>,
        table_id: u64,
        small_blind: u64,
        big_blind: u64,
        buy_in: u64,
    ) -> Result<()> {
        instructions::create_table(ctx, table_id, small_blind, big_blind, buy_in)
    }

    /// Instruction for a player to join an existing table.
    pub fn join_table(ctx: Context<JoinTable>, table_id: u64, buy_in: u64) -> Result<()> {
        instructions::join_table(ctx, table_id, buy_in)
    }

    /// Instruction for a player to leave a table and cash out their chips.
    pub fn leave_table(ctx: Context<LeaveTable>, table_id: u64) -> Result<()> {
        instructions::leave_table(ctx, table_id)
    }
}

/// Context for initializing the `PlatformConfig` account.
#[derive(Accounts)]
pub struct InitializePlatformConfig<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + PlatformConfig::INIT_SPACE,
        seeds = [b"platform_config"],
        bump
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
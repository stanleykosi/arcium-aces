//! src/instructions/update_rake_params.rs
//!
//! @description
//! This instruction allows the platform administrator to update the rake parameters
//! stored in the `PlatformConfig` singleton account. This is a critical administrative
//! function that enables tuning the platform's economy without requiring a full
//! program redeployment.
//!
//! @security
//! This instruction is secured by an `address` constraint on the `admin` account,
//! ensuring that only the wallet public key stored in `platform_config.admin` can
//! sign and successfully execute this transaction.

use anchor_lang::prelude::*;
use crate::state::PlatformConfig;
use crate::error::AcesUnknownErrorCode;

/// The instruction logic for updating platform rake parameters.
///
/// It validates the input and updates the `rake_bps` and `rake_max_cap` fields
/// in the `PlatformConfig` account.
///
/// # Arguments
/// * `ctx` - The context containing the required accounts.
/// * `new_rake_bps` - The new rake percentage in basis points (e.g., 500 for 5%).
/// * `new_rake_max_cap` - The new maximum rake amount in the smallest token denomination.
pub fn update_rake_params(
    ctx: Context<UpdateRakeParams>,
    new_rake_bps: u16,
    new_rake_max_cap: u64,
) -> Result<()> {
    // Input validation: A rake of 100% (10000 bps) or more is nonsensical.
    require!(new_rake_bps <= 10000, AcesUnknownErrorCode::InvalidAction);

    let platform_config = &mut ctx.accounts.platform_config;
    platform_config.rake_bps = new_rake_bps;
    platform_config.rake_max_cap = new_rake_max_cap;

    msg!(
        "Rake parameters updated: new_rake_bps = {}, new_rake_max_cap = {}",
        new_rake_bps,
        new_rake_max_cap
    );

    Ok(())
}

/// The context struct for the `update_rake_params` instruction.
///
/// It defines the accounts required for this operation and enforces security constraints.
#[derive(Accounts)]
pub struct UpdateRakeParams<'info> {
    /// The platform configuration account to be modified.
    #[account(mut)]
    pub platform_config: Account<'info, PlatformConfig>,

    /// The administrator's signer account.
    /// The `address` constraint ensures that the signer's public key matches the
    /// `admin` field stored within the `platform_config` account, providing
    /// robust authorization for this sensitive action.
    #[account(address = platform_config.admin)]
    pub admin: Signer<'info>,
}
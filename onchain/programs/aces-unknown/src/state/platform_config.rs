//! src/state/platform_config.rs
//!
//! @description
//! Defines the `PlatformConfig` account, a singleton that holds global configuration
//! for the Aces Unknown platform. This allows for administrative control over key
//! parameters like rake percentages without requiring a program redeploy.
//!
//! Key features:
//! - Stores the administrative authority wallet.
//! - Defines configurable rake parameters (basis points and max cap).

use anchor_lang::prelude::*;

/// A singleton account that stores global platform settings.
/// This account is controlled by an administrative key.
#[account]
#[derive(InitSpace)]
pub struct PlatformConfig {
    /// The public key of the administrator wallet.
    /// This wallet has the authority to update platform-wide settings, such as rake.
    pub admin: Pubkey,

    /// The rake percentage, expressed in basis points (bps).
    /// For example, 500 bps represents a 5% rake.
    /// 1 basis point = 0.01%.
    pub rake_bps: u16,

    /// The maximum amount of rake that can be taken from a single pot,
    /// expressed in the smallest denomination of the table's currency (e.g., lamports for SOL).
    pub rake_max_cap: u64,
}
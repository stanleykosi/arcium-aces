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
use arcium_anchor::prelude::*;
use arcium_anchor::prelude::comp_def_offset;

// Import local modules.
pub mod state;
pub mod error;
pub mod instructions;

// Make their contents available for the program.
use state::*;
use instructions::*;

// Arcium Computation Definition Offsets
// These constants are unique identifiers for each confidential instruction.
const COMP_DEF_OFFSET_SHUFFLE_AND_DEAL: u32 = comp_def_offset("shuffle_and_deal");
const COMP_DEF_OFFSET_REVEAL_COMMUNITY_CARDS: u32 = comp_def_offset("reveal_community_cards");
const COMP_DEF_OFFSET_EVALUATE_HANDS_AND_PAYOUT: u32 = comp_def_offset("evaluate_hands_and_payout");

// Program ID
declare_id!("8aftkGgLGF2LWDPbvzdJSYwFPoYCxdhk25HAwMAopygZ");

#[arcium_program]
pub mod aces_unknown {
    use super::*;

    // ========================================
    // Admin & Table Management Instructions
    // ========================================

    /// Initializes a `PlatformConfig` singleton account with the deployer as the admin.
    /// This should be called once after the program is deployed.
    pub fn initialize_platform_config(ctx: Context<InitializePlatformConfig>) -> Result<()> {
        ctx.accounts.platform_config.admin = ctx.accounts.admin.key();
        ctx.accounts.platform_config.rake_bps = 500; // Default 5.00%
        ctx.accounts.platform_config.rake_max_cap = 0; // Default no cap
        ctx.accounts.platform_config.treasury_vault = ctx.accounts.treasury_vault.key();
        Ok(())
    }

    /// Instruction for the platform admin to update rake parameters.
    pub fn update_rake_params(
        ctx: Context<UpdateRakeParams>,
        new_rake_bps: u16,
        new_rake_max_cap: u64,
    ) -> Result<()> {
        instructions::update_rake_params::update_rake_params(ctx, new_rake_bps, new_rake_max_cap)
    }

    /// Instruction for a player to create a new poker table.
    pub fn create_table(
        ctx: Context<CreateTable>,
        table_id: u64,
        small_blind: u64,
        big_blind: u64,
        buy_in: u64,
    ) -> Result<()> {
        instructions::create_table::create_table(ctx, table_id, small_blind, big_blind, buy_in)
    }

    /// Instruction for a player to join an existing table.
    pub fn join_table(ctx: Context<JoinTable>, table_id: u64, buy_in: u64) -> Result<()> {
        instructions::join_table::join_table(ctx, table_id, buy_in)
    }

    /// Instruction for a player to leave a table and cash out their chips.
    pub fn leave_table(ctx: Context<LeaveTable>, table_id: u64) -> Result<()> {
        instructions::leave_table::leave_table(ctx, table_id)
    }

    // ========================================
    // Hand Lifecycle Instructions
    // ========================================

    /// Starts a new hand, collects blinds, and queues the shuffle/deal computation.
    pub fn start_hand(ctx: Context<StartHand>, table_id: u64, computation_offset: u64, arcium_pubkeys: [u8; 32]) -> Result<()> {
        instructions::start_hand::start_hand(ctx, table_id, computation_offset, arcium_pubkeys)
    }

    /// Reveals the next community cards (flop, turn, or river).
    pub fn deal_community_cards(ctx: Context<DealCommunityCards>, table_id: u64, computation_offset: u64) -> Result<()> {
        instructions::deal_community_cards::deal_community_cards(ctx, table_id, computation_offset)
    }

    /// Resolves the showdown, determines the winner, and handles payouts.
    pub fn resolve_showdown(ctx: Context<ResolveShowdown>, table_id: u64, computation_offset: u64) -> Result<()> {
        instructions::resolve_showdown::resolve_showdown(ctx, table_id, computation_offset)
    }
    
    // ========================================
    // Player Action & Timeout Instructions
    // ========================================
    
    /// The main instruction for a player to take an action (fold, check, call, bet, raise).
    pub fn player_action(ctx: Context<PlayerActionAccounts>, table_id: u64, action: crate::state::PlayerAction) -> Result<()> {
        instructions::player_action::player_action(ctx, table_id, action)
    }

    /// Instruction for anyone to fold a player whose turn timer has expired.
    pub fn force_player_fold(ctx: Context<ForcePlayerFold>, table_id: u64) -> Result<()> {
        instructions::force_player_fold::force_player_fold(ctx, table_id)
    }

    /// Safety instruction to refund all bets if a hand becomes unrecoverably stuck.
    pub fn force_hand_refund(ctx: Context<ForceHandRefund>, table_id: u64) -> Result<()> {
        instructions::force_hand_refund::force_hand_refund(ctx, table_id)
    }


    // ========================================
    // Arcium Callbacks
    // ========================================

    /// Callback for the `start_hand` instruction's `shuffle_and_deal` computation.
    // #[arcium_callback(encrypted_ix = "shuffle_and_deal")]
    pub fn shuffle_and_deal_callback(
        ctx: Context<StartHandCallback>,
        output: ComputationOutputs<ShuffleAndDealOutput>,
    ) -> Result<()> {
        instructions::shuffle_and_deal_callback(ctx, output)
    }

    /// Callback for the `deal_community_cards` instruction's `reveal_community_cards` computation.
    // #[arcium_callback(encrypted_ix = "reveal_community_cards")]
    pub fn reveal_community_cards_callback(
        ctx: Context<DealCommunityCardsCallback>,
        output: ComputationOutputs<RevealCommunityCardsOutput>,
    ) -> Result<()> {
        instructions::reveal_community_cards_callback(ctx, output)
    }

    /// Callback for the `resolve_showdown` instruction's `evaluate_hands_and_payout` computation.
    // #[arcium_callback(encrypted_ix = "evaluate_hands_and_payout")]
    pub fn evaluate_hands_and_payout_callback(
        ctx: Context<ResolveShowdownCallback>,
        output: ComputationOutputs<EvaluateHandsAndPayoutOutput>,
    ) -> Result<()> {
        instructions::evaluate_hands_and_payout_callback(ctx, output)
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
    /// CHECK: Treasury vault account for platform rake collection
    pub treasury_vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
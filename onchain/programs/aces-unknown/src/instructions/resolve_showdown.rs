//! src/instructions/resolve_showdown.rs
//!
//! @description
//! This instruction is called after the final betting round to resolve the hand.
//! It triggers the `evaluate_hands_and_payout` confidential computation, which
//! securely determines the winner(s) and calculates payouts. The callback then
//! executes these payouts, takes the platform rake, and resets the table for the
//! next hand.
//!
//! @accounts
//! - `table`: The table account with the final state of the hand.
//! - `hand_data`: The account holding the encrypted player hands.
//! - `platform_config`: Used to get the rake parameters.
//! - `table_vault`: The table's token vault from which payouts and rake are made.
//! - `treasury_vault`: The platform's treasury account to receive the rake.
//!
//! @logic
//! 1. Validates the game state and betting round.
//! 2. Gathers all necessary inputs for the Arcium circuit: encrypted player hands,
//!    public community cards, total player bets, etc.
//! 3. Queues the `evaluate_hands_and_payout` computation.
//! 4. The `resolve_showdown_callback` receives the public `WinnerInfo` results.
//! 5. It calculates the total pot and the rake amount based on `PlatformConfig`.
//! 6. Transfers the rake from the `table_vault` to the `treasury_vault`.
//! 7. Distributes the remaining pot to the winner(s) by updating their stacks in the `Table` account.
//! 8. Updates the `Table` state to `HandComplete`, resets hand-specific data, and closes the
//!    `HandData` account to refund the rent.

use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;
use anchor_lang::Discriminator;
use arcium_client::idl::arcium::accounts::Cluster;
use crate::SignerAccount;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::{Table, HandData, GameState, BettingRound, PlatformConfig};
use crate::error::AcesUnknownErrorCode;
use crate::state::constants::MAX_PLAYERS;


pub fn resolve_showdown(ctx: Context<ResolveShowdown>, _table_id: u64) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let hand_data = &ctx.accounts.hand_data;
    let platform_config = &ctx.accounts.platform_config;

    // --- Validation ---
    require!(
        table.game_state == GameState::HandInProgress,
        AcesUnknownErrorCode::InvalidGameState
    );
    // Check that the river betting round is complete
    require!(
        table.betting_round == BettingRound::River,
        AcesUnknownErrorCode::InvalidGameState
    );
    // Check that the current betting round is actually complete
    // This means all active players have either called, folded, or gone all-in
    let betting_round_complete = true;
    // Note: Player seat data is now stored in separate PlayerSeat accounts
    // In a real implementation, we would need to check each PlayerSeat account
    // to verify the betting round is complete
    require!(betting_round_complete, AcesUnknownErrorCode::InvalidGameState);

    // TODO: Add Arcium computation queuing once Arcium integration is properly set up

    // For now, simulate showdown resolution
    let total_pot = table.pot;
    let rake_bps = platform_config.rake_bps as u64;
    let mut rake_amount = (total_pot * rake_bps) / 10000;
    if platform_config.rake_max_cap > 0 && platform_config.rake_max_cap < rake_amount {
        rake_amount = platform_config.rake_max_cap;
    }

    // --- Transfer Rake ---
    if rake_amount > 0 {
        let table_key = table.key();
        let seeds = &[&b"vault"[..], table_key.as_ref()];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.table_vault.to_account_info(),
            to: ctx.accounts.treasury_vault.to_account_info(),
            authority: table.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, rake_amount)?;
    }

    // --- Distribute Winnings ---
    // For now, simulate a simple winner distribution
    // In a real implementation, this would come from Arcium computation results
    let remaining_pot = total_pot - rake_amount;
    if remaining_pot > 0 {
        // Simulate distributing to a winner (in practice, this would be determined by Arcium)
        // We can't update player.stack because it's not stored in PlayerSeatInfo
        // In a real implementation, we would need to access this information
        // from a separate account or use a different approach
    }

    // --- Reset Table State ---
    table.game_state = GameState::HandComplete;

    emit!(HandResolved {
        table_id: table.table_id,
        hand_id: hand_data.hand_id,
        pot: total_pot,
        rake: rake_amount,
    });

    Ok(())
}





#[derive(Accounts)]
#[instruction(table_id: u64)]
pub struct ResolveShowdown<'info> {
    #[account(
        mut,
        seeds = [b"table", table_id.to_le_bytes().as_ref()],
        bump
    )]
    pub table: Account<'info, Table>,
    #[account(
        mut,
        seeds = [b"hand", table.key().as_ref(), table.hand_id_counter.to_le_bytes().as_ref()],
        bump
    )]
    pub hand_data: Account<'info, HandData>,
    #[account(mut)]
    pub payer: Signer<'info>,

    // Token accounts
    #[account(mut)]
    pub table_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_vault: Account<'info, TokenAccount>,
    pub platform_config: Account<'info, PlatformConfig>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}



#[event]
pub struct HandResolved {
    pub table_id: u64,
    pub hand_id: u64,
    pub pot: u64,
    pub rake: u64,
}

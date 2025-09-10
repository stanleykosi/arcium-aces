//! src/instructions/deal_community_cards.rs
//!
//! @description
//! This instruction is called after a betting round is complete (e.g., post-flop,
//! post-turn) to reveal the next set of community cards. It queues a confidential
//! computation on Arcium to securely draw cards from the encrypted deck.
//!
//! @accounts
//! - `table`: The poker table account containing public game state.
//! - `hand_data`: The account with the encrypted deck for the current hand.
//! - `payer`: The player initiating the transaction. Any active player can do this.
//! - Arcium-related accounts for the `reveal_community_cards` computation.
//!
//! @logic
//! 1. Validates the game state (`HandInProgress`).
//! 2. Determines how many cards to reveal based on the current betting round.
//! 3. Calculates the offset and length of the encrypted deck within the `HandData`
//!    account to pass it to Arcium by reference (`Argument::Account`).
//! 4. Queues the `reveal_community_cards` computation on Arcium.
//! 5. The `deal_community_cards_callback` receives the now-public card indices and
//!    the updated encrypted deck state. It updates both the `Table` (with public cards)
//!    and `HandData` (with the new encrypted deck) accounts.

use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;
use arcium_client::idl::arcium::types::CallbackAccount;
use arcium_client::idl::arcium::accounts::Cluster;
use arcium_client::idl::arcium::ID_CONST;
use crate::state::{Table, HandData, GameState, BettingRound, Card};
use crate::error::AcesUnknownErrorCode;
use crate::ID;

/// Instruction logic for dealing community cards.
pub fn deal_community_cards(ctx: Context<DealCommunityCards>, _table_id: u64, computation_offset: u64) -> Result<()> {
    let table = &ctx.accounts.table;

    // --- Validation ---
    require!(
        table.game_state == GameState::HandInProgress,
        AcesUnknownErrorCode::InvalidGameState
    );
    // Check that the current betting round is actually complete
    // This means all active players have either called, folded, or gone all-in
    let betting_round_complete = true;
    // Note: Player seat data is now stored in separate PlayerSeat accounts
    // In a real implementation, we would need to check each PlayerSeat account
    // to verify the betting round is complete
    require!(betting_round_complete, AcesUnknownErrorCode::InvalidGameState);

    let (num_cards_to_reveal, deck_top_card_idx) = match table.betting_round {
        BettingRound::PreFlop => (3, 0), // Flop (3 cards), top card index is after hole cards
        BettingRound::Flop => (1, 3),    // Turn (1 card)
        BettingRound::River => (1, 4),   // River (1 card)
        _ => return err!(AcesUnknownErrorCode::InvalidAction),
    };

    // --- Queue Arcium Computation ---
    // The offset to `encrypted_deck_ciphertexts` within `HandData` account data.
    const DECK_CIPHERTEXTS_OFFSET: u16 = 80;
    const DECK_CIPHERTEXTS_LEN: u32 = 32 * 3; // 3 ciphertexts, 32 bytes each

    // Use a more memory-efficient approach to avoid stack overflow
    let mut args = Vec::with_capacity(4); // Pre-allocate with exact capacity needed
    args.push(Argument::PlaintextU128(ctx.accounts.hand_data.encrypted_deck_nonce));
    args.push(Argument::Account(ctx.accounts.hand_data.key(), DECK_CIPHERTEXTS_OFFSET as u32, DECK_CIPHERTEXTS_LEN));
    args.push(Argument::PlaintextU8(deck_top_card_idx));
    args.push(Argument::PlaintextU8(num_cards_to_reveal));

    queue_computation(
        ctx.accounts,
        computation_offset,
        args,
        vec![
            CallbackAccount { pubkey: ctx.accounts.table.key(), is_writable: true },
            CallbackAccount { pubkey: ctx.accounts.hand_data.key(), is_writable: true },
        ],
        None,
    )?;

    Ok(())
}

/// Callback logic for `deal_community_cards`.
// #[arcium_callback(encrypted_ix = "reveal_community_cards")]
pub fn reveal_community_cards_callback(
    ctx: Context<DealCommunityCardsCallback>,
    output: ComputationOutputs<RevealCommunityCardsOutput>,
) -> Result<()> {
    let results = match output {
        ComputationOutputs::Success(data) => data.field_0,
        _ => return err!(AcesUnknownErrorCode::AbortedComputation),
    };

    let revealed_card_indices = results.field_0;
    let updated_deck = results.field_1;

    // --- Update HandData ---
    let hand_data = &mut ctx.accounts.hand_data;
    hand_data.encrypted_deck_ciphertexts = updated_deck.ciphertexts;
    hand_data.encrypted_deck_nonce = updated_deck.nonce;

    // --- Update Table ---
    let table = &mut ctx.accounts.table;
    let mut community_card_idx = 0;
    while community_card_idx < 5 && table.community_cards[community_card_idx].is_some() {
        community_card_idx += 1;
    }

    for card_index_bytes in revealed_card_indices.ciphertexts.iter() {
        // Extract the first byte as the card index
        let card_index = card_index_bytes[0];
        if card_index != 255 && community_card_idx < 5 { // 255 is INVALID_CARD_INDEX
            table.community_cards[community_card_idx] = Some(Card {
                rank: card_index % 13,
                suit: card_index / 13,
            });
            community_card_idx += 1;
        }
    }
    
    // Advance betting round
    table.betting_round = match table.betting_round {
        BettingRound::PreFlop => BettingRound::Flop,
        BettingRound::Flop => BettingRound::Turn,
        BettingRound::Turn => BettingRound::River,
        _ => table.betting_round, // Should not happen
    };

    // Reset round-based betting info and set turn to first active player after dealer
    table.current_bet = 0;
    // We can't reset player.bet_this_round because it's not stored in PlayerSeatInfo
    // In a real implementation, we would need to access this information
    // from a separate account or use a different approach
    
    // Set turn to first active player after dealer
    let mut next_player_pos = (table.dealer_position + 1) % crate::state::constants::MAX_PLAYERS as u8;
    while (table.occupied_seats & (1 << next_player_pos)) == 0 {
        // Note: In a real implementation, we would need to check the PlayerSeat account
        // to verify the player is active in the hand
        next_player_pos = (next_player_pos + 1) % crate::state::constants::MAX_PLAYERS as u8;
    }
    table.turn_position = next_player_pos;
    table.last_aggressor_position = next_player_pos; // Initialize for new betting round
    table.turn_started_at = Clock::get()?.unix_timestamp;
    
    emit!(CommunityCardsDealt {
        table_id: table.table_id,
        hand_id: hand_data.hand_id,
        cards: table.community_cards,
    });
    
    Ok(())
}

#[queue_computation_accounts("reveal_community_cards", payer)]
#[derive(Accounts)]
#[instruction(table_id: u64, computation_offset: u64)]
pub struct DealCommunityCards<'info> {
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

    // Arcium accounts
    #[account(address = derive_mxe_pda!())]
    pub mxe_account: Account<'info, MXEAccount>,
    #[account(mut, address = derive_mempool_pda!())]
    /// CHECK: Checked by Arcium program
    pub mempool_account: UncheckedAccount<'info>,
    #[account(mut, address = derive_execpool_pda!())]
    /// CHECK: Checked by Arcium program
    pub executing_pool: UncheckedAccount<'info>,
    #[account(mut, address = derive_comp_pda!(computation_offset))]
    /// CHECK: Checked by Arcium program
    pub computation_account: UncheckedAccount<'info>,
    #[account(address = derive_comp_def_pda!(crate::COMP_DEF_OFFSET_REVEAL_COMMUNITY_CARDS))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(mut)]
    pub cluster_account: Account<'info, Cluster>,
    #[account(mut, address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS)]
    pub pool_account: Account<'info, FeePool>,
    #[account(address = ARCIUM_CLOCK_ACCOUNT_ADDRESS)]
    pub clock_account: Account<'info, ClockAccount>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

#[callback_accounts("reveal_community_cards", payer)]
#[derive(Accounts)]
pub struct DealCommunityCardsCallback<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(address = derive_comp_def_pda!(crate::COMP_DEF_OFFSET_REVEAL_COMMUNITY_CARDS))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,
    
    #[account(mut)]
    pub table: Account<'info, Table>,
    #[account(mut)]
    pub hand_data: Account<'info, HandData>,
}

#[event]
pub struct CommunityCardsDealt {
    pub table_id: u64,
    pub hand_id: u64,
    pub cards: [Option<Card>; 5],
}
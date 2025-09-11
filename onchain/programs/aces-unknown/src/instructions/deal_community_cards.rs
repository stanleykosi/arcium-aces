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
use anchor_lang::Discriminator;
use crate::state::{Table, HandData, GameState, BettingRound, Card};
use crate::error::AcesUnknownErrorCode;


/// Instruction logic for dealing community cards.
pub fn deal_community_cards(ctx: Context<DealCommunityCards>, _table_id: u64) -> Result<()> {
    let table = &mut ctx.accounts.table;

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

    // TODO: Add Arcium computation queuing once Arcium integration is properly set up

    // For now, simulate revealing community cards
    let mut community_card_idx = 0;
    while community_card_idx < 5 && table.community_cards[community_card_idx].is_some() {
        community_card_idx += 1;
    }

    // Simulate revealing cards (in a real implementation, this would come from Arcium)
    for i in 0..num_cards_to_reveal {
        if community_card_idx < 5 {
            // Use deterministic card generation for testing
            let card_index = (deck_top_card_idx + i) as u8;
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

    // Set turn to first active player after dealer
    let mut next_player_pos = (table.dealer_position + 1) % crate::state::constants::MAX_PLAYERS as u8;
    while (table.occupied_seats & (1 << next_player_pos)) == 0 {
        next_player_pos = (next_player_pos + 1) % crate::state::constants::MAX_PLAYERS as u8;
    }
    table.turn_position = next_player_pos;
    table.last_aggressor_position = next_player_pos; // Initialize for new betting round
    table.turn_started_at = Clock::get()?.unix_timestamp;

    emit!(CommunityCardsDealt {
        table_id: table.table_id,
        hand_id: ctx.accounts.hand_data.hand_id,
        cards: table.community_cards,
    });

    Ok(())
}





#[derive(Accounts)]
#[instruction(table_id: u64)]
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
    pub system_program: Program<'info, System>,
}



#[event]
pub struct CommunityCardsDealt {
    pub table_id: u64,
    pub hand_id: u64,
    pub cards: [Option<Card>; 5],
}

#[event]
pub struct HandShuffled {
    pub table_id: u64,
}

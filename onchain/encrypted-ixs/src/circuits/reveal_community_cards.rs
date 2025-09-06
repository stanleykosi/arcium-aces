//! src/circuits/reveal_community_cards.rs
//!
//! @description
//! Defines the `reveal_community_cards` confidential instruction. This circuit is
//! responsible for securely revealing community cards (Flop, Turn, and River)
//! from the encrypted deck during a hand of poker.
//!
//! @logic
//! 1. Takes the encrypted deck, an index to the top of the deck, and the number
//!    of cards to reveal as input.
//! 2. Decrypts the deck within the Arcium MPC environment.
//! 3. "Burns" the top card of the deck by marking its value as invalid.
//! 4. Reveals the specified number of cards that follow the burn card.
//! 5. Marks the revealed cards as used within the deck array.
//! 6. Returns the publicly revealed card indices in a fixed-size array (padded with
//!    an invalid value if fewer than 3 cards are revealed).
//! 7. Returns the re-encrypted, updated deck state to be stored back on-chain.
//!
//! @dependencies
//! - `arcis_imports`: For all Arcis-related macros and types.
//! - `crate::types`: For the `Deck` data structure.

use arcis_imports::*;
use crate::types::Deck;

/// An invalid card index used for padding and marking cards as "used".
const INVALID_CARD_INDEX: u8 = 255;

/// The maximum number of cards that can be revealed in a single operation (the flop).
const MAX_REVEAL: usize = 3;

/// Securely reveals community cards from the encrypted deck.
///
/// This instruction is called multiple times during a hand: once for the flop (3 cards),
/// once for the turn (1 card), and once for the river (1 card). It ensures that
/// no one knows the community cards before they are officially dealt.
///
/// # Arguments
/// * `deck_ctxt`: The `Enc<Mxe, Deck>` containing the current state of the shuffled deck.
/// * `deck_top_card_idx`: The index of the next card to be dealt from the deck array.
/// * `num_cards_to_reveal`: The number of cards to reveal (e.g., 3 for flop, 1 for turn/river).
///
/// # Returns
/// A tuple containing:
/// - `[u8; 3]`: A fixed-size array with the indices of the revealed cards. If fewer
///   than 3 cards are revealed, the remaining slots are padded with `INVALID_CARD_INDEX`.
/// - `Enc<Mxe, Deck>`: The updated encrypted deck with the dealt cards marked as used.
#[instruction]
pub fn reveal_community_cards(
    deck_ctxt: Enc<Mxe, Deck>,
    deck_top_card_idx: u8,
    num_cards_to_reveal: u8,
) -> ([u8; MAX_REVEAL], Enc<Mxe, Deck>) {
    // 1. Decrypt the deck inside the MPC.
    let mut deck_array = deck_ctxt.to_arcis().to_array();

    // 2. Burn the top card.
    // The on-chain program must ensure deck_top_card_idx is valid.
    let burn_card_idx = deck_top_card_idx as usize;
    if burn_card_idx < deck_array.len() {
        deck_array[burn_card_idx] = INVALID_CARD_INDEX;
    }

    // 3. Reveal the next N cards.
    let mut revealed_cards = [INVALID_CARD_INDEX; MAX_REVEAL];
    let reveal_start_idx = (deck_top_card_idx + 1) as usize;

    // The loop iterates up to `num_cards_to_reveal` but not exceeding the MAX_REVEAL constant.
    // This is a data-independent loop for security in MPC.
    for i in 0..MAX_REVEAL {
        let current_idx = reveal_start_idx + i;
        // The condition ensures we only reveal the requested number of cards
        // and do not go out of bounds.
        if i < num_cards_to_reveal as usize && current_idx < deck_array.len() {
            revealed_cards[i] = deck_array[current_idx];
            deck_array[current_idx] = INVALID_CARD_INDEX; // Mark the card as used
        }
    }

    // 4. Re-encrypt the updated deck.
    let updated_deck = Deck::from_array(deck_array);
    let updated_deck_ctxt = deck_ctxt.owner.from_arcis(updated_deck);

    // 5. Return the public cards and the new encrypted deck state.
    (revealed_cards, updated_deck_ctxt)
}
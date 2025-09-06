//! src/types.rs
//!
//! @description
//! This module defines the core data structures used within the Arcis confidential
//! computation environment for the Aces Unknown poker game. These structs are designed
//! for efficiency within the Multi-Party Computation (MPC) context, often using
//! packed integer representations to minimize data size and computational overhead.
//!
//! Key Structs:
//! - Card: A representation of a single playing card.
//! - Deck: A memory-efficient, packed representation of a 52-card deck.
//! - Hand: A packed representation of a player's 2-card hole hand.
//! - WinnerInfo: A struct to hold showdown results for on-chain processing.
//!
//! @dependencies
//! - `arcis_imports`: Provides core types for Arcis circuit development, such as `ArcisPublicKey`.

use arcis_imports::*;

/// Powers of 64 used for encoding/decoding cards into/from u128 values.
/// Each card is represented by 6 bits (value 0-51), so we can pack multiple cards
/// into a single u128 by treating each card as a digit in a base-64 number system.
/// This array contains 64^i for i in 0..21.
const POWS_OF_SIXTY_FOUR: [u128; 21] = [
    1,
    64,
    4096,
    262144,
    16777216,
    1073741824,
    68719476736,
    4398046511104,
    281474976710656,
    18014398509481984,
    1152921504606846976,
    73786976294838206464,
    4722366482869645213696,
    302231454903657293676544,
    19342813113834066795298816,
    1237940039285380274899124224,
    79228162514264337593543950336,
    5070602400912917605986812821504,
    324518553658426726783156020576256,
    20769187434139310514121985316880384,
    1329227995784915872903807060280344576,
];

/// Represents a single playing card within an Arcis circuit.
#[derive(Clone, Copy, Debug)]
pub struct Card {
    /// The rank of the card, from 0 to 12.
    /// 0 = Two, 1 = Three, ..., 8 = Ten, 9 = Jack, 10 = Queen, 11 = King, 12 = Ace.
    pub rank: u8,
    /// The suit of the card, from 0 to 3.
    /// 0 = Clubs, 1 = Diamonds, 2 = Hearts, 3 = Spades.
    pub suit: u8,
}

impl Card {
    /// Converts the Card struct to a single `u8` index (0-51) for packing.
    pub fn to_u8_index(&self) -> u8 {
        // The formula ensures a unique index for each of the 52 cards.
        self.suit * 13 + self.rank
    }

    /// Creates a Card struct from a single `u8` index (0-51).
    pub fn from_u8_index(index: u8) -> Self {
        // This will panic if index > 51, which is intended behavior as it indicates a logic error.
        assert!(index < 52, "Invalid card index");
        Card {
            suit: index / 13,
            rank: index % 13,
        }
    }
}

/// Represents a full 52-card deck, packed into three u128 values for MPC efficiency.
/// This structure is inspired by the Arcium Blackjack example and is highly optimized.
pub struct Deck {
    /// Packs cards with indices 0-20. (21 cards * 6 bits = 126 bits)
    pub cards_chunk_0: u128,
    /// Packs cards with indices 21-41. (21 cards * 6 bits = 126 bits)
    pub cards_chunk_1: u128,
    /// Packs cards with indices 42-51. (10 cards * 6 bits = 60 bits)
    pub cards_chunk_2: u128,
}

impl Deck {
    /// Converts a 52-card array of indices into the packed Deck representation.
    pub fn from_array(array: [u8; 52]) -> Deck {
        let mut cards_chunk_0 = 0;
        for i in 0..21 {
            cards_chunk_0 += POWS_OF_SIXTY_FOUR[i] * array[i] as u128;
        }

        let mut cards_chunk_1 = 0;
        for i in 21..42 {
            cards_chunk_1 += POWS_OF_SIXTY_FOUR[i - 21] * array[i] as u128;
        }

        let mut cards_chunk_2 = 0;
        for i in 42..52 {
            cards_chunk_2 += POWS_OF_SIXTY_FOUR[i - 42] * array[i] as u128;
        }

        Deck {
            cards_chunk_0,
            cards_chunk_1,
            cards_chunk_2,
        }
    }

    /// Converts the packed Deck representation back to a 52-card array of indices.
    pub fn to_array(&self) -> [u8; 52] {
        let mut card_one = self.cards_chunk_0;
        let mut card_two = self.cards_chunk_1;
        let mut card_three = self.cards_chunk_2;

        let mut bytes = [0u8; 52];

        // Unpack the first two chunks simultaneously
        for i in 0..21 {
            bytes[i] = (card_one % 64) as u8;
            bytes[i + 21] = (card_two % 64) as u8;
            card_one >>= 6;
            card_two >>= 6;
        }

        // Unpack the final chunk
        for i in 42..52 {
            bytes[i] = (card_three % 64) as u8;
            card_three >>= 6;
        }

        bytes
    }
}

/// Represents a player's two hole cards, packed into a single u128.
pub struct Hand {
    /// The two card indices (0-51) are packed into this field.
    /// Card 1 uses the first 6 bits, Card 2 uses the next 6 bits.
    pub cards_packed: u128,
}

impl Hand {
    /// Creates a packed Hand from an array of 2 card indices.
    pub fn from_array(cards: [u8; 2]) -> Self {
        let cards_packed = (cards[0] as u128) * POWS_OF_SIXTY_FOUR[0]
            + (cards[1] as u128) * POWS_OF_SIXTY_FOUR[1];
        Self { cards_packed }
    }

    /// Converts the packed Hand back to an array of 2 card indices.
    pub fn to_array(&self) -> [u8; 2] {
        let mut packed = self.cards_packed;
        let mut cards = [0u8; 2];

        cards[0] = (packed % 64) as u8;
        packed >>= 6; // Shift to get the next card
        cards[1] = (packed % 64) as u8;

        cards
    }
}

/// Contains information about a winner at showdown.
/// This struct is returned as a public value from the `evaluate_hands_and_payout` circuit.
#[derive(Clone, Copy)]
pub struct WinnerInfo {
    /// The public key of the winning player, represented as an ArcisPublicKey within the circuit.
    pub player_pubkey: ArcisPublicKey,
    /// The amount of chips won by the player.
    pub amount_won: u64,
}
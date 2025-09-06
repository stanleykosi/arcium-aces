//! src/state/card.rs
//!
//! @description
//! Defines the on-chain representation of a single playing card.
//! This struct is used for public-facing card data, such as community cards.
//! It is distinct from the packed, encrypted representation used within Arcis.
//!
//! Key features:
//! - Represents a card with a `rank` and a `suit`.
//! - Derives necessary traits for on-chain storage and client-side deserialization.

use anchor_lang::prelude::*;

/// Represents a single playing card with its rank and suit.
/// This struct is intended for storing public card information on-chain,
/// like the community cards (Flop, Turn, River).
///
/// The confidential representation used within Arcis is a `u8` index for packing efficiency.
/// This struct provides a more descriptive format for public state.
#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
    /// The rank of the card, from 0 to 12.
    /// 0 = Two, 1 = Three, ..., 8 = Ten, 9 = Jack, 10 = Queen, 11 = King, 12 = Ace.
    pub rank: u8,
    /// The suit of the card, from 0 to 3.
    /// 0 = Clubs, 1 = Diamonds, 2 = Hearts, 3 = Spades.
    pub suit: u8,
}
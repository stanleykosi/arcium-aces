//! src/state/player_seat.rs
//!
//! @description
//! This module defines the `PlayerSeat` account, which stores information about
//! a single player seated at a poker table. This approach allows us to avoid
//! large arrays in the main `Table` account, which can cause stack overflow issues.
//!
//! Key features:
//! - Stores player information for a single seat at a table
//! - Uses a PDA with the table and seat index as seeds
//! - Can be efficiently accessed by instructions that need player data

use anchor_lang::prelude::*;

/// Contains the state for a single player seated at a table.
#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerInfo {
    /// The player's wallet public key.
    pub pubkey: Pubkey,
    /// The player's current chip stack at the table.
    pub stack: u64,
    /// Flag indicating if the player is currently participating in the hand.
    pub is_active_in_hand: bool,
    /// Flag indicating if the player is all-in.
    pub is_all_in: bool,
    /// The amount the player has bet in the current betting round.
    pub bet_this_round: u64,
    /// The total amount the player has committed to the pot in the entire hand.
    pub total_bet_this_hand: u64,
}

/// Represents a player seated at a specific position at a table
#[account]
#[derive(InitSpace)]
pub struct PlayerSeat {
    /// The public key of the table this seat belongs to
    pub table_pubkey: Pubkey,
    
    /// The index of this seat at the table (0-5 for 6-max)
    pub seat_index: u8,
    
    /// The player's wallet public key
    pub player_pubkey: Pubkey,
    
    /// The player's current chip stack at the table
    pub stack: u64,
    
    /// Flag indicating if the player is currently participating in the hand
    pub is_active_in_hand: bool,
    
    /// Flag indicating if the player is all-in
    pub is_all_in: bool,
    
    /// The amount the player has bet in the current betting round
    pub bet_this_round: u64,
    
    /// The total amount the player has committed to the pot in the entire hand
    pub total_bet_this_hand: u64,
    
    /// Bump seed for the PDA
    pub bump: u8,
}
//! src/state/encrypted_hand.rs
//!
//! @description
//! This module defines the `EncryptedHand` account, which stores the encrypted
//! hole cards for a single player in a hand. This approach allows us to avoid
//! large arrays in the main `HandData` account, which can cause stack overflow issues.
//!
//! Key features:
//! - Stores encrypted hand information for a single player in a hand
//! - Uses a PDA with the hand and player pubkey as seeds
//! - Can be efficiently accessed by instructions that need encrypted hand data

use anchor_lang::prelude::*;

/// An account to store the encrypted information for a single player's hand.
#[account]
#[derive(InitSpace)]
pub struct EncryptedHand {
    /// The public key of the `HandData` account this encrypted hand belongs to.
    pub hand_pubkey: Pubkey,
    
    /// The player's public key.
    pub player_pubkey: Pubkey,
    
    /// The encrypted hole cards (packed into a single ciphertext).
    pub ciphertext: [u8; 32],
    
    /// The nonce used for this specific encryption.
    pub nonce: u128,
    
    /// The player's x25519 public key used for the key exchange.
    pub encryption_key: [u8; 32],
    
    /// Bump seed for the PDA
    pub bump: u8,
}
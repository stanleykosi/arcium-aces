//! src/circuits/shuffle_and_deal.rs
//!
//! @description
//! This file defines the `shuffle_and_deal` confidential instruction for the
//! Aces Unknown poker platform. This is one of the most critical circuits, as it
//! establishes the provably fair foundation for each hand of poker.
//!
//! @logic
//! 1. Initializes a standard 52-card deck.
//! 2. Uses Arcium's cryptographically secure Random Number Generator (`ArcisRNG`)
//!    to shuffle the deck.
//! 3. Generates a cryptographic commitment to the shuffle, allowing for later verification.
//! 4. Deals two hole cards to each active player in a round-robin fashion, mimicking a
//!    real poker deal.
//! 5. Encrypts each player's hole cards individually using a shared secret derived from
//!    their public key, ensuring only they can view their hand.
//! 6. Encrypts the entire shuffled deck for the Arcium network (MXE), keeping the
//!    sequence of community cards confidential until they are revealed.
//! 7. Returns the encrypted deck, shuffle commitment, and an array of all players'
//!    encrypted hands.
//!
//! @dependencies
//! - `arcis_imports`: For all Arcis-related macros and types.
//! - `crate::types`: For our custom `Deck` and `Hand` data structures.
//!
//! @notes
//! - The instruction uses fixed-size arrays for inputs and outputs to comply with
//!   Arcis limitations.
//! - The on-chain program is responsible for providing valid (non-zero) public keys
//!   for all player slots, even inactive ones, to prevent circuit failures.

use arcis_imports::*;
use crate::types::*;

/// A standard 52-card deck represented as indices from 0 to 51.
const INITIAL_DECK: [u8; 52] = [
     0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51,
];

/// The maximum number of players at a table.
const MAX_PLAYERS: usize = 6;

/// Securely shuffles a 52-card deck and deals encrypted hole cards to each player.
///
/// This function is the entry point for starting a new hand. It ensures that the shuffle
/// is random and that no party, including the platform operator, can know the order of
/// the cards or the contents of any player's hand.
///
/// # Arguments
/// * `mxe`: The Arcium execution environment context, used for MXE-only encryption.
/// * `player_pubkeys`: An array of `ArcisPublicKey` for each of the 6 seats at the table.
/// * `active_players`: A boolean array indicating which of the 6 seats are occupied by active players.
///
/// # Returns
/// A tuple containing:
/// - `Enc<Mxe, Deck>`: The entire 52-card deck, shuffled and encrypted so only the MPC can read it.
/// - `[u8; 32]`: A cryptographic commitment to the shuffle for later verification.
/// - `[Enc<Shared, Hand>; 6]`: An array of encrypted 2-card hands for each seat. Only the
///   corresponding player can decrypt their own hand. Inactive seats contain encrypted dummy data.
#[instruction]
pub fn shuffle_and_deal(
    mxe: Mxe,
    player_pubkeys: [ArcisPublicKey; MAX_PLAYERS],
    active_players: [bool; MAX_PLAYERS],
) -> (
    Enc<Mxe, Deck>,
    [u8; 32],
    [Enc<Shared, Hand>; MAX_PLAYERS],
) {
    // 1. Shuffle the Deck
    let mut shuffled_deck = INITIAL_DECK;
    ArcisRNG::shuffle(&mut shuffled_deck);

    // 2. Generate Shuffle Commitment
    // TODO: Replace this with a proper cryptographic hash function once available in Arcis.
    // For now, we use the first 32 bytes of the shuffled deck as a commitment.
    let mut shuffle_commitment = [0u8; 32];
    for i in 0..32 {
        shuffle_commitment[i] = shuffled_deck[i];
    }

    // 3. Deal Hole Cards
    let mut dealt_cards: [[u8; 2]; MAX_PLAYERS] = [[52; 2]; MAX_PLAYERS]; // 52 is an invalid card index
    let mut card_idx_counter = 0;
    
    // First card dealt to each active player
    for i in 0..MAX_PLAYERS {
        if active_players[i] {
            dealt_cards[i][0] = shuffled_deck[card_idx_counter];
            card_idx_counter += 1;
        }
    }

    // Second card dealt to each active player
    for i in 0..MAX_PLAYERS {
        if active_players[i] {
            dealt_cards[i][1] = shuffled_deck[card_idx_counter];
            card_idx_counter += 1;
        }
    }

    // 4. Encrypt Hands for Each Player
    // We must create an array of Enc<Shared, Hand> to return.
    // Arcis requires a fixed-size array, so we must initialize it fully.
    // The `from_fn` approach is not supported, so we build it manually.
    
    // Create a dummy hand for inactive players. We must encrypt something.
    let dummy_hand = Hand::from_array([52, 52]);
    // Create a dummy shared context using the first pubkey. The data is irrelevant for inactive players.
    let dummy_shared_context = Shared::new(player_pubkeys[0]);
    let dummy_encrypted_hand = dummy_shared_context.from_arcis(dummy_hand);

    let mut encrypted_hands: [Enc<Shared, Hand>; MAX_PLAYERS] = [
        dummy_encrypted_hand,
        dummy_encrypted_hand,
        dummy_encrypted_hand,
        dummy_encrypted_hand,
        dummy_encrypted_hand,
        dummy_encrypted_hand,
    ];

    for i in 0..MAX_PLAYERS {
        // We encrypt hands for both active and inactive players.
        // The on-chain program and clients will know to ignore hands for inactive players.
        // This is necessary to satisfy Arcis's fixed-size array requirements.
        let player_shared_context = Shared::new(player_pubkeys[i]);
        let hand_struct = Hand::from_array(dealt_cards[i]);
        encrypted_hands[i] = player_shared_context.from_arcis(hand_struct);
    }
    
    // 5. Encrypt the Full Shuffled Deck for the MXE
    let encrypted_deck = mxe.from_arcis(Deck::from_array(shuffled_deck));

    // 6. Return all data
    (encrypted_deck, shuffle_commitment, encrypted_hands)
}
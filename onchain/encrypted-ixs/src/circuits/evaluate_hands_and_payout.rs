//! src/circuits/evaluate_hands_and_payout.rs
//!
//! @description
//! This file defines the `evaluate_hands_and_payout` confidential instruction, which
//! is the final and most critical step in a poker hand. This circuit is responsible
//! for securely determining the winner(s) at showdown and calculating the precise
//! distribution of the pot, including any side pots.
//!
//! @logic
//! 1. Inputs: Takes encrypted hole cards for all players, public community cards,
//!    total bets for each player, an active player mask, and player public keys.
//! 2. Decryption: Securely decrypts each active player's 2-card hand within the MPC.
//! 3. Hand Evaluation: For each active player, it combines their 2 hole cards with
//!    the 5 community cards and calls the `poker_evaluator` logic to determine the
//!    best possible 5-card hand and its rank.
//! 4. Payout Calculation: It passes the list of hand ranks and player bets to the
//!    `pot_calculator` logic, which handles the complex task of distributing the
//!    main pot and any side pots according to poker rules.
//! 5. Output: Returns a publicly visible, fixed-size array of `WinnerInfo` structs,
//!    detailing which players won and the exact amounts they are to be paid.
//!
//! @dependencies
//! - `arcis_imports`: For all Arcis-related macros and types.
//! - `crate::types`: For `Hand`, `WinnerInfo`.
//! - `crate::logic::poker_evaluator`: For hand evaluation.
//! - `crate::logic::pot_calculator`: For payout calculations.

use arcis_imports::*;
use crate::types::{Hand, WinnerInfo};
use crate::logic::{poker_evaluator, pot_calculator};

/// The maximum number of players at a table.
pub const MAX_PLAYERS: usize = 6;

/// Evaluates all active hands at showdown and calculates the pot distribution.
///
/// This is the final confidential computation in a hand. It takes all private
/// (hole cards) and public (bets, community cards) data, performs the comparison
/// and financial calculations securely, and outputs the public results for the
/// on-chain program to execute the payouts.
///
/// # Arguments
/// * `player_hands`: An array of encrypted 2-card hands for each seat.
/// * `community_cards`: A public array of the 5 community cards indices.
/// * `player_bets`: The total amount each player has bet in the hand.
/// * `active_players`: A boolean mask indicating which players are part of the showdown.
/// * `player_pubkeys`: The Arcis public keys for each player, used to identify winners.
///
/// # Returns
/// An array of `WinnerInfo` structs. Each entry corresponds to a player seat and
/// contains their public key and the amount of chips they won. Non-winners will have
/// an amount of 0.
#[instruction]
pub fn evaluate_hands_and_payout(
    player_hands: [Enc<Shared, Hand>; MAX_PLAYERS],
    community_cards: [u8; 5],
    player_bets: [u64; MAX_PLAYERS],
    active_players: [bool; MAX_PLAYERS],
    player_pubkeys: [ArcisPublicKey; MAX_PLAYERS],
) -> [WinnerInfo; MAX_PLAYERS] {

    // 1. Evaluate each active player's hand
    let dummy_rank = poker_evaluator::HandRank::NoHand;
    let mut player_ranks = [dummy_rank; MAX_PLAYERS];

    for i in 0..MAX_PLAYERS {
        if active_players[i] {
            // Decrypt the player's hole cards
            let hole_cards = player_hands[i].to_arcis().to_array();

            // Combine hole cards and community cards
            let mut seven_cards = [0u8; 7];
            seven_cards[0] = hole_cards[0];
            seven_cards[1] = hole_cards[1];
            for j in 0..5 {
                seven_cards[j + 2] = community_cards[j];
            }

            // Evaluate the best 5-card hand from the 7 cards
            player_ranks[i] = poker_evaluator::evaluate_7_cards(seven_cards);
        }
    }

    // 2. Calculate payouts using the pot calculator logic
    let winner_payouts = pot_calculator::calculate_payouts(
        player_bets,
        player_ranks,
        active_players,
        player_pubkeys,
    );
    
    // 3. Return the results
    winner_payouts
}
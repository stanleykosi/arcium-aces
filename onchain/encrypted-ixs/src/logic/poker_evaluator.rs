//! src/logic/poker_evaluator.rs
//!
//! @description
//! This module provides the core logic for evaluating Texas Hold'em poker hands
//! within the Arcis MPC environment. It is designed to determine the best possible
//! 5-card hand from a set of 7 cards (2 hole cards + 5 community cards) and
//! assign it a rank for comparison.
//!
//! @logic
//! The evaluation process follows these steps:
//! 1. Card Representation: Cards are represented by their `u8` index (0-51).
//!    Helper functions extract rank (0-12) and suit (0-3).
//! 2. Rank Counting: An array is used to count the occurrences of each rank to
//!    identify pairs, three-of-a-kind, four-of-a-kind, etc.
//! 3. Flush and Straight Detection: Logic to check for flushes (five cards of
//!    the same suit) and straights (five cards of sequential rank).
//! 4. Hand Ranking: The main evaluation function checks for hand types in
//!    descending order of strength (from Straight Flush down to High Card).
//! 5. Tie-breaking: The `HandRank` enum stores kicker information, allowing for
//!    accurate tie-breaking according to poker rules.
//!
//! @dependencies
//! - `arcis_imports`: For Arcis types and functions.
//!
//! @notes
//! - All algorithms are implemented using fixed-size loops and data-independent
//!   operations to be compatible with the MPC environment.
//! - The `evaluate_7_cards` function is the primary entry point for this module.

use arcis_imports::*;

// Constants for card properties
const NUM_SUITS: u8 = 4;
const NUM_RANKS: u8 = 13;
const ACE_RANK: u8 = 12; // In our system: 2=0, ..., K=11, A=12

/// Represents the rank of a poker hand, including data for tie-breaking.
/// The enum is ordered from highest rank to lowest rank.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HandRank {
    StraightFlush { high_card_rank: u8 },
    FourOfAKind { quad_rank: u8, kicker_rank: u8 },
    FullHouse { three_rank: u8, pair_rank: u8 },
    Flush { ranks: [u8; 5] },
    Straight { high_card_rank: u8 },
    ThreeOfAKind { three_rank: u8, kickers: [u8; 2] },
    TwoPair { high_pair_rank: u8, low_pair_rank: u8, kicker_rank: u8 },
    OnePair { pair_rank: u8, kickers: [u8; 3] },
    HighCard { ranks: [u8; 5] },
    NoHand, // Placeholder for initialization
}

/// Helper function to get the rank of a card from its index.
fn get_rank(card_idx: u8) -> u8 {
    card_idx % NUM_RANKS
}

/// Helper function to get the suit of a card from its index.
fn get_suit(card_idx: u8) -> u8 {
    card_idx / NUM_RANKS
}

/// Primary function to evaluate the best 5-card hand from a given set of 7 cards.
pub fn evaluate_7_cards(cards: [u8; 7]) -> HandRank {
    // --- Data Preparation ---
    let mut ranks = [0u8; 7];
    let mut suits = [0u8; 7];
    for i in 0..7 {
        ranks[i] = get_rank(cards[i]);
        suits[i] = get_suit(cards[i]);
    }
    // Sort ranks in descending order for easier processing.
    // Arcis supports .sort() for integer arrays.
    ranks.sort();
    ranks.reverse();

    // --- Check for Flush ---
    let mut suit_counts = [0u8; NUM_SUITS as usize];
    for suit in suits {
        suit_counts[suit as usize] += 1;
    }

    let mut flush_suit = 255u8; // Invalid suit
    for i in 0..NUM_SUITS {
        if suit_counts[i as usize] >= 5 {
            flush_suit = i;
        }
    }
    
    let is_flush = flush_suit != 255;
    
    // --- Check for Straight ---
    // Use a unique, sorted list of ranks to detect straights.
    let mut unique_ranks = [255u8; 7];
    let mut unique_count = 0;
    for i in 0..7 {
        let mut found = false;
        for j in 0..unique_count {
            if ranks[i] == unique_ranks[j] {
                found = true;
            }
        }
        if !found {
            unique_ranks[unique_count] = ranks[i];
            unique_count += 1;
        }
    }
    unique_ranks.sort(); // Sort ascending for straight check
    unique_ranks.reverse();

    let mut straight_high_card = 255u8;
    if unique_count >= 5 {
        for i in 0..(unique_count - 4) {
            if unique_ranks[i] == unique_ranks[i+1] + 1 &&
               unique_ranks[i] == unique_ranks[i+2] + 2 &&
               unique_ranks[i] == unique_ranks[i+3] + 3 &&
               unique_ranks[i] == unique_ranks[i+4] + 4 {
                straight_high_card = unique_ranks[i];
                // Break after finding the highest straight
                // Cannot `break` in Arcis, so we let it complete
            }
        }
        // Special case for Ace-low straight (A, 2, 3, 4, 5)
        let has_ace = unique_ranks[0] == ACE_RANK;
        let has_2 = unique_ranks[unique_count-1] == 0; // 2 is rank 0
        let has_3 = unique_ranks[unique_count-2] == 1;
        let has_4 = unique_ranks[unique_count-3] == 2;
        let has_5 = unique_ranks[unique_count-4] == 3;

        if has_ace && has_2 && has_3 && has_4 && has_5 {
            // High card of an Ace-low straight is 5 (rank 3)
            if straight_high_card == 255 {
                straight_high_card = 3; 
            }
        }
    }
    let is_straight = straight_high_card != 255;

    // --- Check for Straight Flush ---
    if is_flush && is_straight {
        let mut flush_ranks = [255u8; 7];
        let mut flush_ranks_count = 0;
        for i in 0..7 {
            if get_suit(cards[i]) == flush_suit {
                flush_ranks[flush_ranks_count] = get_rank(cards[i]);
                flush_ranks_count += 1;
            }
        }
        flush_ranks.sort();
        flush_ranks.reverse();

        // Check for straight within the flush ranks
        let mut straight_flush_high_card = 255u8;
        if flush_ranks_count >= 5 {
             for i in 0..(flush_ranks_count - 4) {
                if flush_ranks[i] == flush_ranks[i+1] + 1 &&
                   flush_ranks[i] == flush_ranks[i+2] + 2 &&
                   flush_ranks[i] == flush_ranks[i+3] + 3 &&
                   flush_ranks[i] == flush_ranks[i+4] + 4 {
                    straight_flush_high_card = flush_ranks[i];
                }
            }
            // Ace-low straight flush check
            let has_ace = flush_ranks[0] == ACE_RANK;
            let has_2 = flush_ranks[flush_ranks_count-1] == 0;
            let has_3 = flush_ranks[flush_ranks_count-2] == 1;
            let has_4 = flush_ranks[flush_ranks_count-3] == 2;
            let has_5 = flush_ranks[flush_ranks_count-4] == 3;
            if has_ace && has_2 && has_3 && has_4 && has_5 {
                if straight_flush_high_card == 255 {
                    straight_flush_high_card = 3;
                }
            }
        }

        if straight_flush_high_card != 255 {
            return HandRank::StraightFlush { high_card_rank: straight_flush_high_card };
        }
    }

    // --- Count Ranks for Pairs, Threes, Fours ---
    let mut rank_counts = [0u8; NUM_RANKS as usize];
    for rank in ranks {
        rank_counts[rank as usize] += 1;
    }

    let mut fours = 255u8;
    let mut threes = [255u8; 2];
    let mut pairs = [255u8; 3];
    let mut threes_count = 0;
    let mut pairs_count = 0;

    for i in 0..NUM_RANKS {
        let rank = (NUM_RANKS - 1 - i) as u8; // Iterate from Ace down to 2
        let count = rank_counts[rank as usize];
        if count == 4 { fours = rank; }
        if count == 3 { 
            if threes_count < 2 { threes[threes_count] = rank; threes_count += 1; }
        }
        if count == 2 {
            if pairs_count < 3 { pairs[pairs_count] = rank; pairs_count += 1; }
        }
    }

    // --- Determine Hand Rank based on counts (and flush/straight checks) ---
    // Order of checks is important (highest rank first)
    
    // Four of a Kind
    if fours != 255 {
        let mut kicker = 255u8;
        for rank in ranks {
            if rank != fours {
                kicker = rank;
                // cannot break
            }
        }
        return HandRank::FourOfAKind { quad_rank: fours, kicker_rank: kicker };
    }

    // Full House
    if threes_count > 0 && pairs_count > 0 {
        return HandRank::FullHouse { three_rank: threes[0], pair_rank: pairs[0] };
    }
    // Case of two three-of-a-kinds (e.g., AAA KKK Q) -> higher three makes full house
    if threes_count > 1 {
        return HandRank::FullHouse { three_rank: threes[0], pair_rank: threes[1] };
    }

    // Flush
    if is_flush {
        let mut flush_ranks = [0u8; 7];
        let mut count = 0;
        for card in cards {
            if get_suit(card) == flush_suit {
                flush_ranks[count] = get_rank(card);
                count += 1;
            }
        }
        flush_ranks.sort();
        flush_ranks.reverse();
        return HandRank::Flush { ranks: [flush_ranks[0], flush_ranks[1], flush_ranks[2], flush_ranks[3], flush_ranks[4]] };
    }

    // Straight
    if is_straight {
        return HandRank::Straight { high_card_rank: straight_high_card };
    }
    
    // Three of a Kind
    if threes_count > 0 {
        let mut kickers = [255u8; 2];
        let mut kicker_count = 0;
        for rank in ranks {
            if rank != threes[0] && kicker_count < 2 {
                kickers[kicker_count] = rank;
                kicker_count += 1;
            }
        }
        return HandRank::ThreeOfAKind { three_rank: threes[0], kickers };
    }

    // Two Pair
    if pairs_count >= 2 {
        let mut kicker = 255u8;
        for rank in ranks {
            if rank != pairs[0] && rank != pairs[1] {
                kicker = rank;
                // cannot break
            }
        }
        return HandRank::TwoPair { high_pair_rank: pairs[0], low_pair_rank: pairs[1], kicker_rank: kicker };
    }

    // One Pair
    if pairs_count == 1 {
        let mut kickers = [255u8; 3];
        let mut kicker_count = 0;
        for rank in ranks {
            if rank != pairs[0] && kicker_count < 3 {
                kickers[kicker_count] = rank;
                kicker_count += 1;
            }
        }
        return HandRank::OnePair { pair_rank: pairs[0], kickers };
    }
    
    // High Card
    HandRank::HighCard { ranks: [ranks[0], ranks[1], ranks[2], ranks[3], ranks[4]] }
}
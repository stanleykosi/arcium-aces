//! src/state/table.rs
//!
//! @description
//! This module defines the `Table` account, which is the primary on-chain record
//! for a single poker game. It holds all public state information necessary for
//! players to participate and for the UI to render the game correctly.
//!
//! Key features:
//! - Manages player seating, stacks, and bets.
//! - Tracks the public game state: pot size, community cards, turn order.
//! - Enforces turn timers using on-chain timestamps.
//! - Supports multiple SPL tokens for gameplay.

use anchor_lang::prelude::*;
use crate::state::card::Card;
use crate::state::constants::MAX_PLAYERS;

/// Represents a single poker table.
#[account]
#[derive(InitSpace)]
pub struct Table {
    /// A unique identifier for the table.
    pub table_id: u64,
    /// The public key of the player who created the table.
    pub creator: Pubkey,
    /// The public key of the platform admin, copied from `PlatformConfig` on creation.
    pub admin: Pubkey,
    /// An array representing the seats at the table. `None` signifies an empty seat.
    #[max_len(MAX_PLAYERS)]
    pub seats: Vec<Option<PlayerInfo>>,
    /// The number of players currently seated at the table.
    pub player_count: u8,
    /// The index in the `seats` array corresponding to the player with the dealer button.
    pub dealer_position: u8,
    /// The index in the `seats` array corresponding to the player whose turn it is to act.
    pub turn_position: u8,
    /// The current state of the game (e.g., waiting for players, hand in progress).
    pub game_state: GameState,
    /// The current betting round (e.g., PreFlop, Flop, Turn, River).
    pub betting_round: BettingRound,
    /// The small blind amount.
    pub small_blind: u64,
    /// The big blind amount.
    pub big_blind: u64,
    /// The mint address of the SPL token being used for this table's currency.
    pub token_mint: Pubkey,
    /// The total amount of chips in the main pot for the current hand.
    pub pot: u64,
    /// The current amount a player must call to stay in the hand.
    pub current_bet: u64,
    /// The five community cards. `None` if not yet dealt.
    pub community_cards: [Option<Card>; 5],
    /// The Unix timestamp when the current player's turn started. Used for the turn timer.
    pub turn_started_at: i64,
    /// The duration of a player's turn in seconds.
    pub turn_duration_seconds: u32,
    /// A counter for the number of hands played at this table, used to create unique hand IDs.
    pub hand_id_counter: u64,
}

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

/// Enum representing the possible states of a poker game.
#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    WaitingForPlayers,
    HandInProgress,
    HandComplete,
}

/// Enum representing the different betting rounds in a hand of Texas Hold'em.
#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BettingRound {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}
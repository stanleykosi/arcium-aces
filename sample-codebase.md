Directory structure:
└── blackjack/
    ├── README.md
    ├── Anchor.toml
    ├── Arcium.toml
    ├── Cargo.toml
    ├── package.json
    ├── rust-toolchain
    ├── tsconfig.json
    ├── .prettierignore
    ├── encrypted-ixs/
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs
    ├── migrations/
    │   └── deploy.ts
    ├── programs/
    │   └── blackjack/
    │       ├── Cargo.toml
    │       ├── Xargo.toml
    │       └── src/
    │           └── lib.rs
    └── tests/
        └── blackjack.ts

================================================
FILE: blackjack/README.md
================================================
# Confidential Blackjack on Solana

This example demonstrates a fully confidential blackjack game implemented using Arcium's Multi-Party Computation network. Players can enjoy a complete blackjack experience while keeping all card information private throughout the game.

## How Blackjack Works

Blackjack is a card game where players try to get their hand value as close to 21 as possible without going over (busting). Card values are:

- Number cards (2-10): Face value
- Face cards (Jack, Queen, King): 10 points each  
- Aces: 1 or 11 points (whichever is better for the hand)

The player receives two cards initially and can choose to "hit" (take another card), "stand" (keep current hand), or "double down" (double the bet and take exactly one more card). The dealer follows fixed rules: hit on 16 or less, stand on 17 or more.

## Why Arcium is Essential

Traditional on-chain card games face a fundamental problem: blockchain transparency means all data is public. In blackjack, if card values were visible, players could see the dealer's hole card and upcoming cards in the deck, completely breaking the game's fairness.

Arcium solves this by:

- **Confidential Deck Shuffling**: The 52-card deck is shuffled using cryptographically secure randomness within MPC
- **Private Card Values**: Player and dealer hands remain encrypted throughout gameplay
- **Hidden Information**: Players can't see the dealer's hole card or future cards in the deck
- **Fair Gameplay**: Only necessary information is revealed (like whether a player busted) while maintaining game integrity

## Technical Implementation

### Deck Encoding Innovation

The most complex part of this implementation is efficiently storing a 52-card deck in encrypted form. The solution uses a clever base-64 encoding scheme:

- Each card is represented as a 6-bit value (0-63 range)
- Multiple cards are packed into u128 integers using powers of 64
- The full deck splits across three u128 values for storage efficiency
- Cards 0-20 go in the first u128, cards 21-41 in the second, cards 42-51 in the third

This encoding allows the entire shuffled deck to be stored and manipulated within MPC while remaining completely confidential.

### Game Flow

1. **Initialization**: Player creates a game session and the deck is shuffled in MPC
2. **Deal**: Initial cards are dealt (2 to player, 2 to dealer with 1 face up)  
3. **Player Turn**: Player can hit, stand, or double down based on their encrypted hand
4. **Dealer Turn**: Dealer follows standard rules within MPC computation
5. **Resolution**: Final hand comparison determines the winner

### MPC Operations

Each game action triggers a specific MPC computation:

- `shuffle_and_deal_cards`: Initial deck shuffle and card dealing
- `player_hit`: Drawing additional cards for the player
- `player_stand`: Checking if player's current hand is valid
- `player_double_down`: Taking exactly one more card with doubled stakes
- `dealer_play`: Dealer follows hitting rules until reaching 17+
- `resolve_game`: Final comparison to determine the winner

All computations maintain card confidentiality while revealing only the minimum information needed for gameplay.

## Project Structure

**In order to build this project, cargo will require access to the arcium registry where the arcium dependencies are published to.
This is done by editing the generated `.cargo/credentials.toml` file to the root of the project with the provided token.**

The project follows Arcium's standard structure:

- `programs/blackjack/` - Solana program handling game state and user interactions
- `encrypted-ixs/` - MPC computations for confidential card operations  
- `tests/` - Integration tests demonstrating complete game flows
- `app/` - Frontend application for playing the game

The confidential computations in `encrypted-ixs/` handle all card-related logic while the Solana program manages game sessions, player accounts, and state transitions.



================================================
FILE: blackjack/Anchor.toml
================================================
[toolchain]
package_manager = "yarn"

[features]
resolution = true
skip-lint = false

[programs.localnet]
blackjack = "A7sNeBnrQAFxmj6BVmoYC6PYnebURaar7xhKuaEyRh4j"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "localnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"



================================================
FILE: blackjack/Arcium.toml
================================================
[localnet]
# number of nodes in the single cluster of the localnet
nodes = 2
# number of seconds to wait for the localnet to come online
localnet_timeout_secs = 60



================================================
FILE: blackjack/Cargo.toml
================================================
[workspace]
members = ["programs/*", "encrypted-ixs"]
resolver = "2"

[profile.release]
overflow-checks = true
lto = "fat"
codegen-units = 1
[profile.release.build-override]
opt-level = 3
incremental = false
codegen-units = 1



================================================
FILE: blackjack/package.json
================================================
{
  "license": "ISC",
  "scripts": {
    "lint:fix": "prettier */*.js \"*/**/*{.js,.ts}\" -w",
    "lint": "prettier */*.js \"*/**/*{.js,.ts}\" --check"
  },
  "dependencies": {
    "@coral-xyz/anchor": "^0.31.1",
    "@arcium-hq/client": "0.2.0"
  },
  "devDependencies": {
    "chai": "^4.3.4",
    "mocha": "^9.0.3",
    "ts-mocha": "^10.0.0",
    "@types/bn.js": "^5.1.0",
    "@types/chai": "^4.3.0",
    "@types/mocha": "^9.0.0",
    "typescript": "^4.3.5",
    "prettier": "^2.6.2"
  }
}



================================================
FILE: blackjack/rust-toolchain
================================================
1.85.0



================================================
FILE: blackjack/tsconfig.json
================================================
{
  "compilerOptions": {
    "types": ["mocha", "chai"],
    "typeRoots": ["./node_modules/@types"],
    "lib": ["es2020"],
    "module": "commonjs",
    "target": "es2020",
    "esModuleInterop": true,
    "resolveJsonModule": true
  }
}



================================================
FILE: blackjack/.prettierignore
================================================
.anchor
.DS_Store
target
node_modules
dist
build
test-ledger



================================================
FILE: blackjack/encrypted-ixs/Cargo.toml
================================================
[package]
name = "encrypted-ixs"
version = "0.1.0"
edition = "2021"

[dependencies]
arcis-imports = { version = "0.2.0" }


================================================
FILE: blackjack/encrypted-ixs/src/lib.rs
================================================
use arcis_imports::*;

#[encrypted]
mod circuits {
    use arcis_imports::*;

    /// Standard 52-card deck represented as indices 0-51
    const INITIAL_DECK: [u8; 52] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51,
    ];

    /// Powers of 64 used for encoding cards into u128 values.
    /// Each card takes 6 bits (values 0-63), so we can pack multiple cards efficiently.
    /// This array contains 64^i for i in 0..21, allowing us to encode up to 21 cards per u128.
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

    /// Represents a full 52-card deck encoded into three u128 values for efficiency.
    ///
    /// Each card is represented by 6 bits (0-63 range), allowing us to pack:
    /// - Cards 0-20 in card_one (21 cards × 6 bits = 126 bits < 128 bits)
    /// - Cards 21-41 in card_two (21 cards × 6 bits = 126 bits < 128 bits)  
    /// - Cards 42-51 in card_three (10 cards × 6 bits = 60 bits < 128 bits)
    pub struct Deck {
        pub card_one: u128,
        pub card_two: u128,
        pub card_three: u128,
    }

    impl Deck {
        /// Converts a 52-card array into the packed Deck representation.
        /// Uses base-64 encoding where each card index is treated as a digit in base 64.
        pub fn from_array(array: [u8; 52]) -> Deck {
            let mut card_one = 0;
            for i in 0..21 {
                card_one += POWS_OF_SIXTY_FOUR[i] * array[i] as u128;
            }

            let mut card_two = 0;
            for i in 21..42 {
                card_two += POWS_OF_SIXTY_FOUR[i - 21] * array[i] as u128;
            }

            let mut card_three = 0;
            for i in 42..52 {
                card_three += POWS_OF_SIXTY_FOUR[i - 42] * array[i] as u128;
            }

            Deck {
                card_one,
                card_two,
                card_three,
            }
        }

        /// Converts the packed Deck representation back to a 52-card array.
        /// Reverses the base-64 encoding by extracting 6 bits at a time.
        fn to_array(&self) -> [u8; 52] {
            let mut card_one = self.card_one;
            let mut card_two = self.card_two;
            let mut card_three = self.card_three;

            let mut bytes = [0u8; 52];
            for i in 0..21 {
                bytes[i] = (card_one % 64) as u8;
                bytes[i + 21] = (card_two % 64) as u8;
                card_one >>= 6;
                card_two >>= 6;
            }

            for i in 42..52 {
                bytes[i] = (card_three % 64) as u8;
                card_three >>= 6;
            }

            bytes
        }
    }

    // Initial hand is 2 player cards and 2 dealer cards (1 face up, 1 face down)
    pub struct InitialHandVisible {
        pub player_card_one: u8,
        pub player_card_two: u8,
        pub dealer_card_one: u8,
    }

    pub struct Hand {
        pub cards: u128,
    }

    impl Hand {
        pub fn from_array(array: [u8; 11]) -> Hand {
            let mut cards = 0;
            for i in 0..11 {
                cards += POWS_OF_SIXTY_FOUR[i] * array[i] as u128;
            }

            Hand { cards }
        }

        fn to_array(&self) -> [u8; 11] {
            let mut cards = self.cards;

            let mut bytes = [0u8; 11];
            for i in 0..11 {
                bytes[i] = (cards % 64) as u8;
                cards >>= 6;
            }

            bytes
        }
    }

    #[instruction]
    pub fn shuffle_and_deal_cards(
        mxe: Mxe,
        mxe_again: Mxe,
        client: Shared,
        client_again: Shared,
    ) -> (
        Enc<Mxe, Deck>,    // 16 + 32 x 3
        Enc<Mxe, Hand>,    // 16 + 32
        Enc<Shared, Hand>, // 32 + 16 + 32
        Enc<Shared, u8>,   // 32 + 16 + 32
    ) {
        let mut initial_deck = INITIAL_DECK;
        ArcisRNG::shuffle(&mut initial_deck);

        let deck = mxe.from_arcis(Deck::from_array(initial_deck));

        let mut dealer_cards = [53; 11];
        dealer_cards[0] = initial_deck[1];
        dealer_cards[1] = initial_deck[3];

        let dealer_hand = mxe_again.from_arcis(Hand::from_array(dealer_cards));

        let mut player_cards = [53; 11];
        player_cards[0] = initial_deck[0];
        player_cards[1] = initial_deck[2];

        let player_hand = client.from_arcis(Hand::from_array(player_cards));

        (
            deck,
            dealer_hand,
            player_hand,
            client_again.from_arcis(initial_deck[1]),
        )
    }

    #[instruction]
    pub fn player_hit(
        deck_ctxt: Enc<Mxe, Deck>,
        player_hand_ctxt: Enc<Shared, Hand>,
        player_hand_size: u8,
        dealer_hand_size: u8,
    ) -> (Enc<Shared, Hand>, bool) {
        let deck = deck_ctxt.to_arcis().to_array();

        let mut player_hand = player_hand_ctxt.to_arcis().to_array();

        let player_hand_value = calculate_hand_value(&player_hand, player_hand_size);

        let is_bust = player_hand_value > 21;

        let new_card = if !is_bust {
            let card_index = (player_hand_size + dealer_hand_size) as usize;

            // Get the next card from the deck
            deck[card_index]
        } else {
            53
        };

        player_hand[player_hand_size as usize] = new_card;

        let player_updated_hand_value = calculate_hand_value(&player_hand, player_hand_size + 1);

        (
            player_hand_ctxt
                .owner
                .from_arcis(Hand::from_array(player_hand)),
            is_bust.reveal(),
        )
    }

    // Returns true if the player has busted
    #[instruction]
    pub fn player_stand(player_hand_ctxt: Enc<Shared, Hand>, player_hand_size: u8) -> bool {
        let player_hand = player_hand_ctxt.to_arcis().to_array();
        let value = calculate_hand_value(&player_hand, player_hand_size);
        (value > 21).reveal()
    }

    // Returns true if the player has busted, if not, returns the new card
    #[instruction]
    pub fn player_double_down(
        deck_ctxt: Enc<Mxe, Deck>,
        player_hand_ctxt: Enc<Shared, Hand>,
        player_hand_size: u8,
        dealer_hand_size: u8,
    ) -> (Enc<Shared, Hand>, bool) {
        let deck = deck_ctxt.to_arcis();
        let deck_array = deck.to_array();

        let mut player_hand = player_hand_ctxt.to_arcis().to_array();

        let player_hand_value = calculate_hand_value(&player_hand, player_hand_size);

        let is_bust = player_hand_value > 21;

        let new_card = if !is_bust {
            let card_index = (player_hand_size + dealer_hand_size) as usize;

            // Get the next card from the deck
            deck_array[card_index]
        } else {
            53
        };

        player_hand[player_hand_size as usize] = new_card;

        (
            player_hand_ctxt
                .owner
                .from_arcis(Hand::from_array(player_hand)),
            is_bust.reveal(),
        )
    }

    // Function for dealer to play (reveal hole card and follow rules)
    #[instruction]
    pub fn dealer_play(
        deck_ctxt: Enc<Mxe, Deck>,
        dealer_hand_ctxt: Enc<Mxe, Hand>,
        client: Shared,
        player_hand_size: u8,
        dealer_hand_size: u8,
    ) -> (Enc<Mxe, Hand>, Enc<Shared, Hand>, u8) {
        let deck = deck_ctxt.to_arcis();
        let mut deck_array = deck.to_array();
        let mut dealer = dealer_hand_ctxt.to_arcis().to_array();
        let mut size = dealer_hand_size as usize;

        for i in 0..7 {
            let val = calculate_hand_value(&dealer, size as u8);
            if val < 17 {
                let idx = (player_hand_size as usize + size) as usize;
                dealer[size] = deck_array[idx];
                size += 1;
            }
        }

        (
            dealer_hand_ctxt.owner.from_arcis(Hand::from_array(dealer)),
            client.from_arcis(Hand::from_array(dealer)),
            (size as u8).reveal(),
        )
    }

    /// Calculates the blackjack value of a hand according to standard rules.
    ///
    /// Card values: Ace = 1 or 11 (whichever is better), Face cards = 10, Others = face value.
    /// Aces are initially valued at 11, but automatically reduced to 1 if the hand would bust.
    ///
    /// # Arguments
    /// * `hand` - Array of up to 11 cards (more than enough for blackjack)
    /// * `hand_length` - Number of actual cards in the hand
    ///
    /// # Returns
    /// The total value of the hand (1-21, or >21 if busted)
    fn calculate_hand_value(hand: &[u8; 11], hand_length: u8) -> u8 {
        let mut value = 0;
        let mut has_ace = false;

        // Process each card in the hand
        for i in 0..11 {
            let rank = if i < hand_length as usize {
                (hand[i] % 13) // Card rank (0=Ace, 1-9=pip cards, 10-12=face cards)
            } else {
                0
            };

            if i < hand_length as usize {
                if rank == 0 {
                    // Ace: start with value of 11
                    value += 11;
                    has_ace = true;
                } else if rank > 10 {
                    // Face cards (Jack, Queen, King): value of 10
                    value += 10;
                } else {
                    // Pip cards (2-10): face value (rank 1-9 becomes value 1-9)
                    value += rank;
                }
            }
        }

        // Convert Ace from 11 to 1 if hand would bust with 11
        if value > 21 && has_ace {
            value -= 10;
        }

        value
    }

    /// Determines the final winner of the blackjack game.
    ///
    /// Compares the final hand values according to blackjack rules and returns
    /// a numeric result indicating the outcome. Both hands are evaluated for busts
    /// and compared for the winner.
    ///
    /// # Returns
    /// * 0 = Player busts (dealer wins)
    /// * 1 = Dealer busts (player wins)
    /// * 2 = Player wins (higher value, no bust)
    /// * 3 = Dealer wins (higher value, no bust)
    /// * 4 = Push/tie (same value, no bust)
    #[instruction]
    pub fn resolve_game(
        player_hand: Enc<Shared, Hand>,
        dealer_hand: Enc<Mxe, Hand>,
        player_hand_length: u8,
        dealer_hand_length: u8,
    ) -> u8 {
        let player_hand = player_hand.to_arcis().to_array();
        let dealer_hand = dealer_hand.to_arcis().to_array();

        // Calculate final hand values
        let player_value = calculate_hand_value(&player_hand, player_hand_length);
        let dealer_value = calculate_hand_value(&dealer_hand, dealer_hand_length);

        // Apply blackjack rules to determine winner
        let result = if player_value > 21 {
            0 // Player busts - dealer wins automatically
        } else if dealer_value > 21 {
            1 // Dealer busts - player wins automatically
        } else if player_value > dealer_value {
            2 // Player has higher value without busting
        } else if dealer_value > player_value {
            3 // Dealer has higher value without busting
        } else {
            4 // Equal values - push (tie)
        };

        result.reveal()
    }
}



================================================
FILE: blackjack/migrations/deploy.ts
================================================
// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

import * as anchor from "@coral-xyz/anchor";

module.exports = async function (provider: anchor.AnchorProvider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here.
};



================================================
FILE: blackjack/programs/blackjack/Cargo.toml
================================================
[package]
name = "blackjack"
version = "0.1.0"
description = "Created with Arcium & Anchor"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]
name = "blackjack"

[features]
default = []
cpi = ["no-entrypoint"]
no-entrypoint = []
no-idl = []
no-log-ix-name = []
idl-build = ["anchor-lang/idl-build"]

[dependencies]
anchor-lang = "0.31.1"

arcium-client = { version = "0.2.0", default-features = false }
arcium-macros = { version = "0.2.0" }
arcium-anchor = { version = "0.2.0" }



================================================
FILE: blackjack/programs/blackjack/Xargo.toml
================================================
[target.bpfel-unknown-unknown.dependencies.std]
features = []



================================================
FILE: blackjack/programs/blackjack/src/lib.rs
================================================
use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;
use arcium_client::idl::arcium::types::CallbackAccount;

const COMP_DEF_OFFSET_SHUFFLE_AND_DEAL_CARDS: u32 = comp_def_offset("shuffle_and_deal_cards");
const COMP_DEF_OFFSET_PLAYER_HIT: u32 = comp_def_offset("player_hit");
const COMP_DEF_OFFSET_PLAYER_DOUBLE_DOWN: u32 = comp_def_offset("player_double_down");
const COMP_DEF_OFFSET_PLAYER_STAND: u32 = comp_def_offset("player_stand");
const COMP_DEF_OFFSET_DEALER_PLAY: u32 = comp_def_offset("dealer_play");
const COMP_DEF_OFFSET_RESOLVE_GAME: u32 = comp_def_offset("resolve_game");

declare_id!("A7sNeBnrQAFxmj6BVmoYC6PYnebURaar7xhKuaEyRh4j");

#[arcium_program]
pub mod blackjack {
    use super::*;

    /// Initializes the computation definition for shuffling and dealing cards.
    /// This sets up the MPC environment for the initial deck shuffle and card dealing operation.
    pub fn init_shuffle_and_deal_cards_comp_def(
        ctx: Context<InitShuffleAndDealCardsCompDef>,
    ) -> Result<()> {
        init_comp_def(ctx.accounts, true, 0, None, None)?;
        Ok(())
    }

    /// Creates a new blackjack game session and initiates the deck shuffle.
    ///
    /// This function sets up a new game account with initial state and triggers the MPC computation
    /// to shuffle a standard 52-card deck and deal the opening hands (2 cards each to player and dealer).
    /// The actual shuffling and dealing happens confidentially within the Arcium network.
    ///
    /// # Arguments
    /// * `game_id` - Unique identifier for this game session
    /// * `mxe_nonce` - Cryptographic nonce for MXE operations  
    /// * `client_pubkey` - Player's encryption public key for receiving encrypted cards
    /// * `client_nonce` - Player's cryptographic nonce for encryption operations
    pub fn initialize_blackjack_game(
        ctx: Context<InitializeBlackjackGame>,
        computation_offset: u64,
        game_id: u64,
        mxe_nonce: u128,
        mxe_again_nonce: u128,
        client_pubkey: [u8; 32],
        client_nonce: u128,
        client_again_nonce: u128,
    ) -> Result<()> {
        // Initialize the blackjack game account
        let blackjack_game = &mut ctx.accounts.blackjack_game;
        blackjack_game.bump = ctx.bumps.blackjack_game;
        blackjack_game.game_id = game_id;
        blackjack_game.player_pubkey = ctx.accounts.payer.key();
        blackjack_game.player_hand = [0; 32];
        blackjack_game.dealer_hand = [0; 32];
        blackjack_game.deck_nonce = 0;
        blackjack_game.client_nonce = 0;
        blackjack_game.dealer_nonce = 0;
        blackjack_game.player_enc_pubkey = client_pubkey;
        blackjack_game.game_state = GameState::Initial;
        blackjack_game.player_hand_size = 0;
        blackjack_game.dealer_hand_size = 0;

        // Queue the shuffle and deal cards computation
        let args = vec![
            Argument::PlaintextU128(mxe_nonce),
            Argument::PlaintextU128(mxe_again_nonce),
            Argument::ArcisPubkey(client_pubkey),
            Argument::PlaintextU128(client_nonce),
            Argument::ArcisPubkey(client_pubkey),
            Argument::PlaintextU128(client_again_nonce),
        ];

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            vec![CallbackAccount {
                pubkey: ctx.accounts.blackjack_game.key(),
                is_writable: true,
            }],
            None,
        )?;
        Ok(())
    }

    /// Handles the result of the shuffle and deal cards MPC computation.
    ///
    /// This callback processes the shuffled deck and dealt cards from the MPC computation.
    /// It updates the game state with the new deck, initial hands, and sets the game to PlayerTurn.
    /// The player receives their encrypted hand while the dealer gets one face-up card visible to the player.
    #[arcium_callback(encrypted_ix = "shuffle_and_deal_cards")]
    pub fn shuffle_and_deal_cards_callback(
        ctx: Context<ShuffleAndDealCardsCallback>,
        output: ComputationOutputs<ShuffleAndDealCardsOutput>,
    ) -> Result<()> {
        let o = match output {
            ComputationOutputs::Success(ShuffleAndDealCardsOutput {
                field_0:
                    ShuffleAndDealCardsTupleStruct0 {
                        field_0: deck,
                        field_1: dealer_hand,
                        field_2: player_hand,
                        field_3: dealer_face_up_card,
                    },
            }) => (deck, dealer_hand, player_hand, dealer_face_up_card),
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        let deck_nonce = o.0.nonce;

        let deck: [[u8; 32]; 3] = o.0.ciphertexts;

        let dealer_nonce = o.1.nonce;

        let dealer_hand: [u8; 32] = o.1.ciphertexts[0];

        let client_pubkey: [u8; 32] = o.2.encryption_key;

        let client_nonce = o.2.nonce;

        let player_hand: [u8; 32] = o.2.ciphertexts[0];

        let dealer_client_pubkey: [u8; 32] = o.3.encryption_key;

        let dealer_client_nonce = o.3.nonce;

        let dealer_face_up_card: [u8; 32] = o.3.ciphertexts[0];

        // Update the blackjack game account
        let blackjack_game = &mut ctx.accounts.blackjack_game;
        blackjack_game.deck = deck;
        blackjack_game.deck_nonce = deck_nonce;
        blackjack_game.client_nonce = client_nonce;
        blackjack_game.dealer_nonce = dealer_nonce;
        blackjack_game.player_enc_pubkey = client_pubkey;
        blackjack_game.game_state = GameState::PlayerTurn; // It is now the player's turn

        require!(
            dealer_client_pubkey == blackjack_game.player_enc_pubkey,
            ErrorCode::InvalidDealerClientPubkey
        );

        // Initialize player hand with first two cards
        blackjack_game.player_hand = player_hand;
        // Initialize dealer hand with face up card and face down card
        blackjack_game.dealer_hand = dealer_hand;
        blackjack_game.player_hand_size = 2;
        blackjack_game.dealer_hand_size = 2;

        emit!(CardsShuffledAndDealtEvent {
            client_nonce,
            dealer_client_nonce,
            player_hand,
            dealer_face_up_card,
            game_id: blackjack_game.game_id,
        });
        Ok(())
    }
    pub fn init_player_hit_comp_def(ctx: Context<InitPlayerHitCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, true, 0, None, None)?;
        Ok(())
    }

    /// Allows the player to request an additional card (hit).
    ///
    /// This triggers an MPC computation that draws the next card from the shuffled deck
    /// and adds it to the player's hand. The computation also checks if the player busts (exceeds 21)
    /// and returns this information while keeping the actual card values encrypted.
    pub fn player_hit(
        ctx: Context<PlayerHit>,
        computation_offset: u64,
        _game_id: u64,
    ) -> Result<()> {
        require!(
            ctx.accounts.blackjack_game.game_state == GameState::PlayerTurn,
            ErrorCode::InvalidGameState
        );
        require!(
            !ctx.accounts.blackjack_game.player_has_stood,
            ErrorCode::InvalidMove
        );

        let args = vec![
            // Deck
            Argument::PlaintextU128(ctx.accounts.blackjack_game.deck_nonce),
            Argument::Account(ctx.accounts.blackjack_game.key(), 8, 32 * 3),
            // Player hand
            Argument::ArcisPubkey(ctx.accounts.blackjack_game.player_enc_pubkey),
            Argument::PlaintextU128(ctx.accounts.blackjack_game.client_nonce),
            Argument::Account(ctx.accounts.blackjack_game.key(), 8 + 32 * 3, 32),
            // Player hand size
            Argument::PlaintextU8(ctx.accounts.blackjack_game.player_hand_size),
            // Dealer hand size
            Argument::PlaintextU8(ctx.accounts.blackjack_game.dealer_hand_size),
        ];

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            vec![CallbackAccount {
                pubkey: ctx.accounts.blackjack_game.key(),
                is_writable: true,
            }],
            None,
        )?;
        Ok(())
    }

    #[arcium_callback(encrypted_ix = "player_hit")]
    pub fn player_hit_callback(
        ctx: Context<PlayerHitCallback>,
        output: ComputationOutputs<PlayerHitOutput>,
    ) -> Result<()> {
        let o = match output {
            ComputationOutputs::Success(PlayerHitOutput {
                field_0:
                    PlayerHitTupleStruct0 {
                        field_0: player_hand,
                        field_1: is_bust,
                    },
            }) => (player_hand, is_bust),
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        let client_nonce = o.0.nonce;

        let player_hand: [u8; 32] = o.0.ciphertexts[0];

        let is_bust: bool = o.1;

        let blackjack_game = &mut ctx.accounts.blackjack_game;
        blackjack_game.player_hand = player_hand;
        blackjack_game.client_nonce = client_nonce;

        if is_bust {
            blackjack_game.game_state = GameState::DealerTurn;
            emit!(PlayerBustEvent {
                client_nonce,
                game_id: blackjack_game.game_id,
            });
        } else {
            blackjack_game.game_state = GameState::PlayerTurn;
            emit!(PlayerHitEvent {
                player_hand,
                client_nonce,
                game_id: blackjack_game.game_id,
            });
            blackjack_game.player_hand_size += 1;
        }

        Ok(())
    }

    pub fn init_player_double_down_comp_def(
        ctx: Context<InitPlayerDoubleDownCompDef>,
    ) -> Result<()> {
        init_comp_def(ctx.accounts, true, 0, None, None)?;
        Ok(())
    }

    pub fn player_double_down(
        ctx: Context<PlayerDoubleDown>,
        computation_offset: u64,
        _game_id: u64,
    ) -> Result<()> {
        require!(
            ctx.accounts.blackjack_game.game_state == GameState::PlayerTurn,
            ErrorCode::InvalidGameState
        );
        require!(
            !ctx.accounts.blackjack_game.player_has_stood,
            ErrorCode::InvalidMove
        );

        let args = vec![
            // Deck
            Argument::PlaintextU128(ctx.accounts.blackjack_game.deck_nonce),
            Argument::Account(ctx.accounts.blackjack_game.key(), 8, 32 * 3),
            // Player hand
            Argument::ArcisPubkey(ctx.accounts.blackjack_game.player_enc_pubkey),
            Argument::PlaintextU128(ctx.accounts.blackjack_game.client_nonce),
            Argument::Account(ctx.accounts.blackjack_game.key(), 8 + 32 * 3, 32),
            // Player hand size
            Argument::PlaintextU8(ctx.accounts.blackjack_game.player_hand_size),
            // Dealer hand size
            Argument::PlaintextU8(ctx.accounts.blackjack_game.dealer_hand_size),
        ];

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            vec![CallbackAccount {
                pubkey: ctx.accounts.blackjack_game.key(),
                is_writable: true,
            }],
            None,
        )?;
        Ok(())
    }

    #[arcium_callback(encrypted_ix = "player_double_down")]
    #[arcium_callback(encrypted_ix = "player_double_down")]
    pub fn player_double_down_callback(
        ctx: Context<PlayerDoubleDownCallback>,
        output: ComputationOutputs<PlayerDoubleDownOutput>,
    ) -> Result<()> {
        let o = match output {
            ComputationOutputs::Success(PlayerDoubleDownOutput {
                field_0:
                    PlayerDoubleDownTupleStruct0 {
                        field_0: player_hand,
                        field_1: is_bust,
                    },
            }) => (player_hand, is_bust),
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        let client_nonce = o.0.nonce;

        let player_hand: [u8; 32] = o.0.ciphertexts[0];

        let is_bust: bool = o.1;

        let blackjack_game = &mut ctx.accounts.blackjack_game;
        blackjack_game.player_hand = player_hand;
        blackjack_game.client_nonce = client_nonce;
        blackjack_game.player_has_stood = true;

        if is_bust {
            blackjack_game.game_state = GameState::DealerTurn;
            emit!(PlayerBustEvent {
                client_nonce,
                game_id: blackjack_game.game_id,
            });
        } else {
            blackjack_game.game_state = GameState::DealerTurn;
            emit!(PlayerDoubleDownEvent {
                player_hand,
                client_nonce,
                game_id: blackjack_game.game_id,
            });
        }

        Ok(())
    }

    pub fn init_player_stand_comp_def(ctx: Context<InitPlayerStandCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, true, 0, None, None)?;
        Ok(())
    }

    pub fn player_stand(
        ctx: Context<PlayerStand>,
        computation_offset: u64,
        _game_id: u64,
    ) -> Result<()> {
        require!(
            ctx.accounts.blackjack_game.game_state == GameState::PlayerTurn,
            ErrorCode::InvalidGameState
        );
        require!(
            !ctx.accounts.blackjack_game.player_has_stood,
            ErrorCode::InvalidMove
        );

        let args = vec![
            // Player hand
            Argument::ArcisPubkey(ctx.accounts.blackjack_game.player_enc_pubkey),
            Argument::PlaintextU128(ctx.accounts.blackjack_game.client_nonce),
            Argument::Account(ctx.accounts.blackjack_game.key(), 8 + 32 * 3, 32),
            // Player hand size
            Argument::PlaintextU8(ctx.accounts.blackjack_game.player_hand_size),
        ];

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            vec![CallbackAccount {
                pubkey: ctx.accounts.blackjack_game.key(),
                is_writable: true,
            }],
            None,
        )?;
        Ok(())
    }

    #[arcium_callback(encrypted_ix = "player_stand")]
    pub fn player_stand_callback(
        ctx: Context<PlayerStandCallback>,
        output: ComputationOutputs<PlayerStandOutput>,
    ) -> Result<()> {
        let is_bust = match output {
            ComputationOutputs::Success(PlayerStandOutput { field_0 }) => field_0,
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        let blackjack_game = &mut ctx.accounts.blackjack_game;
        blackjack_game.player_has_stood = true;

        if is_bust {
            // This should never happen
            blackjack_game.game_state = GameState::PlayerTurn;
            emit!(PlayerBustEvent {
                client_nonce: blackjack_game.client_nonce,
                game_id: blackjack_game.game_id,
            });
        } else {
            blackjack_game.game_state = GameState::DealerTurn;
            emit!(PlayerStandEvent {
                is_bust,
                game_id: blackjack_game.game_id
            });
        }

        Ok(())
    }

    pub fn init_dealer_play_comp_def(ctx: Context<InitDealerPlayCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, true, 0, None, None)?;
        Ok(())
    }

    pub fn dealer_play(
        ctx: Context<DealerPlay>,
        computation_offset: u64,
        _game_id: u64,
        nonce: u128,
    ) -> Result<()> {
        require!(
            ctx.accounts.blackjack_game.game_state == GameState::DealerTurn,
            ErrorCode::InvalidGameState
        );

        let args = vec![
            // Deck
            Argument::PlaintextU128(ctx.accounts.blackjack_game.deck_nonce),
            Argument::Account(ctx.accounts.blackjack_game.key(), 8, 32 * 3),
            // Dealer hand
            Argument::PlaintextU128(ctx.accounts.blackjack_game.dealer_nonce),
            Argument::Account(ctx.accounts.blackjack_game.key(), 8 + 32 * 3 + 32, 32),
            // Client nonce
            Argument::ArcisPubkey(ctx.accounts.blackjack_game.player_enc_pubkey),
            Argument::PlaintextU128(nonce),
            // Player hand size
            Argument::PlaintextU8(ctx.accounts.blackjack_game.player_hand_size),
            // Dealer hand size
            Argument::PlaintextU8(ctx.accounts.blackjack_game.dealer_hand_size),
        ];

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            vec![CallbackAccount {
                pubkey: ctx.accounts.blackjack_game.key(),
                is_writable: true,
            }],
            None,
        )?;
        Ok(())
    }

    #[arcium_callback(encrypted_ix = "dealer_play")]
    pub fn dealer_play_callback(
        ctx: Context<DealerPlayCallback>,
        output: ComputationOutputs<DealerPlayOutput>,
    ) -> Result<()> {
        let o = match output {
            ComputationOutputs::Success(DealerPlayOutput {
                field_0:
                    DealerPlayTupleStruct0 {
                        field_0: dealer_hand,
                        field_1: dealer_client_hand,
                        field_2: dealer_hand_size,
                    },
            }) => (dealer_hand, dealer_client_hand, dealer_hand_size),
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        let dealer_nonce = o.0.nonce;
        let dealer_hand = o.0.ciphertexts[0];
        let dealer_client_hand = o.1.ciphertexts[0];
        let dealer_hand_size = o.2;
        let client_nonce = o.1.nonce;

        let blackjack_game = &mut ctx.accounts.blackjack_game;
        blackjack_game.dealer_hand = dealer_hand;
        blackjack_game.dealer_nonce = dealer_nonce;
        blackjack_game.dealer_hand_size = dealer_hand_size;
        blackjack_game.game_state = GameState::Resolving;

        emit!(DealerPlayEvent {
            dealer_hand: dealer_client_hand,
            dealer_hand_size,
            client_nonce,
            game_id: ctx.accounts.blackjack_game.game_id,
        });

        Ok(())
    }

    pub fn init_resolve_game_comp_def(ctx: Context<InitResolveGameCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, true, 0, None, None)?;
        Ok(())
    }

    pub fn resolve_game(
        ctx: Context<ResolveGame>,
        computation_offset: u64,
        _game_id: u64,
    ) -> Result<()> {
        require!(
            ctx.accounts.blackjack_game.game_state == GameState::Resolving,
            ErrorCode::InvalidGameState
        );

        let args = vec![
            // Player hand
            Argument::ArcisPubkey(ctx.accounts.blackjack_game.player_enc_pubkey),
            Argument::PlaintextU128(ctx.accounts.blackjack_game.client_nonce),
            Argument::Account(ctx.accounts.blackjack_game.key(), 8 + 32 * 3, 32),
            // Dealer hand
            Argument::PlaintextU128(ctx.accounts.blackjack_game.dealer_nonce),
            Argument::Account(ctx.accounts.blackjack_game.key(), 8 + 32 * 3 + 32, 32),
            // Player hand size
            Argument::PlaintextU8(ctx.accounts.blackjack_game.player_hand_size),
            // Dealer hand size
            Argument::PlaintextU8(ctx.accounts.blackjack_game.dealer_hand_size),
        ];

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            vec![CallbackAccount {
                pubkey: ctx.accounts.blackjack_game.key(),
                is_writable: true,
            }],
            None,
        )?;
        Ok(())
    }

    #[arcium_callback(encrypted_ix = "resolve_game")]
    pub fn resolve_game_callback(
        ctx: Context<ResolveGameCallback>,
        output: ComputationOutputs<ResolveGameOutput>,
    ) -> Result<()> {
        let result = match output {
            ComputationOutputs::Success(ResolveGameOutput { field_0 }) => field_0,
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        if result == 0 {
            // Player busts (dealer wins)
            emit!(ResultEvent {
                winner: "Dealer".to_string(),
                game_id: ctx.accounts.blackjack_game.game_id,
            });
        } else if result == 1 {
            // Dealer busts (player wins)
            emit!(ResultEvent {
                winner: "Player".to_string(),
                game_id: ctx.accounts.blackjack_game.game_id,
            });
        } else if result == 2 {
            // Player wins
            emit!(ResultEvent {
                winner: "Player".to_string(),
                game_id: ctx.accounts.blackjack_game.game_id,
            });
        } else if result == 3 {
            // Dealer wins
            emit!(ResultEvent {
                winner: "Dealer".to_string(),
                game_id: ctx.accounts.blackjack_game.game_id,
            });
        } else {
            // Push (tie)
            emit!(ResultEvent {
                winner: "Tie".to_string(),
                game_id: ctx.accounts.blackjack_game.game_id,
            });
        }

        let blackjack_game = &mut ctx.accounts.blackjack_game;
        blackjack_game.game_state = GameState::Resolved;

        Ok(())
    }
}

#[queue_computation_accounts("shuffle_and_deal_cards", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64, game_id: u64)]
pub struct InitializeBlackjackGame<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Account<'info, MXEAccount>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_SHUFFLE_AND_DEAL_CARDS)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS,
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        init,
        payer = payer,
        space = 8 + BlackjackGame::INIT_SPACE,
        seeds = [b"blackjack_game".as_ref(), game_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub blackjack_game: Account<'info, BlackjackGame>,
}

#[callback_accounts("shuffle_and_deal_cards", payer)]
#[derive(Accounts)]
pub struct ShuffleAndDealCardsCallback<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_SHUFFLE_AND_DEAL_CARDS)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,
    #[account(mut)]
    pub blackjack_game: Account<'info, BlackjackGame>,
}

#[init_computation_definition_accounts("shuffle_and_deal_cards", payer)]
#[derive(Accounts)]
pub struct InitShuffleAndDealCardsCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by arcium program.
    /// Can't check it here as it's not initialized yet.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

#[queue_computation_accounts("player_hit", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64, _game_id: u64)]
pub struct PlayerHit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Account<'info, MXEAccount>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_PLAYER_HIT)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS,
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        mut,
        seeds = [b"blackjack_game".as_ref(), _game_id.to_le_bytes().as_ref()],
        bump = blackjack_game.bump,
    )]
    pub blackjack_game: Account<'info, BlackjackGame>,
}

#[callback_accounts("player_hit", payer)]
#[derive(Accounts)]
pub struct PlayerHitCallback<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_PLAYER_HIT)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,
    #[account(mut)]
    pub blackjack_game: Account<'info, BlackjackGame>,
}

#[init_computation_definition_accounts("player_hit", payer)]
#[derive(Accounts)]
pub struct InitPlayerHitCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by arcium program.
    /// Can't check it here as it's not initialized yet.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

#[queue_computation_accounts("player_double_down", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64, _game_id: u64)]
pub struct PlayerDoubleDown<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Account<'info, MXEAccount>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_PLAYER_DOUBLE_DOWN)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS,
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        mut,
        seeds = [b"blackjack_game".as_ref(), _game_id.to_le_bytes().as_ref()],
        bump = blackjack_game.bump,
    )]
    pub blackjack_game: Account<'info, BlackjackGame>,
}

#[callback_accounts("player_double_down", payer)]
#[derive(Accounts)]
pub struct PlayerDoubleDownCallback<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_PLAYER_DOUBLE_DOWN)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,
    #[account(mut)]
    pub blackjack_game: Account<'info, BlackjackGame>,
}

#[init_computation_definition_accounts("player_double_down", payer)]
#[derive(Accounts)]
pub struct InitPlayerDoubleDownCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by arcium program.
    /// Can't check it here as it's not initialized yet.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

#[queue_computation_accounts("player_stand", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64, _game_id: u64)]
pub struct PlayerStand<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Account<'info, MXEAccount>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_PLAYER_STAND)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS,
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        mut,
        seeds = [b"blackjack_game".as_ref(), _game_id.to_le_bytes().as_ref()],
        bump = blackjack_game.bump,
    )]
    pub blackjack_game: Account<'info, BlackjackGame>,
}

#[callback_accounts("player_stand", payer)]
#[derive(Accounts)]
pub struct PlayerStandCallback<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_PLAYER_STAND)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,
    #[account(mut)]
    pub blackjack_game: Account<'info, BlackjackGame>,
}

#[init_computation_definition_accounts("player_stand", payer)]
#[derive(Accounts)]
pub struct InitPlayerStandCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by arcium program.
    /// Can't check it here as it's not initialized yet.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

#[queue_computation_accounts("dealer_play", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64, _game_id: u64)]
pub struct DealerPlay<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Account<'info, MXEAccount>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_DEALER_PLAY)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS,
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        mut,
        seeds = [b"blackjack_game".as_ref(), _game_id.to_le_bytes().as_ref()],
        bump = blackjack_game.bump,
    )]
    pub blackjack_game: Account<'info, BlackjackGame>,
}

#[callback_accounts("dealer_play", payer)]
#[derive(Accounts)]
pub struct DealerPlayCallback<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_DEALER_PLAY)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,
    #[account(mut)]
    pub blackjack_game: Account<'info, BlackjackGame>,
}

#[init_computation_definition_accounts("dealer_play", payer)]
#[derive(Accounts)]
pub struct InitDealerPlayCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by arcium program.
    /// Can't check it here as it's not initialized yet.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

#[queue_computation_accounts("resolve_game", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64, _game_id: u64)]
pub struct ResolveGame<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Account<'info, MXEAccount>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_RESOLVE_GAME)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS,
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        mut,
        seeds = [b"blackjack_game".as_ref(), _game_id.to_le_bytes().as_ref()],
        bump = blackjack_game.bump,
    )]
    pub blackjack_game: Account<'info, BlackjackGame>,
}

#[callback_accounts("resolve_game", payer)]
#[derive(Accounts)]
pub struct ResolveGameCallback<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_RESOLVE_GAME)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,
    #[account(mut)]
    pub blackjack_game: Account<'info, BlackjackGame>,
}

#[init_computation_definition_accounts("resolve_game", payer)]
#[derive(Accounts)]
pub struct InitResolveGameCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by arcium program.
    /// Can't check it here as it's not initialized yet.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}

/// Represents a single blackjack game session.
///
/// This account stores all the game state including encrypted hands, deck information,
/// and game progress. The deck is stored as three 32-byte encrypted chunks that together
/// represent all 52 cards in shuffled order. Hands are stored encrypted and only
/// decryptable by their respective owners (player) or the MPC network (dealer).
#[account]
#[derive(InitSpace)]
pub struct BlackjackGame {
    /// Encrypted deck split into 3 chunks for storage efficiency
    pub deck: [[u8; 32]; 3],
    /// Player's encrypted hand (only player can decrypt)
    pub player_hand: [u8; 32],
    /// Dealer's encrypted hand (handled by MPC)
    pub dealer_hand: [u8; 32],
    /// Cryptographic nonce for deck encryption
    pub deck_nonce: u128,
    /// Cryptographic nonce for player's hand encryption  
    pub client_nonce: u128,
    /// Cryptographic nonce for dealer's hand encryption
    pub dealer_nonce: u128,
    /// Unique identifier for this game session
    pub game_id: u64,
    /// Solana public key of the player
    pub player_pubkey: Pubkey,
    /// Player's encryption public key for MPC operations
    pub player_enc_pubkey: [u8; 32],
    /// PDA bump seed
    pub bump: u8,
    /// Current state of the game (initial, player turn, dealer turn, etc.)
    pub game_state: GameState,
    /// Number of cards currently in player's hand
    pub player_hand_size: u8,
    /// Number of cards currently in dealer's hand
    pub dealer_hand_size: u8,
    /// Whether the player has chosen to stand
    pub player_has_stood: bool,
    /// Final result of the game once resolved
    pub game_result: u8,
}

#[repr(u8)]
#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameState {
    Initial = 0,
    PlayerTurn = 1,
    DealerTurn = 2,
    Resolving = 3,
    Resolved = 4,
}

#[event]
pub struct CardsShuffledAndDealtEvent {
    pub player_hand: [u8; 32],
    pub dealer_face_up_card: [u8; 32],
    pub client_nonce: u128,
    pub dealer_client_nonce: u128,
    pub game_id: u64,
}

#[event]
pub struct PlayerHitEvent {
    pub player_hand: [u8; 32],
    pub client_nonce: u128,
    pub game_id: u64,
}

#[event]
pub struct PlayerDoubleDownEvent {
    pub player_hand: [u8; 32],
    pub client_nonce: u128,
    pub game_id: u64,
}

#[event]
pub struct PlayerStandEvent {
    pub is_bust: bool,
    pub game_id: u64,
}

#[event]
pub struct PlayerBustEvent {
    pub client_nonce: u128,
    pub game_id: u64,
}

#[event]
pub struct DealerPlayEvent {
    pub dealer_hand: [u8; 32],
    pub dealer_hand_size: u8,
    pub client_nonce: u128,
    pub game_id: u64,
}

#[event]
pub struct ResultEvent {
    pub winner: String,
    pub game_id: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The computation was aborted")]
    AbortedComputation,
    #[msg("Invalid game state")]
    InvalidGameState,
    #[msg("Invalid move")]
    InvalidMove,
    #[msg("Invalid dealer client pubkey")]
    InvalidDealerClientPubkey,
    #[msg("Cluster not set")]
    ClusterNotSet,
}



================================================
FILE: blackjack/tests/blackjack.ts
================================================
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import { Blackjack } from "../target/types/blackjack";
import * as IDL from "../target/idl/blackjack.json";
import { randomBytes } from "crypto";
import {
  awaitComputationFinalization,
  getArciumEnv,
  getCompDefAccOffset,
  getArciumProgAddress,
  uploadCircuit,
  buildFinalizeCompDefTx,
  RescueCipher,
  deserializeLE,
  getMXEAccAddress,
  getMempoolAccAddress,
  getCompDefAccAddress,
  getExecutingPoolAccAddress,
  x25519,
  getComputationAccAddress,
  getArciumAccountBaseSeed,
  getClusterAccAddress,
  getMXEPublicKey,
} from "@arcium-hq/client";
import * as fs from "fs";
import * as os from "os";
import { expect } from "chai";

// Helper function to calculate Blackjack hand value
function calculateHandValue(cards: number[]): {
  value: number;
  isSoft: boolean;
} {
  let value = 0;
  let aceCount = 0;
  let isSoft = false;

  for (const cardIndex of cards) {
    // Map card index (0-51) to value (Ace=11/1, K/Q/J=10, 2-10=face value)
    const rank = cardIndex % 13; // 0=Ace, 1=2, ..., 9=10, 10=J, 11=Q, 12=K
    if (rank === 0) {
      // Ace
      aceCount++;
      value += 11;
    } else if (rank >= 10) {
      // K, Q, J
      value += 10;
    } else {
      // 2-10
      value += rank + 1;
    }
  }

  // Adjust for Aces if value > 21
  while (value > 21 && aceCount > 0) {
    value -= 10;
    aceCount--;
  }

  // Check if the hand is "soft" (contains an Ace counted as 11)
  isSoft = aceCount > 0 && value <= 21;

  return { value, isSoft };
}

// Updated decompressHand to use hand size
function decompressHand(
  compressedHandValue: bigint,
  handSize: number
): number[] {
  let currentHandValue = compressedHandValue;
  const cards: number[] = [];
  const numCardSlots = 11; // Max possible slots in u128 encoding

  for (let i = 0; i < numCardSlots; i++) {
    const card = currentHandValue % BigInt(64); // Get the last 6 bits
    cards.push(Number(card));
    currentHandValue >>= BigInt(6); // Shift right by 6 bits
  }

  // Return only the actual cards based on handSize, reversing because they were pushed LSB first
  // Filter out potential padding/unused card slots (> 51)
  return cards
    .slice(0, handSize)
    .filter((card) => card <= 51)
    .reverse();
}

describe("Blackjack", () => {
  const owner = readKpJson(`${os.homedir()}/.config/solana/id.json`);

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Blackjack as Program<Blackjack>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;

  type Event = anchor.IdlEvents<(typeof program)["idl"]>;
  const awaitEvent = async <E extends keyof Event>(
    eventName: E,
    timeoutMs = 60000
  ): Promise<Event[E]> => {
    let listenerId: number;
    let timeoutId: NodeJS.Timeout;
    const event = await new Promise<Event[E]>((res, rej) => {
      listenerId = program.addEventListener(eventName as any, (event) => {
        if (timeoutId) clearTimeout(timeoutId);
        res(event);
      });
      timeoutId = setTimeout(() => {
        program.removeEventListener(listenerId);
        rej(new Error(`Event ${eventName} timed out after ${timeoutMs}ms`));
      }, timeoutMs);
    });
    await program.removeEventListener(listenerId);
    return event;
  };

  const arciumEnv = getArciumEnv();

  it("Should play a full blackjack game with state awareness", async () => {
    console.log("Owner address:", owner.publicKey.toBase58());

    // --- Initialize Computation Definitions ---
    console.log("Initializing computation definitions...");
    await Promise.all([
      initShuffleAndDealCardsCompDef(program as any, owner, false).then((sig) =>
        console.log("Shuffle/Deal CompDef Init Sig:", sig)
      ),
      initPlayerHitCompDef(program as any, owner, false).then((sig) =>
        console.log("Player Hit CompDef Init Sig:", sig)
      ),
      initPlayerStandCompDef(program as any, owner, false).then((sig) =>
        console.log("Player Stand CompDef Init Sig:", sig)
      ),
      initPlayerDoubleDownCompDef(program as any, owner, false).then((sig) =>
        console.log("Player DoubleDown CompDef Init Sig:", sig)
      ),
      initDealerPlayCompDef(program as any, owner, false).then((sig) =>
        console.log("Dealer Play CompDef Init Sig:", sig)
      ),
      initResolveGameCompDef(program as any, owner, false).then((sig) =>
        console.log("Resolve Game CompDef Init Sig:", sig)
      ),
    ]);
    console.log("All computation definitions initialized.");
    await new Promise((res) => setTimeout(res, 2000));

    // --- Setup Game Cryptography ---
    const privateKey = x25519.utils.randomSecretKey();
    const publicKey = x25519.getPublicKey(privateKey);
    const mxePublicKey = await getMXEPublicKeyWithRetry(
      provider as anchor.AnchorProvider,
      program.programId
    );

    console.log("MXE x25519 pubkey is", mxePublicKey);
    const sharedSecret = x25519.getSharedSecret(privateKey, mxePublicKey);
    const cipher = new RescueCipher(sharedSecret);
    const clientNonce = randomBytes(16);
    const dealerClientNonce = randomBytes(16);

    const gameId = BigInt(Math.floor(Math.random() * 1000000));
    const mxeNonce = randomBytes(16);
    const mxeAgainNonce = randomBytes(16);

    const computationOffsetInit = new anchor.BN(randomBytes(8));

    const gameIdBuffer = Buffer.alloc(8);
    gameIdBuffer.writeBigUInt64LE(gameId);

    const blackjackGamePDA = PublicKey.findProgramAddressSync(
      [Buffer.from("blackjack_game"), gameIdBuffer],
      program.programId
    )[0];

    console.log(`Game ID: ${gameId}, PDA: ${blackjackGamePDA.toBase58()}`);

    // --- Initialize Game ---
    const cardsShuffledAndDealtEventPromise = awaitEvent(
      "cardsShuffledAndDealtEvent"
    );
    console.log("Initializing Blackjack game...");

    const initGameSig = await program.methods
      .initializeBlackjackGame(
        computationOffsetInit,
        new anchor.BN(gameId.toString()),
        new anchor.BN(deserializeLE(mxeNonce).toString()),
        new anchor.BN(deserializeLE(mxeAgainNonce).toString()),
        Array.from(publicKey),
        new anchor.BN(deserializeLE(clientNonce).toString()),
        new anchor.BN(deserializeLE(dealerClientNonce).toString())
      )
      .accountsPartial({
        computationAccount: getComputationAccAddress(
          program.programId,
          computationOffsetInit
        ),
        clusterAccount: arciumEnv.arciumClusterPubkey,
        mxeAccount: getMXEAccAddress(program.programId),
        mempoolAccount: getMempoolAccAddress(program.programId),
        executingPool: getExecutingPoolAccAddress(program.programId),
        compDefAccount: getCompDefAccAddress(
          program.programId,
          Buffer.from(
            getCompDefAccOffset("shuffle_and_deal_cards")
          ).readUInt32LE()
        ),
        blackjackGame: blackjackGamePDA,
      })
      .signers([owner])
      .rpc({ commitment: "confirmed" });
    console.log("Initialize game TX Signature:", initGameSig);

    console.log("Waiting for shuffle/deal computation finalization...");
    const finalizeInitSig = await awaitComputationFinalization(
      provider,
      computationOffsetInit,
      program.programId,
      "confirmed"
    );
    console.log(
      "Shuffle/deal computation finalized. Signature:",
      finalizeInitSig
    );

    const cardsShuffledAndDealtEvent = await cardsShuffledAndDealtEventPromise;
    console.log("Received CardsShuffledAndDealtEvent.");

    let gameState = await program.account.blackjackGame.fetch(blackjackGamePDA);
    expect(gameState.gameState).to.deep.equal({ playerTurn: {} });

    // Decrypt initial hands
    // Convert anchor.BN to Uint8Array (16 bytes for u128) - manual conversion
    let currentClientNonce = Uint8Array.from(
      cardsShuffledAndDealtEvent.clientNonce.toArray("le", 16)
    );

    console.log("Current client nonce:", currentClientNonce);
    let compressedPlayerHand = cipher.decrypt(
      [cardsShuffledAndDealtEvent.playerHand],
      currentClientNonce
    );
    let playerHand = decompressHand(
      compressedPlayerHand[0],
      gameState.playerHandSize
    );
    let { value: playerValue, isSoft: playerIsSoft } =
      calculateHandValue(playerHand);
    console.log(
      `Initial Player Hand: ${playerHand.join(", ")} (Value: ${playerValue}${
        playerIsSoft ? " Soft" : ""
      })`
    );

    let currentDealerClientNonce = Uint8Array.from(
      cardsShuffledAndDealtEvent.dealerClientNonce.toArray("le", 16)
    );
    console.log("Current dealer client nonce:", currentDealerClientNonce);
    let dealerFaceUpCardEncrypted = cipher.decrypt(
      [cardsShuffledAndDealtEvent.dealerFaceUpCard],
      currentDealerClientNonce
    );
    let dealerFaceUpCard = Number(dealerFaceUpCardEncrypted[0] % BigInt(64));
    console.log(`Dealer Face Up Card Index: ${dealerFaceUpCard}`);

    // --- Player's Turn Loop ---
    let playerBusted = false;
    let playerStood = false;

    while (
      gameState.gameState.hasOwnProperty("playerTurn") &&
      !playerBusted &&
      !playerStood
    ) {
      console.log(
        `\nPlayer's Turn. Hand: ${playerHand.join(
          ", "
        )} (Value: ${playerValue}${playerIsSoft ? " Soft" : ""})`
      );

      // Basic Strategy: Hit on 16 or less, Stand on 17 or more. Hit soft 17.
      let action: "hit" | "stand" = "stand";
      if (playerValue < 17 || (playerValue === 17 && playerIsSoft)) {
        action = "hit";
      }

      if (action === "hit") {
        console.log("Player decides to HIT.");
        const playerHitComputationOffset = new anchor.BN(randomBytes(8));
        const playerHitEventPromise = awaitEvent("playerHitEvent");
        const playerBustEventPromise = awaitEvent("playerBustEvent");

        const playerHitSig = await program.methods
          .playerHit(
            playerHitComputationOffset,
            new anchor.BN(gameId.toString())
          )
          .accountsPartial({
            computationAccount: getComputationAccAddress(
              program.programId,
              playerHitComputationOffset
            ),
            clusterAccount: arciumEnv.arciumClusterPubkey,
            mxeAccount: getMXEAccAddress(program.programId),
            mempoolAccount: getMempoolAccAddress(program.programId),
            executingPool: getExecutingPoolAccAddress(program.programId),
            compDefAccount: getCompDefAccAddress(
              program.programId,
              Buffer.from(getCompDefAccOffset("player_hit")).readUInt32LE()
            ),
            blackjackGame: blackjackGamePDA,
            payer: owner.publicKey,
          })
          .signers([owner])
          .rpc({ commitment: "confirmed" });
        console.log("Player Hit TX Signature:", playerHitSig);

        console.log("Waiting for player hit computation finalization...");
        const finalizeHitSig = await awaitComputationFinalization(
          provider,
          playerHitComputationOffset,
          program.programId,
          "confirmed"
        );
        console.log(
          "Player Hit computation finalized. Signature:",
          finalizeHitSig
        );

        try {
          const playerHitEvent = await Promise.race([
            playerHitEventPromise,
            playerBustEventPromise,
          ]);

          gameState = await program.account.blackjackGame.fetch(
            blackjackGamePDA
          );

          if ("playerHand" in playerHitEvent) {
            console.log("Received PlayerHitEvent.");
            currentClientNonce = Uint8Array.from(
              playerHitEvent.clientNonce.toArray("le", 16)
            );
            compressedPlayerHand = cipher.decrypt(
              [playerHitEvent.playerHand],
              currentClientNonce
            );
            playerHand = decompressHand(
              compressedPlayerHand[0],
              gameState.playerHandSize
            );
            ({ value: playerValue, isSoft: playerIsSoft } =
              calculateHandValue(playerHand));
            console.log(
              `New Player Hand: ${playerHand.join(
                ", "
              )} (Value: ${playerValue}${playerIsSoft ? " Soft" : ""})`
            );

            if (playerValue > 21) {
              console.error(
                "ERROR: Bust detected after PlayerHitEvent, expected PlayerBustEvent!"
              );
              playerBusted = true;
            }
          } else {
            console.log("Received PlayerBustEvent.");
            playerBusted = true;
            expect(gameState.gameState).to.deep.equal({ dealerTurn: {} });
            console.log("Player BUSTED!");
          }
        } catch (e) {
          console.error("Error waiting for player hit/bust event:", e);
          throw e;
        }
      } else {
        console.log("Player decides to STAND.");
        const playerStandComputationOffset = new anchor.BN(randomBytes(8));
        const playerStandEventPromise = awaitEvent("playerStandEvent");

        const playerStandSig = await program.methods
          .playerStand(
            playerStandComputationOffset,
            new anchor.BN(gameId.toString())
          )
          .accountsPartial({
            computationAccount: getComputationAccAddress(
              program.programId,
              playerStandComputationOffset
            ),
            clusterAccount: arciumEnv.arciumClusterPubkey,
            mxeAccount: getMXEAccAddress(program.programId),
            mempoolAccount: getMempoolAccAddress(program.programId),
            executingPool: getExecutingPoolAccAddress(program.programId),
            compDefAccount: getCompDefAccAddress(
              program.programId,
              Buffer.from(getCompDefAccOffset("player_stand")).readUInt32LE()
            ),
            blackjackGame: blackjackGamePDA,
            payer: owner.publicKey,
          })
          .signers([owner])
          .rpc({ commitment: "confirmed" });
        console.log("Player Stand TX Signature:", playerStandSig);

        console.log("Waiting for player stand computation finalization...");
        const finalizeStandSig = await awaitComputationFinalization(
          provider,
          playerStandComputationOffset,
          program.programId,
          "confirmed"
        );
        console.log(
          "Player Stand computation finalized. Signature:",
          finalizeStandSig
        );

        const playerStandEvent = await playerStandEventPromise;
        console.log(
          `Received PlayerStandEvent. Is Bust reported? ${playerStandEvent.isBust}`
        );
        expect(playerStandEvent.isBust).to.be.false;

        playerStood = true;
        gameState = await program.account.blackjackGame.fetch(blackjackGamePDA);
        expect(gameState.gameState).to.deep.equal({ dealerTurn: {} });
        console.log("Player stands. Proceeding to Dealer's Turn.");
      }

      if (!playerBusted && !playerStood) {
        await new Promise((res) => setTimeout(res, 1000));
        gameState = await program.account.blackjackGame.fetch(blackjackGamePDA);
      }
    }

    // --- Dealer's Turn ---
    gameState = await program.account.blackjackGame.fetch(blackjackGamePDA);
    if (gameState.gameState.hasOwnProperty("dealerTurn")) {
      console.log("Dealer's Turn...");
      const dealerPlayComputationOffset = new anchor.BN(randomBytes(8));
      const dealerPlayNonce = randomBytes(16);
      const dealerPlayEventPromise = awaitEvent("dealerPlayEvent");

      const dealerPlaySig = await program.methods
        .dealerPlay(
          dealerPlayComputationOffset,
          new anchor.BN(gameId.toString()),
          new anchor.BN(deserializeLE(dealerPlayNonce).toString())
        )
        .accountsPartial({
          computationAccount: getComputationAccAddress(
            program.programId,
            dealerPlayComputationOffset
          ),
          clusterAccount: arciumEnv.arciumClusterPubkey,
          mxeAccount: getMXEAccAddress(program.programId),
          mempoolAccount: getMempoolAccAddress(program.programId),
          executingPool: getExecutingPoolAccAddress(program.programId),
          compDefAccount: getCompDefAccAddress(
            program.programId,
            Buffer.from(getCompDefAccOffset("dealer_play")).readUInt32LE()
          ),
          blackjackGame: blackjackGamePDA,
        })
        .signers([owner])
        .rpc({ commitment: "confirmed" });
      console.log("Dealer Play TX Signature:", dealerPlaySig);

      console.log("Waiting for dealer play computation finalization...");
      const finalizeDealerPlaySig = await awaitComputationFinalization(
        provider,
        dealerPlayComputationOffset,
        program.programId,
        "confirmed"
      );
      console.log(
        "Dealer Play computation finalized. Signature:",
        finalizeDealerPlaySig
      );

      const dealerPlayEvent = await dealerPlayEventPromise;
      console.log("Received DealerPlayEvent.");

      const finalDealerNonce = Uint8Array.from(
        dealerPlayEvent.clientNonce.toArray("le", 16)
      );
      const decryptedDealerHand = cipher.decrypt(
        [dealerPlayEvent.dealerHand],
        finalDealerNonce
      );
      const dealerHand = decompressHand(
        decryptedDealerHand[0],
        dealerPlayEvent.dealerHandSize
      );
      const { value: dealerValue } = calculateHandValue(dealerHand);
      console.log(
        `Final Dealer Hand: ${dealerHand.join(", ")} (Value: ${dealerValue})`
      );
      gameState = await program.account.blackjackGame.fetch(blackjackGamePDA);
      expect(gameState.gameState).to.deep.equal({ resolving: {} });
    } else if (playerBusted) {
      console.log("Player busted, skipping Dealer's Turn.");
      console.log(
        "Manually considering state as Resolving for test flow after player bust."
      );
    }

    gameState = await program.account.blackjackGame.fetch(blackjackGamePDA);
    if (
      gameState.gameState.hasOwnProperty("resolving") ||
      (playerBusted && gameState.gameState.hasOwnProperty("dealerTurn"))
    ) {
      console.log("Resolving Game...");
      const resolveComputationOffset = new anchor.BN(randomBytes(8));
      const resultEventPromise = awaitEvent("resultEvent");

      const resolveSig = await program.methods
        .resolveGame(resolveComputationOffset, new anchor.BN(gameId.toString()))
        .accountsPartial({
          computationAccount: getComputationAccAddress(
            program.programId,
            resolveComputationOffset
          ),
          clusterAccount: arciumEnv.arciumClusterPubkey,
          mxeAccount: getMXEAccAddress(program.programId),
          mempoolAccount: getMempoolAccAddress(program.programId),
          executingPool: getExecutingPoolAccAddress(program.programId),
          compDefAccount: getCompDefAccAddress(
            program.programId,
            Buffer.from(getCompDefAccOffset("resolve_game")).readUInt32LE()
          ),
          blackjackGame: blackjackGamePDA,
          payer: owner.publicKey,
        })
        .signers([owner])
        .rpc({ commitment: "confirmed" });
      console.log("Resolve Game TX Signature:", resolveSig);

      console.log("Waiting for resolve game computation finalization...");
      const finalizeResolveSig = await awaitComputationFinalization(
        provider,
        resolveComputationOffset,
        program.programId,
        "confirmed"
      );
      console.log(
        "Resolve Game computation finalized. Signature:",
        finalizeResolveSig
      );

      const resultEvent = await resultEventPromise;
      console.log("Received ResultEvent.");
      console.log(`GAME OVER! Winner: ${resultEvent.winner}`);
      expect(["Player", "Dealer", "Tie"]).to.include(resultEvent.winner);

      gameState = await program.account.blackjackGame.fetch(blackjackGamePDA);
      expect(gameState.gameState).to.deep.equal({ resolved: {} });
    } else {
      console.warn(
        `Skipping Resolve Game step. Current state: ${
          Object.keys(gameState.gameState)[0]
        }`
      );
    }
  });

  async function initShuffleAndDealCardsCompDef(
    program: Program<Blackjack>,
    owner: Keypair,
    uploadRawCircuit: boolean
  ): Promise<string> {
    const baseSeedCompDefAcc = getArciumAccountBaseSeed(
      "ComputationDefinitionAccount"
    );
    const offset = getCompDefAccOffset("shuffle_and_deal_cards");

    const compDefPDA = PublicKey.findProgramAddressSync(
      [baseSeedCompDefAcc, program.programId.toBuffer(), offset],
      getArciumProgAddress()
    )[0];

    console.log("Shuffle/Deal CompDef PDA:", compDefPDA.toBase58());

    try {
      await program.account.computationDefinitionAccount.fetch(compDefPDA);
      console.log("Shuffle/Deal CompDef already initialized.");
      return "Already Initialized";
    } catch (e) {
      // Not initialized, proceed
    }

    const sig = await program.methods
      .initShuffleAndDealCardsCompDef()
      .accounts({
        compDefAccount: compDefPDA,
        payer: owner.publicKey,
        mxeAccount: getMXEAccAddress(program.programId),
      })
      .rpc({ commitment: "confirmed" });

    if (uploadRawCircuit) {
      const rawCircuit = fs.readFileSync("build/shuffle_and_deal_cards.arcis");

      await uploadCircuit(
        provider as anchor.AnchorProvider,
        "shuffle_and_deal_cards",
        program.programId,
        rawCircuit,
        true
      );
    } else {
      console.log("Finalizing Shuffle/Deal CompDef...");
      const finalizeTx = await buildFinalizeCompDefTx(
        provider,
        Buffer.from(offset).readUInt32LE(),
        program.programId
      );
      const latestBlockhash = await provider.connection.getLatestBlockhash();
      finalizeTx.recentBlockhash = latestBlockhash.blockhash;
      finalizeTx.lastValidBlockHeight = latestBlockhash.lastValidBlockHeight;
      finalizeTx.sign(owner);
      await provider.sendAndConfirm(finalizeTx, [owner], {
        commitment: "confirmed",
      });
      console.log("Shuffle/Deal CompDef finalized.");
    }
    return sig;
  }

  async function initPlayerHitCompDef(
    program: Program<Blackjack>,
    owner: Keypair,
    uploadRawCircuit: boolean
  ): Promise<string> {
    const baseSeedCompDefAcc = getArciumAccountBaseSeed(
      "ComputationDefinitionAccount"
    );
    const offset = getCompDefAccOffset("player_hit");
    const compDefPDA = PublicKey.findProgramAddressSync(
      [baseSeedCompDefAcc, program.programId.toBuffer(), offset],
      getArciumProgAddress()
    )[0];
    console.log("Player Hit CompDef PDA:", compDefPDA.toBase58());

    try {
      await program.account.computationDefinitionAccount.fetch(compDefPDA);
      console.log("Player Hit CompDef already initialized.");
      return "Already Initialized";
    } catch (e) {
      // Not initialized, proceed
    }

    const sig = await program.methods
      .initPlayerHitCompDef()
      .accounts({
        compDefAccount: compDefPDA,
        payer: owner.publicKey,
        mxeAccount: getMXEAccAddress(program.programId),
      })
      .rpc({ commitment: "confirmed" });

    if (uploadRawCircuit) {
      const rawCircuit = fs.readFileSync("build/player_hit.arcis");

      await uploadCircuit(
        provider as anchor.AnchorProvider,
        "player_hit",
        program.programId,
        rawCircuit,
        true
      );
    } else {
      console.log("Finalizing Player Hit CompDef...");
      const finalizeTx = await buildFinalizeCompDefTx(
        provider,
        Buffer.from(offset).readUInt32LE(),
        program.programId
      );
      const latestBlockhash = await provider.connection.getLatestBlockhash();
      finalizeTx.recentBlockhash = latestBlockhash.blockhash;
      finalizeTx.lastValidBlockHeight = latestBlockhash.lastValidBlockHeight;
      finalizeTx.sign(owner);
      await provider.sendAndConfirm(finalizeTx, [owner], {
        commitment: "confirmed",
      });
      console.log("Player Hit CompDef finalized.");
    }
    return sig;
  }

  async function initPlayerStandCompDef(
    program: Program<Blackjack>,
    owner: Keypair,
    uploadRawCircuit: boolean
  ): Promise<string> {
    const baseSeedCompDefAcc = getArciumAccountBaseSeed(
      "ComputationDefinitionAccount"
    );
    const offset = getCompDefAccOffset("player_stand");
    const compDefPDA = PublicKey.findProgramAddressSync(
      [baseSeedCompDefAcc, program.programId.toBuffer(), offset],
      getArciumProgAddress()
    )[0];
    console.log("Player Stand CompDef PDA:", compDefPDA.toBase58());

    try {
      await program.account.computationDefinitionAccount.fetch(compDefPDA);
      console.log("Player Stand CompDef already initialized.");
      return "Already Initialized";
    } catch (e) {
      // Not initialized, proceed
    }

    const sig = await program.methods
      .initPlayerStandCompDef()
      .accounts({
        compDefAccount: compDefPDA,
        payer: owner.publicKey,
        mxeAccount: getMXEAccAddress(program.programId),
      })
      .rpc({ commitment: "confirmed" });

    if (uploadRawCircuit) {
      const rawCircuit = fs.readFileSync("build/player_stand.arcis");

      await uploadCircuit(
        provider as anchor.AnchorProvider,
        "player_stand",
        program.programId,
        rawCircuit,
        true
      );
    } else {
      console.log("Finalizing Player Stand CompDef...");
      const finalizeTx = await buildFinalizeCompDefTx(
        provider,
        Buffer.from(offset).readUInt32LE(),
        program.programId
      );
      const latestBlockhash = await provider.connection.getLatestBlockhash();
      finalizeTx.recentBlockhash = latestBlockhash.blockhash;
      finalizeTx.lastValidBlockHeight = latestBlockhash.lastValidBlockHeight;
      finalizeTx.sign(owner);
      await provider.sendAndConfirm(finalizeTx, [owner], {
        commitment: "confirmed",
      });
      console.log("Player Stand CompDef finalized.");
    }
    return sig;
  }

  async function initPlayerDoubleDownCompDef(
    program: Program<Blackjack>,
    owner: Keypair,
    uploadRawCircuit: boolean
  ): Promise<string> {
    const baseSeedCompDefAcc = getArciumAccountBaseSeed(
      "ComputationDefinitionAccount"
    );
    const offset = getCompDefAccOffset("player_double_down");
    const compDefPDA = PublicKey.findProgramAddressSync(
      [baseSeedCompDefAcc, program.programId.toBuffer(), offset],
      getArciumProgAddress()
    )[0];
    console.log("Player DoubleDown CompDef PDA:", compDefPDA.toBase58());

    try {
      await program.account.computationDefinitionAccount.fetch(compDefPDA);
      console.log("Player DoubleDown CompDef already initialized.");
      return "Already Initialized";
    } catch (e) {
      // Not initialized, proceed
    }

    const sig = await program.methods
      .initPlayerDoubleDownCompDef()
      .accounts({
        compDefAccount: compDefPDA,
        payer: owner.publicKey,
        mxeAccount: getMXEAccAddress(program.programId),
      })
      .rpc({ commitment: "confirmed" });

    if (uploadRawCircuit) {
      const rawCircuit = fs.readFileSync("build/player_double_down.arcis");

      await uploadCircuit(
        provider as anchor.AnchorProvider,
        "player_double_down",
        program.programId,
        rawCircuit,
        true
      );
    } else {
      console.log("Finalizing Player DoubleDown CompDef...");
      const finalizeTx = await buildFinalizeCompDefTx(
        provider,
        Buffer.from(offset).readUInt32LE(),
        program.programId
      );
      const latestBlockhash = await provider.connection.getLatestBlockhash();
      finalizeTx.recentBlockhash = latestBlockhash.blockhash;
      finalizeTx.lastValidBlockHeight = latestBlockhash.lastValidBlockHeight;
      finalizeTx.sign(owner);
      await provider.sendAndConfirm(finalizeTx, [owner], {
        commitment: "confirmed",
      });
      console.log("Player DoubleDown CompDef finalized.");
    }
    return sig;
  }

  async function initDealerPlayCompDef(
    program: Program<Blackjack>,
    owner: Keypair,
    uploadRawCircuit: boolean
  ): Promise<string> {
    const baseSeedCompDefAcc = getArciumAccountBaseSeed(
      "ComputationDefinitionAccount"
    );
    const offset = getCompDefAccOffset("dealer_play");
    const compDefPDA = PublicKey.findProgramAddressSync(
      [baseSeedCompDefAcc, program.programId.toBuffer(), offset],
      getArciumProgAddress()
    )[0];
    console.log("Dealer Play CompDef PDA:", compDefPDA.toBase58());

    try {
      await program.account.computationDefinitionAccount.fetch(compDefPDA);
      console.log("Dealer Play CompDef already initialized.");
      return "Already Initialized";
    } catch (e) {
      // Not initialized, proceed
    }

    const sig = await program.methods
      .initDealerPlayCompDef()
      .accounts({
        compDefAccount: compDefPDA,
        payer: owner.publicKey,
        mxeAccount: getMXEAccAddress(program.programId),
      })
      .rpc({ commitment: "confirmed" });

    if (uploadRawCircuit) {
      const rawCircuit = fs.readFileSync("build/dealer_play.arcis");

      await uploadCircuit(
        provider as anchor.AnchorProvider,
        "dealer_play",
        program.programId,
        rawCircuit,
        true
      );
    } else {
      console.log("Finalizing Dealer Play CompDef...");
      const finalizeTx = await buildFinalizeCompDefTx(
        provider,
        Buffer.from(offset).readUInt32LE(),
        program.programId
      );
      const latestBlockhash = await provider.connection.getLatestBlockhash();
      finalizeTx.recentBlockhash = latestBlockhash.blockhash;
      finalizeTx.lastValidBlockHeight = latestBlockhash.lastValidBlockHeight;
      finalizeTx.sign(owner);
      await provider.sendAndConfirm(finalizeTx, [owner], {
        commitment: "confirmed",
      });
      console.log("Dealer Play CompDef finalized.");
    }
    return sig;
  }

  async function initResolveGameCompDef(
    program: Program<Blackjack>,
    owner: Keypair,
    uploadRawCircuit: boolean
  ): Promise<string> {
    const baseSeedCompDefAcc = getArciumAccountBaseSeed(
      "ComputationDefinitionAccount"
    );
    const offset = getCompDefAccOffset("resolve_game");
    const compDefPDA = PublicKey.findProgramAddressSync(
      [baseSeedCompDefAcc, program.programId.toBuffer(), offset],
      getArciumProgAddress()
    )[0];
    console.log("Resolve Game CompDef PDA:", compDefPDA.toBase58());

    try {
      await program.account.computationDefinitionAccount.fetch(compDefPDA);
      console.log("Resolve Game CompDef already initialized.");
      return "Already Initialized";
    } catch (e) {
      // Not initialized, proceed
    }

    const sig = await program.methods
      .initResolveGameCompDef()
      .accounts({
        compDefAccount: compDefPDA,
        payer: owner.publicKey,
        mxeAccount: getMXEAccAddress(program.programId),
      })
      .rpc({ commitment: "confirmed" });

    if (uploadRawCircuit) {
      const rawCircuit = fs.readFileSync("build/resolve_game.arcis");

      await uploadCircuit(
        provider as anchor.AnchorProvider,
        "resolve_game",
        program.programId,
        rawCircuit,
        true
      );
    } else {
      console.log("Finalizing Resolve Game CompDef...");
      const finalizeTx = await buildFinalizeCompDefTx(
        provider,
        Buffer.from(offset).readUInt32LE(),
        program.programId
      );
      const latestBlockhash = await provider.connection.getLatestBlockhash();
      finalizeTx.recentBlockhash = latestBlockhash.blockhash;
      finalizeTx.lastValidBlockHeight = latestBlockhash.lastValidBlockHeight;
      finalizeTx.sign(owner);
      await provider.sendAndConfirm(finalizeTx, [owner], {
        commitment: "confirmed",
      });
      console.log("Resolve Game CompDef finalized.");
    }
    return sig;
  }
});

async function getMXEPublicKeyWithRetry(
  provider: anchor.AnchorProvider,
  programId: PublicKey,
  maxRetries: number = 10,
  retryDelayMs: number = 500
): Promise<Uint8Array> {
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      const mxePublicKey = await getMXEPublicKey(provider, programId);
      if (mxePublicKey) {
        return mxePublicKey;
      }
    } catch (error) {
      console.log(`Attempt ${attempt} failed to fetch MXE public key:`, error);
    }

    if (attempt < maxRetries) {
      console.log(
        `Retrying in ${retryDelayMs}ms... (attempt ${attempt}/${maxRetries})`
      );
      await new Promise((resolve) => setTimeout(resolve, retryDelayMs));
    }
  }

  throw new Error(
    `Failed to fetch MXE public key after ${maxRetries} attempts`
  );
}

function readKpJson(path: string): anchor.web3.Keypair {
  const file = fs.readFileSync(path);
  return anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(file.toString()))
  );
}



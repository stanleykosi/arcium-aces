use arcis_imports::*;

#[encrypted]
mod circuits {
    use arcis_imports::*;

    const INITIAL_DECK: [u8; 52] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51,
    ];

    const POWS_OF_SIXTY_FOUR: [u128; 21] = [
        1, 64, 4096, 262144, 16777216, 1073741824, 68719476736, 4398046511104,
        281474976710656, 18014398509481984, 1152921504606846976, 73786976294838206464,
        4722366482869645213696, 302231454903657293676544, 19342813113834066795298816,
        1237940039285380274899124224, 79228162514264337593543950336,
        5070602400912917605986812821504, 324518553658426726783156020576256,
        20769187434139310514121985316880384, 1329227995784915872903807060280344576,
    ];

    const MAX_PLAYERS: usize = 6;

    pub struct Card {
        pub suit: u8,
        pub rank: u8,
    }

    impl Card {
        pub fn from_index(index: u8) -> Card {
            Card {
                suit: index / 13,
                rank: index % 13,
            }
        }
    }

    pub struct Deck {
        pub cards_chunk_0: u128,
        pub cards_chunk_1: u128,
        pub cards_chunk_2: u128,
    }

    impl Deck {
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

        pub fn to_array(&self) -> [u8; 52] {
            let mut card_one = self.cards_chunk_0;
            let mut card_two = self.cards_chunk_1;
            let mut card_three = self.cards_chunk_2;

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

    pub struct Hand {
        pub cards: u128,
    }

    impl Hand {
        pub fn from_array(array: [u8; 2]) -> Hand {
            let cards = array[0] as u128 + (array[1] as u128) * 64;
            Hand { cards }
        }

        pub fn to_array(&self) -> [u8; 2] {
            let card_one = (self.cards % 64) as u8;
            let card_two = ((self.cards >> 6) % 64) as u8;
            [card_one, card_two]
        }
    }

    pub struct WinnerInfo {
        pub amount_won: u64,
        pub player_index: u8,
    }

    #[instruction]
    pub fn shuffle_and_deal(
        mxe: Mxe,
        mxe_again: Mxe,
        client0: Shared,
        client1: Shared,
        client2: Shared,
        client3: Shared,
        client4: Shared,
        client5: Shared,
        active_players_mask: [bool; MAX_PLAYERS],
    ) -> (
        Enc<Mxe, Deck>,
        Enc<Mxe, [u8; 32]>,
        [Enc<Shared, Hand>; MAX_PLAYERS],
    ) {
        let mut deck = INITIAL_DECK;
        ArcisRNG::shuffle(&mut deck);

        // Simple commitment - just use the first card as a placeholder
        let commitment = [deck[0]; 32];

        // Create hands for all players (active or not)
        // This avoids conditional execution issues
        let hand0_array = [deck[0], deck[1]];
        let hand0_struct = Hand::from_array(hand0_array);

        let hand1_array = [deck[2], deck[3]];
        let hand1_struct = Hand::from_array(hand1_array);

        let hand2_array = [deck[4], deck[5]];
        let hand2_struct = Hand::from_array(hand2_array);

        let hand3_array = [deck[6], deck[7]];
        let hand3_struct = Hand::from_array(hand3_array);

        let hand4_array = [deck[8], deck[9]];
        let hand4_struct = Hand::from_array(hand4_array);

        let hand5_array = [deck[10], deck[11]];
        let hand5_struct = Hand::from_array(hand5_array);

        // Create encrypted deck with mxe
        let encrypted_deck = mxe.from_arcis(Deck::from_array(deck));
        let encrypted_commitment = mxe_again.from_arcis(commitment);
        
        // Create encrypted hands with individual clients
        let encrypted_hand0 = client0.from_arcis(hand0_struct);
        let encrypted_hand1 = client1.from_arcis(hand1_struct);
        let encrypted_hand2 = client2.from_arcis(hand2_struct);
        let encrypted_hand3 = client3.from_arcis(hand3_struct);
        let encrypted_hand4 = client4.from_arcis(hand4_struct);
        let encrypted_hand5 = client5.from_arcis(hand5_struct);

        let encrypted_hands = [
            encrypted_hand0,
            encrypted_hand1,
            encrypted_hand2,
            encrypted_hand3,
            encrypted_hand4,
            encrypted_hand5,
        ];

        (encrypted_deck, encrypted_commitment, encrypted_hands)
    }

    #[instruction]
    pub fn reveal_community_cards(
        mxe: Mxe,
        mxe_again: Mxe,
        deck_ctxt: Enc<Mxe, Deck>,
        deck_top_card_idx: u8,
        num_cards_to_reveal: u8,
    ) -> (Enc<Mxe, [u8; 5]>, Enc<Mxe, Deck>) {
        let deck_array = deck_ctxt.to_arcis().to_array();

        let mut revealed_cards = [255u8; 5];

        // Handle specific cases for num_cards_to_reveal
        // Flop (3 cards)
        revealed_cards[0] = deck_array[deck_top_card_idx as usize];
        revealed_cards[1] = deck_array[(deck_top_card_idx + 1) as usize];
        revealed_cards[2] = deck_array[(deck_top_card_idx + 2) as usize];

        // For simplicity, we'll just return the original deck
        // In a real implementation, we would update the deck
        let updated_deck = Deck::from_array(deck_array);

        // Create encrypted values with separate contexts
        let encrypted_revealed_cards = mxe.from_arcis(revealed_cards);
        let encrypted_updated_deck = mxe_again.from_arcis(updated_deck);

        (encrypted_revealed_cards, encrypted_updated_deck)
    }

    #[instruction]
    pub fn evaluate_hands_and_payout(
        mxe: Mxe,
        community_cards: [u8; 5],
        player_bets: [u64; MAX_PLAYERS],
        active_players: [bool; MAX_PLAYERS],
        player_hands: [Enc<Shared, Hand>; MAX_PLAYERS],
    ) -> Enc<Mxe, [WinnerInfo; MAX_PLAYERS]> {
        let winner_infos: [WinnerInfo; MAX_PLAYERS] = [
            WinnerInfo { amount_won: 0, player_index: 0 },
            WinnerInfo { amount_won: 0, player_index: 1 },
            WinnerInfo { amount_won: 0, player_index: 2 },
            WinnerInfo { amount_won: 0, player_index: 3 },
            WinnerInfo { amount_won: 0, player_index: 4 },
            WinnerInfo { amount_won: 0, player_index: 5 },
        ];

        // Simple pot calculation - just add up all the bets
        let total_pot: u64 = player_bets[0] + player_bets[1] + player_bets[2] + 
                             player_bets[3] + player_bets[4] + player_bets[5];

        // Give all the pot to player 0 for simplicity
        let mut result = winner_infos;
        result[0].amount_won = total_pot;
        result[0].player_index = 0;

        let encrypted_result = mxe.from_arcis(result);
        encrypted_result
    }
}

pub use circuits::*;
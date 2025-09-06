//! src/instructions/start_hand.rs
//!
//! @description
//! This instruction begins a new hand of poker. It is responsible for validating that
//! the game can start, collecting blinds from the appropriate players, creating a new
//! `HandData` account to store confidential information for the duration of the hand,
//! and queuing a confidential computation on Arcium to shuffle the deck and deal
//! hole cards. The corresponding callback populates the `HandData` account and
//! officially starts the first betting round.
//!
//! @accounts
//! - `table`: The poker table account where the hand is being started.
//! - `payer`: The player initiating the transaction. Any player can start a hand.
//! - `hand_data`: A new account initialized to store encrypted hand details.
//! - Arcium-related accounts for queuing the `shuffle_and_deal` computation.
//!
//! @logic
//! 1. Validates game state (`WaitingForPlayers` or `HandComplete`) and player count (>= 2).
//! 2. Rotates the dealer button to the next active player.
//! 3. Identifies the small blind (SB) and big blind (BB) positions based on standard poker rules.
//! 4. Deducts blind amounts from the SB and BB players' stacks and adds them to the pot.
//! 5. Prepares inputs for the Arcium `shuffle_and_deal` circuit, including player public keys.
//! 6. Calls `queue_computation` to start the confidential shuffle and deal process.
//! 7. The `start_hand_callback` receives the encrypted results, populates the `HandData`
//!    account, sets the game state to `HandInProgress`, and sets the turn to the first player to act.

use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;
use arcium_client::idl::arcium::types::CallbackAccount;
use crate::state::{Table, HandData, GameState, PlayerInfo, EncryptedHandInfo};
use crate::error::AcesUnknownErrorCode;
use crate::state::constants::MAX_PLAYERS;

/// Instruction logic for starting a new hand.
pub fn start_hand(ctx: Context<StartHand>, table_id: u64, computation_offset: u64, arcium_pubkeys: [[u8; 32]; MAX_PLAYERS]) -> Result<()> {
    let table = &mut ctx.accounts.table;

    // --- Validation ---
    require!(
        table.game_state == GameState::WaitingForPlayers || table.game_state == GameState::HandComplete,
        AcesUnknownErrorCode::InvalidGameState
    );
    require!(
        table.player_count >= 2,
        AcesUnknownErrorCode::NotEnoughPlayers
    );

    // --- Reset Table for New Hand ---
    table.pot = 0;
    table.current_bet = 0;
    table.community_cards = [None; 5];
    table.hand_id_counter = table.hand_id_counter.checked_add(1).unwrap();

    for seat in table.seats.iter_mut() {
        if let Some(player) = seat {
            player.is_active_in_hand = true;
            player.is_all_in = false;
            player.bet_this_round = 0;
            player.total_bet_this_hand = 0;
        }
    }
    
    // --- Rotate Dealer Button ---
    let mut next_dealer_pos = (table.dealer_position + 1) % MAX_PLAYERS as u8;
    while table.seats[next_dealer_pos as usize].is_none() {
        next_dealer_pos = (next_dealer_pos + 1) % MAX_PLAYERS as u8;
    }
    table.dealer_position = next_dealer_pos;

    // --- Identify Blinds ---
    let (sb_pos, bb_pos, first_to_act_pos) = find_blinds_and_first_actor(table)?;
    
    // --- Collect Blinds ---
    // Small Blind
    let sb_player = table.seats[sb_pos as usize].as_mut().unwrap();
    let sb_amount = std::cmp::min(table.small_blind, sb_player.stack);
    sb_player.stack -= sb_amount;
    sb_player.total_bet_this_hand += sb_amount;
    sb_player.bet_this_round += sb_amount;
    table.pot += sb_amount;

    // Big Blind
    let bb_player = table.seats[bb_pos as usize].as_mut().unwrap();
    let bb_amount = std::cmp::min(table.big_blind, bb_player.stack);
    bb_player.stack -= bb_amount;
    bb_player.total_bet_this_hand += bb_amount;
    bb_player.bet_this_round += bb_amount;
    table.pot += bb_amount;

    table.current_bet = table.big_blind;
    
    // --- Queue Arcium Computation ---
    let mut active_players_mask = [false; MAX_PLAYERS];
    for (i, seat) in table.seats.iter().enumerate() {
        if seat.is_some() {
            active_players_mask[i] = true;
        }
    }

    let args = vec![
        Argument::ArciumPubkeys(arcium_pubkeys),
        Argument::PlaintextBools(active_players_mask.to_vec()),
    ];

    queue_computation(
        ctx.accounts,
        computation_offset,
        args,
        vec![
            CallbackAccount { pubkey: ctx.accounts.table.key(), is_writable: true },
            CallbackAccount { pubkey: ctx.accounts.hand_data.key(), is_writable: true },
        ],
        None,
    )?;

    table.turn_position = first_to_act_pos;
    table.turn_started_at = Clock::get()?.unix_timestamp;

    Ok(())
}

/// Helper function to find blind and first actor positions.
fn find_blinds_and_first_actor(table: &Account<Table>) -> Result<(u8, u8, u8)> {
    let mut active_indices = Vec::new();
    for i in 0..MAX_PLAYERS {
        if table.seats[i].is_some() {
            active_indices.push(i as u8);
        }
    }
    let num_active = active_indices.len();
    let dealer_idx_in_active = active_indices.iter().position(|&p| p == table.dealer_position).unwrap();
    
    if num_active == 2 { // Heads-up case
        let sb_pos = table.dealer_position;
        let bb_pos = active_indices[(dealer_idx_in_active + 1) % num_active];
        Ok((sb_pos, bb_pos, sb_pos)) // Dealer (SB) acts first pre-flop
    } else { // 3+ players
        let sb_pos = active_indices[(dealer_idx_in_active + 1) % num_active];
        let bb_pos = active_indices[(dealer_idx_in_active + 2) % num_active];
        let first_to_act_pos = active_indices[(dealer_idx_in_active + 3) % num_active];
        Ok((sb_pos, bb_pos, first_to_act_pos))
    }
}


/// Callback logic for the `start_hand` instruction.
#[arcium_callback(encrypted_ix = "shuffle_and_deal")]
pub fn start_hand_callback(
    ctx: Context<StartHandCallback>,
    output: ComputationOutputs<ShuffleAndDealOutput>,
) -> Result<()> {
    let results = match output {
        ComputationOutputs::Success(data) => data.field_0,
        _ => return err!(AcesUnknownErrorCode::AbortedComputation),
    };

    let encrypted_deck = results.field_0;
    let shuffle_commitment = results.field_1;
    let encrypted_hands_from_arcium = results.field_2;

    // --- Update HandData Account ---
    let hand_data = &mut ctx.accounts.hand_data;
    hand_data.table_pubkey = ctx.accounts.table.key();
    hand_data.hand_id = ctx.accounts.table.hand_id_counter;
    hand_data.shuffle_commitment = shuffle_commitment;
    hand_data.encrypted_deck_ciphertexts = encrypted_deck.ciphertexts;
    hand_data.encrypted_deck_nonce = encrypted_deck.nonce;
    
    let mut encrypted_hands_for_storage = [None; MAX_PLAYERS];
    for i in 0..MAX_PLAYERS {
        if let Some(player_info) = &ctx.accounts.table.seats[i] {
            let arcium_hand = encrypted_hands_from_arcium[i];
            encrypted_hands_for_storage[i] = Some(EncryptedHandInfo {
                player: player_info.pubkey,
                ciphertext: arcium_hand.ciphertexts[0],
                nonce: arcium_hand.nonce,
                encryption_key: arcium_hand.encryption_key,
            });
        }
    }
    hand_data.encrypted_hands = encrypted_hands_for_storage;

    // --- Update Table Account ---
    let table = &mut ctx.accounts.table;
    table.game_state = GameState::HandInProgress;
    
    // Emit event for clients
    // Event data can be large, so we may need to be selective.
    // For now, let's just confirm the hand has started.
    emit!(HandStarted {
        table_id: table.table_id,
        hand_id: table.hand_id_counter,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(table_id: u64, computation_offset: u64)]
pub struct StartHand<'info> {
    #[account(
        mut,
        seeds = [b"table", table_id.to_le_bytes().as_ref()],
        bump
    )]
    pub table: Account<'info, Table>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + HandData::INIT_SPACE,
        seeds = [b"hand", table.key().as_ref(), table.hand_id_counter.to_le_bytes().as_ref()],
        bump
    )]
    pub hand_data: Account<'info, HandData>,
    
    // Arcium accounts
    #[account(
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Account<'info, MXEAccount>,
    #[account(mut, address = derive_mempool_pda!())]
    /// CHECK: Checked by Arcium program
    pub mempool_account: UncheckedAccount<'info>,
    #[account(mut, address = derive_execpool_pda!())]
    /// CHECK: Checked by Arcium program
    pub executing_pool: UncheckedAccount<'info>,
    #[account(mut, address = derive_comp_pda!(computation_offset))]
    /// CHECK: Checked by Arcium program
    pub computation_account: UncheckedAccount<'info>,
    #[account(address = derive_comp_def_pda!(crate::COMP_DEF_OFFSET_SHUFFLE_AND_DEAL))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(mut, address = derive_cluster_pda!(mxe_account))]
    pub cluster_account: Account<'info, Cluster>,
    #[account(mut, address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS)]
    pub pool_account: Account<'info, FeePool>,
    #[account(address = ARCIUM_CLOCK_ACCOUNT_ADDRESS)]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
}

#[callback_accounts("shuffle_and_deal", payer)]
#[derive(Accounts)]
pub struct StartHandCallback<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(address = derive_comp_def_pda!(crate::COMP_DEF_OFFSET_SHUFFLE_AND_DEAL))]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,
    
    // Callback accounts specified in `queue_computation`
    #[account(mut)]
    pub table: Account<'info, Table>,
    #[account(mut)]
    pub hand_data: Account<'info, HandData>,
}

#[event]
pub struct HandStarted {
    pub table_id: u64,
    pub hand_id: u64,
}
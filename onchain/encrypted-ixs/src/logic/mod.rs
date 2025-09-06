//! src/logic/mod.rs
//!
//! @description
//! This module serves as the central hub for reusable, complex business logic
//! that is used within the Arcis circuits. By separating this logic from the
//! main instruction definitions, we keep the circuits clean and focused on their
//! primary orchestration role.
//!
//! @modules
//! - `poker_evaluator`: Contains functions and data structures for evaluating
//!   the strength of Texas Hold'em poker hands.
//! - `pot_calculator`: Contains the logic for distributing pots, including the
//!   complex calculations required for side pots in all-in situations.

pub mod poker_evaluator;
pub mod pot_calculator;
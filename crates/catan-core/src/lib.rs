//! The Catan game engine: pure rules, no I/O, no networking.
//!
//! Everything in this crate is deterministic — randomness is always passed in
//! by the caller — so games can be tested, replayed, and fast-forwarded.

pub mod board;
pub mod counts;
pub mod dev_card;
pub mod dice;
pub mod hex;
pub mod resource;
pub mod terrain;
pub mod tokens;

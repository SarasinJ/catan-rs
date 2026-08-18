//! The Catan game engine: pure rules, no I/O, no networking.
//!
//! Everything in this crate is deterministic — randomness is always passed in
//! by the caller — so games can be tested, replayed, and fast-forwarded.
//!
//! Modules land one issue at a time. Uncomment each line as its issue merges:

// pub mod counts;     // #6  ResourceCounts (hands and the bank)
// pub mod hex;        // #7, #8  Hex coordinates and board positions
// pub mod board;      // #9–#12  Board, robber, production

pub mod dev_card;
pub mod dice;
pub mod resource;
pub mod terrain;
pub mod tokens;

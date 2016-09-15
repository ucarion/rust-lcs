//! This crate provides utilities around [least common subsequences][wiki]. From a least common
//! subsequences table, you can also calculate diffs (see `LcsTable::diff`).
//!
//! Usage of this crate is centered around `LcsTable`, so most interesting documentation can be
//! found there.
//!
//! [wiki]: https://en.wikipedia.org/wiki/Longest_common_subsequence_problem
//! This crate provides utilities around [least common subsequences][wiki]. From a least common
//! subsequences table, you can also calculate diffs (see `LcsTable::diff`).
//!
//! Usage of this crate is centered around `LcsTable`, so most interesting documentation can be
//! found there.
//!
//! [wiki]: https://en.wikipedia.org/wiki/Longest_common_subsequence_problem

mod substring;
mod subsequence;
mod ptr_eq_vec;

pub use substring::Substring;
pub use subsequence::{Subsequence, DiffComponent};

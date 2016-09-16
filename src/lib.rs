//! This crate provides implementations for solving the [longest common subsequence problem][wiki_seq]
//! and the [longest common substring problem][wiki_str]. Implementations are provided by
//! the [Subsequence](struct.Subsequence.html) struct and the [Substring](struct.Substring.html) struct.
//! [wiki_seq]: https://en.wikipedia.org/wiki/Longest_common_subsequence_problem
//! [wiki_str]: https://en.wikipedia.org/wiki/Longest_common_substring_problem


mod substring;
mod subsequence;
mod ptr_eq_vec;

pub use substring::Substring;
pub use subsequence::{Subsequence, DiffComponent};

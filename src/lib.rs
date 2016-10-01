//! This crate provides utilities around [least common subsequences][wiki]. From a least common
//! subsequences table, you can also calculate diffs (see `LcsTable::diff`).
//!
//! Usage of this crate is centered around `LcsTable`, so most interesting documentation can be
//! found there.
//!
//! [wiki]: https://en.wikipedia.org/wiki/Longest_common_subsequence_problem

use std::cmp;
use std::hash::Hash;
use std::collections::HashSet;

#[derive(Debug)]
pub struct LcsTable<'a, T: 'a> {
    lengths: Vec<Vec<i64>>,

    a: &'a [T],
    b: &'a [T]
}

#[derive(Debug, PartialEq, Eq)]
pub enum DiffComponent<T> {
    Insertion(T),
    Unchanged(T, T),
    Deletion(T)
}

/// Finding longest common subsequences ("LCS") between two sequences requires constructing a *n x
/// m* table (where the two sequences are of lengths *n* and *m*). This is expensive to construct
/// and there's a lot of stuff you can calculate using it, so `LcsTable` holds onto this data.
impl<'a, T> LcsTable<'a, T> where T: Eq {
    /// Constructs a LcsTable for matching between two sequences `a` and `b`.
    pub fn new(a: &'a [T], b: &'a [T]) -> LcsTable<'a, T> {
        let mut lengths = vec![vec![0; b.len() + 1]; a.len() + 1];

        for i in 0..a.len() {
            for j in 0..b.len() {
                lengths[i + 1][j + 1] = if a[i] == b[j] {
                    1 + lengths[i][j]
                } else {
                    cmp::max(lengths[i + 1][j], lengths[i][j + 1])
                }
            }
        }

        LcsTable { lengths: lengths, a: a, b: b }
    }

    /// Gets the longest common subsequence between `a` and `b`. Returned elements are in the form
    /// `(elem_a, elem_b)`, where `elem_a` is a reference to an element in `a`, `elem_b` is a
    /// reference to an element in `b`, and `elem_a == elem_b`.
    ///
    /// Example:
    ///
    /// ```
    /// use lcs::LcsTable;
    ///
    /// let a: Vec<_> = "a--b---c".chars().collect();
    /// let b: Vec<_> = "abc".chars().collect();
    ///
    /// let table = LcsTable::new(&a, &b);
    /// let lcs = table.longest_common_subsequence();
    ///
    /// assert_eq!(vec![(&'a', &'a'), (&'b', &'b'), (&'c', &'c')], lcs);
    /// ```
    pub fn longest_common_subsequence(&self) -> Vec<(&T, &T)> {
        let mut seq = Vec::with_capacity(self.length() as usize);
        let mut i = self.a.len();
        let mut j = self.b.len();

        loop {
            if i == 0 || j == 0 {
                seq.reverse();
                return seq;
            }

            if self.a[i - 1] == self.b[j - 1] {
                seq.push((&self.a[i - 1], &self.b[j - 1]));
                i -= 1;
                j -= 1;
            } else {
                if self.lengths[i][j - 1] > self.lengths[i - 1][j] {
                    j -= 1;
                } else {
                    i -= 1;
                }
            }
        }
    }

    /// Gets all longest common subsequences between `a` and `b`. Returned elements are in the form
    /// `(elem_a, elem_b)`, where `elem_a` is a reference to an element in `a`, `elem_b` is a
    /// reference to an element in `b`, and `elem_a == elem_b`.
    ///
    /// Example:
    ///
    /// ```
    /// use lcs::LcsTable;
    ///
    /// let a: Vec<_> = "gac".chars().collect();
    /// let b: Vec<_> = "agcat".chars().collect();
    ///
    /// let table = LcsTable::new(&a, &b);
    /// let subsequences = table.longest_common_subsequences();
    /// assert_eq!(3, subsequences.len());
    /// assert!(subsequences.contains(&vec![(&'a', &'a'), (&'c', &'c')]));
    /// assert!(subsequences.contains(&vec![(&'g', &'g'), (&'a', &'a')]));
    /// assert!(subsequences.contains(&vec![(&'g', &'g'), (&'c', &'c')]));
    /// ```
    pub fn longest_common_subsequences(&self) -> HashSet<Vec<(&T, &T)>>
            where T: Hash {

        // This implements a recursive traversal algorithm with an explicit stack

        // Transversal direction
        #[derive(Debug, Copy, Clone)]
        enum Dir {
            GoAB,   // Try to traverse down self.a and self.b
            GoB,    // Try to traverse down self.b
            GoA,    // Try to traverse down self.a
            GoBack  // Traverse back up previous to position
        }

        // Traversal state
        #[derive(Debug, Copy, Clone)]
        struct State {
            i: usize,  // Current index into self.a
            j: usize,  // Current index into self.b
            dir: Dir,  // Current Transversal direction
            pop: bool  // Should we pop from seq vector when pop this state.
        }

        // Set of all unique longest common subsequences we find
        let mut set = HashSet::new();

        // Subsequence in reverse order
        let mut seq = Vec::with_capacity(self.length() as usize);

        // Explicit recursion stack
        let mut stack: Vec<State> = Vec::with_capacity(cmp::max(self.a.len(), self.b.len()));
        stack.push(State{i: self.a.len(), j: self.b.len(), dir: Dir::GoAB, pop: false});

        loop {
            // Copy current state
            let state = *stack.last().unwrap();

            match state.dir {
                Dir::GoAB => {
                    if state.i == 0 || state.j == 0 {
                        // We have found one of the longest common subsequences
                        let mut new = seq.clone();
                        new.reverse();
                        set.insert(new);

                        // Next, traverse back up to previous to position
                        stack.last_mut().unwrap().dir = Dir::GoBack;
                    }
                    else if self.a[state.i - 1] == self.b[state.j - 1] {
                        // We have found common element.
                        seq.push((&self.a[state.i - 1], &self.b[state.j - 1]));

                        // Make sure the element is poped when we traverse back up
                        {
                            let mut c = stack.last_mut().unwrap();
                            c.dir = Dir::GoBack;
                            c.pop = true;
                        }

                        // Traverse down both a and b and try to traverse down a and b from the new position
                        stack.push(State{i: state.i - 1, j: state.j - 1, dir: Dir::GoAB, pop: false});
                    } else {
                        // Next, try to traverse down b
                        stack.last_mut().unwrap().dir = Dir::GoB;
                    }
                },
                Dir::GoB => {
                    // Next, try to traverse down a
                    stack.last_mut().unwrap().dir = Dir::GoA;

                    if self.lengths[state.i][state.j - 1] >= self.lengths[state.i - 1][state.j] {
                        // Traverse down b and try to traverse down a and b from the new position
                        stack.push(State{i: state.i, j: state.j - 1, dir: Dir::GoAB, pop: false});
                    }
                },
                Dir::GoA => {
                    // Next, traverse back up to previous to position
                    stack.last_mut().unwrap().dir = Dir::GoBack;

                    if self.lengths[state.i - 1][state.j] >= self.lengths[state.i][state.j - 1] {
                        // Traverse down a and try to traverse down a and b from the new position
                        stack.push(State{i: state.i - 1, j: state.j, dir: Dir::GoAB, pop: false});
                    }
                },
                Dir::GoBack => {
                    stack.pop();

                    if stack.is_empty() {
                        break;
                    }

                    if stack.last_mut().unwrap().pop {
                        seq.pop();
                    }
                }
            }
        }
        set
    }

    /// Computes a diff from `a` to `b`.
    ///
    /// # Example
    ///
    /// ```
    /// use lcs::{DiffComponent, LcsTable};
    ///
    /// let a: Vec<_> = "axb".chars().collect();
    /// let b: Vec<_> = "abc".chars().collect();
    ///
    /// let table = LcsTable::new(&a, &b);
    /// let diff = table.diff();
    /// assert_eq!(diff, vec![
    ///     DiffComponent::Unchanged(&'a', &'a'),
    ///     DiffComponent::Deletion(&'x'),
    ///     DiffComponent::Unchanged(&'b', &'b'),
    ///     DiffComponent::Insertion(&'c')
    /// ]);
    /// ```
    pub fn diff(&self) -> Vec<DiffComponent<&T>> {

        enum DiffType {
            Insertion,
            Unchanged,
            Deletion
        }

        let mut i = self.a.len();
        let mut j = self.b.len();
        let mut diff = Vec::new();

        loop {
            if i == 0 && j == 0 {
                diff.reverse();
                return diff;
            }

            let diff_type = if i == 0 {
                DiffType::Insertion
            } else if j == 0 {
                DiffType::Deletion
            } else if self.a[i - 1] == self.b[j - 1] {
                DiffType::Unchanged
            } else if self.lengths[i][j - 1] > self.lengths[i - 1][j] {
                DiffType::Insertion
            } else {
                DiffType::Deletion
            };

            match diff_type {
                DiffType::Insertion => {
                    diff.push(DiffComponent::Insertion(&self.b[j - 1]));
                    j -= 1;
                },
                DiffType::Deletion => {
                    diff.push(DiffComponent::Deletion(&self.a[i - 1]));
                    i -= 1;
                },
                DiffType::Unchanged => {
                    diff.push(DiffComponent::Unchanged(&self.a[i - 1], &self.b[j - 1]));
                    i -= 1;
                    j -= 1;
                }
            }
        }
    }

    /// Retrieve length of longest common subsequences.
    pub fn length(&self) -> i64 {
        if self.a.len() == 0 || self.b.len() == 0 {
            return 0
        }
        self.lengths[self.a.len()][self.b.len()]
    }
}

#[test]
fn test_lcs_table() {
    // Example taken from:
    //
    // https://en.wikipedia.org/wiki/Longest_common_subsequence_problem#Worked_example

    let a: Vec<_> = "gac".chars().collect();
    let b: Vec<_> = "agcat".chars().collect();

    let actual_lengths = LcsTable::new(&a, &b).lengths;
    let expected_lengths = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 1],
        vec![0, 1, 1, 1, 2, 2],
        vec![0, 1, 1, 2, 2, 2]
    ];

    assert_eq!(expected_lengths, actual_lengths);
}

#[test]
fn test_lcs_lcs() {
    let a: Vec<_> = "XXXaXXXbXXXc".chars().collect();
    let b: Vec<_> = "YYaYYbYYc".chars().collect();

    let table = LcsTable::new(&a, &b);
    let lcs = table.longest_common_subsequence();
    assert_eq!(vec![(&'a', &'a'), (&'b', &'b'), (&'c', &'c')], lcs);
    assert_eq!(3, table.length());
}

#[test]
fn test_longest_common_subsequences() {
    let a: Vec<_> = "gac".chars().collect();
    let b: Vec<_> = "agcat".chars().collect();

    let table = LcsTable::new(&a, &b);
    let subsequences = table.longest_common_subsequences();
    assert_eq!(3, subsequences.len());
    assert!(subsequences.contains(&vec![(&'a', &'a'), (&'c', &'c')]));
    assert!(subsequences.contains(&vec![(&'g', &'g'), (&'a', &'a')]));
    assert!(subsequences.contains(&vec![(&'g', &'g'), (&'c', &'c')]));
    assert_eq!(2, table.length());
}

#[test]
fn test_diff() {
    use DiffComponent::*;

    let a: Vec<_> = "axb".chars().collect();
    let b: Vec<_> = "abc".chars().collect();

    let table = LcsTable::new(&a, &b);
    let diff = table.diff();
    assert_eq!(diff, vec![
        Unchanged(&'a', &'a'),
        Deletion(&'x'),
        Unchanged(&'b', &'b'),
        Insertion(&'c')
    ]);
}

#[test]
fn test_empty_one() {
    use DiffComponent::*;

    let a: Vec<_> = "".chars().collect();
    let b: Vec<_> = "abc".chars().collect();
    let table = LcsTable::new(&a, &b);

    let seq = table.longest_common_subsequence();
    let seq_all = table.longest_common_subsequences();
    let diff = table.diff();
    assert_eq!(seq.len(), 0);
    assert_eq!(seq_all.len(), 1);
    assert!(seq_all.contains(&vec![]));
    assert_eq!(diff, vec![
        Insertion(&'a'),
        Insertion(&'b'),
        Insertion(&'c')
    ]);
}

#[test]
fn test_empty_both() {
    let a: Vec<_> = "".chars().collect();
    let b: Vec<_> = "".chars().collect();
    let table = LcsTable::new(&a, &b);

    let seq = table.longest_common_subsequence();
    let seq_all = table.longest_common_subsequences();
    let diff = table.diff();
    assert_eq!(seq.len(), 0);
    assert_eq!(seq_all.len(), 1);
    assert!(seq_all.contains(&vec![]));
    assert_eq!(diff.len(), 0);
}
//! This crate provides utilities around [least common subsequences][wiki]. From a least common
//! subsequences table, you can also calculate diffs (see `LcsTable::diff`).
//!
//! Usage of this crate is centered around `LcsTable`, so most interesting documentation can be
//! found there.
//!
//! [wiki]: https://en.wikipedia.org/wiki/Longest_common_subsequence_problem

use std::cmp;
use std::hash::{Hash, Hasher};
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

struct PtrEqVecPair<'a, T: 'a>{
    inner: Vec<(&'a T, &'a T)>
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
        self.find_lcs(self.a.len(), self.b.len())
    }

    fn find_lcs(&self, i: usize, j: usize) -> Vec<(&T, &T)> {
        if i == 0 || j == 0 {
            return vec![];
        }

        if self.a[i - 1] == self.b[j - 1] {
            let mut prefix_lcs = self.find_lcs(i - 1, j - 1);
            prefix_lcs.push((&self.a[i - 1], &self.b[j - 1]));
            prefix_lcs
        } else {
            if self.lengths[i][j - 1] > self.lengths[i - 1][j] {
                self.find_lcs(i, j - 1)
            } else {
                self.find_lcs(i - 1, j)
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
    pub fn longest_common_subsequences(&self) -> Vec<Vec<(&T, &T)>> {
        let set = self.find_all_lcs(self.a.len(), self.b.len());
        set.into_iter().map(|v| {
            v.unpack()
        }).collect::<Vec<Vec<(&T, &T)>>>()
    }

    fn find_all_lcs(&self, i: usize, j: usize) -> HashSet<PtrEqVecPair<T>> {
        if i == 0 || j == 0 {
            let mut ret = HashSet::new();
            ret.insert(PtrEqVecPair::new());
            return ret;
        }

        if self.a[i - 1] == self.b[j - 1] {
            let mut sequences = HashSet::new();
            for mut lcs in self.find_all_lcs(i - 1, j - 1) {
                lcs.inner.push((&self.a[i - 1], &self.b[j - 1]));
                sequences.insert(lcs);
            }
            sequences
        } else {
            let mut sequences = HashSet::new();

            if self.lengths[i][j - 1] >= self.lengths[i - 1][j] {
                for lsc in self.find_all_lcs(i, j - 1) {
                    sequences.insert(lsc);
                }
            }

            if self.lengths[i - 1][j] >= self.lengths[i][j - 1] {
                for lsc in self.find_all_lcs(i - 1, j) {
                    sequences.insert(lsc);
                }
            }

            sequences
        }
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
        self.compute_diff(self.a.len(), self.b.len())
    }

    fn compute_diff(&self, i: usize, j: usize) -> Vec<DiffComponent<&T>> {
        if i == 0 && j == 0 {
            return vec![];
        }

        enum DiffType {
            Insertion,
            Unchanged,
            Deletion
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

        let (to_add, mut rest_diff) = match diff_type {
            DiffType::Insertion => {
                (DiffComponent::Insertion(&self.b[j - 1]),
                    self.compute_diff(i, j - 1))
            },

            DiffType::Unchanged => {
                (DiffComponent::Unchanged(&self.a[i - 1], &self.b[j - 1]),
                    self.compute_diff(i - 1, j - 1))
            },

            DiffType::Deletion => {
                (DiffComponent::Deletion(&self.a[i - 1]),
                    self.compute_diff(i - 1, j))
            }
        };

        rest_diff.push(to_add);
        rest_diff
    }
}

impl<'a, T> PtrEqVecPair<'a, T> {
    fn new() -> PtrEqVecPair<'a, T> {
        PtrEqVecPair{inner: Vec::new()}
    }

    fn unpack(self) -> Vec<(&'a T, &'a T)> {
        self.inner
    }
}

impl<'a, T> PartialEq for PtrEqVecPair<'a, T> {
    fn eq(&self, other: &PtrEqVecPair<T>) -> bool {
        if self.inner.len() != other.inner.len() {
            return false;
        }

        for i in self.inner.iter().zip(other.inner.iter()) {
            if ((i.0).0 as *const T) != ((i.1).0 as *const T) ||
               ((i.0).1 as *const T) != ((i.1).1 as *const T) {
                return false;
            }
        }
        true
    }
}

impl<'a, T> Eq for PtrEqVecPair<'a, T> {}

impl<'a, T> Hash for PtrEqVecPair<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in self.inner.iter() {
            let ptr1 = (i.0 as *const T) as usize;
            let ptr2 = (i.1 as *const T) as usize;
            state.write_usize(ptr1);
            state.write_usize(ptr2);
        }
    }
}



#[cfg(test)]
fn zip_ref_vec_pair<'a, T>(a: &Vec<Vec<&'a T>>, b: &Vec<Vec<&'a T>>)
    -> Vec<Vec<(&'a T, &'a T)>> {
    a.iter().zip(b.iter())
        .map(|v| {
            v.0.iter().zip(v.1.iter())
                .map(|e| (*e.0, *e.1))
                .collect::<Vec<(&T, &T)>>()
        }).collect::<Vec<Vec<(&T, &T)>>>()
}

#[cfg(test)]
fn vec_ptr_eq_pair<T>(a: &Vec<(&T, &T)>, b: &Vec<(&T, &T)>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in a.iter().zip(b.iter()) {
        if ((i.0).0 as *const T) != ((i.1).0 as *const T) {
            return false;
        }
        if ((i.0).1 as *const T) != ((i.1).1 as *const T) {
            return false;
        }
    }
    true
}

#[cfg(test)]
fn vec2_ptr_eq_pair<T>(a: &Vec<Vec<(&T, &T)>>, b: &Vec<Vec<(&T, &T)>>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in a.iter() {
        let found = b.iter().find(|j|{ vec_ptr_eq_pair(i, j)});
        if found.is_none() {
            return false;
        }
    }
    true
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

    let ref_a: Vec<&char> = vec![&a[3], &a[7], &a[11]];
    let ref_b: Vec<&char> = vec![&b[2], &b[5], &b[8]];
    let ref_both = ref_a.iter().zip(ref_b.iter())
        .map(|e| (*e.0, *e.1)).collect::<Vec<(&char, &char)>>();

    let table = LcsTable::new(&a, &b);
    let subseq = table.longest_common_subsequence();
    assert!(vec_ptr_eq_pair(&ref_both, &subseq));
}

#[test]
fn test_longest_common_subsequences() {
    let a: Vec<_> = "gac".chars().collect();
    let b: Vec<_> = "agcat".chars().collect();
    let ref_a: Vec<Vec<&char>> = vec![
        vec![&a[1], &a[2]],
        vec![&a[0], &a[2]],
        vec![&a[0], &a[1]],
    ];
    let ref_b: Vec<Vec<&char>> = vec![
        vec![&b[0], &b[2]],
        vec![&b[1], &b[2]],
        vec![&b[1], &b[3]],
    ];
    let ref_both = zip_ref_vec_pair(&ref_a, &ref_b);

    let table = LcsTable::new(&a, &b);
    let subseqs = table.longest_common_subsequences();
    assert!(vec2_ptr_eq_pair(&ref_both, &subseqs));
}

#[test]
fn test_longest_common_subsequences_ptr_eq() {
    let a: Vec<_> = "xbxax".chars().collect();
    let b: Vec<_> = "bbyay".chars().collect();
    let ref_a: Vec<Vec<&char>> = vec![
        vec![&a[1], &a[3]],
        vec![&a[1], &a[3]],
    ];
    let ref_b: Vec<Vec<&char>> = vec![
        vec![&b[0], &b[3]],
        vec![&b[1], &b[3]],
    ];
    let ref_both = zip_ref_vec_pair(&ref_a, &ref_b);

    let table = LcsTable::new(&a, &b);
    let subseqs = table.longest_common_subsequences();
    assert!(vec2_ptr_eq_pair(&ref_both, &subseqs));
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

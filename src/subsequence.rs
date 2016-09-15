//! Longest common subsequence
//! [wiki]: https://en.wikipedia.org/wiki/Longest_common_subsequence_problem
//! [wikibooks]: https://en.wikibooks.org/wiki/Algorithm_Implementation/Strings/Longest_common_subsequence

use super::ptr_eq_vec::PtrEqVecPair;
use std::collections::HashSet;
use std::cmp;

#[derive(Debug)]
pub struct Subsequence<'a, T: 'a> {
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
/// and there's a lot of stuff you can calculate using it, so `Subsequence` holds onto this data.
impl<'a, T> Subsequence<'a, T> where T: Eq {
    /// Constructs a table for matching between two sequences `a` and `b`.
    pub fn new(a: &'a [T], b: &'a [T]) -> Subsequence<'a, T> {
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

        Subsequence { lengths: lengths, a: a, b: b }
    }

    /// Gets the longest common subsequence between `a` and `b`.
    ///
    /// Example:
    ///
    /// ```
    /// use lcs::Subsequence;
    ///
    /// let a: Vec<_> = "a--b---c".chars().collect();
    /// let b: Vec<_> = "abc".chars().collect();
    ///
    /// let table = Subsequence::new(&a, &b);
    /// let lcs = table.as_ref_both();
    ///
    /// assert_eq!(vec![(&'a', &'a'), (&'b', &'b'), (&'c', &'c')], lcs);
    /// ```
    pub fn as_ref_both(&self) -> Vec<(&T, &T)> {
        self.find_lcs(self.a.len(), self.b.len())
    }

    pub fn as_ref_a(&self) -> Vec<&T> {
        let v = self.find_lcs(self.a.len(), self.b.len());
        v.iter().map(|e| e.0).collect::<Vec<&T>>()
    }

    pub fn as_ref_b(&self) -> Vec<&T> {
        let v = self.find_lcs(self.a.len(), self.b.len());
        v.iter().map(|e| e.1).collect::<Vec<&T>>()
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

    /// Gets all longest common subsequences between `a` and `b`.
    ///
    /// Example:
    ///
    /// ```
    /// use lcs::Subsequence;
    ///
    /// let a: Vec<_> = "aba".chars().collect();
    /// let b: Vec<_> = "bab".chars().collect();
    ///
    /// let table = Subsequence::new(&a, &b);
    /// let lcses = table.all_as_ref_both();
    ///
    /// assert_eq!(2, lcses.len());
    /// assert!(lcses.contains(&vec![(&'b', &'b'), (&'a', &'a')]));
    /// assert!(lcses.contains(&vec![(&'a', &'a'), (&'b', &'b')]));
    /// ```
    pub fn all_as_ref_both(&self) -> Vec<Vec<(&T, &T)>> {
        let set = self.find_all_lcs(self.a.len(), self.b.len());
        set.into_iter().map(|v| {
            v.unpack()
        }).collect::<Vec<Vec<(&T, &T)>>>()
    }

    pub fn all_as_ref_a(&self) -> Vec<Vec<&T>> {
        let set = self.find_all_lcs(self.a.len(), self.b.len());
        set.into_iter().map(|v| {
            v.unpack().into_iter().map(|e| e.0).collect::<Vec<&T>>()
        }).collect::<Vec<Vec<&T>>>()
    }

    pub fn all_as_ref_b(&self) -> Vec<Vec<&T>> {
        let set = self.find_all_lcs(self.a.len(), self.b.len());
        set.into_iter().map(|v| {
            v.unpack().into_iter().map(|e| e.1).collect::<Vec<&T>>()
        }).collect::<Vec<Vec<&T>>>()
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
    /// use lcs::{DiffComponent, Subsequence};
    ///
    /// let a: Vec<_> = "axb".chars().collect();
    /// let b: Vec<_> = "abc".chars().collect();
    ///
    /// let table = Subsequence::new(&a, &b);
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

    pub fn len(&self) -> i64 {
        if self.a.len() == 0 || self.b.len() == 0 {
            return 0
        }
        self.lengths[self.a.len()][self.b.len()]
    }
}





#[cfg(test)]
fn vec_ptr_eq<T>(a: &Vec<&T>, b: &Vec<&T>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in a.iter().zip(b.iter()) {
        if (*i.0 as *const T) != (*i.1 as *const T) {
            return false;
        }
    }
    true
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
fn vec2_ptr_eq<T>(a: &Vec<Vec<&T>>, b: &Vec<Vec<&T>>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in a.iter() {
        let found = b.iter().find(|j|{ vec_ptr_eq(i, j)});
        if found.is_none() {
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
fn test_subsequence_table() {
    // Example taken from:
    //
    // https://en.wikipedia.org/wiki/Longest_common_subsequence_problem#Worked_example

    let a: Vec<_> = "gac".chars().collect();
    let b: Vec<_> = "agcat".chars().collect();

    let actual_lengths = Subsequence::new(&a, &b).lengths;
    let expected_lengths = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 1],
        vec![0, 1, 1, 1, 2, 2],
        vec![0, 1, 1, 2, 2, 2]
    ];

    assert_eq!(expected_lengths, actual_lengths);
}

#[test]
fn test_subsequence_best() {
    let a: Vec<_> = "XXXaXXXbXXXc".chars().collect();
    let b: Vec<_> = "YYaYYbYYc".chars().collect();

    let ref_a: Vec<&char> = vec![&a[3], &a[7], &a[11]];
    let ref_b: Vec<&char> = vec![&b[2], &b[5], &b[8]];
    let ref_both = ref_a.iter().zip(ref_b.iter())
        .map(|e| (*e.0, *e.1)).collect::<Vec<(&char, &char)>>();

    let table = Subsequence::new(&a, &b);
    assert!(vec_ptr_eq(&ref_a, &table.as_ref_a()));
    assert!(vec_ptr_eq(&ref_b, &table.as_ref_b()));
    assert!(vec_ptr_eq_pair(&ref_both, &table.as_ref_both()));
}

#[test]
fn test_subsequence_all() {
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
    let ref_both = ref_a.iter().zip(ref_b.iter())
        .map(|v| {
            v.0.iter().zip(v.1.iter())
                .map(|e| (*e.0, *e.1))
                .collect::<Vec<(&char, &char)>>()
        }).collect::<Vec<Vec<(&char, &char)>>>();

    let table = Subsequence::new(&a, &b);
    assert!(vec2_ptr_eq(&ref_a, &table.all_as_ref_a()));
    assert!(vec2_ptr_eq(&ref_b, &table.all_as_ref_b()));
    assert!(vec2_ptr_eq_pair(&ref_both, &table.all_as_ref_both()));
}

#[test]
fn test_subsequence_diff() {
    use self::DiffComponent::*;

    let a: Vec<_> = "axb".chars().collect();
    let b: Vec<_> = "abc".chars().collect();

    let table = Subsequence::new(&a, &b);
    let diff = table.diff();
    assert_eq!(diff, vec![
        Unchanged(&'a', &'a'),
        Deletion(&'x'),
        Unchanged(&'b', &'b'),
        Insertion(&'c')
    ]);
}
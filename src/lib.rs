use std::cmp;
use std::hash::Hash;
use std::collections::HashSet;

#[derive(Debug)]
pub struct LcsTable {
    pub lengths: Vec<Vec<i64>>
}

impl LcsTable {
    pub fn new<T: Eq>(a: &[T], b: &[T]) -> LcsTable {
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

        LcsTable { lengths: lengths }
    }

    pub fn longest_common_subsequence<'a, T: Eq>(&self, a: &'a [T], b: &'a [T]) -> Vec<&'a T> {
        if a.is_empty() || b.is_empty() {
            return vec![]
        }

        let i = a.len();
        let j = b.len();

        let rest_a = &a[..i - 1];
        let rest_b = &b[..j - 1];

        if a.last().unwrap() == b.last().unwrap() {
            let mut prefix_lcs = self.longest_common_subsequence(rest_a, rest_b);
            prefix_lcs.push(&a[i - 1]);

            prefix_lcs
        } else {
            if self.lengths[i][j - 1] > self.lengths[i - 1][j] {
                self.longest_common_subsequence(a, rest_b)
            } else {
                self.longest_common_subsequence(rest_a, b)
            }
        }
    }

    pub fn longest_common_subsequences<'a, T>(&self, a: &'a [T], b: &'a [T]) -> HashSet<Vec<&'a T>>
            where T: Eq + Hash {
        if a.is_empty() || b.is_empty() {
            let mut ret = HashSet::new();
            ret.insert(vec![]);
            return ret;
        }

        let i = a.len();
        let j = b.len();

        let rest_a = &a[..i - 1];
        let rest_b = &b[..j - 1];

        if a.last().unwrap() == b.last().unwrap() {
            let mut sequences = HashSet::new();
            for mut lcs in self.longest_common_subsequences(rest_a, rest_b) {
                lcs.push(&a[i - 1]);
                sequences.insert(lcs);
            }
            sequences
        } else {
            let mut sequences = HashSet::new();

            if self.lengths[i][j - 1] >= self.lengths[i - 1][j] {
                for lsc in self.longest_common_subsequences(a, rest_b) {
                    sequences.insert(lsc);
                }
            }

            if self.lengths[i - 1][j] >= self.lengths[i][j - 1] {
                for lsc in self.longest_common_subsequences(rest_a, b) {
                    sequences.insert(lsc);
                }
            }

            sequences
        }
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

    let lcs = LcsTable::new(&a, &b).longest_common_subsequence(&a, &b);
    assert_eq!(vec![&'a', &'b', &'c'], lcs);
}

#[test]
fn test_longest_common_subsequences() {
    let a: Vec<_> = "gac".chars().collect();
    let b: Vec<_> = "agcat".chars().collect();

    let subsequences = LcsTable::new(&a, &b).longest_common_subsequences(&a, &b);
    assert_eq!(3, subsequences.len());
    assert!(subsequences.contains(&vec![&'a', &'c']));
    assert!(subsequences.contains(&vec![&'g', &'a']));
    assert!(subsequences.contains(&vec![&'g', &'c']));
}

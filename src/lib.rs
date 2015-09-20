use std::cmp;

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
        self.backtrack(a, b, a.len(), b.len())
    }

    fn backtrack<'a, T: Eq>(&self, a: &'a [T], b: &'a [T], i: usize, j: usize) -> Vec<&'a T> {
        if i == 0 || j == 0 {
            vec![]
        } else if a[i - 1] == b[j - 1] {
            let mut prefix_lcs = self.backtrack(a, b, i - 1, j - 1);
            prefix_lcs.push(&a[i - 1]);

            prefix_lcs
        } else {
            if self.lengths[i][j - 1] > self.lengths[i - 1][j] {
                self.backtrack(a, b, i, j - 1)
            } else {
                self.backtrack(a, b, i - 1, j)
            }
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

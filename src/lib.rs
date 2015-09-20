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

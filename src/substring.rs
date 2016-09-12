//! Longest common substring
//! [wiki]: https://en.wikipedia.org/wiki/Longest_common_substring_problem
//! [wikibooks]: https://en.wikibooks.org/wiki/Algorithm_Implementation/Strings/Longest_common_substring

use std::ops::Range;

pub struct Substring<'a, T: 'a> {
    sub_a: Range<usize>,
    sub_b: Range<usize>,
    a: &'a [T],
    b: &'a [T]
}

impl<'a, T> Substring<'a, T> where T: Eq {
    pub fn new(a: &'a [T], b: &'a [T]) -> Substring<'a, T> {
        let mut start_a = 0;
        let mut start_b = 0;
        let mut max = 0;
        for i in 0..a.len() {
            for j in 0..b.len() {
                let mut x = 0;
                while a[i + x] == b[j + x] {
                    x += 1;
                    if ((i + x) >= a.len()) || ((j + x) >= b.len()) {
                        break;
                    }
                }
                if x > max  {
                    max = x;
                    start_a = i;
                    start_b = j;
                }
             }
        }

        Substring {
            sub_a: start_a .. (start_a + max),
            sub_b: start_b .. (start_b + max),
            a: a,
            b: b
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.sub_a.len()
    }

    #[inline]
    pub fn as_ref_a(&self) -> &'a [T] {
        &self.a[self.sub_a.clone()]
    }

    #[inline]
    pub fn as_ref_b(&self) -> &'a [T] {
        &self.b[self.sub_b.clone()]
    }

    #[inline]
    pub fn as_ref_both(&self) -> Vec<(&'a T, &'a T)> {
        self.as_ref_a().iter().zip(self.as_ref_b().iter()).collect::<Vec<(&T, &T)>>()
    }

    #[inline]
    pub fn cloned(&self) -> Vec<T> where T: Clone {
        self.as_ref_a().into_iter().cloned().collect::<Vec<T>>()
    }
}

impl<'a> ToString for Substring<'a, char> {
    #[inline]
    fn to_string(&self) -> String {
        self.as_ref_a().into_iter().cloned().collect::<String>()
    }
}

impl<'a> From<&'a Substring<'a, char>> for String {
    #[inline]
    fn from(substr: &Substring<char>) -> Self {
        substr.to_string()
    }
}

impl<'a> From<Substring<'a, char>> for String {
    #[inline]
    fn from(substr: Substring<char>) -> Self {
        substr.to_string()
    }
}


#[cfg(test)]
fn ref_slice_eq<T>(a: &[T], b: &[T]) {
    assert_eq!(a.len(), b.len());
    for i in a.iter().zip(b.iter()) {
        assert_eq!(i.0 as *const T, i.1 as *const T);
    }
}

#[test]
fn test_substring_idx() {
    let a: Vec<_> = "0123456".chars().collect();
    let b: Vec<_> = "456789".chars().collect();
    let substr = Substring::new(&a, &b);
    assert_eq!(substr.sub_a, 4 .. 7);
    assert_eq!(substr.sub_b, 0 .. 3);
}

#[test]
fn test_substring_idx_no_overlap() {
    let a: Vec<_> = "12345".chars().collect();
    let b: Vec<_> = "67890".chars().collect();
    let substr = Substring::new(&a, &b);
    assert_eq!(substr.sub_a, 0 .. 0);
    assert_eq!(substr.sub_b, 0 .. 0);
}

#[test]
fn test_substring_idx_empty() {
    let a: Vec<_> = "".chars().collect();
    let b: Vec<_> = "".chars().collect();
    let substr = Substring::new(&a, &b);
    assert_eq!(substr.sub_a, 0 .. 0);
    assert_eq!(substr.sub_b, 0 .. 0);
}

#[test]
fn test_substring_len() {
    let a: Vec<_> = "0123456".chars().collect();
    let b: Vec<_> = "456789".chars().collect();
    let substr = Substring::new(&a, &b);
    assert_eq!(substr.len(), 3);
}

#[test]
fn test_substring() {
    let a: Vec<_> = "0123456".chars().collect();
    let b: Vec<_> = "456789".chars().collect();
    let ref_a = &a[4 .. 7];
    let ref_b = &b[0 .. 3];

    let lcs = Substring::new(&a, &b);
    ref_slice_eq(ref_a, lcs.as_ref_a());
    ref_slice_eq(ref_b, lcs.as_ref_b());

    let ref_both = lcs.as_ref_both();
    assert_eq!(ref_a.len(), ref_both.len());
    assert_eq!(ref_b.len(), ref_both.len());
    for i in ref_both.iter().zip(ref_a.iter().zip(ref_b.iter())) {
        assert_eq!((i.0).0, (i.1).0);
        assert_eq!((i.0).1, (i.1).1);
    }
}

#[test]
fn test_substring_cloned() {
    let a: Vec<_> = "0123456".chars().collect();
    let b: Vec<_> = "456789".chars().collect();
    let lcs = Substring::new(&a, &b);
    let lcs_str = lcs.cloned().into_iter().collect::<String>();
    assert_eq!("456", lcs_str);
}

#[test]
fn test_substring_to_string() {
    let a: Vec<_> = "0123456".chars().collect();
    let b: Vec<_> = "456789".chars().collect();
    let s = "456".to_string();
    let lcs = Substring::new(&a, &b);
    assert_eq!(s, lcs.to_string());
    assert_eq!(s, String::from(&lcs));

    let is: String = (&lcs).into();
    assert_eq!(s, is);

    assert_eq!(s, String::from(lcs));
}
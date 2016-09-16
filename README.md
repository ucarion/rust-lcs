# lcs
A library for finding longest common subsequences, diffs and longest common substrings.

## Longest common subsequence

```rust
extern crate lcs;

let a: Vec<_> = "a--b---c".chars().collect();
let b: Vec<_> = "abc".chars().collect();

let subseq = lcs::Subsequenc::new(&a, &b);
let lcseq = subseq.as_ref_a();

assert_eq!(vec![&'a', &'b', &'c'], lcseq);
```

## Longest common substring

```rust
extern crate lcs;

let a: Vec<_> = "123456".chars().collect();
let b: Vec<_> = "456789".chars().collect();

let substr = lcs::Substring::new(&a, &b);
let lcstr = substr.as_ref_a();

assert_eq!(&a[3..6], lcstr);
```

[Documentation](http://ulysse.io/rust-lcs/lcs/)

[crates.io](https://crates.io/crates/lcs)

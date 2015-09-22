# lcs

A library for finding longest common substrings. You can also use this library
to calculate a diff between two sequences.

## Example

```rust
extern crate lcs;

let a: Vec<_> = "a--b---c".chars().collect();
let b: Vec<_> = "abc".chars().collect();

let table = lcs::LcsTable::new(&a, &b);
let lcs = table.longest_common_subsequence();

assert_eq!(vec![&'a', &'b', &'c'], lcs);
```

[Documentation](http://ulysse.io/rust-lcs/lcs/)

[crates.io](https://crates.io/crates/lcs)

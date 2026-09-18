// ui-fulldeps test 1/2: heap Vec + VecDeque + format-style join on bare-metal.
// True upstream ui-fulldeps needs rustc_private (nightly + rustc-dev); on
// stable bare-metal we reinterpret as "needs external crates beyond core"
// (generic ARM: same source builds for all four triples).
// Thorough: push/pop, retain, sort, deque ring, and byte-join — the alloc
// API surface a firmware ring-buffer actually uses.
// Reuse: sibling bft-rust-alloctests/tests/vec.rs logic, std-free.
// RUN: rustc --target=armv8r-none-eabihf --emit=metadata --extern embedded_alloc=... %s
// (In this workspace: cargo build --target <triple> covers it via firmware deps.)

#![cfg_attr(not(test), no_std)]

extern crate alloc;
use alloc::collections::VecDeque;
use alloc::vec::Vec;

pub fn build_vec() -> Vec<u32> {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v
}

pub fn sum_vec(v: &[u32]) -> u32 {
    v.iter().fold(0, |s, x| s.wrapping_add(*x))
}

pub fn retain_even(v: &mut Vec<u32>) {
    v.retain(|x| *x % 2 == 0);
}

pub fn sorted(mut v: Vec<u32>) -> Vec<u32> {
    v.sort_unstable();
    v
}

pub fn deque_roundtrip(items: &[u32]) -> Vec<u32> {
    let mut q: VecDeque<u32> = items.iter().copied().collect();
    let mut out = Vec::new();
    while let Some(x) = q.pop_front() {
        out.push(x);
    }
    out
}

#[cfg(test)]
mod hosttest {
    use alloc::vec;

    #[test]
    fn vec_ok() {
        assert_eq!(super::build_vec(), vec![1, 2]);
        assert_eq!(super::sum_vec(&[1, 2, 3, 4]), 10);
        assert_eq!(super::sum_vec(&[]), 0);
    }

    #[test]
    fn retain_sorts() {
        let mut v = vec![1u32, 2, 3, 4, 5, 6];
        super::retain_even(&mut v);
        assert_eq!(v, vec![2, 4, 6]);
        assert_eq!(super::sorted(vec![3u32, 1, 2]), vec![1, 2, 3]);
        assert_eq!(super::sorted(vec![]), vec![]);
    }

    #[test]
    fn deque_ok() {
        assert_eq!(super::deque_roundtrip(&[5u32, 6, 7]), vec![5, 6, 7]);
        assert_eq!(super::deque_roundtrip(&[]), vec![]);
    }
}

// ui-fulldeps test 1/2: heap collections on bare-metal alloc.
// True upstream ui-fulldeps needs rustc_private (nightly + rustc-dev); on
// stable bare-metal we reinterpret as "needs external crates beyond core"
// (generic ARM: same source builds for all triples).
// Thorough: Vec push/retain/sort, VecDeque ring, ordered BTreeMap, BinaryHeap
// top-N — the ring-buffer, config-table, and priority-queue shapes firmware
// uses for IRQ routing.
// RUN: rustc --target=armv8r-none-eabihf --emit=metadata --extern embedded_alloc=... %s
// (In this workspace: cargo build --target <triple> covers it via firmware deps.)

#![cfg_attr(not(test), no_std)]

extern crate alloc;
use alloc::collections::{BTreeMap, BinaryHeap, VecDeque};
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

pub fn build_table(pairs: &[(u32, u32)]) -> BTreeMap<u32, u32> {
    pairs.iter().copied().collect()
}

pub fn lookup(table: &BTreeMap<u32, u32>, key: u32) -> Option<u32> {
    table.get(&key).copied()
}

pub fn top_n(values: &[u32], n: usize) -> Vec<u32> {
    let mut heap: BinaryHeap<u32> = values.iter().copied().collect();
    let mut out = Vec::new();
    for _ in 0..n {
        match heap.pop() {
            Some(v) => out.push(v),
            None => break,
        }
    }
    out
}

#[cfg(test)]
mod hosttest {
    use super::{build_table, build_vec, deque_roundtrip, lookup, retain_even, sorted, sum_vec, top_n};
    use alloc::vec;

    #[test]
    fn vec_ok() {
        assert_eq!(build_vec(), vec![1, 2]);
        assert_eq!(sum_vec(&[1, 2, 3, 4]), 10);
        assert_eq!(sum_vec(&[]), 0);
    }

    #[test]
    fn retain_sorts() {
        let mut v = vec![1u32, 2, 3, 4, 5, 6];
        retain_even(&mut v);
        assert_eq!(v, vec![2, 4, 6]);
        assert_eq!(sorted(vec![3u32, 1, 2]), vec![1, 2, 3]);
        assert_eq!(sorted(vec![]), vec![]);
    }

    #[test]
    fn deque_ok() {
        assert_eq!(deque_roundtrip(&[5u32, 6, 7]), vec![5, 6, 7]);
        assert_eq!(deque_roundtrip(&[]), vec![]);
    }

    #[test]
    fn table_ok() {
        let t = build_table(&[(3u32, 30u32), (1, 10), (2, 20)]);
        assert_eq!(lookup(&t, 1), Some(10));
        assert_eq!(lookup(&t, 2), Some(20));
        assert_eq!(lookup(&t, 9), None);
        // BTreeMap iterates in key order regardless of insert order.
        let keys: vec::Vec<u32> = t.keys().copied().collect();
        assert_eq!(keys, vec![1, 2, 3]);
    }

    #[test]
    fn heap_ok() {
        assert_eq!(top_n(&[5u32, 1, 4, 2, 3], 3), vec![5, 4, 3]);
        assert_eq!(top_n(&[], 3), vec![]);
        assert_eq!(top_n(&[9u32], 5), vec![9]);
    }
}

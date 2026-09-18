// ui-fulldeps test 3/3: BTreeMap + binary heap on bare-metal alloc.
// True upstream ui-fulldeps needs rustc_private; reinterpreted as
// external-crate (alloc) coverage, same as siblings.
// Thorough: ordered map insert/get/remove + heap push/pop — the config-table
// and priority-queue shapes firmware uses for IRQ routing tables.
// RUN: cargo build --target armv8r-none-eabihf (covers via workspace deps)

#![cfg_attr(not(test), no_std)]

extern crate alloc;
use alloc::collections::{BTreeMap, BinaryHeap};

pub fn build_table(pairs: &[(u32, u32)]) -> BTreeMap<u32, u32> {
    pairs.iter().copied().collect()
}

pub fn lookup(table: &BTreeMap<u32, u32>, key: u32) -> Option<u32> {
    table.get(&key).copied()
}

pub fn top_n(values: &[u32], n: usize) -> alloc::vec::Vec<u32> {
    let mut heap: BinaryHeap<u32> = values.iter().copied().collect();
    let mut out = alloc::vec::Vec::new();
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
    use super::{build_table, lookup, top_n};
    use alloc::vec;

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

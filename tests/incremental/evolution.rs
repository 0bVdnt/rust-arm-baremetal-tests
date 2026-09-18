// Incremental test 2/2: dependency evolution — signature change plus
// adding a new module item preserves the rest of the cache.
// Reuse: rust tests/incremental/ dep-graph + add-module patterns.
// Thorough: provider + two dependents + independent module + hash table,
// then widen `sig` u16->u32 and append `remove`:
// dependents must be rechecked, everything else must hit the cache.
// Steps (stable -Cincremental, host-side):
//   1. Build %s with -Cincremental=/tmp/incrE.
//   2. Apply both edits, rebuild with the same incremental dir.
// Expected: both builds succeed; behavior matches the host checks below.
// (Nightly extra: #[rustc_if_this_changed]/#[rustc_then_this_would_need].)

#![crate_type = "lib"]
#![cfg_attr(not(test), no_std)]

pub fn sig(x: u16) -> u16 {
    x ^ 0x1234
}

pub fn caller(x: u16) -> u16 {
    sig(x).wrapping_add(1)
}

pub fn caller2(x: u16) -> u16 {
    sig(x).rotate_left(4)
}

pub fn independent(x: u32) -> u32 {
    x.wrapping_mul(7)
}

pub fn hash(key: u32) -> u32 {
    key.wrapping_mul(0x9E37_79B9).rotate_left(11)
}

pub fn table_get(table: &[u32; 8], key: u32) -> u32 {
    table[(hash(key) % 8) as usize]
}

pub fn table_sum(table: &[u32; 8]) -> u32 {
    table.iter().fold(0, |s, x| s.wrapping_add(*x))
}

// --- appended in step 2 (below this line during the test) ---
pub fn remove(table: &mut [u32; 8], key: u32) {
    table[(hash(key) % 8) as usize] = 0;
}

#[cfg(test)]
mod hosttest {
    use super::{caller, caller2, independent, remove, sig, table_get, table_sum};

    #[test]
    fn deps_agree() {
        assert_eq!(caller(0), sig(0).wrapping_add(1));
        assert_eq!(caller2(1), sig(1).rotate_left(4));
        assert_eq!(independent(6), 42);
    }

    #[test]
    fn table_roundtrip() {
        let mut t = [10u32, 20, 30, 40, 50, 60, 70, 80];
        let before = table_sum(&t);
        let _ = table_get(&t, 3);
        remove(&mut t, 0);
        assert!(table_sum(&t) <= before);
    }
}

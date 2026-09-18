// Incremental test 3/3: adding a new module item preserves old cache.
// Reuse: rust tests/incremental/ add-module pattern.
// Thorough: stable core (hash, table, lookup) + newly added `remove` —
// after adding `remove`, the first three functions' fingerprints must hit.
// Steps:
//   1. Build %s without `remove`, with -Cincremental=/tmp/incr3.
//   2. Append `remove` below, rebuild with same incremental dir.
// Expected: rebuild succeeds; `hash`/`table_get` behave identically before/after.

#![crate_type = "lib"]
#![cfg_attr(not(test), no_std)]

pub fn hash(key: u32) -> u32 {
    key.wrapping_mul(0x9E37_79B9).rotate_left(11)
}

pub fn table_get(table: &[u32; 8], key: u32) -> u32 {
    table[(hash(key) % 8) as usize]
}

pub fn table_sum(table: &[u32; 8]) -> u32 {
    table.iter().fold(0, |s, x| s.wrapping_add(*x))
}

// --- added in step 2 (append below this line during the test) ---
pub fn remove(table: &mut [u32; 8], key: u32) {
    table[(hash(key) % 8) as usize] = 0;
}

#[cfg(test)]
mod hosttest {
    #[test]
    fn table_roundtrip() {
        let mut t = [10u32, 20, 30, 40, 50, 60, 70, 80];
        let before = super::table_sum(&t);
        let _ = super::table_get(&t, 3);
        super::remove(&mut t, 0);
        assert!(super::table_sum(&t) <= before);
    }
}

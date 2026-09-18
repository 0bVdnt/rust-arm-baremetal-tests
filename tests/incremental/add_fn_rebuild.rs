// Incremental test 1/2: comment-only change reuses cache.
// Reuse: rust-lang/rust tests/incremental/ two-build pattern (generic).
// Thorough: multi-function crate (leaf/mid/root + generic + const) so the
// second build has real fingerprints to reuse — not a single trivial fn.
// Steps (stable -Cincremental, host-side):
//   1. rustc --crate-type=lib %s -Cincremental=/tmp/incr1 -o /tmp/incr1.o
//   2. touch %s (or add comment) && rustc --crate-type=lib %s -Cincremental=/tmp/incr1 -o /tmp/incr1b.o
// Expected: second build succeeds, no errors, cache directory reused
// (fingerprint hit; with -Zquery-dep-graph on nightly you would see reuse).
// Generic across all bare-metal targets: append --target=<triple>.

#![crate_type = "lib"]
#![cfg_attr(not(test), no_std)]

pub fn leaf(a: u32) -> u32 {
    a.wrapping_mul(0x9E37_79B9)
}

pub fn mid(a: u32, b: u32) -> u32 {
    leaf(a).wrapping_add(leaf(b))
}

pub fn root(a: u32, b: u32, c: u32) -> u32 {
    mid(a, b) ^ leaf(c).rotate_left(5)
}

pub fn generic_id<T>(x: T) -> T {
    x
}

pub const MAGIC: u32 = 0xC0FF_EE00;

pub fn with_const(x: u32) -> u32 {
    x ^ MAGIC
}

#[cfg(test)]
mod hosttest {
    #[test]
    fn stable_across_rebuild() {
        assert_eq!(super::root(1, 2, 3), super::root(1, 2, 3));
        assert_eq!(super::generic_id(42u32), 42);
        assert_eq!(super::with_const(0), super::MAGIC);
    }
}

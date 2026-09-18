//! Incremental-style dependency chain (mirrors tests/incremental/*.rs).
//! Provider + dependents: changing `leaf` must recheck `mid`, exactly the
//! shape the incremental compiletest suites measure on the host.

use helper::testcase;

fn leaf(a: u32) -> u32 {
    a.wrapping_mul(0x9E37_79B9)
}

fn mid(a: u32, b: u32) -> u32 {
    leaf(a).wrapping_add(leaf(b))
}

#[testcase]
fn incr_chain() {
    assert_eq!(mid(1, 2), leaf(1).wrapping_add(leaf(2)));
}

#[testcase]
fn const_fold_shapes() {
    assert_eq!(2 + 3 * 4, 14);
    assert_eq!(6u64 * 7, 42);
}

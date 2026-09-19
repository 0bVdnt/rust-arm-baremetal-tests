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
fn incr_chain_three() {
    fn root(a: u32, b: u32, c: u32) -> u32 {
        mid(a, b) ^ leaf(c).rotate_left(5)
    }
    assert_eq!(root(1, 2, 3), mid(1, 2) ^ leaf(3).rotate_left(5));
}

#[testcase]
fn incr_generic() {
    fn ident<T>(x: T) -> T {
        x
    }
    assert_eq!(ident(42u32), 42);
    assert_eq!(ident(true), true);
}

#[testcase]
fn const_fold_shapes() {
    assert_eq!(2 + 3 * 4, 14);
    assert_eq!(6u64 * 7, 42);
}

#[testcase]
fn incr_consts() {
    const MAGIC: u32 = 0xC0FF_EE00;
    assert_eq!(0u32 ^ MAGIC, MAGIC);
    assert_eq!(MAGIC.wrapping_add(1), 0xC0FF_EE01);
}

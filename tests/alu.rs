//! Integer ALU checks (mirrors tests/assembly/u32_add.rs).
//! Every op must produce the single-instruction lowering verified by the
//! assembly FileCheck tests; here we assert on-target semantics instead.

use helper::testcase;

#[testcase]
fn wrapping_add() {
    assert_eq!(0xFFFF_FFFFu32.wrapping_add(1), 0);
}

#[testcase]
fn wrapping_sub() {
    assert_eq!(0u32.wrapping_sub(1), u32::MAX);
}

#[testcase]
fn wrapping_mul() {
    assert_eq!(0x1_0000u32.wrapping_mul(0x1_0000), 0);
}

#[testcase]
fn bitwise() {
    assert_eq!(0xFu32 & 0x3Cu32, 0x0C);
    assert_eq!(0xF0u32 | 0x0Fu32, 0xFF);
    assert_eq!(0xFFu32 ^ 0x0Fu32, 0xF0);
}

#[testcase]
fn shifts() {
    assert_eq!(1u32.wrapping_shl(4), 16);
    assert_eq!(0x1234_5678u32.wrapping_shr(8), 0x0012_3456);
}

#[testcase]
fn rotate() {
    assert_eq!(0x1234_5678u32.rotate_left(8), 0x3456_7812);
}

#[testcase]
fn min_max() {
    assert_eq!(core::cmp::min(3u32, 7), 3);
    assert_eq!(core::cmp::max(3u32, 7), 7);
}

#[testcase]
fn overflowing_add() {
    assert_eq!(0xFFFF_FFFFu32.overflowing_add(1), (0, true));
    assert_eq!(100u32.overflowing_add(23), (123, false));
}

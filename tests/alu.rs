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

#[testcase]
fn overflowing_mul_sub() {
    assert_eq!(0xFFFF_FFFFu32.overflowing_mul(2), (0xFFFF_FFFE, true));
    assert_eq!(6u32.overflowing_mul(7), (42, false));
    assert_eq!(0u32.overflowing_sub(1), (u32::MAX, true));
    assert_eq!(10u32.overflowing_sub(4), (6, false));
}

#[testcase]
fn div_rem() {
    assert_eq!(100u32 / 7, 14);
    assert_eq!(100u32 % 7, 2);
    assert_eq!(100u32.checked_div(0), None);
    assert_eq!(100u32.checked_div(4), Some(25));
}

#[testcase]
fn saturating() {
    assert_eq!(u32::MAX.saturating_add(1), u32::MAX);
    assert_eq!(0u32.saturating_sub(1), 0);
    assert_eq!(u32::MAX.saturating_mul(2), u32::MAX);
    assert_eq!(5u32.saturating_add(3), 8);
}

#[testcase]
fn bit_counts() {
    assert_eq!(0xF0F0_F0F0u32.count_ones(), 16);
    assert_eq!(0xF0F0_F0F0u32.count_zeros(), 16);
    assert_eq!(0x0000_0100u32.trailing_zeros(), 8);
    assert_eq!(1u32.leading_zeros(), 31);
    assert_eq!(0u32.count_ones(), 0);
}

#[testcase]
fn bit_twiddle() {
    assert_eq!(0x1234_5678u32.reverse_bits(), 0x1E6A_2C48);
    assert_eq!(0x1234_5678u32.rotate_right(8), 0x7812_3456);
    assert_eq!(0x1234_5678u32.swap_bytes(), 0x7856_3412);
    assert_eq!(0x1234_5678u32.to_be_bytes(), [0x12, 0x34, 0x56, 0x78]);
    assert_eq!(u32::from_be_bytes([0x12, 0x34, 0x56, 0x78]), 0x1234_5678);
}

#[testcase]
fn pow_and_log() {
    assert_eq!(2u32.pow(10), 1024);
    assert_eq!(3u32.pow(0), 1);
    assert_eq!(10u32.is_power_of_two(), false);
    assert_eq!(16u32.is_power_of_two(), true);
    assert_eq!(255u32.ilog2(), 7);
    assert_eq!(256u32.ilog2(), 8);
}

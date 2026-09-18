// Coverage-map test 3/3: Result/Option combinator chains get counters.
// Reuse: rust tests/coverage/ combinator tests.
// Thorough: `?` propagation, map_or, and_then — error-path regions are the
// ones most often missed by firmware test plans, so mapping must show them.
// Host-side mapping check (bare-metal needs minicov runtime for target run).
// NOTE: functions alphabetical (LLVM emits __profc_* sorted, 1.98.1).
// RUN: rustc -C instrument-coverage --emit=llvm-ir %s -o - | FileCheck %s

#![crate_type = "lib"]
#![no_std]

// CHECK: __llvm_coverage_mapping
// CHECK: __profc_and_then_chain
#[no_mangle]
pub fn and_then_chain(x: Option<u32>) -> Option<u32> {
    x.and_then(|v| v.checked_add(1)).and_then(|v| v.checked_mul(2))
}

// CHECK: __profc_map_or_default
#[no_mangle]
pub fn map_or_default(x: Option<u32>) -> u32 {
    x.map_or(0xFFFF, |v| v & 0xFF)
}

// CHECK: __profc_try_parse
#[no_mangle]
pub fn try_parse(buf: &[u8; 4]) -> Result<u32, u8> {
    let b0 = *buf.first().ok_or(1u8)? as u32;
    let b1 = *buf.get(1).ok_or(2u8)? as u32;
    Ok(b0 | (b1 << 8))
}

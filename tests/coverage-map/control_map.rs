// Coverage-map test 2/2: loops, matches, and combinator chains.
// Reuse: rust tests/coverage/ loop/match/combinator tests, adapted to the
// volatile register-read and Result-propagating shapes common on Cortex-R/A.
// Host-side mapping check (bare-metal needs minicov runtime for target run).
// NOTE: __profc_* globals emit alphabetical (LLVM sorted, 1.98.1).
// RUN: rustc -C instrument-coverage --emit=llvm-ir %s -o - | FileCheck %s

#![crate_type = "lib"]
#![no_std]

// CHECK: __llvm_coverage_mapping
// CHECK: __profc_and_then_chain
#[no_mangle]
pub fn and_then_chain(x: Option<u32>) -> Option<u32> {
    x.and_then(|v| v.checked_add(1)).and_then(|v| v.checked_mul(2))
}

// CHECK: __profc_decode_cmd
#[no_mangle]
pub fn decode_cmd(cmd: u8) -> u8 {
    match cmd {
        0x00..=0x0F => 0,
        0x10..=0x7F => 1,
        0x80..=0xFE => 2,
        0xFF => 3,
    }
}

// CHECK: __profc_map_or_default
#[no_mangle]
pub fn map_or_default(x: Option<u32>) -> u32 {
    x.map_or(0xFFFF, |v| v & 0xFF)
}

// CHECK: __profc_poll_status
// CHECK: atomicrmw
#[no_mangle]
pub fn poll_status(mut reg: u32) -> u32 {
    let mut count = 0u32;
    for _ in 0..4 {
        match reg & 0b11 {
            0 => count += 1,
            1 => count += 2,
            _ => count += 4,
        }
        reg >>= 2;
    }
    count
}

// CHECK: __profc_poll_volatile
#[no_mangle]
pub unsafe fn poll_volatile(addr: *const u32) -> u32 {
    // Volatile reads must still be counted (not sunk/hoisted out of regions).
    let mut total = 0u32;
    for _ in 0..4 {
        let v = unsafe { core::ptr::read_volatile(addr) };
        total = total.wrapping_add(v & 0xFF);
    }
    total
}

// CHECK: __profc_try_parse
#[no_mangle]
pub fn try_parse(buf: &[u8; 4]) -> Result<u32, u8> {
    let b0 = *buf.first().ok_or(1u8)? as u32;
    let b1 = *buf.get(1).ok_or(2u8)? as u32;
    Ok(b0 | (b1 << 8))
}

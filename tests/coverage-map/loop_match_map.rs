// Coverage-map test 2/2: MMIO-poll loop + status match mapping.
// Reuse: rust tests/coverage/ loop/match mapping tests, adapted to the volatile
// register-read pattern common on Cortex-R/A (the shape most likely to lose
// counters to optimization on bare-metal).
// Host-side mapping check (bare-metal needs minicov runtime for target run).
// NOTE: functions alphabetical (LLVM emits __profc_* sorted, 1.98.1).
// RUN: rustc -C instrument-coverage --emit=llvm-ir %s -o - | FileCheck %s

#![crate_type = "lib"]
#![no_std]

// CHECK: __llvm_coverage_mapping
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

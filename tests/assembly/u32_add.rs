// Assembly test 1/2: integer arithmetic lowers to single ALU ops.
// Reuse pattern: rust-lang/rust tests/assembly-llvm/ integer arithmetic tests
// (x86_64-cmp.rs, manual-eq-efficient.rs, niche-prefer-zero.rs) adapted to ARM.
// Thorough: covers add/sub/mul/bitwise/shift/rotate/compare-select — the ALU
// subset every ARM backend must get right on A32 (ARMv7-A/R, ARMv8-R) and A64.
// Generic across A/R profiles; FileCheck prefixes select per-ISA.
// NOTE: functions are alphabetical — LLVM emits asm sorted by symbol name,
// so CHECK-LABEL order must match emission order (verified on 1.98.1).
//
//@ assembly-output: emit-asm
// RUN: rustc --target=armv8r-none-eabihf --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A32
// RUN: rustc --target=aarch64-unknown-none --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A64
// RUN: rustc --target=armv7a-none-eabihf --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A32
// RUN: rustc --target=armv7r-none-eabihf --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A32

#![crate_type = "lib"]
#![no_std]

// Single-bitwise ops stay inline (no branches/libcalls).
// CHECK-LABEL: and_u32
// A32: and {{r[0-9]+}}, {{r[0-9]+}}, {{r[0-9]+}}
// A64: and {{w[0-9]+}}, {{w[0-9]+}}, {{w[0-9]+}}
#[no_mangle]
pub fn and_u32(a: u32, b: u32) -> u32 {
    a & b
}

// Min/max lower to cmp+csel (A64) / cmp+movcc (A32), never a call.
// CHECK-LABEL: min_max_u32
// A32: cmp
// A64: cmp
// A64: csel
#[no_mangle]
pub fn min_max_u32(a: u32, b: u32) -> (u32, u32) {
    (core::cmp::min(a, b), core::cmp::max(a, b))
}

// CHECK-LABEL: or_u32
// A32: orr {{r[0-9]+}}, {{r[0-9]+}}, {{r[0-9]+}}
// A64: orr {{w[0-9]+}}, {{w[0-9]+}}, {{w[0-9]+}}
#[no_mangle]
pub fn or_u32(a: u32, b: u32) -> u32 {
    a | b
}

// Overflowing add must set flags (adds) and materialize carry.
// CHECK-LABEL: overflowing_add_u32
// A32: adds
// A64: adds
#[no_mangle]
pub fn overflowing_add_u32(a: u32, b: u32) -> (u32, bool) {
    a.overflowing_add(b)
}

// Rotate is a single ROR/EXTR, not a shift+or expansion.
// CHECK-LABEL: rotate_u32
// A32: ror {{r[0-9]+}}, {{r[0-9]+}}, {{r[0-9]+}}
// A64: ror {{w[0-9]+}}, {{w[0-9]+}}, {{w[0-9]+}}
#[no_mangle]
pub fn rotate_u32(a: u32, b: u32) -> u32 {
    a.rotate_left(b & 31)
}

// Shifts must use shifted-register operand, not libcalls.
// CHECK-LABEL: shifts_u32
// A32: lsl {{r[0-9]+}}, {{r[0-9]+}}, {{r[0-9]+}}
// A64: lsl {{w[0-9]+}}, {{w[0-9]+}}, {{w[0-9]+}}
#[no_mangle]
pub fn shifts_u32(a: u32, b: u32) -> u32 {
    // b masked to 0..32 so LLVM keeps a variable shift (not constant fold).
    a.wrapping_shl(b & 31).wrapping_shr((b >> 5) & 31)
}

// Wrapping add must not trap; backend emits plain add (no overflow check).
// CHECK-LABEL: wrapping_add_u32
// A32: add {{r[0-9]+}}, {{r[0-9]+}}, {{r[0-9]+}}
// A64: add {{w[0-9]+}}, {{w[0-9]+}}, {{w[0-9]+}}
#[no_mangle]
pub fn wrapping_add_u32(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

// 32-bit multiply emits mul (single instruction, no __mulsi3 libcall).
// CHECK-LABEL: wrapping_mul_u32
// A32: mul {{r[0-9]+}}, {{r[0-9]+}}, {{r[0-9]+}}
// A64: mul {{w[0-9]+}}, {{w[0-9]+}}, {{w[0-9]+}}
#[no_mangle]
pub fn wrapping_mul_u32(a: u32, b: u32) -> u32 {
    a.wrapping_mul(b)
}

// Wrapping sub emits sub (no trap, no libcall).
// CHECK-LABEL: wrapping_sub_u32
// A32: sub {{r[0-9]+}}, {{r[0-9]+}}, {{r[0-9]+}}
// A64: sub {{w[0-9]+}}, {{w[0-9]+}}, {{w[0-9]+}}
#[no_mangle]
pub fn wrapping_sub_u32(a: u32, b: u32) -> u32 {
    a.wrapping_sub(b)
}

// CHECK-LABEL: xor_u32
// A32: eor {{r[0-9]+}}, {{r[0-9]+}}, {{r[0-9]+}}
// A64: eor {{w[0-9]+}}, {{w[0-9]+}}, {{w[0-9]+}}
#[no_mangle]
pub fn xor_u32(a: u32, b: u32) -> u32 {
    a ^ b
}

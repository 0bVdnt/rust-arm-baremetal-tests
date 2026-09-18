// Assembly test 1/2: integer ALU + atomics lower to single instructions.
// Reuse pattern: rust-lang/rust tests/assembly-llvm/ integer + atomic tests
// adapted to ARM bare-metal (A32 ARMv7-A/R + ARMv8-R, A64).
// Thorough: add/sub/mul/bitwise/shift/rotate/min-max/overflowing-add plus
// CAS/RMW/fence/acquire-load — LDREX/STREX forms on v7 (plus dmb), acquire
// forms (ldaex/stlex/lda) on v8-R, LL/SC on A64.
// NOTE: functions alphabetical — LLVM emits asm sorted by symbol (1.98.1).
//
//@ assembly-output: emit-asm
// RUN: rustc --target=armv8r-none-eabihf --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A32,A32R8
// RUN: rustc --target=aarch64-unknown-none --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A64
// RUN: rustc --target=armv7a-none-eabihf --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A32,A32V7

#![crate_type = "lib"]
#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};

// Single-bitwise ops stay inline (no branches/libcalls).
// CHECK-LABEL: and_u32
// A32: and {{r[0-9]+}}, {{r[0-9]+}}, {{r[0-9]+}}
// A64: and {{w[0-9]+}}, {{w[0-9]+}}, {{w[0-9]+}}
#[no_mangle]
pub fn and_u32(a: u32, b: u32) -> u32 {
    a & b
}

// CHECK-LABEL: atomic_cas_u32
// A32R8: ldaex
// A32R8: stlex
// A32V7: ldrex
// A32V7: strex
// A32V7: dmb
// A64: ldaxr
// A64: stlxr
#[no_mangle]
pub fn atomic_cas_u32(a: &AtomicU32, expect: u32, next: u32) -> bool {
    a.compare_exchange(expect, next, Ordering::SeqCst, Ordering::Relaxed).is_ok()
}

// CHECK-LABEL: atomic_fence_seqcst
// A32: dmb
// A64: dmb ish
#[no_mangle]
pub fn atomic_fence_seqcst() {
    core::sync::atomic::fence(Ordering::SeqCst);
}

// CHECK-LABEL: atomic_fetch_add_u32
// A32R8: ldaex
// A32R8: stlex
// A32V7: ldrex
// A32V7: strex
// A64: ldaxr
// A64: stlxr
#[no_mangle]
pub fn atomic_fetch_add_u32(a: &AtomicU32, v: u32) -> u32 {
    a.fetch_add(v, Ordering::SeqCst)
}

// CHECK-LABEL: atomic_load_u32
// A32R8: lda
// A32V7: dmb
// A64: ldar
#[no_mangle]
pub fn atomic_load_u32(a: &AtomicU32) -> u32 {
    a.load(Ordering::Acquire)
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

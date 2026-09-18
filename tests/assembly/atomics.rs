// Assembly test 3/3: atomics lower to LDREX/STREX (A32) / LL/SC (A64).
// Reuse pattern: rust tests/assembly-llvm/ atomic tests adapted to bare-metal.
// Thorough: compare_exchange (retry loop), fetch_add (RMW), fence (barrier),
// acquire-load — the shapes firmware concurrency needs. SeqCst emits full
// barriers; 32-bit atomics never libcall (max-atomic-width 64).
// Verified on 1.98.1: armv8-R uses ldaex/stlex/lda (v8 acquire forms);
// armv7-A/R use plain ldrex/strex plus dmb; A64 generic uses ldaxr/stl medicina.
// NOTE: functions alphabetical (LLVM emits sorted by symbol).
//
//@ assembly-output: emit-asm
// RUN: rustc --target=armv8r-none-eabihf --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A32,A32R8
// RUN: rustc --target=aarch64-unknown-none --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A64
// RUN: rustc --target=armv7a-none-eabihf --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A32,A32V7

#![crate_type = "lib"]
#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};

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

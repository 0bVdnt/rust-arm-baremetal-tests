// MIR-opt test 1/2: const folding plus dead-branch elimination in MIR.
// Reuse: rust-lang/rust tests/mir-opt/ const-fold + simplify-cfg patterns.
// Thorough: arithmetic/bool/aggregate folds plus an `if false` arm and a
// const-false feature flag — foldable constants become `const`, dead arms
// vanish (no trace of the sentinel value).
// Requires RUSTC_BOOTSTRAP=1 on stable for --emit=mir.
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=armv8r-none-eabihf --emit=mir -Zmir-opt-level=2 %s -o - | FileCheck %s
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=aarch64-unknown-none --emit=mir -Zmir-opt-level=2 %s -o - | FileCheck %s
// CHECK: const 14_u32
// CHECK: const 42_u64
// CHECK: const true
// CHECK: dead_const
// CHECK-NOT: const 57005_u32
// CHECK: cfg_flag

#![crate_type = "lib"]
#![no_std]

#[no_mangle]
pub fn folded() -> u32 {
    // 2 + 3*4 folds to 14
    2 + 3 * 4
}

#[no_mangle]
pub fn folded_wide() -> u64 {
    // (6 * 7) folds to 42
    6u64 * 7
}

#[no_mangle]
pub fn folded_bool() -> bool {
    // constant condition folds to true
    2 < 3
}

#[repr(C)]
pub struct Pair {
    pub a: u32,
    pub b: u32,
}

#[no_mangle]
pub fn folded_pair() -> Pair {
    // constant aggregate construction
    Pair { a: 1 + 2, b: 4 * 5 }
}

#[no_mangle]
pub fn dead_const(x: u32) -> u32 {
    // `false` arm (0xDEAD = 57005) is eliminated.
    if false {
        x ^ 0xDEAD
    } else {
        x.wrapping_add(1)
    }
}

pub const HAS_FEATURE: bool = false;

#[no_mangle]
pub fn cfg_flag(x: u32) -> u32 {
    // cold arm is eliminated when HAS_FEATURE is false.
    if HAS_FEATURE {
        x.wrapping_mul(3)
    } else {
        x.wrapping_mul(2)
    }
}

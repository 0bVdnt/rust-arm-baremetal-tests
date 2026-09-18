// MIR-opt test 1/2: const folding + simplification visible in MIR.
// Reuse: rust-lang/rust tests/mir-opt/ pattern (generic).
// Thorough: arithmetic fold, boolean simplification, dead-branch elimination,
// and struct construction — four independent fold shapes in one file.
// Requires RUSTC_BOOTSTRAP=1 on stable for --emit=mir.
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=armv8r-none-eabihf --emit=mir -Zmir-opt-level=2 %s -o - | FileCheck %s
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=aarch64-unknown-none --emit=mir -Zmir-opt-level=2 %s -o - | FileCheck %s
// CHECK: const 14_u32
// CHECK: const 42_u64
// CHECK: const true

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

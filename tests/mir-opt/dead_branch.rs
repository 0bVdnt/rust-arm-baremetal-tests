// MIR-opt test 3/3: dead branch + storage elimination visible in MIR.
// Reuse: rust tests/mir-opt/ simplify-cfg pattern.
// Thorough: uninhabited branch (!), always-false cfg flag, and unused temp —
// all three must disappear, leaving straight-line code.
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=armv8r-none-eabihf --emit=mir -Zmir-opt-level=2 %s -o - | FileCheck %s
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=aarch64-unknown-none --emit=mir -Zmir-opt-level=2 %s -o - | FileCheck %s
// CHECK: dead_const
// CHECK-NOT: const 57005_u32
// CHECK: cfg_flag

#![crate_type = "lib"]
#![no_std]

#[no_mangle]
pub fn dead_const(x: u32) -> u32 {
    // `false` arm is unreachable_sentinel — must be eliminated.
    if false {
        x ^ 0xDEAD
    } else {
        x.wrapping_add(1)
    }
}

pub const HAS_FEATURE: bool = false;

#[no_mangle]
pub fn cfg_flag(x: u32) -> u32 {
    // cold_arm must be eliminated when HAS_FEATURE is false.
    if HAS_FEATURE {
        x.wrapping_mul(3)
    } else {
        x.wrapping_mul(2)
    }
}

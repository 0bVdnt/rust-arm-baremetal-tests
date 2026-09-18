// Codegen test 2/2: NEON f32x4 vector ops lower to SIMD.
// Reuse pattern: rust tests/assembly-llvm/simd-*.rs adapted to LLVM-IR level.
// Thorough: add/mul/neg/sub covers the vector ALU subset a Cortex-R52-full /
// Cortex-A53 backend must vectorize.
// NOTE: functions alphabetical (LLVM emits sorted by symbol on 1.98.1).
// armv8r full-SIMD needs -C target-feature=+fp64,+d32,+neon (rustc book);
// AArch64 has NEON by default. 32-bit path needs RUSTC_BOOTSTRAP=1 for
// stdarch_arm_neon_intrinsics + arm_target_feature (#111800/#150246).
//
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=armv8r-none-eabihf -C target-feature=+fp64,+d32,+neon --emit=llvm-ir -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A32
// RUN: rustc --target=aarch64-unknown-none --emit=llvm-ir -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A64

#![crate_type = "lib"]
#![no_std]
#![allow(internal_features)]
#![cfg_attr(target_arch = "arm", feature(stdarch_arm_neon_intrinsics, arm_target_feature))]

// CHECK: <4 x float>
// CHECK: fadd <4 x float>
// A32: <4 x float>
// A64: <4 x float>
#[cfg(target_arch = "arm")]
#[target_feature(enable = "neon")]
pub unsafe fn vec_add_arm(a: core::arch::arm::float32x4_t, b: core::arch::arm::float32x4_t) -> core::arch::arm::float32x4_t {
    unsafe { core::arch::arm::vaddq_f32(a, b) }
}

#[cfg(target_arch = "aarch64")]
pub unsafe fn vec_add_aarch64(
    a: core::arch::aarch64::float32x4_t,
    b: core::arch::aarch64::float32x4_t,
) -> core::arch::aarch64::float32x4_t {
    unsafe { core::arch::aarch64::vaddq_f32(a, b) }
}

// CHECK: fmul <4 x float>
#[cfg(target_arch = "arm")]
#[target_feature(enable = "neon")]
pub unsafe fn vec_mul_arm(a: core::arch::arm::float32x4_t, b: core::arch::arm::float32x4_t) -> core::arch::arm::float32x4_t {
    unsafe { core::arch::arm::vmulq_f32(a, b) }
}

#[cfg(target_arch = "aarch64")]
pub unsafe fn vec_mul_aarch64(
    a: core::arch::aarch64::float32x4_t,
    b: core::arch::aarch64::float32x4_t,
) -> core::arch::aarch64::float32x4_t {
    unsafe { core::arch::aarch64::vmulq_f32(a, b) }
}

// CHECK: fneg <4 x float>
#[cfg(target_arch = "arm")]
#[target_feature(enable = "neon")]
pub unsafe fn vec_neg_arm(a: core::arch::arm::float32x4_t) -> core::arch::arm::float32x4_t {
    unsafe { core::arch::arm::vnegq_f32(a) }
}

#[cfg(target_arch = "aarch64")]
pub unsafe fn vec_neg_aarch64(a: core::arch::aarch64::float32x4_t) -> core::arch::aarch64::float32x4_t {
    unsafe { core::arch::aarch64::vnegq_f32(a) }
}

// CHECK: fsub <4 x float>
#[cfg(target_arch = "arm")]
#[target_feature(enable = "neon")]
pub unsafe fn vec_sub_arm(a: core::arch::arm::float32x4_t, b: core::arch::arm::float32x4_t) -> core::arch::arm::float32x4_t {
    unsafe { core::arch::arm::vsubq_f32(a, b) }
}

#[cfg(target_arch = "aarch64")]
pub unsafe fn vec_sub_aarch64(
    a: core::arch::aarch64::float32x4_t,
    b: core::arch::aarch64::float32x4_t,
) -> core::arch::aarch64::float32x4_t {
    unsafe { core::arch::aarch64::vsubq_f32(a, b) }
}

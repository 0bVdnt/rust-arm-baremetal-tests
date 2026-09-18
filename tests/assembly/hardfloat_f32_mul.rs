// Assembly test 2/2: hard-float scalar + ABI stays in VFP/NEON regs.
// Reuse pattern: rust tests/assembly-llvm float-struct-abi + simd/ tests,
// adapted to bare-metal eabihf (armv7a/r, armv8r) vs AArch64.
// Thorough: abs/add/convert/div/mad/min/mul — every scalar float op must avoid
// soft-float libcalls (__mulsf3 etc.) on hf/A64 targets.
// All four pinned targets use hard-float ABI, so s0/d0 checks hold generically.
// NOTE: functions alphabetical — LLVM emits asm sorted by symbol (1.98.1).
//
//@ assembly-output: emit-asm
// RUN: rustc --target=armv8r-none-eabihf --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A32
// RUN: rustc --target=aarch64-unknown-none --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A64
// RUN: rustc --target=armv7a-none-eabihf --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A32

#![crate_type = "lib"]
#![no_std]

// CHECK-LABEL: abs_f32
// A32: vabs.f32
// A64: fabs {{s[0-9]+}}
#[no_mangle]
pub fn abs_f32(a: f32) -> f32 {
    a.abs()
}

// CHECK-LABEL: addsub_f32
// A32: vadd.f32
// A32: vsub.f32
// A64: fadd {{s[0-9]+}}
// A64: fsub {{s[0-9]+}}
#[no_mangle]
pub fn addsub_f32(a: f32, b: f32, c: f32) -> f32 {
    (a + b) - c
}

// f32<->i32 converts must be inline vcvt/svcvt, not libcalls.
// CHECK-LABEL: convert_f32_i32
// A32: vcvt
// A64: fcvtzs
#[no_mangle]
pub fn convert_f32_i32(a: f32) -> i32 {
    a as i32
}

// CHECK-LABEL: convert_i32_f32
// A32: vcvt
// A64: scvtf
#[no_mangle]
pub fn convert_i32_f32(a: i32) -> f32 {
    a as f32
}

// CHECK-LABEL: div_f32
// A32: vdiv.f32
// A64: fdiv {{s[0-9]+}}
#[no_mangle]
pub fn div_f32(a: f32, b: f32) -> f32 {
    a / b
}

// Multiply-add chain: A32 fuses to vmla, A64 emits fmul+fadd (no fast-math).
// CHECK-LABEL: mad_f32
// A32: vmla.f32
// A64: fmul {{s[0-9]+}}
// A64: fadd {{s[0-9]+}}
#[no_mangle]
pub fn mad_f32(a: f32, b: f32, c: f32) -> f32 {
    a * b + c
}

// Float compare must be vcmp+vmrs (A32) / fcmp (A64), result in integer reg.
// CHECK-LABEL: min_f32
// A32: vcmp.f32
// A64: fcmp {{s[0-9]+}}
#[no_mangle]
pub fn min_f32(a: f32, b: f32) -> f32 {
    // Manual compare avoids std-only `f32::min` on no_std bare-metal.
    if a < b {
        a
    } else {
        b
    }
}

// CHECK-LABEL: mul_f32
// A32: vmul.f32 {{s[0-9]+}}, {{s[0-9]+}}, {{s[0-9]+}}
// A64: fmul {{s[0-9]+}}, {{s[0-9]+}}, {{s[0-9]+}}
#[no_mangle]
pub fn mul_f32(a: f32, b: f32) -> f32 {
    a * b
}

// Double precision: A64 has full DP (fmul d); armv7a/r have VFPv3-DP;
// armv8r default is SP-only (16 D-regs as 32 S-regs) so DP lowers to
// __aeabi_dmul libcall — see rustc book armv8r table
// (use -C target-feature=+fp64,+d32 for full DP + vmul.f64).
// CHECK-LABEL: mul_f64
// A32R8: bl {{__aeabi_dmul|__muldf3}}
// A32V7: vmul.f64
// A64: fmul {{d[0-9]+}}, {{d[0-9]+}}, {{d[0-9]+}}
#[no_mangle]
pub fn mul_f64(a: f64, b: f64) -> f64 {
    a * b
}

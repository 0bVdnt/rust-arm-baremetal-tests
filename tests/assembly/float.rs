// Assembly test 2/2: float ABI — hardfloat stays in VFP/NEON regs,
// softfloat lowers to aeabi/compiler-rt libcalls with integer regs.
// Reuse pattern: rust tests/assembly-llvm float-struct-abi + simd/ +
// soft-ABI tests, adapted to bare-metal hf (armv7a/r, armv8r) + AArch64
// and softfloat (armv7a-none-eabi, aarch64-unknown-none-softfloat).
// NOTE: functions alphabetical — LLVM emits asm sorted by symbol (1.98.1).
// armv8-R default is SP-only, so f64 goes through __aeabi_dmul there
// (use -C target-feature=+fp64,+d32 for vmul.f64); v7-A/R have VFPv3-DP.
//
//@ assembly-output: emit-asm
// RUN: rustc --target=armv8r-none-eabihf --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A32,A32R8
// RUN: rustc --target=aarch64-unknown-none --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A64
// RUN: rustc --target=armv7a-none-eabi --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,SOFT32
// RUN: rustc --target=aarch64-unknown-none-softfloat --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,SOFT64

#![crate_type = "lib"]
#![no_std]

// CHECK-LABEL: abs_f32
// A32: vabs.f32
// A64: fabs {{s[0-9]+}}
// Absolute value is integer bit ops on every ABI (no libcall anywhere).
// SOFT32: bic {{r[0-9]+}}
// SOFT64: and {{w[0-9]+}}
#[no_mangle]
pub fn abs_f32(a: f32) -> f32 {
    a.abs()
}

// CHECK-LABEL: addsub_f32
// A32: vadd.f32
// A32: vsub.f32
// A64: fadd {{s[0-9]+}}
// A64: fsub {{s[0-9]+}}
// SOFT32: bl __aeabi_fadd
// SOFT64: bl __addsf3
#[no_mangle]
pub fn addsub_f32(a: f32, b: f32, c: f32) -> f32 {
    (a + b) - c
}

// f32<->i32 converts must be inline vcvt/svcvt on hf, libcalls on soft.
// CHECK-LABEL: convert_f32_i32
// A32: vcvt
// A64: fcvtzs
// SOFT32: bl __aeabi_f2iz
// SOFT64: bl __fixsfsi
#[no_mangle]
pub fn convert_f32_i32(a: f32) -> i32 {
    a as i32
}

// CHECK-LABEL: convert_i32_f32
// A32: vcvt
// A64: scvtf
// SOFT32: bl __aeabi_i2f
// SOFT64: bl __floatsisf
#[no_mangle]
pub fn convert_i32_f32(a: i32) -> f32 {
    a as f32
}

// CHECK-LABEL: div_f32
// A32: vdiv.f32
// A64: fdiv {{s[0-9]+}}
// SOFT32: bl __aeabi_fdiv
// SOFT64: bl __divsf3
#[no_mangle]
pub fn div_f32(a: f32, b: f32) -> f32 {
    a / b
}

// Multiply-add chain: A32 fuses to vmla, A64 emits fmul+fadd (no fast-math).
// Softfloat emits both libcalls in order.
// CHECK-LABEL: mad_f32
// A32: vmla.f32
// A64: fmul {{s[0-9]+}}
// A64: fadd {{s[0-9]+}}
// SOFT32: bl __aeabi_fmul
// SOFT32: bl __aeabi_fadd
// SOFT64: bl __mulsf3
// SOFT64: bl __addsf3
#[no_mangle]
pub fn mad_f32(a: f32, b: f32, c: f32) -> f32 {
    a * b + c
}

// Float compare must be vcmp+vmrs (A32) / fcmp (A64), libcall-free on hf.
// Softfloat compares via aeabi/compiler-rt helpers (NaN-aware families).
// CHECK-LABEL: min_f32
// A32: vcmp.f32
// A64: fcmp {{s[0-9]+}}
// SOFT32: __aeabi_fcmp
// SOFT64: sf2
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
// SOFT32: bl __aeabi_fmul
// SOFT64: bl __mulsf3
#[no_mangle]
pub fn mul_f32(a: f32, b: f32) -> f32 {
    a * b
}

// Double precision: A64/v7 have full DP (fmul/vmul.f64); armv8-R default is
// SP-only so DP lowers to __aeabi_dmul there.
// CHECK-LABEL: mul_f64
// A32R8: bl {{__aeabi_dmul|__muldf3}}
// A32V7: vmul.f64
// A64: fmul {{d[0-9]+}}, {{d[0-9]+}}, {{d[0-9]+}}
// SOFT32: bl __aeabi_dmul
// SOFT64: bl __muldf3
#[no_mangle]
pub fn mul_f64(a: f64, b: f64) -> f64 {
    a * b
}

// Integer ALU is unaffected by the float ABI.
// CHECK-LABEL: soft_add_u32
// SOFT32: add {{r[0-9]+}}, {{r[0-9]+}}, {{r[0-9]+}}
// SOFT64: add {{w[0-9]+}}, {{w[0-9]+}}, {{w[0-9]+}}
#[no_mangle]
pub fn soft_add_u32(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

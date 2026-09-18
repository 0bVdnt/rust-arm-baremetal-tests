// Assembly test 4/4 (softfloat-only): soft-float ABI lowers FP to libcalls.
// Reuse pattern: rust tests/assembly-llvm soft-ABI tests
// (riscv-soft-abi-with-float-features.rs, s390x-softfloat-abi.rs) adapted to
// ARM bare-metal softfloat triples. Only runs on softfloat targets — the
// hardfloat file (hardfloat_f32_mul.rs) is the hf/A64 counterpart.
// Thorough: f32 mul/add go through aeabi/compiler-rt libcalls with integer
// regs (no VFP/NEON), while integer ALU stays single-instruction.
// NOTE: functions alphabetical (LLVM emits sorted by symbol, 1.98.1).
//
//@ assembly-output: emit-asm
// RUN: rustc --target=armv7a-none-eabi --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,SOFT32
// RUN: rustc --target=aarch64-unknown-none-softfloat --emit=asm -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,SOFT64

#![crate_type = "lib"]
#![no_std]

// CHECK-LABEL: soft_add_f32
// SOFT32-NOT: vadd
// SOFT64-NOT: fadd {{s[0-9]+}}
// SOFT32: bl __aeabi_fadd
// SOFT64: bl __addsf3
#[no_mangle]
pub fn soft_add_f32(a: f32, b: f32) -> f32 {
    a + b
}

// Integer ALU is unaffected by the float ABI.
// CHECK-LABEL: soft_add_u32
// SOFT32: add {{r[0-9]+}}, {{r[0-9]+}}, {{r[0-9]+}}
// SOFT64: add {{w[0-9]+}}, {{w[0-9]+}}, {{w[0-9]+}}
#[no_mangle]
pub fn soft_add_u32(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

// CHECK-LABEL: soft_mul_f32
// SOFT32-NOT: vmul
// SOFT64-NOT: fmul {{s[0-9]+}}
// SOFT32: bl __aeabi_fmul
// SOFT64: bl __mulsf3
#[no_mangle]
pub fn soft_mul_f32(a: f32, b: f32) -> f32 {
    a * b
}

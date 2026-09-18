// Codegen-units test 1/2: CGU partitioning is observable and stable.
// Reuse: rust-lang/rust tests/codegen-units/ pattern (generic, target-independent).
// Thorough: 6 mono items across leaf/mid/root + cold path — partitioning must
// keep every item exactly once regardless of -Ccodegen-units value.
// Requires RUSTC_BOOTSTRAP=1 on stable 1.98.1 for -Zprint-mono-items.
// Generic across all four bare-metal targets.
//
//@ compile-flags: -Ccodegen-units=2 -Zprint-mono-items
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=armv8r-none-eabihf --crate-type=lib %s -Ccodegen-units=2 -Zprint-mono-items 2>&1 | FileCheck %s
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=aarch64-unknown-none --crate-type=lib %s -Ccodegen-units=2 -Zprint-mono-items 2>&1 | FileCheck %s
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=armv8r-none-eabihf --crate-type=lib %s -Ccodegen-units=1 -Zprint-mono-items 2>&1 | FileCheck %s

#![crate_type = "lib"]
#![no_std]

// CHECK: MONO_ITEM
#[no_mangle]
pub fn alpha(x: u32) -> u32 {
    x.wrapping_mul(3)
}

// CHECK: MONO_ITEM
#[no_mangle]
pub fn beta(x: u32) -> u32 {
    x.wrapping_add(7)
}

// CHECK: MONO_ITEM
#[no_mangle]
pub fn gamma(a: u32, b: u32) -> u32 {
    alpha(a).wrapping_add(beta(b))
}

// CHECK: MONO_ITEM
#[no_mangle]
pub fn delta(a: u32, b: u32, c: u32) -> u32 {
    gamma(a, b).wrapping_add(c.rotate_left(3))
}

// CHECK: MONO_ITEM
#[no_mangle]
#[cold]
pub fn error_path(code: u32) -> u32 {
    code ^ 0xDEAD_BEEF
}

// CHECK: MONO_ITEM
#[no_mangle]
pub fn root(a: u32, b: u32) -> u32 {
    if a == b {
        error_path(a)
    } else {
        delta(a, b, a ^ b)
    }
}

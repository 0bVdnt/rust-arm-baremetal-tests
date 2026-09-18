// Codegen-units test 2/2: generics monomorphize into distinct mono items,
// including nested generic + const-generic + trait-bound cases.
// Reuse: rust-lang/rust tests/codegen-units/ generic-mono pattern.
// Requires RUSTC_BOOTSTRAP=1 on stable. Generic across ARM triples.
// NOTE: CHECK lines are alphabetical — `-Zprint-mono-items` emits sorted
// by symbol on 1.98.1 (verified on armv8r + aarch64).
//
//@ compile-flags: -Ccodegen-units=1 -Zprint-mono-items
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=armv8r-none-eabihf --crate-type=lib %s -Ccodegen-units=1 -Zprint-mono-items 2>&1 | FileCheck %s
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=aarch64-unknown-none --crate-type=lib %s -Ccodegen-units=1 -Zprint-mono-items 2>&1 | FileCheck %s

#![crate_type = "lib"]
#![no_std]

pub fn array_sum<const N: usize>(a: [u32; N]) -> u32 {
    a.iter().fold(0, |s, x| s.wrapping_add(*x))
}

// CHECK: MONO_ITEM {{.*}}array_sum
#[no_mangle]
pub fn use_array(a: [u32; 4]) -> u32 {
    array_sum(a)
}

pub fn clamp<T: PartialOrd>(x: T, lo: T, hi: T) -> T {
    if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}

// CHECK: MONO_ITEM {{.*}}clamp
#[no_mangle]
pub fn use_clamp(x: u32) -> u32 {
    clamp(x, 10, 100)
}

pub fn ident<T>(x: T) -> T {
    x
}

// CHECK: MONO_ITEM {{.*}}ident{{.*}}u32
#[no_mangle]
pub fn use_u32(x: u32) -> u32 {
    ident(x)
}

// CHECK: MONO_ITEM {{.*}}ident{{.*}}u64
#[no_mangle]
pub fn use_u64(x: u64) -> u64 {
    ident(x)
}

pub fn pair<A, B>(a: A, b: B) -> (B, A) {
    (b, a)
}

// CHECK: MONO_ITEM {{.*}}pair
#[no_mangle]
pub fn use_pair(a: u16, b: u32) -> (u32, u16) {
    pair(a, b)
}

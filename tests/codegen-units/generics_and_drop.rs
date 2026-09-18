// Codegen-units test 2/2: generics, drop glue, consts, trait objects.
// Reuse: rust-lang/rust tests/codegen-units/ generic-mono + drop-glue patterns.
// Requires RUSTC_BOOTSTRAP=1 on stable for -Zprint-mono-items.
// NOTE: CHECK lines are byte-alphabetical — `-Zprint-mono-items` emits
// sorted by symbol on 1.98.1 (`<Trait>` items first), verified on armv8r.
//@ compile-flags: -Ccodegen-units=1 -Zprint-mono-items
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=armv8r-none-eabihf --crate-type=lib %s -Ccodegen-units=1 -Zprint-mono-items 2>&1 | FileCheck %s
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=aarch64-unknown-none --crate-type=lib %s -Ccodegen-units=1 -Zprint-mono-items 2>&1 | FileCheck %s

#![crate_type = "lib"]
#![no_std]

pub struct Guard(u32);

impl Drop for Guard {
    fn drop(&mut self) {}
}

pub struct Sensor(u32);

pub trait Describe {
    fn describe(&self) -> u32;
}

impl Describe for Sensor {
    fn describe(&self) -> u32 {
        self.0
    }
}

pub fn array_sum<const N: usize>(a: [u32; N]) -> u32 {
    a.iter().fold(0, |s, x| s.wrapping_add(*x))
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

pub const fn const_double(x: u32) -> u32 {
    x * 2
}

pub fn ident<T>(x: T) -> T {
    x
}

pub fn pair<A, B>(a: A, b: B) -> (B, A) {
    (b, a)
}

// CHECK: MONO_ITEM {{.*}}Guard{{.*}}Drop
#[no_mangle]
pub fn use_guard(x: u32) -> u32 {
    let g = Guard(x);
    g.0.wrapping_add(1)
}

// CHECK: MONO_ITEM {{.*}}describe
#[no_mangle]
pub fn use_dyn(s: &Sensor) -> u32 {
    let d: &dyn Describe = s;
    d.describe()
}

// CHECK: MONO_ITEM {{.*}}array_sum
#[no_mangle]
pub fn use_array(a: [u32; 4]) -> u32 {
    array_sum(a)
}

// CHECK: MONO_ITEM {{.*}}clamp
#[no_mangle]
pub fn use_clamp(x: u32) -> u32 {
    clamp(x, 10, 100)
}

// CHECK: MONO_ITEM {{.*}}const_double
#[no_mangle]
pub static ANSWER: u32 = const_double(21);

// CHECK: MONO_ITEM {{.*}}drop_glue
#[no_mangle]
pub fn use_answer() -> u32 {
    ANSWER
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

// CHECK: MONO_ITEM {{.*}}pair
#[no_mangle]
pub fn use_pair(a: u16, b: u32) -> (u32, u16) {
    pair(a, b)
}

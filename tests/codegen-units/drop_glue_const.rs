// Codegen-units test 3/3: drop glue, consts, and trait objects get mono items.
// Reuse: rust-lang/rust tests/codegen-units/ drop-glue pattern.
// Requires RUSTC_BOOTSTRAP=1 on stable for -Zprint-mono-items.
// Generic across ARM triples.
//
//@ compile-flags: -Zprint-mono-items
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=armv8r-none-eabihf --crate-type=lib %s -Zprint-mono-items 2>&1 | FileCheck %s
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=aarch64-unknown-none --crate-type=lib %s -Zprint-mono-items 2>&1 | FileCheck %s

#![crate_type = "lib"]
#![no_std]

pub struct Guard(u32);

impl Drop for Guard {
    fn drop(&mut self) {}
}

// CHECK: MONO_ITEM
#[no_mangle]
pub fn use_guard(x: u32) -> u32 {
    let g = Guard(x);
    g.0.wrapping_add(1)
}

pub const fn const_double(x: u32) -> u32 {
    x * 2
}

// CHECK: MONO_ITEM
#[no_mangle]
pub static ANSWER: u32 = const_double(21);

// CHECK: MONO_ITEM
#[no_mangle]
pub fn use_answer() -> u32 {
    ANSWER
}

pub trait Describe {
    fn describe(&self) -> u32;
}

pub struct Sensor(u32);

impl Describe for Sensor {
    fn describe(&self) -> u32 {
        self.0
    }
}

// CHECK: MONO_ITEM
#[no_mangle]
pub fn use_dyn(s: &Sensor) -> u32 {
    let d: &dyn Describe = s;
    d.describe()
}

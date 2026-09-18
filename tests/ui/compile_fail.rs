// UI test 1/2: borrowck + type diagnostics are precise on ARM targets.
// Reuse: rust-lang/rust tests/ui/ pattern (blessed .stderr, $DIR normalization).
// Thorough: two E0308 mismatches plus E0499 (double mut borrow — the classic
// ISR/main shared-buffer firmware bug) and E0506 (assign while borrowed).
//@ check-fail
// RUN: rustc --target=armv8r-none-eabihf --crate-type=lib --emit=metadata %s 2>&1 | FileCheck %s
// (Canonical: compare against compile_fail.stderr via blessed snapshot.)

#![no_std]

pub fn mismatch() -> u32 {
    let x: u32 = "arm";
    //~^ ERROR mismatched types
    x
}

pub fn mismatch_bool() -> bool {
    let y: bool = 42u32;
    //~^ ERROR mismatched types
    y
}

pub fn double_borrow(buf: &mut [u32]) -> u32 {
    let a = &mut buf[0];
    let b = &mut buf[1];
    // use both so neither borrow is dead
    *a += *b;
    *a
}

pub fn alias_and_write(x: &mut u32) -> u32 {
    let r = &*x;
    *x = 1;
    //~^ ERROR cannot assign to `*x` because it is borrowed
    *r
}

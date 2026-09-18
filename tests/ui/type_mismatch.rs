// UI test 1/2: type mismatch diagnostics are precise on ARM targets.
// Reuse: rust-lang/rust tests/ui/ pattern (blessed .stderr, $DIR normalization).
// Thorough: two E0308 shapes (u32/&str, bool/u32) at different lines —
// normalization ($DIR, LL) and multi-error emission both exercised, and the
// same .stderr holds for all four bare-metal triples (target-independent).
//@ check-fail
// RUN: rustc --target=armv8r-none-eabihf --crate-type=lib --emit=metadata %s 2>&1 | FileCheck %s
// (Canonical: compare against type_mismatch.stderr; suite-runner/tests/ui.rs
//  checks it byte-identically after $DIR normalization.)

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

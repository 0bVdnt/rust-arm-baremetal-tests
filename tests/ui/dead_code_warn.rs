// UI test 2/2: unused-code + naming warnings snapshot cross-target.
// Reuse: rust tests/ui/ lint snapshot pattern.
// Thorough: dead function, unused variable, unused import, non-snake-case fn —
// four lint shapes in one file, all reproducible with
// --target=<any of the four bare-metal triples>.
//@ build-pass
// RUN: rustc --target=armv8r-none-eabihf --crate-type=lib --emit=metadata %s 2>&1 | FileCheck %s --check-prefixes=CHECK
// CHECK: warning: function `never_used` is never used
// CHECK: warning: unused variable
// CHECK: warning: unused import
// CHECK: should have a snake case name
#![crate_type = "lib"]
#![no_std]

use core::mem as _mem_alias_unused;

fn never_used() -> u32 {
    0xBEEF
}

#[no_mangle]
pub fn used() -> u32 {
    let unused_var = 40u32;
    42
}

#[no_mangle]
pub fn BadName() -> u32 {
    42
}

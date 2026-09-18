// UI test 3/3: borrow checker rejects overlapping mutable borrows.
// Reuse: rust-lang/rust tests/ui/ borrowck pattern (blessed diagnostics).
// Thorough: double-mut-borrow (E0499) at two sites — the classic firmware
// bug (ISR + main sharing a static mut buffer), caught at compile time.
//@ check-fail
// RUN: rustc --target=armv8r-none-eabihf --crate-type=lib --emit=metadata %s 2>&1 | FileCheck %s
// CHECK: cannot borrow `buf[_]` as mutable more than once
// CHECK: cannot assign to `*x` because it is borrowed
#![no_std]

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

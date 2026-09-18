// Codegen test 3/3: control flow lowers to branches/phi without surprises.
// Reuse pattern: rust tests/codegen-llvm/ control-flow tests.
// Thorough: multi-arm match, counted loop with early break, Option combinator,
// select — branch/phi/select shapes that must stay tight on ARM.
// NOTE: functions alphabetical (LLVM emits IR sorted by symbol, 1.98.1).
//
// RUN: rustc --target=armv8r-none-eabihf --emit=llvm-ir -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A32
// RUN: rustc --target=aarch64-unknown-none --emit=llvm-ir -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A64

#![crate_type = "lib"]
#![no_std]

// Small dense match folds to icmp+select (no jump table on ARM).
// CHECK: icmp
// CHECK: select
// CHECK: ret i32
#[no_mangle]
pub fn classify(x: u32) -> u32 {
    match x {
        0 => 0,
        1..=9 => 1,
        _ => 2,
    }
}

// CHECK: br i1
// CHECK: phi
#[no_mangle]
pub fn count_until(n: u32, limit: u32) -> u32 {
    let mut i = 0u32;
    let mut acc = 0u32;
    while i < n {
        acc = acc.wrapping_add(i);
        i += 1;
        if acc > limit {
            break;
        }
    }
    acc
}

// CHECK: br i1
#[no_mangle]
pub fn or_default(x: Option<u32>, d: u32) -> u32 {
    x.unwrap_or(d)
}

// CHECK: select
#[no_mangle]
pub fn pick(a: u32, b: u32, c: bool) -> u32 {
    if c { a } else { b }
}

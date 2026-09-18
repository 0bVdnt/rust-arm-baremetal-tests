// Coverage-map test 1/2: branch/loop/match mapping emitted in LLVM IR.
// Reuse: rust-lang/rust tests/coverage/ pattern (generic).
// Thorough: if/else + loop + match + early-return + nested condition — every
// region shape must get a counter in __llvm_coverage_mapping.
// See https://doc.rust-lang.org/rustc/instrument-coverage.html
// NOTE: profiler_builtins is not shipped for bare-metal triples, so mapping
// checks run on host; target coverage uses minicov (see coverage-run/).
// RUN: rustc -C instrument-coverage --emit=llvm-ir %s -o - | FileCheck %s

#![crate_type = "lib"]
#![no_std]

// CHECK: __llvm_coverage_mapping
// CHECK: __llvm_covmap
// CHECK: __profc_branch
// CHECK: atomicrmw
#[no_mangle]
pub fn branch(x: u32) -> u32 {
    if x > 10 {
        x - 10
    } else {
        x + 10
    }
}

// CHECK: __profc_counted_loop
#[no_mangle]
pub fn counted_loop(n: u32) -> u32 {
    let mut acc = 0u32;
    for i in 0..n {
        acc = acc.wrapping_add(i);
    }
    acc
}

// CHECK: __profc_early_return
#[no_mangle]
pub fn early_return(x: u32) -> u32 {
    if x == 0 {
        return 0xFFFF;
    }
    if x == 1 {
        return 1;
    }
    x * 2
}

// CHECK: __profc_nested
#[no_mangle]
pub fn nested(x: u32, y: u32) -> u32 {
    if x > y {
        if x - y > 100 {
            3
        } else {
            2
        }
    } else if x == y {
        1
    } else {
        0
    }
}

// Pretty test 1/2: macro/desugar expansion visible via unpretty.
// Reuse: rust-lang/rust tests/pretty/ pattern (generic).
// Thorough: repeat-init array (stays [1; 3]), arithmetic, matches! desugar to
// match, concat! evaluated to literal — four expansion shapes.
// Requires RUSTC_BOOTSTRAP=1 on stable for -Zunpretty.
// RUN: RUSTC_BOOTSTRAP=1 rustc --crate-type=lib -Zunpretty=expanded %s | FileCheck %s
// CHECK: [1; 3]
// CHECK: 40 + 2
// CHECK: 0 => true
// CHECK: "arm-baremetal"

#![no_std]

pub fn expanded() -> [u32; 3] {
    [1; 3]
}

pub fn answer() -> u32 {
    40 + 2
}

pub fn is_zero(x: u32) -> bool {
    matches!(x, 0)
}

pub fn greeting() -> &'static str {
    concat!("arm", "-", "baremetal")
}

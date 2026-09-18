// Pretty test 2/2: HIR typed pretty-print shows inferred types everywhere.
// Reuse: rust tests/pretty/ typed pattern (hir,typed is the 1.98 mode name).
// Thorough: integer, float, tuple, and generic-instantiated bindings —
// each shows `as <type>` annotations in the typed HIR dump.
// RUN: RUSTC_BOOTSTRAP=1 rustc --crate-type=lib -Zunpretty=hir,typed %s | FileCheck %s
// CHECK: (41u32 as u32)
// CHECK: (1.5f32 as f32)
// CHECK: ((7u32 as u32), (true as bool))
// CHECK: fn(u64) -> u64

#![no_std]

pub fn typed() -> u32 {
    let x = 41u32;
    x + 1
}

pub fn typed_float() -> f32 {
    let y = 1.5f32;
    y * 2.0
}

pub fn typed_tuple() -> (u32, bool) {
    let t = (7u32, true);
    t
}

pub fn typed_generic() -> u64 {
    fn id<T>(x: T) -> T {
        x
    }
    let z = id(99u64);
    z
}

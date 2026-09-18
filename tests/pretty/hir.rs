// Pretty test 2/2: HIR views show types and crate structure.
// Reuse: rust tests/pretty/ typed + hir-tree patterns
// (hir,typed is the 1.98 mode name).
// Thorough: integer/float/tuple/generic bindings show `as <type>`
// annotations under hir,typed; the hir-tree dump lists every item path.
// RUN: RUSTC_BOOTSTRAP=1 rustc --crate-type=lib -Zunpretty=hir,typed %s | FileCheck %s --check-prefixes=CHECK,HT
// RUN: RUSTC_BOOTSTRAP=1 rustc --crate-type=lib -Zunpretty=hir-tree %s | FileCheck %s --check-prefixes=CHECK,HTREE
// HT: (41u32 as u32)
// HT: (1.5f32 as f32)
// HT: ((7u32 as u32), (true as bool))
// HT: fn(u64) -> u64
// HTREE: ::first
// HTREE: ::second
// HTREE: ::Config
// HTREE: ::LIMIT
// HTREE: ::typed
// HTREE: ::typed_float
// HTREE: ::typed_tuple
// HTREE: ::typed_generic

#![no_std]

pub struct Config {
    pub baud: u32,
}

pub const LIMIT: u32 = 128;

pub fn first(x: u32) -> u32 {
    x.wrapping_add(1)
}

pub fn second(c: &Config) -> u32 {
    c.baud
}

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

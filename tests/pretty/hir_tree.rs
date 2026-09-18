// Pretty test 3/3: HIR tree dump shows crate structure.
// Reuse: rust tests/pretty/ hir-tree pattern.
// Thorough: two fns + a struct + a const — the tree must list all four items
// with their paths, proving the frontend ingested the whole module.
// RUN: RUSTC_BOOTSTRAP=1 rustc --crate-type=lib -Zunpretty=hir-tree %s | FileCheck %s
// CHECK: ::first
// CHECK: ::second
// CHECK: ::Config
// CHECK: ::LIMIT

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

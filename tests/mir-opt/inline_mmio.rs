// MIR-opt test 2/2: #[inline] helpers inlined at mir-opt-level.
// Reuse: rust tests/mir-opt/ inline pattern.
// Thorough: int helper, branchy helper, and generic helper — inlining must
// apply to all three call sites, leaving no `call` to the helpers.
// RUN: RUSTC_BOOTSTRAP=1 rustc --target=armv8r-none-eabihf --emit=mir -Zmir-opt-level=2 %s -o - | FileCheck %s
// CHECK-NOT: call {{.*}}mmio_add
// CHECK-NOT: call {{.*}}clamp_u32
// CHECK-NOT: call {{.*}}ident_u32

#![crate_type = "lib"]
#![no_std]

#[inline(always)]
pub fn mmio_add(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

#[inline(always)]
pub fn clamp_u32(x: u32, lo: u32, hi: u32) -> u32 {
    if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}

#[inline(always)]
pub fn ident_u32(x: u32) -> u32 {
    x
}

#[no_mangle]
pub fn drive(a: u32) -> u32 {
    mmio_add(a, 0x1000)
}

#[no_mangle]
pub fn drive_clamp(x: u32) -> u32 {
    clamp_u32(x, 10, 100)
}

#[no_mangle]
pub fn drive_ident(x: u32) -> u32 {
    ident_u32(x).wrapping_add(1)
}

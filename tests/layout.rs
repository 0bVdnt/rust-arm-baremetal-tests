//! Layout / C-ABI checks (mirrors tests/codegen/align_repr_c_struct.rs).
//! Proves repr(C) sizes and field access on-target for both 32- and 64-bit
//! bare-metal (generic via core::mem, no target-specific code).

use helper::testcase;

#[repr(C)]
struct Regs {
    ctrl: u32,
    status: u16,
    flags: u8,
}

#[testcase]
fn size_of_u32() {
    assert_eq!(core::mem::size_of::<u32>(), 4);
}

#[testcase]
fn align_of_u64() {
    assert_eq!(core::mem::align_of::<u64>(), 8);
}

#[testcase]
fn tuple_layout() {
    assert_eq!(core::mem::size_of::<(u32, u16, u8)>(), 8);
}

#[testcase]
fn regs_size() {
    assert_eq!(core::mem::size_of::<Regs>(), 8);
}

#[testcase]
fn regs_fields() {
    let r = Regs { ctrl: 0x1234_5678, status: 0xABCD, flags: 0xEF };
    assert_eq!(r.ctrl, 0x1234_5678);
    assert_eq!(r.status, 0xABCD);
    assert_eq!(r.flags, 0xEF);
}

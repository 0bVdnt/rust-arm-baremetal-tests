//! Layout / C-ABI checks (mirrors tests/codegen/align_repr_c_struct.rs).
//! Proves repr(C) sizes and field access on-target for both 32- and 64-bit
//! bare-metal (generic via core::mem, no target-specific code).

use core::sync::atomic::AtomicU32;
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

#[testcase]
fn scalar_sizes() {
    assert_eq!(core::mem::size_of::<u64>(), 8);
    assert_eq!(core::mem::size_of::<i64>(), 8);
    assert_eq!(core::mem::size_of::<f64>(), 8);
    assert_eq!(core::mem::size_of::<bool>(), 1);
    assert_eq!(core::mem::size_of::<[u32; 4]>(), 16);
    assert_eq!(core::mem::align_of::<AtomicU32>(), 4);
}

#[testcase]
fn pointer_width() {
    use core::sync::atomic::AtomicU32;
    #[cfg(target_pointer_width = "64")]
    {
        assert_eq!(core::mem::size_of::<usize>(), 8);
        assert_eq!(core::mem::size_of::<&u32>(), 8);
    }
    #[cfg(target_pointer_width = "32")]
    {
        assert_eq!(core::mem::size_of::<usize>(), 4);
        assert_eq!(core::mem::size_of::<&u32>(), 4);
    }
    // &u32 is always one word, whatever the width.
    assert_eq!(core::mem::size_of::<&u32>(), core::mem::size_of::<usize>());
    assert_eq!(core::mem::size_of::<AtomicU32>(), 4);
}

#[testcase]
fn niche_layouts() {
    use core::num::NonZeroU32;
    // Niche-filling: Option costs no extra tag word.
    assert_eq!(core::mem::size_of::<Option<u32>>(), 8);
    assert_eq!(core::mem::size_of::<Option<NonZeroU32>>(), 4);
    assert_eq!(core::mem::size_of::<Option<&u32>>(), core::mem::size_of::<usize>());
    assert_eq!(None::<NonZeroU32>.is_none(), true);
}

#[testcase]
fn field_offsets() {
    assert_eq!(core::mem::offset_of!(Regs, ctrl), 0);
    assert_eq!(core::mem::offset_of!(Regs, status), 4);
    assert_eq!(core::mem::offset_of!(Regs, flags), 6);
}

#[testcase]
fn unsized_sizes() {
    let s: &[u32] = &[1, 2, 3];
    assert_eq!(core::mem::size_of_val(s), 12);
    assert_eq!(core::mem::size_of_val("arm"), 3);
    assert_eq!(core::mem::size_of_val(&s), core::mem::size_of::<usize>() * 2);
}

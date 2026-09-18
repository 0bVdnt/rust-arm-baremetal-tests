// Codegen test 1/2: C-ABI layouts + control flow lower cleanly on ARM.
// Reuse pattern: rust tests/assembly-llvm/*-struct-abi.rs + codegen-llvm
// control-flow tests, adapted to bare-metal.
// Thorough: MMIO reg block (align 4), u64 DMA descriptor (align 8 even on
// 32-bit), packed wire header (unaligned), value-aggregate swap, volatile
// MMIO, plus match-folded-to-select, counted loop with phi, Option unwrap.
// NOTE: functions alphabetical — LLVM emits IR sorted by symbol (1.98.1).
//
// RUN: rustc --target=armv8r-none-eabihf --emit=llvm-ir -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A32,A32R8
// RUN: rustc --target=aarch64-unknown-none --emit=llvm-ir -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A64
// RUN: rustc --target=armv7a-none-eabi --emit=llvm-ir -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A32,A32V7

#![crate_type = "lib"]
#![no_std]

#[repr(C)]
pub struct Regs {
    pub ctrl: u32,
    pub status: u16,
    pub flags: u8,
}

#[repr(C)]
pub struct DmaDesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u32,
}

#[repr(C, packed)]
pub struct WireHdr {
    pub kind: u8,
    pub len: u16,
    pub tag: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pair {
    pub lo: u32,
    pub hi: u32,
}

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

// CHECK: volatile
#[no_mangle]
pub unsafe fn mmio_read(addr: *const u32) -> u32 {
    unsafe { core::ptr::read_volatile(addr) }
}

// CHECK: volatile
#[no_mangle]
pub unsafe fn mmio_write(addr: *mut u32, val: u32) {
    unsafe { core::ptr::write_volatile(addr, val) }
}

// Option unwrap folds to select on optimal targets (no branch).
// CHECK: select
#[no_mangle]
pub fn or_default(x: Option<u32>, d: u32) -> u32 {
    x.unwrap_or(d)
}

// CHECK: select
#[no_mangle]
pub fn pick(a: u32, b: u32, c: bool) -> u32 {
    if c { a } else { b }
}

// CHECK: align 4
// CHECK: i32
// A32: align 4
// A64: align 4
#[no_mangle]
pub fn read_ctrl(r: &Regs) -> u32 {
    r.ctrl
}

// CHECK: i64
// CHECK: align 8
#[no_mangle]
pub fn read_dma_addr(d: &DmaDesc) -> u64 {
    d.addr
}

// CHECK: i32
#[no_mangle]
pub fn read_dma_len(d: &DmaDesc) -> u32 {
    d.len
}

// Offset 4 in an align-4 struct is itself 4-aligned: LLVM loads i16 @ align 4.
// CHECK: i16
// CHECK: align 4
#[no_mangle]
pub fn read_status(r: &Regs) -> u16 {
    r.status
}

// CHECK: i8
// CHECK: i16
#[no_mangle]
pub fn read_wire_kind(h: &WireHdr) -> u8 {
    // packed => unaligned; must not assume align 2/4.
    unsafe { core::ptr::addr_of!((*h).kind).read_unaligned() }
}

// CHECK: align 1
#[no_mangle]
pub fn read_wire_len(h: &WireHdr) -> u16 {
    unsafe { core::ptr::addr_of!((*h).len).read_unaligned() }
}

// Pair (2×u32) passes as two i32 scalars / {i32,i32} — no padding, no sret.
// CHECK: i32
#[no_mangle]
pub fn swap_pair(p: Pair) -> Pair {
    Pair { lo: p.hi, hi: p.lo }
}

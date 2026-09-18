// Codegen test 1/2: repr(C) layouts keep C ABI size/align on ARM.
// Reuse pattern: rust tests/assembly-llvm/*-struct-abi.rs adapted to ARM.
// Thorough: u32 MMIO reg block, 64-bit DMA descriptor, packed wire header,
// value-aggregate swap, volatile MMIO — size/align/load/store ABI-correct on
// 32-bit (4-byte ptr) and 64-bit (8-byte ptr) bare-metal.
// NOTE: functions alphabetical — LLVM emits IR sorted by symbol (1.98.1).
//
// RUN: rustc --target=armv8r-none-eabihf --emit=llvm-ir -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A32,A32R8
// RUN: rustc --target=aarch64-unknown-none --emit=llvm-ir -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A64
// RUN: rustc --target=armv7a-none-eabihf --emit=llvm-ir -C opt-level=2 %s -o - | FileCheck %s --check-prefixes=CHECK,A32,A32V7

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

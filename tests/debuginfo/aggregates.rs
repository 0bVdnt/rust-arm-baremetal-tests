// Debuginfo test 2/2: DWARF aggregates — structs, enums, arrays, consts.
// Reuse: rust tests/debuginfo/ struct/ptype + aggregate patterns.
// Thorough: repr(C) reg block (field offsets), status enum (discriminant),
// nested descriptor, fixed array element, slice len/ptr, const print:
//   gdb> ptype Regs   -> struct with ctrl:u32, status:u16
//   gdb> print r.ctrl -> correct value
//   gdb> ptype Mode   -> enum with Idle/Run/Fault
//   gdb> print buf[2] -> 30
//   gdb> print LIMIT  -> 128
//
// Build: rustc --target=aarch64-unknown-none -g -C opt-level=0 --crate-type=lib %s
// CHECK-gdb: type = struct Regs
// CHECK-gdb: ctrl : u32
// CHECK-gdb: status : u16
// CHECK-gdb: type = enum Mode
// CHECK-gdb: $1 = 30
// CHECK-gdb: $3 = 128

#![crate_type = "lib"]
#![no_std]

pub const LIMIT: usize = 128;

#[repr(C)]
pub struct Regs {
    pub ctrl: u32,
    pub status: u16,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Idle = 0,
    Run = 1,
    Fault = 2,
}

#[repr(C)]
pub struct Device {
    pub regs: Regs,
    pub mode: Mode,
    pub irq: u8,
}

#[no_mangle]
#[inline(never)]
pub fn read_regs(r: &Regs) -> u32 {
    // gdb-break: read_regs
    r.ctrl + r.status as u32
}

#[no_mangle]
#[inline(never)]
pub fn mode_id(m: Mode) -> u8 {
    // gdb-break: mode_id, print m shows discriminant
    m as u8
}

#[no_mangle]
#[inline(never)]
pub fn device_irq(d: &Device) -> u8 {
    // gdb-break: device_irq, print d.regs.ctrl / d.mode / d.irq
    d.regs.ctrl.wrapping_add(d.irq as u32) as u8 ^ (d.mode as u8)
}

#[no_mangle]
#[inline(never)]
pub fn sum3(buf: &[u32]) -> u32 {
    // gdb-break: sum3, print buf[0] + buf[1] + buf[2], print buf.len()
    buf[0].wrapping_add(buf[1]).wrapping_add(buf[2])
}

#[no_mangle]
#[inline(never)]
pub fn fill() -> [u32; 4] {
    // gdb-break: fill, print arr after loop
    let mut arr = [0u32; 4];
    for (i, slot) in arr.iter_mut().enumerate() {
        *slot = (i as u32 + 1) * 10;
    }
    arr
}

#[no_mangle]
pub fn drive() -> u32 {
    let arr = fill();
    // arr[2] == 30, len 4, LIMIT 128
    sum3(&arr[0..3]).wrapping_add(LIMIT as u32)
}

// Debuginfo test 2/2: DWARF struct/enum layout visible in GDB.
// Reuse: rust tests/debuginfo/ struct/ptype pattern.
// Thorough: repr(C) reg block (field offsets), status enum (discriminant),
// nested descriptor — `ptype` + `print` must all agree with the C ABI.
//   gdb> ptype Regs   -> struct with ctrl:u32, status:u16
//   gdb> print r.ctrl -> correct value
//   gdb> ptype Mode   -> enum with Idle/Run/Fault
//
// Build: rustc --target=aarch64-unknown-none -g -C opt-level=0 --crate-type=lib %s
// CHECK-gdb: type = struct Regs
// CHECK-gdb: ctrl : u32
// CHECK-gdb: status : u16
// CHECK-gdb: type = enum Mode

#![crate_type = "lib"]
#![no_std]

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

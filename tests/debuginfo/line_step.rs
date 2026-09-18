// Debuginfo test 1/2: line tables + locals + call args via GDB.
// Reuse: rust-lang/rust tests/debuginfo/ gdb pattern (break/print/backtrace).
// Thorough: arithmetic helper (arg print), loop accumulator (local print +
// step), nested call (backtrace) — the three interactive operations a
// bare-metal bring-up session needs.
// Bare-metal: run under QEMU gdbstub, debug with system gdb
// (built --enable-targets=all, supports arm/aarch64).
//
// Build:
//   rustc --target=armv8r-none-eabihf -g -C opt-level=0 --crate-type=lib %s -o /tmp/dbg1.o
// Debug:
//   qemu-system-arm -M versatileab -cpu cortex-r52 -semihosting -nographic -audio none \
//     -gdb tcp::1234 -S -kernel <firmware-with-this-object> &
//   gdb -ex 'target remote :1234' -ex 'break tick' -ex continue \
//       -ex 'print x' -ex 'print acc' -ex 'bt' -ex quit | FileCheck %s --check-prefix gdb
//
// CHECK-gdb: $1 = 41
// CHECK-gdb: $2 = 42

#![crate_type = "lib"]
#![no_std]

#[no_mangle]
#[inline(never)]
pub fn tick(x: u32) -> u32 {
    // gdb-break: tick
    x + 1
}

#[no_mangle]
#[inline(never)]
pub fn accumulate(n: u32) -> u32 {
    // gdb-break: accumulate loop-step through `acc`
    let mut acc = 0u32;
    for i in 0..n {
        acc = acc.wrapping_add(i);
    }
    acc
}

#[no_mangle]
pub fn drive() -> u32 {
    // gdb-break: drive then `bt` shows drive -> tick
    tick(41) + accumulate(4)
}

// Debuginfo test 3/3: arrays, slices, and consts in GDB.
// Reuse: rust tests/debuginfo/ aggregate pattern.
// Thorough: fixed array element print, slice len/ptr print, const print —
// `print arr[2]`, `print sl.len()`, `print LIMIT` must all resolve.
//   gdb> print buf[2]  -> 30
//   gdb> print n       -> 3
//   gdb> print LIMIT   -> 128
//
// Build: rustc --target=armv8r-none-eabihf -g -C opt-level=0 --crate-type=lib %s
// CHECK-gdb: \$1 = 30
// CHECK-gdb: \$2 = 3
// CHECK-gdb: \$3 = 128

#![crate_type = "lib"]
#![no_std]

pub const LIMIT: usize = 128;

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

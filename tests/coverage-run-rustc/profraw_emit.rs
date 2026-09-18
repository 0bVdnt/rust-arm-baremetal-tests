// Coverage-run-rustc test 1/2: firmware emits .profraw via semihosting.
// Upstream coverage-run-rustc runs an *instrumented rustc* itself; on bare-metal
// stable we reinterpret as "instrumented target driver emits profraw".
// Thorough: boot record + IRQ record + fault record — three probe shapes so the
// merged report must show three executed functions, not one.
// Reuse: minicov pattern vendored at ../bft-rust-coretests/minicov
// (Amanieu/minicov, Apache-2.0/MIT) + rustc instrument-coverage docs.
//
// Steps (also wired in suite-runner/tests/coverage_run_rustc.rs):
//   1. cargo build --target armv8r-none-eabihf --features coverage --bin firmware
//   2. LLVM_PROFILE_FILE=/tmp/fw.profraw qemu-system-arm -M versatileab \
//        -cpu cortex-r52 -semihosting -nographic -audio none \
//        -kernel target/armv8r-none-eabihf/debug/firmware
//   3. test -s /tmp/fw.profraw
//
// This file is the minimal instrumented unit linked into that firmware.

#![cfg_attr(not(test), no_std)]

/// Called once at boot when `coverage` feature is on; must appear in covmap.
pub fn record_boot(reason: u32) -> u32 {
    if reason == 0 {
        0xB007
    } else {
        reason ^ 0xB007
    }
}

/// Called on each IRQ entry; exercises loop-adjacent counter.
pub fn record_irq(n: u32) -> u32 {
    let mut acc = 0u32;
    for i in 0..n {
        acc = acc.wrapping_add(i ^ 0x100);
    }
    acc
}

/// Called on fault; exercises early-return counters.
pub fn record_fault(code: u32) -> u32 {
    if code == 0 {
        return 0;
    }
    if code == 0xFFFF_FFFF {
        return u32::MAX;
    }
    code.count_ones()
}

#[cfg(test)]
mod hosttest {
    #[test]
    fn emits() {
        assert_eq!(super::record_boot(0), 0xB007);
        assert_eq!(super::record_boot(1), 1 ^ 0xB007);
        assert_eq!(super::record_irq(4), (0 ^ 0x100) + (1 ^ 0x100) + (2 ^ 0x100) + (3 ^ 0x100));
        assert_eq!(super::record_fault(0), 0);
        assert_eq!(super::record_fault(0xFFFF_FFFF), u32::MAX);
        assert_eq!(super::record_fault(0b1011), 3);
    }
}

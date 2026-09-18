// Coverage-run-rustc test 1/2: firmware emits .profraw via semihosting.
// Upstream coverage-run-rustc runs an *instrumented rustc* itself; on bare-metal
// stable we reinterpret as "instrumented target driver emits profraw".
// Thorough: boot/IRQ/fault records plus timeout backoff and fault classifier —
// five probe shapes so the merged report must show five executed functions.
// Reuse: minicov pattern vendored at ../bft-rust-coretests/minicov
// (Amanieu/minicov, Apache-2.0/MIT) + rustc instrument-coverage docs.
//
// Steps (also wired in suite-runner/tests/coverage_run_rustc.rs):
//   1. cargo build --target armv8r-none-eabihf --bin firmware
//      (builds with -C instrument-coverage + minicov InstrProfiling)
//   2. LLVM_PROFILE_FILE=/tmp/fw.profraw qemu-system-arm -M versatileab \
//        -cpu cortex-r52 -semihosting -nographic -audio none \
//        -kernel target/armv8r-none-eabihf/debug/firmware
//   3. test -s /tmp/fw.profraw
//
// This file is the minimal instrumented unit linked into that firmware.
//
// RUN (host-side): rustc --edition=2021 --test %s -o /tmp/cov_probes && /tmp/cov_probes

#![cfg_attr(not(test), no_std)]

/// Called once at boot; must appear in covmap.
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

/// Timeout backoff accumulator; exercises counted loop.
pub fn record_timeout(ticks: u32) -> u32 {
    let mut backoff = 0u32;
    let mut t = ticks;
    while t > 0 {
        backoff = backoff.wrapping_add(t & 0xF);
        t >>= 1;
    }
    backoff
}

/// Fault classifier; exercises range-match counters.
pub fn classify_fault(code: u32) -> u8 {
    match code {
        0 => 0,
        1..=15 => 1,
        16..=255 => 2,
        _ => 3,
    }
}

#[cfg(test)]
mod hosttest {
    use super::{classify_fault, record_boot, record_fault, record_irq, record_timeout};

    #[test]
    fn emits() {
        assert_eq!(record_boot(0), 0xB007);
        assert_eq!(record_boot(1), 1 ^ 0xB007);
        assert_eq!(record_irq(4), (0 ^ 0x100) + (1 ^ 0x100) + (2 ^ 0x100) + (3 ^ 0x100));
        assert_eq!(record_fault(0), 0);
        assert_eq!(record_fault(0xFFFF_FFFF), u32::MAX);
        assert_eq!(record_fault(0b1011), 3);
    }

    #[test]
    fn timeouts_and_faults() {
        assert_eq!(record_timeout(0), 0);
        assert_eq!(record_timeout(1), 1);
        assert_eq!(record_timeout(0b1111), 15 + 7 + 3 + 1);
        assert_eq!(classify_fault(0), 0);
        assert_eq!(classify_fault(7), 1);
        assert_eq!(classify_fault(100), 2);
        assert_eq!(classify_fault(1000), 3);
    }
}

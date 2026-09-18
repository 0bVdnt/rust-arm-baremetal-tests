// ui-fulldeps test 2/2: atomics + critical-section counter (generic R/A).
// Reuse: rust-embedded semihosting docs + aarch32 examples pattern.
// Thorough: fetch_add/fetch_or/compare_exchange/fence across three statics —
// the exact primitives firmware tick/GIC/ring code needs on both A and R.
// Needs `semihosting` + atomics (beyond core) -> qualifies as fulldeps.
// Builds for AArch32; AArch64 variant uses HLT semihosting in firmware-aarch64.
// RUN: cargo build --target armv8r-none-eabihf --bin firmware (covers this unit)

#![cfg_attr(not(test), no_std)]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

static TICK: AtomicU32 = AtomicU32::new(0);
static FLAGS: AtomicU32 = AtomicU32::new(0);
static READY: AtomicBool = AtomicBool::new(false);

pub fn tick_once() -> u32 {
    TICK.fetch_add(1, Ordering::SeqCst) + 1
}

pub fn set_flag(bit: u32) {
    FLAGS.fetch_or(1 << (bit & 31), Ordering::SeqCst);
}

pub fn take_flags() -> u32 {
    FLAGS.swap(0, Ordering::SeqCst)
}

pub fn cas_tick(expect: u32, next: u32) -> bool {
    TICK.compare_exchange(expect, next, Ordering::SeqCst, Ordering::Relaxed).is_ok()
}

pub fn publish() {
    READY.store(true, Ordering::Release);
}

pub fn is_ready() -> bool {
    READY.load(Ordering::Acquire)
}

#[cfg(test)]
mod hosttest {
    use super::*;
    use core::sync::atomic::Ordering;

    #[test]
    fn ticks_and_flags() {
        TICK.store(0, Ordering::SeqCst);
        FLAGS.store(0, Ordering::SeqCst);
        assert_eq!(tick_once(), 1);
        assert!(cas_tick(1, 10));
        assert!(!cas_tick(1, 20));
        set_flag(0);
        set_flag(3);
        assert_eq!(take_flags(), 0b1001);
        assert_eq!(take_flags(), 0);
    }

    #[test]
    fn publish_ready() {
        READY.store(false, Ordering::SeqCst);
        assert!(!is_ready());
        publish();
        assert!(is_ready());
    }
}

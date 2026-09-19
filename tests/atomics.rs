//! Atomic checks (mirrors tests/assembly/atomics.rs).
//! R-profile uses LDREX/STREX, A-profile LSE/LL-SC; semantics are identical.
//! Each primitive gets its own testcase so a failure names the operation.

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering};
use helper::testcase;

static ADD_CELL: AtomicU32 = AtomicU32::new(41);
static OR_CELL: AtomicU32 = AtomicU32::new(0);
static CAS_CELL: AtomicU32 = AtomicU32::new(7);
static FLAG: AtomicBool = AtomicBool::new(false);
static SUB_CELL: AtomicU32 = AtomicU32::new(100);
static SWAP_CELL: AtomicU32 = AtomicU32::new(1);
static LOGIC_CELL: AtomicU32 = AtomicU32::new(0xFF);
static MINMAX_CELL: AtomicU32 = AtomicU32::new(50);
static BYTE_CELL: AtomicU8 = AtomicU8::new(0x0F);

#[testcase]
fn atomic_add() {
    ADD_CELL.store(41, Ordering::SeqCst);
    ADD_CELL.fetch_add(1, Ordering::SeqCst);
    assert_eq!(ADD_CELL.load(Ordering::SeqCst), 42);
}

#[testcase]
fn atomic_or() {
    OR_CELL.store(0x0F, Ordering::SeqCst);
    OR_CELL.fetch_or(0xF0, Ordering::SeqCst);
    assert_eq!(OR_CELL.load(Ordering::SeqCst), 0xFF);
}

#[testcase]
fn atomic_cas() {
    CAS_CELL.store(7, Ordering::SeqCst);
    assert!(CAS_CELL.compare_exchange(7, 100, Ordering::SeqCst, Ordering::Relaxed).is_ok());
    assert_eq!(CAS_CELL.load(Ordering::SeqCst), 100);
    assert!(CAS_CELL.compare_exchange(7, 200, Ordering::SeqCst, Ordering::Relaxed).is_err());
}

#[testcase]
fn atomic_fence() {
    FLAG.store(true, Ordering::Release);
    assert!(FLAG.load(Ordering::Acquire));
}

#[testcase]
fn atomic_sub() {
    SUB_CELL.store(100, Ordering::SeqCst);
    assert_eq!(SUB_CELL.fetch_sub(30, Ordering::SeqCst), 100);
    assert_eq!(SUB_CELL.load(Ordering::SeqCst), 70);
}

#[testcase]
fn atomic_swap() {
    SWAP_CELL.store(1, Ordering::SeqCst);
    assert_eq!(SWAP_CELL.swap(2, Ordering::SeqCst), 1);
    assert_eq!(SWAP_CELL.load(Ordering::Relaxed), 2);
}

#[testcase]
fn atomic_logic() {
    LOGIC_CELL.store(0xFF, Ordering::SeqCst);
    LOGIC_CELL.fetch_and(0x0F, Ordering::SeqCst);
    assert_eq!(LOGIC_CELL.load(Ordering::SeqCst), 0x0F);
    LOGIC_CELL.fetch_xor(0xFF, Ordering::SeqCst);
    assert_eq!(LOGIC_CELL.load(Ordering::SeqCst), 0xF0);
}

#[testcase]
fn atomic_min_max() {
    MINMAX_CELL.store(50, Ordering::SeqCst);
    MINMAX_CELL.fetch_max(100, Ordering::SeqCst);
    assert_eq!(MINMAX_CELL.load(Ordering::SeqCst), 100);
    MINMAX_CELL.fetch_min(25, Ordering::SeqCst);
    assert_eq!(MINMAX_CELL.load(Ordering::SeqCst), 25);
    // No-ops when already beyond.
    MINMAX_CELL.fetch_max(10, Ordering::SeqCst);
    assert_eq!(MINMAX_CELL.load(Ordering::SeqCst), 25);
}

#[testcase]
fn atomic_byte() {
    BYTE_CELL.store(0x0F, Ordering::SeqCst);
    BYTE_CELL.fetch_or(0xF0, Ordering::SeqCst);
    assert_eq!(BYTE_CELL.load(Ordering::SeqCst), 0xFF);
}

#[testcase]
fn atomic_weak_cas_loop() {
    static W: AtomicU32 = AtomicU32::new(0);
    W.store(0, Ordering::SeqCst);
    // compare_exchange_weak may fail spuriously: retry bounded times.
    let mut cur = W.load(Ordering::Relaxed);
    let mut ok = false;
    for _ in 0..100 {
        match W.compare_exchange_weak(cur, cur + 1, Ordering::SeqCst, Ordering::Relaxed) {
            Ok(_) => {
                ok = true;
                break;
            }
            Err(actual) => cur = actual,
        }
    }
    assert!(ok);
    assert!(W.load(Ordering::SeqCst) >= 1);
}

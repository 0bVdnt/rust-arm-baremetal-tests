//! Atomic checks (mirrors tests/assembly/atomics.rs).
//! R-profile uses LDREX/STREX, A-profile LSE/LL-SC; semantics are identical.
//! Each primitive gets its own testcase so a failure names the operation.

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use helper::testcase;

static ADD_CELL: AtomicU32 = AtomicU32::new(41);
static OR_CELL: AtomicU32 = AtomicU32::new(0);
static CAS_CELL: AtomicU32 = AtomicU32::new(7);
static FLAG: AtomicBool = AtomicBool::new(false);

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

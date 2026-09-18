//! Coverage-probe logic checks (mirrors tests/coverage-run/*.rs).
//! The same branch/loop/match shapes the coverage suites instrument, asserted
//! here so QEMU output proves the probes execute (minicov writes profraw).

use helper::testcase;

fn branch(x: u32) -> u32 {
    if x > 10 { x - 10 } else { x + 10 }
}

#[testcase]
fn cov_branch() {
    assert_eq!(branch(20), 10);
    assert_eq!(branch(5), 15);
    assert_eq!(branch(10), 20);
}

#[testcase]
fn cov_loop() {
    let sum: u32 = (0..10u32).sum();
    assert_eq!(sum, 45);
}

fn poll(mut reg: u32) -> u32 {
    let mut c = 0;
    for _ in 0..4 {
        c += match reg & 0b11 {
            0 => 1,
            1 => 2,
            _ => 4,
        };
        reg >>= 2;
    }
    c
}

#[testcase]
fn cov_match() {
    assert_eq!(poll(0b11_10_01_00), 1 + 2 + 4 + 4);
}

#[testcase]
fn cov_saturating() {
    assert_eq!(1u32.saturating_add(2), 3);
    assert_eq!(u32::MAX.saturating_add(1), u32::MAX);
}

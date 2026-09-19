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

#[testcase]
fn cov_if_let() {
    let some: Option<u32> = Some(7);
    let v = if let Some(x) = some { x * 2 } else { 0 };
    assert_eq!(v, 14);
    let none: Option<u32> = None;
    let w = if let Some(x) = none { x } else { 99 };
    assert_eq!(w, 99);
}

#[testcase]
fn cov_match_guard() {
    fn classify(x: u32) -> u8 {
        match x {
            n if n < 10 => 0,
            n if n < 100 => 1,
            _ => 2,
        }
    }
    assert_eq!(classify(5), 0);
    assert_eq!(classify(50), 1);
    assert_eq!(classify(500), 2);
}

#[testcase]
fn cov_nested_loops() {
    let mut acc = 0u32;
    for i in 0..4 {
        for j in 0..4 {
            acc += (i == j) as u32;
        }
    }
    assert_eq!(acc, 4);
}

#[testcase]
fn cov_while_count() {
    let mut n = 0u32;
    let mut x = 100u32;
    while x > 1 {
        x /= 2;
        n += 1;
    }
    assert_eq!((n, x), (6, 1));
}

#[testcase]
fn cov_early_multi() {
    fn first_hit(vals: &[u32], target: u32) -> Option<usize> {
        for (i, v) in vals.iter().enumerate() {
            if *v == target {
                return Some(i);
            }
        }
        None
    }
    assert_eq!(first_hit(&[5, 6, 7], 6), Some(1));
    assert_eq!(first_hit(&[5, 6, 7], 9), None);
    assert_eq!(first_hit(&[], 1), None);
}

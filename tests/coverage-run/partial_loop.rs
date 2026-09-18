// Coverage-run test 2/2: partial coverage — covered loop vs dead ISR/code.
// Reuse: rust tests/coverage/ partial-coverage tests.
// Bare-metal: `never_called` + `half_covered` else-arm must appear as
// uncovered in `llvm-cov show`. Host check mirrors firmware logic.
//
// RUN: rustc --edition=2021 --test %s -o /tmp/cov_partial && /tmp/cov_partial
// QEMU: same LLVM_PROFILE_FILE + llvm-profdata/llvm-cov flow as branch_exec.rs
// Expected: `covered` 100%, `never_called` 0%, `half_covered(false)` misses one arm.

#![cfg_attr(not(test), no_std)]

pub fn covered(x: u32) -> u32 {
    let mut acc = 0;
    for i in 0..x {
        acc += i;
    }
    acc
}

pub fn half_covered(flag: bool, x: u32) -> u32 {
    if flag {
        x + 1
    } else {
        x + 2
    }
}

#[allow(dead_code)]
pub fn never_called() -> u32 {
    0xDEAD
}

#[allow(dead_code)]
pub fn never_loop(n: u32) -> u32 {
    let mut s = 0;
    for i in 0..n {
        s += i * 3;
    }
    s
}

#[cfg(test)]
mod hosttest {
    #[test]
    fn partial() {
        assert_eq!(super::covered(0), 0);
        assert_eq!(super::covered(4), 6);
        // only true arm executed:
        assert_eq!(super::half_covered(true, 10), 11);
        // never_called / never_loop intentionally not executed
    }
}

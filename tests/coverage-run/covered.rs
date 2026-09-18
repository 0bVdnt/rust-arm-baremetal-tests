// Coverage-run test 1/2: full branch/loop/edge/saturating/state coverage.
// Reuse: rust tests/coverage/ run tests. Bare-metal adaptation: the SAME logic
// runs in firmware under QEMU with minicov profraw via semihosting FS, then:
//
//   LLVM_PROFILE_FILE=branch.profraw qemu-system-arm -M versatileab -cpu cortex-r52 \
//     -nographic -semihosting -audio none -kernel firmware
//   llvm-profdata merge -sparse branch.profraw -o branch.profdata
//   llvm-cov show firmware -instr-profile=branch.profdata --show-regions
// Expected: every region below executed at least once.
//
// RUN (host-side logic check): rustc --edition=2021 --test %s -o /tmp/cov_covered && /tmp/cov_covered

#![cfg_attr(not(test), no_std)]

pub fn branch(x: u32) -> u32 {
    if x > 10 {
        x - 10
    } else {
        x + 10
    }
}

pub fn edges(x: u32) -> u32 {
    if x == 0 {
        0xFFFF
    } else if x == 1 {
        1
    } else {
        // wrapping: bare-metal release and debug-host must agree.
        x.wrapping_mul(2)
    }
}

pub fn sum_to(n: u32) -> u32 {
    let mut acc = 0;
    for i in 0..n {
        acc += i;
    }
    acc
}

pub fn sat_add(a: u32, b: u32) -> u32 {
    a.saturating_add(b)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    Run,
    Fault,
}

pub fn step(s: State, go: bool, err: bool) -> State {
    match (s, go, err) {
        (State::Idle, true, false) => State::Run,
        (State::Run, _, true) => State::Fault,
        (State::Fault, _, false) => State::Idle,
        _ => s,
    }
}

#[cfg(test)]
mod hosttest {
    use super::{branch, edges, sat_add, step, sum_to, State};

    #[test]
    fn branches_and_edges() {
        assert_eq!(branch(20), 10);
        assert_eq!(branch(5), 15);
        // boundary: exactly 10 takes else arm
        assert_eq!(branch(10), 20);
        assert_eq!(branch(11), 1);
        assert_eq!(edges(0), 0xFFFF);
        assert_eq!(edges(1), 1);
        assert_eq!(edges(7), 14);
        assert_eq!(edges(u32::MAX), u32::MAX.wrapping_mul(2));
    }

    #[test]
    fn loops_and_saturating() {
        assert_eq!(sum_to(0), 0);
        assert_eq!(sum_to(1), 0);
        assert_eq!(sum_to(4), 6);
        assert_eq!(sum_to(10), 45);
        assert_eq!(sat_add(1, 2), 3);
        assert_eq!(sat_add(u32::MAX, 1), u32::MAX);
        assert_eq!(sat_add(0, 0), 0);
    }

    #[test]
    fn machine() {
        assert_eq!(step(State::Idle, true, false), State::Run);
        assert_eq!(step(State::Idle, false, false), State::Idle);
        assert_eq!(step(State::Run, false, true), State::Fault);
        assert_eq!(step(State::Run, true, false), State::Run);
        assert_eq!(step(State::Fault, false, true), State::Fault);
        assert_eq!(step(State::Fault, false, false), State::Idle);
    }
}

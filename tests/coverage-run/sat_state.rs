// Coverage-run test 3/3: saturating arithmetic + state machine full cover.
// Reuse: rust tests/coverage/ run tests.
// Bare-metal: same LLVM_PROFILE_FILE + llvm-profdata/llvm-cov flow as siblings.
// Expected: every arm below executed at least once.
//
// RUN (host-side logic check): rustc --edition=2021 --test %s -o /tmp/cov_sm && /tmp/cov_sm

#![cfg_attr(not(test), no_std)]

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
    use super::{sat_add, step, State};

    #[test]
    fn saturating() {
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

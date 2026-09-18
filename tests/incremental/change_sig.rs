// Incremental test 2/2: signature change forces dependent rebuild.
// Reuse: rust tests/incremental/ dep-graph pattern.
// Thorough: provider + two dependents + independent module — changing `sig`
// must recheck dependents but leave the independent module's cache valid.
// Steps:
//   1. Build %s with current sig, then widen `sig` u16->u32 and rebuild
//      with same -Cincremental dir; dependents `caller`/`caller2` must be
//      rechecked, `independent` untouched.
// Expected: both builds succeed; second reflects new signature.
// (Nightly extra: #[rustc_if_this_changed]/#[rustc_then_this_would_need] in ui.)

#![crate_type = "lib"]
#![cfg_attr(not(test), no_std)]

pub fn sig(x: u16) -> u16 {
    x ^ 0x1234
}

pub fn caller(x: u16) -> u16 {
    sig(x).wrapping_add(1)
}

pub fn caller2(x: u16) -> u16 {
    sig(x).rotate_left(4)
}

pub fn independent(x: u32) -> u32 {
    x.wrapping_mul(7)
}

#[cfg(test)]
mod hosttest {
    #[test]
    fn deps_agree() {
        assert_eq!(super::caller(0), super::sig(0).wrapping_add(1));
        assert_eq!(super::caller2(1), super::sig(1).rotate_left(4));
        assert_eq!(super::independent(6), 42);
    }
}

// Coverage-run test 1/2: full branch + loop + edge coverage -> 100% regions.
// Reuse: rust tests/coverage/ run tests. Bare-metal adaptation: the SAME logic
// runs in firmware under QEMU with minicov profraw via semihosting FS, then:
//
//   LLVM_PROFILE_FILE=branch.profraw qemu-system-arm -M versatileab -cpu cortex-r52 \
//     -nographic -semihosting -audio none -kernel firmware
//   llvm-profdata merge -sparse branch.profraw -o branch.profdata
//   llvm-cov show firmware -instr-profile=branch.profdata --show-regions
// Expected: all regions executed for branch/edges/loop below.
//
// RUN (host-side logic check): rustc --edition=2021 --test %s -o /tmp/cov_branch && /tmp/cov_branch

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

#[cfg(test)]
mod hosttest {
    #[test]
    fn both_branches() {
        assert_eq!(super::branch(20), 10);
        assert_eq!(super::branch(5), 15);
        // boundary: exactly 10 takes else arm
        assert_eq!(super::branch(10), 20);
        assert_eq!(super::branch(11), 1);
    }

    #[test]
    fn all_edges() {
        assert_eq!(super::edges(0), 0xFFFF);
        assert_eq!(super::edges(1), 1);
        assert_eq!(super::edges(7), 14);
        assert_eq!(super::edges(u32::MAX), u32::MAX.wrapping_mul(2));
    }

    #[test]
    fn loop_counts() {
        assert_eq!(super::sum_to(0), 0);
        assert_eq!(super::sum_to(1), 0);
        assert_eq!(super::sum_to(4), 6);
        assert_eq!(super::sum_to(10), 45);
    }
}

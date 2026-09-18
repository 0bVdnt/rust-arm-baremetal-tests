// Coverage-run-rustc test 3/3: error-path probes (fault + timeout records).
// Upstream coverage-run-rustc runs instrumented rustc; reinterpreted for
// bare-metal as instrumented-driver probes (see profraw_emit.rs).
// Thorough: timeout counter (loop) + fault classifier (match) — the two
// shapes whose counters most often go missing after LTO/thin archives.
//
// Steps: same LLVM_PROFILE_FILE + llvm-profdata merge + llvm-cov show flow;
// expected: both functions listed as executed.
//
// RUN (host-side): rustc --edition=2021 --test %s -o /tmp/cov_err && /tmp/cov_err

#![cfg_attr(not(test), no_std)]

pub fn record_timeout(ticks: u32) -> u32 {
    let mut backoff = 0u32;
    let mut t = ticks;
    while t > 0 {
        backoff = backoff.wrapping_add(t & 0xF);
        t >>= 1;
    }
    backoff
}

pub fn classify_fault(code: u32) -> u8 {
    match code {
        0 => 0,
        1..=15 => 1,
        16..=255 => 2,
        _ => 3,
    }
}

#[cfg(test)]
mod hosttest {
    #[test]
    fn timeouts() {
        assert_eq!(super::record_timeout(0), 0);
        assert_eq!(super::record_timeout(1), 1);
        assert_eq!(super::record_timeout(0b1111), 15 + 7 + 3 + 1);
    }

    #[test]
    fn faults() {
        assert_eq!(super::classify_fault(0), 0);
        assert_eq!(super::classify_fault(7), 1);
        assert_eq!(super::classify_fault(100), 2);
        assert_eq!(super::classify_fault(1000), 3);
    }
}

// Coverage-run-rustc test 2/2: host merges target profraw.
// Upstream: `llvm-profdata merge` + `llvm-cov show` over instrumented-rustc run.
// Thorough: two probes (xor-fold + saturating path) so merge+report must list
// two functions with distinct region counts.
// Reinterpreted pasos for bare-metal (host-side, needs only LLVM tools):
//
//   llvm-profdata merge -sparse /tmp/fw.profraw -o /tmp/fw.profdata
//   llvm-cov show target/armv8r-none-eabihf/debug/firmware \
//     -instr-profile=/tmp/fw.profdata --show-line-counts
// Expected: both probes show executed regions; exit 0.
//
// Synthetic self-coverage when no QEMU profraw is present:
//   rustc --edition=2021 -C instrument-coverage --test %s -o /tmp/merge_self
//   LLVM_PROFILE_FILE=/tmp/self.profraw /tmp/merge_self
//   llvm-profdata merge -sparse /tmp/self.profraw -o /tmp/self.profdata
//   llvm-cov report /tmp/merge_self -instr-profile=/tmp/self.profdata | grep merge_probe

#![cfg_attr(not(test), no_std)]

pub fn merge_probe(x: u32) -> u32 {
    x ^ 0xC0FFEE
}

pub fn merge_saturating(x: u32) -> u32 {
    if x > 1000 {
        1000
    } else {
        x * 2
    }
}

#[cfg(test)]
mod hosttest {
    #[test]
    fn probe() {
        assert_eq!(super::merge_probe(0), 0xC0FFEE);
        assert_eq!(super::merge_probe(0xC0FFEE), 0);
    }

    #[test]
    fn saturating() {
        assert_eq!(super::merge_saturating(10), 20);
        assert_eq!(super::merge_saturating(5000), 1000);
        assert_eq!(super::merge_saturating(1000), 2000);
    }
}

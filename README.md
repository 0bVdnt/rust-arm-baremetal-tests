# bft-rust-arm-baremetal-tests

Generic bare-metal ARM test suite for the latest Rust stable (**1.98.1**),
covering **3 tests × 12 compiletest suites plus one softfloat-ABI test** (37 tests):

`assembly`, `codegen`, `codegen-units`, `coverage-map`, `coverage-run`,
`coverage-run-rustc`, `debuginfo`, `incremental`, `mir-opt`, `pretty`,
`ui`, `ui-fulldeps`.

Plus a QEMU semihosting harness (`cargo test`, 34 testcases) built exactly
like the sibling `bft-rust-coretests` / `bft-rust-alloctests` suites.

Targets (all Tier 2 bare-metal, `rust-lld` self-contained — no cross-gcc needed):

* `aarch64-unknown-none` (Armv8-A AArch64 hardfloat, QEMU `virt`)
* `aarch64-unknown-none-softfloat` (same board, soft-float ABI)
* `armv8r-none-eabihf` (Cortex-R52, QEMU `versatileab` + `mps3-an536` ref)
* `armv7a-none-eabihf` / `armv7r-none-eabihf` (QEMU `versatileab`)
* `armv7a-none-eabi` (softfloat ARMv7-A, QEMU `versatileab`)

Every `cargo test --target <triple>` behaves identically on all six
targets: `Starting test execution` → `Executing 34 tests` →
`test result: ok. 34 passed; 0 failed` → exit 0.

## Reuse-first, generic

* Compiler-suite tests vendor/adapt `rust-lang/rust@1.98.1` `tests/*`
  patterns (see `SOURCES.md`); `// CHECK:` + `//@` directives preserved.
* Harness reuses sibling `../bft-rust-alloctests/aarch32/*`
  (`aarch32-cpu`, `aarch32-rt`, `arm-targets`), `semihosting`,
  `embedded-alloc`; AArch64 startup follows `rust-embedded/aarch64-cpu`
  and `google/aarch64-rt` patterns, and its `critical-section` impl
  mirrors `aarch32-cpu`'s single-core one (DAIF masking).
* Generic gating via `arm-targets` cfgs (`arm_isa`, `arm_architecture`,
  `arm_profile`, `arm_abi`) — not per-triple `#[cfg]`.
* Optional system-level: `ARM-software/sysarch-acs` (BSA/SBSA/MemTest).

## Layout (same harness shape as the sibling suites)

```text
rust-toolchain.toml  pins 1.98.1 + 6 targets
.cargo/config.toml   default target + QEMU runners per target
lib.rs               empty no_std stub ([lib], like siblings)
build.rs             arm_targets::process() + memory.x staging (like siblings)
memory.x             VersatilePB map; memory-mps3.x for MPS3-AN536
memory-aarch64.ld    QEMU virt map for aarch64-unknown-none
helper = ../bft-rust-alloctests/helper  (shared #[testcase] proc-macro,
                     not vendored — same crate the siblings use)
tests/lib.rs         harness root: custom_test_frameworks + dual-arch entry
                     (aarch32-rt on AArch32, local _start on AArch64) +
                     shared heap/boot/custom_runner (like siblings),
                     34 testcases via:
tests/alu.rs, float.rs, atomics.rs, layout.rs, alloc_checks.rs,
      coverage_probes.rs, incr_chain.rs
tests/<suite>/<test>.rs      37 compiletest-style tests (data files;
                             assembly has a 4th softfloat-only test)
suite-runner/          cargo-native driver: 12 suite test-targets +
                       harness matrix, zero dependencies. Host-only tool
                       (own workspace + host default target); drives rustc
                       directly and checks output with a built-in FileCheck
                       subset (tests/common/mod.rs, parity-tested against
                       the real FileCheck in engine_check).
```

## Quick start

```sh
rustup show            # expect stable 1.98.1

# QEMU semihosting harness, like the sibling suites.
# custom_test_frameworks is auto-enabled via .cargo/config.toml [env];
# bare `cargo test` defaults to armv8r-none-eabihf (same file).
cargo test --target armv8r-none-eabihf
cargo test --target aarch64-unknown-none
cargo test --target aarch64-unknown-none-softfloat
cargo test --target armv7a-none-eabihf
cargo test --target armv7a-none-eabi
cargo test --target armv7r-none-eabihf
# → "Starting test execution" / "Executing 34 tests" /
#   "test result: ok. 34 passed; 0 failed", exit 0, on every target.

# Whole harness matrix with summary table
# (extend via ALL_TARGETS in suite-runner/tests/common/mod.rs):
cd suite-runner && cargo test --test harness -- --nocapture && cd ..
# target                      result  tests
# aarch64-unknown-none        PASS    34/34
# ...

# Compiler suites — everything is cargo (run from suite-runner/):
cd suite-runner
cargo test                       # all 12 suites × all targets + parity
cargo test --test assembly       # one suite
cargo test --test ui type_mismatch  # one test
cd ..

# Single compiletest-style checks (no wrapper needed):
rustc --target armv8r-none-eabihf --emit=asm tests/assembly/u32_add.rs -o /tmp/u.asm
# (FileCheck binary optional — the engine in suite-runner/tests/common
#  implements the same CHECK subset and is parity-tested against it.)
```

## Stable caveats

`-Z` suites (`codegen-units`, `mir-opt`, `pretty`, parts of `coverage-map`)
need `RUSTC_BOOTSTRAP=1` on stable; the suite-runner sets it on every
rustc invocation itself. `ui-fulldeps`/`coverage-run-rustc` are
reinterpreted for `no_std` (see `SOURCES.md`); true `rustc_private` /
instrumented-`rustc` forms require a rustc source build.

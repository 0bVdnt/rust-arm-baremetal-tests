# Reuse provenance (open-source sources)

All tests are adaptations (bare-metal `--target`, `no_std` shims) of
open-source originals. The QEMU harness shape (`tests/lib.rs` with
`custom_test_frameworks` + `#[testcase]`, the shared sibling `helper`
proc-macro crate, `build.rs` with `arm_targets::process()`, `memory.x`,
semihosting panic/exit) mirrors the sibling `bft-rust-coretests` /
`bft-rust-alloctests` suites in this directory. No logic invented from scratch.

## rust-lang/rust@1.98.1 `tests/` (MIT/Apache-2.0)

Pinned tag `1.98.1` (`48a229ceaefd4985c50990b14116b6d856af0985`).
Browse: https://github.com/rust-lang/rust/tree/1.98.1/tests

| Suite dir in this repo | Upstream dir | Reused pattern |
|---|---|---|
| tests/assembly | tests/assembly-llvm (+ `aarch64/` subdir, e.g. `aarch64-arm-load-store.rs`, `aarch64-pointer-auth.rs`, `align_offset.rs`) | `//@ assembly-output: emit-asm` + `// CHECK:` + FileCheck |
| tests/codegen | tests/codegen-llvm (NEON/float-ABI IR tests) | `--emit=llvm-ir` + `// CHECK:` |
| tests/codegen-units | tests/codegen-units | `-Zprint-mono-items` + `// CHECK:` CGU lines |
| tests/coverage-map | tests/coverage | `-Cinstrument-coverage --emit=llvm-ir`, `__llvm_coverage_mapping` / `__profc_*` counters |
| tests/coverage-run | tests/coverage | run + `llvm-profdata merge` + `llvm-cov show` (host runs + QEMU firmware probes) |
| tests/coverage-run-rustc | tests/coverage + rustc-dev-guide “coverage-run-rustc runs instrumented rustc” | reinterpreted: coverage probes as instrumented-driver units; host merge flow |
| tests/debuginfo | tests/debuginfo | `-g` + gdb `break/print/ptype` scripts |
| tests/incremental | tests/incremental | `-Cincremental` two-build fingerprint reuse |
| tests/mir-opt | tests/mir-opt (e.g. `sroa.rs`, README EMIT_MIR) | `--emit=mir -Zmir-opt-level` + blessed dumps |
| tests/pretty | tests/pretty | `-Zunpretty=expanded/typed` |
| tests/ui | tests/ui (+ README, rustc-dev-guide ui chapter) | `//@ check-fail/build-pass` + blessed `.stderr`, `$DIR` normalization |
| tests/ui-fulldeps | tests/ui-fulldeps | reinterpreted: external-crate (`serde`/`embedded-alloc`) builds for ARM; true `rustc_private` tests need rustc source build |

Docs reused: https://rustc-dev-guide.rust-lang.org/tests/compiletest.html,
https://rustc-dev-guide.rust-lang.org/tests/ui.html,
https://doc.rust-lang.org/rustc/instrument-coverage.html,
https://doc.rust-lang.org/stable/nightly-rustc/compiletest/runtest/index.html

## rust-embedded / Arm ecosystem (MIT/Apache-2.0)

* `rust-embedded/aarch32` — https://github.com/rust-embedded/aarch32
  (`aarch32-cpu`, `aarch32-rt`, `arm-targets`, `examples/versatileab`,
  `examples/mps3-an536`, `tests.sh`). Vendored at
  `../bft-rust-alloctests/aarch32`; depended on via `path =`.
* `rust-embedded/aarch64-cpu` — https://github.com/rust-embedded/aarch64-cpu
  (`examples/armv8-r/`, `testing/`). Pattern source for AArch64 startup.
* `google/aarch64-rt` — https://github.com/google/aarch64-rt
  (`entry!`, `memory.ld` ORIGIN/LENGTH pattern → `memory-aarch64.ld`).
* `rust-embedded/qemu-exit`, `semihosting` crate docs,
  Arm `abi-aa` semihosting spec (HLT 0xF000 / SVC), Embedded Rust Book
  QEMU/semihosting chapters — exit-code + console patterns.
* `Amanieu/minicov` — vendored at `../bft-rust-coretests/minicov`
  (bare-metal `InstrProfiling` + `LLVM_PROFILE_FILE` via semihosting fs).
* `ARM-software/sysarch-acs` (Apache-2.0) — https://github.com/ARM-software/sysarch-acs
  (BSA/SBSA/MemTest, bare-metal PAL). Optional side-by-side system validation;
  not compiled by rustc.

## License compatibility

All reused sources are MIT OR Apache-2.0 (sysarch-acs: Apache-2.0),
compatible with this suite's `MIT OR Apache-2.0`.

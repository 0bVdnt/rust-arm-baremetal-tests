//! assembly suite: `--emit=asm` + FileCheck per ISA/ABI prefix.
#[path = "common/mod.rs"]
mod common;

use common::{assert_success, check_file, data_path, rustc, TempDir, ALL_TARGETS};

fn asm_case(file: &str, target: &str, prefixes: &[&str]) {
    let src = data_path(file);
    let tmp = TempDir::new("asm");
    let out = tmp.join("out.s");
    let s = |p: &std::path::Path| p.to_string_lossy().into_owned();
    let o = rustc(
        Some(target),
        &[
            "--emit=asm".to_string(),
            "--crate-type=lib".to_string(),
            s(&src),
            "-o".to_string(),
            s(&out),
            "-C".to_string(),
            "opt-level=2".to_string(),
        ],
    );
    assert_success(&format!("{file} compile for {target}"), &o);
    let text = std::fs::read_to_string(&out).unwrap();
    check_file(&src, &text, prefixes);
}

/// Prefixes for the integer file (same ISA split as the float hardfloat part).
fn int_prefixes(target: &str) -> Vec<&'static str> {
    if target.starts_with("aarch64") {
        vec!["CHECK", "A64"]
    } else if target.starts_with("armv8r") {
        vec!["CHECK", "A32", "A32R8"]
    } else {
        vec!["CHECK", "A32", "A32V7"]
    }
}

/// Prefixes for the float file: hardfloat triples check VFP/NEON lowering,
/// softfloat triples check aeabi/compiler-rt libcalls.
fn float_prefixes(target: &str) -> Vec<&'static str> {
    if target == "aarch64-unknown-none-softfloat" {
        vec!["CHECK", "SOFT64"]
    } else if target == "armv7a-none-eabi" {
        vec!["CHECK", "SOFT32"]
    } else {
        int_prefixes(target)
    }
}

#[test]
fn integer() {
    for t in ALL_TARGETS {
        asm_case("assembly/integer.rs", t, &int_prefixes(t));
    }
}

#[test]
fn float() {
    for t in ALL_TARGETS {
        asm_case("assembly/float.rs", t, &float_prefixes(t));
    }
}

//! assembly suite: `--emit=asm` + FileCheck per ISA prefix.
#[path = "common/mod.rs"]
mod common;

use common::{
    assert_success, check_file, data_path, prefixes_for, rustc, TempDir, ALL_TARGETS, HF_TARGETS,
    SOFT_TARGETS,
};

fn asm_case(file: &str, target: &str) {
    let src = data_path(file);
    let tmp = TempDir::new("asm");
    let out = tmp.join("out.s");
    let out_s = |p: &std::path::Path| p.to_string_lossy().into_owned();
    let o = rustc(
        Some(target),
        &[
            "--emit=asm".to_string(),
            "--crate-type=lib".to_string(),
            out_s(&src),
            "-o".to_string(),
            out_s(&out),
            "-C".to_string(),
            "opt-level=2".to_string(),
        ],
    );
    assert_success(&format!("{file} compile for {target}"), &o);
    let text = std::fs::read_to_string(&out).unwrap();
    check_file(&src, &text, &prefixes_for(target));
}

#[test]
fn u32_add() {
    for t in ALL_TARGETS {
        asm_case("assembly/u32_add.rs", t);
    }
}

#[test]
fn hardfloat_f32_mul() {
    // Hardfloat ABI only — softfloat triples use softfloat_abi.rs instead.
    for t in HF_TARGETS {
        asm_case("assembly/hardfloat_f32_mul.rs", t);
    }
}

#[test]
fn softfloat_abi() {
    for t in SOFT_TARGETS {
        let src = data_path("assembly/softfloat_abi.rs");
        let tmp = TempDir::new("asm");
        let out = tmp.join("out.s");
        let s = |p: &std::path::Path| p.to_string_lossy().into_owned();
        let o = rustc(
            Some(t),
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
        assert_success(&format!("assembly/softfloat_abi.rs compile for {t}"), &o);
        let text = std::fs::read_to_string(&out).unwrap();
        let prefixes: &[&str] = if t.starts_with("aarch64") {
            &["CHECK", "SOFT64"]
        } else {
            &["CHECK", "SOFT32"]
        };
        check_file(&src, &text, prefixes);
    }
}

#[test]
fn atomics() {
    for t in ALL_TARGETS {
        asm_case("assembly/atomics.rs", t);
    }
}

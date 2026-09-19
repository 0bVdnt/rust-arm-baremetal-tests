//! codegen suite: `--emit=llvm-ir` + FileCheck.
#[path = "common/mod.rs"]
mod common;

use common::{assert_success, check_file, data_path, prefixes_for, rustc, TempDir, ALL_TARGETS};

fn ir_case(file: &str, target: &str) {
    let src = data_path(file);
    let tmp = TempDir::new("ir");
    let out = tmp.join("out.ll");
    let s = |p: &std::path::Path| p.to_string_lossy().into_owned();
    let o = rustc(
        Some(target),
        &[
            "--emit=llvm-ir".to_string(),
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
    check_file(&src, &text, &prefixes_for(target));
}

#[test]
fn layout_and_flow() {
    for t in ALL_TARGETS {
        ir_case("codegen/layout_and_flow.rs", t);
    }
}

#[test]
fn neon_vector_add() {
    for t in ALL_TARGETS {
        // aarch64-softfloat never inlines the core::arch wrappers (bodies
        // stay calls into intrinsic shims), so per-op CHECKs would match
        // intrinsic bodies out of order. Every other triple inlines fully.
        if *t == "aarch64-unknown-none-softfloat" {
            continue;
        }
        ir_case("codegen/neon_vector_add.rs", t);
    }
}

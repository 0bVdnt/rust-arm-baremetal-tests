//! codegen suite: `--emit=llvm-ir` + FileCheck.
#[path = "common/mod.rs"]
mod common;

use common::{assert_success, check_file, data_path, prefixes_for, rustc, TempDir, ALL_TARGETS};

fn ir_case(file: &str, target: &str, extra: &[&str]) {
    let src = data_path(file);
    let tmp = TempDir::new("ir");
    let out = tmp.join("out.ll");
    let s = |p: &std::path::Path| p.to_string_lossy().into_owned();
    let mut args = vec![
        "--emit=llvm-ir".to_string(),
        "--crate-type=lib".to_string(),
        s(&src),
        "-o".to_string(),
        s(&out),
        "-C".to_string(),
        "opt-level=2".to_string(),
    ];
    for e in extra {
        args.push(e.to_string());
    }
    let o = rustc(Some(target), &args);
    assert_success(&format!("{file} compile for {target}"), &o);
    let text = std::fs::read_to_string(&out).unwrap();
    check_file(&src, &text, &prefixes_for(target));
}

#[test]
fn align_repr_c_struct() {
    for t in ALL_TARGETS {
        ir_case("codegen/align_repr_c_struct.rs", t, &[]);
    }
}

#[test]
fn neon_vector_add() {
    for t in ALL_TARGETS {
        ir_case("codegen/neon_vector_add.rs", t, &[]);
    }
}

#[test]
fn control_flow() {
    for t in ALL_TARGETS {
        ir_case("codegen/control_flow.rs", t, &[]);
    }
}

//! mir-opt suite: `--emit=mir` + MIR dump checks.
#[path = "common/mod.rs"]
mod common;

use common::{assert_success, check_file, data_path, rustc, TempDir};

const MIR_TARGETS: &[&str] = &["armv8r-none-eabihf", "aarch64-unknown-none"];

fn mir_case(file: &str, target: &str) {
    let src = data_path(file);
    let tmp = TempDir::new("mir");
    let out = tmp.join("out.mir");
    let s = |p: &std::path::Path| p.to_string_lossy().into_owned();
    let o = rustc(
        Some(target),
        &[
            "--emit=mir".to_string(),
            "-Zmir-opt-level=2".to_string(),
            "--crate-type=lib".to_string(),
            s(&src),
            "-o".to_string(),
            s(&out),
        ],
    );
    assert_success(&format!("{file} mir dump for {target}"), &o);
    let text = std::fs::read_to_string(&out).unwrap();
    check_file(&src, &text, &["CHECK"]);
}

#[test]
fn const_fold() {
    for t in MIR_TARGETS {
        mir_case("mir-opt/const_fold.rs", t);
    }
}

#[test]
fn inline_mmio() {
    for t in MIR_TARGETS {
        mir_case("mir-opt/inline_mmio.rs", t);
    }
}

#[test]
fn dead_branch() {
    for t in MIR_TARGETS {
        mir_case("mir-opt/dead_branch.rs", t);
    }
}

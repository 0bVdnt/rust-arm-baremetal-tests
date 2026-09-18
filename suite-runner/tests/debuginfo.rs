//! debuginfo suite: cross-compile with `-g`; the interactive GDB session
//! stays documented in each file header (needs a live QEMU gdbstub).
#[path = "common/mod.rs"]
mod common;

use common::{assert_success, data_path, rustc, TempDir, ALL_TARGETS};

fn dbg_case(file: &str, target: &str) {
    let src = data_path(file);
    let tmp = TempDir::new("dbg");
    let out = tmp.join("out.o");
    let s = |p: &std::path::Path| p.to_string_lossy().into_owned();
    let o = rustc(
        Some(target),
        &[
            "--crate-type=lib".to_string(),
            "-g".to_string(),
            "-C".to_string(),
            "opt-level=0".to_string(),
            s(&src),
            "-o".to_string(),
            s(&out),
        ],
    );
    assert_success(&format!("{file} debuginfo build for {target}"), &o);
    assert!(out.exists(), "object file missing for {file} on {target}");
}

#[test]
fn line_step() {
    for t in ALL_TARGETS {
        dbg_case("debuginfo/line_step.rs", t);
    }
}

#[test]
fn struct_dwarf() {
    for t in ALL_TARGETS {
        dbg_case("debuginfo/struct_dwarf.rs", t);
    }
}

#[test]
fn array_slice() {
    for t in ALL_TARGETS {
        dbg_case("debuginfo/array_slice.rs", t);
    }
}

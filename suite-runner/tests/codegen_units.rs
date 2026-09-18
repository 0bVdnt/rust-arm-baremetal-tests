//! codegen-units suite: `-Zprint-mono-items` + MONO_ITEM checks.
#[path = "common/mod.rs"]
mod common;

use common::{assert_success, check_file, data_path, rustc, TempDir, ALL_TARGETS};

fn mono_case(file: &str, target: &str, units: &str) {
    let src = data_path(file);
    let tmp = TempDir::new("cgu");
    let out = tmp.join("out.rmeta");
    let s = |p: &std::path::Path| p.to_string_lossy().into_owned();
    let o = rustc(
        Some(target),
        &[
            "--crate-type=lib".to_string(),
            s(&src),
            "-o".to_string(),
            s(&out),
            format!("-Ccodegen-units={units}"),
            "-Zprint-mono-items".to_string(),
        ],
    );
    assert_success(&format!("{file} compile for {target}"), &o);
    let text = String::from_utf8_lossy(&o.stdout).into_owned()
        + &String::from_utf8_lossy(&o.stderr);
    check_file(&src, &text, &["CHECK"]);
}

#[test]
fn cgu_split() {
    for t in ALL_TARGETS {
        mono_case("codegen-units/cgu_split.rs", t, "2");
    }
}

#[test]
fn generic_mono() {
    for t in ALL_TARGETS {
        mono_case("codegen-units/generic_mono.rs", t, "1");
    }
}

#[test]
fn drop_glue_const() {
    for t in ALL_TARGETS {
        mono_case("codegen-units/drop_glue_const.rs", t, "1");
    }
}

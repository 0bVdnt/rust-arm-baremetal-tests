//! ui-fulldeps suite: external-crate builds for every target plus host runs.
//! (True `rustc_private` tests need a rustc source build; reinterpreted for
//! `no_std` — see SOURCES.md.)
#[path = "common/mod.rs"]
mod common;

use common::{assert_success, data_path, run_binary, rustc, TempDir, ALL_TARGETS};

fn cross_builds(file: &str) {
    for t in ALL_TARGETS {
        let src = data_path(file);
        let tmp = TempDir::new("fulldeps");
        let out = tmp.join("out.rmeta");
        let s = |p: &std::path::Path| p.to_string_lossy().into_owned();
        let o = rustc(
            Some(t),
            &[
                "--crate-type=lib".to_string(),
                "--emit=metadata".to_string(),
                s(&src),
                "-o".to_string(),
                s(&out),
            ],
        );
        assert_success(&format!("{file} cross build for {t}"), &o);
    }
}

fn host_run(file: &str, bin: &str) {
    let src = data_path(file);
    let tmp = TempDir::new("fulldeps");
    let exe = tmp.join(bin);
    let s = |p: &std::path::Path| p.to_string_lossy().into_owned();
    let o = rustc(
        None,
        &[
            "--edition=2021".to_string(),
            "--test".to_string(),
            s(&src),
            "-o".to_string(),
            s(&exe),
        ],
    );
    assert_success(&format!("{file} host build"), &o);
    let r = run_binary(&exe);
    assert_success(&format!("{file} host run"), &r);
}

#[test]
fn collections() {
    cross_builds("ui-fulldeps/collections.rs");
    host_run("ui-fulldeps/collections.rs", "collections");
}

#[test]
fn semihost_tick() {
    cross_builds("ui-fulldeps/semihost_tick.rs");
    host_run("ui-fulldeps/semihost_tick.rs", "semihost_tick");
}

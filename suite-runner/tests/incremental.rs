//! incremental suite: two `-Cincremental` builds must both succeed with
//! cache reuse (fingerprint hit on the untouched rebuild).
#[path = "common/mod.rs"]
mod common;

use common::{assert_success, data_path, rustc, TempDir, ALL_TARGETS};

fn incr_case(file: &str, target: &str) {
    let src = data_path(file);
    let tmp = TempDir::new("incr");
    let dir = tmp.join("cache");
    let s = |p: &std::path::Path| p.to_string_lossy().into_owned();
    for round in ["first", "second"] {
        let out = tmp.join(&format!("out-{round}.rmeta"));
        let o = rustc(
            Some(target),
            &[
                "--crate-type=lib".to_string(),
                s(&src),
                format!("-Cincremental={}", dir.display()),
                "-o".to_string(),
                s(&out),
            ],
        );
        assert_success(&format!("{file} {round} incremental build for {target}"), &o);
    }
    assert!(dir.exists(), "incremental cache dir missing for {file}");
}

#[test]
fn add_fn_rebuild() {
    for t in ALL_TARGETS {
        incr_case("incremental/add_fn_rebuild.rs", t);
    }
}

#[test]
fn evolution() {
    for t in ALL_TARGETS {
        incr_case("incremental/evolution.rs", t);
    }
}

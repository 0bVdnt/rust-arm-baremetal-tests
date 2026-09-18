//! coverage-run-rustc suite: host `--test` runs of the instrumented-driver
//! probes (upstream runs an instrumented rustc; reinterpreted — see SOURCES).
#[path = "common/mod.rs"]
mod common;

use common::{assert_success, data_path, run_binary, rustc, TempDir};

fn host_run(file: &str, bin: &str) {
    let src = data_path(file);
    let tmp = TempDir::new("covrustc");
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
fn profraw_emit() {
    host_run("coverage-run-rustc/profraw_emit.rs", "cov_emit");
}

#[test]
fn host_merge() {
    host_run("coverage-run-rustc/host_merge.rs", "cov_merge");
}

#[test]
fn error_probes() {
    host_run("coverage-run-rustc/error_probes.rs", "cov_err");
}

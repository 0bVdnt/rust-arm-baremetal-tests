//! coverage-run suite: host `rustc --test` + execute, asserting the same
//! logic the QEMU firmware covers under minicov on-target.
#[path = "common/mod.rs"]
mod common;

use common::{assert_success, data_path, run_binary, rustc, TempDir};

fn host_run(file: &str, bin: &str) {
    let src = data_path(file);
    let tmp = TempDir::new("covrun");
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
fn branch_exec() {
    host_run("coverage-run/branch_exec.rs", "cov_branch");
}

#[test]
fn partial_loop() {
    host_run("coverage-run/partial_loop.rs", "cov_partial");
}

#[test]
fn sat_state() {
    host_run("coverage-run/sat_state.rs", "cov_sat");
}

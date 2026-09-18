//! coverage-map suite: host `-C instrument-coverage --emit=llvm-ir` +
//! mapping checks. (Bare-metal triples lack profiler_builtins; target-side
//! coverage goes through minicov — see SOURCES.md.)
#[path = "common/mod.rs"]
mod common;

use common::{assert_success, check_file, data_path, rustc, TempDir};

fn map_case(file: &str) {
    let src = data_path(file);
    let tmp = TempDir::new("covmap");
    let out = tmp.join("out.ll");
    let s = |p: &std::path::Path| p.to_string_lossy().into_owned();
    let o = rustc(
        None,
        &[
            "-C".to_string(),
            "instrument-coverage".to_string(),
            "--emit=llvm-ir".to_string(),
            "--crate-type=lib".to_string(),
            s(&src),
            "-o".to_string(),
            s(&out),
        ],
    );
    assert_success(&format!("{file} host instrument"), &o);
    let text = std::fs::read_to_string(&out).unwrap();
    check_file(&src, &text, &["CHECK"]);
}

#[test]
fn branch_map() {
    map_case("coverage-map/branch_map.rs");
}

#[test]
fn loop_match_map() {
    map_case("coverage-map/loop_match_map.rs");
}

#[test]
fn result_chain_map() {
    map_case("coverage-map/result_chain_map.rs");
}

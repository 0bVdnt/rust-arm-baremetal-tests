//! pretty suite: `-Zunpretty` snapshot checks.
#[path = "common/mod.rs"]
mod common;

use common::{assert_success, check_file, data_path, rustc};

fn pretty_case(file: &str, mode: &str) {
    let src = data_path(file);
    let s = |p: &std::path::Path| p.to_string_lossy().into_owned();
    // NOTE: no `-o` — `-Zunpretty` prints to stdout only when no output
    // file is given; with `-o` the dump goes to the file instead.
    let o = rustc(
        None,
        &[
            "--crate-type=lib".to_string(),
            format!("-Zunpretty={mode}"),
            s(&src),
        ],
    );
    assert_success(&format!("{file} unpretty={mode}"), &o);
    let text = String::from_utf8_lossy(&o.stdout).into_owned();
    assert!(!text.trim().is_empty(), "{file} unpretty={mode} printed nothing");
    check_file(&src, &text, &["CHECK"]);
}

#[test]
fn expand_vec() {
    pretty_case("pretty/expand_vec.rs", "expanded");
}

#[test]
fn typed() {
    pretty_case("pretty/typed.rs", "hir,typed");
}

#[test]
fn hir_tree() {
    pretty_case("pretty/hir_tree.rs", "hir-tree");
}

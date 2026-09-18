//! ui suite: blessed-diagnostic checks (check-fail + warning snapshots).
#[path = "common/mod.rs"]
mod common;

use common::{data_path, rustc, TempDir, ALL_TARGETS};

fn stderr_of(out: &std::process::Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// Normalize compiler output the way compiletest does for blessed files:
/// the test's own directory becomes `$DIR`.
fn normalize(output: &str, test_file: &std::path::Path) -> String {
    let dir = test_file.parent().unwrap().to_string_lossy().into_owned();
    output.replace(&dir as &str, "$DIR")
}

fn compile(file: &str, target: &str) -> (std::path::PathBuf, std::process::Output) {
    let src = data_path(file);
    let tmp = TempDir::new("ui");
    let out = tmp.join("out.rmeta");
    let s = |p: &std::path::Path| p.to_string_lossy().into_owned();
    let o = rustc(
        Some(target),
        &[
            "--crate-type=lib".to_string(),
            "--emit=metadata".to_string(),
            s(&src),
            "-o".to_string(),
            s(&out),
        ],
    );
    (src, o)
}

#[test]
fn compile_fail() {
    // check-fail: E0308 x2 + E0499 + E0506, byte-identical to blessed .stderr.
    for t in ALL_TARGETS {
        let (src, o) = compile("ui/compile_fail.rs", t);
        let err = stderr_of(&o);
        assert!(!o.status.success(), "compile_fail must fail on {t}");
        for code in ["E0308", "E0499", "E0506"] {
            assert!(err.contains(code), "{code} missing on {t}:\n{err}");
        }
        let blessed =
            std::fs::read_to_string(src.with_extension("stderr")).expect("blessed .stderr");
        assert_eq!(
            normalize(&err, &src),
            blessed,
            "stderr drift vs blessed snapshot on {t}"
        );
    }
}

#[test]
fn dead_code_warn() {
    // build-pass with exactly the four documented lints.
    for t in ALL_TARGETS {
        let (_, o) = compile("ui/dead_code_warn.rs", t);
        let err = stderr_of(&o);
        assert!(o.status.success(), "dead_code_warn must pass on {t}:\n{err}");
        for w in [
            "function `never_used` is never used",
            "unused variable",
            "unused import",
            "should have a snake case name",
        ] {
            assert!(err.contains(w), "missing warning {w:?} on {t}:\n{err}");
        }
    }
}

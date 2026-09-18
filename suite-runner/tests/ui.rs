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

#[test]
fn type_mismatch() {
    // check-fail: two E0308s, byte-identical to the blessed .stderr.
    for t in ALL_TARGETS {
        let src = data_path("ui/type_mismatch.rs");
        let tmp = TempDir::new("ui");
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
        let err = stderr_of(&o);
        assert!(!o.status.success(), "type_mismatch must fail on {t}");
        assert!(err.contains("mismatched types"), "E0308 missing on {t}:\n{err}");
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
        let src = data_path("ui/dead_code_warn.rs");
        let tmp = TempDir::new("ui");
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

#[test]
fn borrowck() {
    // check-fail: E0499 + E0506.
    for t in ALL_TARGETS {
        let src = data_path("ui/borrowck.rs");
        let tmp = TempDir::new("ui");
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
        let err = stderr_of(&o);
        assert!(!o.status.success(), "borrowck must fail on {t}");
        assert!(err.contains("E0499"), "E0499 missing on {t}:\n{err}");
        assert!(err.contains("E0506"), "E0506 missing on {t}:\n{err}");
    }
}

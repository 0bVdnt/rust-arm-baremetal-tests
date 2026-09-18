//! Parity: the pure-Rust mini-FileCheck must agree with the real FileCheck
//! binary on every data file that carries CHECK directives. Skipped (with a
//! note, still passing) where FileCheck is not installed — the engine is
//! then the only checker, which is exactly the no-external-tools goal.
#[path = "common/mod.rs"]
mod common;

use common::{check_file, data_path, prefixes_for, rustc, suite_dir, TempDir};
use std::process::Command;

/// Representative target + prefixes per data file — mirrors what the real
/// suite tests use (hardfloat files only make sense on hf triples, etc.).
fn cases_for(file: &str) -> Vec<(String, Vec<String>)> {
    if file == "assembly/softfloat_abi.rs" {
        return vec![
            (
                "aarch64-unknown-none-softfloat".to_string(),
                vec!["CHECK".to_string(), "SOFT64".to_string()],
            ),
            (
                "armv7a-none-eabi".to_string(),
                vec!["CHECK".to_string(), "SOFT32".to_string()],
            ),
        ];
    }
    // Canonical representatives: one AArch64 + one AArch32-hardfloat.
    // (The full per-target matrix lives in the suite tests themselves.)
    ["aarch64-unknown-none", "armv8r-none-eabihf"]
        .iter()
        .map(|t| (t.to_string(), prefixes_for(t).iter().map(|s| s.to_string()).collect()))
        .collect()
}

fn check_files() -> Vec<String> {
    let mut v = Vec::new();
    for suite in ["assembly", "codegen", "coverage-map"] {
        for entry in std::fs::read_dir(suite_dir(suite)).unwrap() {
            let p = entry.unwrap().path();
            if p.extension().map(|e| e == "rs").unwrap_or(false) {
                v.push(format!("{}/{}", suite, p.file_name().unwrap().to_string_lossy()));
            }
        }
    }
    v.sort();
    v
}

#[test]
fn engine_agrees_with_filecheck() {
    let has_filecheck = Command::new("FileCheck")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if !has_filecheck {
        println!("FileCheck binary missing; engine-only mode");
    }
    for file in check_files() {
        let src = data_path(&file);
        for (target, prefixes) in cases_for(&file) {
            // coverage-map is host-side; covered by its own suite test.
            let emit = if file.starts_with("assembly") {
                "asm"
            } else if file.starts_with("codegen") {
                "llvm-ir"
            } else {
                continue;
            };
            let tmp = TempDir::new("parity");
            let out = tmp.join("out.txt");
            let args = vec![
                format!("--emit={emit}"),
                "--crate-type=lib".to_string(),
                src.to_string_lossy().into_owned(),
                "-o".to_string(),
                out.to_string_lossy().into_owned(),
                // Same -C opt-level=2 as the real suite tests: without it,
                // inlinable helpers (e.g. f32::abs) go out-of-line and the
                // CHECKs legitimately do not match.
                "-C".to_string(),
                "opt-level=2".to_string(),
            ];
            let o = rustc(Some(&target), &args);
            assert!(o.status.success(), "{file} on {target} failed to compile");
            let text = std::fs::read_to_string(&out).unwrap();
            let prefs: Vec<&str> = prefixes.iter().map(|s| s.as_str()).collect();
            // Engine verdict (panics on mismatch).
            check_file(&src, &text, &prefs);
            // Real FileCheck verdict must agree.
            if has_filecheck {
                let joined = prefs.join(",");
                let fc = Command::new("FileCheck")
                    .arg(&src)
                    .arg("--input-file")
                    .arg(&out)
                    .arg("--check-prefixes")
                    .arg(&joined)
                    .arg("--allow-unused-prefixes")
                    .output()
                    .unwrap();
                assert!(
                    fc.status.success(),
                    "FileCheck disagrees with engine on {file} {target}:\n{}",
                    String::from_utf8_lossy(&fc.stderr)
                );
            }
        }
    }
}

#[test]
fn engine_trivial_label() {
    let text = "noise\nand_u32:\n\tand w0\n";
    // Reach into common's internals via a data-free check: reuse check_file
    // with a synthetic test file.
    let dir = std::env::temp_dir().join("bft-engdbg");
    std::fs::create_dir_all(&dir).unwrap();
    let f = dir.join("t.rs");
    std::fs::write(&f, "// CHECK-LABEL: and_u32\n").unwrap();
    common::check_file(&f, text, &["CHECK"]);
}

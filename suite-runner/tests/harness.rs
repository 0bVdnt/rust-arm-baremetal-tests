//! Harness matrix: `cargo test --target <triple>` on every architecture with
//! a result summary. Same command, same output shape everywhere — a row is
//! PASS iff the on-target summary `test result: ok. M passed; 0 failed`
//! appears with M == N from `Executing N tests` and cargo exits 0.
#[path = "common/mod.rs"]
mod common;

use common::{cargo_binary, ALL_TARGETS};
use std::process::Command;

fn package_root() -> std::path::PathBuf {
    let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop(); // suite-runner/ -> package root
    p
}

fn run_with_timeout(mut cmd: Command, secs: u64) -> std::process::Output {
    // Prefer GNU timeout so a hung QEMU session fails instead of hanging CI.
    // Environment is inherited by default (RUSTC_BOOTSTRAP, PATH, ...).
    let prog = cmd.get_program().to_os_string();
    let args: Vec<std::ffi::OsString> = cmd.get_args().map(|a| a.to_os_string()).collect();
    let mut t = Command::new("timeout");
    t.arg(secs.to_string());
    t.arg(prog);
    t.args(args);
    if let Some(dir) = cmd.get_current_dir() {
        t.current_dir(dir);
    }
    match t.output() {
        Ok(o) => o,
        Err(_) => cmd.output().expect("run cargo test"),
    }
}

#[test]
fn harness_matrix() {
    let root = package_root();
    let mut rows: Vec<(&str, &str, String)> = Vec::new();
    for target in ALL_TARGETS {
        // NOTE: `-p` scopes to the bare-metal package: without it cargo
        // would also build suite-runner itself (a std host tool) for the
        // ARM target and fail with "can't find crate for std".
        let mut cmd = Command::new(cargo_binary());
        cmd.arg("test")
            .arg("-p")
            .arg("bft-rust-arm-baremetal-tests")
            .arg("--target")
            .arg(target)
            .current_dir(&root);
        let out = run_with_timeout(cmd, 300);
        let text = String::from_utf8_lossy(&out.stdout).into_owned()
            + &String::from_utf8_lossy(&out.stderr);
        let total = text
            .lines()
            .find_map(|l| l.strip_prefix("Executing "))
            .and_then(|l| l.split_whitespace().next())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "?".to_string());
        let summary = text.lines().find(|l| {
            l.starts_with("test result: ok.") && l.contains("passed;")
        });
        let (status, detail) = match summary {
            Some(s) if out.status.success() => {
                // "test result: ok. 34 passed; 0 failed" — executed == passed?
                let passed = s
                    .split("ok.")
                    .nth(1)
                    .and_then(|r| r.split_whitespace().next())
                    .unwrap_or("?");
                if passed == total && s.contains("0 failed") {
                    ("PASS", format!("{passed}/{total}"))
                } else {
                    ("FAIL", s.to_string())
                }
            }
            _ => ("FAIL", format!("rc={:?}", out.status.code())),
        };
        if status == "FAIL" {
            println!("--- {target} output tail ---\n{}", tail(&text, 30));
        }
        rows.push((target, status, detail));
    }
    println!();
    println!("{:<28}{:<8}tests", "target", "result");
    for (t, s, d) in &rows {
        println!("{t:<28}{s:<8}{d}");
    }
    let failed = rows.iter().filter(|(_, s, _)| *s != "PASS").count();
    println!("HARNESS: {}/{} targets PASS", rows.len() - failed, rows.len());
    assert_eq!(failed, 0, "harness matrix has failures");
}

fn tail(text: &str, n: usize) -> String {
    let lines: Vec<&str> = text.lines().collect();
    lines[lines.len().saturating_sub(n)..].join("\n")
}

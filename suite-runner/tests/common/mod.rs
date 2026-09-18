//! Shared helpers for the suite-runner integration tests.
//!
//! Zero dependencies (stable only): subprocess driving for `rustc`/`cargo`,
//! temp dirs, and a minimal FileCheck subset covering exactly what the
//! `tests/<suite>/*.rs` data files use — `CHECK:` / `CHECK-LABEL:` /
//! `CHECK-NOT:` with `A32`/`A64`/`A32R8`/`A32V7` prefixes and `{{regex}}`
//! patterns (`.*`, `[...]`, `+*?`, `(...)`, `|`). Anything fancier fails
//! loudly instead of silently passing.
//!
//! NOTE: every test binary compiles this module wholesale, so helpers used
//! by only some binaries would warn as dead code — allowed below on purpose.
#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

pub const AARCH64: &str = "aarch64-unknown-none";
pub const AARCH64_SOFT: &str = "aarch64-unknown-none-softfloat";
pub const ARMV8R: &str = "armv8r-none-eabihf";
pub const ARMV7A: &str = "armv7a-none-eabihf";
pub const ARMV7A_SOFT: &str = "armv7a-none-eabi";
pub const ARMV7R: &str = "armv7r-none-eabihf";

/// All bare-metal targets. To extend to a new architecture, append the triple
/// here (and its QEMU line to `.cargo/config.toml`); every matrix picks it up.
pub const ALL_TARGETS: &[&str] = &[AARCH64, AARCH64_SOFT, ARMV8R, ARMV7A, ARMV7A_SOFT, ARMV7R];

/// Hardfloat + AArch64-hardfloat targets (VFP/NEON ABI).
pub const HF_TARGETS: &[&str] = &[AARCH64, ARMV8R, ARMV7A, ARMV7R];

/// Softfloat targets (FP via aeabi/compiler-rt libcalls).
pub const SOFT_TARGETS: &[&str] = &[AARCH64_SOFT, ARMV7A_SOFT];

/// FileCheck prefixes for a target.
pub fn prefixes_for(target: &str) -> Vec<&'static str> {
    if target.starts_with("aarch64") {
        vec!["CHECK", "A64"]
    } else if target.starts_with("armv8r") {
        vec!["CHECK", "A32", "A32R8"]
    } else {
        // armv7a / armv7r (VFPv3 with double precision)
        vec!["CHECK", "A32", "A32V7"]
    }
}

/// Path to a compiletest data file: `tests/<suite>/<name>.rs` next to the
/// parent package (suite-runner lives one level below it).
pub fn data_path(suite_file: &str) -> PathBuf {
    let mut p = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    p.pop(); // suite-runner/ -> package root
    p.push("tests");
    for part in suite_file.split('/') {
        p.push(part);
    }
    p
}

/// Directory holding a compiletest suite: `tests/<suite>` next to parent.
pub fn suite_dir(suite: &str) -> PathBuf {
    let mut p = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    p.pop(); // suite-runner/ -> package root
    p.push("tests");
    if !suite.is_empty() {
        p.push(suite);
    }
    p
}

/// Scratch dir under the system temp dir; removed on drop (best effort).
pub struct TempDir {
    pub path: PathBuf,
}

static TMP_COUNTER: AtomicU64 = AtomicU64::new(0);

impl TempDir {
    pub fn new(tag: &str) -> Self {
        let id = TMP_COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = std::env::temp_dir().join(format!(
            "bft-suite-{}-{}-{}",
            tag,
            std::process::id(),
            id
        ));
        std::fs::create_dir_all(&path).expect("create temp dir");
        TempDir { path }
    }

    pub fn join(&self, name: &str) -> PathBuf {
        self.path.join(name)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// Run `rustc` with `RUSTC_BOOTSTRAP=1` (unlocks `-Z` flags on stable).
/// `target=None` means host build (no `--target` flag).
pub fn rustc(target: Option<&str>, args: &[String]) -> Output {
    let mut cmd = Command::new("rustc");
    cmd.env("RUSTC_BOOTSTRAP", "1");
    if let Some(t) = target {
        cmd.arg("--target").arg(t);
    }
    for a in args {
        cmd.arg(a);
    }
    cmd.output().expect("spawn rustc")
}

pub fn assert_success(what: &str, out: &Output) {
    assert!(
        out.status.success(),
        "{what} failed (rc={:?})\n--- stdout ---\n{}\n--- stderr ---\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

pub fn run_binary(path: &Path) -> Output {
    Command::new(path).output().expect("run test binary")
}

/// Path to the cargo binary driving nested builds (respects the ambient one).
pub fn cargo_binary() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}

// ---------------------------------------------------------------------------
// Minimal FileCheck.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Check,
    Label,
    Not,
}

struct Check {
    kind: Kind,
    re: Regex,
    raw: String,
}

fn parse_checks(src: &str, prefixes: &[&str]) -> Vec<Check> {
    let mut out = Vec::new();
    for line in src.lines() {
        let t = line.trim_start();
        let rest = match t.strip_prefix("//") {
            Some(r) => r.trim_start(),
            None => continue,
        };
        let ci = match rest.find(':') {
            Some(i) => i,
            None => continue,
        };
        let (tok, pat) = rest.split_at(ci);
        let pat = &pat[1..]; // skip ':'
        let tok = tok.trim();
        let mut matched = false;
        for p in prefixes {
            if tok == *p {
                out.push(Check { kind: Kind::Check, re: parse_check_text(pat), raw: pat.to_string() });
                matched = true;
                break;
            }
            if tok == format!("{p}-LABEL") {
                out.push(Check { kind: Kind::Label, re: parse_check_text(pat), raw: pat.to_string() });
                matched = true;
                break;
            }
            if tok == format!("{p}-NOT") {
                out.push(Check { kind: Kind::Not, re: parse_check_text(pat), raw: pat.to_string() });
                matched = true;
                break;
            }
        }
        // Anything else (RUN:, CHECK-gdb:, prose comments, ...) is ignored.
        let _ = matched;
    }
    out
}

/// Build the match regex for one CHECK line.
///
/// FileCheck semantics (this was verified against the real binary): the
/// CHECK text is LITERAL except `{{...}}` spans, whose contents are regex.
/// (An earlier revision treated the whole line as regex — wrong: e.g.
/// `40 + 2` must match literally, not as "4","0",spaces-plus.) Horizontal
/// whitespace runs canonicalize to match any space/tab run.
fn parse_check_text(pat: &str) -> Regex {
    let pat = pat.trim_matches(|c| c == ' ' || c == '\t');
    let mut seq: Vec<(Atom, Quant)> = Vec::new();
    let mut rest = pat;
    loop {
        match rest.find("{{") {
            Some(i) => {
                push_literal(&rest[..i], &mut seq);
                let after = &rest[i + 2..];
                match after.find("}}") {
                    Some(j) => {
                        let mut p = Parser { b: after[..j].as_bytes(), i: 0 };
                        let r = p.parse_alt();
                        assert!(
                            p.i == after[..j].len(),
                            "trailing chars in {{{{...}}}} of CHECK line: {pat}"
                        );
                        // Splice the {{regex}} AST inline as one group atom.
                        seq.push((Atom::Group(Box::new(r)), Quant::One));
                        rest = &after[j + 2..];
                    }
                    None => panic!("unbalanced {{{{ in CHECK line: {pat}"),
                }
            }
            None => {
                push_literal(rest, &mut seq);
                break;
            }
        }
    }
    Regex { alts: vec![seq] }
}

/// Push literal text: every byte literal except horizontal-whitespace runs,
/// which collapse to one HSpace atom (canonicalization).
fn push_literal(lit: &str, seq: &mut Vec<(Atom, Quant)>) {
    let b = lit.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b' ' || b[i] == b'\t' {
            while i < b.len() && (b[i] == b' ' || b[i] == b'\t') {
                i += 1;
            }
            seq.push((Atom::HSpace, Quant::One));
        } else {
            seq.push((Atom::Lit(b[i]), Quant::One));
            i += 1;
        }
    }
}

/// Assert every requested check matches `text` in order (FileCheck order).
pub fn check_file(test_file: &Path, text: &str, prefixes: &[&str]) {
    let src = std::fs::read_to_string(test_file)
        .unwrap_or_else(|e| panic!("read {}: {e}", test_file.display()));
    let checks = parse_checks(&src, prefixes);
    assert!(
        !checks.is_empty(),
        "no checks with prefixes {prefixes:?} in {}",
        test_file.display()
    );
    if let Err(e) = match_checks(&checks, text) {
        panic!(
            "FileCheck failed for {} (prefixes {prefixes:?}): {e}",
            test_file.display()
        );
    }
}

fn line_of(text: &str, byte: usize) -> usize {
    text[..byte.min(text.len())].bytes().filter(|&b| b == b'\n').count() + 1
}

fn match_checks(checks: &[Check], text: &str) -> Result<(), String> {
    let b = text.as_bytes();
    let mut pos = 0usize;
    let mut pending: Vec<&Check> = Vec::new();
    for c in checks {
        if c.kind == Kind::Not {
            pending.push(c);
            continue;
        }
        match find_from(&c.re, b, pos) {
            Some((s, e)) => {
                for n in pending.drain(..) {
                    if let Some((ns, _)) = find_from(&n.re, b, pos) {
                        if ns < s {
                            return Err(format!(
                                "CHECK-NOT '{}' matched at line {} (forbidden between line {} and {})",
                                n.raw.trim(),
                                line_of(text, ns),
                                line_of(text, pos),
                                line_of(text, s)
                            ));
                        }
                    }
                }
                pos = e;
            }
            None => {
                let label = if c.kind == Kind::Label { "CHECK-LABEL" } else { "CHECK" };
                return Err(format!(
                    "{label} '{}' not found after line {}",
                    c.raw.trim(),
                    line_of(text, pos)
                ));
            }
        }
    }
    for n in pending {
        if find_from(&n.re, b, pos).is_some() {
            return Err(format!("trailing CHECK-NOT '{}' matched", n.raw.trim()));
        }
    }
    Ok(())
}

fn find_from(re: &Regex, b: &[u8], pos: usize) -> Option<(usize, usize)> {
    // Empty pattern matches at pos, FileCheck-style.
    if re.alts.iter().all(|a| a.is_empty()) {
        let p = pos.min(b.len());
        return Some((p, p));
    }
    let mut s = pos;
    loop {
        if s > b.len() {
            return None;
        }
        let ends = match_regex(re, b, s);
        if let Some(&e) = ends.iter().max() {
            return Some((s, e));
        }
        s += 1;
    }
}

// ---------------------------------------------------------------------------
// Tiny regex engine (subset): literals, `.`, `*+?`, `[...]`, `(...)`, `|`,
// escapes `\d \w \s` + punctuation. Byte-oriented, backtracking.
// ---------------------------------------------------------------------------

#[derive(Clone)]
enum Atom {
    Lit(u8),
    Dot,
    /// A run of horizontal whitespace in the pattern (FileCheck
    /// canonicalization): matches one or more spaces/tabs in the input.
    HSpace,
    Class { neg: bool, items: Vec<ClassItem> },
    Group(Box<Regex>),
}

#[derive(Clone)]
enum ClassItem {
    Byte(u8),
    Range(u8, u8),
    Digit,
    Word,
    Space,
}

#[derive(Clone, Copy)]
enum Quant {
    One,
    Opt,
    Star,
    Plus,
}

#[derive(Clone)]
struct Regex {
    alts: Vec<Vec<(Atom, Quant)>>,
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<u8> {
        self.b.get(self.i).copied()
    }
    fn next(&mut self) -> Option<u8> {
        let c = self.peek()?;
        self.i += 1;
        Some(c)
    }
    fn parse_alt(&mut self) -> Regex {
        let mut alts = vec![self.parse_seq()];
        while self.peek() == Some(b'|') {
            self.i += 1;
            alts.push(self.parse_seq());
        }
        Regex { alts }
    }
    fn parse_seq(&mut self) -> Vec<(Atom, Quant)> {
        let mut seq = Vec::new();
        while let Some(c) = self.peek() {
            if c == b'|' || c == b')' {
                break;
            }
            let atom = self.parse_atom();
            let q = match self.peek() {
                Some(b'*') => { self.i += 1; Quant::Star }
                Some(b'+') => { self.i += 1; Quant::Plus }
                Some(b'?') => { self.i += 1; Quant::Opt }
                _ => Quant::One,
            };
            seq.push((atom, q));
        }
        seq
    }
    fn parse_atom(&mut self) -> Atom {
        // Collapse horizontal whitespace runs (canonicalization, see above).
        if self.peek() == Some(b' ') || self.peek() == Some(b'\t') {
            while self.peek() == Some(b' ') || self.peek() == Some(b'\t') {
                self.i += 1;
            }
            return Atom::HSpace;
        }
        match self.next().expect("unexpected end of CHECK regex") {
            b'(' => {
                let r = self.parse_alt();
                assert_eq!(self.next(), Some(b')'), "unbalanced ( in CHECK regex");
                Atom::Group(Box::new(r))
            }
            b'[' => self.parse_class(),
            b'.' => Atom::Dot,
            b'\\' => {
                let e = self.next().expect("trailing \\ in CHECK regex");
                match e {
                    b'd' => Atom::Class { neg: false, items: vec![ClassItem::Digit] },
                    b'w' => Atom::Class { neg: false, items: vec![ClassItem::Word] },
                    b's' => Atom::Class { neg: false, items: vec![ClassItem::Space] },
                    b'n' => Atom::Lit(b'\n'),
                    b't' => Atom::Lit(b'\t'),
                    b'r' => Atom::Lit(b'\r'),
                    c => Atom::Lit(c),
                }
            }
            c => Atom::Lit(c),
        }
    }
    fn parse_class(&mut self) -> Atom {
        let neg = if self.peek() == Some(b'^') { self.i += 1; true } else { false };
        let mut items = Vec::new();
        // Leading ']' is literal (FileCheck/LLVM convention).
        if self.peek() == Some(b']') {
            self.i += 1;
            items.push(ClassItem::Byte(b']'));
        }
        loop {
            match self.next().expect("unbalanced [ in CHECK regex") {
                b']' => break,
                b'\\' => {
                    let e = self.next().expect("trailing \\ in class");
                    match e {
                        b'd' => items.push(ClassItem::Digit),
                        b'w' => items.push(ClassItem::Word),
                        b's' => items.push(ClassItem::Space),
                        b'n' => items.push(ClassItem::Byte(b'\n')),
                        b't' => items.push(ClassItem::Byte(b'\t')),
                        c => items.push(ClassItem::Byte(c)),
                    }
                }
                lo => {
                    // a-z range?
                    if self.peek() == Some(b'-') && self.b.get(self.i + 1) != Some(&b']') {
                        self.i += 1; // consume '-'
                        let hi = self.next().expect("unbalanced [ range");
                        items.push(ClassItem::Range(lo, hi));
                    } else {
                        items.push(ClassItem::Byte(lo));
                    }
                }
            }
        }
        Atom::Class { neg, items }
    }
}

fn class_matches(items: &[ClassItem], b: u8) -> bool {
    for it in items {
        let hit = match it {
            ClassItem::Byte(x) => b == *x,
            ClassItem::Range(lo, hi) => *lo <= b && b <= *hi,
            ClassItem::Digit => b.is_ascii_digit(),
            ClassItem::Word => b.is_ascii_alphanumeric() || b == b'_',
            ClassItem::Space => matches!(b, b' ' | b'\t' | b'\n' | b'\r'),
        };
        if hit {
            return true;
        }
    }
    false
}

fn match_atom(atom: &Atom, b: &[u8], pos: usize) -> Vec<usize> {
    if pos >= b.len() {
        return vec![];
    }
    let c = b[pos];
    let hit = match atom {
        Atom::Lit(x) => c == *x,
        Atom::Dot => c != b'\n',
        Atom::HSpace => c == b' ' || c == b'\t',
        Atom::Class { neg, items } => class_matches(items, c) != *neg,
        Atom::Group(g) => return match_regex(g, b, pos),
    };
    if hit {
        // HSpace consumes the whole run (greedy); the Plus-like behavior
        // falls out of the repetition machinery only for explicit quants,
        // so extend here: HSpace is inherently one-or-more.
        if matches!(atom, Atom::HSpace) {
            let mut e = pos + 1;
            while e < b.len() && (b[e] == b' ' || b[e] == b'\t') {
                e += 1;
            }
            return vec![e];
        }
        vec![pos + 1]
    } else {
        vec![]
    }
}

fn match_seq(seq: &[(Atom, Quant)], idx: usize, b: &[u8], pos: usize) -> Vec<usize> {
    if idx == seq.len() {
        return vec![pos];
    }
    let (atom, q) = &seq[idx];
    let (lo, hi) = match q {
        Quant::One => (1usize, Some(1usize)),
        Quant::Opt => (0, Some(1)),
        Quant::Star => (0, None),
        Quant::Plus => (1, None),
    };
    // Candidate end-sets per repetition count, built greedily.
    // counts[k] = end positions after exactly k repetitions.
    let mut counts: Vec<Vec<usize>> = vec![vec![pos]];
    let target_max = hi.unwrap_or(b.len().saturating_sub(pos) + 1);
    while counts.len() - 1 < target_max {
        let k = counts.len() - 1;
        let mut next = Vec::new();
        for &p in &counts[k] {
            for e in match_atom(atom, b, p) {
                if !next.contains(&e) {
                    next.push(e);
                }
            }
        }
        if next.is_empty() {
            break;
        }
        counts.push(next);
        if counts.len() > b.len() + 2 {
            break; // zero-width safety net
        }
    }
    // Greedy order (longest repetition first) is only a search heuristic:
    // collect ALL reachable ends so enclosing groups/patterns can backtrack
    // (returning just the first success breaks nested `{{.*}}` cases).
    let mut out = Vec::new();
    for k in (lo..counts.len()).rev() {
        for &p in &counts[k] {
            for e in match_seq(seq, idx + 1, b, p) {
                if !out.contains(&e) {
                    out.push(e);
                }
            }
        }
    }
    out
}

fn match_regex(re: &Regex, b: &[u8], pos: usize) -> Vec<usize> {
    let mut out = Vec::new();
    for alt in &re.alts {
        for e in match_seq(alt, 0, b, pos) {
            if !out.contains(&e) {
                out.push(e);
            }
        }
    }
    out
}

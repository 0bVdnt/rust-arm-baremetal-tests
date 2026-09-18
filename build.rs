use std::io::Write;

fn main() {
    arm_targets::process();
    let target = std::env::var("TARGET").unwrap_or_default();
    // Per-board memory map: versatileab (ORIGIN=0) vs mps3-an536 (QSPI/DDR).
    // aarch32-rt's link.x does `INCLUDE memory.x`, so we must provide the
    // right one as `memory.x` in the link search path.
    let mem_bytes: &[u8] = if target.starts_with("armv8r") {
        include_bytes!("memory-mps3.x")
    } else {
        include_bytes!("memory.x")
    };
    for (name, bytes) in [
        ("memory.x", mem_bytes),
        ("memory-aarch64.ld", include_bytes!("memory-aarch64.ld") as &[u8]),
    ] {
        let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
        std::fs::File::create(out.join(name))
            .unwrap()
            .write_all(bytes)
            .unwrap();
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=memory.x");
        println!("cargo:rerun-if-changed=memory-mps3.x");
        println!("cargo:rerun-if-changed=memory-aarch64.ld");
    }
    // TARGET already read above for memory map selection.
    // AArch32 runtime expects link.x from aarch32-rt; only pass it for 32-bit ARM.
    // AArch64 uses -Tmemory-aarch64.ld from .cargo/config.toml; passing both
    // scripts makes rust-lld complain about missing memory regions.
    if target.starts_with("arm") || target.starts_with("thumb") {
        println!("cargo:rustc-link-arg=-Tlink.x");
    }
}

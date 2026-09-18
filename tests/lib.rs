#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(custom_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(internal_features)]

// Harness area modules — one per concern, mirroring how bft-rust-coretests
// and bft-rust-alloctests split tests/*.rs by area. Each test is a
// `#[testcase]` fn (see helper/) asserting on-target behaviour; the runner
// below executes them under QEMU semihosting like the sibling suites.
//
// This harness is dual-architecture: AArch32 enters via `aarch32-rt`'s
// `#[entry]`, AArch64 via the local `_start` below (same pattern as
// google/aarch64-rt). Both converge on `boot()`, so
// `cargo test --target <any of the four triples>` behaves identically.
mod alloc_checks;
mod alu;
mod atomics;
mod coverage_probes;
mod float;
mod incr_chain;
mod layout;

extern crate alloc;
use embedded_alloc::LlffHeap as Heap;
use core::panic::PanicInfo;
use semihosting::println;

#[cfg(target_arch = "aarch64")]
use core::arch::global_asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    semihosting::println!("PANIC: {:#?}", _info);
    semihosting::process::abort();
}

#[global_allocator]
static HEAP: Heap = Heap::empty();

// On AArch32 the critical-section impl comes from `aarch32-cpu`'s
// `critical-section-single-core` feature (same as the sibling suites).
// AArch64 has no such provider crate here, so supply the equivalent
// single-core impl locally (mask DAIF, single hart — QEMU virt).
#[cfg(target_arch = "aarch64")]
struct SingleCoreCs;

#[cfg(target_arch = "aarch64")]
// SAFETY: single-core system; masking DAIF excludes all interrupt handlers.
unsafe impl critical_section::Impl for SingleCoreCs {
    #[inline]
    unsafe fn acquire() -> critical_section::RawRestoreState {
        // DAIF mask bits fit in u8 (matches restore-state-u8, same as
        // aarch32-cpu's single-core impl so features unify).
        let daif: u64;
        unsafe {
            core::arch::asm!("mrs {}, daif", out(reg) daif, options(nomem, nostack, preserves_flags));
            core::arch::asm!("msr daifset, #15", options(nostack, preserves_flags));
        }
        // true = interrupts were unmasked (we must restore them on release).
        (daif & 15 == 0) as u8
    }

    #[inline]
    unsafe fn release(was_unmasked: critical_section::RawRestoreState) {
        if was_unmasked != 0 {
            unsafe {
                core::arch::asm!("msr daifclr, #15", options(nostack, preserves_flags));
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
critical_section::set_impl!(SingleCoreCs);

fn init_heap() {
    const HEAP_SIZE: usize = 16 * 1024;
    static mut HEAP_MEM: [core::mem::MaybeUninit<u8>; HEAP_SIZE] =
        [core::mem::MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
}

fn boot() -> ! {
    println!("Starting test execution");
    init_heap();

    #[cfg(test)]
    test_main();

    loop {}
}

// --- AArch32 entry (aarch32-rt startup, linker script link.x) ---
#[cfg(target_arch = "arm")]
use aarch32_rt::entry;

#[cfg(target_arch = "arm")]
#[entry]
unsafe fn arch_start() -> ! {
    boot()
}

// --- AArch64 entry (local _start, linker script memory-aarch64.ld) ---
#[cfg(target_arch = "aarch64")]
global_asm!(
    r#"
    .section .text._start,"ax",%progbits
    .global _start
    .type _start,%function
_start:
    // Enable the FPU (QEMU boots with FP trapped): set FPEN in CPACR_EL1
    // (EL1) or CPTR_EL2 (EL2) according to the current exception level,
    // then ISB before any FP instruction executes.
    mrs x9, CurrentEL
    lsr x9, x9, #2
    cmp x9, #2
    b.ne 1f
    mrs x10, cptr_el2
    orr x10, x10, #(0x3 << 20)   // CPTR_EL2.FPEN = 0b11 (no trap)
    msr cptr_el2, x10
    b 2f
1:  mov x10, #(0x3 << 20)        // CPACR_EL1.FPEN = 0b11 (no trap)
    msr cpacr_el1, x10
2:  isb
    ldr x30, =__stack_top
    mov sp, x30
    bl kmain
3:  wfi
    b 3b
"#
);

#[cfg(target_arch = "aarch64")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmain() -> ! {
    boot()
}

fn custom_runner(tests: &[&dyn Fn()]) {
    println!("Executing {} tests", tests.len());
    for t in tests {
        t();
    }
    // NOTE: a #[testcase] failure panics, which the #[panic_handler] above
    // turns into semihosting abort (nonzero exit), so reaching here means
    // every test passed. The host-side `test result: ok. 0 passed` line
    // cargo prints afterwards is just the empty doc-test phase — THIS is
    // the real on-target summary.
    println!("test result: ok. {} passed; 0 failed", tests.len());
    semihosting::process::exit(0);
}

//! Scalar float checks (mirrors tests/assembly/hardfloat_f32_mul.rs).
//! Hard-float lowering (VFP/NEON regs, no soft-float libcalls) is verified by
//! the assembly tests; here we assert on-target semantics. NB: `f32::abs`
//! stays manual — it is out-of-line at opt-level=0 (see firmware-aarch64).

use helper::testcase;

#[testcase]
fn fmul_f32() {
    assert_eq!(1.5f32 * 2.0, 3.0);
}

#[testcase]
fn fadd_fsub() {
    assert_eq!(1.5f32 + 2.0 - 0.5, 3.0);
}

#[testcase]
fn fdiv() {
    assert_eq!(7.0f32 / 2.0, 3.5);
}

#[testcase]
fn fabs() {
    let a = -3.0f32;
    assert_eq!(if a < 0.0 { -a } else { a }, 3.0);
}

#[testcase]
fn converts() {
    assert_eq!(1i32 as f32, 1.0);
    assert_eq!(2.0f32 as i32, 2);
}

#[testcase]
fn fcmp_min() {
    let (a, b) = (2.0f32, 3.0f32);
    assert_eq!(if a < b { a } else { b }, 2.0);
}

#[testcase]
fn fmul_f64() {
    assert_eq!(2.0f64 * 3.0, 6.0);
}

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

#[testcase]
fn f64_add_div() {
    assert_eq!(1.25f64 + 2.5, 3.75);
    assert_eq!(7.0f64 / 2.0, 3.5);
    assert_eq!(10.0f64 - 4.25, 5.75);
}

#[testcase]
fn fneg() {
    let a = 5.0f32;
    assert_eq!(-a, -5.0);
    assert_eq!(-(-a), 5.0);
    let b = 2.5f64;
    assert_eq!(-b, -2.5);
}

#[testcase]
fn copysign() {
    assert_eq!((-3.0f32).copysign(2.0), 3.0);
    assert_eq!((3.0f32).copysign(-1.0), -3.0);
    assert_eq!((-0.0f32).copysign(-5.0).is_sign_negative(), true);
}

#[testcase]
fn total_cmp() {
    use core::cmp::Ordering::*;
    assert_eq!(2.0f32.total_cmp(&3.0), Less);
    assert_eq!(3.0f32.total_cmp(&3.0), Equal);
    assert_eq!(4.0f32.total_cmp(&3.0), Greater);
    // NaN sorts after everything, deterministically.
    assert_eq!(f32::NAN.total_cmp(&f32::INFINITY), Greater);
}

#[testcase]
fn f64_min_max_manual() {
    let (a, b) = (1.5f64, -2.5f64);
    let mn = if a < b { a } else { b };
    let mx = if a < b { b } else { a };
    assert_eq!((mn, mx), (-2.5, 1.5));
}

#[testcase]
fn float_consts() {
    let pi = core::f32::consts::PI;
    assert!(pi > 3.14 && pi < 3.15);
    assert!(f32::EPSILON > 0.0);
    assert_eq!(f64::MAX, f64::MAX);
    assert_eq!(3.25f64.to_bits(), 0x400A_0000_0000_0000);
}

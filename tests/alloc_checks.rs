//! Allocator checks (mirrors tests/ui-fulldeps/heap_vec.rs).
//! Exercises embedded-alloc LlffHeap via Vec/VecDeque/sort — the same API
//! surface used by the compiletest host-side checks, now on-target.

use alloc::collections::VecDeque;
use alloc::vec;
use helper::testcase;

#[testcase]
fn alloc_vec() {
    let v = vec![1u32, 2, 3];
    assert_eq!(v.iter().sum::<u32>(), 6);
}

#[testcase]
fn alloc_deque() {
    let mut vd = VecDeque::new();
    vd.push_back(5u32);
    vd.push_back(6u32);
    assert_eq!(vd.pop_front(), Some(5));
    assert_eq!(vd.len(), 1);
}

#[testcase]
fn alloc_sort() {
    let mut s = vec![3u32, 1, 2];
    s.sort_unstable();
    assert_eq!(s, vec![1, 2, 3]);
}

#[testcase]
fn alloc_retain() {
    let mut v = vec![1u32, 2, 3, 4, 5, 6];
    v.retain(|x| *x % 2 == 0);
    assert_eq!(v, vec![2, 4, 6]);
}

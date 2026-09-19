//! Allocator checks (mirrors tests/ui-fulldeps/heap_vec.rs).
//! Exercises embedded-alloc LlffHeap via Vec/VecDeque/sort — the same API
//! surface used by the compiletest host-side checks, now on-target.

use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
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

#[testcase]
fn alloc_string() {
    let mut s = String::from("arm");
    s.push_str("-baremetal");
    s.push('!');
    assert_eq!(s, "arm-baremetal!");
    assert_eq!(s.len(), 14);
    assert_eq!(s.into_bytes(), vec![b'a', b'r', b'm', b'-', b'b', b'a', b'r', b'e', b'm', b'e', b't', b'a', b'l', b'!']);
}

#[testcase]
fn alloc_boxed() {
    let b = alloc::boxed::Box::new(0xBEEFu32);
    assert_eq!(*b, 0xBEEF);
    let raw = alloc::boxed::Box::into_raw(b);
    let back = unsafe { alloc::boxed::Box::from_raw(raw) };
    assert_eq!(*back, 0xBEEF);
}

#[testcase]
fn alloc_vec_insert_remove() {
    let mut v = vec![1u32, 3];
    v.insert(1, 2);
    assert_eq!(v, vec![1, 2, 3]);
    assert_eq!(v.remove(0), 1);
    assert_eq!(v.pop(), Some(3));
    assert_eq!(v, vec![2]);
}

#[testcase]
fn alloc_deque_both_ends() {
    let mut vd = VecDeque::new();
    vd.push_front(2u32);
    vd.push_front(1u32);
    vd.push_back(3u32);
    assert_eq!(vd.pop_front(), Some(1));
    assert_eq!(vd.pop_back(), Some(3));
    assert_eq!(vd.len(), 1);
}

#[testcase]
fn alloc_btreemap() {
    let mut m = BTreeMap::new();
    m.insert(3u32, 30u32);
    m.insert(1u32, 10u32);
    m.insert(2u32, 20u32);
    assert_eq!(m.get(&2), Some(&20));
    assert_eq!(m.len(), 3);
    let keys: Vec<u32> = m.keys().copied().collect();
    assert_eq!(keys, vec![1, 2, 3]);
}

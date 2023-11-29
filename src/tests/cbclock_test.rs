use std::{cmp::Ordering, collections::HashMap};

use crate::cbcast::*;
use cbclock::CbcastClock;

//
// Delivery tests
//

#[test]
fn clock_init_and_increment_one() {
    let mut cc = CbcastClock::new(1);
    assert!(cc.into_vec() == vec![]);
    cc.increment();
    assert!(cc.into_vec() == vec![(1, 1)]);
}

#[test]
fn clock_deliverable_empty() {
    let mut cc = CbcastClock::new(1);
    let mut other_cc = CbcastClock::new(2);
    other_cc.increment();
    assert!(cc.is_deliverable(&other_cc));
}

#[test]
fn clock_deliverable_many() {
    let mut cc = CbcastClock::new(1);
    cc.insert(4, 1);
    cc.insert(5, 1);
    cc.insert(6, 1);
    cc.insert(1, 1);
    let mut other_cc = CbcastClock::new(2);
    other_cc.insert(1, 3);
    other_cc.increment();
    assert!(cc.is_deliverable(&other_cc));
}

#[test]
fn clock_deliverable_many_other() {
    let mut cc = CbcastClock::new(1);
    let mut other_cc = CbcastClock::new(2);
    other_cc.increment();
    other_cc.insert(4, 1);
    assert!(!cc.is_deliverable(&other_cc));
}

#[test]
fn clock_not_deliverable_many_other() {
    let mut cc = CbcastClock::new(1);
    let mut other_cc = CbcastClock::new(2);
    other_cc.insert(1, 1);
    other_cc.increment();
    assert!(!cc.is_deliverable(&other_cc));
}

#[test]
fn clock_not_deliverable_other_too_large() {
    let mut cc = CbcastClock::new(1);
    let mut other_cc = CbcastClock::new(2);
    other_cc.insert(1, 3);
    assert!(!cc.is_deliverable(&other_cc));
}

#[test]
fn clock_not_deliverable() {
    let mut cc = CbcastClock::new(1);
    let other_cc = CbcastClock::new(2);
    assert!(!cc.is_deliverable(&other_cc));
}

//
// Compare tests
//

#[test]
fn clock_eq() {
    let cc = CbcastClock::new(1);
    let other_cc = CbcastClock::new(2);
    assert!(cc.eq(&other_cc))
}

#[test]
fn clock_neq() {
    let mut cc_map = HashMap::new();
    cc_map.insert(1, 0);
    cc_map.insert(2, 0);
    let mut other_map = HashMap::new();
    other_map.insert(1, 0);
    other_map.insert(2, 0);
    let cc = CbcastClock::from_map(1, cc_map);
    let mut other_cc = CbcastClock::from_map(2, other_map);
    other_cc.increment();
    assert!(!cc.eq(&other_cc))
}

#[test]
fn clock_cmp_equal() {
    let cc = CbcastClock::new(1);
    let other_cc = CbcastClock::new(2);
    assert!(cc.cmp(&other_cc) == Ordering::Equal);
}

#[test]
fn clock_cmp_equal_cc_greater() {
    let mut cc = CbcastClock::new(1);
    cc.increment();
    let other_cc = CbcastClock::new(2);
    assert!(cc.cmp(&other_cc) == Ordering::Equal);
}

#[test]
#[should_panic]
fn clock_cmp_fail() {
    let mut hmap = HashMap::new();
    hmap.insert(1, 1);
    hmap.insert(2, 0);

    let mut hmap_other = HashMap::new();
    hmap_other.insert(1, 0);
    hmap_other.insert(2, 1);

    let cc = CbcastClock::from_map(1, hmap);
    let other_cc = CbcastClock::from_map(2, hmap_other);
    // Below is to get around clippy
    if cc.cmp(&other_cc) == Ordering::Greater {
        return
    }
}

#[test]
fn clock_cmp_greater() {
    let mut hmap = HashMap::new();
    hmap.insert(1, 1);
    hmap.insert(2, 0);

    let mut hmap_other = HashMap::new();
    hmap_other.insert(1, 0);
    hmap_other.insert(2, 0);

    let cc = CbcastClock::from_map(1, hmap);
    let other_cc = CbcastClock::from_map(2, hmap_other);
    assert!(cc.cmp(&other_cc) == Ordering::Greater);
}

#[test]
fn clock_cmp_less() {
    let mut hmap = HashMap::new();
    hmap.insert(1, 1);
    hmap.insert(2, 0);

    let mut hmap_other = HashMap::new();
    hmap_other.insert(1, 0);
    hmap_other.insert(2, 0);

    let cc = CbcastClock::from_map(1, hmap);
    let other_cc = CbcastClock::from_map(2, hmap_other);
    assert!(other_cc.cmp(&cc) == Ordering::Less);
}

#[test]
fn clock_fmt() {
    let mut cc = CbcastClock::new(1);
    assert!(cc.to_string() == "[]");
    cc.increment();
    assert!(cc.to_string() == "[(1, 1)]");
}

#[test]
fn clock_from_map() {
    let hmap: HashMap<i32, u32> = HashMap::new();
    let cc = CbcastClock::from_map(1, hmap);
    assert!(cc.to_string() == "[]")
}

#[test]
fn clock_insert() {
    let mut cc = CbcastClock::new(1);
    cc.insert(1, 3);
    assert!(cc.to_string() == "[(1, 3)]")
}

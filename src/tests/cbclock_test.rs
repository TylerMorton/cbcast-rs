use std::collections::HashMap;

use crate::cbcast::*;
use cbclock::CbcastClock;

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
fn clock_not_deliverable() {
    let mut cc = CbcastClock::new(1);
    let other_cc = CbcastClock::new(2);
    assert!(!cc.is_deliverable(&other_cc));
}

#[test]
fn clock_eq() {
    let cc = CbcastClock::new(1);
    let other_cc = CbcastClock::new(2);
    assert!(cc.eq(&other_cc))
}

#[test]
fn clock_neq() {
    let cc = CbcastClock::new(1);
    let mut other_cc = CbcastClock::new(2);
    other_cc.increment();
    assert!(!cc.eq(&other_cc))
}

// #[test]
// fn clock_cmp() {
//   let cc = CbcastClock::new(1);
//   let mut other_cc = CbcastClock::new(2);
//   other_cc.increment();
//   assert!(cc.cmp(&other_cc) == Ordering::Less);
// }

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

use std::net::*;
use crate::cbcast::*;
use cbclock::CbcastClock;
use cbmessage::CbcastMessage;
use cbprocess::CbcastProcess;

// What tests do I need?



// Initialization addr is not a ToSocketAddrs type
// Initialization addr is a ToSocketAddrs type
// Initialization

#[test]
fn clock_init_and_increment_one() {
  let mut cc = CbcastClock::new(1);
  assert!(cc.into_vec() == vec![]);
  cc.increment();
  assert!(cc.into_vec() == vec![(1, 1)]);
}

#[test]
fn clock_deliverable() {
  let mut cc = CbcastClock::new(1);
  let mut other_cc = CbcastClock::new(2);
  other_cc.increment();
  assert!(cc.is_deliverable(&other_cc));
}

#[test]
fn clock_not_deliverable() {
  let mut cc = CbcastClock::new(1);
  let other_cc = CbcastClock::new(2);
  assert!(!cc.is_deliverable(&other_cc));
}


#[test]
fn process_init() {
  let p_1 = CbcastProcess::new(
    1,
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
);
  assert!(p_1.id == 1);
}




// Major test case: two processes talk, delivery is out of order.
// Message is queue'd and delivery according to causal ordering.
#[test]
fn causal_ordering() {}

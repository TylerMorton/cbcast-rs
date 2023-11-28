use crate::cbcast::*;
use cbmessage::CbcastMessage;
use cbprocess::CbcastProcess;
use std::net::*;

#[test]
fn process_init() {
    let p_1 = CbcastProcess::new(
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    );
    assert!(p_1.id == 1);
}

#[test]
fn process_fmt() {
    let p_1 = CbcastProcess::new(
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    );
    assert!(p_1.to_string() == "id: 1, vector clock: [(1, 0)]");
}

#[test]
fn process_send() {
    let mut p_1 = CbcastProcess::new(
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    );
    p_1.send("hello", 1);
    assert!(p_1.to_string() == "id: 1, vector clock: [(1, 1)]");
}

#[test]
fn process_receive() {
    fn handler(_: u32) {}

    let mut p_1 = CbcastProcess::new(
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    );
    let p_2 = CbcastProcess::new(
        2,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    );
    let message = CbcastMessage::new(p_2.id, p_2.cc.into_vec(), 3);
    let message = serde_json::to_string(&message).expect("will work");

    p_1.receive(&message, handler);
    println!("{}", p_1);
    let result_str = p_1.to_string();
    assert!(
        result_str == "id: 1, vector clock: [(1, 0), (2, 0)]"
            || result_str == "id: 1, vector clock: [(2, 0), (1, 0)]"
    );
}

#[test]
fn process_receive_causally_unordered_no_delivery() {
    fn handler(_: u32) {}

    let mut p_1 = CbcastProcess::new(
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    );
    let p_2 = CbcastProcess::new(
        2,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    );
    let mut p_3 = CbcastProcess::new(
        3,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    );
    let message = CbcastMessage::new(p_2.id, p_2.cc.into_vec(), 3);
    let message = serde_json::to_string(&message).expect("will work");

    p_1.receive(&message, handler);
    let reply_message = CbcastMessage::new(p_1.id, p_1.cc.into_vec(), 4);
    let reply_message = serde_json::to_string(&reply_message).expect("will work");
    p_3.receive(&reply_message, handler);
    let result_str = p_3.to_string();
    assert!(result_str == "id: 3, vector clock: [(3, 0)]");
}

#[test]
fn process_receive_causally_unordered_delivery() {
    fn handler(_: u32) {}

    let mut p_1 = CbcastProcess::new(
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    );
    let p_2 = CbcastProcess::new(
        2,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    );
    let mut p_3 = CbcastProcess::new(
        3,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    );
    let message = CbcastMessage::new(p_2.id, p_2.cc.into_vec(), 3);
    let message = serde_json::to_string(&message).expect("will work");

    p_1.receive(&message, handler);
    let reply_message = CbcastMessage::new(p_1.id, p_1.cc.into_vec(), 4);
    let reply_message = serde_json::to_string(&reply_message).expect("will work");
    p_3.receive(&reply_message, handler);
    p_3.receive(&message, handler);
    let result_str = p_3.to_string();
    println!("{}", result_str);
    assert!(
        result_str == "id: 3, vector clock: [(1, 0), (2, 0), (3, 0)]"
            || result_str == "id: 3, vector clock: [(3, 0), (1, 0), (2, 0)]"
            || result_str == "id: 3, vector clock: [(3, 0), (2, 0), (1, 0)]"
            || result_str == "id: 3, vector clock: [(2, 0), (3, 0), (1, 0)]"
            || result_str == "id: 3, vector clock: [(2, 0), (1, 0), (3, 0)]"
            || result_str == "id: 3, vector clock: [(1, 0), (3, 0), (2, 0)]"
    );
}
// Major test case: two processes talk, delivery is out of order.
// Message is queue'd and delivery according to causal ordering.
// #[test]
// fn causal_ordering() {}

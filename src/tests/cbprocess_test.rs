use crate::cbcast::*;
use cbmessage::CbcastMessage;
use cbprocess::CbcastProcess;
use std::{net::*, vec};

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
    assert!(
        p_1.to_string() == "socket address exists, listener active, id: 1, vector clock: [(1, 0)]"
    );
}

#[test]
fn process_broadcast() {
    let mut p_1 = CbcastProcess::new(
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6000),
    );
    let mut p_2 = CbcastProcess::new(
        2,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6001),
    );
    p_1.viewgroup_add((
        2,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6001),
    ));
    p_2.listener_up();
    p_1.connections_up();
    assert!(p_1.connections_list().len() == 1);
    // let cc = p_1.cc.into_vec();
    // let message = CbcastMessage::new(1, cc, 10);
    p_1.broadcast("hello");
    // assert!(String::from_utf8(a).unwrap() == String::from("hello"));
    // p_1.viewgroup_add(node)
    // p_1.("hello", 1);
    p_1.connection_down();
    assert!(p_1.connections_list().len() == 0);
    println!("{}", p_1.to_string());
    let string = p_1.to_string();
    assert!(
        string == "socket address exists, id: 1, vector clock: [(2, 0), (1, 1)]"
            || string == "socket address exists, id: 1, vector clock: [(1, 1), (2, 0)]"
    );
}

#[test]
fn process_read() {
    let mut p_1 = CbcastProcess::new(
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7000),
    );
    println!("{:?}", p_1.read());
    assert!(p_1.read().len() == 0);
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
        result_str
            == "socket address exists, listener active, id: 1, vector clock: [(1, 0), (2, 0)]"
            || result_str
                == "socket address exists, listener active, id: 1, vector clock: [(2, 0), (1, 0)]"
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
    assert!(result_str == "socket address exists, listener active, id: 3, vector clock: [(3, 0)]");
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
        result_str == "socket address exists, listener active, id: 3, vector clock: [(1, 0), (2, 0), (3, 0)]"
            || result_str == "socket address exists, listener active, id: 3, vector clock: [(3, 0), (1, 0), (2, 0)]"
            || result_str == "socket address exists, listener active, id: 3, vector clock: [(3, 0), (2, 0), (1, 0)]"
            || result_str == "socket address exists, listener active, id: 3, vector clock: [(2, 0), (3, 0), (1, 0)]"
            || result_str == "socket address exists, listener active, id: 3, vector clock: [(2, 0), (1, 0), (3, 0)]"
            || result_str == "socket address exists, listener active, id: 3, vector clock: [(1, 0), (3, 0), (2, 0)]"
    );
}
// Major test case: two processes talk, delivery is out of order.
// Message is queue'd and delivery according to causal ordering.
// #[test]
// fn causal_ordering() {}

// Viewgroup tests

#[test]
fn viewgroup_add() {
    let mut p_1 = CbcastProcess::new(
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5001),
    );
    // let mut p_2 = CbcastProcess::new(
    //     2,
    //     SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5001),
    // );
    p_1.viewgroup_add((
        2,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5001),
    ));
    println!("{:?}", p_1.viewgroup_list());
    assert!(p_1.viewgroup_list() == vec![2]);
}

#[test]
fn viewgroup_list() {
    let p_1 = CbcastProcess::new(
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5001),
    );
    assert!(p_1.viewgroup_list().len() == 0);
}

#[test]
fn viewgroup_remove() {
    let mut p_1 = CbcastProcess::new(
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5001),
    );
    p_1.viewgroup_add((
        2,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5001),
    ));
    assert!(p_1.viewgroup_list() == vec![2]);
    p_1.viewgroup_remove(2);
    assert!(p_1.viewgroup_list().len() == 0);
}

#[test]
#[should_panic]
fn process_listener_failed() {
    let mut p_1 = CbcastProcess::new(
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(200, 255, 0, 1)), 0),
    );
    p_1.listener_up();
}

#[test]
fn process_connections_up() {
    let mut p_1 = CbcastProcess::new(
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000),
    );
    let mut p_2 = CbcastProcess::new(
        2,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8001),
    );
    p_1.viewgroup_add((
        2,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8001),
    ));
    p_2.listener_up();
    p_1.connections_up();
    assert!(p_1.connections_list().len() == 1);
    assert!(p_1.cc.into_vec().len() == 2);
}

#[test]
fn process_connections_up_and_down() {
    let mut p_1 = CbcastProcess::new(
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5000),
    );
    let mut p_2 = CbcastProcess::new(
        2,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5001),
    );
    p_1.viewgroup_add((
        2,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5001),
    ));
    p_2.listener_up();
    p_1.connections_up();
    assert!(p_1.connections_list().len() == 1);
    p_1.connection_down();
    assert!(p_1.connections_list().len() == 0);
}

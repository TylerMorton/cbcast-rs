use cbcast::cbcast::cbmessage::CbcastMessage;
use cbcast::cbcast::cbprocess::CbcastProcess;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn normal_process(
    mut p_1: CbcastProcess<i32, SocketAddr>,
    mut p_2: CbcastProcess<i32, SocketAddr>,
    handler: fn(u32),
) {
    println!("Processes are initialized");
    println!("{}", p_1);
    println!("{}", p_2);
    println!("Process 2 sends a message");
    p_2.send("hello", 1);
    println!("{}", p_2);
    let message = CbcastMessage::new(p_2.id, p_2.cc.into_vec(), 3);
    let message = serde_json::to_string(&message).expect("will work");
    p_1.receive(&message, handler);
    println!("{}", p_1);
    p_1.send("hello", 1);
    println!("{}", p_1);
    println!("{}", p_2);
    let message = CbcastMessage::new(p_1.id, p_1.cc.into_vec(), 4);
    let message = serde_json::to_string(&message).expect("will work");
    p_2.receive(&message, handler);
    println!("{}", p_2);
}

fn causal_order_out_of_order(
    mut p_1: CbcastProcess<i32, SocketAddr>,
    mut p_2: CbcastProcess<i32, SocketAddr>,
    mut p_3: CbcastProcess<i32, SocketAddr>,
    handler: fn(u32),
) {
    println!("Processes are initialized");
    println!("{}", p_1);
    println!("{}", p_2);
    println!("{}", p_3);
    println!("Process 2 sends a message");
    p_2.send("where are my keys?", 1);
    println!("{}", p_2);
    let where_message = CbcastMessage::new(p_2.id, p_2.cc.into_vec(), 3);
    let where_message = serde_json::to_string(&where_message).expect("where are my keys?");
    println!("Process 3 receives message from p2 and responds");
    p_3.receive(&where_message, handler);
    p_3.send("found it!", 1);
    println!("{}", p_3);
    let found_message = CbcastMessage::new(p_3.id, p_3.cc.into_vec(), 4);
    let found_message = serde_json::to_string(&found_message).expect("found it.");

    println!("{}", p_1);
    println!("Process 1 receives message from p3 ");
    p_1.receive(&found_message, handler);
    println!("{}", p_1);
    println!("{}", p_2);
    println!("{}", p_3);
    println!("Process 2 receives message from p3 ");
    p_2.receive(&found_message, handler);
    println!("{}", p_2);
    println!("{}", p_3);

    p_1.receive(&where_message, handler);
    println!("{}", p_1);
    p_1.send("hello", 1);
    println!("{}", p_1);
    println!("{}", p_2);
    let message = CbcastMessage::new(p_1.id, p_1.cc.into_vec(), 4);
    let message = serde_json::to_string(&message).expect("will work");
    p_2.receive(&message, handler);
    println!("{}", p_2);
    println!("resulting vector clocks");
    println!("{}", p_1);
    println!("{}", p_2);
    println!("{}", p_3);
}

fn main() {
    // how to handle message
    pub fn handler(i: u32) {
        println!("handled number: {}", i);
    }

    // Process initialization
    let p_1 = CbcastProcess::new(
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    );
    let p_2 = CbcastProcess::new(
        2,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    );
    let p_3 = CbcastProcess::new(
        3,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    );

    if env::args().next_back().is_some() {
        causal_order_out_of_order(p_1, p_2, p_3, handler);
    } else {
        normal_process(p_1, p_2, handler);
    }
}

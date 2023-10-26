use cbcast::cbcast::{CbcastMessage, CbcastProcess};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
    // how to handle message
    pub fn handler(i: u32) {
        println!("handled number: {}", i);
    }

    // Process initialization
    let mut p_1 = CbcastProcess::new(
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    );
    let mut p_2 = CbcastProcess::new(
        2,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    );

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

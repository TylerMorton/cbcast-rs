use crate::cbcast::*;
use cbprocess::CbcastProcess;
use std::{collections::VecDeque, net::*};

// Utility functions
pub fn create_processes(num: u32, ports: Vec<u16>) -> VecDeque<CbcastProcess<i32, SocketAddr>> {
    if num as usize != ports.len() {
        panic!("number of processes must equal length of ports");
    }
    let mut result: VecDeque<CbcastProcess<i32, SocketAddr>> = VecDeque::new();
    for i in 0..num {
        result.push_back(CbcastProcess::new(
            i as i32 + 1,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), ports[i as usize]),
        ))
    }
    result
}

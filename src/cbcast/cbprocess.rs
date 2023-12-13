use serde::de;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::hash::Hash;
use std::io::{Read, Write};
use std::net::*;

use crate::cbcast::cbclock::CbcastClock;
use crate::cbcast::cbmessage::CbcastMessage;

pub struct CbcastProcess<
    I: serde::Serialize + de::DeserializeOwned + Display + Eq + Hash + Copy,
    A: ToSocketAddrs,
> {
    pub id: I,
    addr: A,
    pub cc: CbcastClock<I>,
    listener: Option<TcpListener>,
    // TODO: use a crappy multicast by iterating through tcp sets? group: HashSet<>
    viewgroup: HashMap<I, A>,
    streams: HashMap<I, TcpStream>,
    // Since the paper assumes FIFO I really want to use the guarantees of TCP!
    causal_queue: BTreeMap<CbcastClock<I>, u32>,
    // TODO: Confirm ordering in BTree is correct. Make multiple different lamport diagram scenarios to test this.
}

impl<I: serde::Serialize + de::DeserializeOwned + Copy + Eq + Hash + Display, A: ToSocketAddrs>
    Display for CbcastProcess<I, A>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO: Doing this to make clippy happy, will change once implemented.
        if self.addr.to_socket_addrs().is_ok() {
            write!(f, "socket address exists, ")?;
        }
        if self.listener.is_none() {
            write!(f, "listener active, ")?;
        }
        write!(f, "id: {},", self.id)?;
        write!(f, " vector clock: {}", self.cc)?;
        Ok(())
    }
}

impl<I: serde::Serialize + de::DeserializeOwned + Copy + Eq + Hash + Display, A: ToSocketAddrs>
    CbcastProcess<I, A>
{
    pub fn new(id: I, addr: A) -> CbcastProcess<I, A> {
        let mut s = CbcastProcess {
            id,
            cc: CbcastClock::new(id),
            listener: None,
            addr,
            viewgroup: HashMap::new(),
            streams: HashMap::new(),
            causal_queue: BTreeMap::new(),
        };
        s.cc.insert(s.id, 0);
        s
    }

    pub fn viewgroup_add(&mut self, node: (I, A)) {
        if self.viewgroup.get(&node.0).is_none() {
            self.viewgroup.insert(node.0, node.1);
        }
    }

    pub fn viewgroup_remove(&mut self, id: I) {
        self.viewgroup.remove(&id);
    }

    pub fn viewgroup_list(&self) -> Vec<I> {
        self.viewgroup.iter().map(|a| *a.0).collect()
    }

    // attempts to create streams to all members of view
    // all available connections are viewable in self.streams

    pub fn listener_up(&mut self) {
        let addr = &self.addr;
        self.listener = Some(TcpListener::bind(addr).expect("bind failed"));
    }

    pub fn connections_up(&mut self) {
        if self.listener.is_none() {
            self.listener_up();
        }
        for (id, addr) in self.viewgroup.iter() {
            if self.streams.get(id).is_some() {
                continue;
            }
            if let Ok(stream) = TcpStream::connect(addr) {
                self.streams.insert(*id, stream);
                self.cc.insert(*id, 0);
            }
        }
    }

    pub fn connections_list(&self) -> Vec<(&I, &TcpStream)> {
        self.streams.iter().collect()
    }

    // drops connections (nicely) to all members
    pub fn connection_down(&mut self) {
        for (i, _) in self.viewgroup.iter() {
            if let Some(stream) = self.streams.get(i) {
                stream
                    .shutdown(Shutdown::Both)
                    .expect("shutdown failed for a stream");
                self.streams.remove(i);
            }
        }
    }

    // TODO make message generic or some other more useful datatype instead of u32.
    // TODO: Actually send the message with TCP (I assume FIFO & am too lazy to implement my own protocol on UDP).
    pub fn broadcast<J>(&mut self, message: J)
    where
        J: serde::Serialize,
    {
        self.cc.increment();
        let vector_clock = self.cc.into_vec();
        let cbmessage = CbcastMessage::new(self.id, vector_clock, message);
        let mut serial_message = serde_json::to_string(&cbmessage).unwrap();
        serial_message.push_str("\r\n");
        let expected_len = serial_message.len();

        for stream in self.streams.values_mut() {
            if expected_len != (*stream).write(serial_message.as_bytes()).unwrap() {
                panic!("tcp write didn't broadcast all data.");
            }
        }
    }

    // pub fn send(&mut self, _sender_address: &str, message: u32) {
    //     let id = self.id;
    //     self.cc.increment();
    //     let i: Vec<(I, u32)> = self.cc.into_vec();
    //     let _message = CbcastMessage::new(id, i, message);
    // }

    //TODO combine read & receive
    pub fn read(&mut self) -> Vec<u8> {
        let mut buffer: [u8; 128] = [0; 128];
        let mut stream_read = false;
        for i in &mut self.streams {
            if (*i.1).read(&mut buffer).is_ok() {
                stream_read = true
            }
        }
        if !stream_read && !self.causal_queue.is_empty() {
            self.causal_queue
                .pop_first()
                .unwrap()
                .1
                .to_be_bytes()
                .to_vec()
            // read from queue
        } else {
            Vec::new()
        }
    }

    // TODO: Implement receive
    // TODO: need 5 tuple here
    pub fn receive(&mut self, message: &str, handler: fn(u32)) {
        // TODO serialize generics
        let message: CbcastMessage<I, u32> = serde_json::from_str(message).unwrap();
        let hmap: HashMap<_, _> = message.cc.into_iter().collect();
        let cc = CbcastClock::from_map(message.sender_id, hmap);
        if self.cc.is_deliverable(&cc) {
            println!("message delivered");
            handler(message.data)
        } else {
            // if causality order is ok then process a bunch of messages
            // add to map
            self.causal_queue.insert(cc, message.data);
            println!("message not delivered");
        }
        while !self.causal_queue.is_empty() {
            if let Some(entry) = self.causal_queue.first_entry() {
                if self.cc.is_deliverable(entry.key()) {
                    self.causal_queue.pop_first();
                    continue;
                }
                break;
            }
        }
    }
}

use serde::de;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::hash::Hash;
use std::net::{TcpListener, ToSocketAddrs};

use crate::cbcast::cbclock::CbcastClock;
use crate::cbcast::cbmessage::CbcastMessage;

pub struct CbcastProcess<I: de::DeserializeOwned + Display + Eq + Hash + Copy, A: ToSocketAddrs> {
    pub id: I,
    addr: A,
    pub cc: CbcastClock<I>,
    listener: Option<TcpListener>,
    // TODO: use a crappy multicast by iterating through tcp sets? group: HashSet<>
    // Since the paper assumes FIFO I really want to use the guarantees of TCP!
    causal_queue: BTreeMap<CbcastClock<I>, u32>,
    // TODO: Confirm ordering in BTree is correct. Make multiple different lamport diagram scenarios to test this.
}

impl<I: de::DeserializeOwned + Copy + Eq + Hash + Display, A: ToSocketAddrs> Display
    for CbcastProcess<I, A>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {},", self.id)?;
        write!(f, " vector clock: {}", self.cc)?;
        Ok(())
    }
}

impl<I: de::DeserializeOwned + Copy + Eq + Hash + Display, A: ToSocketAddrs> CbcastProcess<I, A> {
    pub fn new(id: I, addr: A) -> CbcastProcess<I, A> {
        let mut s = CbcastProcess {
            id,
            cc: CbcastClock::new(id),
            listener: None,
            addr,
            causal_queue: BTreeMap::new(),
        };
        s.cc.insert(s.id, 0);
        s
    }

    // TODO: Actually send the message with TCP (I assume FIFO & am too lazy to implement my own protocol on UDP).
    pub fn send(&mut self, _sender_address: &str, message: u32) {
        let id = self.id;
        self.cc.increment();
        let i = self.cc.into_vec();
        let _message = CbcastMessage::new(id, i, message);
    }

    // Starts a listener on local address
    pub fn listener(&mut self) {
        let addr = &self.addr;
        self.listener = Some(TcpListener::bind(addr).expect("bind failed"));
    }

    //   pub fn read(&mut self, handler: fn(u32)) {
    //       let mut incoming = self.listener.as_ref().unwrap().incoming();
    //       let accept = incoming.next();
    //       if let Some(Ok(mut stream)) = accept {
    //           let mut buffer = String::new();
    //           stream
    //               .read_to_string(&mut buffer)
    //               .expect("Socket to buffer fail.");
    //           self.receive(&buffer, handler);
    //       }
    //   }

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
                } else {
                    break;
                }
            }
            break;
        }
    }
}

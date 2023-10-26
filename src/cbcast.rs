use serde::de;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::io::Read;
use std::net::{TcpListener, ToSocketAddrs};

// NOTE: Make the library generic.

// TODO: Figure out a good way to allow a user to send operations
// TODO: Figure out a good way for user to receiv operations.

pub struct CbcastProcess<I: de::DeserializeOwned + Display + Eq + Hash + Copy, A: ToSocketAddrs> {
    pub id: I,
    addr: A,
    pub cc: CbcastClock<I>,
    listener: Option<TcpListener>,
    send_queue: Vec<u32>,
    // TODO: use a crappy multicast by iterating through tcp sets? group: HashSet<>
    // TODO: CBCAST QUEUE to ensure Casual delivery queue: CbcastQueue,
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
            send_queue: vec![],
        };
        s.cc.vc.insert(s.id, 0);
        s
    }

    // TODO: Actually send the message with TCP (I assume FIFO & am too lazy to implement something on UDP).
    pub fn send(&mut self, _sender_address: &str, message: u32) {
        let id = self.id;
        self.cc.increment(id);
        let i = self.cc.into_vec();
        let _message = CbcastMessage::new(id, i, message);
    }

    // Starts a listener on local address
    pub fn listener(&mut self) {
        let addr = &self.addr;
        self.listener = Some(TcpListener::bind(addr).expect("bind failed"));
    }

    // TODO: Work in progress for allowing operations??
    pub fn worker(&mut self, handler: fn(u32)) {
        if !self.send_queue.is_empty() {
            let val = self.send_queue.pop().unwrap();
            self.send("placeholder ip address", val);
            return;
        }
        self.read(handler);
    }

    pub fn read(&mut self, handler: fn(u32)) {
        let mut incoming = self.listener.as_ref().unwrap().incoming();
        let accept = incoming.next();
        if let Some(Ok(mut stream)) = accept {
            let mut buffer = String::new();
            stream
                .read_to_string(&mut buffer)
                .expect("Socket to buffer fail.");
            self.receive(&buffer, handler);
        }
    }

    // TODO: Implement receive
    // TODO: need 5 tuple here
    // FIXME: Not updating vector clock
    pub fn receive(&mut self, message: &str, handler: fn(u32)) {
        // TODO serialize generics
        let message: CbcastMessage<I, u32> = serde_json::from_str(message).unwrap();
        let hmap: HashMap<_, _> = message.cc.into_iter().collect();
        let cc = CbcastClock {
            vc: hmap,
            id: message.sender_id,
        };
        if self.cc.is_deliverable(cc) {
            println!("message delivered");
            handler(message.data)
        } else {
            println!("message not delivered");
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CbcastClock<T: Eq + Hash + Display> {
    vc: HashMap<T, u32>,
    id: T,
}

impl<T: Eq + Hash + Display> Display for CbcastClock<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut iterator = self.vc.iter();
        let first = iterator.next();
        if let Some(val) = first {
            write!(f, "({}, {})", val.0, val.1)?;
        }
        if first.is_some() {}
        for i in iterator {
            write!(f, ", ({}, {})", i.0, i.1)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}
impl<T: Display + Eq + Hash + Copy> CbcastClock<T> {
    pub fn new(id: T) -> Self {
        CbcastClock {
            vc: HashMap::new(),
            id,
        }
    }

    pub fn increment(&mut self, k: T) -> Option<u32> {
        let val = *self.vc.get_mut(&k).unwrap_or(&mut 0);
        self.vc.insert(k, val + 1)
    }

    //FIXME: Not correctly checking vector clocks. Should skip over own process index.
    pub fn is_deliverable(&mut self, mut other: CbcastClock<T>) -> bool {
        let mut incr_condition: Option<T> = None;

        for (k, v) in self.vc.iter() {
            if (*k) == self.id {
                continue;
            }
            let other_val = *other.vc.get(k).unwrap_or(&0);
            if other_val == *v + 1 && incr_condition.is_none() {
                other.vc.remove(k);
                incr_condition = Some(*k);
                continue;
            } else if other_val > *v {
                return false;
            }
            other.vc.remove(k);
        }
        let mut ve = Vec::new();
        for (k, v) in other.vc.iter() {
            println!("other hashmap: {} {}", *k, *v);
            if *v <= 1 {
                ve.push((*k, *v));
            } else {
                return false;
            }
        }
        for (k, v) in &ve {
            self.vc.insert(*k, *v);
        }
        if let Some(index) = incr_condition {
            self.increment(index);
        }
        incr_condition.is_some() || !ve.is_empty()
    }

    pub fn into_vec(&self) -> Vec<(T, u32)> {
        let mut v: Vec<(T, u32)> = Vec::new();
        for i in self.vc.iter() {
            v.push((*i.0, *i.1));
        }
        v
    }
}

#[derive(Serialize, Deserialize)]
pub struct CbcastMessage<I: Display + Eq + Hash, D> {
    sender_id: I,
    cc: Vec<(I, u32)>,
    data: D,
}

impl<I: Display + Eq + Hash, D> CbcastMessage<I, D> {
    pub fn new(sender_id: I, cc: Vec<(I, u32)>, data: D) -> Self {
        CbcastMessage {
            sender_id,
            cc,
            data,
        }
    }
}
// pub struct CbcastQueue<T> {
//   bh: BinaryHeap<T>,
// }

// impl<T> CbcastQueue<T> {
//   pub fn new() {}

// }

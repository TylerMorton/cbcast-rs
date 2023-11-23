use serde::de;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::hash::Hash;
use std::io::Read;
use std::net::{TcpListener, ToSocketAddrs};

// NOTE: Make the library generic.
// TODO: Figure out a good way to allow a user to send operations
// TODO: Figure out a good way for user to receiv operations.
// TODO: Refactor

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
        s.cc.vc.insert(s.id, 0);
        s
    }

    // TODO: Actually send the message with TCP (I assume FIFO & am too lazy to implement my own protocol on UDP).
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
    pub fn receive(&mut self, message: &str, handler: fn(u32)) {
        // TODO serialize generics
        let message: CbcastMessage<I, u32> = serde_json::from_str(message).unwrap();
        let hmap: HashMap<_, _> = message.cc.into_iter().collect();
        let cc = CbcastClock {
            vc: hmap,
            id: message.sender_id,
        };
        if self.cc.is_deliverable(&cc) {
            println!("message delivered");
            handler(message.data)
        } else {
            // if causality order is ok then process a bunch of messages
            // add to map
            self.causal_queue.insert(cc, message.data);
            println!("message not delivered");
        }
        while self.causal_queue.len() > 0 {
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

#[derive(Serialize, Deserialize, Eq)]
pub struct CbcastClock<T: Eq + Hash + Display> {
    vc: HashMap<T, u32>,
    id: T,
}
impl<T: Eq + Hash + Display> PartialEq for CbcastClock<T> {
    fn eq(&self, other: &Self) -> bool {
        self.vc.iter().eq_by(other.vc.iter(), |x, y| x.1.eq(y.1))
    }
}

impl<T: Eq + Hash + Display> PartialOrd for CbcastClock<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.vc.iter().cmp_by(other.vc.iter(), |x, y| x.1.cmp(y.1)))
    }
}

impl<T: Eq + Hash + Display> Ord for CbcastClock<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.vc.iter().cmp_by(other.vc.iter(), |x, y| x.1.cmp(y.1))
    }
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
    pub fn is_deliverable(&mut self, other: &CbcastClock<T>) -> bool {
        let mut incr_condition: Option<T> = None;

        let mut s_vec = self.into_vec();
        let mut o_vec = other.into_vec();
        s_vec.sort_by(|a, b| a.1.cmp(&b.1));
        o_vec.sort_by(|a, b| a.1.cmp(&b.1));

        // check self vector clock
        for (k, v) in s_vec {
            if (k) == self.id {
                if let Some(index) = o_vec.iter().position(|v| v.0 == k) {
                    o_vec.remove(index);
                }
                continue;
            }
            let other_val = *other.vc.get(&k).unwrap_or(&0);
            if other_val == v + 1 && incr_condition.is_none() {
                incr_condition = Some(k);
            } else if other_val > v {
                return false;
            }
            if let Some(index) = o_vec.iter().position(|v| v.0 == k) {
                o_vec.remove(index);
            }
        }

        let mut ve = Vec::new();

        // We now check the other vector to see if there are new keys
        // for (k, v) in other.vc.iter() {
        for (k, v) in o_vec.iter() {
            if *v <= 1 && incr_condition.is_none() {
                incr_condition = Some(*k);
                ve.push((*k, *v));
            } else {
                return false;
            }
        }
        for (k, v) in &ve {
            self.vc.insert(*k, *v);
        }
        // if let Some(index) = incr_condition {
        //     self.increment(index);
        // }
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

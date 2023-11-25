use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Serialize, Deserialize, Eq)]
pub struct CbcastClock<T: Eq + Hash + Display> {
    vc: HashMap<T, u32>,
    id: T,
}
impl<T: Eq + Hash + Display> PartialEq for CbcastClock<T> {
    fn eq(&self, other: &Self) -> bool {
        let other_len = other.vc.len();
        let mut cur_len = 0;
        for (k, v) in self.vc.iter() {
            if let Some(val) = other.vc.get(k) {
                cur_len += 1;
                if v != val {
                    return false;
                }
            }
        }
        if cur_len == other_len {
            true
        } else {
            false
        }
    }
}

// TODO: fix this
impl<T: Eq + Hash + Display> PartialOrd for CbcastClock<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut ordering: Ordering = Ordering::Equal;
        let other_len = other.vc.len();
        let mut cur_len = 0;
        for (k, v) in self.vc.iter() {
            if let Some(val) = other.vc.get(k) {
                cur_len += 1;
                if val > v {
                    ordering = Ordering::Greater;
                    // return Some(std::cmp::Ordering::Greater);
                }
                if val < v && ordering != Ordering::Greater {
                    ordering = Ordering::Less;
                }
            }
        }
        if cur_len == other_len {
            return Some(ordering);
        }
        return Some(Ordering::Less)
    }
}

impl<T: Eq + Hash + Display> Ord for CbcastClock<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(ord) = self.partial_cmp(other) {
            return ord;
        } else {
            Ordering::Equal
        }
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

    pub fn from_map(id: T, hmap: HashMap<T, u32>) -> Self {
        CbcastClock { vc: hmap, id }
    }

    pub fn increment(&mut self) -> Option<u32> {
        let k = self.id;
        let val = *self.vc.get_mut(&k).unwrap_or(&mut 0);
        self.vc.insert(k, val + 1)
    }

    pub fn insert(&mut self, k: T, v: u32) -> Option<u32> {
        self.vc.insert(k, v)
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

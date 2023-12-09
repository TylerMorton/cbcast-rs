use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::hash::Hash;

#[derive(Serialize, Deserialize)]
pub struct CbcastMessage<I: Serialize + Display + Eq + Hash, D> {
    pub sender_id: I,
    pub cc: Vec<(I, u32)>,
    pub data: D,
}

impl<I: Serialize + Display + Eq + Hash, D> CbcastMessage<I, D> {
    pub fn new(sender_id: I, cc: Vec<(I, u32)>, data: D) -> Self {
        CbcastMessage {
            sender_id,
            cc,
            data,
        }
    }
}

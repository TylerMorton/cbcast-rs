use crate::cbcast::*;
use cbmessage::CbcastMessage;

#[test]
pub fn message_init() {
    let id = 3;
    let cc: Vec<(i32, u32)> = Vec::new();
    let m1 = CbcastMessage::new(id, cc, "message");
    assert!(m1.sender_id == id);
    assert!(m1.cc == Vec::new());
    assert!(m1.data == "message");
}

#[test]
pub fn serde() {
    let id = 3;
    let cc: Vec<(i32, u32)> = Vec::new();
    // let m1 = CbcastMessage::new(id, cc, "message");
    let m1 = CbcastMessage::new(id, cc, 4);
    let message = serde_json::to_string(&m1).expect("failed");
    let m2: CbcastMessage<i32, i32> = serde_json::from_str(&message).unwrap();
    assert!(m1.sender_id == m2.sender_id);
    assert!(m1.cc == m2.cc);
    assert!(m1.data == m2.data);
}

extern crate mp_stomp;

use mp_stomp::message::*;


fn main() {
    let message = Message {
            message_type: "SEND".to_string(),
            headers: vec!["A".to_string(), "B".to_string()],
            payload: "prova".to_string(),
        };
    println!("Hello {:?}", message);
}
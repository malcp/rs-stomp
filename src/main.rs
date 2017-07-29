extern crate mp_stomp;

use mp_stomp::message::*;


fn main() {
    let message = Message::new();
    
    println!("Hello {:?}", message);
}

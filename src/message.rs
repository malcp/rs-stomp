use std::fmt::Write;

use header::HeaderItems;

#[derive(Debug)]
pub struct Message {
    pub message_type: String,
    pub headers: HeaderItems,
    pub payload: String,
}

pub trait StompMessage {
    fn new() -> Message;
    fn set_message_type(&self, message_type: &'static str) -> Message;
    fn get_message_type(&self) -> String;
    fn as_string(&self) -> String;
}

impl StompMessage for Message {
    fn new() -> Message {
        Message {
            message_type: "<NOT DEFINED".to_string(),
            headers: HeaderItems { headers: vec!["A".to_string(), "B".to_string()] },
            payload: "lorem ipsum".to_string(),
        }
    }

    fn set_message_type(&self, message_type: &'static str) -> Message {
        Message {
            message_type: message_type.to_string(),
            headers: self.headers.clone(),
            payload: "lorem ipsum".to_string(),
        }
    }

    fn get_message_type(&self) -> String {
        self.message_type.to_string()
    }

    fn as_string(&self) -> String {
        let mut message = String::new();
        writeln!(&mut message, "{message_type}", message_type = self.message_type);
        writeln!(&mut message, "{}", "accept-version:1.2");
        writeln!(&mut message, "{}", "");
        writeln!(&mut message, "{body}\0", body = "");
        message
    }
}

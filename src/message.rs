#[derive(Debug)]
pub struct Message {
    pub message_type: String,
    pub headers: Vec<String>,
    pub payload: String,
}

pub trait StompMessage {
    //fn set_message_type(&self, String);
    fn get_message_type(&self) -> String;
}

impl StompMessage for Message {
    //fn set_message_type(&self, type) {
        
    //}
    
    fn get_message_type(&self) -> String {
        self.message_type.to_string()
    }
}
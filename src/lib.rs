pub mod message;

#[cfg(test)]
mod tests {
    use super::message::Message;

    #[test]
    fn can_create_message() {
        let message = Message {
            message_type: "SEND".to_string(),
            headers: vec!["A".to_string(), "B".to_string()],
            payload: "prova".to_string(),
        };

        assert!("SEND" == message.message_type);
    }
}

pub mod message;
mod header;
pub mod client;

#[cfg(test)]
mod tests {
    use super::message::*;
    use super::client::*;

    #[test]
    fn can_create_message() {

        let message = Message::new();
        let message = message.set_message_type("SEND");

        assert!("SEND" == message.message_type);
    }

    #[test]
    fn can_connect() {
        let connection = ConnectionDown::new();
        let mut connection = connection
            .connect("localhost:61613")
            .expect("Unnable to connect :(");
        connection.connect();
        connection.shutdown().expect("Unnable to close connection");
    }
}

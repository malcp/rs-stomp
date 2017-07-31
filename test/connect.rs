mod rabbitmq {
    pub const USER: &'static str = "rabbitmq";
    pub const PASSWORD: &'static str = "rabbitmq";
}

#[cfg(test)]
mod tests {
    use mp_stomp::frames::Frame;
    use mp_stomp::client::ConnectionDown;
    use super::rabbitmq;

    #[test]
    fn can_create_frame() {
        let frame = Frame::new_connect(rabbitmq::USER, rabbitmq::PASSWORD);
        assert!("CONNECT" == frame.get_frame_type());
    }

    #[test]
    fn can_connect() {
        let connection = ConnectionDown::new();
        let mut connection = connection
            .connect("localhost:61613")
            .expect("Unnable to connect :(");
        connection.connect(rabbitmq::USER, rabbitmq::PASSWORD);
        connection.shutdown().expect("Unnable to close connection");
    }
}

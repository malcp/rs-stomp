#[cfg(test)]
mod tests {
    use mp_stomp::frames::Frame;
    use mp_stomp::client::ConnectionDown;

    #[test]
    fn can_create_frame() {
        let frame = Frame::new_connect();
        assert!("CONNECT" == frame.get_frame_type());
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

#[cfg(test)]
mod tests {
    use mp_stomp::frames::FrmConnect;

    #[test]
    fn can_create_frame() {
        let frame = FrmConnect::new();
        assert!("SEND" == frame.frame_type);
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

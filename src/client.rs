use std::net::TcpStream;
use std::io::Error;
use std::net::Shutdown;
use std::io::Write;
use std::io::Read;
use std::ascii::AsciiExt;

use message::Message;
use message::StompMessage;

#[derive(Debug)]
pub struct ConnectionDown {
}

impl ConnectionDown {
    #[inline]
    pub fn new() -> ConnectionDown {
        ConnectionDown {}
    }

    pub fn connect(self, addr: &'static str) -> Result<ConnectionUp, (ConnectionDown, Error)> {
        match TcpStream::connect(addr) {
            Ok(s) => Ok(ConnectionUp { tcp_stream: s, buffer: vec!() }),
            Err(e) => Err((self, e))
        }
    }
}

#[derive(Debug)]
pub struct ConnectionUp {
    tcp_stream: TcpStream,
    buffer: Vec<u8>
}

impl ConnectionUp {
    pub fn shutdown(self) -> Result<ConnectionDown, Error> {
        match self.tcp_stream.shutdown(Shutdown::Both) {
            Ok(s) => Ok(ConnectionDown::new()),
            Err(e) => Err(e)
        }
    }

    fn write_message(&mut self, message: Message) {
        self.tcp_stream.write(message.as_string().as_bytes()).unwrap();
        print!("{:?}", message.as_string());
        self.tcp_stream.flush().unwrap();
    }

    fn read_message(&mut self) -> Vec<u8> {
        let mut inc_message = [0;10000];
        let num_bytes = self.tcp_stream.read(&mut inc_message).unwrap();
        for s in inc_message.into_iter() {
            print!("{}", s.to_owned() as char);
        }

        inc_message.to_vec()
    }

    pub fn connect(&mut self) {
        let message = Message::new();
        let message = message.set_message_type("CONNECT");
        self.write_message(message);
        self.buffer = self.read_message();
    }
}

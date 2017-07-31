use std::net::{TcpStream, Shutdown};
use std::io::{Error, Read, Write};

use frames::Frame;

#[derive(Debug)]
pub struct ConnectionDown {}

impl ConnectionDown {
    #[inline]
    pub fn new() -> ConnectionDown {
        ConnectionDown {}
    }

    pub fn connect(self, addr: &'static str) -> Result<ConnectionUp, (ConnectionDown, Error)> {
        match TcpStream::connect(addr) {
            Ok(s) => {
                Ok(ConnectionUp {
                       tcp_stream: s,
                       buffer: Vec::new(),
                   })
            }
            Err(e) => Err((self, e)),
        }
    }
}

#[derive(Debug)]
pub struct ConnectionUp {
    tcp_stream: TcpStream,
    buffer: Vec<u8>,
}

impl ConnectionUp {
    pub fn shutdown(self) -> Result<ConnectionDown, Error> {
        match self.tcp_stream.shutdown(Shutdown::Both) {
            Ok(_) => Ok(ConnectionDown::new()),
            Err(e) => Err(e),
        }
    }

    fn write_frame(&mut self, frame: Frame) {
        self.tcp_stream.write(frame.as_string().as_bytes()).unwrap();
        print!("{:?}", frame.as_string());
        self.tcp_stream.flush().unwrap();
    }

    fn read_frame(&mut self) -> Vec<u8> {
        let mut inc_frame = [0; 10000];
        let _ = self.tcp_stream.read(&mut inc_frame).unwrap();
        for s in inc_frame.into_iter() {
            print!("{}", s.to_owned() as char);
        }

        inc_frame.to_vec()
    }

    pub fn connect(&mut self, user: &'static str, passwd: &'static str) {
        let frame = Frame::new_connect(user, passwd);
        self.write_frame(frame);
        self.buffer = self.read_frame();
    }
}

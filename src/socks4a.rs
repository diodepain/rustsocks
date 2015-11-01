use util::io_err;
use std::io::{Result, OtherIoError};
use std::net::{TcpStream, ConnectionFailed, ConnectionRefused};

pub struct Socks4a<'a> {
    socks_host: &'a str,
    socks_port: u16,
}

impl<'a> Socks4a<'a> {
    pub fn new(host: &'a str, port: u16) -> Socks4a {
        Socks4a { socks_host: host, socks_port: port }
    }

    pub fn connect(&mut self, host: &str, port: u16) -> IoResult<TcpStream> {
        let mut stream = try!(TcpStream::connect(self.socks_host, self.socks_port));
        try!(stream.write([0x04, 0x01]));
        try!(stream.write_be_u16(port));
        try!(stream.write([0x00, 0x00, 0x00, 0x01]));
        try!(stream.write([0x00]));
        try!(stream.write_str(host));
        try!(stream.write([0x00]));

        // read null byte
        if 0 != try!(stream.read_u8()) {
            return io_err(OtherIoError, "Expected null byte not found");
        }

        // read status
        match try!(stream.read_u8()) {
            // request granted
            0x5a => {
                let _port = try!(stream.read_be_u16());
                let _ip = try!(stream.read_be_u32());

                Ok(stream)
            }
            // request rejected or failed
            0x5b => io_err(ConnectionRefused, "Request rejected or failed"),
            // request failed because client is not running identd (or unreachable)
            0x5c => io_err(ConnectionFailed, "Client unreachable"),
            // request failed because client's identd could not confirm the user ID
            // string in the request
            0x5d => io_err(ConnectionRefused, "Unknown user"),
            x    => io_err(OtherIoError, format!("Unexpected status byte: {}", x)) 
        }
    }
}

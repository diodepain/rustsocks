use util::io_err;
use std::io::{IoResult, TcpStream, ConnectionRefused, ConnectionFailed,
                OtherIoError, InvalidInput};
use std::io::net::ip::{ IpAddr, Ipv4Addr };

pub struct Socks4<'a> {
    socks_host: &'a str,
    socks_port: u16,
}

impl<'a> Socks4<'a> {
    pub fn new(host: &'a str, port: u16) -> Socks4 {
        Socks4 { socks_host: host, socks_port: port }
    }

    pub fn connect(&mut self, host: IpAddr, port: u16) -> IoResult<TcpStream> {
        let mut stream = try!(TcpStream::connect(self.socks_host, self.socks_port));
        try!(stream.write([0x04, 0x01]));
        try!(stream.write_be_u16(port));
        match host {
            Ipv4Addr(oct1, oct2, oct3, oct4) => {
                try!(stream.write([oct1, oct2, oct3, oct4]));
            },
            _ => { return io_err(InvalidInput, "Must be IPv4 address"); }
        };
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
            x => { return io_err(OtherIoError, format!("Unexpected status byte: {}", x)); }
        }
    }
}

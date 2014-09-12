use std::io::{IoResult, IoError, IoErrorKind, TcpStream, ConnectionRefused,
              ConnectionFailed, OtherIoError};

enum AuthMethod<'s> {
    NoAuth,
    UPass(&'s str, &'s str)
}

pub struct Socks5<'a> {
    socks_host: &'a str,
    socks_port: u16,
    socks_auth: AuthMethod<'a>,
}

impl<'a> Socks5<'a> {
    pub fn new(host: &'a str, port: u16) -> Socks5 {
        Socks5 {
            socks_host: host,
            socks_port: port,
            socks_auth: NoAuth
        }
    }

    pub fn login(&mut self, uname: &'a str, passwd: &'a str) {
        self.socks_auth = UPass(uname, passwd);
    }

    pub fn connect(&mut self, host: &str, port: u16) -> IoResult<TcpStream> {
        let mut stream = try!(TcpStream::connect(self.socks_host, self.socks_port));
        try!(stream.write([0x05u8]));
        match self.socks_auth {
            NoAuth => { try!(stream.write([0x01u8, 0x00])); },
            UPass(uname, passwd) => { try!(stream.write([0x01u8, 0x02])); }
        }

        if try!(stream.read_u8()) != 0x05 {
            return io_err(OtherIoError, "Unexpected SOCKS version number");
        }

        match try!(stream.read_u8()) {
            0x00 => {
                match self.socks_auth {
                    NoAuth => { /* Continue */ },
                    _ => return io_err(OtherIoError,
                        "Wrong authentication method from server")
                }
            }
            0x02 => {
                match self.socks_auth {
                    UPass(uname, passwd) => {
                        try!(stream.write([0x01u8, uname.len() as u8]));
                        try!(stream.write_str(uname));
                        try!(stream.write([passwd.len() as u8]));
                        try!(stream.write_str(passwd));

                        if try!(stream.read_u8()) != 0x01 {
                            return io_err(OtherIoError,
                                "Invalid authentication version");
                        }

                        if try!(stream.read_u8()) != 0x00 {
                            return io_err(ConnectionRefused, "Authentication failed");
                        }
                    }
                    _ => { return io_err(OtherIoError,
                            "Wrong authentication method from server");
                    }
                }
            }
            0xFF => { return io_err(ConnectionRefused,
                      "Server refused authentication methods"); }
            _ => { return io_err(OtherIoError,
                    "Wrong authentication method from server"); }
        }

        try!(stream.write([0x05u8, 0x01, 0x00, 0x03, host.len() as u8]));
        try!(stream.write_str(host));
        try!(stream.write_be_u16(port));

        if try!(stream.read_u8()) != 0x05 {
            return io_err(OtherIoError, "Invalid SOCKS version number");
        }

        match try!(stream.read_u8()) {
            0x00 => {
                let _null = try!(stream.read_u8());
                let addrtype = try!(stream.read_u8());
                if addrtype != 0x03 {
                    return io_err(OtherIoError, "Unimplemented");
                }
                let _addrlen = try!(stream.read_u8());
                let mut _addr: Vec<u8> = vec![];
                for i in range(0, _addrlen) {
                    _addr.push(try!(stream.read_u8()));
                }
                let _port = try!(stream.read_be_u16());
                Ok(stream)
            }
            0x01 => io_err(OtherIoError, "General failure"),
            0x02 => io_err(OtherIoError, "Connection not allowed by ruleset"),
            0x03 => io_err(OtherIoError, "Network unreachable"),
            0x04 => io_err(OtherIoError, "Host unreachable"),
            0x05 => io_err(OtherIoError, "Connection refused by destination"),
            0x06 => io_err(OtherIoError, "TTL expired"),
            0x07 => io_err(OtherIoError, "Protocol Error"),
            0x08 => io_err(OtherIoError, "Address type not supported"),
            _ => io_err(OtherIoError, "Unknown error")
        }
    }
}

fn io_err<T>(kind: IoErrorKind, desc: &'static str) -> IoResult<T> {
    Err(IoError { kind: kind, desc: desc, detail: None })
}


use util::io_err;
use std::io::{Write, Read, ErrorKind};
use std::io::Result as IoResult;
use std::net::{IpAddr, TcpStream};
use std::mem;

enum AuthMethod<'s> {
    NoAuth,
    UPass(&'s str, &'s str)
}

pub trait ReadU8 {
    fn read_u8(&mut self) -> IoResult<u8>;
}

impl ReadU8 for TcpStream {
    #[inline]
    fn read_u8(&mut self) -> IoResult<u8> {
        let mut buf = [0u8];
        try!(self.read_exact(&mut buf));
        Ok(buf[0])
    }
}

pub trait Socks5Destination {
    fn write_destination(&self, &mut TcpStream) -> IoResult<()>;
}

impl<'a> Socks5Destination for &'a str {
    fn write_destination(&self, stream: &mut TcpStream) -> IoResult<()> {
        if self.len() > 255 {
            return io_err(ErrorKind::Other, "Domain length is too long");
        }

        try!(stream.write(&[0x03, self.len() as u8]));
        try!(stream.write(&self.as_bytes()));
        Ok(())
    }
}

impl<'a> Socks5Destination for IpAddr {
    fn write_destination(&self, stream: &mut TcpStream) -> IoResult<()> {
        match *self {
            IpAddr::V4(addr) => {
                try!(stream.write_all(&[0x01]));
                try!(stream.write_all(&addr.octets()));
            },
            IpAddr::V6(addr) => {
                let segments = addr.segments();
                try!(stream.write_all(&[0x04]));
                for &pair in segments.iter() {
                    let array : [u8; 2] = unsafe { 
                        mem::transmute(pair.to_be()) 
                    };
                    try!(stream.write_all(&array));
                }
            }
        }

        Ok(())
    }
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
            socks_auth: AuthMethod::NoAuth
        }
    }

    pub fn login(&mut self, uname: &'a str, passwd: &'a str) {
        self.socks_auth = AuthMethod::UPass(uname, passwd);
    }

    pub fn connect<T: Socks5Destination>(&mut self, destination: T, port: u16)
                                                        -> IoResult<TcpStream> {
        let mut stream = try!(TcpStream::connect((self.socks_host, self.socks_port)));
        try!(stream.write(&[0x05u8]));
        match self.socks_auth {
            AuthMethod::NoAuth => { try!(stream.write(&[0x01u8, 0x00])); },
            AuthMethod::UPass(..) => { try!(stream.write(&[0x01u8, 0x02])); }
        }

        if try!(stream.read_u8()) != 0x05 {
            return io_err(ErrorKind::Other, "Unexpected SOCKS version number");
        }

        match try!(stream.read_u8()) {
            0x00 => {
                match self.socks_auth {
                    AuthMethod::NoAuth => { /* Continue */ },
                    _ => return io_err(ErrorKind::Other,
                        "Wrong authentication method from server")
                }
            }
            0x02 => {
                match self.socks_auth {
                    AuthMethod::UPass(uname, passwd) => {
                        try!(stream.write_all(&[0x01u8, uname.len() as u8]));
                        //try!(stream.write_str(uname));
                        try!(write!(stream, "{}", uname));
                        try!(stream.write_all(&[passwd.len() as u8]));
                        try!(write!(stream, "{}", passwd));
                        //try!(stream.write_str(passwd));

                        if try!(stream.read_u8()) != 0x01 {
                            return io_err(ErrorKind::Other,
                                "Invalid authentication version");
                        }

                        if try!(stream.read_u8()) != 0x00 {
                            return io_err(ErrorKind::ConnectionRefused, "Authentication failed");
                        }
                    }
                    _ => { return io_err(ErrorKind::Other,
                            "Wrong authentication method from server");
                    }
                }
            }
            0xFF => { return io_err(ErrorKind::ConnectionRefused,
                      "Server refused authentication methods"); }
            _ => { return io_err(ErrorKind::Other,
                    "Wrong authentication method from server"); }
        }

        try!(stream.write(&[0x05u8, 0x01, 0x00]));
        try!(destination.write_destination(&mut stream));
        let be_port = port.to_be();
        let array : [u8; 2] = unsafe { mem::transmute(be_port) };
        try!(stream.write_all(&array));

        if try!(stream.read_u8()) != 0x05 {
            return io_err(ErrorKind::Other, "Invalid SOCKS version number");
        }

        match try!(stream.read_u8()) {
            0x00 => {
                let _null = try!(stream.read_u8());

                match try!(stream.read_u8()) {
                    0x01 => {
                        let mut _ipv4 = [0; 4];
                        try!(stream.read_exact(&mut _ipv4));
                    }
                    0x03 => {
                        let addrlen = try!(stream.read_u8());
                        let mut _domain = Vec::with_capacity(addrlen as usize);
                        try!(stream.read_exact(&mut _domain));
                    }
                    0x04 => {
                        let mut _ipv6 = [0; 16];
                        try!(stream.read_exact(&mut _ipv6));
                    }
                    _ => return io_err(ErrorKind::Other, "Invalid address type"),
                }

                let mut _port_slice : [u8; 2] = [0u8; 2];
                try!(stream.read_exact(&mut _port_slice));
                let _port_be : u16 = unsafe { mem::transmute(_port_slice) };
                let _port = u16::from_be(_port_be);
                Ok(stream)
            }
            0x01 => io_err(ErrorKind::Other, "General failure"),
            0x02 => io_err(ErrorKind::Other, "Connection not allowed by ruleset"),
            0x03 => io_err(ErrorKind::NotConnected, "Network unreachable"),
            0x04 => io_err(ErrorKind::Other, "Host unreachable"),
            0x05 => io_err(ErrorKind::ConnectionRefused, "Connection refused by destination"),
            0x06 => io_err(ErrorKind::Other, "TTL expired"),
            0x07 => io_err(ErrorKind::Other, "Protocol Error"),
            0x08 => io_err(ErrorKind::Other, "Address type not supported"),
            _ => io_err(ErrorKind::Other, "Unknown error")
        }
    }
}

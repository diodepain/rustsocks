use std::io::{IoResult, TcpStream};

pub struct Socks4a {
	sockshost: String,
	socksport: u16,
	host: String,
	port: u16
}

impl Socks4a {
	pub fn new(host: String, port: u16) -> Socks4a {
		let tmphost = "";
		let tmpport = 0;
		Socks4a { sockshost: host, socksport: port, host: tmphost.to_string(), 
			port: tmpport }
	}
	pub fn connect(&mut self, host: String, port: u16) {
		self.host = host;
		self.port = port;
	}
	pub fn build(&mut self) -> IoResult<TcpStream> {
		let mut stream = try!(TcpStream::connect(self.sockshost.as_slice(),
																						 self.socksport));
		try!(stream.write([0x04, 0x01]));
		try!(stream.write_be_u16(self.port));
		try!(stream.write([0x00, 0x00, 0x00, 0x01]));
		try!(stream.write([0x00]));
		try!(stream.write_str(self.host.as_slice()));
		try!(stream.write([0x00]));

		Ok(stream)
	}
}

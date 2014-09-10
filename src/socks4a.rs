use std::io::TcpStream;

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
	pub fn build(&mut self) -> TcpStream {
		let mut stream = TcpStream::connect(self.sockshost.as_slice(), 
			self.socksport).unwrap();
		stream.write([0x04u8, 0x01u8]);
		stream.write_be_u16(self.port);
		stream.write([0x00u8, 0x00u8, 0x00u8, 0x01u8]);
		stream.write([0x00u8]);
		stream.write_str(self.host.as_slice());
		stream.write([0x00u8]);
		stream
	}
}

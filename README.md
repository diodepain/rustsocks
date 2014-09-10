rustsocks
=========

A library for interacting with SOCKS proxies in Rust.

It currently only supports Socks4a, but support for Socks5 and plain Socks4 is should be added soon.

Example
-------
    extern crate rustsocks;
    
    use std::io::TcpStream;
    use rustsocks::socks4a::Socks4a;
    
    fn main() {
      let mut rs = Socks4a::new("127.0.0.1".to_string(), 9050);
      rs.connect("example.com", 80);
      let mut stream: TcpStream = rs.build();
      // Use stream like any other TcpStream.
    }

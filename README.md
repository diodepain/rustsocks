rustsocks
=========

A library for interacting with SOCKS proxies in Rust.

Supports Socks4 and Socks4a.  Socks5 coming soon.

Example
-------
    extern crate rustsocks;

    use std::io::TcpStream;
    use rustsocks::socks4a::Socks4a;

    fn main() {
      // Use the SOCKS proxy at 127.0.0.1 on port 9050
      let mut rs = Socks4a::new("127.0.0.1", 9050);

      // Connect through the socks proxy to example.com on port 80
      let mut stream = rs.connect("example.com", 80).unwrap();

      // Use stream like any other TcpStream.
    }

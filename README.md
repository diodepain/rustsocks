rustsocks
=========

A library for interacting with SOCKS proxies in Rust.

It currently only supports Socks4a, but support for Socks5 and plain Socks4 should be added soon.

Example
-------
    extern crate rustsocks;

    use std::io::TcpStream;
    use rustsocks::socks4a::Socks4a;

    fn main() {
      // Use the SOCKS proxy at 127.0.0.1 on port 9050
      let mut rs = Socks4a::new("127.0.0.1".to_string(), 9050);

      // Connect through the socks proxy to example.com on port 80
      rs.connect("example.com", 80);

      // Build the TcpStream and automatically set up the SOCKS proxy
      let mut stream: TcpStream = rs.build().unwrap();

      // Use stream like any other TcpStream.
    }

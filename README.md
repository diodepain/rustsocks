rustsocks
=========

A library for interacting with SOCKS proxies in Rust.

Supports Socks4, Socks4a, and Socks5.

Example
-------
Socks4:

    extern crate rustsocks;

    use rustsocks::socks4::Socks4;
    use std::io::net::ip::Ipv4Addr;

    fn main() {
        // Use the Socks proxy at 127.0.0.1 on port 9050
        let mut rs = Socks4::new("127.0.0.1", 9050);

        // Connect through the Socks proxy to 173.194.43.3 on port 80
        let mut stream = rs.connect(Ipv4Addr(173, 194, 43, 3), 80);

        // Use stream like any other TcpStream
    }

Socks4a:

    extern crate rustsocks;

    use rustsocks::socks4a::Socks4a;

    fn main() {
      // Use the Socks proxy at 127.0.0.1 on port 9050
      let mut rs = Socks4a::new("127.0.0.1", 9050);

      // Connect through the Socks proxy to example.com on port 80
      let mut stream = rs.connect("example.com", 80).unwrap();

      // Use stream like any other TcpStream.
    }

Socks5:

    extern crate rustsocks;

    use rustsocks::socks5::Socks5;

    fn main() {
        // Use the Socks proxy at 127.0.0.1 on port 9050
        let mut rs = Socks5::new("127.0.0.1", 9050);

        // To authenticate (if needed, authentication is not necessary)
        rs.login("username", "password");

        // Connect through the Socks proxy to example.com on port 80
        let mut stream = rs.connect("example.com", 80).unwrap();

        // Use stream like any other TcpStream
    }

The tests assume a SOCKS4/4a/5 server is running at 127.0.0.1:9050.

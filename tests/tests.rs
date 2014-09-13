extern crate rustsocks;

use rustsocks::{Socks4, Socks4a, Socks5};
use std::io::net::ip::IpAddr;

static SOCKS_HOST : &'static str = "127.0.0.1";
static SOCKS_PORT : u16 = 9150;
static GET_REQUEST : &'static str =
      "GET /404 HTTP/1.1\nHost: www.google.com\nConnection: close\n\n";

#[test]
fn socks4a() {
    let mut socks = Socks4a::new(SOCKS_HOST, SOCKS_PORT);
    let mut stream = socks.connect("www.google.com", 80);

    let _ = stream.write_str(GET_REQUEST);
    println!("{}", stream.read_to_string().unwrap());
}

#[test]
fn socks4() {
    let mut socks = Socks4::new(SOCKS_HOST, SOCKS_PORT);
    let addr = from_str::<IpAddr>("74.125.230.65").unwrap();
    let mut stream = socks.connect(addr, 80);

    let _ = stream.write_str(GET_REQUEST);
    println!("{}", stream.read_to_string().unwrap());
}

#[test]
fn socks5_domain() {
    let mut socks = Socks5::new(SOCKS_HOST, SOCKS_PORT);
    let mut stream = socks.connect("www.google.com", 80);

    let _ = stream.write_str(GET_REQUEST);
    println!("{}", stream.read_to_string().unwrap());
}

#[test]
fn socks5_ipv4() {
    let mut socks = Socks5::new(SOCKS_HOST, SOCKS_PORT);
    let addr = from_str::<IpAddr>("74.125.230.65").unwrap();
    let mut stream = socks.connect(addr, 80);

    let _ = stream.write_str(GET_REQUEST);
    println!("{}", stream.read_to_string().unwrap());
}

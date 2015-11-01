#![crate_name = "rustsocks"]
#![crate_type = "rlib"]
#![feature(slice_patterns)]
#![feature(ip_addr)]
#![feature(read_exact)]

//pub use socks4a::Socks4a;
//pub use socks4::Socks4;
pub use socks5::Socks5;

//pub mod socks4a;
//pub mod socks4;
pub mod socks5;

mod util {
  use std::io::{Result, Error, ErrorKind};

  pub fn io_err<T>(kind: ErrorKind, desc: &'static str) -> Result<T> {
      Err(Error::new(kind, desc))
  }
}

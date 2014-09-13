#![crate_name = "rustsocks"]
#![crate_type = "rlib"]

pub use socks4a::Socks4a;
pub use socks4::Socks4;
pub use socks5::Socks5;

pub mod socks4a;
pub mod socks4;
pub mod socks5;

mod util {
  use std::io::{IoResult, IoError, IoErrorKind};

  pub fn io_err<T>(kind: IoErrorKind, desc: &'static str) -> IoResult<T> {
      Err(IoError { kind: kind, desc: desc, detail: None })
  }
}

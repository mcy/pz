//! Runtime support for Rust gencode.

mod arena;
mod tdp;

pub mod ptr;
pub mod str;

pub use self::ptr::Mut;
pub use self::ptr::View;
pub use self::str::Str;
pub use self::str::StrBuf;

#[doc(hidden)]
pub mod __z {
  pub mod tdp {
    pub use super::super::tdp::*;
  }
  pub use super::arena::*;
}

pub use tdp::Error;

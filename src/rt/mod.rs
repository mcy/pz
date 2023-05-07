//! Runtime support for Rust gencode.

mod arena;
mod debug;
mod tdp;

pub mod ptr;
pub mod str;
pub mod value;

pub use self::ptr::Mut;
pub use self::ptr::View;
pub use self::str::Str;
pub use self::str::StrBuf;
pub use self::value::Repeated;
pub use self::value::Slice;
pub use self::value::SliceMut;

#[doc(hidden)]
pub mod __z {
  pub mod tdp {
    pub use super::super::tdp::*;
  }
  pub use super::arena::*;
  pub use super::debug::Debug;
}

pub use tdp::Error;

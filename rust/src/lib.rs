//! Runtime support for Rust gencode.

mod arena;
mod debug;
mod tdp;

pub mod ptr;
pub mod rep;
pub mod str;
pub mod value;

pub mod proto;

pub use self::ptr::Mut;
pub use self::ptr::View;
pub use self::rep::Repeated;
pub use self::rep::Slice;
pub use self::rep::SliceMut;
pub use self::str::Str;
pub use self::str::StrBuf;

#[doc(hidden)]
pub mod __z {
  pub mod tdp {
    pub use super::super::tdp::*;
  }
  pub use super::arena::*;
  pub use super::debug::Debug;
}

pub use tdp::Error;

//! `pz` Runtime
//!
//! This crate provides runtime support for generated Rust code from `.pz`
//! files.

mod arena;
mod debug;
mod tdp;

pub mod ptr;
#[doc(hidden)]
pub mod rep;
#[doc(hidden)]
pub mod str;
#[doc(hidden)]
pub mod value;

pub mod proto;

pub use crate::ptr::Mut;
pub use crate::ptr::Proxied;
pub use crate::ptr::View;

pub use crate::rep::Repeated;
pub use crate::rep::Slice;
pub use crate::rep::SliceMut;

pub use crate::str::Str;
pub use crate::str::StrBuf;

pub use crate::value::OptMut;
pub use crate::value::Type;

pub use crate::tdp::parse::Error;

#[doc(hidden)]
pub mod __z {
  pub use std;

  pub mod tdp {
    pub use crate::tdp::*;
  }

  pub use crate::arena::*;
  pub use crate::debug::Debug;
  pub use crate::str::private::Storage as RawStr;
}

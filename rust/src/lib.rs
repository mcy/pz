//! `pz` Runtime
//!
//! This crate provides runtime support for generated Rust code from `.pz`
//! files.

mod arena;
mod debug;

mod tdp;
pub use crate::tdp::parse::Error;

pub mod reflect;
pub use crate::reflect::Mut;
pub use crate::reflect::Ref;
pub use crate::reflect::Type;
pub use crate::reflect::Message;

mod rep;
pub use crate::rep::Repeated;
pub use crate::rep::Slice;
pub use crate::rep::SliceMut;

mod str;
pub use crate::str::String;
pub use crate::str::Str;
pub use crate::str::StrBuf;

mod opt;
pub use crate::opt::OptMut;

mod scalar;
pub use crate::scalar::ScalarMut;

mod macros;

pub mod proto;

pub extern crate prost;

/// A serialization format for a [`Message`].
pub enum Codec {
  /// The Protobuf wire format, with pz's own twists.
  Protobuf,
  // TODO: Json, Textproto,
}

#[doc(hidden)]
pub mod __z {
  pub use std;

  pub enum Void {}

  pub mod tdp {
    pub use crate::tdp::*;
  }

  pub mod macros {
    pub use crate::macros::*;
  }

  pub use crate::arena::*;
  pub use crate::debug::Debug;
  pub use crate::str::private::Storage as RawStr;
  
  pub use crate::reflect::private::*;
  pub use crate::reflect::names::*;

  pub use crate::seal::*;
}

pub(crate) mod seal {
  pub struct Seal;
  pub trait Sealed {}
  impl Sealed for Seal {}
}

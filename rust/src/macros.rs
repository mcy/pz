//! This file defines the `proto!` macro.
//!
//! The way the macro itself works is it generates a complicated type that
//! records all of the keys and values pushed into it, resulting in something
//! that can be `.into()`'d into the appropriate proto type.

use crate::reflect::Field;
use crate::reflect::MutView;
use crate::reflect::Name;
use crate::reflect::Rep;
use crate::reflect::Selector;
use crate::reflect::Set;
use crate::Mut;
use crate::Type;

/// The no-op command. This is the root of our settings command chain.
pub struct Nop;
impl<M: Type> Set<M> for Nop {
  #[inline(always)]
  fn apply_to(self, _: Mut<M>) {}
}

/// A key-value command. This sets a single singular or repeated value.
pub struct Kv<Prev, Name, Value>(pub Prev, pub Name, pub Value);
impl<M, P, N, V> Set<M> for Kv<P, N, V>
where
  M: Field<N>,
  N: Name,
  V: Set<M::Type>,
  P: Set<M>,
{
  #[inline(always)]
  fn apply_to(self, mut m: Mut<M>) {
    self.0.apply_to(m.as_mut());
    self.2.apply_to(<N as Selector<M>>::get_mut(m));
  }
}

/// A reserve command. This reserves space in a repeated field for a followup
/// set of push entries.
pub struct Reserve<Prev, Name>(pub Prev, pub Name, pub usize);
impl<M, P, N, T> Set<M> for Reserve<P, N>
where
  M: Field<N, Type = Rep<T>>,
  N: Name,
  T: Type,
  P: Set<M>,
{
  #[inline(always)]
  fn apply_to(self, mut m: Mut<M>) {
    self.0.apply_to(m.as_mut());
    let mut repeated = <N as Selector<M>>::get_mut(m);
    repeated.reserve(self.2);
  }
}

/// A push command. This pushes a value onto a repeated field.
pub struct Push<Prev, Name, Value>(pub Prev, pub Name, pub Value);
impl<M, P, N, V, T> Set<M> for Push<P, N, V>
where
  M: Field<N, Type = Rep<T>>,
  N: Name,
  V: Set<T>,
  T: Type,
  P: Set<M>,
{
  #[inline(always)]
  fn apply_to(self, mut m: Mut<M>) {
    self.0.apply_to(m.as_mut());
    <N as Selector<M>>::get_mut(m).push(self.2);
  }
}

/*
// This runs into orphan issues around users implementing Field themselves.
// Which they're not supposed to. Ugh.
impl<M: Type, N, V, Next> Set<Opt<M>> for Kv<N, V, Next>
where
  Self: Set<M>,
{
  fn apply_to(self, m: Mut<Opt<M>>) {
    self.apply_to(m.into_inner())
  }
}
*/

/// Constructs a message type using JSON-like syntax.
/// 
/// This macro is intended to be used like this:
/// 
/// ```rust
/// pz::proto! {
///   foo: 42,
///   bar: "my cool string",
///   sub_message: { name: "Solomon" },
///   array: [1, 2, 3, 4],
/// }
/// # ;
/// ```
/// 
/// This expands into an expression that is not actually a message, but rather
/// a description of the setters to call on an unknown message type. This value
/// can be passed into `MyMessage::from()`, pushed onto a repeated field, etc,
/// since it will implement [`Set<MyMessage>`][Set] if `MyMessage` happens to
/// have all of the fields (and subfields!) specified in the macro call.
#[cfg(doc)]
#[macro_export]
macro_rules! proto {
    ($($key:ident: $value:expr),*) => {
      compile_error!("for exposition only")
    };
}

#[cfg(not(doc))]
#[macro_export]
macro_rules! proto {
  (@entry $p:expr, $k:ident: {$($v:tt)*} $(, $($tt:tt)*)?) => {
    $crate::proto!(
      @entry $p, $k: $crate::proto!($($v)*), $($($tt)*)?
    )
  };
  (@entry $p:expr, $k:ident: [$($v:tt)*] $(, $($tt:tt)*)?) => {{
    let _k = $crate::field!($k);
    $crate::proto!(
      @entry $crate::proto!(
        @push $crate::__z::macros::Reserve($p, _k, $crate::proto!(@count $($v)*)),
        _k: $($v)*
      ), $($($tt)*)?
    )
  }};
  (@entry $p:expr, $k:ident: $v:expr $(, $($tt:tt)*)?) => {
    $crate::proto!(
      @entry
      $crate::__z::macros::Kv($p, $crate::field!($k), $v),
      $($($tt)*)?
    )
  };
  (@entry $p:expr,) => { $p };

  (@push $p:expr, $k:ident: {$($v:tt)*} $(, $($tt:tt)*)?) => {
    $crate::proto!(
      @push $p, $k: $crate::proto!($($v)*), $($($tt)*)?
    )
  };
  (@push $p:expr, $k:ident: $v:expr $(, $($tt:tt)*)?) => {
    $crate::proto!(
      @push
      $crate::__z::macros::Push($p, $k, $v),
      $k: $($($tt)*)?
    )
  };
  (@push $p:expr, $k:ident:) => { $p };

  (@count $(,)?) => { 0 };
  (@count , $($tt:tt)*) => {
    1 + $crate::proto!(@count $($tt)*)
  };
  (@count $_:tt $($tt:tt)*) => {
    1 + $crate::proto!(@count $($tt)*)
  };

  ($($tt:tt)*) => {
    $crate::proto!(
      @entry $crate::__z::macros::Nop, $($tt)*
    )
  };
}

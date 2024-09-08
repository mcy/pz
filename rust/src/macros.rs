//! This file defines the `proto!` macro.

use crate::reflect::Field;
use crate::reflect::MutView;
use crate::reflect::Name;
use crate::reflect::Rep;
use crate::reflect::Selector;
use crate::reflect::Set;
use crate::Mut;
use crate::Type;

pub struct Nop;

impl<M: Type> Set<M> for Nop {
  #[inline(always)]
  fn apply_to(self, _: Mut<M>) {}
}

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

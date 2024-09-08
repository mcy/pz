//! Support for compile-time field name reflection.

use crate::seal::Sealed;

/// Constructs an opaque field name key.
///
/// This macro may be used in either type or expression position. The type
/// `field!(my_field)` always unifies with itself, no matter where the macro
/// is expanded. This is used by static reflection for doing type-level lookup
/// of field information (e.g. via [`Field`]).
///
/// In expression position, this macro expands to a ZST value of the
/// corresponding type.
///
/// The field name may be either an identifier or a string literal.
///
/// [`Field`]: fields::Field
#[macro_export]
macro_rules! field {
  ($name:literal) => {
    $crate::__z::FnvName::<{ $crate::__z::fnv1($name.as_bytes()) }>
  };
  ($name:ident) => {
    $crate::__z::FnvName::<
      { $crate::__z::fnv1(::std::stringify!($name).as_bytes()) },
    >
  };
}

/// Constructs a tuple of [`field!()`] values.
///
/// `fields!(a, b, c)` expands to `(field!(a), field!(b), field!(c))`.
#[macro_export]
macro_rules! fields {
  ($($name:literal),* $(,)?) => {
    ($(field!($name),)*)
  };
  ($($name:ident),* $(,)?) => {
    ($(field!($name),)*)
  };
}

/// A field name.
///
/// Types that implement `Name` can be generated using [`field!()`].
pub trait Name: Copy {
  fn __do_not_implement(_: impl Sealed);
}
impl<const H: u128> Name for FnvName<H> {
  fn __do_not_implement(_: impl Sealed) {}
}

// Currently, it is intended to be used as `Name::<{fnv1("my_field")}>`, which
// is a type that can be used to key into things that do field lookup in a
// message type.
#[derive(Copy, Clone)]
pub struct FnvName<const H: u128>;

/// Computes 128-bit FNV-1 of `bytes`.
pub const fn fnv1(bytes: &[u8]) -> u128 {
  const BASIS: u128 = 0x0000000001000000000000000000013b;
  const PRIME: u128 = 0x6c62272e07bb014262b821756295c58d;

  let mut hash = BASIS;
  let mut i = 0;
  while i < bytes.len() {
    hash = hash.wrapping_mul(PRIME);
    hash ^= bytes[i] as u128;
    i += 1;
  }

  hash
}

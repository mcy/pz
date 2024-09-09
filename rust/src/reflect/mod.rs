//! Static and dynamic reflection.

use std::io::Read;
use std::io::Write;
use std::marker::PhantomData;

use crate::arena::AVec;
use crate::arena::RawArena;
use crate::seal::Seal;
use crate::seal::Sealed;
use crate::tdp;
use crate::Codec;
use crate::Error;
use crate::OptMut;
use crate::Repeated;
use crate::Slice;

pub(crate) mod names;
pub(crate) mod private;
pub(crate) mod tuple;

mod impls;
mod view;

pub use names::Name;
pub use view::*;

/// A pz type.
///
/// This type is implemented by all `pz` generated types, as well as [`bool`],
/// [`i32`], [`i64`], [`u32`], [`u64`], [`f32`], [`f64`], and [`String`].
///
/// It is also implemented by tuples of size up to 12 and the [`Opt`] and
/// [`Rep`] markers.
pub trait Type: Views + private::Type {}
impl<T: Views + private::Type> Type for T {}

/// A field setting argument.
///
/// This abstracts the notion of setting a value on a field: for example, all
/// optional `i32` fields are settable from both `i32` and `Option<i32>`. This
/// type allows `set()` to take `impl Set<Opt<i32>>`.
pub trait Set<T: Views> {
  /// Applies this value to the given mutator.
  fn apply_to(self, m: Mut<T>);
}

/// A message type.
///
/// This does not just include types declared with `message`, but any type that
/// has fields, such as `choice` and `struct`.
pub trait Message: Type + Views + Default + private::Message
// XXX: Adding these bounds causes rustc to have a stroke?
// where
// for<'a> Ref<'a, Self>: MessageRef<'a>,
// for<'a> Mut<'a, Self>: MessageMut<'a>,
{
  /// The default value for this message, provided as a static constant.
  const DEFAULT: &'static Self;

  /// Deserializes a new message from the given stream.
  fn parse(codec: Codec, input: &mut dyn Read) -> Result<Self, Error> {
    let mut new = Self::default();
    new.parse_in_place(codec, input)?;
    Ok(new)
  }

  /// Deserializes onto this message in-place from the given stream.
  fn parse_in_place(
    &mut self,
    codec: Codec,
    input: &mut dyn Read,
  ) -> Result<(), Error> {
    match codec {
      Codec::Protobuf => {
        let mut ctx = tdp::parse::Context::new(
          input,
          Self::__arena(Seal, &mut self.as_mut()),
        );
        return ctx.parse(Self::__raw(Seal, self.as_ref()), Self::__TDP);
      }
    }
  }

  /// Serializes this message to the given stream.
  fn emit(&self, codec: Codec, output: &mut dyn Write) -> Result<(), Error> {
    let _ = (codec, output);
    todo!()
  }

  /// Serializes this message to an in-memory byte array.
  fn to_bytes(&self, codec: Codec) -> Vec<u8> {
    let mut bytes = Vec::new();
    let _ = self.emit(codec, &mut bytes);
    bytes
  }

  /// Converts an ordinary Rust reference into a message reference.
  fn as_ref(&self) -> Ref<Self>;

  /// Converts an ordinary Rust reference into a mutable message reference.
  fn as_mut(&mut self) -> Mut<Self>;

  /// Selects the fields given by `selector` out of this message by reference.
  fn get<S>(&self, selector: S) -> Ref<S::Type>
  where
    S: Selector<Self>,
  {
    let _ = selector;
    S::get(self.as_ref())
  }

  /// Selects the fields given by `selector` out of this message by mutable
  /// reference.
  ///
  /// If this would result in aliasing, it generates a post-monomorphization
  /// error.
  fn get_mut<S>(&mut self, selector: S) -> Mut<S::Type>
  where
    S: Selector<Self>,
  {
    let _ = selector;
    S::get_mut(self.as_mut())
  }

  /// Clears this message, resetting it to its original state, without
  /// discarding any held allocations.
  fn clear(&mut self) {
    if private::Message::__is_null(self, Seal) {
      return;
    }

    // No need to go through as_mut(); if we made it past the null check we know
    // this message is writable.
    unsafe { Self::__TDP.clear(Self::__raw(Seal, self.as_ref())) }
  }
}

/// A message reference type.
///
/// Each [`Message`]'s [`Ref`] type implements this trait.
pub trait MessageRef<'a>: Sized
where
  Self: RefView<'a, Target = Self::Message>,
  Self::Message: Views<Ref<'a> = Self>,
{
  /// The message type this reference is for.
  type Message: Message;

  /// Serializes this message to the given stream.
  fn emit(&self, codec: Codec, output: &mut dyn Write) -> Result<(), Error> {
    let _ = (codec, output);
    todo!()
  }

  /// Serializes this message to an in-memory byte array.
  fn to_bytes(&self, codec: Codec) -> Vec<u8> {
    let mut bytes = Vec::new();
    let _ = self.emit(codec, &mut bytes);
    bytes
  }

  /// Selects the fields given by `selector` out of this reference.
  fn get<S>(self, selector: S) -> Ref<'a, S::Type>
  where
    S: Selector<Self::Message>,
  {
    let _ = selector;
    S::get(self)
  }
}

/// A message mutable reference type.
///
/// Each [`Message`]'s [`Mut`] type implements this trait.
pub trait MessageMut<'a>: Sized
where
  Self: MutView<'a, Target = Self::Message>,
  Self::Message: Views<Mut<'a> = Self>,
  Ref<'a, Self::Message>: MessageRef<'a, Message = Self::Message>,
{
  /// The message type this reference is for.
  type Message: Message;

  /// Parses onto this message in place from the given stream.
  fn parse_in_place(
    &mut self,
    codec: Codec,
    input: &mut dyn Read,
  ) -> Result<(), Error> {
    match codec {
      Codec::Protobuf => {
        let mut ctx = tdp::parse::Context::new(
          input,
          <Self::Message as private::Message>::__arena(Seal, self),
        );
        return ctx.parse(
          <Self::Message as private::Message>::__raw(Seal, self.as_ref()),
          <Self::Message as private::Message>::__TDP,
        );
      }
    }
  }

  /// Serializes this message to the given stream.
  fn emit(&self, codec: Codec, output: &mut dyn Write) -> Result<(), Error> {
    let _ = (codec, output);
    todo!()
  }

  /// Serializes this message to an in-memory byte array.
  fn to_bytes(&self, codec: Codec) -> Vec<u8> {
    let mut bytes = Vec::new();
    let _ = self.emit(codec, &mut bytes);
    bytes
  }

  /// Selects the fields given by `selector` out of this reference.
  fn get<S>(self, selector: S) -> Ref<'a, S::Type>
  where
    S: Selector<Self::Message>,
  {
    self.into_ref().get(selector)
  }

  /// Selects the fields given by `selector` out of this message by mutable
  /// reference.
  ///
  /// If this would result in aliasing, it generates a post-monomorphization
  /// error.
  fn get_mut<S>(self, selector: S) -> Mut<'a, S::Type>
  where
    S: Selector<Self::Message>,
  {
    let _ = selector;
    S::get_mut(self)
  }

  /// Clears this message, resetting it to its original state, without
  /// discarding any held allocations.
  fn clear(&mut self) {
    // No need to go through as_mut(); if we made it past the null check we know
    // this message is writable.
    unsafe {
      <Self::Message as private::Message>::__TDP.clear(
        <Self::Message as private::Message>::__raw(Seal, self.as_ref()),
      )
    }
  }
}

/// A field of a message.
///
/// If `M: Message`, then `M: Field<field!(xyz)>` if `M` is statically know to
///  a field named `xyz`. The trait methods of that implementation allow
/// operating on that field generically.
pub trait Field<Name>: Message
where
  Name: Selector<Self, Type = Self::Type>,
{
  /// The type of this field.
  type Type: Views + access::Access;

  /// The name of this field, as a type (this is always the same as the `Name`
  /// type parameter).
  type Name;

  /// The tag number for this field, as declared in the schema.
  const NUMBER: i32;

  /// An index for this field, i.e., the relative position in which it appears
  /// in the schema.
  const INDEX: usize;

  /// The name of this field, as a string.
  const NAME: &'static str;
}

/// A message selector.
///
/// A selector generalizes single field access: if `M: Field<X>`, then
/// `X: Selector<M>`. Moreover, `fields!(a, b, c): Selector<M>` if `M` has
/// each of those fields. This makes it possible to select multiple disjoint
/// fields of a message.
pub trait Selector<M: Message> {
  /// The result of this selector. If this selector selects more than one
  /// message, it will be a tuple.
  type Type: Views;

  /// Whether this selects disjoint fields. If this is false,
  /// [`Selector::get_mut()`] will fail to compile.
  const DISJOINT: bool;

  #[doc(hidden)]
  const __M: tuple::Mask;

  /// Selects fields of `message`.
  fn get(message: Ref<M>) -> Ref<Self::Type>;

  /// Mutably selects fields out of `message`.
  ///
  /// If this would result in aliasing, it generates a post-monomorphization
  /// error.
  fn get_mut(mut message: Mut<M>) -> Mut<Self::Type> {
    const {
      assert!(
        Self::DISJOINT,
        "attempted to mutably select non-disjoint fields"
      );
    }

    unsafe { Self::get_mut_unchecked(Seal, &mut message) }
  }

  /// Mutably selects fields out of `message`.
  ///
  /// If this would result in aliasing, returns [`None`].
  fn try_mut(mut message: Mut<M>) -> Option<Mut<Self::Type>> {
    if !Self::DISJOINT {
      return None;
    }
    unsafe { Some(Self::get_mut_unchecked(Seal, &mut message)) }
  }

  /// Mutably selects fields out of `message`.
  ///
  /// # Safety
  ///
  /// This function does not check `DISJOINT`, nor does it correctly bind the
  /// outputs' lifetime to the input. This function is intended for internal
  /// use only.
  #[doc(hidden)]
  unsafe fn get_mut_unchecked<'a>(
    _: impl Sealed,
    message: &mut Mut<M>,
  ) -> Mut<'a, Self::Type>;
}

impl<N: Name, M: Field<N, Type: access::Access>> Selector<M> for N {
  type Type = M::Type;

  const DISJOINT: bool = true;

  const __M: tuple::Mask = tuple::Mask::single(M::INDEX);

  fn get(message: Ref<M>) -> Ref<Self::Type> {
    let raw = M::__raw(Seal, message);
    unsafe {
      let tdp = M::__TDP.field(M::INDEX as u16);
      <M::Type as access::Access>::__ref(raw, tdp)
    }
  }

  unsafe fn get_mut_unchecked<'a>(
    _: impl Sealed,
    message: &mut Mut<M>,
  ) -> Mut<'a, Self::Type> {
    let raw = M::__raw(Seal, message.as_ref());
    let arena = M::__arena(Seal, message);
    let tdp = M::__TDP.field(M::INDEX as u16);
    <M::Type as access::Access>::__mut(raw, arena, tdp)
  }
}

mod access {
  use std::ptr::NonNull;

  use tdp::Opaque;

  use super::*;

  pub trait Access: Views {
    unsafe fn __ref<'a>(raw: Opaque, tdp: tdp::Field) -> Ref<'a, Self>;

    unsafe fn __mut<'a>(
      raw: Opaque,
      arena: RawArena,
      tdp: tdp::Field,
    ) -> Mut<'a, Self>;
  }

  impl<T: Type> Access for Opt<T> {
    unsafe fn __ref<'a>(raw: Opaque, tdp: tdp::Field) -> Ref<'a, Self> {
      if !tdp.has(raw) {
        return None;
      }
      Some(T::__ref(Seal, tdp.cast(raw)))
    }

    unsafe fn __mut<'a>(
      raw: Opaque,
      arena: RawArena,
      tdp: tdp::Field,
    ) -> Mut<'a, Self> {
      OptMut::new(raw, arena, tdp)
    }
  }

  impl<T: Type> Access for Rep<T> {
    unsafe fn __ref<'a>(raw: Opaque, tdp: tdp::Field) -> Ref<'a, Self> {
      if !tdp.has(raw) {
        return Slice::default();
      }

      let vec = tdp.cast::<AVec<T::__Storage<Seal>>>(raw).as_ref();
      Slice::from_raw_parts(
        vec.as_ptr().unwrap_or(NonNull::dangling()),
        vec.len(),
      )
    }

    unsafe fn __mut<'a>(
      raw: Opaque,
      arena: RawArena,
      tdp: tdp::Field,
    ) -> Mut<'a, Self> {
      if !tdp.has(raw) {
        tdp.init(raw, arena)
      }

      let vec = tdp.cast::<AVec<T::__Storage<Seal>>>(raw).as_mut();
      Repeated::from_raw_parts(vec, arena)
    }
  }
}

/// Type-level adapter for optional fields.
///
/// The type of an optional field of type `i32` is not `i32`, but `Opt<i32>`.
/// This allows e.g. `Mut<Opt<i32>>` to expose a way to set and clear the
/// presence of the field.
pub struct Opt<T>(PhantomData<T>);

/// Type-level adapter for repeated fields.
///
/// The type of a repeated field of type `i32` is `Rep<i32>`.
pub struct Rep<T>(PhantomData<T>);

impl<T: Type> Views for Opt<T> {
  type Ref<'a> = Option<Ref<'a, T>>;
  type Mut<'a> = OptMut<'a, T>;
}

impl<'a, R: RefView<'a, Target: Type>> RefView<'a> for Option<R> {
  type Target = Opt<R::Target>;

  fn as_ref(&self) -> Ref<Self::Target> {
    self.as_ref().map(|r| r.as_ref())
  }
}

impl<T: Type> Views for Rep<T> {
  type Ref<'a> = Slice<'a, T>;
  type Mut<'a> = Repeated<'a, T>;
}

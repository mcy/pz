//! Table-driven codec support.

use std::alloc::Layout;
use std::mem;
use std::ptr::NonNull;
use std::usize;

use crate::arena::RawArena;

pub mod parse;

/// A descriptor for a parseable type.
///
/// This is essentially a vtable pointer for a Protobuf type, containing all the
/// necessary information to parse and serialize it.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Desc(NonNull<DescHeader>);
impl Desc {
  fn header(&self) -> &DescHeader {
    unsafe { self.0.as_ref() }
  }

  fn raw(self) -> *mut DescHeader {
    self.0.as_ptr()
  }

  /// Gets a valid layout for this type's storage.
  pub fn layout(self) -> Layout {
    unsafe { Layout::from_size_align_unchecked(self.header().size as usize, 8) }
  }

  /// The number of bytes in this type's hasbits block, if it has one.
  pub fn hasbit_bytes(self) -> usize {
    self.header().num_hasbit_words as usize * 4
  }

  /// Gets the `n`th type associated with this `DescPtr`.
  ///
  /// # Safety
  ///
  /// `idx` must be in range.
  pub unsafe fn desc(self, idx: u16) -> Desc {
    let getter: fn() -> Desc =
      unsafe { self.header().descs.add(idx as usize).read() };
    getter()
  }

  /// Gets the kind of type this descriptor is for.
  pub fn kind(self) -> DescKind {
    self.header().kind
  }

  /// Whether this is the descriptor for a message.
  pub fn is_message(self) -> bool {
    self.kind() == DescKind::Message
  }

  /// Whether this is the descriptor for a choice.
  pub fn is_choice(self) -> bool {
    self.kind() == DescKind::Choice
  }

  /// Returns the first field in this `Desc`.
  ///
  /// Returns `None` if this `Desc` has no fields.
  pub fn first_field(self) -> Option<Field> {
    let first =
      unsafe { Field(self, NonNull::new_unchecked(self.raw().add(1).cast())) };
    (first.number() != 0).then_some(first)
  }

  /// Returns the `n`th field in this `Desc`.
  ///
  /// # Safety
  ///
  /// `n` must be in-range for this `Desc`.
  pub unsafe fn field(self, n: u16) -> Field {
    Field(
      self,
      NonNull::new_unchecked(
        self.raw().add(1).cast::<FieldStorage>().add(n as usize),
      ),
    )
  }
}

/// A descriptor for a `Desc`'s field.
///
/// This is essentially a vtable pointer for a Protobuf type, containing all the
/// necessary information to parse and serialize it.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Field(Desc, NonNull<FieldStorage>);
impl Field {
  /// Returns this field's `Desc`.
  pub fn parent(self) -> Desc {
    self.0
  }

  fn storage(&self) -> &FieldStorage {
    unsafe { self.1.as_ref() }
  }

  fn raw(self) -> *mut FieldStorage {
    self.1.as_ptr()
  }

  /// Gets the next field after this one, wrapping around if necessary.
  pub fn next(self) -> Field {
    let next = unsafe {
      Field(self.parent(), NonNull::new_unchecked(self.raw().add(1)))
    };

    if next.number() == 0 {
      // Wrap back to the beginning.
      return self.parent().first_field().unwrap();
    }

    next
  }

  /// Gets this field's number.
  pub fn number(self) -> u32 {
    self.storage().number
  }

  pub const KIND_MASK: u32 = 0b1111;
  pub const KIND_SHIFT: u32 = 0;

  /// Gets the kind of field this is.
  pub fn kind(self) -> Kind {
    let nybble = (self.storage().flags & Self::KIND_MASK) as u8;
    unsafe { mem::transmute::<u8, Kind>(nybble) }
  }

  pub const REP_MASK: u32 = 0b1_0000;
  pub const REP_SHIFT: u32 = 4;

  /// Gets whether this is a repeated field.
  pub fn is_repeated(self) -> bool {
    self.storage().flags & Self::REP_MASK != 0
  }

  /// The offset at which this field's storage is in an allocated value.
  pub fn offset(self) -> usize {
    self.storage().offset as usize
  }

  /// Gets this field's type.
  ///
  /// # Safety
  ///
  /// This must be a `Type`-kinded field.
  pub unsafe fn desc(self) -> Desc {
    self.parent().desc(self.storage().desc)
  }

  /// The index of the word of the hasbit vector that contains this field's
  /// hasbit (if it has a hasbit).
  pub fn hasbit_word(self) -> usize {
    self.storage().hasbit as usize / 32
  }

  /// The mask to extract the hasbit from the hasbit word.
  pub fn hasbit_mask(self) -> u32 {
    1 << (self.storage().hasbit % 32)
  }

  /// Checks whether this field is set in a raw allocated value.
  ///
  /// # Safety
  ///
  /// `raw` must point to a valid, allocated value of this type.
  pub unsafe fn has(&self, raw: *mut u8) -> bool {
    if self.parent().is_choice() {
      return raw.cast::<u32>().read() == self.number();
    }

    if self.is_repeated() {
      return true;
    }

    raw.cast::<u32>().add(self.hasbit_word()).read() & self.hasbit_mask() != 0
  }

  /// Initializes this field in a raw allocated value.
  ///
  /// # Safety
  ///
  /// `raw` must point to a valid, allocated value of this type.
  #[inline(always)]
  pub unsafe fn init(&self, raw: *mut u8, arena: RawArena) {
    let value = raw.add(self.offset());
    if self.parent().is_choice() {
      let which = unsafe { &mut *raw.cast::<u32>() };
      if mem::replace(which, self.number()) == self.number() {
        return;
      }

      if self.is_repeated() {
        // We need to vaporize the *whole* AVec, since we don't know a
        // priori if whatever used to be here had a compatible layout.
        value.write_bytes(0, 3 * mem::size_of::<usize>());
      }
    } else {
      // NOTE: The hasbits are always the first thing in the message
      // right now.
      let word = raw.cast::<u32>().add(self.hasbit_word());
      if *word & self.hasbit_mask() != 0 {
        return;
      }

      *word |= self.hasbit_mask();
    }

    // We need to make sure to clear the *values* now, because the actual
    // clear operatation only zeroes hasbits. This means that if we clear
    // e.g. a string field, its pointer and length will not have been reset.
    let to_overwrite = match self.kind() {
      Kind::I32 | Kind::F32 => 4,
      Kind::I64 | Kind::F64 => 8,
      Kind::Bool => 1,
      Kind::Str => mem::size_of::<(*mut u8, usize)>(),
      Kind::Type => {
        // Types are a little special because we need to allocate stuff.
        let desc = self.desc();
        let ptr = &mut *value.cast::<*mut u8>();
        match desc.kind() {
          DescKind::Message if !ptr.is_null() => {
            ptr.write_bytes(0, desc.hasbit_bytes())
          }
          _ => {
            *ptr = arena.alloc(desc.layout()).as_ptr();
            ptr.write_bytes(0, desc.layout().size());
          }
        }

        return;
      }
    };

    value.write_bytes(0, to_overwrite);
  }

  /// Clears this value.
  ///
  /// # Safety
  ///
  /// `raw` must point to a valid, allocated value of this type.
  pub unsafe fn clear(&self, raw: *mut u8) {
    if self.parent().is_choice() {
      raw.cast::<u32>().write(0);
      return;
    }

    if !self.is_repeated() {
      *raw.cast::<u32>().add(self.hasbit_word()) &= !self.hasbit_mask();
    }
  }
}

/// Storage for `Desc` global.
///
/// This struct all the information needed to parse any top-level Protobuf type.
/// There is a static variable of this type associated with every non-enum
/// generated type, which is fed into the parser.
///
/// This type should not be constructed or referred to directly, except by the
/// code generator.
#[repr(C)]
#[derive(Debug)]
pub struct DescStorage<const FIELDS: usize> {
  /// The header, which contains all "type-level" information.
  pub header: DescHeader,
  /// The fields. This array must have at least one element and the final
  /// element's number must be zero.
  pub fields: [FieldStorage; FIELDS],
}

unsafe impl<const FIELDS: usize> Sync for DescStorage<FIELDS> {}
impl<const FIELDS: usize> DescStorage<FIELDS> {
  /// Gets the `Desc` pointer.
  ///
  /// `Desc`s should only ever exist as global variables, so this requires a
  /// `'static` lifetime.
  ///
  /// # Safety
  ///
  /// This function assumes that all the relevant invariants of a `Desc` are
  /// observed (in other words, that this was constructed by the code
  /// generator or the runtime).
  pub unsafe fn get(&'static self) -> Desc {
    Desc(NonNull::from(self).cast())
  }
}

/// The header for a `DescStorage`.
#[derive(Debug)]
pub struct DescHeader {
  /// The size of this type's storage, in bytes; 4GB limit.
  pub size: u32,
  /// A pointer to an array of functions that return the `DescPtr` for a type
  /// referred to directly by this message. See `ty` under `Field`.
  pub descs: *const fn() -> Desc,
  /// The number of hasbit words in an allocated value of this type.
  pub num_hasbit_words: u16,
  /// The kind of type this descriptor is for.
  pub kind: DescKind,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DescKind {
  Message = 0,
  Choice = 1,
}

/// Storage for one of a `Desc`'s fields.
#[derive(Debug)]
pub struct FieldStorage {
  pub number: u32,
  pub flags: u32,
  pub offset: u32,
  pub desc: u16,
  pub hasbit: u16,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Kind {
  I32 = 0,
  I64 = 1,
  F32 = 2,
  F64 = 3,
  Bool = 4,
  Str = 5,
  Type = 6,
}

impl Kind {
  // Converts this `Kind` into a raw value.
  pub const fn raw(self) -> u32 {
    self as u32
  }
}

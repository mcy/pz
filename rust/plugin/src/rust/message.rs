//! Message codegen.

use std::collections::HashMap;

use pz::proto;
use pz::proto::field::Type as TypeEnum;
use pz::proto::r#type::Kind;

use crate::emit::SourceWriter;
use crate::rust::fields::FieldGenerators;
use crate::rust::fields::Where;
use crate::rust::names::ident;
use crate::rust::names::type_name;
use crate::rust::names::field_name_type_name;
use crate::Type;

pub fn emit_main_struct(w: &mut SourceWriter) {
  w.write(
    r#"
      /// $keyword `$package.$Name`
      pub struct $Ident {
        ptr: $NonNull<$priv::Storage>,
        arena: $Option<__z::RawArena>,
      }

      const _: () = {
        assert!(
          $size_of::<$priv::Storage>() < (u32::MAX as usize),
          "storage size exceeds 4GB",
        );
      };
    "#,
  )
}

pub fn emit_common_methods(w: &mut SourceWriter) {
  w.write(
    r#"
      /// Constructs a new, empty [`$Type`].
      pub const fn new() -> Self {
        Self { 
          ptr: $NonNull::dangling(),
          arena: $None, 
        }
      }

      fn __init(&mut self) {
        if self.arena.is_none() {
          self.arena = $Some(__z::RawArena::new());
        }
      }

      /// Deserializes a new [`$Type`] from the given stream.
      /// 
      /// See [`Message::parse()`][__r::Message::parse].
      pub fn parse(codec: __rt::Codec, input: &mut dyn $Read) -> $Result<Self, __rt::Error> {
        <Self as __r::Message>::parse(codec, input)
      }
      
      /// Deserializes onto this [`$Type`] in place from the given stream.
      /// 
      /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
      pub fn parse_in_place(
        &mut self,
        codec: __rt::Codec,
        input: &mut dyn $Read,
      ) -> $Result<(), __rt::Error> {
        <Self as __r::Message>::parse_in_place(self, codec, input)
      }

      /// Serializes this [`$Type`] to the given stream.
      /// 
      /// See [`Message::emit()`][__r::Message::emit].
      fn emit(&self, codec: __rt::Codec, output: &mut dyn $Write) -> $Result<(), __rt::Error> {
        <Self as __r::Message>::emit(self, codec, output)
      }

      /// Serializes this [`$Type`] to an in-memory byte array.
      /// 
      /// See [`Message::to_bytes()`][__r::Message::to_bytes].
      fn to_bytes(&self, codec: __rt::Codec) -> $Vec<__s::primitive::u8> {
        <Self as __r::Message>::to_bytes(self, codec)
      }

      /// Converts an ordinary Rust reference into a message reference.
      /// 
      /// See [`Message::as_ref()`][__r::Message::as_ref].
      pub fn as_ref(&self) -> $Ref<Self> {
        use __s::convert::From;
        unsafe { <Self as __z::Type>::__ref(
          __z::Seal,
          $NonNull::from(&self.ptr).cast::<$Option<__z::tdp::Opaque>>(),
        )}
      }
    
      /// Converts an ordinary Rust reference into a mutable message reference.
      /// 
      /// See [`Message::as_mut()`][__r::Message::as_mut].
      pub fn as_mut(&mut self) -> $Mut<Self> {
        use __s::convert::From;
        self.__init();
        unsafe { <Self as __z::Type>::__mut(
          __z::Seal,
          $NonNull::from(&mut self.ptr).cast::<$Option<__z::tdp::Opaque>>(),
          self.arena.unwrap(),
        )}
      }

      /// Selects the fields given by `selector` out of this message by reference.
      /// 
      /// See [`Message::get()`][__r::Message::get].
      pub fn get<S>(&self, selector: S) -> __r::Ref<S::Type>
      where
        S: __r::Selector<$Type>,
      {
        <Self as __r::Message>::get(self, selector)
      }

      /// Selects the fields given by `selector` out of this message by mutable
      /// reference.
      ///
      /// If this would result in aliasing, it generates a post-monomorphization
      /// error.
      /// 
      /// See [`Message::get_mut()`][__r::Message::get_mut].
      pub fn get_mut<S>(&mut self, selector: S) -> __r::Mut<S::Type>
      where
        S: __r::Selector<$Type>,
      {
        <Self as __r::Message>::get_mut(self, selector)
      }

      /// Resets this [`$Type`] to its default state.
      /// 
      /// See [`Message::clear()`][__r::Message::clear].
      pub fn clear(&mut self) {
        <Self as __r::Message>::clear(self)
      }
    "#,
  )
}

pub fn emit_common_ref_methods(w: &mut SourceWriter) {
  w.write(
    r#"
      /// Shortens this view's lifetime, analogous to reborrowing.
      ///
      /// See [`RefView::as_ref()`][__r::RefView::as_ref].
      pub fn as_ref(&self) -> $priv::Ref { *self }

      /// Serializes this [`$Type`] to the given stream.
      /// 
      /// See [`MessageRef::emit()`][__r::MessageRef::emit].
      fn emit(&self, codec: __rt::Codec, output: &mut dyn $Write) -> $Result<(), __rt::Error> {
        <Self as __r::MessageRef>::emit(self, codec, output)
      }

      /// Serializes this [`$Type`] to an in-memory byte array.
      /// 
      /// See [`MessageRef::to_bytes()`][__r::MessageRef::to_bytes].
      fn to_bytes(&self, codec: __rt::Codec) -> $Vec<__s::primitive::u8> {
        <Self as __r::MessageRef>::to_bytes(self, codec)
      }

      /// Selects the fields given by `selector` out of this message by reference.
      /// 
      /// See [`MessageRef::get()`][__r::MessageRef::get].
      pub fn get<S>(self, selector: S) -> __r::Ref<'proto, S::Type>
      where
        S: __r::Selector<$Type>,
      {
        <Self as __r::MessageRef>::get(self, selector)
      }
    "#,
  )
}

pub fn emit_common_mut_methods(w: &mut SourceWriter) {
  w.write(
    r#"
      /// Shortens this view's lifetime, analogous to reborrowing.
      ///
      /// See [`MutView::as_ref()`][__r::MutView::as_ref].
      pub fn as_ref(&self) -> $priv::Ref { self.r }

      /// Consumes this mutator, converting it into an immutable view.
      ///
      /// See [`MutView::into_ref()`][__r::MutView::into_ref].
      pub fn into_ref(self) -> $priv::Ref<'proto> { self.r }

      /// Shortens this view's lifetime, analogous to reborrowing.
      ///
      /// See [`MutView::as_mut()`][__r::MutView::as_mut].
      pub fn as_mut(&mut self) -> $priv::Mut {
        $priv::Mut { r: self.r, arena: self.arena, _ph: $PhantomData, }
      }

      /// Parses onto this [`$Type`] in place from the given stream.
      /// 
      /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
      pub fn parse_in_place(
        &mut self,
        codec: __rt::Codec,
        input: &mut dyn $Read,
      ) -> $Result<(), __rt::Error> {
        <Self as __r::MessageMut>::parse_in_place(self, codec, input)
      }

      /// Serializes this [`$Type`] to the given stream.
      /// 
      /// See [`MessageMut::emit()`][__r::MessageMut::emit].
      fn emit(&self, codec: __rt::Codec, output: &mut dyn $Write) -> $Result<(), __rt::Error> {
        <Self as __r::MessageMut>::emit(self, codec, output)
      }

      /// Serializes this [`$Type`] to an in-memory byte array.
      /// 
      /// See [`MessageMut::to_bytes()`][__r::MessageMut::to_bytes].
      fn to_bytes(&self, codec: __rt::Codec) -> $Vec<__s::primitive::u8> {
        <Self as __r::MessageMut>::to_bytes(self, codec)
      }

      /// Selects the fields given by `selector` out of this message by reference.
      /// 
      /// See [`MessageMut::get()`][__r::MessageMut::get].
      pub fn get<S>(self, selector: S) -> __r::Ref<'proto, S::Type>
      where
        S: __r::Selector<$Type>,
      {
        <Self as __r::MessageMut>::get(self, selector)
      }

      /// Selects the fields given by `selector` out of this message by mutable
      /// reference.
      ///
      /// If this would result in aliasing, it generates a post-monomorphization
      /// error.
      /// 
      /// See [`MessageMut::get_mut()`][__r::MessageMut::get_mut].
      pub fn get_mut<S>(self, selector: S) -> __r::Mut<'proto, S::Type>
      where
        S: __r::Selector<$Type>,
      {
        <Self as __r::MessageMut>::get_mut(self, selector)
      }

      /// Resets this [`$Type`] to its default state.
      /// 
      /// See [`MessageMut::clear()`][__r::MessageMut::clear].
      pub fn clear(&mut self) {
        <Self as __r::MessageMut>::clear(self)
      }
    "#
  )
}

pub fn emit_impls(w: &mut SourceWriter) {
  w.write(
    r#"
      impl __z::Type for $Type {
        type __Storage<S: __z::Sealed> = $Option<__z::tdp::Opaque>;

        unsafe fn __ref<'a, S: __z::Sealed>(
          _: S,
          ptr: $NonNull<$Option<__z::tdp::Opaque>>,
        ) -> __r::Ref<'a, Self> {
          match ptr.read() {
            $None => $Type::DEFAULT.as_ref(),
            $Some(ptr) if ptr == $NonNull::dangling() => $Type::DEFAULT.as_ref(),
            $Some(ptr) => $priv::Ref { ptr: ptr.cast(), _ph: $PhantomData }
          }
        }

        unsafe fn __mut<'a, S: __z::Sealed>(
          s: S,
          mut outer: $NonNull<$Option<__z::tdp::Opaque>>,
          arena: __z::RawArena,
        ) -> __r::Mut<'a, Self> {
          let ptr = outer.as_mut();
          if ptr.is_none() || *ptr == $Some($NonNull::<$priv::Storage>::dangling().cast::<u8>()) {
            let new = arena.alloc(Self::__LAYOUT);
            new.write_bytes(0, Self::__LAYOUT.size());
            *ptr = $Some(new);
          }

          $priv::Mut {
            r: Self::__ref(s, outer),
            arena,
            _ph: $PhantomData,
          }
        }

        unsafe fn __resize<S: __z::Sealed>(
          _: S,
          vec: &mut __z::AVec<$Option<__z::tdp::Opaque>>,
          new_len: usize,
          arena: __z::RawArena,
        ) {
          vec.resize(new_len, arena)
        }
      }

      impl __r::Views for $Type {
        type Ref<'a> = $priv::Ref<'a>;
        type Mut<'a> = $priv::Mut<'a>;
      }

      impl<'a> __r::RefView<'a> for $priv::Ref<'a> {
        type Target = $Type;
        fn as_ref(&self) -> $priv::Ref { *self }
      }

      impl<'a> __r::MutView<'a> for $priv::Mut<'a> {
        type Target = $Type;
        fn as_ref(&self) -> $priv::Ref { self.r }
        fn into_ref(self) -> $priv::Ref<'a> { self.r }
        fn as_mut(&mut self) -> $priv::Mut {
          $priv::Mut { r: self.r, arena: self.arena, _ph: $PhantomData, }
        }
      }

      impl __r::Message for $Type {
        const DEFAULT: &'static Self = $Type::DEFAULT;

        fn as_ref(&self) -> __r::Ref<Self> {
          Self::as_ref(self)
        }
        fn as_mut(&mut self) -> __r::Mut<Self> {
          Self::as_mut(self)
        }
      }

      impl<'a> __r::MessageRef<'a> for $priv::Ref<'a> {
        type Message = $Type;
      }
      impl<'a> __r::MessageMut<'a> for $priv::Mut<'a> {
        type Message = $Type;
      }

      impl $Default for $Type {
        fn default() -> Self {
          Self::new()
        }
      }

      impl<T: __r::Set<$Type>> __s::convert::From<T> for $Type {
        fn from(value: T) -> $Type {
          let mut msg = Self::default();
          value.apply_to(msg.as_mut());
          msg
        }
      }

      impl __r::Set<$Type> for &$Type {
        fn apply_to(self, mut m: __r::Mut<$Type>) {
          $Type::__memcpy(m, self.as_ref())
        }
      }

      impl __r::Set<__r::Opt<$Type>> for &$Type {
        fn apply_to(self, m: __r::Mut<__r::Opt<$Type>>) {
          $Type::__memcpy(m.into_inner(), self.as_ref())
        }
      }

      impl __r::Set<$Type> for $Ref<'_, $Type> {
        fn apply_to(self, mut m: __r::Mut<$Type>) {
          $Type::__memcpy(m, self)
        }
      }

      impl __r::Set<__r::Opt<$Type>> for $Ref<'_, $Type> {
        fn apply_to(self, m: __r::Mut<__r::Opt<$Type>>) {
          $Type::__memcpy(m.into_inner(), self)
        }
      }

      impl __r::Set<$Type> for &$Mut<'_, $Type> {
        fn apply_to(self, mut m: __r::Mut<$Type>) {
          $Type::__memcpy(m, self.as_ref())
        }
      }

      impl __r::Set<__r::Opt<$Type>> for &$Mut<'_, $Type> {
        fn apply_to(self, m: __r::Mut<__r::Opt<$Type>>) {
          $Type::__memcpy(m.into_inner(), self.as_ref())
        }
      }

      impl $Default for $priv::Ref<'_> {
        fn default() -> Self {
          $Type::DEFAULT.as_ref()
        }
      }

      impl __s::ops::Drop for $Type {
        fn drop(&mut self) {
          if let $Some(arena) = self.arena {
            unsafe { arena.destroy() }
          }
        }
      }

      impl $fmt::Debug for $priv::Ref<'_> {
        fn fmt(&self, fmt: &mut $fmt::Formatter) -> $fmt::Result {
          fmt.write_str("$package.$Name ")?;
          let mut debug = __z::Debug::new(fmt);
          self.__debug(&mut debug)
        }
      }

      impl $fmt::Debug for $priv::Mut<'_> {
        fn fmt(&self, fmt: &mut $fmt::Formatter) -> $fmt::Result {
          $fmt::Debug::fmt(&self.as_ref(), fmt)
        }
      }

      impl $fmt::Debug for $Type {
        fn fmt(&self, fmt: &mut $fmt::Formatter) -> $fmt::Result {
          $fmt::Debug::fmt(&self.as_ref(), fmt)
        }
      }
    "#,
  )
}

pub fn emit_view_types(w: &mut SourceWriter) {
  w.write(
    r#"
      #[derive(Copy, Clone)]
      pub struct Ref<'proto> {
        pub(in super) ptr: $NonNull<$priv::Storage>,
        pub(in super) _ph: $PhantomData<&'proto $Type>,
      }
      
      pub struct Mut<'proto> {
        pub(in super) r: Ref<'proto>,
        pub(in super) arena: __z::RawArena,
        pub(in super) _ph: $PhantomData<&'proto mut $Type>,
      }
    "#,
  )
}

pub fn emit(ty: Type, w: &mut SourceWriter) {
  let gen = FieldGenerators::build(ty.fields());
  let hasbit_words = gen.num_hasbits / 32 + (gen.num_hasbits % 32 != 0) as u32;

  let mut ty_ptrs = ty
    .fields()
    .filter_map(|f| f.ty().1.filter(|ty| ty.kind() != Kind::Enum))
    .collect::<Vec<_>>();
  ty_ptrs.sort_by(|a, b| (a.package(), a.name()).cmp(&(b.package(), b.name())));
  ty_ptrs.dedup_by(|a, b| (a.package(), a.name()) == (b.package(), b.name()));

  w.emit(
    vars! {
      hasbit_words,
      NUM_TYS: ty_ptrs.len(),
      "Type::struct": |w| emit_main_struct(w),
      "Type::common": |w| emit_common_methods(w),
      "Type::impls": |w| emit_impls(w),
      "Type::views": |w| emit_view_types(w),
      "Type::fields": |w| for field in &gen.fields {
        field.in_storage(w);
      },
      "Type::field_init": |w| for field in &gen.fields {
        field.in_storage_init(w);
      },
      "Type::access": |w| for field in &gen.fields {
        field.in_ref_methods(Where::TypeImpl, w);
        field.in_mut_methods(Where::TypeImpl, w);
        w.new_line();
      },
      "Type::memcpys": |w| for field in &gen.fields {
        w.emit(
          vars! {
            __name: field_name_type_name(field.field),
          },
          r#"
            __r::Set::<<$Type as __r::Field<$__name>>::Type>::apply_to(src.get($__name{}), dst.as_mut().get_mut($__name{}));
          "#
        );
      },
      "Type::field_impls": |w| for field in &gen.fields {
        field.in_impls(w);
        w.new_line();
      },
      "Type::debug": |w| for field in &gen.fields {
        field.in_debug(w);
      },
      "Ref::common": |w| emit_common_ref_methods(w),
      "Ref::access": |w| for field in &gen.fields {
        field.in_ref_methods(Where::ViewImpl, w);
        w.new_line();
      },
      "Mut::common": |w| emit_common_mut_methods(w),
      "Mut::access": |w| for field in &gen.fields {
        field.in_ref_methods(Where::MutImpl, w);
        field.in_mut_methods(Where::MutImpl, w);
        w.new_line();
      },
      tdp_descs: |w| for &ty in &ty_ptrs {
        w.emit(
          vars!{ Submsg: type_name(ty) },
          "
            $Submsg::__tdp_info,
          "
        );
      },
      tdp_fields: |w| {
        let mut hasbit_index = 0u32;
        let ty_map = ty_ptrs.iter().enumerate()
          .map(|(i, t)| (t.proto() as *const proto::Type, i))
          .collect::<HashMap<_, _>>();

        for field in ty.fields() {
          let mut ty_idx = 0;
          let tdp_kind = match field.ty() {
            (TypeEnum::I32 | TypeEnum::U32, _) => "I32",
            (TypeEnum::I64 | TypeEnum::U64, _) => "I64",
            (TypeEnum::F32, _) => "F32",
            (TypeEnum::F64, _) => "F64",
            (TypeEnum::Bool, _) => "Bool",
            (TypeEnum::String, _) => "Str",
            (TypeEnum::Type, Some(ty)) => match ty.kind() {
              Kind::Message | Kind::Choice => {
                ty_idx = ty_map[&(ty.proto() as *const _)];
                "Type"
              }
              Kind::Struct => todo!(),
              Kind::Enum => "I32",
            },
            _ => unreachable!(),
          };

          w.emit(
            vars! {
              hasbit_index,
              tdp_kind,
              ty_idx,
              name: ident(field.name()),
              number: field.number().unwrap(),
              raw_name: field.name(),
              repeated: field.is_repeated() as u32,
            },
            "
              __z::tdp::FieldStorage {
                number: $number,
                flags: 
                  __z::tdp::Kind::$tdp_kind.raw() << __z::tdp::Field::KIND_SHIFT |
                  $repeated << __z::tdp::Field::REP_SHIFT,
                offset: unsafe {
                  let msg = $Type::DEFAULT;
                  let top = msg.ptr.as_ptr().cast::<u8>();
                  let field = &msg.ptr.as_ref().$name as *const _ as *const u8;
                  field.offset_from(top) as u32
                },
                desc: $ty_idx,
                hasbit: $hasbit_index,
              },
            "
          );
          if !field.is_repeated() {
            hasbit_index += 1;
          }
        }
      },
    },
    r#"
      ${Type::struct}

      mod $priv {
        pub use super::*;

        ${Type::views}

        #[repr(C)]
        pub struct Storage {
          pub(crate) __hasbits: [u32; $hasbit_words],
          ${Type::fields}    
        }
      }

      impl $Type {
        /// The default value for [`Type`], provided as a static constant.
        ///
        /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
        pub const DEFAULT: &'static Self = unsafe { &Self {
          ptr: $NonNull::new_unchecked(&const { $priv::Storage {
            __hasbits: [0; $hasbit_words],
            ${Type::field_init}
          }} as *const $priv::Storage as *mut $priv::Storage),
          arena: $None,
        }};
        
        ${Type::common}

        ${Type::access}

        #[doc(hidden)]
        pub const __LAYOUT: $Layout = $Layout::new::<$priv::Storage>();
        #[doc(hidden)]
        pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
          raw.cast::<$priv::Storage>().as_mut().__hasbits = [0; $hasbit_words];
        }
        #[doc(hidden)]
        pub fn __tdp_info() -> __z::tdp::Desc {
          <Self as __z::Message>::__TDP
        }

        #[doc(hidden)]
        fn __memcpy(mut dst: $Mut<$Type>, src: $Ref<$Type>) {
          ${Type::memcpys}
        }
      }

      impl __z::Message for $Type {
        const __TDP: __z::tdp::Desc = {
          type Tdp = __z::tdp::DescStorage<{$NUM_FIELDS + 1}>;
          const STATIC: Tdp = Tdp {
            header: __z::tdp::DescHeader {
              size: {
                let size = $Type::__LAYOUT.size();
                assert!(size <= (u32::MAX as usize));
                size as u32
              },

              descs: {
                const DESCS: &[fn() -> __z::tdp::Desc] = &[
                  $tdp_descs
                ];
                DESCS.as_ptr()
              },
              
              num_hasbit_words: $hasbit_words,
              kind: __z::tdp::DescKind::Message,
            },

            fields: [
              $tdp_fields
              __z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
            ],
          };

          unsafe { STATIC.get() }
        };

        fn __is_null(&self, _: impl __z::Sealed) -> bool {
          self.ptr == $NonNull::dangling()
        }
        fn __raw(_: impl __z::Sealed, ptr: __r::Ref<Self>) -> __z::tdp::Opaque { ptr.ptr.cast() }
        fn __arena(_: impl __z::Sealed, ptr: &mut __r::Mut<Self>) -> __z::RawArena { ptr.arena }
      }

      ${Type::impls}

      ${Type::field_impls}

      impl<'proto> $priv::Ref<'proto> {
        ${Ref::common}

        ${Ref::access}

        #[doc(hidden)]
        pub fn __debug(self, debug: &mut __z::Debug) -> $fmt::Result {
          let mut count = 0;
          debug.start_block()?;
          ${Type::debug}
          if count != 0 {
            debug.comma(true)?;
          }
          debug.end_block()?;
          $Ok(())
        }
      }

      impl<'proto> $priv::Mut<'proto>  {
        ${Mut::common}

        ${Mut::access}
      }
    "#,
  );
  w.new_line();
}

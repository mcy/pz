//! TDP parser.

use core::slice;
use std::alloc::Layout;
use std::io;
use std::mem;
use std::ops::BitOr;
use std::ops::Shl;

use crate::arena::AVec;
use crate::arena::RawArena;
use crate::tdp::Desc;
use crate::tdp::Kind;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Io(io::Error),
  Overflow,
  OverlongVarint,
  BadTag,
}

fn eof<T>() -> Result<T> {
  Err(Error::Io(io::ErrorKind::UnexpectedEof.into()))
}

impl From<io::Error> for Error {
  fn from(e: io::Error) -> Self {
    Self::Io(e)
  }
}

struct WireFormat<'r> {
  input: &'r mut dyn io::Read,
  bytes: Box<[u8; BUF_LEN + SLACK_LEN]>,
  read_ptr: usize,
  write_ptr: usize,

  total_read: u64,
  limits: Vec<u64>,
}

#[derive(Debug)]
struct Tag {
  wire_type: u32,
  number: u32,
}

const BUF_LEN: usize = 512;
const SLACK_LEN: usize = 15;

impl<'r> WireFormat<'r> {
  const VARINT: u32 = 0;
  const I64: u32 = 1;
  const LEN: u32 = 2;
  const SGROUP: u32 = 3;
  const EGROUP: u32 = 4;
  const I32: u32 = 5;

  pub fn new(input: &'r mut dyn io::Read) -> Self {
    Self {
      input,
      bytes: Box::new([0; BUF_LEN + SLACK_LEN]),
      read_ptr: 0,
      write_ptr: 0,

      total_read: 0,
      limits: Vec::new(),
    }
  }

  pub fn push_limit(&mut self, len: u64) -> Result {
    self
      .limits
      .push(self.total_read.checked_add(len).ok_or(Error::Overflow)?);
    Ok(())
  }

  pub fn pop_limit(&mut self) -> bool {
    if self.limits.last().copied().unwrap_or(!0) != self.total_read {
      return false;
    }

    let popped = self.limits.pop();
    debug_assert!(popped.is_some());
    true
  }

  fn refresh(&mut self) -> Result {
    if self.read_ptr > BUF_LEN / 2 {
      unsafe {
        self.bytes.as_mut_ptr().copy_from_nonoverlapping(
          self.bytes.as_mut_ptr().add(self.read_ptr),
          self.write_ptr - self.read_ptr,
        );
      }
      self.write_ptr -= self.read_ptr;
      self.read_ptr = 0;
    }

    if self.write_ptr == BUF_LEN {
      return Ok(());
    }

    let bytes_read = self.input.read(&mut self.bytes[self.write_ptr..])?;
    self.write_ptr += bytes_read;
    Ok(())
  }

  fn buf(&self) -> &[u8] {
    &self.bytes[self.read_ptr..self.write_ptr]
  }

  fn consume(&mut self, len: usize) {
    self.read_ptr += len;
    self.total_read += len as u64;
  }

  pub fn has_buf(&self) -> bool {
    self.read_ptr < self.write_ptr
  }

  pub fn is_done(&mut self) -> Result<bool> {
    if self.has_buf() {
      return Ok(false);
    }
    self.refresh()?;
    Ok(!self.has_buf())
  }

  pub fn read(&mut self, mut buf: &mut [u8]) -> Result {
    if let Some(&limit) = self.limits.last() {
      let expected_new_len = self
        .total_read
        .checked_add(buf.len() as u64)
        .ok_or(Error::Overflow)?;

      if expected_new_len > limit {
        return eof();
      }
    }

    let to_read = usize::min(buf.len(), self.buf().len());
    unsafe {
      buf
        .as_mut_ptr()
        .copy_from_nonoverlapping(self.buf().as_ptr(), to_read);
    }
    self.consume(to_read);
    buf = &mut buf[to_read..];

    if buf.is_empty() {
      return Ok(());
    }

    // If we make it in here, we do not have any buffered data, so we will
    // simply read directly from the underlying stream instead.
    debug_assert!(self.read_ptr == self.write_ptr);
    self.input.read_exact(buf)?;
    Ok(())
  }

  fn varint_inner<T, const LEN: usize>(&mut self) -> Result<T>
  where
    T: Default + From<u8> + BitOr<Output = T> + Shl<u32, Output = T>,
  {
    match self.buf() {
      [b, ..] if (*b as i8) >= 0 => {
        let v = (*b).into();
        self.consume(1);
        Ok(v)
      }
      [b1, b2, ..] if (*b2 as i8) >= 0 => {
        let v = T::from(*b1 & 0x7f) | T::from(*b2) << 7;
        self.consume(2);
        Ok(v)
      }
      _ => {
        while self.buf().len() < LEN {
          if self.is_done()? {
            break;
          }
        }
        let data = unsafe { self.buf().as_ptr().cast::<[u8; LEN]>().read() };

        let mut varint = T::default();
        let mut mask = 0x80; // Set to 0xff after we see a zero high bit.
        let mut shift = 0; // Equal to 7 * iteration index.
        let mut count = 0; // Equal to the number of iterations where `mask`
                           // was not 0xff, which gives the number of bytes
                           // we actually consumed.

        for b in data {
          varint = varint | T::from(b & !mask) << shift;
          shift += 7;
          count += !mask & 1;
          mask |= ((!b as i8) >> 7) as u8;
        }
        if mask != !0 {
          return Err(Error::OverlongVarint);
        }

        self.consume(count as usize);
        Ok(varint)
      }
    }
  }

  fn varint32(&mut self) -> Result<u32> {
    self.varint_inner::<u32, 5>()
  }

  fn varint64(&mut self) -> Result<u64> {
    self.varint_inner::<u64, 10>()
  }

  fn tag(&mut self) -> Result<Tag> {
    let tag = self.varint32()?;
    Ok(Tag {
      wire_type: tag & 0b111,
      number: tag >> 3,
    })
  }

  fn fixed32(&mut self) -> Result<u32> {
    if cfg!(target_endian = "little") && self.buf().len() >= 4 {
      let v = unsafe { self.buf().as_ptr().cast::<u32>().read_unaligned() };
      self.consume(4);
      return Ok(v);
    }

    let mut data = [0; 4];
    self.read(&mut data)?;
    Ok(u32::from_le_bytes(data))
  }

  fn fixed64(&mut self) -> Result<u64> {
    if cfg!(target_endian = "little") && self.buf().len() >= 8 {
      let v = unsafe { self.buf().as_ptr().cast::<u64>().read_unaligned() };
      self.consume(8);
      return Ok(v);
    }

    let mut data = [0; 8];
    self.read(&mut data)?;
    Ok(u64::from_le_bytes(data))
  }
}

pub struct Context<'r> {
  input: WireFormat<'r>,
  arena: RawArena,

  // NOTE: 0 means we're inside of a type 2 record, and thus an end group
  // tag is invalid.
  group_stack: Vec<u32>,
}

pub enum Limit {
  Buf(usize),
  Group(u32),
}

impl<'r> Context<'r> {
  pub fn new(input: &'r mut dyn io::Read, arena: RawArena) -> Self {
    Self {
      input: WireFormat::new(input),
      arena,
      group_stack: Vec::new(),
    }
  }

  fn in_group(&self) -> bool {
    self.group_stack.last().copied().unwrap_or(0) != 0
  }

  pub fn parse(&mut self, raw: *mut u8, desc: Desc) -> Result {
    let mut ty_stack = vec![(raw, desc.first_field())];
    'parse: while let Some((raw, field)) = ty_stack.last_mut() {
      if !self.in_group() && self.input.pop_limit() {
        ty_stack.pop();
        self.group_stack.pop();
        continue;
      }

      if self.input.is_done()? {
        if ty_stack.len() == 1 {
          break;
        }
        return eof();
      }

      if self.input.buf().starts_with(&[0]) {
        if ty_stack.len() == 1 {
          break;
        } else {
          let tag = self.input.tag()?;
          self.skip(tag)?;
          continue;
        }
      }

      let tag = self.input.tag()?;
      if tag.wire_type == WireFormat::EGROUP
        && self.group_stack.last().copied().unwrap_or(0) == tag.number
      {
        ty_stack.pop();
        continue;
      }

      let Some(field) = field.as_mut() else {
        self.skip(tag)?;
        continue;
      };

      let start = *field;
      loop {
        let number = field.number();
        if number == tag.number {
          break;
        }

        *field = field.next();

        if *field == start {
          self.skip(tag)?;
          continue 'parse;
        }
      }

      unsafe {
        field.init(*raw, self.arena);
      }

      // Here, `field` now points to the field corresponding to the current
      // message.
      match field.kind() {
        k @ (Kind::I32 | Kind::I64 | Kind::F32 | Kind::F64 | Kind::Bool) => {
          if matches!(k, Kind::F32) && tag.wire_type != WireFormat::I32 {
            self.skip(tag)?;
            continue;
          }

          if matches!(k, Kind::F64) && tag.wire_type != WireFormat::I64 {
            self.skip(tag)?;
            continue;
          }

          let v = match tag.wire_type {
            WireFormat::VARINT => self.input.varint64()?,
            WireFormat::I32 => self.input.fixed32()? as u64,
            WireFormat::I64 => self.input.fixed64()?,
            _ => {
              self.skip(tag)?;
              continue;
            }
          };
          match (k, field.is_repeated()) {
            (Kind::I32 | Kind::F32, false) => unsafe {
              // Singular i32.
              let ptr = raw.add(field.offset());
              ptr.cast::<u32>().write(v as u32);
            },
            (Kind::I64 | Kind::F64, false) => unsafe {
              // Singular i64.
              let ptr = raw.add(field.offset());
              ptr.cast::<u64>().write(v);
            },
            (Kind::Bool, false) => unsafe {
              // Singular bool.
              let ptr = raw.add(field.offset());
              ptr.cast::<bool>().write(v != 0);
            },

            (Kind::I32 | Kind::F32, true) => unsafe {
              // Repeated i32.
              let ptr = raw.add(field.offset());
              let vec = &mut *ptr.cast::<AVec<u32>>();
              vec.resize(vec.len() + 1, self.arena);
              *vec.as_mut_slice().last_mut().unwrap_unchecked() = v as u32;
            },
            (Kind::I64 | Kind::F64, true) => unsafe {
              // Repeated i64.
              let ptr = raw.add(field.offset());
              let vec = &mut *ptr.cast::<AVec<u64>>();
              vec.resize(vec.len() + 1, self.arena);
              *vec.as_mut_slice().last_mut().unwrap_unchecked() = v;
            },
            (Kind::Bool, true) => unsafe {
              // Repeated bool.
              let ptr = raw.add(field.offset());
              let vec = &mut *ptr.cast::<AVec<bool>>();
              vec.resize(vec.len() + 1, self.arena);
              *vec.as_mut_slice().last_mut().unwrap_unchecked() = v != 0;
            },
            _ => unreachable!(),
          }
        }
        Kind::Str => {
          if tag.wire_type != WireFormat::LEN {
            self.skip(tag)?;
            continue;
          }

          let len = self.input.varint32()? as usize;
          let (slice_ptr, slice_len): &mut (*mut u8, usize) =
            if !field.is_repeated() {
              // Singular str.
              unsafe {
                let ptr = raw.add(field.offset());
                &mut *ptr.cast::<(*mut u8, usize)>()
              }
            } else {
              // Repeated str.
              unsafe {
                let ptr = raw.add(field.offset());
                let vec = &mut *ptr.cast::<AVec<(*mut u8, usize)>>();
                vec.resize(vec.len() + 1, self.arena);
                vec.as_mut_slice().last_mut().unwrap_unchecked()
              }
            };

          if len == 0 {
            *slice_len = 0;
          } else {
            unsafe {
              if *slice_len < len {
                *slice_ptr = self
                  .arena
                  .alloc(Layout::from_size_align_unchecked(len, 1))
                  .as_ptr();
                slice_ptr.write_bytes(0, len);
              }
              self
                .input
                .read(slice::from_raw_parts_mut(*slice_ptr, len))?;
              *slice_len = len;
            }
          }
        }
        Kind::Type => {
          match tag.wire_type {
            WireFormat::LEN => {
              let len = self.input.varint32()?;
              self.input.push_limit(len as u64)?;
              self.group_stack.push(0);
            }
            WireFormat::SGROUP => {
              self.group_stack.push(tag.number);
            }
            _ => {
              self.skip(tag)?;
              continue;
            }
          }

          let desc = unsafe { field.desc() };
          let layout = desc.layout();
          let raw_ptr: *mut u8 = if !field.is_repeated() {
            // Singular msg.
            unsafe {
              let ptr = raw.add(field.offset()).cast::<*mut u8>();
              let mut sub = ptr.read();
              if sub.is_null() {
                sub = self.arena.alloc(layout).as_ptr();
                sub.write_bytes(0, layout.size());
                ptr.write(sub);
              }
              sub
            }
          } else {
            // Repeated msg.
            unsafe {
              let ptr = raw.add(field.offset());
              let vec = &mut *ptr.cast::<AVec<*mut u8>>();

              vec.resize_msg(vec.len() + 1, self.arena, layout);
              *vec.as_mut_slice().last_mut().unwrap_unchecked()
            }
          };

          ty_stack.push((raw_ptr, desc.first_field()));
        }
      }
    }

    Ok(())
  }

  fn skip(&mut self, mut tag: Tag) -> Result {
    // TODO: This should dump into unknown fields, instead.

    let mut buf = Vec::new();
    let group_watermark = self.group_stack.len();
    loop {
      match tag.wire_type {
        WireFormat::VARINT => {
          self.input.varint64()?;
        }
        WireFormat::I32 => {
          self.input.fixed32()?;
        }
        WireFormat::I64 => {
          self.input.fixed64()?;
        }
        WireFormat::LEN => {
          let len = self.input.varint32()? as usize;
          if buf.len() < len {
            buf.resize(len, 0);
          }
          self.input.read(&mut buf[..len])?
        }
        WireFormat::SGROUP => {
          self.group_stack.push(tag.number);
        }
        WireFormat::EGROUP => {
          if self.group_stack.len() == group_watermark
            || *self.group_stack.last().unwrap() != tag.number
          {
            return Err(Error::BadTag);
          }

          self.group_stack.pop();
        }
        _ => return Err(Error::BadTag),
      }

      if group_watermark == self.group_stack.len() {
        return Ok(());
      }

      tag = self.input.tag()?;
      if tag.number == 0 {
        return Err(Error::BadTag);
      }
    }
  }
}

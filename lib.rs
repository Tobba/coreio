#![no_std]
#![feature(no_std)]
#![feature(core, core_prelude, core_slice_ext, slice_bytes)]

#[macro_use]
extern crate core;

use core::prelude::*;
use core::cmp::min;
use core::slice::bytes::copy_memory;

#[macro_export]
macro_rules! try {
	($expr:expr) => (match $expr {
		core::result::Result::Ok(val) => val,
		core::result::Result::Err(err) => {
			return core::result::Result::Err(core::convert::From::from(err))
		}
	})
}

#[derive(Copy, Clone, Debug)]
pub struct EndOfFile;

#[derive(Copy, Clone, Debug)]
pub struct OutOfBounds;

pub trait Read {
	type Err;

	fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Err>;

	fn read_all<E=<Self as Read>::Err>(&mut self, buf: &mut [u8]) -> Result<(), E> where E: From<Self::Err> + From<EndOfFile> {
		let mut offset = 0;
		while offset < buf.len() {
			match try!(self.read(&mut buf[offset..])) {
				0 => return Err(E::from(EndOfFile)),
				n => offset += n
			}
		}
		Ok(())
	}
}

pub trait Write {
	type Err;

	fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Err>;

	fn write_all<E=<Self as Write>::Err>(&mut self, mut buf: &[u8]) -> Result<(), E> where E: From<Self::Err> + From<EndOfFile> {
		while buf.len() > 0 {
			match try!(self.write(buf)) {
				0 => return Err(E::from(EndOfFile)),
				n => buf = &buf[n..]
			}
		}
		Ok(())
	}
}

pub enum SeekFrom {
	Start(u64),
	End(i64),
	Current(i64),
}

pub trait Seek {
	type Err;

	fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Err>;

	fn tell(&mut self) -> Result<u64, Self::Err> {
		self.seek(SeekFrom::Current(0))
	}
}

impl<'a> Read for &'a [u8] {
	type Err = EndOfFile;

	fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Err> {
		let len = min(buf.len(), self.len());
		if len == 0 {
			return Err(EndOfFile)
		}
		let (a, b) = self.split_at(len);
		copy_memory(a, buf);
		*self = b;
		Ok(len)
	}
}

pub struct Cursor<T> {
	inner: T,
	pos: u64
}

impl<T> Cursor<T> {
	pub fn new(inner: T) -> Cursor<T> {
		Cursor {
			inner: inner,
			pos: 0
		}
	}
}

impl<'a> Read for Cursor<&'a [u8]> {
	type Err = EndOfFile;

	fn read(&mut self, buf: &mut [u8]) -> Result<usize, EndOfFile> {
		let len = try!((&self.inner[self.pos as usize..]).read(buf));
		self.pos += len as u64;
		Ok(len)
	}
}

impl<'a> Seek for Cursor<&'a [u8]> {
	type Err = OutOfBounds;

	fn seek(&mut self, from: SeekFrom) -> Result<u64, OutOfBounds> {
		let pos = match from {
			SeekFrom::Start(offset) => offset as i64,
			SeekFrom::End(offset) => self.inner.len() as i64 + offset as i64,
			SeekFrom::Current(offset) => self.pos as i64 + offset
		};
		if pos < 0 {
			return Err(OutOfBounds);
		}
		self.pos = min(pos as u64, self.inner.len() as u64);
		Ok(self.pos)
	}
}

impl<'a, R: Read> Read for &'a mut R {
	type Err = R::Err;

	fn read(&mut self, buf: &mut [u8]) -> Result<usize, R::Err> {
		(**self).read(buf)
	}

	fn read_all<E=<Self as Read>::Err>(&mut self, buf: &mut [u8]) -> Result<(), E> where E: From<R::Err> + From<EndOfFile> {
		(**self).read_all(buf)
	}
}

impl<'a, W: Write> Write for &'a mut W {
	type Err = W::Err;

	fn write(&mut self, buf: &[u8]) -> Result<usize, W::Err> {
		(**self).write(buf)
	}

	fn write_all<E=<Self as Write>::Err>(&mut self, buf: &[u8]) -> Result<(), E> where E: From<W::Err> + From<EndOfFile> {
		(**self).write_all(buf)
	}
}

impl<'a, S: Seek> Seek for &'a mut S {
	type Err = S::Err;

	fn seek(&mut self, pos: SeekFrom) -> Result<u64, S::Err> {
		(**self).seek(pos)
	}

	fn tell(&mut self) -> Result<u64, S::Err> {
		(**self).tell()
	}
}

#![no_std]
#![feature(no_std)]
#![feature(core, core_prelude, core_slice_ext, slice_bytes)]

#[macro_use]
extern crate core;
extern crate void;

use core::prelude::*;

pub mod slice;
pub mod cursor;
pub mod wrapper;

#[macro_export]
macro_rules! try {
	($expr:expr) => (match $expr {
		::core::result::Result::Ok(val) => val,
		::core::result::Result::Err(err) => {
			return ::core::result::Result::Err(::core::convert::From::from(err))
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

	fn read_all<E=<Self as Read>::Err>(&mut self, mut buf: &mut [u8]) -> Result<(), E>
		where E: From<Self::Err> + From<EndOfFile>
	{
		while buf.len() > 0 {
			match try!(self.read(&mut buf)) {
				0 => return Err(E::from(EndOfFile)),
				n => {
					let tmp = buf;
					buf = &mut tmp[n..]
				}
			}
		}
		Ok(())
	}
}

pub trait Write {
	type Err;

	fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Err>;

	fn write_all<E=<Self as Write>::Err>(&mut self, mut buf: &[u8]) -> Result<(), E>
		where E: From<Self::Err> + From<EndOfFile>
	{
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

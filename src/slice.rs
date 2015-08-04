use core::prelude::*;
use core::cmp::min;
use core::mem::{replace, uninitialized};
use core::slice::bytes::copy_memory;

use void::{Void, ResultVoidExt};

use super::*;

impl<'a> Read for &'a [u8] {
	type Err = Void;

	fn read(&mut self, buf: &mut [u8]) -> Result<usize, Void> {
		let len = min(buf.len(), self.len());
		let (a, b) = self.split_at(len);
		copy_memory(a, buf);
		*self = b;
		Ok(len)
	}

	fn read_all<E=EndOfFile>(&mut self, buf: &mut [u8]) -> Result<(), E>
		where E: From<Void> + From<EndOfFile>
	{
		if buf.len() < self.len() {
			Err(E::from(EndOfFile))
		} else {
			self.read(buf).void_unwrap();
			Ok(())
		}
	}

}

impl<'a> Write for &'a mut [u8] {
	type Err = Void;

	fn write(&mut self, buf: &[u8]) -> Result<usize, Void> {
		let len = min(buf.len(), self.len());

		let mut tmp = replace(self, unsafe { uninitialized() });
		let (a, b) = tmp.split_at_mut(len);

		copy_memory(buf, a);
		*self = b;
		Ok(len)
	}

	fn write_all<E=Void>(&mut self, buf: &[u8]) -> Result<(), E>
		where E: From<Void> + From<EndOfFile>
	{
		if self.len() < buf.len()  {
			Err(E::from(EndOfFile))
		} else {
			self.write(buf).void_unwrap();
			Ok(())
		}
	}
}

// luau-rs - Rust bindings to Roblox's Luau
// Copyright (C) 2021 LoganDark
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of version 3 of the GNU General Public License as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::ptr::{addr_of, NonNull};

use luau_sys::glue::{gluau_newthread, gluauL_sandboxthread};
use luau_sys::luau::{lua_Status, luau_load};

use crate::compiler::CompiledFunction;
use crate::vm::error::{LError, LResult};
use crate::vm::raw::thread::RawThread;
use crate::vm::raw::value::RawValue;
use crate::vm::value::buffer::Buffer;
use crate::vm::value::closure::Closure;
use crate::vm::value::gc::{Datatype, LuauRef};
use crate::vm::value::LuauValue;
use crate::vm::value::string::LString;
use crate::vm::value::table::Table;

#[derive(Debug)]
#[repr(transparent)]
pub struct Thread<'a>(&'a RawThread);

unsafe impl<'a> Datatype<'a> for Thread<'a> {
	type Ref = LuauRef<'a>;

	fn acquire_ref(&self, thread: &'a Thread<'a>) -> LResult<'a, Self::Ref> {
		unsafe { LuauRef::new(thread, RawValue::new_thread(NonNull::from(self.0))) }
	}

	fn raw_value(&self) -> RawValue { unsafe { RawValue::new_thread(NonNull::from(self.0)) } }
}

impl<'a> Thread<'a> {
	pub unsafe fn from_raw(raw: &'a RawThread) -> Self { Self(raw) }
	pub fn raw(&self) -> &'a RawThread { self.0 }

	pub unsafe fn new(parent: &'a Thread<'a>) -> LResult<'a, Self> {
		LError::protect(parent, false, move |result: *mut Self| {
			gluau_newthread(parent.raw().ptr(), result.cast())
		})
	}

	pub fn new_string(&self, data: impl AsRef<[u8]>) -> LResult<LuauValue<LString>> {
		LuauValue::new(self, unsafe { LString::new(self, data.as_ref()) }?)
	}

	pub fn new_table(&self, narray: usize, lnhash: usize) -> LResult<LuauValue<Table>> {
		LuauValue::new(self, unsafe { Table::new(self, narray, lnhash) }?)
	}

	pub fn new_closure(&self, bytecode: CompiledFunction) -> LResult<LuauValue<Closure>> {
		unsafe {
			let closure = LError::protect(self, true, move |result| {
				let bytecode = bytecode.as_ref();

				if luau_load(self.raw().ptr(), b"=lua\0".as_ptr().cast(), bytecode.as_ptr().cast(), bytecode.len(), 0) == 0 {
					let value = self.raw().stack().pop().unwrap();
					*result = Closure::from_raw(addr_of!(value.data().closure).read_unaligned().as_ref());
					lua_Status::LUA_OK
				} else {
					lua_Status::LUA_ERRRUN
				}
			})?;

			LuauValue::new(self, closure)
		}
	}

	pub fn new_thread(&self) -> LResult<LuauValue<Thread>> {
		unsafe {
			let new_thread = Thread::new(self)?;
			LError::protect(self, false, |_result: *mut ()| gluauL_sandboxthread(new_thread.raw().ptr()))?;
			LuauValue::new(self, new_thread)
		}
	}

	pub fn new_buffer(&self, len: usize) -> LResult<LuauValue<Buffer>> {
		LuauValue::new(self, unsafe { Buffer::new(self, len) }?)
	}
}

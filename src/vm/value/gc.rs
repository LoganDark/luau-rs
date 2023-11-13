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

use std::ffi::c_int;

use luau_sys::glue::gluau_ref;
use luau_sys::luau::{lua_unref, luaH_getnum};

use crate::vm::error::{LError, LResult};
use crate::vm::raw::value::RawValue;
use crate::vm::value::thread::Thread;

#[derive(Debug)]
pub struct LuauRef<'a> {
	thread: &'a Thread<'a>,
	handle: c_int
}

impl<'a> LuauRef<'a> {
	pub unsafe fn new(thread: &'a Thread<'a>, value: RawValue) -> LResult<'a, Self> {
		thread.raw().stack().push(value).ok_or(LError::StackOverflow)?;

		LError::protect(thread, false, move |handle| {
			gluau_ref(thread.raw().ptr(), -1, handle)
		}).map(|handle| Self { thread, handle })
	}

	pub unsafe fn get(&self) -> RawValue {
		let registry = self.thread.raw().registry();
		let slot = luaH_getnum(registry.as_ptr().cast(), self.handle);
		RawValue::from(*slot)
	}
}

impl<'a> Drop for LuauRef<'a> {
	fn drop(&mut self) {
		unsafe { lua_unref(self.thread.raw().ptr(), self.handle); }
	}
}

impl<'a> Clone for LuauRef<'a> {
	fn clone(&self) -> Self {
		unsafe { Self::new(self.thread, self.get()) }.expect("failed to clone LuauRef")
	}
}

pub unsafe trait Datatype<'a> {
	type Ref;
	fn acquire_ref(&self, thread: &'a Thread<'a>) -> LResult<'a, Self::Ref>;
	fn raw_value(&self) -> RawValue;
}

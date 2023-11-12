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
use luau_sys::luau::{LUA_NOREF, lua_Status, lua_unref, luaH_getnum};

use crate::vm::raw::thread::RawThread;
use crate::vm::raw::value::RawValue;
use crate::vm::value::thread::Thread;

#[derive(Debug)]
pub struct LuauRef<'a> {
	thread: &'a RawThread,
	handle: c_int
}

impl<'a> LuauRef<'a> {
	pub unsafe fn new(thread: &'a RawThread, value: RawValue) -> Option<Self> {
		thread.stack().save_restore(move |stack| {
			stack.push(value.into_tvalue());

			let mut handle = LUA_NOREF;
			match gluau_ref(thread.ptr(), -1, &mut handle) {
				lua_Status::LUA_OK => Some(Self { thread, handle }),
				_ => None
			}
		})
	}

	pub unsafe fn get(&self) -> RawValue {
		let registry = self.thread.registry();
		let slot = luaH_getnum(registry.as_ptr().cast(), self.handle);
		RawValue::from_tvalue(*slot)
	}
}

impl<'a> Drop for LuauRef<'a> {
	fn drop(&mut self) {
		unsafe { lua_unref(self.thread.ptr(), self.handle); }
	}
}

impl<'a> Clone for LuauRef<'a> {
	fn clone(&self) -> Self {
		unsafe { Self::new(self.thread, self.get()) }.expect("failed to clone LuauRef")
	}
}

pub trait Datatype<'a> {
	type Ref;
	fn acquire_ref(&self, thread: Thread<'a>) -> Option<Self::Ref>;
}

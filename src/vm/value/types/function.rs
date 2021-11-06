// luau-rs - Rust bindings to Roblox's Luau
// Copyright (C) 2021 LoganDark
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::ffi::CString;

use luau_sys::luau::{lua_tolstring, luau_load, luaV_tostring};

use crate::compiler::Chunk;
use crate::vm::{Error, Luau, Thread, ThreadUserdata, Value};

pub struct Function<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread>(pub Value<'borrow, 'thread, 'vm, UD>);

impl<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> Function<'borrow, 'thread, 'vm, UD> {
	pub fn load(thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>, chunk: Chunk, chunkname: &str) -> Result<Self, Error> {
		unsafe {
			let cstring = CString::new(chunkname).expect("Don't pass a chunk name with an embedded null!");

			if luau_load(thread.state as _, cstring.as_ptr(), &chunk.as_ref()[0] as *const u8 as _, chunk.as_ref().len() as _, 0) == 0 {
				Ok(Self(Value::from_stack_top(thread)))
			} else {
				// error is at the top of the stack
				let mut length: std::os::raw::c_ulong = 0;
				let data = lua_tolstring(thread.state as _, -1, &mut length as _);
				Err(Error::Runtime(String::from_raw_parts(data as _, length as _, length as _)))
			}
		}
	}
}

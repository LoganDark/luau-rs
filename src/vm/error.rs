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

use bstr::{BStr, BString};

use luau_sys::luau::{lua_settop, lua_State, lua_tolstring, size_t};

#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum Error {
	/// There was a runtime error during execution. Only the error message is
	/// available. Other information (tracebacks) may be obtainable externally.
	#[error("{0}")]
	Runtime(BString),

	/// There wasn't enough stack space available to perform the requested
	/// operation.
	#[error("out of stack space")]
	OutOfStack,

	/// The called function yielded in a non-yieldable context.
	#[error("unexpected yield from a non-yieldable context")]
	Yielded
}

impl Error {
	pub(crate) unsafe fn pop_runtime_error(state: *mut lua_State) -> Self {
		// error is at the top of the stack
		let mut length: size_t = 0;
		let data = lua_tolstring(state, -1, &mut length as _);
		let message = <BString as From<&BStr>>::from(std::slice::from_raw_parts(data as *const u8, length as _).into());
		lua_settop(state, -2);
		Error::Runtime(message)
	}
}

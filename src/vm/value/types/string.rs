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

use std::str::Utf8Error;

use luau_sys::luau::TString;

use crate::vm::{Thread, ThreadUserdata, ToLuauValue, Value};

/// A value that has been verified to be a string.
pub struct StringValue<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread>(Value<'borrow, 'thread, 'vm, UD>);

impl<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> StringValue<'borrow, 'thread, 'vm, UD> {
	pub fn new(thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>, value: &str) -> Self {
		Self(Value::new_string(thread, value))
	}

	pub fn as_str(&self) -> Result<&str, Utf8Error> {
		std::str::from_utf8(self.as_ref())
	}
}

impl<'thread, UD: ThreadUserdata + 'thread> AsRef<[u8]> for StringValue<'_, 'thread, '_, UD> {
	fn as_ref(&self) -> &[u8] {
		let string = unsafe { self.0.value.value.gc as *mut TString };

		// SAFETY: While this struct is borrowed, the string is guaranteed to
		// not move or be deallocated.
		unsafe { std::slice::from_raw_parts(&(*string).data as *const std::os::raw::c_char as *const _, (*string).len as _) }
	}
}

impl<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> ToLuauValue<'borrow, 'thread, 'vm, UD> for StringValue<'borrow, 'thread, 'vm, UD> {
	fn to_luau_value(self, thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>) -> Value<'borrow, 'thread, 'vm, UD> {
		todo!()
	}
}

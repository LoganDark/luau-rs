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

use std::ptr::NonNull;

use luau_sys::glue::gluauS_newlstr;

use crate::vm::error::{LError, LResult};
use crate::vm::raw::string::RawString;
use crate::vm::raw::value::RawValue;
use crate::vm::value::gc::{Datatype, LuauRef};
use crate::vm::value::thread::Thread;

#[derive(Debug)]
#[repr(transparent)]
pub struct LString<'a>(&'a RawString);

unsafe impl<'a> Datatype<'a> for LString<'a> {
	type Ref = LuauRef<'a>;

	fn acquire_ref(&self, thread: &'a Thread<'a>) -> LResult<'a, Self::Ref> {
		unsafe { LuauRef::new(thread, RawValue::new_string(NonNull::from(self.0))) }
	}

	fn raw_value(&self) -> RawValue { unsafe { RawValue::new_string(NonNull::from(self.0)) } }
}

impl<'a> LString<'a> {
	pub unsafe fn from_raw(raw: &'a RawString) -> Self { Self(raw) }
	pub fn raw(&self) -> &'a RawString { self.0 }

	pub unsafe fn new(thread: &'a Thread<'a>, data: &[u8]) -> LResult<'a, Self> {
		LError::protect(thread, false, move |result: *mut Self| {
			gluauS_newlstr(thread.raw().ptr(), data.as_ptr().cast(), data.len(), result.cast())
		})
	}
}

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

use luau_sys::glue::gluauH_new;

use crate::vm::error::{LError, LResult};
use crate::vm::raw::table::RawTable;
use crate::vm::raw::value::RawValue;
use crate::vm::value::gc::{Datatype, LuauRef};
use crate::vm::value::thread::Thread;

#[derive(Debug)]
#[repr(transparent)]
pub struct Table<'a>(&'a RawTable);

unsafe impl<'a> Datatype<'a> for Table<'a> {
	type Ref = LuauRef<'a>;

	fn acquire_ref(&self, thread: &'a Thread<'a>) -> LResult<'a, Self::Ref> {
		unsafe { LuauRef::new(thread, RawValue::new_table(NonNull::from(self.0))) }
	}

	fn raw_value(&self) -> RawValue { unsafe { RawValue::new_table(NonNull::from(self.0)) } }
}

impl<'a> Table<'a> {
	pub unsafe fn from_raw(raw: &'a RawTable) -> Self { Self(raw) }
	pub fn raw(&self) -> &'a RawTable { self.0 }

	pub unsafe fn new(thread: &'a Thread<'a>, narray: usize, lnhash: usize) -> LResult<'a, Self> {
		LError::protect(thread, false, move |result: *mut Self| {
			gluauH_new(thread.raw().ptr(), narray as _, lnhash as _, result.cast())
		})
	}
}

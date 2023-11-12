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

use crate::vm::raw::table::RawTable;
use crate::vm::raw::value::RawValue;
use crate::vm::value::gc::{Datatype, LuauRef};
use crate::vm::value::thread::Thread;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Table<'a>(&'a RawTable);

impl<'a> Datatype<'a> for Table<'a> {
	type Ref = LuauRef<'a>;

	fn acquire_ref(&self, thread: Thread<'a>) -> Option<Self::Ref> {
		unsafe { LuauRef::new(thread.raw(), RawValue::new_table(NonNull::from(self.0))) }
	}
}

impl<'a> Table<'a> {
	pub unsafe fn from_raw(raw: &'a RawTable) -> Self { Self(raw) }
	pub fn raw(&self) -> &'a RawTable { self.0 }
}

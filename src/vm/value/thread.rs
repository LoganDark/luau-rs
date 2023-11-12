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

use crate::vm::raw::thread::RawThread;
use crate::vm::raw::value::RawValue;
use crate::vm::value::gc::{Datatype, LuauRef};

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Thread<'a>(&'a RawThread);

impl<'a> Datatype<'a> for Thread<'a> {
	type Ref = LuauRef<'a>;

	fn acquire_ref(&self, thread: Thread<'a>) -> Option<Self::Ref> {
		unsafe { LuauRef::new(thread.raw(), RawValue::new_thread(NonNull::from(self.0))) }
	}
}

impl<'a> Thread<'a> {
	pub unsafe fn from_raw(raw: &'a RawThread) -> Self { Self(raw) }
	pub fn raw(&self) -> &'a RawThread { self.0 }
}

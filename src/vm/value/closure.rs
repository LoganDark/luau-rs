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

use crate::vm::error::LResult;
use crate::vm::raw::closure::RawClosure;
use crate::vm::raw::value::RawValue;
use crate::vm::value::gc::{Datatype, LuauRef};
use crate::vm::value::thread::Thread;

#[derive(Debug)]
#[repr(transparent)]
pub struct Closure<'a>(&'a RawClosure);

unsafe impl<'a> Datatype<'a> for Closure<'a> {
	type Ref = LuauRef<'a>;

	fn acquire_ref(&self, thread: &'a Thread<'a>) -> LResult<'a, Self::Ref> {
		unsafe { LuauRef::new(thread, RawValue::new_closure(NonNull::from(self.0))) }
	}

	fn raw_value(&self) -> RawValue { unsafe { RawValue::new_closure(NonNull::from(self.0)) } }
}

impl<'a> Closure<'a> {
	pub unsafe fn from_raw(raw: &'a RawClosure) -> Self { Self(raw) }
	pub fn raw(&self) -> &'a RawClosure { self.0 }
}

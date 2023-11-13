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

use std::ops::Deref;

use crate::vm::error::LResult;
use crate::vm::raw::value::RawValue;
use crate::vm::value::boolean::Boolean;
use crate::vm::value::buffer::Buffer;
use crate::vm::value::closure::Closure;
use crate::vm::value::dynamic::Dynamic;
use crate::vm::value::gc::Datatype;
use crate::vm::value::lightuserdata::LightUserdata;
use crate::vm::value::nil::Nil;
use crate::vm::value::number::Number;
use crate::vm::value::string::LString;
use crate::vm::value::table::Table;
use crate::vm::value::thread::Thread;
use crate::vm::value::userdata::Userdata;
use crate::vm::value::vector::Vector;

pub mod gc;
pub mod nil;
pub mod boolean;
pub mod lightuserdata;
pub mod number;
pub mod vector;
pub mod string;
pub mod table;
pub mod closure;
pub mod userdata;
pub mod thread;
pub mod buffer;
pub mod dynamic;

#[derive(Debug)]
pub struct LuauValue<'a, T: Datatype<'a>> {
	handle: T::Ref,
	inner: T
}

impl<'a, T: Datatype<'a>> Deref for LuauValue<'a, T> {
	type Target = T;
	fn deref(&self) -> &Self::Target { &self.inner }
}

impl<'a, T: Datatype<'a>> LuauValue<'a, T> {
	pub fn new(thread: &'a Thread<'a>, value: T) -> LResult<'a, Self> {
		Ok(Self { handle: value.acquire_ref(thread)?, inner: value })
	}
}

impl<'a> LuauValue<'a, Dynamic<'a>> {
	pub unsafe fn from_raw(thread: &'a Thread<'a>, value: RawValue) -> LResult<'a, Self> {
		Self::new(thread, Dynamic::from_raw(value))
	}

	pub unsafe fn pop(thread: &'a Thread<'a>) -> Option<LResult<'a, Self>> {
		thread.raw().stack().pop().map(move |value| Self::from_raw(thread, value))
	}

	pub fn get_nil(&self, thread: &'a Thread<'a>) -> Option<LResult<'a, LuauValue<'a, Nil>>> { self.inner.get_nil().map(|inner| LuauValue::new(thread, inner)) }
	pub fn get_boolean(&self, thread: &'a Thread<'a>) -> Option<LResult<'a, LuauValue<'a, Boolean>>> { self.inner.get_boolean().map(|inner| LuauValue::new(thread, inner)) }
	pub fn get_lightuserdata(&self, thread: &'a Thread<'a>) -> Option<LResult<'a, LuauValue<'a, LightUserdata>>> { self.inner.get_lightuserdata().map(|inner| LuauValue::new(thread, inner)) }
	pub fn get_number(&self, thread: &'a Thread<'a>) -> Option<LResult<'a, LuauValue<'a, Number>>> { self.inner.get_number().map(|inner| LuauValue::new(thread, inner)) }
	pub fn get_vector(&self, thread: &'a Thread<'a>) -> Option<LResult<'a, LuauValue<'a, Vector>>> { self.inner.get_vector().map(|inner| LuauValue::new(thread, inner)) }
	pub fn get_string(&self, thread: &'a Thread<'a>) -> Option<LResult<'a, LuauValue<'a, LString<'a>>>> { self.inner.get_string().map(|inner| LuauValue::new(thread, inner)) }
	pub fn get_table(&self, thread: &'a Thread<'a>) -> Option<LResult<'a, LuauValue<'a, Table<'a>>>> { self.inner.get_table().map(|inner| LuauValue::new(thread, inner)) }
	pub fn get_closure(&self, thread: &'a Thread<'a>) -> Option<LResult<'a, LuauValue<'a, Closure<'a>>>> { self.inner.get_closure().map(|inner| LuauValue::new(thread, inner)) }
	pub fn get_userdata(&self, thread: &'a Thread<'a>) -> Option<LResult<'a, LuauValue<'a, Userdata<'a>>>> { self.inner.get_userdata().map(|inner| LuauValue::new(thread, inner)) }
	pub fn get_thread(&self, thread: &'a Thread<'a>) -> Option<LResult<'a, LuauValue<'a, Thread<'a>>>> { self.inner.get_thread().map(|inner| LuauValue::new(thread, inner)) }
	pub fn get_buffer(&self, thread: &'a Thread<'a>) -> Option<LResult<'a, LuauValue<'a, Buffer<'a>>>> { self.inner.get_buffer().map(|inner| LuauValue::new(thread, inner)) }
}

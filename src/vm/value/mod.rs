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
	pub fn new(thread: Thread<'a>, value: T) -> Option<Self> {
		Some(Self { handle: value.acquire_ref(thread)?, inner: value })
	}
}

impl<'a> LuauValue<'a, Dynamic<'a>> {
	pub fn get_nil(&self, thread: Thread<'a>) -> Option<LuauValue<Nil>> { self.inner.get_nil().and_then(|inner| LuauValue::new(thread, inner)) }
	pub fn get_boolean(&self, thread: Thread<'a>) -> Option<LuauValue<Boolean>> { self.inner.get_boolean().and_then(|inner| LuauValue::new(thread, inner)) }
	pub fn get_lightuserdata(&self, thread: Thread<'a>) -> Option<LuauValue<LightUserdata>> { self.inner.get_lightuserdata().and_then(|inner| LuauValue::new(thread, inner)) }
	pub fn get_number(&self, thread: Thread<'a>) -> Option<LuauValue<Number>> { self.inner.get_number().and_then(|inner| LuauValue::new(thread, inner)) }
	pub fn get_vector(&self, thread: Thread<'a>) -> Option<LuauValue<Vector>> { self.inner.get_vector().and_then(|inner| LuauValue::new(thread, inner)) }
	pub fn get_string(&self, thread: Thread<'a>) -> Option<LuauValue<LString>> { self.inner.get_string().and_then(|inner| LuauValue::new(thread, inner)) }
	pub fn get_table(&self, thread: Thread<'a>) -> Option<LuauValue<Table>> { self.inner.get_table().and_then(|inner| LuauValue::new(thread, inner)) }
	pub fn get_closure(&self, thread: Thread<'a>) -> Option<LuauValue<Closure>> { self.inner.get_closure().and_then(|inner| LuauValue::new(thread, inner)) }
	pub fn get_userdata(&self, thread: Thread<'a>) -> Option<LuauValue<Userdata>> { self.inner.get_userdata().and_then(|inner| LuauValue::new(thread, inner)) }
	pub fn get_thread(&self, thread: Thread<'a>) -> Option<LuauValue<Thread>> { self.inner.get_thread().and_then(|inner| LuauValue::new(thread, inner)) }
	pub fn get_buffer(&self, thread: Thread<'a>) -> Option<LuauValue<Buffer>> { self.inner.get_buffer().and_then(|inner| LuauValue::new(thread, inner)) }
}

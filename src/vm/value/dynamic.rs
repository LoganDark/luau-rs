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

use std::ptr::addr_of;
use crate::vm::error::LResult;
use crate::vm::raw::value::{RawValue, RawValueTag};
use crate::vm::value::boolean::Boolean;
use crate::vm::value::buffer::Buffer;
use crate::vm::value::closure::Closure;
use crate::vm::value::gc::{Datatype, LuauRef};
use crate::vm::value::lightuserdata::LightUserdata;
use crate::vm::value::nil::Nil;
use crate::vm::value::number::Number;
use crate::vm::value::string::LString;
use crate::vm::value::table::Table;
use crate::vm::value::thread::Thread;
use crate::vm::value::userdata::Userdata;
use crate::vm::value::vector::Vector;

#[derive(Debug)]
pub enum Dynamic<'a> {
	Nil(Nil),
	Boolean(Boolean),
	LightUserdata(LightUserdata),
	Number(Number),
	Vector(Vector),
	String(LString<'a>),
	Table(Table<'a>),
	Closure(Closure<'a>),
	Userdata(Userdata<'a>),
	Thread(Thread<'a>),
	Buffer(Buffer<'a>)
}

unsafe impl<'a> Datatype<'a> for Dynamic<'a> {
	type Ref = Option<LuauRef<'a>>;

	fn acquire_ref(&self, thread: &'a Thread<'a>) -> LResult<'a, Self::Ref> {
		Ok(match self {
			Self::Nil(inner) => Some(inner.acquire_ref(thread)?).and(None),
			Self::Boolean(inner) => Some(inner.acquire_ref(thread)?).and(None),
			Self::LightUserdata(inner) => Some(inner.acquire_ref(thread)?).and(None),
			Self::Number(inner) => Some(inner.acquire_ref(thread)?).and(None),
			Self::Vector(inner) => Some(inner.acquire_ref(thread)?).and(None),
			Self::String(inner) => Some(inner.acquire_ref(thread)?),
			Self::Table(inner) => Some(inner.acquire_ref(thread)?),
			Self::Closure(inner) => Some(inner.acquire_ref(thread)?),
			Self::Userdata(inner) => Some(inner.acquire_ref(thread)?),
			Self::Thread(inner) => Some(inner.acquire_ref(thread)?),
			Self::Buffer(inner) => Some(inner.acquire_ref(thread)?)
		})
	}

	fn raw_value(&self) -> RawValue {
		match self {
			Self::Nil(inner) => inner.raw_value(),
			Self::Boolean(inner) => inner.raw_value(),
			Self::LightUserdata(inner) => inner.raw_value(),
			Self::Number(inner) => inner.raw_value(),
			Self::Vector(inner) => inner.raw_value(),
			Self::String(inner) => inner.raw_value(),
			Self::Table(inner) => inner.raw_value(),
			Self::Closure(inner) => inner.raw_value(),
			Self::Userdata(inner) => inner.raw_value(),
			Self::Thread(inner) => inner.raw_value(),
			Self::Buffer(inner) => inner.raw_value()
		}
	}
}

impl<'a> Dynamic<'a> {
	pub unsafe fn from_raw(data: RawValue) -> Self {
		match data.tag() {
			RawValueTag::Nil => Self::Nil(Nil),
			RawValueTag::Boolean => Self::Boolean(Boolean(data.data().boolean)),
			RawValueTag::LightUserdata => Self::LightUserdata(LightUserdata(data.data().lightuserdata)),
			RawValueTag::Number => Self::Number(Number(data.data().number)),
			RawValueTag::Vector => Self::Vector(Vector(data.data().vector)),
			RawValueTag::String => Self::String(LString::from_raw(addr_of!(data.data().string).read_unaligned().as_ref())),
			RawValueTag::Table => Self::Table(Table::from_raw(addr_of!(data.data().table).read_unaligned().as_ref())),
			RawValueTag::Closure => Self::Closure(Closure::from_raw(addr_of!(data.data().closure).read_unaligned().as_ref())),
			RawValueTag::Userdata => Self::Userdata(Userdata::from_raw(addr_of!(data.data().userdata).read_unaligned().as_ref())),
			RawValueTag::Thread => Self::Thread(Thread::from_raw(addr_of!(data.data().thread).read_unaligned().as_ref())),
			RawValueTag::Buffer => Self::Buffer(Buffer::from_raw(addr_of!(data.data().buffer).read_unaligned().as_ref()))
		}
	}

	pub fn get_nil(&self) -> Option<Nil> {
		let Self::Nil(inner) = *self else { return None };
		Some(inner)
	}

	pub fn get_boolean(&self) -> Option<Boolean> {
		let Self::Boolean(inner) = *self else { return None };
		Some(inner)
	}

	pub fn get_lightuserdata(&self) -> Option<LightUserdata> {
		let Self::LightUserdata(inner) = *self else { return None };
		Some(inner)
	}

	pub fn get_number(&self) -> Option<Number> {
		let Self::Number(inner) = *self else { return None };
		Some(inner)
	}

	pub fn get_vector(&self) -> Option<Vector> {
		let Self::Vector(inner) = *self else { return None };
		Some(inner)
	}

	pub fn get_string(&self) -> Option<LString<'a>> {
		let Self::String(inner) = self else { return None };
		Some(unsafe { LString::from_raw(inner.raw()) })
	}

	pub fn get_table(&self) -> Option<Table<'a>> {
		let Self::Table(inner) = self else { return None };
		Some(unsafe { Table::from_raw(inner.raw()) })
	}

	pub fn get_closure(&self) -> Option<Closure<'a>> {
		let Self::Closure(inner) = self else { return None };
		Some(unsafe { Closure::from_raw(inner.raw()) })
	}

	pub fn get_userdata(&self) -> Option<Userdata<'a>> {
		let Self::Userdata(inner) = self else { return None };
		Some(unsafe { Userdata::from_raw(inner.raw()) })
	}

	pub fn get_thread(&self) -> Option<Thread<'a>> {
		let Self::Thread(inner) = self else { return None };
		Some(unsafe { Thread::from_raw(inner.raw()) })
	}

	pub fn get_buffer(&self) -> Option<Buffer<'a>> {
		let Self::Buffer(inner) = self else { return None };
		Some(unsafe { Buffer::from_raw(inner.raw()) })
	}
}

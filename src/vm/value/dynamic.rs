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

#[derive(Clone, Debug)]
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

impl<'a> Datatype<'a> for Dynamic<'a> {
	type Ref = Option<LuauRef<'a>>;

	fn acquire_ref(&self, thread: Thread<'a>) -> Option<Self::Ref> {
		match self {
			Self::Nil(inner) => Some(inner.acquire_ref(thread)?).and(None),
			Self::Boolean(inner) => Some(inner.acquire_ref(thread)?).and(None),
			Self::LightUserdata(inner) => Some(inner.acquire_ref(thread)?).and(None),
			Self::Number(inner) => Some(inner.acquire_ref(thread)?).and(None),
			Self::Vector(inner) => Some(inner.acquire_ref(thread)?).and(None),
			Self::String(inner) => inner.acquire_ref(thread).map(Some),
			Self::Table(inner) => inner.acquire_ref(thread).map(Some),
			Self::Closure(inner) => inner.acquire_ref(thread).map(Some),
			Self::Userdata(inner) => inner.acquire_ref(thread).map(Some),
			Self::Thread(inner) => inner.acquire_ref(thread).map(Some),
			Self::Buffer(inner) => inner.acquire_ref(thread).map(Some)
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LuauType {
	Nil,
	Boolean,
	LightUserdata,
	Number,
	Vector,
	String,
	Table,
	Closure,
	Userdata,
	Thread,
	Buffer
}

impl<'a> Dynamic<'a> {
	pub fn luau_type(&self) -> LuauType {
		match self {
			Self::Nil(_) => LuauType::Nil,
			Self::Boolean(_) => LuauType::Boolean,
			Self::LightUserdata(_) => LuauType::LightUserdata,
			Self::Number(_) => LuauType::Number,
			Self::Vector(_) => LuauType::Vector,
			Self::String(_) => LuauType::String,
			Self::Table(_) => LuauType::Table,
			Self::Closure(_) => LuauType::Closure,
			Self::Userdata(_) => LuauType::Userdata,
			Self::Thread(_) => LuauType::Thread,
			Self::Buffer(_) => LuauType::Buffer
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
		Some(inner.clone())
	}

	pub fn get_table(&self) -> Option<Table<'a>> {
		let Self::Table(inner) = self else { return None };
		Some(inner.clone())
	}

	pub fn get_closure(&self) -> Option<Closure<'a>> {
		let Self::Closure(inner) = self else { return None };
		Some(inner.clone())
	}

	pub fn get_userdata(&self) -> Option<Userdata<'a>> {
		let Self::Userdata(inner) = self else { return None };
		Some(inner.clone())
	}

	pub fn get_thread(&self) -> Option<Thread<'a>> {
		let Self::Thread(inner) = self else { return None };
		Some(inner.clone())
	}

	pub fn get_buffer(&self) -> Option<Buffer<'a>> {
		let Self::Buffer(inner) = self else { return None };
		Some(inner.clone())
	}
}

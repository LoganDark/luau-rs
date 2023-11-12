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

use std::ffi::c_int;
use std::ops::Deref;

use luau_sys::luau::{lua_ref, lua_unref, TValue};

use crate::vm::raw::RawGlobal;
use crate::vm::ThreadData;
use crate::vm::value::boolean::Boolean;
use crate::vm::value::buffer::Buffer;
use crate::vm::value::closure::Closure;
use crate::vm::value::lightuserdata::LightUserdata;
use crate::vm::value::nil::Nil;
use crate::vm::value::number::Number;
use crate::vm::value::string::LString;
use crate::vm::value::table::Table;
use crate::vm::value::thread::Thread;
use crate::vm::value::userdata::Userdata;
use crate::vm::value::vector::Vector;

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

pub unsafe trait AsTValue {
	fn as_tvalue(&self) -> TValue;
}

#[derive(Debug)]
pub struct Value<'a, T: AsTValue> {
	global: &'a RawGlobal,
	handle: c_int,
	value: T
}

unsafe impl<'a, T: AsTValue> AsTValue for Value<'a, T> {
	fn as_tvalue(&self) -> TValue { self.value.as_tvalue() }
}

impl<'a, T: AsTValue> Drop for Value<'a, T> {
	fn drop(&mut self) {
		unsafe { lua_unref(self.global.mainthread, self.handle); }
	}
}

impl<'a, T: AsTValue> Value<'a, T> {
	pub unsafe fn new(global: &'a RawGlobal, value: T) -> Self {
		global.main_thread().as_ref().push(value.as_tvalue());
		let handle = lua_ref(global.mainthread, -1);
		Self { global, handle, value }
	}
}

impl<'a, T: AsTValue> Deref for Value<'a, T> {
	type Target = T;
	fn deref(&self) -> &Self::Target { &self.value }
}

impl<'a, T: AsTValue + Clone> Clone for Value<'a, T> {
	fn clone(&self) -> Self {
		unsafe { Self::new(self.global, self.value.clone()) }
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ValueType {
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

#[derive(Clone, Debug)]
pub enum Values<'a, D: ThreadData> {
	Nil(Value<'a, Nil>),
	Boolean(Value<'a, Boolean>),
	LightUserdata(Value<'a, LightUserdata>),
	Number(Value<'a, Number>),
	Vector(Value<'a, Vector>),
	String(Value<'a, LString>),
	Table(Value<'a, Table>),
	Closure(Value<'a, Closure>),
	Userdata(Value<'a, Userdata>),
	Thread(Value<'a, Thread<D>>),
	Buffer(Value<'a, Buffer>)
}

unsafe impl<'a, D: ThreadData> AsTValue for Values<'a, D> {
	fn as_tvalue(&self) -> TValue {
		match self {
			Self::Nil(value) => value.as_tvalue(),
			Self::Boolean(value) => value.as_tvalue(),
			Self::LightUserdata(value) => value.as_tvalue(),
			Self::Number(value) => value.as_tvalue(),
			Self::Vector(value) => value.as_tvalue(),
			Self::String(value) => value.as_tvalue(),
			Self::Table(value) => value.as_tvalue(),
			Self::Closure(value) => value.as_tvalue(),
			Self::Userdata(value) => value.as_tvalue(),
			Self::Thread(value) => value.as_tvalue(),
			Self::Buffer(value) => value.as_tvalue()
		}
	}
}

impl<'a, D: ThreadData> Values<'a, D> {
	pub fn value_type(&self) -> ValueType {
		match self {
			Self::Nil(_) => ValueType::Nil,
			Self::Boolean(_) => ValueType::Boolean,
			Self::LightUserdata(_) => ValueType::LightUserdata,
			Self::Number(_) => ValueType::Number,
			Self::Vector(_) => ValueType::Vector,
			Self::String(_) => ValueType::String,
			Self::Table(_) => ValueType::Table,
			Self::Closure(_) => ValueType::Closure,
			Self::Userdata(_) => ValueType::Userdata,
			Self::Thread(_) => ValueType::Thread,
			Self::Buffer(_) => ValueType::Buffer
		}
	}

	pub fn get_nil(&self) -> Option<Value<'a, Nil>> {
		let Self::Nil(value) = self else { return None };
		Some(value.clone())
	}

	pub fn get_boolean(&self) -> Option<Value<'a, Boolean>> {
		let Self::Boolean(value) = self else { return None };
		Some(value.clone())
	}

	pub fn get_lightuserdata(&self) -> Option<Value<'a, LightUserdata>> {
		let Self::LightUserdata(value) = self else { return None };
		Some(value.clone())
	}

	pub fn get_number(&self) -> Option<Value<'a, Number>> {
		let Self::Number(value) = self else { return None };
		Some(value.clone())
	}

	pub fn get_vector(&self) -> Option<Value<'a, Vector>> {
		let Self::Vector(value) = self else { return None };
		Some(value.clone())
	}

	pub fn string(&self) -> Option<Value<'a, LString>> {
		let Self::String(value) = self else { return None };
		Some(value.clone())
	}

	pub fn get_table(&self) -> Option<Value<'a, Table>> {
		let Self::Table(value) = self else { return None };
		Some(value.clone())
	}

	pub fn get_closure(&self) -> Option<Value<'a, Closure>> {
		let Self::Closure(value) = self else { return None };
		Some(value.clone())
	}

	pub fn get_userdata(&self) -> Option<Value<'a, Userdata>> {
		let Self::Userdata(value) = self else { return None };
		Some(value.clone())
	}

	pub fn get_thread(&self) -> Option<Value<'a, Thread<D>>> {
		let Self::Thread(value) = self else { return None };
		Some(value.clone())
	}

	pub fn get_buffer(&self) -> Option<Value<'a, Buffer>> {
		let Self::Buffer(value) = self else { return None };
		Some(value.clone())
	}
}

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

use std::convert::TryFrom;
use std::ffi::c_int;
use std::mem::transmute;
use std::ops::Deref;
use std::ptr::NonNull;

use luau_sys::luau::{GCObject, lua_Type, TValue, Value};

use crate::vm::raw::buffer::RawBuffer;
use crate::vm::raw::closure::RawClosure;
use crate::vm::raw::string::RawString;
use crate::vm::raw::table::RawTable;
use crate::vm::raw::thread::RawThread;
use crate::vm::raw::userdata::RawUserdata;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum RawValueType {
	#[default]
	Nil = lua_Type::LUA_TNIL as _,
	Boolean = lua_Type::LUA_TBOOLEAN as _,
	LightUserdata = lua_Type::LUA_TLIGHTUSERDATA as _,
	Number = lua_Type::LUA_TNUMBER as _,
	Vector = lua_Type::LUA_TVECTOR as _,
	String = lua_Type::LUA_TSTRING as _,
	Table = lua_Type::LUA_TTABLE as _,
	Closure = lua_Type::LUA_TFUNCTION as _,
	Userdata = lua_Type::LUA_TUSERDATA as _,
	Thread = lua_Type::LUA_TTHREAD as _,
	Buffer = lua_Type::LUA_TBUFFER as _
}

impl TryFrom<lua_Type> for RawValueType {
	type Error = ();

	fn try_from(value: lua_Type) -> Result<Self, Self::Error> {
		match value {
			lua_Type::LUA_TNIL => Ok(Self::Nil),
			lua_Type::LUA_TBOOLEAN => Ok(Self::Boolean),
			lua_Type::LUA_TLIGHTUSERDATA => Ok(Self::LightUserdata),
			lua_Type::LUA_TNUMBER => Ok(Self::Number),
			lua_Type::LUA_TVECTOR => Ok(Self::Vector),
			lua_Type::LUA_TSTRING => Ok(Self::String),
			lua_Type::LUA_TTABLE => Ok(Self::Table),
			lua_Type::LUA_TFUNCTION => Ok(Self::Closure),
			lua_Type::LUA_TUSERDATA => Ok(Self::Userdata),
			lua_Type::LUA_TTHREAD => Ok(Self::Thread),
			lua_Type::LUA_TBUFFER => Ok(Self::Buffer),
			lua_Type::LUA_TPROTO |
			lua_Type::LUA_TUPVAL |
			lua_Type::LUA_TDEADKEY => Err(())
		}
	}
}

impl From<RawValueType> for lua_Type {
	fn from(value: RawValueType) -> Self {
		match value {
			RawValueType::Nil => lua_Type::LUA_TNIL,
			RawValueType::Boolean => lua_Type::LUA_TBOOLEAN,
			RawValueType::LightUserdata => lua_Type::LUA_TLIGHTUSERDATA,
			RawValueType::Number => lua_Type::LUA_TNUMBER,
			RawValueType::Vector => lua_Type::LUA_TVECTOR,
			RawValueType::String => lua_Type::LUA_TSTRING,
			RawValueType::Table => lua_Type::LUA_TTABLE,
			RawValueType::Closure => lua_Type::LUA_TFUNCTION,
			RawValueType::Userdata => lua_Type::LUA_TUSERDATA,
			RawValueType::Thread => lua_Type::LUA_TTHREAD,
			RawValueType::Buffer => lua_Type::LUA_TBUFFER
		}
	}
}

impl RawValueType {
	pub fn into_lua_type(self) -> lua_Type { self.into() }

	pub fn is_value_type(&self) -> bool {
		matches!(self, Self::Nil | Self::Boolean | Self::LightUserdata | Self::Number | Self::Vector)
	}

	pub fn is_collectible(&self) -> bool {
		matches!(self, Self::String | Self::Table | Self::Closure | Self::Userdata | Self::Thread | Self::Buffer)
	}
}

#[derive(Copy, Clone, Debug)]
pub enum RawValueData {
	Nil,
	Boolean(bool),
	LightUserdata(*mut ()),
	Number(f64),
	Vector([f32; 3]),
	String(NonNull<RawString>),
	Table(NonNull<RawTable>),
	Closure(NonNull<RawClosure>),
	Userdata(NonNull<RawUserdata>),
	Thread(NonNull<RawThread>),
	Buffer(NonNull<RawBuffer>)
}

impl RawValueData {
	pub unsafe fn from_tvalue(value: TValue) -> Self {
		match RawValue::from_tvalue(value).value_type() {
			RawValueType::Nil => Self::Nil,
			RawValueType::Boolean => Self::Boolean(value.value.b == 1),
			RawValueType::LightUserdata => Self::LightUserdata(value.value.p.cast()),
			RawValueType::Number => Self::Number(value.value.n),
			RawValueType::Vector => Self::Vector({
				let [x, y] = value.value.v;
				let z = f32::from_ne_bytes(value.extra[0].to_ne_bytes());
				[x, y, z]
			}),
			RawValueType::String => Self::String(NonNull::new_unchecked(value.value.gc).cast()),
			RawValueType::Table => Self::Table(NonNull::new_unchecked(value.value.gc).cast()),
			RawValueType::Closure => Self::Closure(NonNull::new_unchecked(value.value.gc).cast()),
			RawValueType::Userdata => Self::Userdata(NonNull::new_unchecked(value.value.gc).cast()),
			RawValueType::Thread => Self::Thread(NonNull::new_unchecked(value.value.gc).cast()),
			RawValueType::Buffer => Self::Buffer(NonNull::new_unchecked(value.value.gc).cast())
		}
	}
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct RawValue(TValue);

impl Deref for RawValue {
	type Target = TValue;
	fn deref(&self) -> &Self::Target { &self.0 }
}

impl From<TValue> for RawValue {
	fn from(value: TValue) -> Self { Self(value) }
}

impl From<RawValue> for TValue {
	fn from(value: RawValue) -> Self { value.0 }
}

impl RawValue {
	pub unsafe fn from_tvalue(value: TValue) -> Self { Self(value) }
	pub fn into_tvalue(self) -> TValue { self.0 }

	pub unsafe fn new(value: Value, extra: [c_int; 1], tag: RawValueType) -> Self {
		Self(TValue { value, extra, tt: tag.into_lua_type() as _ })
	}

	pub unsafe fn new_nil() -> Self { Self::new(Value { b: 0 }, [0], RawValueType::Nil) }
	pub unsafe fn new_boolean(value: bool) -> Self { Self::new(Value { b: value as _ }, [0], RawValueType::Boolean) }
	pub unsafe fn new_lightuserdata(value: *mut ()) -> Self { Self::new(Value { p: value.cast() }, [0], RawValueType::LightUserdata) }
	pub unsafe fn new_number(value: f64) -> Self { Self::new(Value { n: value }, [0], RawValueType::Number) }

	pub unsafe fn new_vector(value: [f32; 3]) -> Self {
		let [x, y, z] = value;
		Self::new(Value { v: [x, y] }, [c_int::from_ne_bytes(z.to_ne_bytes())], RawValueType::Vector)
	}

	pub unsafe fn new_gc(value: NonNull<GCObject>, tag: RawValueType) -> Self { Self::new(Value { gc: value.as_ptr() }, [0], tag) }
	pub unsafe fn new_string(value: NonNull<RawString>) -> Self { Self::new_gc(value.cast(), RawValueType::String) }
	pub unsafe fn new_table(value: NonNull<RawTable>) -> Self { Self::new_gc(value.cast(), RawValueType::Table) }
	pub unsafe fn new_closure(value: NonNull<RawClosure>) -> Self { Self::new_gc(value.cast(), RawValueType::Closure) }
	pub unsafe fn new_userdata(value: NonNull<RawUserdata>) -> Self { Self::new_gc(value.cast(), RawValueType::Userdata) }
	pub unsafe fn new_thread(value: NonNull<RawThread>) -> Self { Self::new_gc(value.cast(), RawValueType::Thread) }
	pub unsafe fn new_buffer(value: NonNull<RawBuffer>) -> Self { Self::new_gc(value.cast(), RawValueType::Buffer) }

	pub unsafe fn nil_value() -> TValue { Self::new_nil().into_tvalue() }
	pub unsafe fn boolean_value(value: bool) -> TValue { Self::new_boolean(value).into_tvalue() }
	pub unsafe fn lightuserdata_value(value: *mut ()) -> TValue { Self::new_lightuserdata(value).into_tvalue() }
	pub unsafe fn number_value(value: f64) -> TValue { Self::new_number(value).into_tvalue() }
	pub unsafe fn vector_value(value: [f32; 3]) -> TValue { Self::new_vector(value).into_tvalue() }
	pub unsafe fn string_value(value: NonNull<RawString>) -> TValue { Self::new_string(value).into_tvalue() }
	pub unsafe fn table_value(value: NonNull<RawTable>) -> TValue { Self::new_table(value).into_tvalue() }
	pub unsafe fn closure_value(value: NonNull<RawClosure>) -> TValue { Self::new_closure(value).into_tvalue() }
	pub unsafe fn userdata_value(value: NonNull<RawUserdata>) -> TValue { Self::new_userdata(value).into_tvalue() }
	pub unsafe fn thread_value(value: NonNull<RawThread>) -> TValue { Self::new_thread(value).into_tvalue() }
	pub unsafe fn buffer_value(value: NonNull<RawBuffer>) -> TValue { Self::new_buffer(value).into_tvalue() }

	pub fn lua_type(&self) -> lua_Type { unsafe { transmute(self.tt) } }
	pub fn value_type(&self) -> RawValueType { unsafe { RawValueType::try_from(self.lua_type()).unwrap_unchecked() } }
	pub fn value_date(&self) -> RawValueData { unsafe { RawValueData::from_tvalue(self.into_tvalue()) } }

	pub fn is_value_type(&self) -> bool { self.value_type().is_value_type() }
	pub fn is_collectible(&self) -> bool { self.value_type().is_collectible() }
}

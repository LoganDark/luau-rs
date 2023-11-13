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

use std::mem::transmute;
use std::ptr::NonNull;

use luau_sys::luau::{lua_Type, TValue};

use crate::vm::raw::buffer::RawBuffer;
use crate::vm::raw::closure::RawClosure;
use crate::vm::raw::string::RawString;
use crate::vm::raw::table::RawTable;
use crate::vm::raw::thread::RawThread;
use crate::vm::raw::userdata::RawUserdata;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
#[repr(u32)]
pub enum RawValueTag {
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

#[derive(Copy, Clone)]
#[repr(C, packed(4))]
pub union RawValueData {
	pub nil: (),
	pub boolean: bool,
	pub lightuserdata: *mut (),
	pub number: f64,
	pub vector: [f32; 3],
	pub string: NonNull<RawString>,
	pub table: NonNull<RawTable>,
	pub closure: NonNull<RawClosure>,
	pub userdata: NonNull<RawUserdata>,
	pub thread: NonNull<RawThread>,
	pub buffer: NonNull<RawBuffer>
}

impl From<lua_Type> for RawValueTag {
	fn from(value: lua_Type) -> Self {
		match value {
			lua_Type::LUA_TNIL => Self::Nil,
			lua_Type::LUA_TBOOLEAN => Self::Boolean,
			lua_Type::LUA_TLIGHTUSERDATA => Self::LightUserdata,
			lua_Type::LUA_TNUMBER => Self::Number,
			lua_Type::LUA_TVECTOR => Self::Vector,
			lua_Type::LUA_TSTRING => Self::String,
			lua_Type::LUA_TTABLE => Self::Table,
			lua_Type::LUA_TFUNCTION => Self::Closure,
			lua_Type::LUA_TUSERDATA => Self::Userdata,
			lua_Type::LUA_TTHREAD => Self::Thread,
			lua_Type::LUA_TBUFFER => Self::Buffer,
			lua_Type::LUA_TPROTO |
			lua_Type::LUA_TUPVAL |
			lua_Type::LUA_TDEADKEY => Self::Nil
		}
	}
}

impl From<RawValueTag> for lua_Type {
	fn from(value: RawValueTag) -> Self { unsafe { transmute(value) } }
}

impl RawValueTag {
	pub fn is_value(&self) -> bool {
		matches!(self, Self::Nil | Self::Boolean | Self::LightUserdata | Self::Number | Self::Vector)
	}

	pub fn is_collectible(&self) -> bool {
		matches!(self, Self::String | Self::Table | Self::Closure | Self::Userdata | Self::Thread | Self::Buffer)
	}
}

#[derive(Copy, Clone)]
#[repr(C, packed(4))]
pub struct RawValue {
	data: RawValueData,
	tag: RawValueTag
}

impl From<TValue> for RawValue {
	fn from(value: TValue) -> Self { unsafe { transmute(value) } }
}

impl From<RawValue> for TValue {
	fn from(value: RawValue) -> Self { unsafe { transmute(value) } }
}

impl RawValue {
	pub unsafe fn new(data: RawValueData, tag: RawValueTag) -> Self { Self { data, tag } }
	pub unsafe fn new_nil() -> Self { Self::new(RawValueData { nil: () }, RawValueTag::Nil) }
	pub unsafe fn new_boolean(value: bool) -> Self { Self::new(RawValueData { boolean: value }, RawValueTag::Boolean) }
	pub unsafe fn new_lightuserdata(value: *mut ()) -> Self { Self::new(RawValueData { lightuserdata: value }, RawValueTag::LightUserdata) }
	pub unsafe fn new_number(value: f64) -> Self { Self::new(RawValueData { number: value }, RawValueTag::Number) }
	pub unsafe fn new_vector(value: [f32; 3]) -> Self { Self::new(RawValueData { vector: value }, RawValueTag::Vector) }
	pub unsafe fn new_string(value: NonNull<RawString>) -> Self { Self::new(RawValueData { string: value }, RawValueTag::String) }
	pub unsafe fn new_table(value: NonNull<RawTable>) -> Self { Self::new(RawValueData { table: value }, RawValueTag::Table) }
	pub unsafe fn new_closure(value: NonNull<RawClosure>) -> Self { Self::new(RawValueData { closure: value }, RawValueTag::Closure) }
	pub unsafe fn new_userdata(value: NonNull<RawUserdata>) -> Self { Self::new(RawValueData { userdata: value }, RawValueTag::Userdata) }
	pub unsafe fn new_thread(value: NonNull<RawThread>) -> Self { Self::new(RawValueData { thread: value }, RawValueTag::Thread) }
	pub unsafe fn new_buffer(value: NonNull<RawBuffer>) -> Self { Self::new(RawValueData { buffer: value }, RawValueTag::Buffer) }

	pub fn data(&self) -> &RawValueData { &self.data }
	pub fn tag(&self) -> RawValueTag { self.tag }

	pub fn is_value_type(&self) -> bool { self.tag().is_value() }
	pub fn is_collectible(&self) -> bool { self.tag().is_collectible() }
}

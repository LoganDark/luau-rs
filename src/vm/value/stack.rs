// luau-rs - Rust bindings to Roblox's Luau
// Copyright (C) 2021 LoganDark
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::ffi::c_void;
use std::fmt::{Display, Formatter};
use std::os::raw::c_char;

use bstr::BStr;

use crate::luau_sys::luau::{Closure, lua_gettop, lua_State, lua_Type_LUA_TBOOLEAN, lua_Type_LUA_TFUNCTION, lua_Type_LUA_TLIGHTUSERDATA, lua_Type_LUA_TNIL, lua_Type_LUA_TNUMBER, lua_Type_LUA_TSTRING, lua_Type_LUA_TTABLE, lua_Type_LUA_TTHREAD, lua_Type_LUA_TUSERDATA, lua_Type_LUA_TVECTOR, Table, TString, TValue, Udata, Value as LValue};

/// StackValue is the union of all values that can exist on the Luau stack. It
/// is a replacement for the unsafe and error-prone FFI type TValue. This type
/// is equivalent to an unmanaged pointer to a Luau value; the garbage collector
/// **may free it at any point** unless it is put into a Value.
///
/// In many cases, this enum is only safe to use to refer to values that are
/// currently on the stack, or to use to construct a Value - cases where the
/// value is not at risk of garbage collection.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum StackValue {
	Nil,
	Boolean(bool),
	LightUserdata(*mut c_void),
	Number(f64),
	Vector([f32; 3]),
	String(*mut TString),
	Table(*mut Table),
	Function(*mut Closure),
	Userdata(*mut Udata),
	Thread(*mut lua_State)
}

impl Display for StackValue {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			StackValue::Nil => f.debug_tuple("Nil").finish(),
			StackValue::Boolean(b) => f.debug_tuple("Boolean").field(b).finish(),
			StackValue::LightUserdata(ptr) => f.debug_tuple("LightUserdata").field(ptr).finish(),
			StackValue::Number(n) => f.debug_tuple("Number").field(n).finish(),
			StackValue::Vector(v) => f.debug_tuple("Vector").field(&v[0]).field(&v[1]).field(&v[2]).finish(),
			StackValue::String(ptr) => {
				let bstr: &BStr = unsafe { std::slice::from_raw_parts(&(**ptr).data as *const c_char as *const u8, (**ptr).len as _) }.into();
				f.debug_tuple("String").field(&bstr).finish()
			},
			StackValue::Table(ptr) => f.debug_tuple("Table").field(ptr).finish(),
			StackValue::Function(ptr) => f.debug_tuple("Function").field(ptr).finish(),
			StackValue::Userdata(ptr) => f.debug_tuple("Userdata").field(ptr).finish(),
			StackValue::Thread(ptr) => f.debug_tuple("Thread").field(ptr).finish()
		}
	}
}

impl StackValue {
	/// Returns true if this value is garbage-collectible. For primitive value
	/// types, such as `nil`, booleans, light userdatas, numbers, and vectors,
	/// this is `false`, but for reference types such as strings, tables,
	/// closures, full userdatas, and threads, this is `true`.
	pub fn is_collectible(&self) -> bool {
		match self {
			Self::String(_) | Self::Table(_) | Self::Function(_) | Self::Userdata(_) | Self::Thread(_) => true,
			_ => false
		}
	}

	pub unsafe fn stack(state: *mut lua_State) -> Vec<Self> {
		std::slice::from_raw_parts((*state).base, lua_gettop(state) as _).iter().copied().map(Self::from).collect()
	}
}

impl Into<TValue> for StackValue {
	fn into(self) -> TValue {
		match self {
			Self::Nil => TValue { value: LValue { b: 0 }, extra: 0, tt: lua_Type_LUA_TNIL as _ },
			Self::Boolean(b) => TValue { value: LValue { b: b as _ }, extra: 0, tt: lua_Type_LUA_TBOOLEAN as _ },
			Self::LightUserdata(ptr) => TValue { value: LValue { p: ptr }, extra: 0, tt: lua_Type_LUA_TLIGHTUSERDATA as _ },
			Self::Number(n) => TValue { value: LValue { n }, extra: 0, tt: lua_Type_LUA_TNUMBER as _ },
			Self::Vector([x, y, z]) => TValue {
				value: LValue { v: [x, y] },
				extra: unsafe { std::mem::transmute(z) },
				tt: lua_Type_LUA_TVECTOR as _
			},
			Self::String(gc) => TValue { value: LValue { gc: gc as _ }, extra: 0, tt: lua_Type_LUA_TSTRING as _ },
			Self::Table(gc) => TValue { value: LValue { gc: gc as _ }, extra: 0, tt: lua_Type_LUA_TTABLE as _ },
			Self::Function(gc) => TValue { value: LValue { gc: gc as _ }, extra: 0, tt: lua_Type_LUA_TFUNCTION as _ },
			Self::Userdata(gc) => TValue { value: LValue { gc: gc as _ }, extra: 0, tt: lua_Type_LUA_TUSERDATA as _ },
			Self::Thread(gc) => TValue { value: LValue { gc: gc as _ }, extra: 0, tt: lua_Type_LUA_TTHREAD as _ }
		}
	}
}

impl From<TValue> for StackValue {
	fn from(value: TValue) -> Self {
		match value.tt as _ {
			luau_sys::luau::lua_Type_LUA_TNIL => StackValue::Nil,
			luau_sys::luau::lua_Type_LUA_TBOOLEAN => StackValue::Boolean(unsafe { value.value.b } != 0),
			luau_sys::luau::lua_Type_LUA_TLIGHTUSERDATA => StackValue::LightUserdata(unsafe { value.value.p }),
			luau_sys::luau::lua_Type_LUA_TNUMBER => StackValue::Number(unsafe { value.value.n }),
			luau_sys::luau::lua_Type_LUA_TVECTOR => StackValue::Vector(unsafe { [value.value.v[0], value.value.v[1], std::mem::transmute(value.extra)] }),
			luau_sys::luau::lua_Type_LUA_TSTRING => StackValue::String(unsafe { value.value.p as *mut TString }),
			luau_sys::luau::lua_Type_LUA_TTABLE => StackValue::Table(unsafe { value.value.p as *mut Table }),
			luau_sys::luau::lua_Type_LUA_TFUNCTION => StackValue::Function(unsafe { value.value.p as *mut Closure }),
			luau_sys::luau::lua_Type_LUA_TUSERDATA => StackValue::Userdata(unsafe { value.value.p as *mut Udata }),
			luau_sys::luau::lua_Type_LUA_TTHREAD => StackValue::Thread(unsafe { value.value.p as *mut lua_State }),
			_ => unreachable!("other types of values cannot exist on the stack")
		}
	}
}

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

use std::ffi::CString;

use luau_sys::luau::{lua_pushnil, lua_pushvalue, lua_rawset, LUA_REGISTRYINDEX, lua_settop, lua_State, lua_Type_LUA_TBOOLEAN, lua_Type_LUA_TLIGHTUSERDATA, lua_Type_LUA_TNIL, lua_Type_LUA_TNUMBER, lua_Type_LUA_TSTRING, lua_Type_LUA_TTABLE, lua_Type_LUA_TVECTOR, luaH_new, luaS_newlstr, TValue, Value as LValue};

use crate::vm::{Thread, ThreadUserdata};

pub mod types;

/// The Value struct represents a reference to a Luau value. If the value is
/// garbage-collectible, Value guarantees it will not be collected until the
/// Value is dropped.
///
/// Value has a helper method for producing the value on the Luau stack so that
/// operations can be performed on it. Stack functions are `unsafe`.
pub struct Value<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> {
	thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>,
	value: TValue,
	key: [u8; 16]
}

impl<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'vm> Value<'borrow, 'thread, 'vm, UD> {
	// Produces a TValue on the stack. Unsafe because no bounds checking is
	// performed and all stack functions are unsafe.
	unsafe fn produce_tvalue(state: &mut lua_State, value: TValue) {
		let top = state.top;
		*top = value;
		state.top = top.offset(1);
	}

	// Produces the key onto the stack using `produce_tvalue`. Unsafe because no
	// bounds checking is performed and all stack functions are unsafe.
	unsafe fn push_key(state: &mut lua_State, object: [u8; 16]) {
		let key = luaS_newlstr(state as _, object.as_ptr() as _, 16);

		Self::produce_tvalue(state, TValue {
			value: LValue {
				gc: key as _
			},
			extra: 0,
			tt: lua_Type_LUA_TSTRING as _
		})
	}

	// Produces the value onto the stack using `produce_tvalue`. Unsafe because
	// no bounds checking is performed and all stack functions are unsafe.
	pub unsafe fn push_value(&mut self, state: &mut lua_State) {
		Self::produce_tvalue(state, self.value)
	}

	// Creates a new `Value` from the specified `TValue` and `lua_State`. Unsafe
	// because `TValue` must not be invalid.
	pub unsafe fn new(thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>, value: TValue) -> Self {
		let key: [u8; 16] = rand::random();

		if value.tt <= lua_Type_LUA_TSTRING as _ {
			// garbage collectible. ORDER TYPE
			lua_pushvalue(thread.state as _, LUA_REGISTRYINDEX);
			Self::push_key(thread.state, key);
			Self::produce_tvalue(thread.state, value);
			lua_rawset(thread.state as _, -3);
			lua_settop(thread.state as _, -1);
		}

		Self { thread, value, key }
	}

	// Creates a new `Value` from the `TValue` at the top of the specified
	// `Thread`'s stack. Unsafe because stack functions are unsafe.
	pub unsafe fn from_stack_top(thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>) -> Self {
		let top = thread.state.top;
		thread.state.top = top.offset(-1);
		Self::new(thread, *top)
	}

	pub fn new_nil(thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>) -> Self {
		unsafe {
			Self::new(thread, TValue {
				value: LValue {
					b: 0
				},
				extra: 0,
				tt: lua_Type_LUA_TNIL as _
			})
		}
	}

	pub fn new_bool(thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>, value: bool) -> Self {
		unsafe {
			Self::new(thread, TValue {
				value: LValue {
					b: value as _
				},
				extra: 0,
				tt: lua_Type_LUA_TBOOLEAN as _
			})
		}
	}

	pub fn new_lightuserdata(thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>, value: *mut std::ffi::c_void) -> Self {
		unsafe {
			Self::new(thread, TValue {
				value: LValue {
					p: value
				},
				extra: 0,
				tt: lua_Type_LUA_TLIGHTUSERDATA as _
			})
		}
	}

	pub fn new_number(thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>, value: f64) -> Self {
		unsafe {
			Self::new(thread, TValue {
				value: LValue {
					n: value
				},
				extra: 0,
				tt: lua_Type_LUA_TNUMBER as _
			})
		}
	}

	pub fn new_vector(thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>, x: f32, y: f32, z: f32) -> Self {
		unsafe {
			Self::new(thread, TValue {
				value: LValue {
					v: [x, y]
				},
				extra: std::mem::transmute(z),
				tt: lua_Type_LUA_TVECTOR as _
			})
		}
	}

	pub fn new_string<B: AsRef<[u8]>>(thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>, bytes: B) -> Self {
		unsafe {
			let bytes = bytes.as_ref();
			let value = luaS_newlstr(thread.state as _, bytes.as_ptr() as _, bytes.len() as _);

			Self::new(thread, TValue {
				value: LValue {
					gc: value as _
				},
				extra: 0,
				tt: lua_Type_LUA_TSTRING as _
			})
		}
	}

	pub fn new_table(thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>, narray: u32, lnhash: u32) -> Self {
		unsafe {
			let value = luaH_new(thread.state as _, narray as _, lnhash as _);

			Self::new(thread, TValue {
				value: LValue {
					gc: value as _
				},
				extra: 0,
				tt: lua_Type_LUA_TTABLE as _
			})
		}
	}
}

impl<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> Drop for Value<'borrow, 'thread, 'vm, UD> {
	fn drop(&mut self) {
		if self.value.tt <= lua_Type_LUA_TSTRING as _ {
			// garbage collectible. ORDER TYPE
			unsafe {
				lua_pushvalue(self.thread.state as _, LUA_REGISTRYINDEX);
				Self::push_key(self.thread.state, self.key);
				lua_pushnil(self.thread.state as _);
				lua_rawset(self.thread.state as _, -3);
				lua_settop(self.thread.state as _, -1);
			}
		}
	}
}

pub trait ProduceLuauValue<'thread, 'vm: 'thread, UD: ThreadUserdata + 'thread> {
	fn luau_value<'borrow: 'thread>(&self, thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>) -> Value<'borrow, 'thread, 'vm, UD>;
}

pub trait ToLuauValue<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> {
	fn to_luau_value(self, thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>) -> Value<'borrow, 'thread, 'vm, UD>;
}

impl<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> ToLuauValue<'borrow, 'thread, 'vm, UD> for &[u8] {
	fn to_luau_value(self, thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>) -> Value<'borrow, 'thread, 'vm, UD> {
		Value::new_string(thread, self)
	}
}

impl<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> ToLuauValue<'borrow, 'thread, 'vm, UD> for &str {
	fn to_luau_value(self, thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>) -> Value<'borrow, 'thread, 'vm, UD> {
		self.as_bytes().to_luau_value(thread)
	}
}

impl<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> ToLuauValue<'borrow, 'thread, 'vm, UD> for String {
	fn to_luau_value(self, thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>) -> Value<'borrow, 'thread, 'vm, UD> {
		self.as_bytes().to_luau_value(thread)
	}
}

impl<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> ToLuauValue<'borrow, 'thread, 'vm, UD> for CString {
	fn to_luau_value(self, thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>) -> Value<'borrow, 'thread, 'vm, UD> {
		self.as_bytes().to_luau_value(thread)
	}
}

impl<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> ToLuauValue<'borrow, 'thread, 'vm, UD> for char {
	fn to_luau_value(self, thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>) -> Value<'borrow, 'thread, 'vm, UD> {
		[self as u8].to_luau_value(thread)
	}
}

macro_rules! to_lua_n {
	($($n:ty),+) => {
$(impl<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> ToLuauValue<'borrow, 'thread, 'vm, UD> for $n {
	fn to_luau_value(self, thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>) -> Value<'borrow, 'thread, 'vm, UD> {
		Value::new_number(thread, self as _)
	}
}

)+
	}
}

to_lua_n!(u8, u16, u32, u64, u128, usize);
to_lua_n!(i8, i16, i32, i64, i128, isize);
to_lua_n!(f32, f64);

impl<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> ToLuauValue<'borrow, 'thread, 'vm, UD> for bool {
	fn to_luau_value(self, thread: &'borrow mut Thread<'borrow, 'thread, 'vm, UD>) -> Value<'borrow, 'thread, 'vm, UD> {
		Value::new_bool(thread, self)
	}
}

pub trait FromLuauValue<UD: ThreadUserdata>: Sized {
	fn from_luau_value<'borrow, 'thread: 'borrow, 'vm: 'thread>(value: Value<'borrow, 'thread, 'vm, UD>) -> Option<Self>;
}

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

use std::ffi::{c_void, CStr};
use std::num::NonZeroU32;

use luau_sys::luau::{Closure, lua_ref, lua_settop, lua_State, lua_Type_LUA_TBOOLEAN, lua_Type_LUA_TFUNCTION, lua_Type_LUA_TLIGHTUSERDATA, lua_Type_LUA_TNIL, lua_Type_LUA_TNUMBER, lua_Type_LUA_TSTRING, lua_Type_LUA_TTABLE, lua_Type_LUA_TTHREAD, lua_Type_LUA_TUSERDATA, lua_Type_LUA_TVECTOR, lua_unref, luaH_new, luaS_newlstr, size_t, StkId, Table, TString, TValue, Udata, Value as LValue};
use types::function::Function;

use crate::compiler::CompiledFunction;
use crate::luau_sys::luau::{lua_remove, lua_tolstring, luau_load};
use crate::vm::{Error, Thread, ThreadUserdata};

pub mod types;

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
}

/// The Value struct represents a reference to a Luau value. If the value is
/// garbage-collectible, Value guarantees it will not be collected until the
/// Value is dropped.
///
/// Value also has the ability to produce the value onto the stack when required
/// by library functions - this is sound because of the aforementioned immunity
/// to garbage collection.
///
/// This is the primitive that the library uses to manipulate Luau values. It
/// can safely produce its value in the Luau stack at any time, so table
/// accesses, function calls, operations and so on are possible without managing
/// the unsafe stack manually from Rust.
pub struct Value<'borrow, 'thread: 'borrow, UD: ThreadUserdata> {
	thread: &'borrow Thread<'thread, UD>,
	value: StackValue,
	ref_: Option<NonZeroU32>
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

impl<'borrow, 'thread: 'borrow, UD: ThreadUserdata + 'thread> Value<'borrow, 'thread, UD> {
	/// Produces a `TValue` at the top of the stack. The stack is checked for
	/// space, and, if none exists, `Err` is returned. Otherwise, the pointer to
	/// the new `TValue` that was written is returned.
	///
	/// # Safety
	///
	/// The value pointed to by the `TValue` must not be in the process of
	/// garbage collection or have already been collected. Additionally, the
	/// `TValue` must be valid.
	pub unsafe fn produce(state: *mut lua_State, value: StackValue) -> Result<StkId, ()> {
		let top = (*state).top;
		*top = value.into();
		(*state).top = top.offset(1);
		Ok(top)
	}

	// Produces the value onto the stack using `produce_tvalue`. Unsafe because
	// no bounds checking is performed and all stack functions are unsafe.
	pub unsafe fn push_value(&mut self, state: &mut lua_State) -> Result<StkId, ()> {
		Self::produce(state, self.value)
	}

	// Creates a new `Value` from the specified `TValue` and `lua_State`. Unsafe
	// because `TValue` must not be invalid.
	pub unsafe fn new(thread: &'borrow Thread<'thread, UD>, value: StackValue) -> Result<Self, ()> {
		let mut ref_: u32 = 0;

		if value.is_collectible() {
			let state = thread.as_ptr();
			Self::produce(state, value).unwrap();
			ref_ = lua_ref(state, -1) as _;
			lua_settop(state, -1);
		}

		Ok(Self { thread, value, ref_: NonZeroU32::new(ref_) })
	}

	/// Creates a new `Value` from the `TValue` at the top of the specified
	/// `Thread`'s stack. Unsafe because stack functions are unsafe.
	///
	/// If `Err` is returned, the value is returned to the stack.
	pub unsafe fn pop_from_stack(thread: &'borrow Thread<'thread, UD>) -> Result<Self, ()> {
		let state = thread.as_ptr();
		let top = (*state).top;
		(*state).top = top.offset(-1);

		if let Ok(val) = Self::new(thread, (*top).into()) {
			Ok(val)
		} else {
			(*state).top = top;
			Err(())
		}
	}

	pub fn new_string<B: AsRef<[u8]>>(thread: &'borrow Thread<'thread, UD>, bytes: B) -> Result<Self, ()> {
		unsafe {
			let bytes = bytes.as_ref();
			let string = luaS_newlstr(thread.as_ptr(), bytes.as_ptr() as _, bytes.len() as _);
			Self::new(thread, StackValue::String(string))
		}
	}

	pub fn new_table(thread: &'borrow Thread<'thread, UD>, narray: u32, lnhash: u32) -> Result<Self, ()> {
		unsafe {
			let table = luaH_new(thread.as_ptr(), narray as _, lnhash as _);
			Self::new(thread, StackValue::Table(table))
		}
	}

	pub fn new_function(thread: &'borrow Thread<'thread, UD>, bytecode: CompiledFunction, chunkname: &CStr) -> Result<Self, Error> {
		unsafe {
			let state = thread.as_ptr();
			let bytecode = bytecode.as_ref();

			if luau_load(state, chunkname.as_ptr(), &bytecode[0] as *const u8 as _, bytecode.len() as _, 0) == 0 {
				Self::pop_from_stack(thread).map_err(|()| Error::OutOfStack)
			} else {
				// error is at the top of the stack
				let mut length: size_t = 0;
				let data = lua_tolstring(state, -1, &mut length as _);
				lua_remove(state, -1);
				// Clone so that the string stays valid even if GCed
				Err(Error::Runtime(String::from_raw_parts(data as _, length as _, length as _).clone()))
			}
		}
	}

	pub fn new_userdata(_thread: &'borrow Thread<'thread, UD>) -> Result<Self, ()> {
		todo!()
	}

	pub fn new_thread(_thread: &'borrow Thread<'thread, UD>) -> Result<Self, ()> {
		todo!()
	}

	pub fn clone<'newborrow: 'borrow>(&'newborrow self) -> Result<Self, ()> {
		unsafe { Value::<'newborrow, 'thread, UD>::new(self.thread, self.value) }
	}

	// /// Synchronously calls a Luau function, without support for yielding.
	// pub fn call_sync<A: AsRef<[Value<UD>]>>(args: A) -> Result<Vec<Self>, Error> {
	// 	todo!()
	// }
}

impl<'borrow, 'thread: 'borrow, UD: ThreadUserdata> Drop for Value<'borrow, 'thread, UD> {
	fn drop(&mut self) {
		if let Some(ref_) = self.ref_ {
			unsafe { lua_unref(self.thread.as_ptr(), ref_.get() as _); }
		}
	}
}

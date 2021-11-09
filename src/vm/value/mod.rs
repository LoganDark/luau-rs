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

use std::ffi::CStr;
use std::fmt::{Debug, Display, Formatter};
use std::hint::unreachable_unchecked;
use std::mem::MaybeUninit;
use std::num::NonZeroU32;
use std::pin::Pin;

use luau_sys::luau::{lua_checkstack, lua_gettop, LUA_MULTRET, lua_pcall, lua_ref, lua_settop, lua_State, lua_Status, lua_unref, luaH_new, luaS_newlstr, StkId};
pub use stack::StackValue;

use crate::compiler::CompiledFunction;
use crate::luau_sys::luau::luau_load;
use crate::vm::{Error, Thread, ThreadUserdata};

pub mod types;
mod stack;

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
#[derive(Clone)]
pub struct Value<'borrow, 'thread: 'borrow, UD: ThreadUserdata> {
	thread: &'borrow Thread<'thread, UD>,
	value: StackValue,
	ref_: Option<NonZeroU32>
}

impl<'borrow, 'thread: 'borrow, UD: ThreadUserdata> Debug for Value<'borrow, 'thread, UD> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if let Some(ref ref_) = self.ref_ {
			f.debug_struct("Value")
				.field("value", &format_args!("{}", self.value))
				.field("ref", ref_)
				.finish()
		} else {
			f.debug_tuple("Value").field(&format_args!("{}", self.value)).finish()
		}
	}
}

impl<'borrow, 'thread: 'borrow, UD: ThreadUserdata> Display for Value<'borrow, 'thread, UD> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.value, f)
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
		let last = (*state).top;
		*last = value.into();
		(*state).top = last.offset(1);
		Ok(last)
	}

	// Produces the value onto the stack using `produce_tvalue`. Unsafe because
	// no bounds checking is performed and all stack functions are unsafe.
	pub unsafe fn push_value(&self, state: *mut lua_State) -> Result<StkId, ()> {
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
			lua_settop(state, -2);
		}

		Ok(Self { thread, value, ref_: NonZeroU32::new(ref_) })
	}

	/// Returns the `StackValue` of this `Value`. Note that the `StackValue`
	/// will not necessarily stay alive if the `Value` is dropped!
	pub fn value(&self) -> StackValue {
		self.value
	}

	/// Creates a new `Value` from the `TValue` at the top of the specified
	/// `Thread`'s stack. Unsafe because stack functions are unsafe.
	pub unsafe fn pop_from_stack(thread: &'borrow Thread<'thread, UD>) -> Self {
		let state = thread.as_ptr();
		let last = (*state).top.offset(-1);
		(*state).top = last;

		if let Ok(val) = Self::new(thread, (*last).into()) {
			val
		} else {
			unreachable_unchecked()
		}
	}

	pub fn new_value(thread: &'borrow Thread<'thread, UD>, value: StackValue) -> Result<Self, ()> {
		assert!(!value.is_collectible(), "value must not be collectible");
		unsafe { Self::new(thread, value) }
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

	pub fn load_function(thread: &'borrow Thread<'thread, UD>, bytecode: CompiledFunction, chunkname: &CStr) -> Result<Self, Error> {
		unsafe {
			let state = thread.as_ptr();
			let bytecode = bytecode.as_ref();

			if luau_load(state, chunkname.as_ptr(), &bytecode[0] as *const u8 as _, bytecode.len() as _, 0) == 0 {
				Ok(Self::pop_from_stack(thread))
			} else {
				Err(Error::pop_runtime_error(state))
			}
		}
	}

	pub fn new_userdata(_thread: &'borrow Thread<'thread, UD>) -> Result<Self, ()> {
		todo!()
	}

	pub fn new_thread(_thread: &'borrow Thread<'thread, UD>, _userdata: Pin<Box<UD>>) -> Result<Self, ()> {
		todo!()
	}

	pub fn clone<'newborrow: 'borrow>(&'newborrow self) -> Result<Self, ()> {
		unsafe { Value::<'newborrow, 'thread, UD>::new(self.thread, self.value) }
	}

	/// Sychronously calls a Luau function, without support for yielding. If the
	/// called code attempts to yield, an exception is raised and an Error is
	/// returned.
	pub fn call_sync<A: AsRef<[Value<'borrow, 'thread, UD>]>>(&self, args: A) -> Result<Vec<Self>, Error> {
		if !matches!(self.value, StackValue::Function(_)) {
			panic!("attempt to call a non-function");
		}

		let state = self.thread.as_ptr();
		let args = args.as_ref();

		if unsafe { lua_checkstack(state, (args.len() + 1) as _) } == 0 {
			return Err(Error::OutOfStack)
		}

		let nresults = unsafe {
			let base = lua_gettop(state);
			self.push_value(state).map_err(|_| Error::OutOfStack)?;

			for value in args {
				value.push_value(state).map_err(|_| Error::OutOfStack)?;
			}

			match lua_pcall(state, args.len() as _, LUA_MULTRET, 0) as lua_Status {
				luau_sys::luau::lua_Status_LUA_OK => (lua_gettop(state) - base) as usize,
				luau_sys::luau::lua_Status_LUA_YIELD => return Err(Error::Yielded),
				luau_sys::luau::lua_Status_LUA_ERRRUN => return Err(Error::pop_runtime_error(state)),
				luau_sys::luau::lua_Status_LUA_ERRMEM => return Err(Error::OutOfStack),
				whatever => {
					let unknown_string = format!("unknown ({})", whatever);
					panic!("Unexpected lua_pcall status encountered: {}", match whatever {
						luau_sys::luau::lua_Status_LUA_OK => "LUA_OK",
						luau_sys::luau::lua_Status_LUA_YIELD => "LUA_YIELD",
						luau_sys::luau::lua_Status_LUA_ERRRUN => "LUA_ERRRUN",
						luau_sys::luau::lua_Status_LUA_ERRSYNTAX => "LUA_ERRSYNTAX",
						luau_sys::luau::lua_Status_LUA_ERRMEM => "LUA_ERRMEM",
						luau_sys::luau::lua_Status_LUA_ERRERR => "LUA_ERRERR",
						luau_sys::luau::lua_Status_LUA_BREAK => "LUA_BREAK",
						_ => &unknown_string
					})
				}
			}
		};

		let mut results = Vec::new();
		results.resize_with(nresults, || MaybeUninit::uninit());

		for slot in results.iter_mut().rev() {
			*slot = MaybeUninit::new(unsafe { Self::pop_from_stack(self.thread) })
		}

		// SAFETY: All values have been initialized from the Lua stack
		Ok(unsafe { std::mem::transmute(results) })
	}
}

impl<'borrow, 'thread: 'borrow, UD: ThreadUserdata> Drop for Value<'borrow, 'thread, UD> {
	fn drop(&mut self) {
		if let Some(ref_) = self.ref_ {
			unsafe { lua_unref(self.thread.as_ptr(), ref_.get() as _); }
		}
	}
}

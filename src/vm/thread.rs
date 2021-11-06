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
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

use luau_sys::luau::{global_State, LUA_ENVIRONINDEX, lua_mainthread, lua_State, luaE_freethread, luau_load};
use crate::compiler::Chunk;

use crate::vm::{Error, Luau};
use crate::vm::types::function::Function;

/// Luau threads contain a special `userdata` field that can be set to be any
/// pointer. This can be used for access controls or other security features, or
/// it could be used for really anything else you want. Each thread under a
/// `Luau<UD>` will be given an instance of the userdata type `UD`, and the host
/// application can easily check the userdata of threads that call its
/// functions.
pub trait ThreadUserdata: Sized {
	fn inherit(parent: &Thread<Self>) -> Self;
}

impl ThreadUserdata for () {
	fn inherit(_parent: &Thread<Self>) -> Self {
		()
	}
}

/// Raw thread primitive, representing a thread in Luau. It may be the main
/// thread or a secondary one, and there is no Drop impl. Roughly equivalent to
/// a raw pointer to the thread. It represents the thread's ownership of the
/// userdata.
#[repr(C)]
pub struct Thread<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> {
	pub vm: &'borrow Luau<'vm, UD>,
	pub state: &'thread mut lua_State,
	phantom: PhantomData<Box<UD>>
}

impl<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> Thread<'borrow, 'thread, 'vm, UD> {
	pub unsafe fn from_lua_state_in_vm(vm: &'borrow Luau<'vm, UD>, state: &'thread mut lua_State) -> Self {
		Self { vm, state, phantom: PhantomData }
	}

	pub fn get_global(&self) -> NonNull<global_State> {
		unsafe { NonNull::new_unchecked(self.state.global) }
	}

	/// Returns a shared reference to the thread's userdata.
	pub fn userdata(&self) -> &UD {
		unsafe { NonNull::new_unchecked(self.state.userdata as _).as_ref() }
	}

	/// Returns an exclusive reference to the thread's userdata.
	pub fn userdata_mut(&mut self) -> &mut UD {
		unsafe { NonNull::new_unchecked(self.state.userdata as _).as_mut() }
	}

	pub fn load(&'borrow mut self, chunk: Chunk, chunkname: &str) -> Result<Function<'borrow, 'thread, 'vm, UD>, Error> {
		Function::load(self, chunk, chunkname)
	}
}

/// References the main thread of a Luau virtual machine.
pub struct MainThread<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread>(pub(crate) Thread<'borrow, 'thread, 'vm, UD>);

impl<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> MainThread<'borrow, 'thread, 'vm, UD> {
	pub unsafe fn from_lua_state_in_vm(vm: &'borrow Luau<'vm, UD>, state: &'thread mut lua_State) -> Self {
		Self(Thread::from_lua_state_in_vm(vm, state))
	}
}

impl<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> Deref for MainThread<'borrow, 'thread, 'vm, UD> {
	type Target = Thread<'borrow, 'thread, 'vm, UD>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> DerefMut for MainThread<'borrow, 'thread, 'vm, UD> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

/// Represents a borrowed secondary thread of a Luau virtual machine.
pub struct Coroutine<'thread, 'vm: 'thread, UD: ThreadUserdata + 'thread>(pub(crate) Thread<'thread, 'thread, 'vm, UD>);

impl<'thread, 'vm: 'thread, UD: ThreadUserdata + 'thread> Coroutine<'thread, 'vm, UD> {
	pub unsafe fn from_lua_state(vm: &'vm Luau<UD>, state: &'thread mut lua_State) -> Self {
		Self(Thread::from_lua_state_in_vm(vm, state))
	}

	pub fn get_main_thread(&self) -> &MainThread<UD> {
		unsafe { std::mem::transmute(&(*self.0.get_global().as_ptr()).mainthread) }
	}
}

impl<'thread, UD: ThreadUserdata + 'thread> Drop for Coroutine<'thread, '_, UD> {
	fn drop(&mut self) {
		// SAFETY: This Thread represents a real thread. It is being freed in
		// the context of its own VM as kept by Luau.
		unsafe { luaE_freethread(lua_mainthread(self.0.state as _), self.0.state as _) }
	}
}

impl<'thread, 'vm: 'thread, UD: ThreadUserdata + 'thread> Deref for Coroutine<'thread, 'vm, UD> {
	type Target = Thread<'thread, 'thread, 'vm, UD>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'thread, UD: ThreadUserdata + 'thread> DerefMut for Coroutine<'thread, '_, UD> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

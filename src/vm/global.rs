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

use std::marker::PhantomData;
use std::ptr::NonNull;

use luau_sys::luau::{global_State, lua_close};

use crate::compiler::{compile, CompileError};
use crate::vm::thread::{MainThread, Thread, ThreadUserdata};
use crate::vm::types::function::Function;

/// Represents a Luau virtual machine. This is the global state that manages all
/// coroutines.
pub struct Luau<'vm, UD: ThreadUserdata> {
	global_state: &'vm mut global_State,
	phantom: PhantomData<&'vm mut UD>
}

// SAFETY: It is safe to send Luau VMs across threads when nothing is using it.
unsafe impl<UD: ThreadUserdata> Send for Luau<'_, UD> {}

impl<'vm, UD: ThreadUserdata> Luau<'vm, UD> {
	fn setup_new_thread(thread: &mut Thread<'_, '_, '_, UD>, userdata: UD) {
		// Allocate a spot for the userdata and pin it there
		let mut userdata = Box::new(userdata);

		thread.state.userdata = userdata.as_mut() as *mut UD as *mut std::ffi::c_void;

		// Prevent the userdata from being deallocated while thread userdata holds a pointer to it
		Box::leak(userdata);
	}

	/// Attempts to create a new Luau virtual machine using the provided
	/// userdata type. Every thread in the VM will have its own unique instance
	/// of the userdata. The virtual machine is created using `luaL_newstate`.
	/// `None` is returned if there was a memory allocation error.
	///
	/// The returned virtual machine only has a freshly-initialized main thread.
	pub fn new_with_userdata(userdata: UD) -> Option<Self> {
		// Using NonNull like this allows us to handle the case where
		// luaL_newstate fails to allocate, by returning `None` early
		let mut main_thread = NonNull::new(unsafe { luau_sys::luau::luaL_newstate() })?;

		// This is the global_State object, which comprises the actual VM
		// It should be impossible to create a thread without a global_State, so
		// this is safe.
		let global_state = unsafe { NonNull::new_unchecked(main_thread.as_ref().global).as_mut() };

		// This VM is only used to construct the MainThread to set it up.
		let vm = Luau { global_state, phantom: PhantomData };

		// SAFETY: All safety guarantees of Thread and MainThread are upheld.
		let mut main_thread = unsafe { MainThread::from_lua_state_in_vm(&vm, main_thread.as_mut()) };

		// Set up the new thread with the userdata that we've been passed
		Self::setup_new_thread(&mut main_thread, userdata);

		// Don't close the VM state early, or instant UAF
		std::mem::forget(vm);

		Some(Self { global_state, phantom: PhantomData })
	}

	pub fn main_thread<'borrow>(&'borrow self) -> MainThread<'borrow, 'vm, 'vm, UD> {
		unsafe { MainThread::from_lua_state_in_vm(self, NonNull::new_unchecked(self.global_state.mainthread).as_mut()) }
	}
}

impl Luau<'_, ()> {
	/// Attempts to create a new Luau virtual machine without any userdata type.
	/// The virtual machine is created using `luaL_newstate`. `None` is returned
	/// if there was a memory allocation error.
	///
	/// The returned virtual machine only has a freshly-initialized main thread.
	pub fn new() -> Option<Self> {
		Self::new_with_userdata(())
	}
}

impl<UD: ThreadUserdata> Drop for Luau<'_, UD> {
	fn drop(&mut self) {
		unsafe { lua_close(self.global_state.mainthread) };
	}
}

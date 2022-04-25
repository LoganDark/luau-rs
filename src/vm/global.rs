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

use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::ptr::NonNull;

use luau_sys::luau::{global_State, lua_close, lua_State};

use crate::compiler::{compile, compile_sneakily, CompiledFunction, CompileError};
use crate::vm::Thread;
use crate::vm::thread::ThreadUserdata;

/// Represents an owned Luau virtual machine. All interaction with the VM must
/// be done through this struct; it is the logical owner of the virtual machine.
#[derive(Debug)]
pub struct Luau<UD: ThreadUserdata> {
	// ManuallyDrop: we free the VM manually on drop
	// Box: we logically own this value
	// UnsafeCell: Luau keeps internal references to the global_State/lua_State
	global_state: ManuallyDrop<Box<UnsafeCell<global_State>>>,
	main_thread: ManuallyDrop<Box<UnsafeCell<lua_State>>>,
	phantom: PhantomData<*mut UD> // this is inside the lua_State
}

// SAFETY: It is safe to send the Luau VM to a different thread.
unsafe impl<UD: ThreadUserdata> Send for Luau<UD> {}

impl<UD: ThreadUserdata> Drop for Luau<UD> {
	fn drop(&mut self) {
		// SAFETY: All of the UD objects will be deallocated by the userthread
		// callback in lua_Callbacks
		unsafe { lua_close((*self.global_state.get()).mainthread) };
	}
}

// It is not safe to use Luau concurrently from multiple threads, due to
// interior mutability.

pub(crate) fn ptr_to_ref<'a, T>(ptr: *mut T) -> Option<&'a UnsafeCell<T>> {
	Some(unsafe { &*(NonNull::new(ptr)?.as_ptr() as *mut UnsafeCell<T>) })
}

impl<UD: ThreadUserdata> Luau<UD> {
	/// Attempts to create a new Luau virtual machine, using the provided
	/// userdata for the main thread. The virtual machine is created using
	/// `luaL_newstate`. `None` is returned if there was an allocation error.
	pub fn with_userdata(userdata: Pin<Box<UD>>) -> Option<Self> {
		// In Lua, lua_newstate (and luaL_newstate) return the first thread in
		// the VM, but the VM itself is a separate data structure called
		// global_State. We have to grab the VM manually if we want to implement
		// thread-agnostic abstractions.
		let main_thread = unsafe {
			let ref_ = NonNull::new(luau_sys::luau::luaL_newstate())?.as_ptr();
			ManuallyDrop::new(Box::from_raw(ref_ as *mut UnsafeCell<lua_State>))
		};

		let global_state = unsafe {
			let ref_ = NonNull::new((*main_thread.get()).global)?.as_ptr();
			ManuallyDrop::new(Box::from_raw(ref_ as *mut UnsafeCell<global_State>))
		};

		unsafe {
			(*main_thread.get()).userdata = Box::leak(Pin::into_inner_unchecked(userdata)) as *mut UD as *mut c_void;

			extern "C" fn userthread(parent: *mut lua_State, child: *mut lua_State) {
				let parent = ptr_to_ref(parent);
				let child = ptr_to_ref(child).unwrap();

				println!("parent: {:?}, child: {:?}", parent, child);
			}

			(*global_state.get()).cb.userthread = Some(userthread);
		}

		Some(Self { global_state, main_thread, phantom: PhantomData })
	}

	pub fn main_thread(&self) -> Thread<UD> {
		unsafe { Thread::new(self, self.main_thread.as_ref()) }
	}
}

impl Luau<()> {
	/// Attempts to create a new Luau virtual machine, with no userdata type.
	/// The virtual machine is created using `luaL_newstate`. `None` is returned
	/// if there was an allocation error.
	pub fn new() -> Option<Self> {
		Self::with_userdata(Box::pin(()))
	}

	/// Compiles Luau source code. The compiled function can then be loaded into
	/// a thread and executed.
	#[cfg(feature = "compiler")]
	pub fn compile(source: &str) -> Result<CompiledFunction, CompileError> {
		compile(source, &Default::default(), &Default::default())
	}

	/// Compiles Luau source code. The compiled function can then be loaded into
	/// a thread and executed.
	///
	/// If there is an error parsing or compiling the bytecode, the error will
	/// actually be returned as a CompiledFunction that will always throw a
	/// runtime error on load.
	#[cfg(feature = "compiler")]
	pub fn compile_sneakily(source: &str) -> CompiledFunction {
		compile_sneakily(source, &Default::default(), &Default::default())
	}
}

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

use std::marker::PhantomData;
use std::ptr::NonNull;

use luau_sys::luau::lua_State;

use crate::compiler::{compile, compile_sneakily, CompiledFunction, CompileError};
use crate::vm::{RawGlobal, RawThread, Thread, UserdataContainer};
use crate::vm::thread::ThreadUserdata;

#[derive(Debug)]
pub struct Luau<UD: ThreadUserdata> {
	global: NonNull<RawGlobal>,
	main_thread: NonNull<RawThread>,
	phantom: PhantomData<NonNull<UD>> // owned by each thread
}

unsafe impl<UD: ThreadUserdata> Send for Luau<UD> {}

impl<UD: ThreadUserdata> Drop for Luau<UD> {
	fn drop(&mut self) { unsafe { RawGlobal::close(self.global) } }
}

impl<UD: ThreadUserdata> Luau<UD> {
	pub unsafe fn from_global(global: NonNull<RawGlobal>) -> Option<NonNull<Self>> {
		NonNull::new(global.as_ref().cb.userdata).map(NonNull::cast)
	}

	pub unsafe fn from_global_unchecked(global: NonNull<RawGlobal>) -> NonNull<Self> {
		NonNull::new_unchecked(global.as_ref().cb.userdata).cast()
	}

	pub fn with_userdata(userdata: UserdataContainer<UD>) -> Option<Self> {
		unsafe {
			let mut main_thread = RawThread::new()?;
			let mut global_state = main_thread.as_ref().global();

			main_thread.as_mut().set_userdata(userdata);
			global_state.as_mut().cb.userthread = Some(userthread::<UD>);

			unsafe extern "C" fn userthread<UD: ThreadUserdata>(parent: *mut lua_State, child: *mut lua_State) {
				let (parent, mut child) = (RawThread::from(parent), RawThread::from_unchecked(child));

				if let Some(parent) = parent {
					let parent = Thread::from_raw(parent);
					let userdata = UD::derive(parent);
					child.as_mut().set_userdata(userdata);
				} else {
					// drop userdata
					child.as_ref().get_userdata::<UD>();
				}
			}

			Some(Self { global: global_state, main_thread, phantom: PhantomData })
		}
	}

	pub fn main_thread(&self) -> Thread<UD> { unsafe { Thread::from_raw(self.main_thread) } }
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

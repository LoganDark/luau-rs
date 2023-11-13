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

use data::{Data, GlobalData, ThreadData};
use luau_sys::luau::lua_State;
use value::thread::Thread;

use crate::compiler::{compile, compile_sneakily, CompiledFunction, CompileError};
use crate::vm::builder::LuauBuildData;
use crate::vm::error::LResult;
use crate::vm::raw::RawGlobal;
use crate::vm::raw::thread::RawThread;
use crate::vm::value::LuauValue;

pub mod error;
pub mod raw;
pub mod value;
pub mod data;
pub mod builder;

#[derive(Debug)]
pub struct Luau<D: GlobalData> {
	global: NonNull<RawGlobal>,
	phantom: PhantomData<NonNull<D>>
}

unsafe impl<D: GlobalData> Send for Luau<D> {}

impl<D: GlobalData> Drop for Luau<D> {
	fn drop(&mut self) { unsafe { RawGlobal::close(self.global) } }
}

impl<D: GlobalData> Luau<D> {
	pub(crate) fn new(global_data: Data<D>, thread_data: Data<D::ThreadData>) -> Option<Self> {
		unsafe {
			let mut global = RawGlobal::new()?;
			global.as_ref().set_userdata(global_data);
			global.as_ref().main_thread().as_ref().set_userdata(thread_data);
			global.as_mut().cb.userthread = Some(userthread::<D::ThreadData>);

			unsafe extern "C" fn userthread<TD: ThreadData>(parent: *mut lua_State, child: *mut lua_State) {
				let (parent, child) = (RawThread::from(parent), RawThread::from_unchecked(child));

				if let Some(parent) = parent {
					let parent = Thread::from_raw(parent.as_ref());
					let userdata = TD::derive(&parent);
					child.as_ref().set_userdata(userdata);
				} else {
					child.as_ref().get_userdata::<TD>(); // drop
				}
			}

			Some(Self { global, phantom: PhantomData })
		}
	}
}

impl Luau<()> {
	pub fn builder() -> LuauBuildData { LuauBuildData }

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

impl<D: GlobalData> Luau<D> {
	pub unsafe fn main_thread(&self) -> &Thread {
		unsafe { NonNull::from(&self.global.as_ref().mainthread).cast().as_ref() }
	}

	pub fn new_thread(&self) -> LResult<LuauValue<Thread>> {
		unsafe { self.main_thread() }.new_thread()
	}
}

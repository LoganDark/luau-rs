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
use std::pin::Pin;
use std::ptr::NonNull;

use luau_sys::luau::lua_State;
use value::thread::Thread;

use crate::compiler::{compile, compile_sneakily, CompiledFunction, CompileError};
use crate::vm::raw::RawGlobal;
use crate::vm::raw::thread::RawThread;

pub mod raw;
pub mod value;

pub type Data<T> = Pin<Box<T>>;

#[derive(Debug, Default)]
pub enum Response {
	#[default]
	None,
	Yield
}

#[allow(unused_variables)]
pub trait GlobalData: Sized {
	type ThreadData: ThreadData;

	fn interrupt(thread: Thread<Self::ThreadData>) -> Response { Response::None }
	fn debug_break(thread: Thread<Self::ThreadData>) -> Response { Response::None }
	fn debug_step(thread: Thread<Self::ThreadData>) -> Response { Response::None }
	fn debug_interrupt(thread: Thread<Self::ThreadData>) -> Response { Response::None }
	fn debug_protectederror(thread: Thread<Self::ThreadData>) -> Response { Response::None }
}

pub trait ThreadData: Sized {
	fn derive(parent: Thread<Self>) -> Data<Self>;
}

impl GlobalData for () {
	type ThreadData = ();
}

impl ThreadData for () {
	fn derive(_parent: Thread<Self>) -> Data<Self> { Box::pin(()) }
}

#[derive(Debug)]
pub struct Luau<GD: GlobalData> {
	global: NonNull<RawGlobal>,
	phantom: PhantomData<NonNull<GD>>
}

unsafe impl<GD: GlobalData> Send for Luau<GD> {}

impl<GD: GlobalData> Drop for Luau<GD> {
	fn drop(&mut self) { unsafe { RawGlobal::close(self.global) } }
}

impl<GD: GlobalData> Luau<GD> {
	pub fn with_data(global_data: Data<GD>, thread_data: Data<GD::ThreadData>) -> Option<Self> {
		unsafe {
			let mut global = RawGlobal::new()?;
			global.as_ref().set_userdata(global_data);
			global.as_ref().main_thread().as_ref().set_userdata(thread_data);
			global.as_mut().cb.userthread = Some(userthread::<GD::ThreadData>);

			unsafe extern "C" fn userthread<TD: ThreadData>(parent: *mut lua_State, child: *mut lua_State) {
				let (parent, child) = (RawThread::from(parent), RawThread::from_unchecked(child));

				if let Some(parent) = parent {
					let parent = Thread::from_raw(parent);
					let userdata = TD::derive(parent);
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
	/// Attempts to create a new Luau virtual machine, with no userdata type.
	/// The virtual machine is created using `luaL_newstate`. `None` is returned
	/// if there was an allocation error.
	pub fn new() -> Option<Self> {
		Self::with_data(Box::pin(()), Box::pin(()))
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

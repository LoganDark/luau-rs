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

use std::cell::UnsafeCell;
use std::ffi::CStr;
use std::hint::unreachable_unchecked;

use luau_sys::luau::lua_State;

use crate::compiler::{compile, CompiledFunction, CompileError};
use crate::vm::{Error, Luau, Value};
use crate::vm::global::ptr_to_ref;
use crate::vm::types::function::Function;

pub trait ThreadUserdata {}

impl ThreadUserdata for () {}

#[derive(Clone, Debug)]
pub struct Thread<'a, UD: ThreadUserdata> {
	vm: &'a Luau<UD>,
	state: &'a UnsafeCell<lua_State>,
	userdata: &'a UnsafeCell<UD>
}

#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum LoadError {
	#[error("compile error: {0}")]
	CompileError(#[from] CompileError),

	#[error("load error: {0}")]
	LoadError(#[from] Error)
}

impl<'a, UD: ThreadUserdata> Thread<'a, UD> {
	pub unsafe fn new(vm: &'a Luau<UD>, state: &'a UnsafeCell<lua_State>) -> Self {
		let userdata = match ptr_to_ref((*state.get()).userdata as *mut UD) {
			Some(ud) => ud,
			// SAFETY: Luau guarantees that all created threads will have fully
			// initialized and valid userdata.
			None => unreachable_unchecked()
		};

		Self { vm, state, userdata }
	}

	pub fn vm(&self) -> &'a Luau<UD> {
		self.vm
	}

	pub fn as_ptr(&self) -> *mut lua_State {
		self.state.get()
	}

	pub fn userdata_ptr(&self) -> *mut UD {
		self.userdata.get()
	}

	pub fn load_compiled<'b>(&'b self, compiled: CompiledFunction, chunkname: &CStr) -> Result<Function<'b, '_, UD>, Error> {
		Ok(Function(Value::load_function(&self, compiled, chunkname)?))
	}

	pub fn load(&self, source: &str, chunkname: &CStr) -> Result<Function<UD>, LoadError> {
		let compiled = compile(source, &Default::default(), &Default::default())?;
		Ok(self.load_compiled(compiled, chunkname)?)
	}
}

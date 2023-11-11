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

use crate::compiler::CompileError;
use crate::vm::{Error, Luau, RawThread};

pub type UserdataContainer<T> = Pin<Box<T>>;

pub trait ThreadUserdata: Sized {
	fn derive(parent: Thread<Self>) -> UserdataContainer<Self>;
}

impl ThreadUserdata for () {
	fn derive(_parent: Thread<Self>) -> UserdataContainer<Self> { Box::pin(()) }
}

#[derive(Clone, Debug)]
pub struct Thread<'a, UD: ThreadUserdata> {
	raw: NonNull<RawThread>,
	phantom: PhantomData<&'a Luau<UD>>
}

#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum LoadError {
	#[error("compile error: {0}")]
	CompileError(#[from] CompileError),

	#[error("load error: {0}")]
	LoadError(#[from] Error)
}

impl<'a, UD: ThreadUserdata> Thread<'a, UD> {
	pub unsafe fn from_raw(raw: NonNull<RawThread>) -> Self {
		Self { raw, phantom: PhantomData }
	}
}

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

use luau_sys::luau::{lua_Type, TValue, Value};

use crate::vm::raw::thread::RawThread;
use crate::vm::ThreadData;
use crate::vm::value::AsTValue;

#[derive(Debug)]
pub struct Thread<D: ThreadData>(NonNull<RawThread>, PhantomData<D>);

impl<D: ThreadData> Clone for Thread<D> {
	fn clone(&self) -> Self { Self(self.0, self.1) }
}

unsafe impl<D: ThreadData> AsTValue for Thread<D> {
	fn as_tvalue(&self) -> TValue {
		TValue {
			value: Value { gc: self.0.as_ptr().cast() },
			extra: Default::default(),
			tt: lua_Type::LUA_TTHREAD as _
		}
	}
}

impl<D: ThreadData> Thread<D> {
	pub unsafe fn from_raw(raw: NonNull<RawThread>) -> Self { Self(raw, PhantomData) }
}

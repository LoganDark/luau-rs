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

use luau_sys::luau::{lua_Type, TValue, Value};

use crate::vm::value::AsTValue;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct LightUserdata(pub *mut ());

unsafe impl AsTValue for LightUserdata {
	fn as_tvalue(&self) -> TValue {
		TValue {
			value: Value { p: self.0.cast() },
			extra: Default::default(),
			tt: lua_Type::LUA_TLIGHTUSERDATA as _,
		}
	}
}

impl From<*mut ()> for LightUserdata {
	fn from(value: *mut ()) -> Self { Self(value) }
}

impl From<LightUserdata> for *mut () {
	fn from(value: LightUserdata) -> Self { value.0 }
}

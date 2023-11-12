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

use std::ffi::c_int;

use luau_sys::luau::{lua_Type, TValue, Value};

use crate::vm::value::AsTValue;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Vector(pub [f32; 3]);

unsafe impl AsTValue for Vector {
	fn as_tvalue(&self) -> TValue {
		let [x, y, z] = self.0;
		TValue {
			value: Value { v: [x, y] },
			extra: [c_int::from_ne_bytes(z.to_ne_bytes())],
			tt: lua_Type::LUA_TVECTOR as _,
		}
	}
}

impl From<[f32; 3]> for Vector {
	fn from(value: [f32; 3]) -> Self { Self(value) }
}

impl From<Vector> for [f32; 3] {
	fn from(value: Vector) -> Self { value.0 }
}

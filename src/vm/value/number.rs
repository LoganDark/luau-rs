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

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Number(pub f64);

unsafe impl AsTValue for Number {
	fn as_tvalue(&self) -> TValue {
		TValue {
			value: Value { n: self.0 },
			extra: Default::default(),
			tt: lua_Type::LUA_TNUMBER as _,
		}
	}
}

impl From<f64> for Number {
	fn from(value: f64) -> Self { Self(value) }
}

impl From<Number> for f64 {
	fn from(value: Number) -> Self { value.0 }
}

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

use crate::vm::{ThreadUserdata, Value};

pub mod function;

pub enum LuauType<'borrow, 'thread: 'borrow, UD: ThreadUserdata> {
	Nil,
	Boolean(bool),
	LightUserdata(*mut std::ffi::c_void),
	Number(f64),
	Vector([f32; 3]),
	String(Value<'borrow, 'thread, UD>),
	Table(Value<'borrow, 'thread, UD>),
	Function(Value<'borrow, 'thread, UD>),
	Userdata(Value<'borrow, 'thread, UD>),
	Thread(Value<'borrow, 'thread, UD>)
}

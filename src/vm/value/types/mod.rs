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

use string::StringValue;

use crate::vm::{Coroutine, ThreadUserdata, Value};

pub mod string;
pub mod function;

pub enum LuauType<'borrow, 'thread: 'borrow, 'vm: 'thread, UD: ThreadUserdata + 'thread> {
	Nil,
	Boolean(bool),
	LightUserdata(*mut std::ffi::c_void),
	Number(f64),
	Vector([f32; 3]),
	String(StringValue<'borrow, 'thread, 'vm, UD>),
	Table(Value<'borrow, 'thread, 'vm, UD>),
	Function(Value<'borrow, 'thread, 'vm, UD>),
	Userdata(Value<'borrow, 'thread, 'vm, UD>),
	Thread(Coroutine<'thread, 'vm, UD>)
}

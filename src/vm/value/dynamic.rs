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

use crate::vm::ThreadData;
use crate::vm::value::boolean::Boolean;
use crate::vm::value::buffer::Buffer;
use crate::vm::value::closure::Closure;
use crate::vm::value::lightuserdata::LightUserdata;
use crate::vm::value::nil::Nil;
use crate::vm::value::number::Number;
use crate::vm::value::string::LString;
use crate::vm::value::table::Table;
use crate::vm::value::thread::Thread;
use crate::vm::value::userdata::Userdata;
use crate::vm::value::vector::Vector;

#[derive(Clone, Debug)]
pub enum Dynamic<D: ThreadData> {
	Nil(Nil),
	Boolean(Boolean),
	LightUserdata(LightUserdata),
	Number(Number),
	Vector(Vector),
	String(LString),
	Table(Table),
	Closure(Closure),
	Userdata(Userdata),
	Thread(Thread<D>),
	Buffer(Buffer)
}

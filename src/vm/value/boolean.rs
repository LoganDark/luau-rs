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

use crate::vm::value::gc::Datatype;
use crate::vm::value::thread::Thread;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
#[repr(transparent)]
pub struct Boolean(pub bool);

impl<'a> Datatype<'a> for Boolean {
	type Ref = ();
	fn acquire_ref(&self, _thread: Thread<'a>) -> Option<Self::Ref> { Some(()) }
}

impl From<bool> for Boolean {
	fn from(value: bool) -> Self { Self(value) }
}

impl From<Boolean> for bool {
	fn from(value: Boolean) -> Self { value.0 }
}

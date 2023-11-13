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

use std::pin::Pin;

use crate::vm::Luau;
use crate::vm::value::LuauValue;
use crate::vm::value::thread::Thread;

pub type Data<T> = Pin<Box<T>>;

#[allow(unused_variables)]
pub trait GlobalData: Sized {
	type ThreadData: ThreadData;

	fn interrupt<'a>(global: &Luau<Self>, thread: LuauValue<'a, Thread<'a>>) {}
	fn debug_break<'a>(global: &Luau<Self>, thread: LuauValue<'a, Thread<'a>>) {}
	fn debug_step<'a>(global: &Luau<Self>, thread: LuauValue<'a, Thread<'a>>) {}
	fn debug_interrupt<'a>(global: &Luau<Self>, thread: LuauValue<'a, Thread<'a>>) {}
	fn debug_protectederror<'a>(global: &Luau<Self>, thread: LuauValue<'a, Thread<'a>>) {}
}

impl GlobalData for () {
	type ThreadData = ();
}

pub trait ThreadData: Sized {
	fn derive(parent: &Thread) -> Data<Self>;
}

impl ThreadData for () {
	fn derive(_parent: &Thread) -> Data<Self> { Box::pin(()) }
}

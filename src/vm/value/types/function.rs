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

use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

use crate::vm::{StackValue, ThreadUserdata, Value};

#[derive(Debug)]
pub struct Function<'borrow, 'thread: 'borrow, UD: ThreadUserdata>(Value<'borrow, 'thread, UD>);

impl<'borrow, 'thread: 'borrow, UD: ThreadUserdata> Display for Function<'borrow, 'thread, UD> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl<'borrow, 'thread: 'borrow, UD: ThreadUserdata> TryFrom<Value<'borrow, 'thread, UD>> for Function<'borrow, 'thread, UD> {
	type Error = ();

	fn try_from(value: Value<'borrow, 'thread, UD>) -> Result<Self, Self::Error> {
		if let StackValue::Function(_) = value.value() {
			Ok(Self(value))
		} else {
			Err(())
		}
	}
}

impl<'borrow, 'thread: 'borrow, UD: ThreadUserdata> Deref for Function<'borrow, 'thread, UD> {
	type Target = Value<'borrow, 'thread, UD>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'borrow, 'thread: 'borrow, UD: ThreadUserdata> DerefMut for Function<'borrow, 'thread, UD> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

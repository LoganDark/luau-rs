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

use bstr::BStr;

use crate::vm::StackValue;

macro_rules! value_wrapper {
	($name:ident, $pattern:pat) => {
		#[derive(Debug)]
		pub struct $name<'borrow, 'thread: 'borrow, UD: crate::vm::ThreadUserdata>(crate::vm::Value<'borrow, 'thread, UD>);

		impl<'borrow, 'thread: 'borrow, UD: crate::vm::ThreadUserdata> ::std::fmt::Display for $name<'borrow, 'thread, UD> {
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
				::std::fmt::Display::fmt(&self.0, f)
			}
		}

		impl<'borrow, 'thread: 'borrow, UD: crate::vm::ThreadUserdata> ::std::convert::TryFrom<crate::vm::Value<'borrow, 'thread, UD>> for $name<'borrow, 'thread, UD> {
			type Error = ();

			fn try_from(value: crate::vm::Value<'borrow, 'thread, UD>) -> Result<Self, Self::Error> {
				if let $pattern = value.value() {
					Ok(Self(value))
				} else {
					Err(())
				}
			}
		}

		impl<'borrow, 'thread: 'borrow, UD: crate::vm::ThreadUserdata> ::std::ops::Deref for $name<'borrow, 'thread, UD> {
			type Target = crate::vm::Value<'borrow, 'thread, UD>;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl<'borrow, 'thread: 'borrow, UD: crate::vm::ThreadUserdata> ::std::ops::DerefMut for $name<'borrow, 'thread, UD> {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}
	}
}

value_wrapper!(Nil, StackValue::Nil);
value_wrapper!(Boolean, StackValue::Boolean(_));
value_wrapper!(LightUserdata, StackValue::LightUserdata(_)); // TODO generic over pointed-to type
value_wrapper!(Number, StackValue::Number(_));
value_wrapper!(Vector, StackValue::Vector(_));
value_wrapper!(String, StackValue::String(_));
value_wrapper!(Table, StackValue::Table(_));
value_wrapper!(Function, StackValue::Function(_));
value_wrapper!(Userdata, StackValue::Userdata(_)); // TODO generic over pointed-to type
value_wrapper!(Thread, StackValue::Thread(_)); // TODO generic over ThreadUserdata

macro_rules! wrapper_constructor {
	($name:ident, $thread:ident $(,$($arg:ident : $ty:ty),*)? ; $body:block) => {
		impl<'borrow, 'thread: 'borrow, UD: crate::vm::ThreadUserdata> $name<'borrow, 'thread, UD> {
			pub fn new($thread: &'borrow crate::vm::Thread<'thread, UD>$(, $($arg : $ty),*)?) -> Self $body
		}
	};

	($struct:ident => ::$variant:ident $(($($name:ident : $ty:ty),+))?) => {
		wrapper_constructor!($struct, thread$(, $($name: $ty),+)?; {
			Self(crate::vm::Value::new_value(thread, crate::vm::StackValue::$variant$(($($name),+))?).unwrap())
		});
	};

	($struct:ident => $method:ident ($($name:ident : $ty:ty),+)) => {
		wrapper_constructor!($struct, thread, $($name: $ty),+; {
			Self(crate::vm::Value::$method(thread, $($name),+).unwrap())
		});
	}
}

wrapper_constructor!(Nil => ::Nil);
wrapper_constructor!(Boolean => ::Boolean(value: bool));
// TODO lightuserdata
wrapper_constructor!(Number => ::Number(value: f64));
wrapper_constructor!(Vector => ::Vector(value: [f32; 3]));
wrapper_constructor!(String => new_string(value: &BStr));
wrapper_constructor!(Table => new_table(narray: u32, nhash: u32));
// TODO function, userdata, threads

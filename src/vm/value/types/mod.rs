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

use std::cell::UnsafeCell;
use std::pin::Pin;

use bstr::BStr;

use crate::vm::StackValue;

/// Trait implemented by objects that can be pointed to by LightUserdatas. All
/// types each have their own Luau tag that can be used to safely check if a
/// LightUserdata points to that type of object.
pub trait LightUserdataTarget {}

/// Trait implemented by objects that can be pointed to by full Userdatas.
pub trait UserdataTarget: LightUserdataTarget {}

macro_rules! value_wrapper {
	($struct:ident : $udataname:ident $(<$($name:ident : $bound:path : $phantomtype:ty),+>)?, $pattern:pat) => {
		#[derive(Debug)]
		pub struct $struct<'borrow, 'thread: 'borrow, $udataname: crate::vm::ThreadUserdata$(, $($name: $bound),+)?>(crate::vm::Value<'borrow, 'thread, UD>$(, $(::core::marker::PhantomData<$phantomtype>),+)?);

		impl<'borrow, 'thread: 'borrow, $udataname: crate::vm::ThreadUserdata$(, $($name: $bound),+)?> ::std::fmt::Display for $struct<'borrow, 'thread, $udataname$(, $($name),+)?> {
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
				::std::fmt::Display::fmt(&self.0, f)
			}
		}

		impl<'borrow, 'thread: 'borrow, $udataname: crate::vm::ThreadUserdata$(, $($name: $bound),+)?> ::std::convert::TryFrom<crate::vm::Value<'borrow, 'thread, $udataname>> for $struct<'borrow, 'thread, $udataname$(, $($name),+)?> {
			type Error = ();

			fn try_from(value: crate::vm::Value<'borrow, 'thread, $udataname>) -> Result<Self, Self::Error> {
				if let $pattern = value.value() {
					Ok(Self(value$(, $(::core::marker::PhantomData::<$phantomtype>),+)?))
				} else {
					Err(())
				}
			}
		}

		impl<'borrow, 'thread: 'borrow, $udataname: crate::vm::ThreadUserdata$(, $($name: $bound),+)?> ::std::ops::Deref for $struct<'borrow, 'thread, $udataname$(, $($name),+)?> {
			type Target = crate::vm::Value<'borrow, 'thread, $udataname>;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl<'borrow, 'thread: 'borrow, $udataname: crate::vm::ThreadUserdata$(, $($name: $bound),+)?> ::std::ops::DerefMut for $struct<'borrow, 'thread, $udataname$(, $($name),+)?> {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}
	};

	($struct:ident $(<$($name:ident : $bound:path : $phantomtype:ty),+>)?, $pattern:pat) => {
		value_wrapper!($struct: UD $(<$($name: $bound: $phantomtype),+>)?, $pattern);
	}
}

value_wrapper!(Nil, StackValue::Nil);
value_wrapper!(Boolean, StackValue::Boolean(_));
value_wrapper!(LightUserdata<T: LightUserdataTarget: &'borrow UnsafeCell<T>>, StackValue::LightUserdata(_));
value_wrapper!(Number, StackValue::Number(_));
value_wrapper!(Vector, StackValue::Vector(_));
value_wrapper!(String, StackValue::String(_));
value_wrapper!(Table, StackValue::Table(_));
value_wrapper!(Function, StackValue::Function(_));
value_wrapper!(Userdata<T: UserdataTarget: &'borrow UnsafeCell<T>>, StackValue::Userdata(_));
value_wrapper!(Thread: UD, StackValue::Thread(_));

macro_rules! wrapper_constructor {
	($name:ident : $udataname: ident, $thread:ident $(,$($arg:ident : $ty:ty),*)? ; $body:block) => {
		impl<'borrow, 'thread: 'borrow, $udataname: crate::vm::ThreadUserdata> $name<'borrow, 'thread, $udataname> {
			pub fn new($thread: &'borrow crate::vm::Thread<'thread, UD>$(, $($arg: $ty),*)?) -> Self $body
		}
	};

	($struct:ident => ::$variant:ident $(($($name:ident : $ty:ty),+))?) => {
		wrapper_constructor!($struct: UD, thread$(, $($name: $ty),+)?; {
			Self(crate::vm::Value::new_value(thread, crate::vm::StackValue::$variant$(($($name),+))?).unwrap())
		});
	};

	($struct:ident => $method:ident ($($name:ident : $ty:ty),+)) => {
		wrapper_constructor!($struct: UD, thread, $($name: $ty),+; {
			Self(crate::vm::Value::$method(thread, $($name),+).unwrap())
		});
	};

	($struct:ident : $udataname:ident => $method:ident ($($name:ident : $ty:ty),+)) => {
		wrapper_constructor!($struct: $udataname, thread, $($name: $ty),+; {
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
// TODO function, userdata
wrapper_constructor!(Thread: UD => new_thread(userdata: Pin<Box<UD>>));

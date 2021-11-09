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

use std::ffi::c_void;
use std::hint::unreachable_unchecked;
use std::marker::PhantomData;

use __sealed::FValueType;

mod __sealed {
	use std::ffi::c_void;
	use std::os::raw::c_int;
	use std::ptr::NonNull;

	use luau_sys::glue;
	use luau_sys::glue::{gluau_Buffer, gluau_fflag_get, gluau_find_fflag, gluau_find_fint, gluau_fint_get, gluau_fint_set, gluau_get_fflag_name, gluau_get_fflags, gluau_get_fint_name, gluau_get_fints};

	use crate::luau_sys::glue::{gluau_fflag_set, gluau_OptionalFValue};

	pub trait FValueType: Copy {
		fn find(search: &str) -> Option<*mut c_void>;
		fn list() -> Option<Vec<*mut c_void>>;
		fn name(fvalue: *mut c_void) -> &'static str;
		fn value(fvalue: *mut c_void) -> Self;
		fn set(self, fvalue: *mut c_void);
	}

	fn str2buf(str: &str) -> gluau_Buffer {
		gluau_Buffer {
			data: str.as_ptr() as _,
			len: str.len() as _
		}
	}

	fn optional_fvalue_to_optional_fvalue(optional_fvalue: gluau_OptionalFValue) -> Option<*mut c_void> {
		match unsafe { optional_fvalue } {
			gluau_OptionalFValue { presence: glue::gluau_Optionality_Some, value } => {
				Some(value)
			}

			_ => None
		}
	}

	unsafe fn fvalues2vec(fvalues: *mut *mut c_void) -> Option<Vec<*mut c_void>> {
		NonNull::new(fvalues).map(|nonnull| {
			let ptr = nonnull.as_ptr();

			let len = {
				let mut pos = (0usize, ptr);

				loop {
					if unsafe { *pos.1 }.is_null() {
						break pos.0
					}

					pos = (pos.0 + 1, unsafe { pos.1.offset(1) })
				}
			};

			unsafe { Vec::from_raw_parts(ptr, len, len + 1) }
		})
	}

	unsafe fn buf2str(buf: gluau_Buffer) -> &'static str {
		std::str::from_utf8_unchecked(std::slice::from_raw_parts(buf.data as _, buf.len as _))
	}

	impl FValueType for bool {
		fn find(search: &str) -> Option<*mut c_void> {
			optional_fvalue_to_optional_fvalue(unsafe { gluau_find_fflag(str2buf(search)) })
		}

		fn list() -> Option<Vec<*mut c_void>> {
			unsafe { fvalues2vec(gluau_get_fflags()) }
		}

		fn name(fvalue: *mut c_void) -> &'static str {
			unsafe { buf2str(gluau_get_fflag_name(fvalue)) }
		}

		fn value(fvalue: *mut c_void) -> Self {
			unsafe { gluau_fflag_get(fvalue) }
		}

		fn set(self, fvalue: *mut c_void) {
			unsafe { gluau_fflag_set(fvalue, self) }
		}
	}

	impl FValueType for c_int {
		fn find(search: &str) -> Option<*mut c_void> {
			optional_fvalue_to_optional_fvalue(unsafe { gluau_find_fint(str2buf(search)) })
		}

		fn list() -> Option<Vec<*mut c_void>> {
			unsafe { fvalues2vec(gluau_get_fints()) }
		}

		fn name(fvalue: *mut c_void) -> &'static str {
			unsafe { buf2str(gluau_get_fint_name(fvalue)) }
		}

		fn value(fvalue: *mut c_void) -> Self {
			unsafe { gluau_fint_get(fvalue) }
		}

		fn set(self, fvalue: *mut c_void) {
			unsafe { gluau_fint_set(fvalue, self) }
		}
	}
}

#[repr(transparent)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct FValue<T: FValueType> {
	inner: *mut c_void,
	phantom: PhantomData<T>
}

impl<T: FValueType> FValue<T> {
	pub fn find(name: &str) -> Option<Self> {
		T::find(name).map(|inner| Self { inner, phantom: PhantomData })
	}

	pub fn list() -> Vec<FValue<T>> {
		// SAFETY: FValue is repr(transparent)
		unsafe { std::mem::transmute(T::list().expect("couldn't allocate Vec")) }
	}

	pub fn name(&self) -> &str {
		T::name(self.inner)
	}

	pub fn value(&self) -> T {
		T::value(self.inner)
	}

	pub fn set(&mut self, value: T) {
		value.set(self.inner);
	}
}

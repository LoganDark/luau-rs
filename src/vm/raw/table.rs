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

use std::cell::UnsafeCell;
use std::convert::TryInto;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

use luau_sys::glue::gluauH_new;
use luau_sys::luau::{lua_Status, lua_Type, Table, TValue};

use crate::vm::raw::thread::RawThread;

#[derive(Debug)]
#[repr(transparent)]
pub struct RawTable(UnsafeCell<Table>);

impl Deref for RawTable {
	type Target = Table;
	fn deref(&self) -> &Self::Target { unsafe { &*self.0.get() } }
}

impl DerefMut for RawTable {
	fn deref_mut(&mut self) -> &mut Self::Target { self.0.get_mut() }
}

impl RawTable {
	pub fn from(ptr: *mut Table) -> Option<NonNull<Self>> { NonNull::new(ptr).map(NonNull::cast) }
	pub unsafe fn from_unchecked(ptr: *mut Table) -> NonNull<Self> { NonNull::new_unchecked(ptr).cast() }

	pub fn from_tvalue(tvalue: TValue) -> Option<NonNull<Self>> {
		if tvalue.tt == lua_Type::LUA_TTABLE as _ {
			Self::from(unsafe { tvalue.value.gc }.cast())
		} else {
			None
		}
	}

	pub unsafe fn new(thread: NonNull<RawThread>, narray: usize, lnhash: usize) -> Result<NonNull<Self>, lua_Status> {
		let mut result = MaybeUninit::<*mut Self>::uninit();
		let result_ptr = result.as_mut_ptr();

		match gluauH_new(thread.as_ref().ptr().cast(), narray.try_into().expect("narray overflow"), lnhash.try_into().expect("lnhash overflow"), result_ptr.cast()) {
			lua_Status::LUA_OK => Ok(NonNull::new_unchecked(result_ptr.read())),
			error => Err(error)
		}
	}

	pub fn ptr(&self) -> *mut Table { self.0.get() }
}

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
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

use luau_sys::luau::Udata;

#[derive(Debug)]
#[repr(transparent)]
pub struct RawUserdata(UnsafeCell<Udata>);

impl Deref for RawUserdata {
	type Target = Udata;
	fn deref(&self) -> &Self::Target { unsafe { &*self.0.get() } }
}

impl DerefMut for RawUserdata {
	fn deref_mut(&mut self) -> &mut Self::Target { self.0.get_mut() }
}

impl RawUserdata {
	pub fn from(ptr: *mut Udata) -> Option<NonNull<Self>> { NonNull::new(ptr).map(NonNull::cast) }
	pub unsafe fn from_unchecked(ptr: *mut Udata) -> NonNull<Self> { NonNull::new_unchecked(ptr).cast() }
}

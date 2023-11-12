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

use luau_sys::luau::Buffer;

#[derive(Debug)]
#[repr(transparent)]
pub struct RawBuffer(UnsafeCell<Buffer>);

impl Deref for RawBuffer {
	type Target = Buffer;
	fn deref(&self) -> &Self::Target { unsafe { &*self.0.get() } }
}

impl DerefMut for RawBuffer {
	fn deref_mut(&mut self) -> &mut Self::Target { self.0.get_mut() }
}

impl RawBuffer {
	pub fn from(ptr: *mut Buffer) -> Option<NonNull<Self>> { NonNull::new(ptr).map(NonNull::cast) }
	pub unsafe fn from_unchecked(ptr: *mut Buffer) -> NonNull<Self> { NonNull::new_unchecked(ptr).cast() }
	pub fn ptr(&self) -> *mut Buffer { self.0.get() }
}

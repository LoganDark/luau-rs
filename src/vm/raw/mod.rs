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
use std::pin::Pin;
use std::ptr::NonNull;

use luau_sys::luau::{global_State, luaL_newstate};
use thread::RawThread;

use crate::vm::data::Data;
use crate::vm::raw::table::RawTable;

pub mod value;
pub mod string;
pub mod table;
pub mod closure;
pub mod userdata;
pub mod thread;
pub mod buffer;

#[derive(Debug)]
#[repr(transparent)]
pub struct RawGlobal(UnsafeCell<global_State>);

impl Deref for RawGlobal {
	type Target = global_State;
	fn deref(&self) -> &Self::Target { unsafe { &*self.0.get() } }
}

impl DerefMut for RawGlobal {
	fn deref_mut(&mut self) -> &mut Self::Target { self.0.get_mut() }
}

impl RawGlobal {
	pub fn from(ptr: *mut global_State) -> Option<NonNull<Self>> { NonNull::new(ptr).map(NonNull::cast) }
	pub unsafe fn from_unchecked(ptr: *mut global_State) -> NonNull<Self> { NonNull::new_unchecked(ptr).cast() }
	pub unsafe fn of(thread: NonNull<RawThread>) -> NonNull<Self> { Self::from_unchecked(thread.as_ref().global) }
	pub fn new() -> Option<NonNull<Self>> { Some(unsafe { Self::of(RawThread::from(luaL_newstate())?) }) }

	pub fn ptr(&self) -> *mut global_State { self.0.get() }
	pub unsafe fn get_userdata<GD>(&self) -> Data<GD> { Pin::new_unchecked(Box::from_raw(self.cb.userdata.cast())) }
	pub unsafe fn set_userdata<GD>(&self, to: Data<GD>) { (*self.0.get()).cb.userdata = Box::into_raw(Pin::into_inner_unchecked(to)).cast() }
	pub unsafe fn main_thread(&self) -> NonNull<RawThread> { RawThread::of(NonNull::from(self)) }
	pub unsafe fn close(global: NonNull<Self>) { RawThread::close(global.as_ref().main_thread()) }

	pub unsafe fn registry(&self) -> NonNull<RawTable> { RawTable::from_unchecked(self.registry.value.gc.cast()) }
}

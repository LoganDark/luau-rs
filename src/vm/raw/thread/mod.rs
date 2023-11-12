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

use luau_sys::luau::{BLACKBIT, lua_close, lua_State, luaC_barrierback};

use crate::vm::data::Data;
use crate::vm::raw::RawGlobal;
use crate::vm::raw::table::RawTable;
use crate::vm::raw::thread::stack::RawStack;

pub mod stack;

#[derive(Debug)]
#[repr(transparent)]
pub struct RawThread(UnsafeCell<lua_State>);

impl Deref for RawThread {
	type Target = lua_State;
	fn deref(&self) -> &Self::Target { unsafe { &*self.0.get() } }
}

impl DerefMut for RawThread {
	fn deref_mut(&mut self) -> &mut Self::Target { self.0.get_mut() }
}

impl RawThread {
	pub fn from(ptr: *mut lua_State) -> Option<NonNull<Self>> { NonNull::new(ptr).map(NonNull::cast) }
	pub unsafe fn from_unchecked(ptr: *mut lua_State) -> NonNull<Self> { NonNull::new_unchecked(ptr).cast() }
	pub unsafe fn of(global: NonNull<RawGlobal>) -> NonNull<Self> { Self::from_unchecked(global.as_ref().mainthread) }

	pub fn ptr(&self) -> *mut lua_State { self.0.get() }
	pub unsafe fn get_userdata<TD>(&self) -> Data<TD> { Pin::new_unchecked(Box::from_raw(self.userdata.cast())) }
	pub unsafe fn set_userdata<TD>(&self, to: Data<TD>) { (*self.0.get()).userdata = Box::into_raw(Pin::into_inner_unchecked(to)).cast() }
	pub unsafe fn global(&self) -> NonNull<RawGlobal> { RawGlobal::of(NonNull::from(self)) }
	pub unsafe fn close(thread: NonNull<Self>) { lua_close(thread.as_ptr().cast()) }

	pub unsafe fn stack(&self) -> RawStack { RawStack::for_thread(NonNull::from(self)) }
	pub unsafe fn registry(&self) -> NonNull<RawTable> { self.global().as_ref().registry() }
	pub unsafe fn threadbarrier(&self) {
		if (self.marked & BLACKBIT as u8) > 0 {
			luaC_barrierback(self.ptr(), self.ptr().cast(), &mut (*self.ptr()).gclist);
		}
	}

	pub unsafe fn global_table(&self) -> NonNull<RawTable> { RawTable::from_unchecked(self.gt) }
}

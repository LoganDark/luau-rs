use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::ptr::NonNull;

use luau_sys::luau::{BLACKBIT, lua_close, lua_State, luaC_barrierback, StkId, TValue};

use crate::vm::Data;
use crate::vm::raw::RawGlobal;

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
	pub unsafe fn global(&self) -> NonNull<RawGlobal> { RawGlobal::of(NonNull::from(self)) }
	pub unsafe fn get_userdata<TD>(&self) -> Data<TD> { Pin::new_unchecked(Box::from_raw(self.userdata.cast())) }
	pub unsafe fn set_userdata<TD>(&self, to: Data<TD>) { (*self.0.get()).userdata = Box::into_raw(Pin::into_inner_unchecked(to)).cast() }
	pub unsafe fn close(thread: NonNull<Self>) { lua_close(thread.as_ptr().cast()) }

	pub unsafe fn threadbarrier(&self) {
		if (self.marked & BLACKBIT) > 0 {
			luaC_barrierback(self.ptr(), self.ptr().cast(), self.ptr().cast());
		}
	}

	pub unsafe fn push(&self, value: TValue) -> Option<StkId> {
		let ptr = self.ptr();

		if (*ptr).top < (*(*ptr).ci).top {
			self.threadbarrier();
			let top = (*ptr).top;
			top.write(value);
			(*ptr).top = top.add(1);
			Some(top)
		} else {
			None
		}
	}

	pub unsafe fn pop(&self) -> Option<TValue> {
		let ptr = self.ptr();

		if (*ptr).top > (*ptr).base {
			let top = (*ptr).top;
			(*ptr).top = top.sub(1);
			Some(top.read())
		} else {
			None
		}
	}
}

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

use std::mem::zeroed;
use std::ptr::{addr_of_mut, NonNull, replace};

use luau_sys::luau::TValue;

use crate::vm::raw::thread::RawThread;

#[derive(Debug)]
pub struct RawStack {
	thread: NonNull<RawThread>,
	min: NonNull<TValue>,
	max: NonNull<TValue>,
	top: NonNull<NonNull<TValue>>
}

impl RawStack {
	pub unsafe fn for_thread(thread: NonNull<RawThread>) -> Self {
		let ptr = thread.as_ref().ptr();

		Self {
			thread,
			min: NonNull::new_unchecked((*(*ptr).ci).base),
			max: NonNull::new_unchecked((*(*ptr).ci).top),
			top: NonNull::new_unchecked(addr_of_mut!((*ptr).top)).cast()
		}
	}

	pub fn min(&self) -> NonNull<TValue> { self.min }
	pub fn max(&self) -> NonNull<TValue> { self.max }
	pub fn top(&self) -> NonNull<TValue> { unsafe { *self.top.as_ptr() } }
	pub fn len(&self) -> usize { unsafe { self.max.as_ptr().offset_from(self.min.as_ptr()) as usize } }
	pub fn used(&self) -> usize { unsafe { self.top().as_ptr().offset_from(self.min.as_ptr()) as usize } }
	pub fn left(&self) -> usize { unsafe { self.max.as_ptr().offset_from(self.top().as_ptr()) as usize } }

	pub fn index_of(&self, value: NonNull<TValue>) -> Option<usize> {
		(value.as_ptr() >= self.min.as_ptr() && value.as_ptr() <= self.max.as_ptr())
			.then(move || self.index_of_unchecked(value))
	}

	pub fn index_of_unchecked(&self, value: NonNull<TValue>) -> usize {
		unsafe { value.as_ptr().offset_from(self.min.as_ptr()) as usize }
	}

	pub fn get(&self, index: usize) -> Option<NonNull<TValue>> {
		(index < self.len()).then(|| unsafe { self.get_unchecked(index) })
	}

	pub unsafe fn get_unchecked(&self, index: usize) -> NonNull<TValue> {
		NonNull::new_unchecked(self.min.as_ptr().add(index))
	}

	pub unsafe fn set_top(&self, top: NonNull<TValue>) {
		if self.index_of(top).is_some() {
			self.set_top_unchecked(top)
		}
	}

	pub unsafe fn set_top_unchecked(&self, top: NonNull<TValue>) {
		*self.top.as_ptr() = top;
	}

	pub unsafe fn alloc<const N: usize>(&self) -> Option<NonNull<[TValue; N]>> { (self.left() >= N).then(move || self.alloc_unchecked()) }
	pub unsafe fn free<const N: usize>(&self) -> Option<[TValue; N]> { (self.used() >= N).then(move || self.free_unchecked()) }

	pub unsafe fn alloc_unchecked<const N: usize>(&self) -> NonNull<[TValue; N]> {
		let region = self.top();
		self.set_top_unchecked(NonNull::new_unchecked(region.as_ptr().add(N)));
		region.cast()
	}

	pub unsafe fn free_unchecked<const N: usize>(&self) -> [TValue; N] {
		let region = NonNull::new_unchecked(self.top().as_ptr().sub(N));
		self.set_top_unchecked(region);
		replace(region.cast().as_ptr(), [zeroed(); N])
	}

	pub unsafe fn used_slice(&self) -> NonNull<[TValue]> { NonNull::slice_from_raw_parts(self.min, self.used()) }
	pub unsafe fn free_slice(&self) -> NonNull<[TValue]> { NonNull::slice_from_raw_parts(self.top(), self.left()) }

	pub unsafe fn push_many<const N: usize>(&self, values: [TValue; N]) -> Option<NonNull<[TValue; N]>> {
		let space = self.alloc()?;
		self.thread.as_ref().threadbarrier();
		*space.as_ptr() = values;
		Some(space)
	}

	pub unsafe fn push(&self, value: TValue) -> Option<NonNull<TValue>> {
		self.push_many([value]).map(NonNull::cast)
	}

	pub unsafe fn pop_many<const N: usize>(&self) -> Option<[TValue; N]> {
		self.free()
	}

	pub unsafe fn pop(&self) -> Option<TValue> {
		let [value] = self.pop_many::<1>()?;
		Some(value)
	}

	pub unsafe fn save_restore<T>(&self, closure: impl FnOnce(&RawStack) -> T) -> T {
		let top = self.top();
		let value = closure(self);
		self.set_top_unchecked(top);
		value
	}
}

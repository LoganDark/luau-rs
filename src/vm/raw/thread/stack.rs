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

use crate::vm::raw::thread::RawThread;
use crate::vm::raw::value::RawValue;

#[derive(Debug)]
pub struct RawStack {
	thread: NonNull<RawThread>,
	min: NonNull<RawValue>,
	max: NonNull<RawValue>,
	top: NonNull<NonNull<RawValue>>
}

impl RawStack {
	pub unsafe fn for_thread(thread: NonNull<RawThread>) -> Self {
		let ptr = thread.as_ref().ptr();

		Self {
			thread,
			min: NonNull::new_unchecked((*(*ptr).ci).base.cast()),
			max: NonNull::new_unchecked((*(*ptr).ci).top.cast()),
			top: NonNull::new_unchecked(addr_of_mut!((*ptr).top)).cast()
		}
	}

	pub fn min(&self) -> NonNull<RawValue> { self.min }
	pub fn max(&self) -> NonNull<RawValue> { self.max }
	pub fn top(&self) -> NonNull<RawValue> { unsafe { *self.top.as_ptr() } }
	pub fn len(&self) -> usize { unsafe { self.max.as_ptr().offset_from(self.min.as_ptr()) as usize } }
	pub fn used(&self) -> usize { unsafe { self.top().as_ptr().offset_from(self.min.as_ptr()) as usize } }
	pub fn left(&self) -> usize { unsafe { self.max.as_ptr().offset_from(self.top().as_ptr()) as usize } }

	pub fn index_of(&self, value: NonNull<RawValue>) -> Option<usize> {
		(value.as_ptr() >= self.min.as_ptr() && value.as_ptr() <= self.max.as_ptr())
			.then(move || self.index_of_unchecked(value))
	}

	pub fn index_of_unchecked(&self, value: NonNull<RawValue>) -> usize {
		unsafe { value.as_ptr().offset_from(self.min.as_ptr()) as usize }
	}

	pub fn get(&self, index: usize) -> Option<NonNull<RawValue>> {
		(index < self.len()).then(|| unsafe { self.get_unchecked(index) })
	}

	pub unsafe fn get_unchecked(&self, index: usize) -> NonNull<RawValue> {
		NonNull::new_unchecked(self.min.as_ptr().add(index))
	}

	pub unsafe fn set_top(&self, top: NonNull<RawValue>) {
		if self.index_of(top).is_some() {
			self.set_top_unchecked(top)
		}
	}

	pub unsafe fn set_top_unchecked(&self, top: NonNull<RawValue>) {
		*self.top.as_ptr() = top;
	}

	pub unsafe fn alloc<const N: usize>(&self) -> Option<NonNull<[RawValue; N]>> { (self.left() >= N).then(move || self.alloc_unchecked()) }
	pub unsafe fn free<const N: usize>(&self) -> Option<[RawValue; N]> { (self.used() >= N).then(move || self.free_unchecked()) }

	pub unsafe fn alloc_unchecked<const N: usize>(&self) -> NonNull<[RawValue; N]> {
		let region = self.top();
		self.set_top_unchecked(NonNull::new_unchecked(region.as_ptr().add(N)));
		region.cast()
	}

	pub unsafe fn free_unchecked<const N: usize>(&self) -> [RawValue; N] {
		let region = NonNull::new_unchecked(self.top().as_ptr().sub(N));
		self.set_top_unchecked(region);
		replace(region.cast().as_ptr(), [zeroed(); N])
	}

	pub unsafe fn used_slice(&self) -> NonNull<[RawValue]> { NonNull::slice_from_raw_parts(self.min, self.used()) }
	pub unsafe fn free_slice(&self) -> NonNull<[RawValue]> { NonNull::slice_from_raw_parts(self.top(), self.left()) }

	pub unsafe fn push_many_with<const N: usize>(&self, with: impl FnOnce() -> [RawValue; N]) -> Option<NonNull<[RawValue; N]>> {
		let space = self.alloc()?;
		self.thread.as_ref().threadbarrier();
		*space.as_ptr() = with();
		Some(space)
	}

	pub unsafe fn push_with(&self, with: impl FnOnce() -> RawValue) -> Option<NonNull<RawValue>> {
		self.push_many_with(move || [with()]).map(NonNull::cast)
	}

	pub unsafe fn push_many<const N: usize>(&self, values: [RawValue; N]) -> Option<NonNull<[RawValue; N]>> {
		self.push_many_with(move || values)
	}

	pub unsafe fn push(&self, value: RawValue) -> Option<NonNull<RawValue>> {
		self.push_many([value]).map(NonNull::cast)
	}

	pub unsafe fn pop_many<const N: usize>(&self) -> Option<[RawValue; N]> {
		self.free()
	}

	pub unsafe fn pop(&self) -> Option<RawValue> {
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

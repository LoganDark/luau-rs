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

use std::ffi::CStr;

use luau::vm::{Luau, StackValue};

fn main() {
	let vm = Luau::new().expect("failed to create Luau VM");
	let compiled = Luau::compile("print('Hello, World!')")
		.expect("failed to compile function");

	let chunkname = CStr::from_bytes_with_nul(b"=basic_run.luau\0").expect("die");
	let main_thread = vm.main_thread();
	let function = main_thread.load_compiled(compiled, chunkname)
		.expect("failed to load function");

	println!("{:?}", function.call_sync([]));

	// assert stack usage is balanced
	assert_eq!(unsafe { StackValue::stack(main_thread.as_ptr()) }, Vec::new());
}

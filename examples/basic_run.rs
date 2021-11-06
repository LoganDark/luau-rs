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

use luau::vm::Luau;

async fn main() {
	let vm = Luau::new().expect("failed to create Luau VM");
	let compiled = luau::compiler::compile("print('Hello, World!')", &Default::default(), &Default::default())
		.expect("failed to compile function");

	let mut main_thread = vm.main_thread();
	let function = main_thread.load(compiled, "=compiled")
		.expect("failed to load function");

	println!("all seems good for now");
}

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

use std::os::raw::c_int;

use luau::fvalue::FValue;
use luau::vm::Luau;

fn main() {
	// the Luau compiler initializes a lot of the more interesting FFlags
	{ let _ = Luau::compile(""); }

	for mut fflag in FValue::<bool>::list() {
		fflag.set(true);
		println!("{}: {}", fflag.name(), fflag.value())
	}

	for fint in FValue::<c_int>::list() {
		println!("{}: {}", fint.name(), fint.value())
	}
}

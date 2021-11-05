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

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

pub mod luau {
	#[cfg(feature = "vm")]
	include!(concat!(env!("OUT_DIR"), "/vm.rs"));
}

#[cfg(any(feature = "ast", feature = "compiler", feature = "analysis", feature = "vm"))]
pub mod glue {
	include!(concat!(env!("OUT_DIR"), "/glue.rs"));
}

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

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case, improper_ctypes)]

pub mod luau {
	include!(concat!(env!("OUT_DIR"), "/luau.rs"));
}

#[cfg(any(feature = "glue"))]
pub mod glue {
	#[allow(unused_imports)]
	use super::luau::*;

	include!(concat!(env!("OUT_DIR"), "/glue.rs"));
}

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

use luau_sys::luau::{luaL_openlibs, luaL_sandbox};

use crate::vm::data::{Data, GlobalData};
use crate::vm::Luau;

pub struct LuauBuildData;

impl LuauBuildData {
	pub fn data<D: GlobalData>(self, global_data: Data<D>, thread_data: Data<D::ThreadData>) -> Option<LuauBuildLibs<D>> {
		Some(LuauBuildLibs(Luau::new(global_data, thread_data)?))
	}

	pub fn no_data(self) -> Option<LuauBuildLibs<()>> {
		self.data(Box::pin(()), Box::pin(()))
	}
}

pub struct LuauBuildLibs<D: GlobalData>(Luau<D>);

impl<D: GlobalData> LuauBuildLibs<D> {
	pub fn all_libs(self) -> LuauBuildEnv<D> {
		unsafe { luaL_openlibs(self.0.global.as_ref().mainthread); }
		self.no_libs()
	}

	pub fn no_libs(self) -> LuauBuildEnv<D> { LuauBuildEnv(self.0) }
}

pub struct LuauBuildEnv<D: GlobalData>(Luau<D>);

impl<D: GlobalData> LuauBuildEnv<D> {
	pub fn setup(self, closure: impl FnOnce(&Luau<D>)) -> Luau<D> {
		closure(&self.0);
		self.no_setup()
	}

	pub fn no_setup(self) -> Luau<D> {
		unsafe { luaL_sandbox(self.0.global.as_ref().mainthread); }
		self.0
	}
}

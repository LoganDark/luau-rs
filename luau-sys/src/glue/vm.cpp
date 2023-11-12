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

#include "vm.h"

#include <ldo.h> // luaD_rawrunprotected
#include <ltable.h> // luaH_new

GLUE_API enum lua_Status gluauH_new(struct lua_State* L, int narray, int lnhash, struct Table* &result) {
	struct gluau_Inner {
		int narray;
		int lnhash;
		struct Table** result;
	};

	struct gluau_Inner inner = { .narray = narray, .lnhash = lnhash, .result = &result };

	return (enum lua_Status) luaD_rawrunprotected(L, [](struct lua_State* L, void* userdata) {
		struct gluau_Inner &inner = *(gluau_Inner*) userdata;
		*inner.result = luaH_new(L, inner.narray, inner.lnhash);
	}, &inner);
}

GLUE_API enum lua_Status gluau_ref(struct lua_State* L, int idx, int &result) {
	struct gluau_Inner {
		int idx;
		int* result;
	};

	struct gluau_Inner inner = { .idx = idx, .result = &result };

	return (enum lua_Status) luaD_rawrunprotected(L, [](struct lua_State* L, void* userdata) {
		struct gluau_Inner &inner = *(gluau_Inner*) userdata;
		*inner.result = lua_ref(L, inner.idx);
	}, &inner);
}

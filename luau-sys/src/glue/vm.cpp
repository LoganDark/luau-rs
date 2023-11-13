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
#include <lstring.h> // luaS_newlstr
#include <ltable.h> // luaH_new
#include <ludata.h> // luaU_newudata
#include <lua.h> // lua_newthread
#include <lbuffer.h> // luaB_newbuffer
#include <lualib.h> // luaL_sandbox, luaL_sandboxthread

template<typename Callback>
	int protect_indirect(struct lua_State* L, Callback callback) {
		return luaD_rawrunprotected(L, [](struct lua_State* L, void* userdata) { (*(Callback*) userdata)(); }, &callback);
	}

template<typename Ret, typename... Args>
	enum lua_Status protect(struct lua_State* L, Ret &result, Ret (*func)(Args...), Args... args) {
		return (enum lua_Status) protect_indirect(L, [&result, func, args...]() { result = (*func)(args...); });
	}

template<typename... Args>
	enum lua_Status protect_noreturn(struct lua_State* L, void (*func)(Args...), Args... args) {
		return (enum lua_Status) protect_indirect(L, [func, args...]() { (*func)(args...); });
	}

GLUE_API enum lua_Status gluau_ref(struct lua_State* L, int idx, int &result) {
	return protect(L, result, lua_ref, L, idx);
}

GLUE_API enum lua_Status gluauS_newlstr(struct lua_State* L, const char* str, size_t len, struct TString* &result) {
	return protect(L, result, luaS_newlstr, L, str, len);
}

GLUE_API enum lua_Status gluauH_new(struct lua_State* L, int narray, int lnhash, struct Table* &result) {
	return protect(L, result, luaH_new, L, narray, lnhash);
}

GLUE_API enum lua_Status gluauU_newudata(struct lua_State* L, size_t size, int tag, struct Udata* &result) {
	return protect(L, result, luaU_newudata, L, size, tag);
}

GLUE_API enum lua_Status gluau_newthread(struct lua_State* L, struct lua_State* &result) {
	return protect(L, result, lua_newthread, L);
}

GLUE_API enum lua_Status gluauB_newbuffer(struct lua_State* L, size_t len, struct Buffer* &result) {
	return protect(L, result, luaB_newbuffer, L, len);
}

GLUE_API enum lua_Status gluauL_sandbox(struct lua_State* L) {
	return protect_noreturn(L, luaL_sandbox, L);
}

GLUE_API enum lua_Status gluauL_sandboxthread(struct lua_State* L) {
	return protect_noreturn(L, luaL_sandboxthread, L);
}

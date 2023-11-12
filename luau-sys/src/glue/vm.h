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

#pragma once

#include "common.h"

#include <lua.h> // lua_Status
#include <lobject.h> // TString, Table, Udata, Buffer

GLUE_API enum lua_Status gluau_ref(struct lua_State* L, int idx, int &result);
GLUE_API enum lua_Status gluauS_newlstr(struct lua_State* L, const char* str, size_t len, struct TString* &result);
GLUE_API enum lua_Status gluauH_new(struct lua_State* L, int narray, int lnhash, struct Table* &result);
GLUE_API enum lua_Status gluauU_newudata(struct lua_State* L, size_t size, int tag, struct Udata* &result);
GLUE_API enum lua_Status gluau_newthread(struct lua_State* L, struct lua_State* &result);
GLUE_API enum lua_Status gluauB_newbuffer(struct lua_State* L, size_t len, struct Buffer* &result);

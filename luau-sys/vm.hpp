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

#include "luau/VM/include/lua.h"
#include "luau/VM/include/luaconf.h"
#include "luau/VM/include/lualib.h"

#undef LUAI_FUNC
#define LUAI_FUNC extern

#include "luau/VM/src/lapi.h"
#include "luau/VM/src/lbuiltins.h"
#include "luau/VM/src/lbytecode.h"
#include "luau/VM/src/lcommon.h"
#include "luau/VM/src/ldebug.h"
#include "luau/VM/src/ldo.h"
#include "luau/VM/src/lfunc.h"
#include "luau/VM/src/lgc.h"
#include "luau/VM/src/lmem.h"
#include "luau/VM/src/lnumutils.h"
#include "luau/VM/src/lobject.h"
#include "luau/VM/src/lstate.h"
#include "luau/VM/src/lstring.h"
#include "luau/VM/src/ltable.h"
#include "luau/VM/src/ltm.h"
#include "luau/VM/src/lvm.h"

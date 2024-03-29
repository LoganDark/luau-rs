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

#include <stddef.h> // NOLINT(modernize-deprecated-headers)
#include <stdbool.h> // NOLINT(modernize-deprecated-headers)
#include <stdint.h> // NOLINT(modernize-deprecated-headers)

#ifdef __cplusplus
#define GLUE_API extern "C"
#else
#define GLUE_API extern
#endif

struct gluau_Buffer {
	char* data;
	size_t len;
};

typedef void* gluau_FValue;
typedef gluau_FValue gluau_FFlag;
typedef gluau_FValue gluau_FInt;

enum gluau_Optionality : uint8_t {
	Some, None
};

struct gluau_OptionalFValue {
	enum gluau_Optionality presence;
	gluau_FValue value;
};

GLUE_API struct gluau_OptionalFValue gluau_find_fflag(struct gluau_Buffer name);
GLUE_API struct gluau_OptionalFValue gluau_find_fint(struct gluau_Buffer name);

GLUE_API gluau_FFlag* gluau_get_fflags();
GLUE_API gluau_FInt* gluau_get_fints();

GLUE_API struct gluau_Buffer gluau_get_fflag_name(gluau_FFlag fflag);
GLUE_API struct gluau_Buffer gluau_get_fint_name(gluau_FInt fflag);

GLUE_API bool gluau_fflag_get(gluau_FFlag fflag);
GLUE_API int gluau_fint_get(gluau_FInt fint);

GLUE_API void gluau_fflag_set(gluau_FFlag fflag, bool value);
GLUE_API void gluau_fint_set(gluau_FInt fint, int value);

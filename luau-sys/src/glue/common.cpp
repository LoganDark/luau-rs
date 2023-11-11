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

#include "common.hpp"

#include "stdlib.h" // NOLINT(modernize-deprecated-headers)
#include "string.h" // NOLINT(modernize-deprecated-headers)

#include <Luau/Common.h>

gluau_Buffer gluauU_strtobuf(const std::string &input) {
	size_t len = input.length();
	char* block = static_cast<char*>(malloc(len));

	if (block) {
		// the memory block is associated with its length, so no null terminator
		memcpy(block, input.data(), len); // NOLINT(bugprone-not-null-terminated-result)
	}

	return {
		.data = block,
		.len = len
	};
}

#define FOR_EACH_FFLAG(T, v) for (Luau::FValue<T>* v = Luau::FValue<T>::list; v; v = v->next) // NOLINT(bugprone-macro-parentheses)

template<typename T>
	Luau::FValue<T>** gluau_get_fvalues() {
		size_t num = 0;
		// @formatter:off
		FOR_EACH_FFLAG(T, flag) num++;
		// @formatter:on
		auto block = static_cast<Luau::FValue<T>**>(calloc(sizeof(void*), num + 1));

		if (block) {
			size_t index = 0;
			FOR_EACH_FFLAG(T, flag) {
				block[index++] = flag;
			}
		}

		return block;
	}

template<typename T>
	Luau::FValue<T>* gluau_find_fvalue(struct gluau_Buffer name) {
		FOR_EACH_FFLAG(T, flag) {
			if (strlen(flag->name) == name.len && memcmp(flag->name, name.data, name.len) == 0) {
				return flag;
			}
		}

		return nullptr;
	}

template<typename T>
	gluau_Buffer gluau_fvalue_name(void* fvalue) {
		auto flag = static_cast<Luau::FValue<T>*>(fvalue);

		return {
			.data = const_cast<char*>(flag->name),
			.len = strlen(flag->name)
		};
	}

template<typename T>
	T gluau_fvalue_get(void* fvalue) {
		return static_cast<Luau::FValue<T>*>(fvalue)->value;
	}

template<typename T>
	void gluau_fvalue_set(void* fvalue, T value) {
		static_cast<Luau::FValue<T>*>(fvalue)->value = value;
	}

GLUE_API struct gluau_OptionalFValue gluau_find_fflag(struct gluau_Buffer name) {
	auto found = gluau_find_fvalue<bool>(name);

	return {
		.presence = found ? gluau_Optionality::Some : gluau_Optionality::None,
		.value = found
	};
}

GLUE_API struct gluau_OptionalFValue gluau_find_fint(struct gluau_Buffer name) {
	auto found = gluau_find_fvalue<int>(name);

	return {
		.presence = found ? gluau_Optionality::Some : gluau_Optionality::None,
		.value = found
	};
}

GLUE_API FFlag* gluau_get_fflags() {
	return reinterpret_cast<FFlag*>(gluau_get_fvalues<bool>());
}

GLUE_API FInt* gluau_get_fints() {
	return reinterpret_cast<FInt*>(gluau_get_fvalues<int>());
}

GLUE_API struct gluau_Buffer gluau_get_fflag_name(FFlag fflag) {
	return gluau_fvalue_name<bool>(fflag);
}

GLUE_API struct gluau_Buffer gluau_get_fint_name(FInt fflag) {
	return gluau_fvalue_name<int>(fflag);
}

GLUE_API bool gluau_fflag_get(FFlag fflag) {
	return gluau_fvalue_get<bool>(fflag);
}

GLUE_API int gluau_fint_get(FInt fint) {
	return gluau_fvalue_get<int>(fint);
}

GLUE_API void gluau_fflag_set(FFlag fflag, bool value) {
	return gluau_fvalue_set(fflag, value);
}

GLUE_API void gluau_fint_set(FInt fint, int value) {
	return gluau_fvalue_set(fint, value);
}

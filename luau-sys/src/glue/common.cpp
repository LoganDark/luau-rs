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

#include "common.hpp"

#include "stdlib.h" // NOLINT(modernize-deprecated-headers)
#include "string.h" // NOLINT(modernize-deprecated-headers)

gluau_Buffer gluauU_strtobuf(const std::string &input) {
	size_t len = input.length();
	char* block = static_cast<char*>(malloc(len)); // ensure max alignment

	if (block) {
		// the memory block is associated with its length, so no null terminator
		memcpy(block, input.data(), len); // NOLINT(bugprone-not-null-terminated-result)
	}

	return {
		.data = block,
		.len = len
	};
}

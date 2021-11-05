#include "common.hpp"

#include "malloc.h"
#include "string.h" // NOLINT(modernize-deprecated-headers)

gluau_Buffer gluauU_strtobuf(const std::string &input) {
	size_t len = input.length();
	char* block = static_cast<char*>(memalign(8, len)); // ensure max alignment

	if (block) {
		// the memory block is associated with its length, so no null terminator
		memcpy(block, input.data(), len); // NOLINT(bugprone-not-null-terminated-result)
	}

	return {
		.data = block,
		.len = len
	};
}

#pragma once

#include <stddef.h> // NOLINT(modernize-deprecated-headers)

#ifdef __cplusplus
#define GLUE_API extern "C"
#else
#define GLUE_API extern
#endif

struct gluau_Buffer {
	char* data;
	size_t len;
};

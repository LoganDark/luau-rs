#pragma once

#include "common.h"

#include "stdbool.h" // NOLINT(modernize-deprecated-headers)

enum gluau_CompileResultType {
	SUCCESS = 0,
	PARSE_FAILURE = 1,
	COMPILE_FAILURE = 2
};

struct gluau_CompileSuccess {
	struct gluau_Buffer bytecode;
};

struct gluau_Span {
	unsigned int start_line;
	unsigned int start_column;
	unsigned int end_line;
	unsigned int end_column;
};

struct gluau_Error {
	struct gluau_Buffer message;
	struct gluau_Span span;
};

struct gluau_Errors {
	struct gluau_Error* errors;
	size_t len;
};

union gluau_CompileUnion {
	struct gluau_CompileSuccess success;
	struct gluau_Errors parse_failure;
	struct gluau_Error compile_failure;
};

struct gluau_CompileResult {
	enum gluau_CompileResultType type;
	union gluau_CompileUnion data;
};

struct gluau_CompileOpts {
	int bytecodeVersion;
	int optimizationLevel;
	int debugLevel;
	int coverageLevel;
	const char* vectorLib;
	const char* vectorCtor;
};

struct gluau_ParseOpts {
	bool allowTypeAnnotations;
	bool supportContinueStatement;
	bool allowDeclarationSyntax;
	bool captureComments;
};

GLUE_API struct gluau_CompileResult gluau_compile(struct gluau_Buffer source, struct gluau_CompileOpts compile_opts, struct gluau_ParseOpts parse_opts);
GLUE_API struct gluau_Buffer gluau_compile_sneakily(struct gluau_Buffer source, struct gluau_CompileOpts compile_opts, struct gluau_ParseOpts parse_opts);

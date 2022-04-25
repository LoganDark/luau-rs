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
	int optimizationLevel;
	int debugLevel;
	int coverageLevel;
	const char* vectorLib;
	const char* vectorCtor;
	const char** mutableGlobals;
};

struct gluau_ParseOpts {
	bool allowTypeAnnotations;
	bool supportContinueStatement;
	bool allowDeclarationSyntax;
	bool captureComments;
};

GLUE_API struct gluau_CompileResult gluau_compile(struct gluau_Buffer source, struct gluau_CompileOpts compile_opts, struct gluau_ParseOpts parse_opts);
GLUE_API struct gluau_Buffer gluau_compile_sneakily(struct gluau_Buffer source, struct gluau_CompileOpts compile_opts, struct gluau_ParseOpts parse_opts);

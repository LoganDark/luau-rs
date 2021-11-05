#include "compiler.h"
#include "common.hpp"

#include <Luau/BytecodeBuilder.h>
#include <Luau/Compiler.h>
#include <Luau/Parser.h>

Luau::CompileOptions gluaC_compopt2luau(const gluau_CompileOpts &opts) {
	return {
		.bytecodeVersion = opts.bytecodeVersion,
		.optimizationLevel = opts.optimizationLevel,
		.debugLevel = opts.debugLevel,
		.coverageLevel = opts.coverageLevel,
		.vectorLib = opts.vectorLib,
		.vectorCtor = opts.vectorCtor
	};
}

Luau::ParseOptions gluaC_parseopt2luau(const gluau_ParseOpts &opts) {
	return {
		.allowTypeAnnotations = opts.allowTypeAnnotations,
		.supportContinueStatement = opts.supportContinueStatement,
		.allowDeclarationSyntax = opts.allowDeclarationSyntax,
		.captureComments = opts.captureComments
	};
}

gluau_Span gluauC_loctospan(const Luau::Location &location) {
	return {
		.start_line = location.begin.line,
		.start_column = location.begin.column,
		.end_line = location.end.line,
		.end_column = location.end.column
	};
}

GLUE_API gluau_CompileResult gluau_compile(
	gluau_Buffer source,
	gluau_CompileOpts compile_opts,
	gluau_ParseOpts parse_opts
) {
	Luau::BytecodeBuilder bcb;
	std::string source_string(source.data, source.len);

	try {
		Luau::compileOrThrow(
			bcb, source_string, gluaC_compopt2luau(compile_opts),
			gluaC_parseopt2luau(parse_opts)
		);

		return {
			.type = gluau_CompileResultType::SUCCESS,
			.data {
				.success = {
					.bytecode = {gluauU_strtobuf(bcb.getBytecode())}
				}
			}
		};
	} catch (Luau::ParseErrors &container) {
		auto len = container.getErrors().size();
		auto* errors = new gluau_Error[len];

		unsigned int i = 0;
		for (const auto &error: container.getErrors()) {
			errors[i++] = {
				.message = gluauU_strtobuf(error.getMessage()),
				.span = gluauC_loctospan(error.getLocation())
			};
		}

		return {
			.type = gluau_CompileResultType::PARSE_FAILURE,
			.data = {
				.parse_failure = {
					.errors = errors,
					.len = len
				}
			}
		};
	} catch (Luau::CompileError &error) {
		return {
			.type = gluau_CompileResultType::COMPILE_FAILURE,
			.data = {
				.compile_failure = {
					.message = gluauU_strtobuf(error.what()),
					.span = gluauC_loctospan(error.getLocation())
				}
			}
		};
	}
}

GLUE_API gluau_Buffer gluau_compile_sneakily(
	gluau_Buffer source,
	gluau_CompileOpts compile_opts,
	gluau_ParseOpts parse_opts
) {
	return gluauU_strtobuf(
		Luau::compile(
			std::string(source.data, source.len),
			gluaC_compopt2luau(compile_opts),
			gluaC_parseopt2luau(parse_opts)
		)
	);
}

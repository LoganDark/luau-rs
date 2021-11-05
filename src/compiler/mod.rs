use crate::ast::{ParseOptions, Span};
use crate::luau_sys::compiler;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum OptimizationLevel {
	/// No optimization is performed.
	None,
	/// Only optimizations that do not impact debuggability are performed.
	Basic,
	/// All optimizations are performed, including exotic ones such as inlining.
	Full
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DebugLevel {
	/// No debug symbols are included. Debugging on the source-code level is not possible.
	None,
	/// Line info and function names are included for useful tracebacks.
	Traceback,
	/// Local and upvalue names are included. Useful for debugging.
	Full
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CoverageLevel {
	/// No coverage is tracked.
	None,
	/// Tracks coverage of statements.
	Statement,
	/// Tracks coverage of each expression.
	Expression
}

/// Instructs the compiler on how it should generate bytecode.
#[derive(Copy, Clone, Debug)]
pub struct CompileOptions(compiler::gluau_CompileOpts);

impl Default for CompileOptions {
	fn default() -> Self {
		Self(compiler::gluau_CompileOpts {
			bytecodeVersion: 1,
			optimizationLevel: 1,
			debugLevel: 1,
			coverageLevel: 0,
			vectorLib: std::ptr::null(),
			vectorCtor: std::ptr::null()
		})
	}
}

impl CompileOptions {
	pub fn set_bytecode_version(&mut self, version: u32) -> &mut Self {
		self.0.bytecodeVersion = version as _;
		self
	}

	pub fn set_opt_level(&mut self, level: OptimizationLevel) -> &mut Self {
		self.0.optimizationLevel = match level {
			OptimizationLevel::None => 0,
			OptimizationLevel::Basic => 1,
			OptimizationLevel::Full => 2
		};

		self
	}

	pub fn set_debug_level(&mut self, level: DebugLevel) -> &mut Self {
		self.0.debugLevel = match level {
			DebugLevel::None => 0,
			DebugLevel::Traceback => 1,
			DebugLevel::Full => 2
		};

		self
	}

	pub fn set_coverage_level(&mut self, level: CoverageLevel) -> &mut Self {
		self.0.coverageLevel = match level {
			CoverageLevel::None => 0,
			CoverageLevel::Statement => 1,
			CoverageLevel::Expression => 2
		};

		self
	}

	pub fn new(opt_level: OptimizationLevel, debug_level: DebugLevel, coverage_level: CoverageLevel) -> Self {
		let mut new = Self::default();
		new.set_opt_level(opt_level);
		new.set_debug_level(debug_level);
		new.set_coverage_level(coverage_level);
		new
	}

	pub fn bytecode_version(&self) -> u32 {
		self.0.bytecodeVersion as u32
	}

	pub fn opt_level(&self) -> OptimizationLevel {
		match self.0.optimizationLevel {
			0 => OptimizationLevel::None,
			1 => OptimizationLevel::Basic,
			_ => OptimizationLevel::Full
		}
	}

	pub fn debug_level(&self) -> DebugLevel {
		match self.0.debugLevel {
			0 => DebugLevel::None,
			1 => DebugLevel::Traceback,
			_ => DebugLevel::Full
		}
	}

	pub fn coverage_level(&self) -> CoverageLevel {
		match self.0.coverageLevel {
			0 => CoverageLevel::None,
			1 => CoverageLevel::Statement,
			_ => CoverageLevel::Expression
		}
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Error {
	pub message: String,
	pub span: Span
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum CompileError {
	Parse(Vec<Error>),
	Compile(Error)
}

/// Represents a compiled Luau chunk, and contains the bytecode for that chunk.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Chunk(Vec<u8>);

fn error_from_gluau(gluau: compiler::gluau_Error) -> Error {
	unsafe {
		Error {
			// SAFETY: this is fine
			message: String::from_raw_parts(gluau.message.data as _, gluau.message.len as _, gluau.message.len as _),
			span: Span::new(
				gluau.span.start_line as _,
				gluau.span.start_column as _,
				gluau.span.end_line as _,
				gluau.span.end_column as _
			)
		}
	}
}

pub fn compile(source: &str, compile_opts: &CompileOptions, parse_opts: &ParseOptions) -> Result<Chunk, CompileError> {
	let source = luau_sys::compiler::gluau_Buffer {
		data: source.as_ptr() as _,
		len: source.len() as _
	};

	// SAFETY: C++ exceptions are caught by the C++ glue, and never unwind into Rust
	let result = unsafe {
		luau_sys::compiler::gluau_compile(source, compile_opts.0, parse_opts.0)
	};

	match result.type_ {
		luau_sys::compiler::gluau_CompileResultType_SUCCESS => Ok(Chunk(unsafe {
			let bytecode = result.data.success.bytecode;
			// SAFETY: this is fine
			Vec::from_raw_parts(bytecode.data as _, bytecode.len as _, bytecode.len as _)
		})),

		luau_sys::compiler::gluau_CompileResultType_PARSE_FAILURE => Err(unsafe {
			let failure = result.data.parse_failure;
			// SAFETY: this is also fine
			CompileError::Parse(
				Vec::from_raw_parts(failure.errors, failure.len as _, failure.len as _)
					.into_iter().map(error_from_gluau).collect()
			)
		}),

		luau_sys::compiler::gluau_CompileResultType_COMPILE_FAILURE => Err(unsafe {
			CompileError::Compile(error_from_gluau(result.data.compile_failure))
		}),

		_ => panic!("Received unexpected type from glue code: {}", result.type_)
	}
}

pub fn compile_sneakily(source: &str, compile_opts: &CompileOptions, parse_opts: &ParseOptions) -> Chunk {
	let source = luau_sys::compiler::gluau_Buffer {
		data: source.as_ptr() as _,
		len: source.len() as _
	};

	Chunk(unsafe {
		// SAFETY: this method cannot throw
		let buffer = luau_sys::compiler::gluau_compile_sneakily(source, compile_opts.0, parse_opts.0);
		Vec::from_raw_parts(buffer.data as _, buffer.len as _, buffer.len as _)
	})
}

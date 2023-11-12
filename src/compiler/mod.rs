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

use std::fmt::{Display, Formatter};

use luau_sys::glue::{gluau_Buffer, gluau_compile, gluau_compile_sneakily, gluau_CompileOpts, gluau_CompileResultType, gluau_Error};

use crate::ast::{ParseOptions, Span};

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

#[derive(Copy, Clone, Debug)]
pub struct CompileOptions(gluau_CompileOpts);

impl Default for CompileOptions {
	fn default() -> Self {
		Self(gluau_CompileOpts {
			optimizationLevel: 1,
			debugLevel: 1,
			coverageLevel: 0,
			vectorLib: std::ptr::null(),
			vectorCtor: std::ptr::null(),
			mutableGlobals: std::ptr::null_mut()
		})
	}
}

impl CompileOptions {
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

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.span, f)?;
		f.write_str(": ")?;
		Display::fmt(&self.message, f)
	}
}

impl std::error::Error for Error {}

#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum CompileError {
	#[error("parse error")]
	Parse(Vec<Error>),

	#[error("{0}")]
	Compile(Error)
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CompiledFunction(Vec<u8>);

impl AsRef<[u8]> for CompiledFunction {
	fn as_ref(&self) -> &[u8] {
		&self.0
	}
}

fn error_from_gluau(gluau: gluau_Error) -> Error {
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

pub(crate) fn compile(source: &str, compile_opts: &CompileOptions, parse_opts: &ParseOptions) -> Result<CompiledFunction, CompileError> {
	let source = gluau_Buffer {
		data: source.as_ptr() as _,
		len: source.len() as _
	};

	// SAFETY: C++ exceptions are caught by the C++ glue, and never unwind into Rust
	let result = unsafe { gluau_compile(source, compile_opts.0, parse_opts.0) };

	match result.type_ {
		gluau_CompileResultType::SUCCESS => Ok(CompiledFunction(unsafe {
			let bytecode = result.data.success.bytecode;
			// SAFETY: this is fine
			Vec::from_raw_parts(bytecode.data as _, bytecode.len as _, bytecode.len as _)
		})),

		gluau_CompileResultType::PARSE_FAILURE => Err(unsafe {
			let failure = result.data.parse_failure;
			// SAFETY: this is also fine
			CompileError::Parse(
				Vec::from_raw_parts(failure.errors, failure.len as _, failure.len as _)
					.into_iter().map(error_from_gluau).collect()
			)
		}),

		gluau_CompileResultType::COMPILE_FAILURE => Err(unsafe {
			CompileError::Compile(error_from_gluau(result.data.compile_failure))
		})
	}
}

pub(crate) fn compile_sneakily(source: &str, compile_opts: &CompileOptions, parse_opts: &ParseOptions) -> CompiledFunction {
	let source = gluau_Buffer {
		data: source.as_ptr() as _,
		len: source.len() as _
	};

	CompiledFunction(unsafe {
		// SAFETY: this method cannot throw
		let buffer = gluau_compile_sneakily(source, compile_opts.0, parse_opts.0);
		Vec::from_raw_parts(buffer.data as _, buffer.len as _, buffer.len as _)
	})
}

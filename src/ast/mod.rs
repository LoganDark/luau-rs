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

use std::fmt::{Debug, Display, Formatter, Write};

use luau_sys::glue::gluau_ParseOpts;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Mode {
	// No type checking or analysis is performed.
	NoCheck,
	// Lenient type checking and analysis is performed.
	Nonstrict,
	// Strict type checking and analysis is performed.
	Strict,
	// Type definition module, has special parsing rules
	Definition
}

#[derive(Copy, Clone, Debug)]
pub struct ParseOptions(pub(crate) gluau_ParseOpts);

impl Default for ParseOptions {
	fn default() -> Self {
		Self(gluau_ParseOpts {
			allowTypeAnnotations: true,
			supportContinueStatement: true,
			allowDeclarationSyntax: false,
			captureComments: false
		})
	}
}

impl ParseOptions {
	/// Sets whether type annotations are allowed.
	pub fn set_allow_type_annotations(&mut self, value: bool) -> &mut Self {
		self.0.allowTypeAnnotations = value;
		self
	}

	/// Sets whether to support the `continue` statement in loops.
	pub fn set_support_continue_statement(&mut self, value: bool) -> &mut Self {
		self.0.supportContinueStatement = value;
		self
	}

	/// Sets whether to enable a new `declare` keyword to declare the existence
	/// of a global function or variable. This hasn't yet been announced.
	pub fn set_allow_declaration_syntax(&mut self, value: bool) -> &mut Self {
		self.0.allowDeclarationSyntax = value;
		self
	}

	/// Sets whether to capture comments during parsing and retain them in the
	/// parse result.
	pub fn set_capture_comments(&mut self, value: bool) -> &mut Self {
		self.0.captureComments = value;
		self
	}

	pub fn allow_type_annotations(&self) -> bool {
		self.0.allowTypeAnnotations
	}

	pub fn support_continue_statement(&self) -> bool {
		self.0.supportContinueStatement
	}

	pub fn allow_declaration_syntax(&self) -> bool {
		self.0.allowDeclarationSyntax
	}

	pub fn capture_comments(&self) -> bool {
		self.0.captureComments
	}
}

/// Represents a position in the source code.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Position {
	pub line: u32,
	pub column: u32
}

impl Display for Position {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_char('(')?;
		Display::fmt(&self.line, f)?;
		f.write_char(',')?;
		Display::fmt(&self.column, f)?;
		f.write_char(')')
	}
}

/// Represents a span in the source code, from one position (the start) to the
/// next (the end).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Span(pub Position, pub Position);

impl Display for Span {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.0, f)?;
		f.write_str("..")?;
		Display::fmt(&self.1, f)
	}
}

impl Span {
	pub fn new(start_line: u32, start_column: u32, end_line: u32, end_column: u32) -> Self {
		Self(Position {
			line: start_line,
			column: start_column
		}, Position {
			line: end_line,
			column: end_column
		})
	}
}

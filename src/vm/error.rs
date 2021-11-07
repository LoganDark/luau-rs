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

#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum Error {
	/// There was a runtime error during execution. Only the error message is
	/// available. Other information (tracebacks) may be obtainable externally.
	#[error("{0}")]
	Runtime(String),

	/// There wasn't enough stack space available to perform the requested
	/// operation.
	#[error("out of stack space")]
	OutOfStack
}

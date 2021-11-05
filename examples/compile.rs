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

use luau::ast::ParseOptions;
use luau::compiler::CompileOptions;

fn main() {
	let source = r#"local

print(x)"#;
	let compile_opts = CompileOptions::default();
	let parse_opts = ParseOptions::default();
	let result = luau::compiler::compile(source, &compile_opts, &parse_opts);
	println!("source code:\n{}\n", source);
	println!("result: {:#?}", result)
}

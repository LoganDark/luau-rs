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

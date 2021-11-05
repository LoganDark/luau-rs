#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

#[cfg(feature = "ast")]
pub mod ast {
	include!(concat!(env!("OUT_DIR"), "/glue_ast.rs"));
}

#[cfg(feature = "compiler")]
pub mod compiler {
	include!(concat!(env!("OUT_DIR"), "/glue_compiler.rs"));
}

#[cfg(feature = "analysis")]
pub mod analysis {
	include!(concat!(env!("OUT_DIR"), "/glue_analysis.rs"));
}

#[cfg(feature = "vm")]
pub mod vm {
	include!(concat!(env!("OUT_DIR"), "/vm.rs"));
	include!(concat!(env!("OUT_DIR"), "/glue_vm.rs"));
}

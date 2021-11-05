#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

pub mod luau {
	#[cfg(feature = "vm")]
	include!(concat!(env!("OUT_DIR"), "/vm.rs"));
}

#[cfg(any(feature = "ast", feature = "compiler", feature = "analysis", feature = "vm"))]
pub mod glue {
	include!(concat!(env!("OUT_DIR"), "/glue.rs"));
}

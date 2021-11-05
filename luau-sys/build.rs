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

#[cfg(any(feature = "ast", feature = "compiler", feature = "analysis", feature = "vm"))]
fn main() {
	let out_dir = std::env::var("OUT_DIR").expect("couldn't find output directory");

	eprintln!("building Luau...");

	let destination = {
		let mut config = cmake::Config::new("luau");

		config.define("LUAU_BUILD_CLI", "OFF")
			.define("LUAU_BUILD_TESTS", "OFF")
			.no_build_target(true);

		// Windows is special, and needs to have an extra flag defined
		// This is due to cmake-rs wiping them for no reason
		// Luau needs unwinding in order for exceptions to work (obviously)
		#[cfg(target_os = "windows")]
			config.cxxflag("/EHsc");

		config.build()
	};

	eprintln!("successfully built Luau");

	#[cfg(not(target_os = "windows"))]
	println!("cargo:rustc-link-search=native={}/build", destination.display());

	// Windows is once again special and outputs libs in even more subdirs
	#[cfg(target_os = "windows")] {
		println!("cargo:rustc-link-search=native={}/build/Debug", destination.display());
		println!("cargo:rustc-link-search=native={}/build/Release", destination.display());
	}

	macro_rules! link {
		($feat:literal, $module:literal) => {
			#[cfg(feature = $feat)]
			println!(concat!("cargo:rustc-link-lib=static=Luau.", $module));
		}
	}

	// link to C++ stdlib, unless we're on windows, which is special
	#[cfg(not(target_os = "windows"))]
	println!("cargo:rustc-link-lib=stdc++");

	// link only to requested features
	link!("ast", "Ast");
	link!("compiler", "Compiler");
	link!("analysis", "Analysis");
	link!("vm", "VM");

	drop(destination);

	// generate bindings to what we can
	#[cfg(feature = "vm")]
	bindgen::builder()
		.clang_arg("-Iluau/VM/include")
		.clang_arg("-std=c++17")
		.header("vm.hpp")
		.generate()
		.expect("couldn't generate Luau VM bindings")
		.write_to_file(format!("{}/vm.rs", out_dir))
		.expect("couldn't write Luau VM bindings to file");

	let mut glue_cc = cc::Build::new();

	glue_cc
		.flag("-Iluau/Ast/include")
		.flag("-Iluau/Compiler/include")
		.flag("-Iluau/Analysis/include")
		.flag("-Iluau/VM/include")
		// MSVC requires /std:c++latest for designated initializers
		// I quite like them so I'm not going to stop using them
		// Windows being special for the fourth time
		.flag(if cfg!(target_os = "windows") { "/std:c++latest" } else { "-std=c++17" });

	let mut glue_bindgen = bindgen::builder()
		.clang_args([
			"-Iluau/Ast/include",
			"-Iluau/Compiler/include",
			"-Iluau/Analysis/include",
			"-Iluau/VM/include"
		])
		.allowlist_function("gluau_.*");

	macro_rules! glue {
		($feat:literal) => {
			glue_cc.file(concat!("src/glue/", $feat, ".cpp"));
			glue_bindgen = glue_bindgen.header(concat!("src/glue/", $feat, ".h"));
		}
	}

	macro_rules! maybe_glue {
		($feat:literal) => {
			#[cfg(feature = $feat)]
			glue!($feat);
		}
	}

	glue!("common");
	maybe_glue!("ast");
	maybe_glue!("compiler");
	maybe_glue!("analysis");
	maybe_glue!("vm");

	// cc links automatically
	glue_cc.compile("gluau");
	glue_bindgen
		.generate()
		.expect("couldn't generate Luau glue bindings")
		.write_to_file(format!("{}/glue.rs", out_dir))
		.expect("couldn't write Luau glue bindings to file");

	// all should be good
}

#[cfg(not(any(feature = "ast", feature = "compiler", feature = "analysis", feature = "vm")))]
fn main() {}

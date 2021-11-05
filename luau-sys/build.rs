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

fn main() {
	#[cfg(any(feature = "ast", feature = "compiler", feature = "analysis", feature = "vm"))]
		let out_dir = std::env::var("OUT_DIR").expect("couldn't find output directory");

	{ // build luau with cmake
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
		#[cfg(all(not(target_os = "windows"), any(feature = "ast", feature = "compiler", feature = "analysis", feature = "vm")))]
		println!("cargo:rustc-link-lib=stdc++");

		// link only to requested features
		link!("ast", "Ast");
		link!("compiler", "Compiler");
		link!("analysis", "Analysis");
		link!("vm", "VM");
	}

	#[cfg(feature = "vm")] { // generate bindings to what we can
		bindgen::builder()
			.clang_arg("-Iluau/VM/include")
			.clang_arg("-std=c++17")
			.header("vm.hpp")
			.generate()
			.expect("couldn't generate Luau VM bindings")
			.write_to_file(format!("{}/vm.rs", out_dir))
			.expect("couldn't write Luau VM bindings to file");
	}

	#[cfg(any(feature = "ast", feature = "compiler", feature = "analysis", feature = "vm"))] {
		eprintln!("compiling glue...");

		// cc links automatically
		let mut build = cc::Build::new();
		build.flag("-Iluau/Ast/include")
			.flag("-Iluau/Compiler/include")
			.flag("-Iluau/Analysis/include")
			.flag("-Iluau/VM/include")
			// MSVC requires /std:c++latest for designated initializers
			// I quite like them so I'm not going to stop using them
			// Windows being special for the fourth time
			.flag(if cfg!(target_os = "windows") { "/std:c++latest" } else { "-std=c++17" });

		macro_rules! build_glue {
			($feat:literal) => {
				build.file(concat!("src/glue/", $feat, ".cpp"));
			}
		}

		macro_rules! maybe_build_glue {
			($feat:literal) => {
				// cc links to it automatically
				#[cfg(feature = $feat)]
				build_glue!($feat);
			}
		}

		// compile in glue code
		#[cfg(any(feature = "ast", feature = "compiler", feature = "analysis", feature = "vm"))]
		build_glue!("common");
		maybe_build_glue!("ast");
		maybe_build_glue!("compiler");
		maybe_build_glue!("analysis");
		maybe_build_glue!("vm");

		build.compile("gluau");

		eprintln!("compiled and linked to glue");
	}

	#[cfg(any(feature = "ast", feature = "compiler", feature = "analysis", feature = "vm"))] {
		let mut glue_builder = bindgen::builder()
			.clang_args([
				"-Iluau/Ast/include",
				"-Iluau/Compiler/include",
				"-Iluau/Analysis/include",
				"-Iluau/VM/include"
			]);

		#[allow(deprecated)] { // don't care about your unnecessary renaming
			glue_builder = glue_builder.whitelist_function("gluau_.*");
		}

		macro_rules! bind_glue {
			($builder:ident, $feat:literal) => {{
				$builder = $builder.header(concat!("src/glue/", $feat, ".h"));
			}}
		}

		macro_rules! maybe_bind_glue {
			($builder:ident, $feat:literal) => {
				#[cfg(feature = $feat)]
				bind_glue!($builder, $feat);
			}
		}

		bind_glue!(glue_builder, "common");
		maybe_bind_glue!(glue_builder, "ast");
		maybe_bind_glue!(glue_builder, "compiler");
		maybe_bind_glue!(glue_builder, "analysis");
		maybe_bind_glue!(glue_builder, "vm");

		glue_builder
			.generate()
			.expect("couldn't generate Luau glue bindings")
			.write_to_file(format!("{}/glue.rs", out_dir))
			.expect("couldn't write Luau glue bindings to file");
	}

	// all should be good
}

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

#[allow(deprecated)]
use bindgen::{CargoCallbacks, EnumVariation};

#[cfg(any(feature = "link", feature = "glue"))]
fn main() {
	let out_dir = std::env::var("OUT_DIR").expect("couldn't find output directory");

	#[cfg(feature = "link")] {
		eprintln!("building Luau...");

		let destination = {
			let mut config = cmake::Config::new("luau");

			config.define("LUAU_BUILD_CLI", "OFF")
				.define("LUAU_BUILD_TESTS", "OFF")
				.define("LUAU_STATIC_CRT", "ON")
				.profile("Release")
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

		// link only to requested features
		link!("ast", "Ast");
		link!("compiler", "Compiler");
		link!("analysis", "Analysis");
		link!("vm", "VM");
	}

	// link to C++ stdlib, unless we're on windows, which is special
	#[cfg(not(target_os = "windows"))]
	println!("cargo:rustc-link-lib=stdc++");

	// generate bindings to what we can

	#[cfg(any(feature = "ast", feature = "vm"))] {
		let mut luau_bindgen = bindgen::builder()
			.parse_callbacks(Box::new(CargoCallbacks::new()))
			.clang_arg("-Iluau/Common/include")
			.clang_arg("-Iluau/Ast/include")
			.clang_arg("-Iluau/Compiler/include")
			.clang_arg("-Iluau/Analysis/include")
			.clang_arg("-Iluau/VM/include")
			.clang_arg("-Iluau/VM/src")
			.clang_arg("-std=c++17")
			.allowlist_var("LUA.*")
			.allowlist_function("lua.*")
			.allowlist_type("lua.*")
			.allowlist_var("Luau::.*")
			.blocklist_item("Luau::list")
			.allowlist_function("Luau::.*")
			.allowlist_type("Luau::.*")
			.allowlist_var("GCS.+|WHITE[01]BIT|BLACKBIT|FIXEDBIT|WHITEBITS")
			.opaque_type("std::.*")
			.opaque_type("Luau::DenseHash.*")
			.default_enum_style(EnumVariation::Rust { non_exhaustive: false })
			.fit_macro_constants(true);

		#[cfg(feature = "ast")] {
			luau_bindgen = luau_bindgen.header("ast.hpp");
		}

		#[cfg(feature = "compiler")] {
			luau_bindgen = luau_bindgen.header("compiler.hpp");
		}

		#[cfg(feature = "vm")] {
			luau_bindgen = luau_bindgen.header("vm.hpp");
		}

		luau_bindgen.generate()
			.expect("couldn't generate Luau bindings")
			.write_to_file(format!("{}/luau.rs", out_dir))
			.expect("couldn't write Luau bindings to file");

		eprintln!("successfully generated Luau bindings");
	}

	#[cfg(feature = "glue")] {
		let mut glue_cc = cc::Build::new();

		glue_cc
			.flag("-Iluau/Common/include")
			.flag("-Iluau/Ast/include")
			.flag("-Iluau/Compiler/include")
			.flag("-Iluau/Analysis/include")
			.flag("-Iluau/VM/include")
			.flag("-Iluau/VM/src")
			// MSVC requires /std:c++latest for designated initializers
			// I quite like them so I'm not going to stop using them
			// Windows being special for the fourth time
			.flag(if cfg!(target_os = "windows") { "/std:c++latest" } else { "-std=c++17" })
			.static_crt(true);

		#[cfg(target_os = "windows")]
		glue_cc.flag("/EHsc");

		let mut glue_bindgen = bindgen::builder()
			.parse_callbacks(Box::new(CargoCallbacks::new()))
			.clang_args([
				"-Iluau/Common/include",
				"-Iluau/Ast/include",
				"-Iluau/Compiler/include",
				"-Iluau/Analysis/include",
				"-Iluau/VM/include",
				"-Iluau/VM/src",
				"-xc++"
			])
			.allowlist_item("gluau.*")
			.allowlist_recursively(false)
			.default_enum_style(EnumVariation::Rust { non_exhaustive: false });

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
		eprintln!("successfully built glue code");

		glue_bindgen
			.generate()
			.expect("couldn't generate Luau glue bindings")
			.write_to_file(format!("{}/glue.rs", out_dir))
			.expect("couldn't write Luau glue bindings to file");

		eprintln!("successfully generated glue code bindings");
	}

	// all should be good
}

#[cfg(not(any(feature = "ast", feature = "compiler", feature = "analysis", feature = "vm")))]
fn main() {}

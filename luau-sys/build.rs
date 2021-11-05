fn main() {
	#[cfg(any(feature = "ast", feature = "compiler", feature = "analysis", feature = "vm"))]
		let out_dir = std::env::var("OUT_DIR").expect("couldn't find output directory");

	#[cfg(feature = "vm")] { // generate bindings to what we can
		bindgen::builder()
			.clang_arg("-Iluau/VM/include")
			.clang_arg("-std=c++17")
			.header("vm.hpp")
			.generate()
			.expect("couldn't generate bindings to Luau VM")
			.write_to_file(format!("{}/vm.rs", out_dir))
			.expect("couldn't write Luau VM bindings to file");
	}

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

	macro_rules! build_glue {
		($feat:literal) => {
			eprintln!(concat!("compiling glue for feature '", $feat, "'"));

			// cc links to it automatically
			cc::Build::new()
				.flag("-Iluau/Ast/include")
				.flag("-Iluau/Compiler/include")
				.flag("-Iluau/Analysis/include")
				.flag("-Iluau/VM/include")
				// MSVC requires /std:c++latest for designated initializers
				// I quite like them so I'm not going to stop using them
				// Windows being special for the fourth time
				.flag(if cfg!(target_os = "windows") { "/std:c++latest" } else { "-std=c++17" })
				.file(concat!("src/glue/", $feat, ".cpp"))
				.compile(concat!("gluau_", $feat));

			eprintln!(concat!("compiled and linked to glue '", $feat, "'"));
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

	macro_rules! bind_glue {
		($feat:literal) => {
			eprintln!(concat!("generating glue bindings for feature '", $feat, "'"));

			bindgen::builder()
				.clang_args([
					"-Iluau/Ast/include",
					"-Iluau/Compiler/include",
					"-Iluau/Analysis/include",
					"-Iluau/VM/include"
				])
				.allowlist_function("gluau_.*")
				.header(concat!("src/glue/", $feat, ".h"))
				.generate()
				.expect(concat!("couldn't generate Luau glue bindings for ", $feat))
				.write_to_file(format!(concat!("{}/glue_", $feat, ".rs"), out_dir))
				.expect(concat!("couldn't write Luau glue bindings for ", $feat, " to file"));

			eprintln!(concat!("generated bindings for glue '", $feat, "'"));
		}
	}

	macro_rules! maybe_bind_glue {
		($feat:literal) => {
			#[cfg(feature = $feat)]
			bind_glue!($feat)
		}
	}

	// generate bindings to glue code
	#[cfg(any(feature = "ast", feature = "compiler", feature = "analysis", feature = "vm"))]
	bind_glue!("common");
	maybe_bind_glue!("ast");
	maybe_bind_glue!("compiler");
	maybe_bind_glue!("analysis");
	maybe_bind_glue!("vm");

	// all should be good
}

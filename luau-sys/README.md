# luau-sys - Raw C bindings to Roblox's Luau

This crate aims to be an interface to Roblox's Luau, but _including_ parts of
Luau that bindgen cannot directly generate bindings for. C++ "glue" will be
compiled alongside Luau, and will expose additional C functions for bindgen to
chew on, that under-the-hood interact with parts of Luau's C++ interface.

The aim is to make it easier and safer to create Luau bindings that:

- can be used at all. Even methods that take `std::string` are too complicated
  to use directly from Rust, and it only goes downhill from there. This means
  it's impossible to even use the Luau compiler without glue code...
- are safe, and don't unwind past the FFI boundary. Uncaught C++ exceptions are
  undefined behavior, so instead the glue code catches exceptions and defines
  tagged unions that can encode errors without raising an exception. That way,
  it can be easily translated into a native Rust `Result`.

Interfaces exposed by the glue code can be identified by the prefix `gluau_`,
and the fact that they are contained in the `glue` submodule. The exact types
available will depend on which crate features you have enabled:

- `ast`: Enables basic support for parsing Luau source code into its AST
  representation and manipulating the returned AST.

  Does not depend on any other feature.
- `compiler`: Enables support for compiling Luau source code into bytecode.

  Depends on `ast`, which will be automatically enabled.
- `analysis`: Enables support for analyzing Luau source code.

  Depends on `ast`, which will be automatically enabled.
- `vm`: Enables support for executing Luau bytecode.

  Does not depend on any other feature.

This crate compiles on Windows and Linux, and should compile on macOS as well.

## !! WIP !!

This library is a heavy work-in-progress - that is, almost none of the features
listed above are actually functional at the moment. Additionally, it is not yet
published to Crates.io and has not yet committed to any release, meaning that
the API (glue code, module structure or otherwise) can and _will_ change
arbitrarily and without warning.

However, the fundamentals are still in place:

- Luau is compiled and linked to by the build script. Luau is included with the
  distribution; it is not retrieved from the network at build time
- C++ "glue" code is compiled and linked to, to expose C interfaces to C++-only
  functionality. Currently, only (certain parts of) the compiler methods are
  usable from Rust; although that is still sufficient to embed Luau as a runtime

You are advised to wait until this crate is more mature before trying to use it;
ideally wait until it is published to Crates.io, which should be in a few months
maybe(?).

No matter how long it takes, though, it will still have taken less than half the
time it'll take `h3` to fulfill their promises. That thing is vaporware by this
point.

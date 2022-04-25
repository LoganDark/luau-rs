# luau - Rust bindings to Roblox's Luau

This library is an interface between Rust and Luau. It aims to be easy-to-use,
fast, and safe (kind of like `rlua`). It also aims to support Luau-specific
features first-class, such as:

- types and analysis
- multiple VMs on different threads (Parallel Luau)
- thread-specific userdata support (for security identities and such)
- async/await support for yieldable calls and yielding from C functions
- very fast namecall
- sandboxing, readonly and other safety features

## !! WIP ALERT !!

This library is a heavy work-in-progress - that is, almost none of the features
listed above are actually functional at the moment. However, the fundamental
pieces are in place - `luau-sys` is available and aims to provide a safe C API
for interfacing with all of Luau, including the C++ parts.

Check out the [`README.md` for `luau-sys`](luau-sys/README.md) for more
information on how it works. The raw bindings are consumed by `luau`, which
smooths over the raw C interface with modern Rust types and safety. Most work on
this repository will be working with the "glue", as that's what dictates what
functionality is available for Rust to call into (by doing C++ stuff and then
translating that to pure C types).

The rest of the work will be consumer-facing API design of the `luau` crate that
consumes `luau-sys` - probably taking heavy inspiration from [`rlua`](
https://docs.rs/rlua). Once the crate is actually usable for creating Luau VMs,
compiling code for them, and executing it inside, all from Safe Rust, it may be
published to Crates.io if the API design is good enough.

For now, most of the code in the `luau` crate is just a proof-of-concept and
will be iterated upon and improved over time. What you see is most likely not
what will make it into the `0.1` release. I'm always open to feedback,
suggestions, and pull requests.

## Contributing

Make sure to read [`CONTRIBUTING.md`](CONTRIBUTING.md) for general guidelines on
questions, bug reports and contributions. 

## License

Copyright (C) 2021 LoganDark

This program is free software: you can redistribute it and/or modify
it under the terms of version 3 of the GNU General Public License as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

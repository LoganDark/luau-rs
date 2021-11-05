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
pieces are in place:

- Luau is compiled and linked to by the build script. Luau is included with the
  distribution; it is not retrieved from the network at build time
- C++ "glue" code is compiled and linked to, to expose C interfaces to C++-only
  functionality
- Both of these are exported by `luau-sys`, and consumed by `luau`, which
  smooths over the raw C interface with modern Rust types and safety

More work - mainly API design, probably heavily taking inspiration from `rlua` -
is needed to get the bindings into a state where they can be published to
`crates.io`. I've reserved the names `luau` and `luau-sys` for this purpose.

Most of the code in the `luau` crate is just a proof-of-concept and will be
iterated upon and improved over time. What you see is most likely not what will
make it into the `0.1` release.

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

# luau-rs - Rust bindings to Roblox's Luau
# Copyright (C) 2021 LoganDark
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of version 3 of the GNU General Public License as
# published by the Free Software Foundation.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# This file only exists for CLion analysis; it is not meant to be actually built

cmake_minimum_required(VERSION 3.0)
project(luau-sys LANGUAGES CXX)

add_subdirectory(luau)

add_library(luau-sys.ast STATIC ast.hpp)
add_library(luau-sys.vm STATIC vm.hpp)
add_library(luau-sys.compiler STATIC compiler.hpp)
add_library(luau-sys.glue-common STATIC src/glue/common.hpp src/glue/common.cpp)
add_library(luau-sys.glue-ast STATIC src/glue/ast.h src/glue/ast.cpp)
add_library(luau-sys.glue-compiler STATIC src/glue/compiler.h src/glue/compiler.cpp)
add_library(luau-sys.glue-analysis STATIC src/glue/analysis.h src/glue/analysis.cpp)
add_library(luau-sys.glue-vm STATIC src/glue/vm.h src/glue/vm.cpp)

target_compile_features(luau-sys.ast PUBLIC cxx_std_17)
target_compile_features(luau-sys.vm PUBLIC cxx_std_17)
target_compile_features(luau-sys.compiler PUBLIC cxx_std_17)
target_compile_features(luau-sys.glue-common PUBLIC cxx_std_17)
target_compile_features(luau-sys.glue-ast PUBLIC cxx_std_17)
target_compile_features(luau-sys.glue-compiler PUBLIC cxx_std_17)
target_compile_features(luau-sys.glue-analysis PUBLIC cxx_std_17)
target_compile_features(luau-sys.glue-vm PUBLIC cxx_std_17)

target_link_libraries(luau-sys.ast PUBLIC Luau.VM)
target_link_libraries(luau-sys.vm PUBLIC Luau.VM)
target_link_libraries(luau-sys.compiler PUBLIC Luau.VM)
target_link_libraries(luau-sys.glue-common PUBLIC Luau.AST Luau.Compiler Luau.Analysis Luau.VM)
target_link_libraries(luau-sys.glue-ast PUBLIC luau-sys.glue-common Luau.AST)
target_link_libraries(luau-sys.glue-compiler PUBLIC luau-sys.glue-common Luau.Compiler)
target_link_libraries(luau-sys.glue-analysis PUBLIC luau-sys.glue-common Luau.Analysis)
target_link_libraries(luau-sys.glue-vm PUBLIC luau-sys.glue-common Luau.VM)

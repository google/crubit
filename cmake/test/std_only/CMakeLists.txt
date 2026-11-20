# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Rust code can use the C++ standard library bindings (cc_std).
cmake_minimum_required(VERSION 3.22)
project(std_only LANGUAGES CXX)
include(${CRUBIT_SOURCE_DIR}/cmake/test/fixture_preamble.cmake)

corrosion_import_crate(MANIFEST_PATH rust_test/Cargo.toml)
crubit_link_rust_bindings(rust_test)

add_executable(main main.cc)
target_link_libraries(main PRIVATE rust_test)
add_test(NAME main COMMAND main)

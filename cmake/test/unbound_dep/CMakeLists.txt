# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# A dependency without bindings does not break generation: declarations that
# use its types are skipped.
cmake_minimum_required(VERSION 3.22)
project(unbound_dep LANGUAGES CXX)
include(${CRUBIT_SOURCE_DIR}/cmake/test/fixture_preamble.cmake)

add_library(dep INTERFACE)
target_include_directories(dep INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})

add_library(top STATIC top.cc)
target_include_directories(top PUBLIC $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>)
target_link_libraries(top PUBLIC dep)
crubit_rust_bindings(top_rs CC_TARGET top HEADERS top.h)

corrosion_import_crate(MANIFEST_PATH rust_test/Cargo.toml)
crubit_link_rust_bindings(rust_test top_rs)

add_executable(main main.cc)
target_link_libraries(main PRIVATE rust_test)
add_test(NAME main COMMAND main)

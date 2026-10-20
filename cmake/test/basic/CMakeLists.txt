# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Rust calls a C++ library through crubit_rust_bindings().
cmake_minimum_required(VERSION 3.22)
if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Debug)
endif()
project(basic LANGUAGES CXX)
include(${CRUBIT_SOURCE_DIR}/cmake/test/fixture_preamble.cmake)

add_library(basic STATIC basic.cc)
target_include_directories(basic PUBLIC $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/inc>)
target_compile_definitions(basic PUBLIC $<$<CONFIG:Debug>:FIXTURE_ALWAYS>)
crubit_rust_bindings(basic_rs CC_TARGET basic HEADERS inc/basic.h)

corrosion_import_crate(MANIFEST_PATH rust_test/Cargo.toml)
crubit_link_rust_bindings(rust_test basic_rs)

add_executable(main main.cc)
target_link_libraries(main PRIVATE rust_test)
add_test(NAME main COMMAND main)

# Test that editing a bound header regenerates the bindings.
add_test(NAME regenerate COMMAND ${CMAKE_COMMAND}
  -DBINARY_DIR=${CMAKE_BINARY_DIR}
  -DHEADER=${CMAKE_CURRENT_SOURCE_DIR}/inc/basic.h
  -DGENERATOR=${CMAKE_GENERATOR}
  -P ${CMAKE_CURRENT_SOURCE_DIR}/check_regenerate.cmake)

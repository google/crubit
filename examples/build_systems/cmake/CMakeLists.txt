# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
cmake_minimum_required(VERSION 3.22)
project(example_project VERSION 0.1.0)

include(FetchContent)

FetchContent_declare(
  crubit
  SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../../"
)

FetchContent_Declare(
  Corrosion
  GIT_REPOSITORY https://github.com/thunderseethe/corrosion
  GIT_TAG master # RESPECTFUL_TERMS_EXCEPTION
)

FetchContent_MakeAvailable(Corrosion)

corrosion_experimental_crubit(rust_lib)
corrosion_import_crate(MANIFEST_PATH rust_lib/Cargo.toml)

add_executable(main main.cpp)
target_link_libraries(main PRIVATE rust_lib)

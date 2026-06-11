# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
cmake_minimum_required(VERSION 3.22)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_C_COMPILER clang)
set(CMAKE_CXX_COMPILER clang++)

project(cmake_demo_scanner VERSION 0.1.0 LANGUAGES CXX)

include(FetchContent)
include(CTest)

# Fetch Crubit from the local repository root
# Because this is embedded within the Crubit repo, it refers to the parent
# directory directly.
# A downstream user would use:
# FetchContent_Declare(
#   crubit
#   GIT_REPOSITORY https://github.com/google/crubit
#   GIT_TAG main # or a git tag or commit hash
# )
# To fetch Crubit in their project.
FetchContent_Declare(
  crubit
  SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../../"
)

# Fetch Corrosion
# If users don't need a particular version of Crubit, Corrosion will fetch the
# latest version of Crubit by deafult, when one has not been declared like
# above.
FetchContent_Declare(
  Corrosion
  GIT_REPOSITORY https://github.com/thunderseethe/corrosion
  GIT_TAG main
)
FetchContent_MakeAvailable(Corrosion)

# Enable Crubit C++ binding generation for the Rust crate
# The CMake target name is derived from the Rust crate name, which is
# `rust_blake3_scanner` here.
corrosion_experimental_crubit(rust_blake3_scanner)
corrosion_import_crate(MANIFEST_PATH rust_lib/Cargo.toml)

add_executable(scanner_demo main.cpp)
target_link_libraries(scanner_demo PRIVATE rust_blake3_scanner)

if (BUILD_TESTING)
  # Run CTest against the current directory (.) as a quick test
  add_test(NAME scanner_demo_test COMMAND scanner_demo ".")
endif()

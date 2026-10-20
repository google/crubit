# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# CMake support for generating Rust bindings for C++ libraries with Crubit
# (`rs_bindings_from_cc`).
#
# Requirements: Clang with libc++, a nightly Rust toolchain (or
# RUSTC_BOOTSTRAP=1, which Crubit sets for the crates it links), and
# `rs_bindings_from_cc`, chosen by CRUBIT_RS_BINDINGS_FROM_CC: if unset, it is
# built from source (cmake/rs_bindings_from_cc); if empty, it is found on the
# PATH; otherwise it is the binary at that path.
#
# See docs/overview/build_systems.md.

include_guard(GLOBAL)

# Functions use the policy settings in effect where they are defined, so this
# applies to all of Crubit's functions, whatever the policies of the project
# calling them. The minimum matches the top-level CMakeLists.txt so that
# including Crubit for its C++ libraries alone keeps working on older CMake;
# bindings need 3.22, which _crubit_init_toolchain checks.
cmake_policy(VERSION 3.16...3.31)

include("${CMAKE_CURRENT_LIST_DIR}/internal/CrubitToolchain.cmake")
include("${CMAKE_CURRENT_LIST_DIR}/internal/CrubitClangArgs.cmake")

# Sets `out_var` to the root of the Crubit source tree.
function(_crubit_source_dir out_var)
  cmake_path(GET CMAKE_CURRENT_FUNCTION_LIST_DIR PARENT_PATH dir)
  set(${out_var} "${dir}" PARENT_SCOPE)
endfunction()

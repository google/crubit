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
include("${CMAKE_CURRENT_LIST_DIR}/internal/CrubitGenerate.cmake")
include("${CMAKE_CURRENT_LIST_DIR}/internal/CrubitStd.cmake")

# Sets `out_var` to the root of the Crubit source tree.
function(_crubit_source_dir out_var)
  cmake_path(GET CMAKE_CURRENT_FUNCTION_LIST_DIR PARENT_PATH dir)
  set(${out_var} "${dir}" PARENT_SCOPE)
endfunction()

# crubit_link_rust_bindings(<rust_target> [<crate>...])
#
# Makes the bindings crates `<crate>...` (created by crubit_rust_bindings), and
# the C++ standard library bindings crate `cc_std`, available to the Rust
# target `<rust_target>` imported with `corrosion_import_crate`. The crate's
# Cargo.toml depends on them as `<crate> = "0.0.0"`, with no path: Crubit's
# Cargo config tells Cargo where this build generated them (see
# CRUBIT_CARGO_CONFIG in _crubit_init_toolchain).
function(crubit_link_rust_bindings rust_target)
  _crubit_init_toolchain()
  _crubit_ensure_std()
  if(NOT TARGET _cargo-build_${rust_target})
    message(FATAL_ERROR
      "crubit_link_rust_bindings: ${rust_target} is not a Rust target imported "
      "with corrosion_import_crate")
  endif()
  set(crates cc_std ${ARGN})
  list(TRANSFORM crates PREPEND crubit_generate_ OUTPUT_VARIABLE generate_targets)
  list(TRANSFORM crates PREPEND crubit:: OUTPUT_VARIABLE libraries)
  add_dependencies(_cargo-build_${rust_target} ${generate_targets})
  target_link_libraries(${rust_target} INTERFACE ${libraries})
  get_target_property(flags ${rust_target} INTERFACE_CORROSION_CARGO_FLAGS)
  if(NOT "${CRUBIT_CARGO_CONFIG}" IN_LIST flags)
    corrosion_set_cargo_flags(${rust_target} --config "${CRUBIT_CARGO_CONFIG}")
    # Generated bindings use unstable Rust features.
    corrosion_set_env_vars(${rust_target} RUSTC_BOOTSTRAP=1)
  endif()
endfunction()

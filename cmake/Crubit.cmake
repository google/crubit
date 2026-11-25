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

# crubit_rust_bindings(<crate> CC_TARGET <target>
#                      [HEADERS <header>...]
#                      [FEATURES <feature>...])
#
# Generates Rust bindings for the C++ library `<target>` as the Rust crate
# `<crate>`, and the C++ library `crubit::<crate>` holding the C++ side of the
# bindings. Use crubit_link_rust_bindings to use the crate from Rust.
#
# HEADERS are the public headers to generate bindings for (relative to the
# current source directory); by default, the target's `HEADERS` file set,
# which needs CMake 3.23.
# FEATURES are the Crubit features enabled for the target (default:
# `supported`); see docs/overview/status.md.
function(crubit_rust_bindings crate)
  cmake_parse_arguments(PARSE_ARGV 1 ARG "" "CC_TARGET" "HEADERS;FEATURES")
  if(NOT ARG_CC_TARGET OR NOT TARGET "${ARG_CC_TARGET}")
    message(FATAL_ERROR
      "crubit_rust_bindings(${crate}): CC_TARGET must name a C++ library target")
  endif()
  _crubit_init_toolchain()
  set(tgt "${ARG_CC_TARGET}")
  get_target_property(aliased ${tgt} ALIASED_TARGET)
  if(aliased)
    set(tgt "${aliased}")
  endif()

  set(headers "")
  foreach(header IN LISTS ARG_HEADERS)
    cmake_path(ABSOLUTE_PATH header NORMALIZE)
    list(APPEND headers "${header}")
  endforeach()
  if(NOT ARG_FEATURES)
    set(ARG_FEATURES supported)
  endif()
  # Crubit's `cxx` crate interop is not supported with CMake.
  list(APPEND ARG_FEATURES no_impl_cxx_extern_type)

  set_target_properties(${tgt} PROPERTIES
    CRUBIT_CRATE_NAME "${crate}"
    CRUBIT_HEADERS "${headers}"
    CRUBIT_FEATURES "${ARG_FEATURES}")
  # Generate once all targets and their dependencies are known. (Arguments of
  # deferred calls are evaluated late, so bind `tgt` now.)
  cmake_language(EVAL CODE
    "cmake_language(DEFER DIRECTORY [[${CMAKE_SOURCE_DIR}]] CALL _crubit_finalize [[${tgt}]])")
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

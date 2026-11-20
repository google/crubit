# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

include_guard(GLOBAL)

set(CRUBIT_CLANG_RESOURCE_DIR "" CACHE PATH
  "Clang resource directory (builtin headers) for rs_bindings_from_cc: that of \
the LLVM it is built with. Required when CRUBIT_RS_BINDINGS_FROM_CC is set.")

# Validates the toolchain and initializes Crubit's global state. Runs once per
# configure; called by every public Crubit function, so that projects which
# include Crubit only for its C++ support libraries are not affected.
#
# Sets (as INTERNAL cache entries):
#   CRUBIT_CRATES_DIR:   directory holding generated bindings crates.
#   CRUBIT_CARGO_CONFIG: cargo config file that patches those crates in.
function(_crubit_init_toolchain)
  get_property(initialized GLOBAL PROPERTY _CRUBIT_TOOLCHAIN_INITIALIZED)
  if(initialized)
    return()
  endif()
  set_property(GLOBAL PROPERTY _CRUBIT_TOOLCHAIN_INITIALIZED TRUE)

  # CRUBIT_RS_BINDINGS_FROM_CC chooses where rs_bindings_from_cc comes from:
  #   - Not set: build it from source (cmake/rs_bindings_from_cc).
  #   - Empty, as from `-DCRUBIT_RS_BINDINGS_FROM_CC=`: search the PATH.
  #   - A path: use that binary, which must exist.
  # Read the result with _crubit_rs_bindings_from_cc.
  if(NOT DEFINED CRUBIT_RS_BINDINGS_FROM_CC)
    # The project may have already added cmake/rs_bindings_from_cc itself.
    if(NOT TARGET crubit::rs_bindings_from_cc)
      _crubit_source_dir(crubit_dir)
      add_subdirectory("${crubit_dir}/cmake/rs_bindings_from_cc"
        "${CMAKE_BINARY_DIR}/crubit_rs_bindings_from_cc")
    endif()
  elseif(NOT CRUBIT_RS_BINDINGS_FROM_CC)
    # Also reached after a failed search, which leaves `...-NOTFOUND`.
    # find_program doesn't search while the variable holds any value, even an
    # empty one, so clear it first.
    unset(CRUBIT_RS_BINDINGS_FROM_CC CACHE)
    find_program(CRUBIT_RS_BINDINGS_FROM_CC rs_bindings_from_cc REQUIRED
      DOC "Path to the rs_bindings_from_cc binary used to generate Rust bindings")
  elseif(NOT EXISTS "${CRUBIT_RS_BINDINGS_FROM_CC}")
    message(FATAL_ERROR
      "Crubit: CRUBIT_RS_BINDINGS_FROM_CC is set to "
      "${CRUBIT_RS_BINDINGS_FROM_CC}, which does not exist.")
  endif()

  include(CheckCXXSourceCompiles)
  # check_cxx_source_compiles caches its result, so after the user fixes
  # CMAKE_CXX_FLAGS a reconfigure would reuse the stale answer. Check again.
  unset(CRUBIT_HAS_LIBCXX CACHE)
  check_cxx_source_compiles([[
    #include <version>
    #ifndef _LIBCPP_VERSION
    #error not libc++
    #endif
    int main() {}
  ]] CRUBIT_HAS_LIBCXX)
  if(NOT CRUBIT_HAS_LIBCXX)
    message(FATAL_ERROR
      "Crubit requires libc++ (set -stdlib=libc++ in CMAKE_CXX_FLAGS and build "
      "all C++ code, including dependencies, against it).")
  endif()

  # rs_bindings_from_cc parses with the builtin headers of its embedded Clang
  # rather than the compiler to be used for actually compiling the source.
  if(NOT CRUBIT_CLANG_RESOURCE_DIR)
    get_property(resource_dir GLOBAL PROPERTY _CRUBIT_LLVM_RESOURCE_DIR)
    if(NOT resource_dir)
      message(FATAL_ERROR
        "Crubit: set CRUBIT_CLANG_RESOURCE_DIR to the Clang resource directory "
        "(such as <llvm>/lib/clang/<version>) of the LLVM that "
        "rs_bindings_from_cc was built with.")
    endif()
    set_property(CACHE CRUBIT_CLANG_RESOURCE_DIR PROPERTY VALUE "${resource_dir}")
  endif()
  # rs_bindings_from_cc names its builtin headers after this path, and they're
  # looked up by exact path, so use a single spelling everywhere.
  set(resource_dir "${CRUBIT_CLANG_RESOURCE_DIR}")
  cmake_path(NORMAL_PATH resource_dir)
  string(REGEX REPLACE "(.)/$" "\\1" resource_dir "${resource_dir}")
  set_property(CACHE CRUBIT_CLANG_RESOURCE_DIR PROPERTY VALUE "${resource_dir}")
  if(NOT IS_DIRECTORY "${CRUBIT_CLANG_RESOURCE_DIR}/include")
    message(FATAL_ERROR
      "Crubit: CRUBIT_CLANG_RESOURCE_DIR (${CRUBIT_CLANG_RESOURCE_DIR}) has no "
      "include directory.")
  endif()

  set(crates_dir "${CMAKE_BINARY_DIR}/crubit_crates")
  set(CRUBIT_CRATES_DIR "${crates_dir}" CACHE INTERNAL "")

  # How Rust crates find the bindings crates.
  #
  # Corrosion builds Rust with Cargo. Because the bindings crates are
  # generated into this build directory, the checked-in `Cargo.toml`
  # cannot reference them (the choice of build directory is not stable).
  #
  # Thus, in `Cargo.toml`, we depend on the generated crates only by version, as
  # `<crate> = "0.0.0"`. A Cargo config file defined here in the cache variable
  # named `CRUBIT_CARGO_CONFIG` tells Cargo where each crate lives.
  #
  # This config file starts as
  #
  # ```
  # [patch.crates-io]
  # ```
  #
  # Each call to `_crubit_generate` appends a line like this:
  #
  # ```
  # cc_std = { path = "<build>/crubit_crates/cc_std" }
  # ```
  #
  # `[patch.crates-io]` is Cargo's standard way to replace crates.io crates:
  # while resolving dependencies, a patched crate whose version matches the
  # requirement is used instead of the registry's. The local patches are
  # preferred to the registry's versions.
  #
  # Cargo's config-file search algorithm doesn't find this file by itself.
  # Instead, `crubit_link_rust_bindings` gives Cargo its path with
  # `--config <file>` (through corrosion_set_cargo_flags), which loads it on top
  # of the files Cargo finds.
  set(CRUBIT_CARGO_CONFIG "${crates_dir}/cargo-config.toml" CACHE INTERNAL "")

  # Rewrite the file from scratch: this runs once at the start of each
  # configure (see _CRUBIT_TOOLCHAIN_INITIALIZED), and each _crubit_generate
  # appends its crate, so the file lists only crates this configure generated.
  file(WRITE "${CRUBIT_CARGO_CONFIG}" "[patch.crates-io]\n")
endfunction()

# If `CRUBIT_RS_BINDINGS_FROM_CC` is set, sets `out_var` to the value held by
# the cache variable `CRUBIT_RS_BINDINGS_FROM_CC`. Otherwise, sets `out_var` to
# the target `crubit::rs_bindings_from_cc`. In either case, this is the value
# to be used for COMMAND and DEPENDS in `add_custom_command`. Use this rather
# than reading `CRUBIT_RS_BINDINGS_FROM_CC` directly, which is empty when the
# target is used.
function(_crubit_rs_bindings_from_cc out_var)
  if(CRUBIT_RS_BINDINGS_FROM_CC)
    set(${out_var} "${CRUBIT_RS_BINDINGS_FROM_CC}" PARENT_SCOPE)
  else()
    set(${out_var} crubit::rs_bindings_from_cc PARENT_SCOPE)
  endif()
endfunction()

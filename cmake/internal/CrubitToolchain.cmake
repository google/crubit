# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

include_guard(GLOBAL)

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

  set(crates_dir "${CMAKE_BINARY_DIR}/crubit_crates")
  set(CRUBIT_CRATES_DIR "${crates_dir}" CACHE INTERNAL "")
  set(CRUBIT_CARGO_CONFIG "${crates_dir}/cargo-config.toml" CACHE INTERNAL "")
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

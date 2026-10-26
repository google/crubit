# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Functions that work out the environment variables Cargo builds
# rs_bindings_from_cc with. CMakeLists.txt calls them.
#
# _crubit_cargo_env() is the entry point. The functions before it each work out
# one part of the environment.

# _crubit_system_library_flags(<out_flags>)
#
# Sets <out_flags> to rustc flags that link the system libraries that LLVM
# and Abseil need.
#
# Reads: llvm_system_libs.
function(_crubit_system_library_flags out_flags)
  set(flags "")
  foreach(lib IN LISTS llvm_system_libs)
    # LLVM releases may name libraries by their absolute paths on the machine
    # that built the release. Where that path doesn't exist here, link the
    # library by name instead.
    if(IS_ABSOLUTE "${lib}" AND NOT EXISTS "${lib}")
      get_filename_component(name "${lib}" NAME_WE)
      string(REGEX REPLACE "^lib" "" name "${name}")
      set(lib "-l${name}")
    endif()
    list(APPEND flags "-C" "link-arg=${lib}")
  endforeach()

  if(APPLE)
    # Abseil's time zone support uses CoreFoundation.
    list(APPEND flags "-C" "link-arg=-framework" "-C" "link-arg=CoreFoundation")
  endif()
  set(${out_flags} "${flags}" PARENT_SCOPE)
endfunction()

# _crubit_cargo_toolchain(<out_env> <out_flags>)
#
# Picks the C compiler, archiver and linker that Cargo uses. Sets <out_env> to
# environment variables (CC, and maybe AR and SDKROOT), and <out_flags> to
# rustc flags that choose the linker.
#
# LLVM releases ship their static libraries as LTO bitcode, which only tools
# from the same LLVM version can read. So if the LLVM install has clang and
# lld, Cargo uses them:
#   * lld, through clang, to link rs_bindings_from_cc.
#   * clang to compile C code, because the `google-protobuf` crate compiles
#     it with `-flto=thin`, which the linker then has to read too.
#   * llvm-ar to archive it, because other archivers, like Apple's `ar`, can't
#     read that bitcode.
# Otherwise, Cargo uses this project's C compiler and its default tools.
#
# Reads: llvm_clang, llvm_lld, llvm_ar.
function(_crubit_cargo_toolchain out_env out_flags)
  if(NOT (llvm_clang AND llvm_lld))
    set(${out_env} "CC=${CMAKE_C_COMPILER}" PARENT_SCOPE)
    set(${out_flags} "" PARENT_SCOPE)
    return()
  endif()

  set(env "CC=${llvm_clang}" "AR=${llvm_ar}")
  set(flags "-C" "linker=${llvm_clang}" "-C" "link-arg=-fuse-ld=lld")
  if(CMAKE_HOST_APPLE)
    # The LLVM release's clang doesn't find the macOS SDK on its own.
    execute_process(
      COMMAND xcrun --show-sdk-path
      OUTPUT_VARIABLE sdk_path
      OUTPUT_STRIP_TRAILING_WHITESPACE
      COMMAND_ERROR_IS_FATAL ANY
    )
    list(APPEND env "SDKROOT=${sdk_path}")
  endif()
  set(${out_env} "${env}" PARENT_SCOPE)
  set(${out_flags} "${flags}" PARENT_SCOPE)
endfunction()

# _crubit_encode_rustflags(<out> <flag>...)
#
# Sets <out> to the flags in the format of CARGO_ENCODED_RUSTFLAGS, which
# separates flags with the ASCII "unit separator" character instead of spaces,
# so that a flag can contain spaces.
function(_crubit_encode_rustflags out)
  string(ASCII 31 unit_separator)
  list(JOIN ARGN "${unit_separator}" encoded)
  set(${out} "${encoded}" PARENT_SCOPE)
endfunction()

# _crubit_cargo_lockfile(<out_path>)
#
# Copies the repository's `Cargo.lock` into the build directory, and sets
# `<out_path>` to the copy. `_crubit_cargo_env()` passes the copy to Cargo as
# `CARGO_RESOLVER_LOCKFILE_PATH`, which tells Cargo to read and write its
# lockfile there instead of next to `Cargo.toml`. (The copy must be named
# exactly `Cargo.lock`.)
#
# `Cargo.lock` serves both Cargo and Bazel. Bazel's Rust rules read it to pin
# the crates in the `crate.spec` entries of `MODULE.bazel`, so it also lists
# crates that are not used by any Cargo package. Cargo notices those unused
# entries and wants to delete them, so:
#   * `cargo build` would quietly rewrite the repository's `Cargo.lock`, leaving
#     a change in the checkout that would break Bazel if committed.
#   * `cargo build --locked` refuses to change the file, and fails.
# With the copy, Cargo starts from the same pinned versions and prunes the
# Bazel-only entries on the copy rather than the repository's file.
#
# The build depends on the repository's `Cargo.lock`, so editing it reruns CMake
# and copies it again. Once Cargo has pruned the copy, the two differ, so every
# rerun of CMake will repeatedly (but harmlessly) copy and prune it again.
#
# Reads: CRUBIT_SOURCE_ROOT.
function(_crubit_cargo_lockfile out_path)
  set(source "${CRUBIT_SOURCE_ROOT}/Cargo.lock")
  set(copy "${CMAKE_CURRENT_BINARY_DIR}/cargo_lock/Cargo.lock")
  file(MAKE_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/cargo_lock")
  file(COPY_FILE "${source}" "${copy}" ONLY_IF_DIFFERENT)
  set_property(DIRECTORY APPEND PROPERTY CMAKE_CONFIGURE_DEPENDS "${source}")
  set(${out_path} "${copy}" PARENT_SCOPE)
endfunction()

# _crubit_cargo_env(<out_env>)
#
# Sets <out_env> to the environment variables that Cargo builds
# rs_bindings_from_cc with, as a list of NAME=value entries. The values may
# contain $<...> generator expressions.
#
# The *_INCLUDE_PATH and *_LIB_STATIC_PATH variables tell cargo/build where the
# headers and static libraries are (see cargo/build/paths.rs). Include paths
# are comma-separated lists.
#
# Reads: the variables set by _crubit_describe_llvm() and
#   _crubit_use_installed_abseil_protobuf() or _crubit_build_abseil_protobuf(),
#   and cargo_lib_dir, proto_gen_dir and cargo_target_dir.
function(_crubit_cargo_env out_env)
  _crubit_system_library_flags(system_library_flags)
  _crubit_cargo_toolchain(toolchain_env toolchain_flags)
  _crubit_encode_rustflags(rustflags ${system_library_flags} ${toolchain_flags})
  _crubit_cargo_lockfile(lockfile)
  list(JOIN absl_include_dirs "," absl_include_path)
  list(JOIN protobuf_include_dirs "," protobuf_include_path)

  set(env
    ${toolchain_env}
    "CXX=${CMAKE_CXX_COMPILER}"
    "CXXFLAGS=-DCRUBIT_LLVM_DEV_DATE=${llvm_dev_date}"
    "CARGO_ENCODED_RUSTFLAGS=${rustflags}"
    "CLANG_INCLUDE_PATH=${llvm_include_dir}"
    "CLANG_LIB_STATIC_PATH=${cargo_lib_dir}"
    "ABSL_INCLUDE_PATH=${absl_include_path}"
    "ABSL_LIB_STATIC_PATH=${cargo_lib_dir}"
    "PROTOBUF_INCLUDE_PATH=${proto_gen_dir},${protobuf_include_path}"
    "PROTOBUF_LIB_STATIC_PATH=${cargo_lib_dir}"
    "PROTOC=${protoc}"
    "CARGO_TARGET_DIR=${cargo_target_dir}"
    "CARGO_RESOLVER_LOCKFILE_PATH=${lockfile}"
  )
  set(${out_env} "${env}" PARENT_SCOPE)
endfunction()

# _crubit_write_env_script(<path> <NAME=value>...)
#
# Writes a shell script to <path> that exports the given environment
# variables. Sourcing it lets you run Cargo directly, with the same
# environment as the CMake build.
#
# The file is written when CMake generates the build files, which replaces any
# $<...> generator expressions in the values. It's only rewritten when its
# contents change.
function(_crubit_write_env_script path)
  set(script "# Generated by cmake/rs_bindings_from_cc.\n")
  foreach(entry IN LISTS ARGN)
    string(FIND "${entry}" "=" equals)
    string(SUBSTRING "${entry}" 0 ${equals} name)
    math(EXPR equals "${equals} + 1")
    string(SUBSTRING "${entry}" ${equals} -1 value)
    # Single-quote the value for the shell; a ' inside it becomes '\''.
    string(REPLACE "'" "'\\''" value "${value}")
    string(APPEND script "export ${name}='${value}'\n")
  endforeach()
  file(GENERATE OUTPUT "${path}" CONTENT "${script}")
endfunction()

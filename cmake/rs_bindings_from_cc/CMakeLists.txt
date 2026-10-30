# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Builds `rs_bindings_from_cc` (rust_api_from_cpp) with CMake and Cargo.
#
# CMake provides the C++ dependencies needed by Cargo:
#   * LLVM/Clang:
#     - Uses `CRUBIT_LLVM_ROOT` if specified
#     - Otherwise finds an installed LLVM with `find_package(LLVM)`
#     - Otherwise downloads an official LLVM release at configure time.
#   * Abseil and Protobuf:
#     - Uses installed packages found with `find_package`, if suitable.
#     - Otherwise builds them from source at the same versions used by Bazel.
#
#     Note that the `protobuf` crate in `Cargo.toml` must match the built
#     protobuf version exactly.
#
# Static libraries are then merged into the layout expected by `cargo/build`,
# C++ Protobuf headers are generated, and `cargo build` is executed.
#
# The tool is available as the imported executable `crubit::rs_bindings_from_cc`,
# e.g. for `$<TARGET_FILE:crubit::rs_bindings_from_cc>` in custom commands.
#
# Usage, from the root of the Crubit checkout:
# ```
# cmake -S cmake/rs_bindings_from_cc -B cmake_build -G Ninja
# cmake --build cmake_build
# ./cmake_build/bin/rs_bindings_from_cc --help
# ```
#
# The build also writes `crubit-cargo-env.sh`. Source it to run Cargo directly:
# ```
# source cmake_build/crubit-cargo-env.sh
# cargo build --release -p rs_bindings_from_cc
# ```

cmake_minimum_required(VERSION 3.22...3.31)

project(crubit_rs_bindings_from_cc LANGUAGES C CXX)

include(ExternalProject)
include(FetchContent)

get_filename_component(CRUBIT_SOURCE_ROOT "${CMAKE_CURRENT_LIST_DIR}/../.." ABSOLUTE)

set(CRUBIT_LLVM_ROOT "" CACHE PATH
  "Root of an LLVM/Clang install with static libraries, headers and llvm-config. If empty, find_package(LLVM) is tried, then an official LLVM release is downloaded.")
set(CRUBIT_LLVM_VERSION "23.1.3" CACHE STRING
  "LLVM version to look for with find_package, or to download.")
set(CRUBIT_LLVM_URL "" CACHE STRING
  "Override the URL of the LLVM release archive to download.")
set(CRUBIT_LLVM_URL_HASH "" CACHE STRING
  "Hash of the LLVM archive, as ALGO=value (e.g. SHA256=...). Known for the default LLVM releases.")
set(CRUBIT_LLVM_DEV_DATE "" CACHE STRING
  "Date (YYYYMMDD) of the LLVM commit, used for Crubit's LLVM compatibility checks. Detected for known LLVM releases.")
option(CRUBIT_FIND_DEPENDENCIES
  "Use installed Abseil and Protobuf packages when they are suitable, instead of building them." ON)
set(CRUBIT_ABSL_VERSION "20260526.0" CACHE STRING "Abseil release to build.")
set(CRUBIT_PROTOBUF_VERSION "36.1" CACHE STRING
  "Protobuf release to use. Must match the protobuf crate version in Cargo.toml.")
set(CRUBIT_CARGO "cargo" CACHE STRING "Cargo executable.")
set(CRUBIT_CARGO_PROFILE "release" CACHE STRING "Cargo profile used to build rs_bindings_from_cc.")

if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
  set(CMAKE_BUILD_TYPE Release)
endif()

include("${CMAKE_CURRENT_LIST_DIR}/hashes.cmake")
include("${CMAKE_CURRENT_LIST_DIR}/llvm.cmake")
include("${CMAKE_CURRENT_LIST_DIR}/abseil_protobuf.cmake")
include("${CMAKE_CURRENT_LIST_DIR}/cargo_inputs.cmake")
include("${CMAKE_CURRENT_LIST_DIR}/cargo_env.cmake")

# Directories in the build directory.
#   deps_dir: where Abseil and Protobuf are built, if they're built.
#   cargo_lib_dir: the static libraries that Cargo links, all in one
#     directory, mirroring `cargo/build/setup_bazel_env.py`.
#   proto_gen_dir: C++ sources generated from .proto files.
#   cargo_target_dir: Cargo's own output.
set(deps_dir "${CMAKE_CURRENT_BINARY_DIR}/deps")
set(cargo_lib_dir "${CMAKE_CURRENT_BINARY_DIR}/cargo_deps/lib")
set(proto_gen_dir "${CMAKE_CURRENT_BINARY_DIR}/cargo_deps/include")
set(cargo_target_dir "${CMAKE_CURRENT_BINARY_DIR}/cargo_target")
file(MAKE_DIRECTORY "${cargo_lib_dir}" "${proto_gen_dir}")

# ===== Step 1: LLVM and Clang =====
# Use CRUBIT_LLVM_ROOT or an installed LLVM, or else download a release.

_crubit_find_llvm(llvm_root)
if(NOT llvm_root)
  _crubit_download_llvm(llvm_root)
endif()
_crubit_describe_llvm("${llvm_root}")

# ===== Step 2: Abseil and Protobuf =====
# Use installed packages if they're suitable, or else build them from source.

set(found_abseil_protobuf FALSE)
if(CRUBIT_FIND_DEPENDENCIES)
  _crubit_find_abseil_protobuf(found_abseil_protobuf)
endif()
if(found_abseil_protobuf)
  _crubit_use_installed_abseil_protobuf()
else()
  _crubit_build_abseil_protobuf()
endif()

# ===== Step 3: Files for Cargo =====
# Merge each dependency's static libraries into one library, and generate the
# C++ Protobuf sources.

_crubit_merge_archives(libclang_monolithic.a
  MODE llvm INPUT_DIR "${llvm_lib_dir}" DEPENDS ${llvm_libraries})
_crubit_merge_archives(libabsl_monolithic.a ${absl_archives})
_crubit_merge_archives(libprotobuf.a ${protobuf_archives})
_crubit_generate_protos(proto_outputs)

# ===== Step 4: Cargo's environment =====
# Also write it to crubit-cargo-env.sh, for running Cargo by hand.

_crubit_cargo_env(cargo_env)
_crubit_write_env_script("${CMAKE_CURRENT_BINARY_DIR}/crubit-cargo-env.sh" ${cargo_env})

# ===== Step 5: Build rs_bindings_from_cc with Cargo =====

if(CRUBIT_CARGO_PROFILE STREQUAL "dev")
  set(cargo_profile_dir "debug")
else()
  set(cargo_profile_dir "${CRUBIT_CARGO_PROFILE}")
endif()
set(cargo_binary
  "${cargo_target_dir}/${cargo_profile_dir}/rs_bindings_from_cc${CMAKE_EXECUTABLE_SUFFIX}")
set(binary "${CMAKE_CURRENT_BINARY_DIR}/bin/rs_bindings_from_cc${CMAKE_EXECUTABLE_SUFFIX}")

add_custom_target(rs_bindings_from_cc ALL
  COMMAND "${CMAKE_COMMAND}" -E env ${cargo_env}
    "${CRUBIT_CARGO}" build --profile "${CRUBIT_CARGO_PROFILE}" -p rs_bindings_from_cc
  COMMAND "${CMAKE_COMMAND}" -E copy_if_different "${cargo_binary}" "${binary}"
  WORKING_DIRECTORY "${CRUBIT_SOURCE_ROOT}"
  DEPENDS
    "${cargo_lib_dir}/libclang_monolithic.a"
    "${cargo_lib_dir}/libabsl_monolithic.a"
    "${cargo_lib_dir}/libprotobuf.a"
    ${proto_outputs}
  BYPRODUCTS "${binary}"
  COMMENT "Building rs_bindings_from_cc with Cargo"
  USES_TERMINAL
  VERBATIM
)

# The tool as an executable target. Targets that use it, e.g. in custom
# commands via `$<TARGET_FILE:crubit::rs_bindings_from_cc>` build it first.
add_executable(crubit_rs_bindings_from_cc IMPORTED GLOBAL)
set_target_properties(crubit_rs_bindings_from_cc PROPERTIES IMPORTED_LOCATION "${binary}")
add_dependencies(crubit_rs_bindings_from_cc rs_bindings_from_cc)
add_executable(crubit::rs_bindings_from_cc ALIAS crubit_rs_bindings_from_cc)

install(IMPORTED_RUNTIME_ARTIFACTS crubit_rs_bindings_from_cc)

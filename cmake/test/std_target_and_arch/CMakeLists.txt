# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Like std_target, but CMAKE_CXX_COMPILER_TARGET names the host's architecture
# while CMAKE_OSX_ARCHITECTURES names the other one. CMake passes both
# `--target` and `-arch`, and Clang builds for the `-arch` one, so cc_std must
# be generated for it too. macOS only.
cmake_minimum_required(VERSION 3.22)
# CMAKE_HOST_SYSTEM_PROCESSOR is not set before project().
execute_process(COMMAND uname -m
  OUTPUT_VARIABLE host_arch OUTPUT_STRIP_TRAILING_WHITESPACE)
if(host_arch MATCHES "^(arm64|aarch64)$")
  set(host_triple arm64-apple-macos)
  set(arch x86_64)
  set(machine_dir i386)
else()
  set(host_triple x86_64-apple-macos)
  set(arch arm64)
  set(machine_dir arm)
endif()
set(CMAKE_CXX_COMPILER_TARGET "${host_triple}" CACHE STRING "" FORCE)
set(CMAKE_OSX_ARCHITECTURES "${arch}" CACHE STRING "" FORCE)
project(std_target_and_arch LANGUAGES CXX)
if(NOT APPLE)
  return()
endif()
include(${CRUBIT_SOURCE_DIR}/cmake/test/fixture_preamble.cmake)

# TODO: Replace calls to internal functions when the public API is available.
_crubit_init_toolchain()
_crubit_ensure_std()
# Only the configure step is tested: don't run rs_bindings_from_cc.
set_target_properties(crubit_impl_cc_std PROPERTIES EXCLUDE_FROM_ALL ON)

add_test(NAME check_headers COMMAND ${CMAKE_COMMAND}
  -DRSP=${CRUBIT_CRATES_DIR}/cc_std/cc_std.rsp
  -DMACHINE_DIR=${machine_dir}
  -P ${CMAKE_CURRENT_SOURCE_DIR}/../std_target/check_headers.cmake)

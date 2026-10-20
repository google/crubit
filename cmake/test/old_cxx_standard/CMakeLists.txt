# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Configuring with a C++ standard below C++20 must fail with a clear message.
cmake_minimum_required(VERSION 3.22)
set(CMAKE_CXX_STANDARD 17)
project(old_cxx_standard LANGUAGES CXX)
include(${CRUBIT_SOURCE_DIR}/cmake/test/fixture_preamble.cmake)

# TODO: Replace calls to internal functions when the public API is available.
_crubit_init_toolchain()
_crubit_ensure_std()

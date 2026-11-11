# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Configuring against libstdc++ must fail with a clear message.
cmake_minimum_required(VERSION 3.22)
set(CMAKE_CXX_FLAGS "-stdlib=libstdc++" CACHE STRING "" FORCE)
project(not_libcxx LANGUAGES CXX)
include(${CRUBIT_SOURCE_DIR}/cmake/test/fixture_preamble.cmake)

# TODO: Replace call to internal `_crubit_init_toolchain` when the public API
# is available.
_crubit_init_toolchain()

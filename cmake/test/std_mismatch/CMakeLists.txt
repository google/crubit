# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Configuring must fail with a clear message when rs_bindings_from_cc would
# parse different standard library headers than the build compiles with.
cmake_minimum_required(VERSION 3.22)
project(std_mismatch LANGUAGES CXX)
include(${CRUBIT_SOURCE_DIR}/cmake/test/fixture_preamble.cmake)

# Differences in the compiler's builtin headers are allowed, once they're
# dropped as _crubit_ensure_std does.
set(build /sdk/usr/include/stdio.h /cc/lib/clang/21/include/stddef.h)
set(crubit /sdk/usr/include/stdio.h /llvm/lib/clang/23/include/stddef.h)
_crubit_drop_builtin_headers(build)
_crubit_drop_builtin_headers(crubit)
_crubit_std_check_headers(BUILD ${build} CRUBIT ${crubit})
# Other differences are not.
_crubit_std_check_headers(
  BUILD /sdk/usr/include/stdio.h /sdk/usr/include/i386/_types.h
  CRUBIT /sdk/usr/include/stdio.h /sdk/usr/include/arm/_types.h)

# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Driver for the Crubit CMake module tests. Each fixture under this directory is
# a standalone CMake project, configured, built and tested with
# `ctest --build-and-test`.
#
#   cmake -S cmake/test -B build-cmake-test \
#     -DCRUBIT_RS_BINDINGS_FROM_CC=$PWD/bazel-bin/rs_bindings_from_cc/rs_bindings_from_cc
#   ctest --test-dir build-cmake-test --output-on-failure

cmake_minimum_required(VERSION 3.22)
project(crubit_cmake_tests LANGUAGES NONE)
enable_testing()

get_filename_component(CRUBIT_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../.." ABSOLUTE)

# Fetch Corrosion once and share its sources with every fixture.
include(FetchContent)
FetchContent_Declare(
  Corrosion
  GIT_REPOSITORY https://github.com/corrosion-rs/corrosion
  GIT_TAG v0.6.1
  GIT_SHALLOW TRUE
)
FetchContent_GetProperties(Corrosion)
if(NOT corrosion_POPULATED)
  FetchContent_Populate(Corrosion)
endif()

set(_crubit_fixture_options
  -DCRUBIT_SOURCE_DIR=${CRUBIT_SOURCE_DIR}
  -DFETCHCONTENT_SOURCE_DIR_CORROSION=${corrosion_SOURCE_DIR}
)
# Pass CRUBIT_RS_BINDINGS_FROM_CC through only if given: Crubit builds the tool
# from source when it is unset, but searches the PATH when it is empty.
if(DEFINED CRUBIT_RS_BINDINGS_FROM_CC)
  list(APPEND _crubit_fixture_options
    "-DCRUBIT_RS_BINDINGS_FROM_CC=${CRUBIT_RS_BINDINGS_FROM_CC}")
endif()
foreach(var CMAKE_CXX_COMPILER CMAKE_CXX_FLAGS CMAKE_BUILD_TYPE)
  if(DEFINED ${var})
    list(APPEND _crubit_fixture_options "-D${var}=${${var}}")
  endif()
endforeach()

# crubit_add_fixture(<name> [FAIL_REGEX <regex>] [LINUX_ONLY] [SOURCE_DIR <dir>])
#
# Registers the fixture project in `<SOURCE_DIR>` (default: `./<name>`) as a
# test. With FAIL_REGEX, the test passes iff the output matches the regex
# (used for fixtures that must fail to configure).
function(crubit_add_fixture name)
  cmake_parse_arguments(PARSE_ARGV 1 ARG "LINUX_ONLY" "FAIL_REGEX;SOURCE_DIR" "")
  if(ARG_LINUX_ONLY AND NOT CMAKE_HOST_SYSTEM_NAME STREQUAL "Linux")
    return()
  endif()
  if(NOT ARG_SOURCE_DIR)
    set(ARG_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/${name}")
  endif()
  add_test(
    NAME ${name}
    COMMAND ${CMAKE_CTEST_COMMAND}
      --build-and-test "${ARG_SOURCE_DIR}" "${CMAKE_CURRENT_BINARY_DIR}/${name}"
      --build-generator "${CMAKE_GENERATOR}"
      --build-options ${_crubit_fixture_options}
      --test-command ${CMAKE_CTEST_COMMAND} --output-on-failure
  )
  if(ARG_FAIL_REGEX)
    set_tests_properties(${name} PROPERTIES PASS_REGULAR_EXPRESSION "${ARG_FAIL_REGEX}")
  endif()
endfunction()

crubit_add_fixture(configure_ok)
crubit_add_fixture(not_libcxx LINUX_ONLY FAIL_REGEX "Crubit requires libc\\+\\+")

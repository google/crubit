# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

cmake_minimum_required(VERSION 3.22)
project(configure_ok LANGUAGES CXX)
include(${CRUBIT_SOURCE_DIR}/cmake/test/fixture_preamble.cmake)

foreach(command IN ITEMS crubit_rust_bindings crubit_link_rust_bindings)
  if(NOT COMMAND ${command})
    message(FATAL_ERROR "${command}() is not defined")
  endif()
endforeach()

add_executable(main main.cc)
add_test(NAME main COMMAND main)

# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# _crubit_clang_args() turns a target's usage requirements into clang args,
# evaluating generator expressions.
cmake_minimum_required(VERSION 3.22)
if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Debug)
endif()
project(clang_args LANGUAGES CXX)
include(${CRUBIT_SOURCE_DIR}/cmake/test/fixture_preamble.cmake)
_crubit_init_toolchain()

add_library(dep INTERFACE)
target_compile_definitions(dep INTERFACE FIXTURE_FROM_DEP)

add_library(lib STATIC lib.cc)
target_include_directories(lib PUBLIC $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/inc>)
target_compile_definitions(lib PUBLIC $<$<CONFIG:Debug>:FIXTURE_DEBUG> FIXTURE_ALWAYS=1)
target_link_libraries(lib PUBLIC dep)

_crubit_clang_args(args lib)
file(GENERATE OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/args.rsp" CONTENT "$<JOIN:${args},\n>\n")

add_test(NAME check_args COMMAND ${CMAKE_COMMAND}
  -DRSP=${CMAKE_CURRENT_BINARY_DIR}/args.rsp
  -DINC=${CMAKE_CURRENT_SOURCE_DIR}/inc
  -DCONFIG=$<CONFIG>
  -P ${CMAKE_CURRENT_SOURCE_DIR}/check_args.cmake)

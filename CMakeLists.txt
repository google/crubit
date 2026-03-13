# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

cmake_minimum_required(VERSION 3.16)

project(crubit LANGUAGES CXX)

include(FetchContent)

FetchContent_Declare(
  absl
  GIT_REPOSITORY https://github.com/abseil/abseil-cpp.git
  FIND_PACKAGE_ARGS NAMES absl
)

FetchContent_MakeAvailable(absl)

add_subdirectory(support/rs_std)
add_subdirectory(support/rs_std/internal)
add_subdirectory(support/internal)

get_property(ALL_TARGETS DIRECTORY . PROPERTY BUILDSYSTEM_TARGETS)

add_library(crubit_support INTERFACE)
set_target_properties(crubit_support PROPERTIES LINKER_LANGUAGE CXX)
target_link_libraries(crubit_support ${ALL_TARGETS})

# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

cmake_minimum_required(VERSION 3.16)

project(crubit LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(FetchContent)

FetchContent_Declare(
  absl
  GIT_REPOSITORY https://github.com/abseil/abseil-cpp.git
  FIND_PACKAGE_ARGS NAMES absl
)

FetchContent_MakeAvailable(absl)

include_directories(${CMAKE_SOURCE_DIR})

set(CRUBIT_SUBDIRS
  support
  support/internal
  support/rs_std
  support/rs_std/internal
)

set(ALL_TARGETS "")
foreach(subdir IN LISTS CRUBIT_SUBDIRS)
  add_subdirectory(${subdir})
  get_property(subdir_targets DIRECTORY ${subdir} PROPERTY BUILDSYSTEM_TARGETS)
  list(APPEND ALL_TARGETS ${subdir_targets})
endforeach()

add_library(crubit_support INTERFACE)
set_target_properties(crubit_support PROPERTIES LINKER_LANGUAGE CXX)
target_link_libraries(crubit_support INTERFACE ${ALL_TARGETS})
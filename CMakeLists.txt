# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

cmake_minimum_required(VERSION 3.16)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(crubit LANGUAGES CXX)

if(CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wno-nullability-completeness)
endif()

include(FetchContent)

FetchContent_Declare(
  absl
  GIT_REPOSITORY https://github.com/abseil/abseil-cpp.git
  FIND_PACKAGE_ARGS NAMES absl
)

FetchContent_MakeAvailable(absl)

find_package(Protobuf QUIET)
if(NOT Protobuf_FOUND)
  set(protobuf_BUILD_TESTS OFF CACHE BOOL "" FORCE)
  set(protobuf_INSTALL OFF CACHE BOOL "" FORCE)
  set(protobuf_BUILD_PROTOC_BINARIES OFF CACHE BOOL "" FORCE)
  set(protobuf_BUILD_LIBPROTOC OFF CACHE BOOL "" FORCE)
  FetchContent_Declare(
    Protobuf
    GIT_REPOSITORY https://github.com/protocolbuffers/protobuf.git
    GIT_TAG main
    FIND_PACKAGE_ARGS NAMES Protobuf
  )
  FetchContent_GetProperties(Protobuf)
  if(NOT protobuf_POPULATED)
    FetchContent_Populate(Protobuf)
    add_subdirectory(${protobuf_SOURCE_DIR} ${protobuf_BINARY_DIR} EXCLUDE_FROM_ALL)
  endif()
endif()

set(CRUBIT_SUBDIRS
  support
  support/internal
  support/rs_std
  support/rs_std/internal
)

include_directories(${CMAKE_CURRENT_SOURCE_DIR})

set(ALL_TARGETS "")
foreach(subdir IN LISTS CRUBIT_SUBDIRS)
  add_subdirectory(${subdir})
  get_property(subdir_targets DIRECTORY ${subdir} PROPERTY BUILDSYSTEM_TARGETS)
  list(APPEND ALL_TARGETS ${subdir_targets})
endforeach()

add_library(crubit_support INTERFACE)
set_target_properties(crubit_support PROPERTIES LINKER_LANGUAGE CXX)
target_link_libraries(crubit_support INTERFACE ${ALL_TARGETS})
target_include_directories(crubit_support INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})
target_compile_options(crubit_support INTERFACE
  $<$<COMPILE_LANG_AND_ID:CXX,Clang,AppleClang>:-Wno-nullability-completeness>
)
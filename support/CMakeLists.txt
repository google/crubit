# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Automatically @generated CMakeLists.txt.

add_library(crubit_support_annotate_internal INTERFACE
"annotate_internal.h"
)
set_target_properties(crubit_support_annotate_internal PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_annotate_internal INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/.."
)

add_library(crubit_support_annotations INTERFACE
"annotations.h"
)
set_target_properties(crubit_support_annotations PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_annotations INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/.."
)

target_link_libraries(crubit_support_annotations INTERFACE
crubit_support_annotations_internal
crubit_support_bitwise_relocatability_annotations
)

add_library(crubit_support_annotations_internal INTERFACE
"annotations_internal.h"
)
set_target_properties(crubit_support_annotations_internal PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_annotations_internal INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/.."
)

target_link_libraries(crubit_support_annotations_internal INTERFACE
crubit_support_annotate_internal
)

add_library(crubit_support_bitwise_relocatability_annotations INTERFACE
"bitwise_relocatability_annotations.h"
)
set_target_properties(crubit_support_bitwise_relocatability_annotations PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_bitwise_relocatability_annotations INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/.."
)

target_link_libraries(crubit_support_bitwise_relocatability_annotations INTERFACE
crubit_support_annotate_internal
)

add_library(crubit_support_bridge_cpp INTERFACE
"bridge.h"
)
set_target_properties(crubit_support_bridge_cpp PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_bridge_cpp INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/.."
)

add_library(crubit_support_lifetime_annotations INTERFACE
"lifetime_annotations.h"
)
set_target_properties(crubit_support_lifetime_annotations PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_lifetime_annotations INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/.."
)

add_library(crubit_support_movable INTERFACE
"movable.h"
)
set_target_properties(crubit_support_movable PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_movable INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/.."
)

target_link_libraries(crubit_support_movable INTERFACE
crubit_support_unsafe_relocate_tag
crubit_support_internal_bindings_support
)

add_library(crubit_support_unsafe_relocate_tag INTERFACE
"unsafe_relocate_tag.h"
)
set_target_properties(crubit_support_unsafe_relocate_tag PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_unsafe_relocate_tag INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/.."
)


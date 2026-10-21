# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Automatically @generated CMakeLists.txt.

add_library(crubit_support_rs_box INTERFACE
"box.h"
)
set_target_properties(crubit_support_rs_box PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_box INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_box INTERFACE
crubit_support_annotations_internal
crubit_support_movable
crubit_support_unsafe_relocate_tag
crubit_support_internal_bindings_support
)

add_library(crubit_support_rs_pin_box INTERFACE
"pin_box.h"
)
set_target_properties(crubit_support_rs_pin_box PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_pin_box INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_pin_box INTERFACE
crubit_support_annotations_internal
crubit_support_movable
crubit_support_unsafe_relocate_tag
crubit_support_internal_bindings_support
)

add_library(crubit_support_rs_slice_box INTERFACE
"slice_box.h"
)
set_target_properties(crubit_support_rs_slice_box PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_slice_box INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_slice_box INTERFACE
crubit_support_annotations_internal
crubit_support_unsafe_relocate_tag
crubit_support_internal_bindings_support
crubit_support_rs_std_slice_ref
)

add_library(crubit_support_rs_str_box INTERFACE
"str_box.h"
)
set_target_properties(crubit_support_rs_str_box PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_str_box INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_str_box INTERFACE
crubit_support_annotations_internal
crubit_support_movable
crubit_support_unsafe_relocate_tag
crubit_support_internal_bindings_support
crubit_support_rs_std_str_ref
crubit_support_rs_std_internal_is_utf8
)


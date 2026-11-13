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
crubit_support_annotations
crubit_support_annotations_internal
crubit_support_movable
crubit_support_unsafe_relocate_tag
crubit_support_internal_bindings_support
)

add_library(crubit_support_rs_char STATIC
"../rs_std/char.cc"
"char.h"
"../rs_std/char.h"
)
set_target_properties(crubit_support_rs_char PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_char PUBLIC
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_char PUBLIC
crubit_support_rs_str_ref
crubit_support_annotations
crubit_support_internal_bindings_support
crubit_support_internal_cpp_config
)

add_library(crubit_support_rs_dyn_callable INTERFACE
"dyn_callable.h"
"../rs_std/dyn_callable.h"
)
set_target_properties(crubit_support_rs_dyn_callable PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_dyn_callable INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_dyn_callable INTERFACE
absl::nullability
absl::any_invocable
crubit_support_bridge_cpp
crubit_support_internal_cpp_config
)

add_library(crubit_support_rs_dyn_erased_future_cc INTERFACE
"dyn_erased_future.h"
"../rs_std/dyn_erased_future.h"
)
set_target_properties(crubit_support_rs_dyn_erased_future_cc PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_dyn_erased_future_cc INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_dyn_erased_future_cc INTERFACE
crubit_support_rs_waker
)

add_library(crubit_support_rs_enum INTERFACE
"enum.h"
"../rs_std/enum.h"
)
set_target_properties(crubit_support_rs_enum PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_enum INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_enum INTERFACE
crubit_support_annotations
crubit_support_internal_bindings_support
crubit_support_rs_std_internal_enum
)

add_library(crubit_support_rs_int INTERFACE
"int.h"
"../rs_std/int.h"
)
set_target_properties(crubit_support_rs_int PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_int INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_int INTERFACE
crubit_support_annotations
crubit_support_internal_cpp_config
)

add_library(crubit_support_rs_iterator_adapter INTERFACE
"iterator_adapter.h"
"../rs_std/iterator_adapter.h"
)
set_target_properties(crubit_support_rs_iterator_adapter PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_iterator_adapter INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_iterator_adapter INTERFACE
crubit_support_rs_traits
crubit_support_internal_cpp_config
)

add_library(crubit_support_rs_lossy_formatter_for_bindings INTERFACE
"lossy_formatter_for_bindings.h"
"../rs_std/lossy_formatter_for_bindings.h"
)
set_target_properties(crubit_support_rs_lossy_formatter_for_bindings PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_lossy_formatter_for_bindings INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

add_library(crubit_support_rs_never INTERFACE
"never.h"
"../rs_std/never.h"
)
set_target_properties(crubit_support_rs_never PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_never INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_never INTERFACE
crubit_support_annotations_internal
crubit_support_internal_cpp_config
)

add_library(crubit_support_rs_option INTERFACE
"option.h"
"../rs_std/option.h"
)
set_target_properties(crubit_support_rs_option PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_option INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_option INTERFACE
crubit_support_rs_never
crubit_support_rs_unit
crubit_support_internal_bindings_support
crubit_support_internal_cpp_config
)

add_library(crubit_support_rs_pin_box INTERFACE
"pin_box.h"
)
set_target_properties(crubit_support_rs_pin_box PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_pin_box INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_pin_box INTERFACE
crubit_support_annotations
crubit_support_annotations_internal
crubit_support_movable
crubit_support_unsafe_relocate_tag
crubit_support_internal_bindings_support
)

add_library(crubit_support_rs_result INTERFACE
"result.h"
"../rs_std/result.h"
)
set_target_properties(crubit_support_rs_result PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_result INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_result INTERFACE
crubit_support_rs_never
crubit_support_rs_unit
crubit_support_internal_bindings_support
crubit_support_internal_cpp_config
)

add_library(crubit_support_rs_slice_box INTERFACE
"slice_box.h"
)
set_target_properties(crubit_support_rs_slice_box PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_slice_box INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_slice_box INTERFACE
crubit_support_rs_slice_ref
crubit_support_annotations
crubit_support_annotations_internal
crubit_support_unsafe_relocate_tag
crubit_support_internal_bindings_support
)

add_library(crubit_support_rs_slice_ref INTERFACE
"slice_ref.h"
"../rs_std/slice_ref.h"
)
set_target_properties(crubit_support_rs_slice_ref PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_slice_ref INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_slice_ref INTERFACE
crubit_support_annotations
crubit_support_internal_bindings_support
crubit_support_internal_cpp_config
)

add_library(crubit_support_rs_str_box INTERFACE
"str_box.h"
)
set_target_properties(crubit_support_rs_str_box PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_str_box INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_str_box INTERFACE
crubit_support_rs_str_ref
crubit_support_annotations
crubit_support_annotations_internal
crubit_support_movable
crubit_support_unsafe_relocate_tag
crubit_support_internal_bindings_support
crubit_support_rs_std_internal_is_utf8
)

add_library(crubit_support_rs_str_ref INTERFACE
"str_ref.h"
"../rs_std/str_ref.h"
)
set_target_properties(crubit_support_rs_str_ref PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_str_ref INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_str_ref INTERFACE
crubit_support_rs_slice_ref
crubit_support_annotations_internal
crubit_support_internal_bindings_support
crubit_support_internal_cpp_config
crubit_support_rs_std_internal_is_utf8
)

add_library(crubit_support_rs_traits INTERFACE
"traits.h"
"../rs_std/traits.h"
)
set_target_properties(crubit_support_rs_traits PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_traits INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_traits INTERFACE
crubit_support_internal_cpp_config
)

add_library(crubit_support_rs_tuple INTERFACE
"tuple.h"
"../rs_std/tuple.h"
)
set_target_properties(crubit_support_rs_tuple PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_tuple INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_tuple INTERFACE
crubit_support_internal_cpp_config
)

add_library(crubit_support_rs_unit INTERFACE
"unit.h"
"../rs_std/unit.h"
)
set_target_properties(crubit_support_rs_unit PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_unit INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_unit INTERFACE
crubit_support_internal_cpp_config
)

add_library(crubit_support_rs_vec INTERFACE
"vec.h"
"../rs_std/vec.h"
)
set_target_properties(crubit_support_rs_vec PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_vec INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_vec INTERFACE
crubit_support_internal_bindings_support
crubit_support_internal_cpp_config
)

add_library(crubit_support_rs_waker STATIC
"../rs_std/waker.cc"
"waker.h"
"../rs_std/waker.h"
)
set_target_properties(crubit_support_rs_waker PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_waker PUBLIC
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_waker PUBLIC
crubit_support_internal_cpp_config
)


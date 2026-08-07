# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Automatically @generated CMakeLists.txt.

add_library(crubit_support_rs_std_char STATIC
"char.cc"
"char.h"
)
set_target_properties(crubit_support_rs_std_char PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_std_char PUBLIC
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_std_char PUBLIC
crubit_support_rs_std_str_ref
crubit_support_annotations
crubit_support_internal_bindings_support
)

add_library(crubit_support_rs_std_dyn_callable INTERFACE
"dyn_callable.h"
)
set_target_properties(crubit_support_rs_std_dyn_callable PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_std_dyn_callable INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_std_dyn_callable INTERFACE
absl::core_headers
absl::nullability
absl::any_invocable
crubit_support_bridge_cpp
)

add_library(crubit_support_rs_std_dyn_erased_future_cc INTERFACE
"dyn_erased_future.h"
)
set_target_properties(crubit_support_rs_std_dyn_erased_future_cc PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_std_dyn_erased_future_cc INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_std_dyn_erased_future_cc INTERFACE
crubit_support_rs_std_waker
)

add_library(crubit_support_rs_std_int INTERFACE
"int.h"
)
set_target_properties(crubit_support_rs_std_int PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_std_int INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_std_int INTERFACE
crubit_support_annotations
)

add_library(crubit_support_rs_std_iterator_adapter INTERFACE
"iterator_adapter.h"
)
set_target_properties(crubit_support_rs_std_iterator_adapter PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_std_iterator_adapter INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_std_iterator_adapter INTERFACE
crubit_support_rs_std_traits
)

add_library(crubit_support_rs_std_lossy_formatter_for_bindings INTERFACE
"lossy_formatter_for_bindings.h"
)
set_target_properties(crubit_support_rs_std_lossy_formatter_for_bindings PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_std_lossy_formatter_for_bindings INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

add_library(crubit_support_rs_std_option INTERFACE
"option.h"
)
set_target_properties(crubit_support_rs_std_option PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_std_option INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_std_option INTERFACE
crubit_support_internal_bindings_support
)

add_library(crubit_support_rs_std_result INTERFACE
"result.h"
)
set_target_properties(crubit_support_rs_std_result PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_std_result INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_std_result INTERFACE
crubit_support_internal_bindings_support
)

add_library(crubit_support_rs_std_slice_ref INTERFACE
"slice_ref.h"
)
set_target_properties(crubit_support_rs_std_slice_ref PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_std_slice_ref INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_std_slice_ref INTERFACE
crubit_support_annotations
crubit_support_internal_bindings_support
)

add_library(crubit_support_rs_std_str_ref INTERFACE
"str_ref.h"
)
set_target_properties(crubit_support_rs_std_str_ref PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_std_str_ref INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_std_str_ref INTERFACE
crubit_support_rs_std_slice_ref
crubit_support_annotations_internal
crubit_support_internal_bindings_support
crubit_support_rs_std_internal_is_utf8
)

add_library(crubit_support_rs_std_traits INTERFACE
"traits.h"
)
set_target_properties(crubit_support_rs_std_traits PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_std_traits INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

add_library(crubit_support_rs_std_tuple INTERFACE
"tuple.h"
)
set_target_properties(crubit_support_rs_std_tuple PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_std_tuple INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

add_library(crubit_support_rs_std_vec INTERFACE
"vec.h"
)
set_target_properties(crubit_support_rs_std_vec PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_std_vec INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_std_vec INTERFACE
crubit_support_internal_bindings_support
)

add_library(crubit_support_rs_std_waker STATIC
"waker.cc"
"waker.h"
)
set_target_properties(crubit_support_rs_std_waker PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_std_waker PUBLIC
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)


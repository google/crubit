# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Automatically @generated CMakeLists.txt.

add_library(crubit_support_rs_char INTERFACE
"char.h"
)
set_target_properties(crubit_support_rs_char PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_char INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_char INTERFACE
crubit_support_rs_std_char
)

add_library(crubit_support_rs_dyn_callable INTERFACE
"dyn_callable.h"
)
set_target_properties(crubit_support_rs_dyn_callable PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_dyn_callable INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_dyn_callable INTERFACE
crubit_support_rs_std_dyn_callable
)

add_library(crubit_support_rs_dyn_erased_future_cc INTERFACE
"dyn_erased_future.h"
)
set_target_properties(crubit_support_rs_dyn_erased_future_cc PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_dyn_erased_future_cc INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_dyn_erased_future_cc INTERFACE
crubit_support_rs_std_dyn_erased_future_cc
)

add_library(crubit_support_rs_enum INTERFACE
"enum.h"
)
set_target_properties(crubit_support_rs_enum PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_enum INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_enum INTERFACE
crubit_support_rs_std_enum
)

add_library(crubit_support_rs_int INTERFACE
"int.h"
)
set_target_properties(crubit_support_rs_int PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_int INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_int INTERFACE
crubit_support_rs_std_int
)

add_library(crubit_support_rs_iterator_adapter INTERFACE
"iterator_adapter.h"
)
set_target_properties(crubit_support_rs_iterator_adapter PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_iterator_adapter INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_iterator_adapter INTERFACE
crubit_support_rs_std_iterator_adapter
)

add_library(crubit_support_rs_lossy_formatter_for_bindings INTERFACE
"lossy_formatter_for_bindings.h"
)
set_target_properties(crubit_support_rs_lossy_formatter_for_bindings PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_lossy_formatter_for_bindings INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_lossy_formatter_for_bindings INTERFACE
crubit_support_rs_std_lossy_formatter_for_bindings
)

add_library(crubit_support_rs_option INTERFACE
"option.h"
)
set_target_properties(crubit_support_rs_option PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_option INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_option INTERFACE
crubit_support_rs_std_option
)

add_library(crubit_support_rs_result INTERFACE
"result.h"
)
set_target_properties(crubit_support_rs_result PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_result INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_result INTERFACE
crubit_support_rs_std_result
)

add_library(crubit_support_rs_slice_ref INTERFACE
"slice_ref.h"
)
set_target_properties(crubit_support_rs_slice_ref PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_slice_ref INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_slice_ref INTERFACE
crubit_support_rs_std_slice_ref
)

add_library(crubit_support_rs_str_ref INTERFACE
"str_ref.h"
)
set_target_properties(crubit_support_rs_str_ref PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_str_ref INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_str_ref INTERFACE
crubit_support_rs_std_str_ref
)

add_library(crubit_support_rs_traits INTERFACE
"traits.h"
)
set_target_properties(crubit_support_rs_traits PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_traits INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_traits INTERFACE
crubit_support_rs_std_traits
)

add_library(crubit_support_rs_tuple INTERFACE
"tuple.h"
)
set_target_properties(crubit_support_rs_tuple PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_tuple INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_tuple INTERFACE
crubit_support_rs_std_tuple
)

add_library(crubit_support_rs_unit INTERFACE
"unit.h"
)
set_target_properties(crubit_support_rs_unit PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_unit INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_unit INTERFACE
crubit_support_rs_std_unit
)

add_library(crubit_support_rs_vec INTERFACE
"vec.h"
)
set_target_properties(crubit_support_rs_vec PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_vec INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_vec INTERFACE
crubit_support_rs_std_vec
)

add_library(crubit_support_rs_waker INTERFACE
"waker.h"
)
set_target_properties(crubit_support_rs_waker PROPERTIES LINKER_LANGUAGE CXX)

target_include_directories(crubit_support_rs_waker INTERFACE
  "${CMAKE_CURRENT_SOURCE_DIR}/../.."
)

target_link_libraries(crubit_support_rs_waker INTERFACE
crubit_support_rs_std_waker
)


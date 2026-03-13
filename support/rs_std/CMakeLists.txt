# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Automatically @generated CMakeLists.txt.

add_library(crubit_support_rs_std_char STATIC
"char.cc"
"char.h"
)
set_target_properties(crubit_support_rs_std_char PROPERTIES LINKER_LANGUAGE CXX)

target_link_libraries(crubit_support_rs_std_char PUBLIC
crubit_support_rs_std_str_ref
absl::core_headers
absl::check
absl::string_view
absl::span
crubit_support_annotations
)

add_library(crubit_support_rs_std_slice_ref STATIC
"slice_ref.h"
)
set_target_properties(crubit_support_rs_std_slice_ref PROPERTIES LINKER_LANGUAGE CXX)

target_link_libraries(crubit_support_rs_std_slice_ref PUBLIC
absl::core_headers
absl::span
crubit_support_annotations_internal
crubit_support_internal_bindings_support
)

add_library(crubit_support_rs_std_str_ref STATIC
"str_ref.h"
)
set_target_properties(crubit_support_rs_std_str_ref PROPERTIES LINKER_LANGUAGE CXX)

target_link_libraries(crubit_support_rs_std_str_ref PUBLIC
crubit_support_rs_std_slice_ref
absl::core_headers
absl::nullability
absl::string_view
crubit_support_annotations_internal
crubit_support_internal_bindings_support
crubit_support_rs_std_internal_is_utf8
)

add_library(crubit_support_rs_std_dyn_callable STATIC
"dyn_callable.h"
)
set_target_properties(crubit_support_rs_std_dyn_callable PROPERTIES LINKER_LANGUAGE CXX)

target_link_libraries(crubit_support_rs_std_dyn_callable PUBLIC
absl::core_headers
absl::nullability
absl::any_invocable
crubit_support_bridge_cpp
)

add_library(crubit_support_rs_std_traits STATIC
"traits.h"
)
set_target_properties(crubit_support_rs_std_traits PROPERTIES LINKER_LANGUAGE CXX)

add_library(crubit_support_rs_std_option STATIC
"option.h"
)
set_target_properties(crubit_support_rs_std_option PROPERTIES LINKER_LANGUAGE CXX)

add_library(crubit_support_rs_std_result STATIC
"result.h"
)
set_target_properties(crubit_support_rs_std_result PROPERTIES LINKER_LANGUAGE CXX)

add_library(crubit_support_rs_std_lossy_formatter_for_bindings STATIC
"lossy_formatter_for_bindings.h"
)
set_target_properties(crubit_support_rs_std_lossy_formatter_for_bindings PROPERTIES LINKER_LANGUAGE CXX)

target_link_libraries(crubit_support_rs_std_lossy_formatter_for_bindings PUBLIC
crubit_support_rs_std_lossy_formatter_cc
)


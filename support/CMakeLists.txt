# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Automatically @generated CMakeLists.txt.

add_library(crubit_support_annotations STATIC
"annotations.h"
)
set_target_properties(crubit_support_annotations PROPERTIES LINKER_LANGUAGE CXX)

target_link_libraries(crubit_support_annotations PUBLIC
crubit_support_annotations_internal
)

add_library(crubit_support_annotations_internal STATIC
"annotations_internal.h"
)
set_target_properties(crubit_support_annotations_internal PROPERTIES LINKER_LANGUAGE CXX)

target_link_libraries(crubit_support_annotations_internal PUBLIC
absl::core_headers
absl::nullability
)

add_library(crubit_support_bridge_cpp STATIC
"bridge.h"
)
set_target_properties(crubit_support_bridge_cpp PROPERTIES LINKER_LANGUAGE CXX)

add_library(crubit_support_lifetime_annotations STATIC
"lifetime_annotations.h"
)
set_target_properties(crubit_support_lifetime_annotations PROPERTIES LINKER_LANGUAGE CXX)


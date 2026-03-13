# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Automatically @generated CMakeLists.txt.

add_library(crubit_support_rs_std_internal_is_utf8 STATIC
"is_utf8.h"
)
set_target_properties(crubit_support_rs_std_internal_is_utf8 PROPERTIES LINKER_LANGUAGE CXX)

target_link_libraries(crubit_support_rs_std_internal_is_utf8 PUBLIC
absl::core_headers
absl::string_view
)


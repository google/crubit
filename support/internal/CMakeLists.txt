# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Automatically @generated CMakeLists.txt.

add_library(crubit_support_internal_bindings_support STATIC
"check_no_mutable_aliasing.cc"
"check_no_mutable_aliasing.h"
"cxx20_backports.h"
"fmt.h"
"lazy_init.h"
"memswap.h"
"offsetof.h"
"sizeof.h"
"slot.h"
)
set_target_properties(crubit_support_internal_bindings_support PROPERTIES LINKER_LANGUAGE CXX)

target_link_libraries(crubit_support_internal_bindings_support PUBLIC
absl::core_headers
absl::nullability
absl::check
absl::strings
absl::has_ostream_operator
absl::span
)


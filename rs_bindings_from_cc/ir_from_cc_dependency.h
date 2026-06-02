// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IR_FROM_CC_DEPENDENCY_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IR_FROM_CC_DEPENDENCY_H_

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "common/ffi_types.h"
#include "rs_bindings_from_cc/ir.h"

namespace crubit {

// LINT.IfChange
inline constexpr absl::string_view kDependencyTarget = "//test:dependency";

inline constexpr absl::string_view kDependencyHeaderName =
    "test/dependency_header.h";
// LINT.ThenChange(//depot/rs_bindings_from_cc/ir_testing.rs)

absl::StatusOr<IR> IrFromCcDependency(FfiU8Slice target_triple,
                                      FfiU8Slice header_source,
                                      FfiU8Slice dependency_header_source,
                                      FfiU8Slice extra_feature,
                                      bool kythe_annotations);

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_FROM_CC_DEPENDENCY_H_

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <string>

#include "absl/status/statusor.h"
#include "common/ffi_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_from_cc_dependency.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/Support/FormatVariadic.h"

namespace crubit {

// This is intended to be called from Rust tests.
extern "C" FfiU8SliceBox proto_from_cc_dependency(
    FfiU8Slice target_triple, FfiU8Slice header_source,
    FfiU8Slice dependency_header_source, FfiU8Slice extra_feature,
    bool kythe_annotations) {
  absl::StatusOr<IR> ir =
      IrFromCcDependency(target_triple, header_source, dependency_header_source,
                         extra_feature, kythe_annotations);

  if (!ir.ok()) {
    llvm::report_fatal_error(llvm::formatv("IrFromCc reported an error: {0}",
                                           ir.status().message()));
  }
  std::string proto = ir->ToFlatProto().SerializeAsString();
  return AllocFfiU8SliceBox(MakeFfiU8Slice(proto));
}

}  // namespace crubit

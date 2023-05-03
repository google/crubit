// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_INFERENCE_POINTER_MODEL_H_
#define CRUBIT_NULLABILITY_INFERENCE_POINTER_MODEL_H_

#include <optional>

#include "absl/container/flat_hash_set.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/SourceLocation.h"

namespace clang {
namespace tidy {
namespace nullability {

// Models a single pointer value with respect to inferred nullability.
struct PointerModel {
  // The first location where the pointer is unconditionally dereferenced.
  std::optional<clang::SourceLocation> UnconditionalDereference;

  // The set of flow conditions under which this pointer was dereferenced.
  absl::flat_hash_set<clang::dataflow::AtomicBoolValue*> DerefFlowConditions;
};

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_INFERENCE_POINTER_MODEL_H_

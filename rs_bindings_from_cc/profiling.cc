// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/profiling.h"

namespace crubit {
void StartHeapProfiling(absl::string_view prefix) {}
void DumpHeapProfiling(absl::string_view prefix) {}
void StopHeapProfiling() {}
}  // namespace crubit

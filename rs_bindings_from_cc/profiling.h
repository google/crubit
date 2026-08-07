// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_PROFILING_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_PROFILING_H_

#include "absl/strings/string_view.h"

#ifdef CRUBIT_HEAP_PROFILING
#define CRUBIT_PROFILE_NOINLINE __attribute__((noinline))
#else
#define CRUBIT_PROFILE_NOINLINE
#endif

namespace crubit {

void StartHeapProfiling(absl::string_view prefix);
void DumpHeapProfiling(absl::string_view prefix);
void StopHeapProfiling();

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_PROFILING_H_

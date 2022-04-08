// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_UTIL_CHECK_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_UTIL_CHECK_H_

#include "third_party/absl/base/attributes.h"
#include "third_party/absl/base/optimization.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/ErrorHandling.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/FormatVariadic.h"

#define CRUBIT_CHECK(condition)                                           \
  do {                                                                    \
    if (ABSL_PREDICT_FALSE(!(condition))) {                               \
      ::llvm::report_fatal_error(                                         \
          ::llvm::formatv("CRUBIT_CHECK failure: {0}:{1}: {2}", __FILE__, \
                          __LINE__, #condition));                         \
    }                                                                     \
  } while (false)

namespace crubit {
template <typename T>
ABSL_MUST_USE_RESULT T DieIfNull(const char* file, int line,
                                 const char* exprtext, T&& t) {
  if (ABSL_PREDICT_FALSE(t == nullptr)) {
    ::llvm::report_fatal_error(llvm::formatv(
        "CRUBIT_DIE_IF_NULL failure: {0}:{1}: {2}", file, line, exprtext));
  }
  return std::forward<T>(t);
}
}  // namespace crubit
#define CRUBIT_DIE_IF_NULL(value) \
  ::crubit::DieIfNull(__FILE__, __LINE__, #value, (value))

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_UTIL_CHECK_H_

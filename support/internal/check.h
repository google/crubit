// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_CHECK_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_CHECK_H_

#include <cstdio>
#include <cstdlib>

namespace crubit::internal {

struct CheckFail {
  CheckFail(const char* file, int line, const char* cond) {
    std::fprintf(stderr, "%s:%d: Check failed: %s ", file, line, cond);
  }
  ~CheckFail() {
    std::fprintf(stderr, "\n");
    std::abort();
  }
  CheckFail& operator<<(const char* msg) {
    std::fprintf(stderr, "%s", msg);
    return *this;
  }
};

}  // namespace crubit::internal

#define CRUBIT_CHECK(condition)  \
  if (!(condition)) [[unlikely]] \
  ::crubit::internal::CheckFail(__FILE__, __LINE__, #condition)

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_CHECK_H_

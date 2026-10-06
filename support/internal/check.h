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

// Like `CRUBIT_CHECK`, but only checked when `NDEBUG` is not defined. With
// `NDEBUG`, the condition and any streamed message are still compiled (so they
// can't silently rot), but never evaluated.
#ifndef NDEBUG
#define CRUBIT_DCHECK(condition) CRUBIT_CHECK(condition)
#else
#define CRUBIT_DCHECK(condition) \
  if (true) {                    \
  } else                         \
    CRUBIT_CHECK(condition)
#endif

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_CHECK_H_

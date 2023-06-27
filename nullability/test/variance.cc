// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for correct treatment of type variance.

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

TEST(PointerNullabilityTest, NonConstPointerIsInvariant) {
  // TODO(b/275458593): This test demonstrates a bug in the checker. The call
  // `target(pp)` should be flagged as an error because non-const pointers are
  // invariant over their pointee type and we should therefore not allow `int *
  // _Nonnull * _Nonnull` to be converted to `int * _Nullable * _Nonnull`.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void callee(int *_Nullable *_Nonnull pp);
    void target(int *_Nonnull *_Nonnull pp) { target(pp); }
  )cc"));
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang

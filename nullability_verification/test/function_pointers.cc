// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for nullability of function pointers.

#include "nullability_verification/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

TEST(PointerNullabilityTest, FunctionToPointerDecayIsNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      // Use `static_cast` to force function-to-pointer decay.
      __assert_nullability<NK_nonnull>(static_cast<void (*)()>(target));
    }
  )cc"));
}

TEST(PointerNullabilityTest, CallExplicitlyDereferencedDirectCallee) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void callee();
    void target() { (*callee)(); }
  )cc"));
}

TEST(PointerNullabilityTest, AnnotationsInReturnType) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable target() {
      // Use `static_cast` to force function-to-pointer decay.
      __assert_nullability<NK_nonnull, NK_nullable>(
          static_cast<int* (*)()>(target));
      return nullptr;
    }
  )cc"));
}

TEST(PointerNullabilityTest, AnnotationsInParameters) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable) {
      // Use `static_cast` to force function-to-pointer decay.
      __assert_nullability<NK_nonnull, NK_nullable>(
          static_cast<void (*)(int *)>(target));
    }
  )cc"));
}

TEST(PointerNullabilityTest, NonnullCallback) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(void (*_Nonnull callback)()) {
      // Both an explicit dereference and an implicit dereference done by a
      // function call should be allowed.
      (*callback)();
      callback();
    }
  )cc"));
}

TEST(PointerNullabilityTest, NullableCallback) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(void (*_Nullable callback)()) {
      // Both an explicit dereference and an implicit dereference done by a
      // function call should be marked as unsafe.
      (*callback)();  // [[unsafe]]
      callback();     // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, NonnullCallbackWithoutCalleeDecl) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    using NonnullCallbackType = void (*_Nonnull)();
    NonnullCallbackType getCallback();
    void target() {
      __assert_nullability<NK_nonnull>(getCallback());
      (*getCallback())();
      getCallback()();
    }
  )cc"));
}

TEST(PointerNullabilityTest, NullableCallbackWithoutCalleeDecl) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    using NullableCallbackType = void (*_Nullable)();
    NullableCallbackType getCallback();
    void target(bool b) {
      __assert_nullability<NK_nullable>(getCallback());
      (*getCallback())();  // [[unsafe]]
      getCallback()();     // [[unsafe]]
    }
  )cc"));
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for diagnostics that flag inconsistent nullability annotations.

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(ConsistentAnnotations, ConsistentParameter) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(Nonnull<int *> p);
    void target(Nonnull<int *> p) { *p; }
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentParameter) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(Nullable<int *> p);
    void target(Nonnull<int *> p) {  // [[unsafe]]
      *p;
    }
  )cc"));
}

TEST(ConsistentAnnotations, ConsistentDoublePointer) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(Nullable<Nonnull<int *> *> p);
    void target(Nullable<Nonnull<int *> *> p) {}
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentOuterPointer) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(Nullable<Nonnull<int *> *> p);
    void target(Nonnull<Nonnull<int *> *> p) {  // [[unsafe]]
    }
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentInnerPointer) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(Nullable<Nonnull<int *> *> p);
    void target(Nullable<Nullable<int *> *> p) {  // [[unsafe]]
    }
  )cc"));
}

TEST(ConsistentAnnotations, ConsistentReturnType) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    Nullable<int *> target();
    Nullable<int *> target() { return nullptr; }
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentReturnType) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    Nonnull<int *> target();
    Nullable<int *> target() {  // [[unsafe]]
      return nullptr;
    }
  )cc"));
}

TEST(ConsistentAnnotations, ConsistentSmartPointer) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target(Nonnull<std::unique_ptr<int>> p);
    void target(Nonnull<std::unique_ptr<int>> p) { *p; }
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentSmartPointer) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target(Nullable<std::unique_ptr<int>> p);
    void target(Nonnull<std::unique_ptr<int>> p) {  // [[unsafe]]
      *p;
    }
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentWithPragma) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#pragma nullability file_default nullable
    void target(int *p);
    void target(Nonnull<int *> p) {  // [[unsafe]]
      *p;
    }
  )cc"));
}

TEST(ConsistentAnnotations, ConsistentGlobal) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    extern Nonnull<int *> target;
    Nonnull<int *> target = new int;
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentGlobal) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    extern Nonnull<int *> target;
    // Annotation has to go within the declaration to be picked up.
    Nullable<int *> target /* [[unsafe]] */ = nullptr;
  )cc"));
}

TEST(ConsistentAnnotations, ConsistentStaticMemberVariable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      static Nonnull<int *> target;
    };
    Nonnull<int *> S::target = new int;
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentStaticMemberVariable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      static Nonnull<int *> target;
    };
    // Annotation has to go within the declaration to be picked up.
    Nullable<int *> S::target /* [[unsafe]] */ = nullptr;
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability

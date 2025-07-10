// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for diagnostics that flag inconsistent nullability annotations.

#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(ConsistentAnnotations, ConsistentParameter) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull p);
    void target(int *_Nonnull p) { *p; }
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentParameter1) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable p);
    void target(int *_Nonnull p) {  // [[unsafe]]
      *p;
    }
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentParameter2) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* p);
    void target(int* _Nonnull p) {  // [[unsafe]]
      *p;
    }
  )cc"));
}

TEST(ConsistentAnnotations, ConsistentParameterAcrossFormats) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#define nonnull_macro _Nonnull
    template <typename T>
    using NonnullAlias = _Nonnull T;
    void target(int *nonnull_macro p);
    void target(NonnullAlias<int *> p);
    void target(int *_Nonnull p) { *p; }
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentParameterAcrossFormats) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#define nonnull_macro _Nonnull
    template <typename T>
    using NullableAlias = _Nullable T;
    void target(int *nonnull_macro p);
    // Note: diagnostic marker must be within `target`s source range.
    void target(NullableAlias<int *> p /* [[unsafe]] */);
    void target(int *_Nonnull p) { *p; }
  )cc"));
}

TEST(ConsistentAnnotations, ConsistentDoublePointer) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull *_Nullable p);
    void target(int *_Nonnull *_Nullable p) {}
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentOuterPointer) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull *_Nullable p);
    void target(int *_Nonnull *_Nonnull p) {  // [[unsafe]]
    }
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentInnerPointer) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull *_Nullable p);
    void target(int *_Nullable *_Nullable p) {  // [[unsafe]]
    }
  )cc"));
}

TEST(ConsistentAnnotations, ConsistentReturnType) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nullable target();
    int *_Nullable target() { return nullptr; }
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentReturnType1) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull target();
    int *_Nullable target() {  // [[unsafe]]
      return nullptr;
    }
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentReturnType2) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* target();
    int* _Nullable target() {  // [[unsafe]]
      return nullptr;
    }
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentReturnType3) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable target();
    int* target() {  // [[unsafe]]
      return nullptr;
    }
  )cc"));
}

TEST(ConsistentAnnotations, ConsistentSmartPointer) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target(_Nonnull std::unique_ptr<int> p);
    void target(_Nonnull std::unique_ptr<int> p) { *p; }
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentSmartPointer) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target(_Nullable std::unique_ptr<int> p);
    void target(_Nonnull std::unique_ptr<int> p) {  // [[unsafe]]
      *p;
    }
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentWithPragma) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#pragma nullability file_default nullable
    void target(int *p);
    void target(int *_Nonnull p) {  // [[unsafe]]
      *p;
    }
  )cc"));
}

TEST(ConsistentAnnotations, ConsistentGlobal) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    extern int *_Nonnull target;
    int *_Nonnull target = new int;
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentGlobal1) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    extern int *_Nonnull target;
    // Annotation has to go within the declaration to be picked up.
    int *_Nullable target /* [[unsafe]] */ = nullptr;
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentGlobal2) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    extern int* target;
    int* _Nullable target /* [[unsafe]] */ = nullptr;
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentGlobal3) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    extern int* _Nullable target;
    int* target /* [[unsafe]] */ = nullptr;
  )cc"));
}

TEST(ConsistentAnnotations, ConsistentStaticMemberVariable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      static int *_Nonnull target;
    };
    int *_Nonnull S::target = new int;
  )cc"));
}

TEST(ConsistentAnnotations, InconsistentStaticMemberVariable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      static int *_Nonnull target;
    };
    // Annotation has to go within the declaration to be picked up.
    int *_Nullable S::target /* [[unsafe]] */ = nullptr;
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability

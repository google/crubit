// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests that check nullability is transferred correctly across initializers.

#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, InitializerListExpressions) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nonnull p;
    };

    void target() {
      int* _Nullable q = nullptr;
      int x = 1;

      S{};         // [[unsafe]]
      S{nullptr};  // [[unsafe]]
      S{q};        // [[unsafe]]
      S{new int};
      S{&x};
    }
  )cc"));
}

TEST(PointerNullabilityTest, DesignatedInitializerListExpressions) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nonnull p;
    };

    void target() {
      int* _Nullable q = nullptr;
      int x = 1;

      S{.p = nullptr};  // [[unsafe]]
      S{.p = q};  // [[unsafe]]
      S{.p = new int};
      S{.p = &x};
    }
  )cc"));
}

TEST(PointerNullabilityTest, InitializerListDeclarations) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nonnull p;
    };

    void target() {
      int* _Nullable q = nullptr;
      int x = 1;

      S s1{};         // [[unsafe]]
      S s2{nullptr};  // [[unsafe]]
      S s3{q};        // [[unsafe]]
      S s4{new int};
      S s5{&x};
    }
  )cc"));
}

TEST(PointerNullabilityTest, DesignatedInitializerListDeclarations) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nonnull p;
    };

    void target() {
      int* _Nullable q = nullptr;
      int x = 1;

      S s1{.p = nullptr};  // [[unsafe]]
      S s2{.p = q};        // [[unsafe]]
      S s3{.p = new int};
      S s4{.p = &x};
    }
  )cc"));
}

TEST(PointerNullabilityTest, InitializerListDeclarationsWithAssignmentSyntax) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nonnull p;
    };

    void target() {
      S s1 = {};              // [[unsafe]]
      S s2 = {nullptr};       // [[unsafe]]
      S s3 = {.p = nullptr};  // [[unsafe]]

      int* _Nullable q = nullptr;
      S s4 = {q};       // [[unsafe]]
      S s5 = {.p = q};  // [[unsafe]]

      S s6 = {new int};
      S s7 = {.p = new int};

      int x = 1;
      S s8 = {&x};
      S s9 = {.p = &x};
    }
  )cc"));
}

TEST(PointerNullabilityTest, InitializerListDeclarationsWithTwoMembers) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nullable NullableMember;
      int* _Nonnull NonnullMember;
    };

    void target() {
      int* _Nullable q = nullptr;
      int x = 1;

      S s1{};                  // [[unsafe]]
      S s2{nullptr};           // [[unsafe]]
      S s3{new int};           // [[unsafe]]
      S s4{nullptr, nullptr};  // [[unsafe]]
      S s5{new int, nullptr};  // [[unsafe]]
      S s6{nullptr, q};        // [[unsafe]]
      S s7{nullptr, new int};
      S s8{nullptr, &x};
    }
  )cc"));
}

TEST(PointerNullabilityTest,
     DesignatedInitializerListDeclarationsWithTwoMembers) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nullable NullableMember;
      int* _Nonnull NonnullMember;
    };

    void target() {
      int* _Nullable q = nullptr;
      int x = 1;

      S s1{.NullableMember = nullptr}; // [[unsafe]]
      S s2{.NullableMember = new int}; // [[unsafe]]

      S s3{.NonnullMember = nullptr};  // [[unsafe]]
      S s4{.NonnullMember = q};        // [[unsafe]]
      S s5{.NonnullMember = new int};
      S s6{.NonnullMember = &x};

      S s7{.NullableMember = nullptr, .NonnullMember = nullptr};  // [[unsafe]]
      S s8{.NullableMember = new int, .NonnullMember = nullptr};  // [[unsafe]]
      S s9{.NullableMember = nullptr, .NonnullMember = q};        // [[unsafe]]
      S s10{.NullableMember = nullptr, .NonnullMember = new int};
      S s11{.NullableMember = nullptr, .NonnullMember = &x};
    }
  )cc"));
}

// Designated initializers with members out of order are not allowed by default,
// but test them in case the warning is disabled.
// https://clang.llvm.org/docs/DiagnosticsReference.html#wreorder-init-list
TEST(PointerNullabilityTest,
     DesignatedInitializerListDeclarationsWithTwoMembersOutOfOrder) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nullable NullableMember;
      int* _Nonnull NonnullMember;
    };

    void target() {
      int* _Nullable q = nullptr;
      int x = 1;

      S s1{.NonnullMember = nullptr, .NullableMember = nullptr};  // [[unsafe]]
      S s2{.NonnullMember = nullptr, .NullableMember = new int};  // [[unsafe]]
      S s3{.NonnullMember = q, .NullableMember = nullptr};        // [[unsafe]]
      S s4{.NonnullMember = new int, .NullableMember = nullptr};
      S s5{.NonnullMember = &x, .NullableMember = nullptr};
    }
  )cc"));
}

TEST(PointerNullabilityTest, InitListWithDefaultInit) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int X = 1;
      int* _Nonnull NonnullMemberWithNull = nullptr;
      int* _Nonnull NonnullMemberWithNonnull = &X;
    };

    void target() {
      S{2, new int, new int};
      // test when we have fewer initializers than members
      S{2, new int};
      S{2};  // [[unsafe]]
      S{};   // [[unsafe]]

      // test when we override the default
      S{2, new int, nullptr};  // [[unsafe]]
      S{.NonnullMemberWithNull = new int};
      S{.NonnullMemberWithNull = new int,
        .NonnullMemberWithNonnull = nullptr};  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, SmartPointerInitializerList) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct S {
      _Nonnull std::unique_ptr<int> NonnullMember;
      _Nullable std::unique_ptr<int> NullableMember;
      std::unique_ptr<int> UnannotatedMember;
    };

    void target() {
      S{nullptr,  // [[unsafe]]
        nullptr, nullptr};
      S{.NonnullMember = nullptr,  // [[unsafe]]
        .NullableMember = nullptr,
        .UnannotatedMember = nullptr};

      // test when we have fewer initializers than members
      S{new int};
      S{};  // [[unsafe]]
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability

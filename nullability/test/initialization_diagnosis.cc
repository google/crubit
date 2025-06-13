// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests that check nullability is transferred correctly across initializers.

#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, TransitiveNullCheck) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable x) {
      int *y = x;
      *x;  // [[unsafe]]
      if (y) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable x) {
      int *y = x;
      *y;  // [[unsafe]]
      if (x) {
        *y;
      } else {
        *y;  // [[unsafe]]
      }
      *y;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, InitializerList) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nonnull p;
    };

    void target() {
      S{nullptr};       // [[unsafe]]
      S{.p = nullptr};  // [[unsafe]]

      S{new int};
      S{.p = new int};
    }
  )cc"));
}

TEST(PointerNullabilityTest, InitializerListFewerInitsThanMembersValueInit) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nullable NullableMember;
      int* _Nonnull NonnullMember;
    };

    void target() {
      S s1{nullptr, new int};
      S s2{nullptr};  // [[unsafe]]
      S s3{};         // [[unsafe]]

      S s4{.NonnullMember = new int, .NullableMember = nullptr};
      S s5{.NullableMember = nullptr};  // [[unsafe]]
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

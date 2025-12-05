// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for default member initializers for smart pointer class members with
// nullability annotations.

#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest,
     WarnsWhenNonNullStructMemberDefaultInitializerIsNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct S {
      std::unique_ptr<int> _Nonnull target /* [[unsafe]] */ = nullptr;
    };
  )cc"));
}

// TODO: b/376638797 - Warn on initializer for `target` below.
TEST(PointerNullabilityTest,
     WarnsWhenNonNullStructMemberDefaultInitializerIsNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    std::unique_ptr<int> _Nullable f();
    struct S {
      std::unique_ptr<int> _Nonnull target = f();
    };
  )cc"));
}

// TODO: b/376638797 - Warn on initializer for `target` below.
TEST(PointerNullabilityTest,
     WarnsWhenNonNullStructMemberDefaultInitializerIsNullableFromOtherMember) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct S {
      std::unique_ptr<int> _Nullable other = nullptr;
      std::unique_ptr<int> _Nonnull target = std::move(other);
    };
  )cc"));
}

TEST(PointerNullabilityTest,
     NoWarningWhenNullableStructMemberDefaultInitializerIsNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct S {
      std::unique_ptr<int> _Nullable target = nullptr;
    };
  )cc"));
}

TEST(PointerNullabilityTest,
     WarnsWhenNonNullClassMemberDefaultInitializerIsNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    class C {
      std::unique_ptr<int> _Nonnull target /* [[unsafe]] */ = nullptr;
    };
  )cc"));
}

// TODO: b/376638797 - Warn on initializer for `target` below.
TEST(PointerNullabilityTest,
     WarnsWhenNonNullClassMemberDefaultInitializerIsNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    std::unique_ptr<int> _Nullable f();
    class C {
      std::unique_ptr<int> _Nonnull target = f();
    };
  )cc"));
}

// TODO: b/376638797 - Warn on initializer for `target` below.
TEST(PointerNullabilityTest,
     WarnsWhenNonNullClassMemberDefaultInitializerIsNullableFromOtherMember) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    class C {
      std::unique_ptr<int> _Nullable other = nullptr;
      std::unique_ptr<int> _Nonnull target = std::move(other);
    };
  )cc"));
}

TEST(PointerNullabilityTest,
     NoWarningWhenNullableClassMemberDefaultInitializerIsNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    class C {
      std::unique_ptr<int> _Nullable target = nullptr;
    };
  )cc"));
}

TEST(PointerNullabilityTest,
     NoWarningWhenNonNullUnionMemberDefaultInitializerIsNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    union U {
      std::unique_ptr<int> _Nonnull target = nullptr;
    };
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability

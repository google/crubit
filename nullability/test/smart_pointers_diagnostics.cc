// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for diagnostics on smart pointers.

#include "nullability/test/check_diagnostics.h"
#include "nullability/type_nullability.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

// Static initializer turns on support for smart pointers.
test::EnableSmartPointers Enable;

TEST(SmartPointerTest, Dereference) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target() {
      *std::unique_ptr<int>();  // [[unsafe]]
      *std::make_unique<int>();
    }
  )cc"));
}

TEST(SmartPointerTest, ArrowOp) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct S {
      int i = 0;
    };
    void target() {
      std::unique_ptr<S>()->i;  // [[unsafe]]
      std::make_unique<S>()->i;
    }
  )cc"));
}

TEST(SmartPointerTest, Subscript) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target() {
      std::unique_ptr<int[]>()[0];  // [[unsafe]]
      std::make_unique<int[]>(1)[0];
    }
  )cc"));
}

TEST(SmartPointerTest, FunctionParameters) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void TakeNonnull(Nonnull<std::shared_ptr<int>>);
    void TakeNullable(Nullable<std::shared_ptr<int>>);
    void TakeUnannotated(std::shared_ptr<int>);
    void target(Nonnull<std::shared_ptr<int>> NonnullPtr,
                Nullable<std::shared_ptr<int>> NullablePtr,
                std::shared_ptr<int> UnannotatedPtr) {
      TakeNonnull(std::shared_ptr<int>());  // [[unsafe]]
      TakeNonnull(NonnullPtr);
      TakeNonnull(NullablePtr);  // [[unsafe]]
      TakeNonnull(UnannotatedPtr);

      TakeNullable(std::shared_ptr<int>());
      TakeNullable(NonnullPtr);
      TakeNullable(NullablePtr);
      TakeNullable(UnannotatedPtr);

      TakeUnannotated(std::shared_ptr<int>());
      TakeUnannotated(NonnullPtr);
      TakeUnannotated(NullablePtr);
      TakeUnannotated(UnannotatedPtr);
    }
  )cc"));
}

TEST(SmartPointerTest, ConstructorParameters) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct TakeNonnull {
      TakeNonnull(Nonnull<std::shared_ptr<int>>);
    };
    struct TakeNullable {
      TakeNullable(Nullable<std::shared_ptr<int>>);
    };
    struct TakeUnannotated {
      TakeUnannotated(std::shared_ptr<int>);
    };
    void target(Nonnull<std::shared_ptr<int>> NonnullPtr,
                Nullable<std::shared_ptr<int>> NullablePtr,
                std::shared_ptr<int> UnannotatedPtr) {
      TakeNonnull{std::shared_ptr<int>()};  // [[unsafe]]
      TakeNonnull{NonnullPtr};
      TakeNonnull{NullablePtr};  // [[unsafe]]
      TakeNonnull{UnannotatedPtr};

      TakeNullable{std::shared_ptr<int>()};
      TakeNullable{NonnullPtr};
      TakeNullable{NullablePtr};
      TakeNullable{UnannotatedPtr};

      TakeUnannotated{std::shared_ptr<int>()};
      TakeUnannotated{NonnullPtr};
      TakeUnannotated{NullablePtr};
      TakeUnannotated{UnannotatedPtr};
    }
  )cc"));
}

TEST(SmartPointerTest, ReturnValue_Nullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    bool cond();
    Nullable<std::unique_ptr<int>> target() {
      if (cond())
        return std::make_unique<int>(0);
      else
        return std::unique_ptr<int>();
    }
  )cc"));
}

TEST(SmartPointerTest, ReturnValue_Unknown) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    bool cond();
    std::unique_ptr<int> target() {
      if (cond())
        return std::make_unique<int>(0);
      else
        return std::unique_ptr<int>();
    }
  )cc"));
}

TEST(SmartPointerTest, InitializeMemberWithNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct target {
      target(Nonnull<std::shared_ptr<int>> NonnullPtr)
          : NonnullMember(NonnullPtr),
            NullableMember(NonnullPtr),
            UnannotatedMember(NonnullPtr) {}
      Nonnull<std::shared_ptr<int>> NonnullMember;
      Nullable<std::shared_ptr<int>> NullableMember;
      std::shared_ptr<int> UnannotatedMember;
    };
  )cc"));
}

TEST(SmartPointerTest, InitializeMemberWithNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct target {
      target(Nullable<std::shared_ptr<int>> NullablePtr)
          : NonnullMember(NullablePtr),  // [[unsafe]]
            NullableMember(NullablePtr),
            UnannotatedMember(NullablePtr) {}
      Nonnull<std::shared_ptr<int>> NonnullMember;
      Nullable<std::shared_ptr<int>> NullableMember;
      std::shared_ptr<int> UnannotatedMember;
    };
  )cc"));
}

TEST(SmartPointerTest, InitializeMemberWithUnannotated) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct target {
      target(std::shared_ptr<int> UnannotatedPtr)
          : NonnullMember(UnannotatedPtr),
            NullableMember(UnannotatedPtr),
            UnannotatedMember(UnannotatedPtr) {}
      Nonnull<std::shared_ptr<int>> NonnullMember;
      Nullable<std::shared_ptr<int>> NullableMember;
      std::shared_ptr<int> UnannotatedMember;
    };
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability

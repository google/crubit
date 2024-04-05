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

TEST(SmartPointerTest, AccessSmartPointerReturnedByReference) {
  // This is a repro for an assertion failure.
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct S {
      void f();
    };
    // The `const` is important for the repro (so that the AST at the callsite
    // below doesn't contain an `ImplicitCastExpr` to remove the const).
    const Nonnull<std::unique_ptr<S>>& ReturnNonnull();
    const Nullable<std::unique_ptr<S>>& ReturnNullable();
    const std::unique_ptr<S>& ReturnUnannotated();
    void target() {
      ReturnNonnull()->f();
      ReturnNullable()->f();  // [[unsafe]]
      ReturnUnannotated()->f();
    }
  )cc"));
}

TEST(SmartPointerTest, AccessSmartPointerReturnedByPointerAlias) {
  // This is a crash repro.
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>

    // When checking whether the base of a `->` member expression is a
    // pointer-to-smart-pointer, it used to be that we didn't canonicalize the
    // type. This test wraps such a return type in a type alias, which used to
    // cause a crash. The `const` is important because, without it, the AST
    // contains an `ImplicitCastExpr` that adds a `const`, desugaring the type
    // in the process.
    template <typename T>
    using Alias = T;
    Alias<const std::unique_ptr<int> *> getPtr();
    void target() { *getPtr()->get(); }
  )cc"));
}

TEST(SmartPointerTest, SmartPointerFlowSensitive) {
  // Simple flow-sensitive test with a smart pointer.
  // This is a repro for a false positive that we used to encounter in C++20
  // mode.
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target(Nullable<std::shared_ptr<int>> NullablePtr) {
      *NullablePtr;  // [[unsafe]]
      if (NullablePtr != nullptr) *NullablePtr;
    }
  )cc"));
}

TEST(SmartPointerTest, SimpleIfFpRepro) {
  // This is a repro for a false positive.
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    bool cond();

    void target() {
      std::shared_ptr<int> p;
      while (cond()) {
        if (p != nullptr) {
          *p;  // False positive here
        } else {
          p = std::make_shared<int>();
        }
        if (cond()) continue;
        p.reset();
      }
    }
  )cc"));
}

TEST(SmartPointerTest, NestedPointersArrowOperatorOnInner) {
  // This is a crash repro.
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>

    struct S {
      int i;
    };

    void target() {
      // The `const` is important because, without it, the AST for the arrow
      // access contains an `ImplicitCastExpr` that adds a `const` and is seen
      // as a smart pointer expression that initializes null state for the inner
      // smart pointer.
      std::unique_ptr<const std::unique_ptr<S>> p = nullptr;

      (void)(*p)->i;  // [[unsafe]]
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability

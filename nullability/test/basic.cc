// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for basic functionality (simple dereferences without control flow).

#include <memory>

#include "nullability/pointer_nullability_diagnosis.h"
#include "nullability/test/check_diagnostics.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/Basic/LLVM.h"
#include "clang/Frontend/ASTUnit.h"
#include "clang/Tooling/Tooling.h"
#include "llvm/Testing/Support/Error.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

using ::testing::IsEmpty;
using ::testing::SizeIs;

TEST(PointerNullabilityTest, NoPointerOperations) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() { 1 + 2; }
  )cc"));
}

TEST(PointerNullabilityTest, DerefNullPtr) {
  // nullptr
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      int *x = nullptr;
      *x;  // [[unsafe]]
    }
  )cc"));

  // 0
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      int *x = 0;
      *x;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, DerefAddrOf) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      int i;
      int *x = &i;
      *x;
    }
  )cc"));

  // transitive
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      int i;
      int *x = &i;
      int *y = x;
      *y;
    }
  )cc"));
}

TEST(PointerNullabilityTest, DerefPtrAnnotatedNonNullWithoutACheck) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull x) { *x; }
  )cc"));

  // transitive
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull x) {
      int *y = x;
      *y;
    }
  )cc"));
}

TEST(PointerNullabilityTest, DerefPtrAnnotatedNullableWithoutACheck) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable x) {
      *x;  // [[unsafe]]
    }
  )cc"));

  // transitive
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable x) {
      int *y = x;
      *y;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, DerefUnknownPtrWithoutACheck) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) { *x; }
  )cc"));

  // transitive
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) {
      int *y = x;
      *y;
    }
  )cc"));
}

TEST(PointerNullabilityTest, DoubleDereference) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int **p) {
      *p;
      **p;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int **_Nonnull p) {
      *p;
      **p;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull *p) {
      *p;
      **p;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull *_Nonnull p) {
      *p;
      **p;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int **_Nullable p) {
      *p;   // [[unsafe]]
      **p;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable *p) {
      *p;
      **p;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable *_Nullable p) {
      *p;   // [[unsafe]]
      **p;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable *_Nonnull p) {
      *p;
      **p;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull *_Nullable p) {
      *p;   // [[unsafe]]
      **p;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, ArrowOperatorOnNonNullPtr) {
  // (->) member field
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *foo;
    };
    void target(Foo *_Nonnull foo) { foo->foo; }
  )cc"));

  // (->) member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *foo();
    };
    void target(Foo *_Nonnull foo) { foo->foo(); }
  )cc"));
}

TEST(PointerNullabilityTest, ArrowOperatorOnNullablePtr) {
  // (->) member field
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *foo;
    };
    void target(Foo *_Nullable foo) {
      foo->foo;  // [[unsafe]]
      if (foo) {
        foo->foo;
      } else {
        foo->foo;  // [[unsafe]]
      }
      foo->foo;  // [[unsafe]]
    }
  )cc"));

  // (->) member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *foo();
    };
    void target(Foo *_Nullable foo) {
      foo->foo();  // [[unsafe]]
      if (foo) {
        foo->foo();
      } else {
        foo->foo();  // [[unsafe]]
      }
      foo->foo();  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, ArrowOperatorOnUnknownPtr) {
  // (->) member field
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *foo;
    };
    void target(Foo *foo) { foo->foo; }
  )cc"));

  // (->) member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *foo();
    };
    void target(Foo *foo) { foo->foo(); }
  )cc"));
}

TEST(PointerNullabilityTest, ArraySubscript) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull nonnull, int *_Nullable nullable, int *unknown) {
      nonnull[0];
      nullable[0];  // [[unsafe]]
      unknown[0];

      0 [nonnull];
      0 [nullable];  // [[unsafe]]
      0 [unknown];
    }
  )cc"));
}

TEST(PointerNullabilityTest, AssignmentToNonnull) {
  // TODO(b/307797224): This test demonstrates that we currently allow
  // null to be assigned to a nonnull pointer; in other words, within a
  // function, types are flow-sensitive. It's not clear, however, whether this
  // is the behavior we want. We should resolve this one way or the other.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull nonnull) {
      nonnull = nullptr;
    }
  )cc"));
}

TEST(PointerNullabilityTest, ForwardDeclaration) {
  // Check that we handle a function with a forward declaration correctly. This
  // is a regression test for a bug where we erroneously used the `ParmVarDecl`s
  // of the first declaration when creating initial values for the parameters;
  // the body uses the `ParmVarDecl`s of the second declaration, so it would not
  // see these initial values.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable p);
    void target(int* _Nullable p) {
      if (p == nullptr) return;
      *p;
    }
  )cc"));
}

TEST(PointerNullabilityTest, AnalyzeFunctionWithForwardDeclarationOnlyOnce) {
  std::unique_ptr<ASTUnit> Unit = tooling::buildASTFromCode(R"cc(
    // Check that we analyze a function with a forward declaration only once
    // (for the definition), and not for every redeclaration that we encounter.
    void target();
    void target() {
      int *p = nullptr;
      *p;
    }
  )cc");

  ASTContext &Context = Unit->getASTContext();
  DeclContextLookupResult Result =
      Context.getTranslationUnitDecl()->lookup(&Context.Idents.get("target"));
  ASSERT_TRUE(Result.isSingleResult());
  auto *Target = cast<FunctionDecl>(Result.front());
  SmallVector<FunctionDecl *> Redecls(Target->redecls());
  ASSERT_EQ(Redecls.size(), 2);

  EXPECT_TRUE(Redecls[0]->doesThisDeclarationHaveABody());
  EXPECT_THAT_EXPECTED(diagnosePointerNullability(Redecls[0]),
                       llvm::HasValue(SizeIs(1)));

  EXPECT_FALSE(Redecls[1]->doesThisDeclarationHaveABody());
  EXPECT_THAT_EXPECTED(diagnosePointerNullability(Redecls[1]),
                       llvm::HasValue(IsEmpty()));
}

TEST(PointerNullabilityTest, CheckMacro) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#define CHECK(x) \
      if (!x) __builtin_abort();
    void target(int* _Nullable p) {
      CHECK(p);
      *p;
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability

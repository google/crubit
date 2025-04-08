// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for basic functionality (simple dereferences without control flow).

#include <memory>

#include "nullability/pointer_nullability_diagnosis.h"
#include "nullability/pragma.h"
#include "nullability/test/check_diagnostics.h"
#include "clang/include/clang/AST/ASTContext.h"
#include "clang/include/clang/AST/Decl.h"
#include "clang/include/clang/AST/DeclBase.h"
#include "clang/include/clang/Basic/LLVM.h"
#include "clang/include/clang/Frontend/ASTUnit.h"
#include "clang/include/clang/Tooling/Tooling.h"
#include "llvm/include/llvm/Testing/Support/Error.h"
#include "external/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

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

TEST(PointerNullabilityTest, DeclarationWithInit) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    // The following global variable initializations are not checked, because
    // the check doesn't look at global variable declarations, and maybe also
    // because the test infra doesn't look outside the target function. When
    // fixing b/395869919, we should address both and remove the TODO below to
    // enable the [[unsafe]] expectation for the initialization of
    // global_nonnull with nullptr.
    int *_Nonnull global_nonnull = nullptr;  // TODO: b/395869919 - [[unsafe]]
    int *_Nonnull global_nonnull_safe = new int;
    int *_Nullable global_nullable = nullptr;
    int *global_unknown = nullptr;

    void target(int *_Nullable nullable) {
      int *_Nonnull local_nonnull_decl_with_init = nullptr;  // [[unsafe]]
      int *_Nonnull local_nonnull_decl_with_init_nullable = nullable;  // [[unsafe]]
      int *_Nonnull a = nullptr,  // [[unsafe]]
          *_Nonnull b = nullptr;  // [[unsafe]]
      int *_Nonnull c = new int;
    }
  )cc"));
}

TEST(PointerNullabilityTest, Assignment) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Null_unspecified getUnknown();

    int *_Nonnull global_nonnull = new int;
    int *_Nullable global_nullable = nullptr;
    int *global_unknown = getUnknown();

    void target(int *_Nonnull nonnull, int *_Nullable nullable, int *unknown) {
      nonnull = new int;
      nonnull = nullptr;  // [[unsafe]]

      nullable = new int;
      nullable = nullptr;

      unknown = new int;
      unknown = nullptr;

      // Check that we can handle cases where there isn't just a simple
      // `DeclRefExpr` on the left-hand side.
      *(&nonnull) = nullptr;  // [[unsafe]]

      nonnull = nullable;  // [[unsafe]]
      unknown = nullable;

      global_nonnull = nullptr;   // [[unsafe]]
      global_nonnull = nullable;  // [[unsafe]]
      global_nonnull = getUnknown();
      global_unknown = nullptr;
      global_unknown = nullable;

      int *_Nonnull local_nonnull = new int;
      local_nonnull = new int;
      local_nonnull = nullptr;   // [[unsafe]]
      local_nonnull = nullable;  // [[unsafe]]
      local_nonnull = getUnknown();
      int *_Null_unspecified local_unknown = getUnknown();
      local_unknown = getUnknown();
      local_unknown = new int;
      local_unknown = nullptr;
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
  NullabilityPragmas NoPragmas;

  ASTContext &Context = Unit->getASTContext();
  DeclContextLookupResult Result =
      Context.getTranslationUnitDecl()->lookup(&Context.Idents.get("target"));
  ASSERT_TRUE(Result.isSingleResult());
  auto *Target = cast<FunctionDecl>(Result.front());
  SmallVector<FunctionDecl *> Redecls(Target->redecls());
  ASSERT_EQ(Redecls.size(), 2);

  EXPECT_TRUE(Redecls[0]->doesThisDeclarationHaveABody());
  EXPECT_THAT_EXPECTED(diagnosePointerNullability(Redecls[0], NoPragmas),
                       llvm::HasValue(SizeIs(1)));

  EXPECT_FALSE(Redecls[1]->doesThisDeclarationHaveABody());
  EXPECT_THAT_EXPECTED(diagnosePointerNullability(Redecls[1], NoPragmas),
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

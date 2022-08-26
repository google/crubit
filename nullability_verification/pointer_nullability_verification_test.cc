// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <string>

#include "nullability_verification/pointer_nullability_analysis.h"
#include "nullability_verification/pointer_nullability_diagnosis.h"
#include "clang/Basic/SourceManager.h"
#include "third_party/llvm/llvm-project/clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/DenseSet.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "llvm/Testing/Support/Error.h"
#include "third_party/llvm/llvm-project/llvm/utils/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

using dataflow::Environment;
using dataflow::TypeErasedDataflowAnalysisState;
using dataflow::test::AnalysisData;
using dataflow::test::checkDataflow;
using ::testing::ContainerEq;
using ::testing::Test;

void checkDiagnostics(llvm::StringRef SourceCode) {
  std::vector<const Stmt *> Diagnostics;
  PointerNullabilityDiagnoser Diagnoser;
  ASSERT_THAT_ERROR(
      checkDataflow<PointerNullabilityAnalysis>(
          SourceCode, ast_matchers::hasName("target"),
          [](ASTContext &ASTCtx, Environment &) {
            return PointerNullabilityAnalysis(ASTCtx);
          },
          [&Diagnostics, &Diagnoser](
              ASTContext &Ctx, const CFGStmt &Stmt,
              const TypeErasedDataflowAnalysisState &State) {
            auto StmtDiagnostics =
                Diagnoser.diagnose(Stmt.getStmt(), Ctx, State.Env);
            if (StmtDiagnostics.has_value()) {
              Diagnostics.push_back(StmtDiagnostics.value());
            }
          },
          [&Diagnostics](AnalysisData AnalysisData) {
            llvm::DenseSet<unsigned> ExpectedLines, ActualLines;
            auto &SrcMgr = AnalysisData.ASTCtx.getSourceManager();
            for (auto [Stmt, _] : AnalysisData.Annotations) {
              ExpectedLines.insert(
                  SrcMgr.getPresumedLineNumber(Stmt->getBeginLoc()));
            }
            for (auto *Stmt : Diagnostics) {
              ActualLines.insert(
                  SrcMgr.getPresumedLineNumber(Stmt->getBeginLoc()));
            }
            EXPECT_THAT(ActualLines, ContainerEq(ExpectedLines));
          },
          {"-fsyntax-only", "-std=c++17", "-Wno-unused-value", "-Wno-nonnull"}),
      llvm::Succeeded());
}

TEST(PointerNullabilityTest, NoPointerOperations) {
  checkDiagnostics(R"(
    void target() {
      1 + 2;
    }
  )");
}

TEST(PointerNullabilityTest, DerefNullPtr) {
  // nullptr
  checkDiagnostics(R"(
    void target() {
      int *x = nullptr;
      *x; // [[unsafe]]
    }
  )");

  // 0
  checkDiagnostics(R"(
    void target() {
      int *x = 0;
      *x; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, DerefAddrOf) {
  checkDiagnostics(R"(
    void target() {
      int i;
      int *x = &i;
      *x;
    }
  )");

  // transitive
  checkDiagnostics(R"(
    void target() {
      int i;
      int *x = &i;
      int *y = x;
      *y;
    }
  )");
}

TEST(PointerNullabilityTest, DerefPtrAnnotatedNonNullWithoutACheck) {
  checkDiagnostics(R"(
    void target(int * _Nonnull x) {
      *x;
    }
  )");

  // transitive
  checkDiagnostics(R"(
    void target(int * _Nonnull x) {
      int *y = x;
      *y;
    }
  )");
}

TEST(PointerNullabilityTest, DerefPtrAnnotatedNullableWithoutACheck) {
  checkDiagnostics(R"(
    void target(int * _Nullable x) {
      *x; // [[unsafe]]
    }
  )");

  // transitive
  checkDiagnostics(R"(
    void target(int * _Nullable x) {
      int *y = x;
      *y; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, DerefUnknownPtrWithoutACheck) {
  checkDiagnostics(R"(
    void target(int *x) {
      *x;
    }
  )");

  // transitive
  checkDiagnostics(R"(
    void target(int *x) {
      int *y = x;
      *y;
    }
  )");
}

// TODO(b/233582219): Implement diagnosis of unreachable program points
TEST(PointerNullabilityTest, NonNullPtrImplicitCastToBool) {
  // x
  checkDiagnostics(R"(
    void target(int * _Nonnull x) {
      *x;
      if (x) {
        *x;
      } else {
        *x; // unreachable
      }
      *x;
    }
  )");

  // !x
  checkDiagnostics(R"(
    void target(int * _Nonnull x) {
      *x;
      if (!x) {
        *x; // unreachable
      } else {
        *x;
      }
      *x;
    }
  )");
}

TEST(PointerNullabilityTest, NullablePtrImplicitCastToBool) {
  // x
  checkDiagnostics(R"(
    void target(int * _Nullable x) {
      *x; // [[unsafe]]
      if (x) {
        *x;
      } else {
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
    }
  )");

  // !x
  checkDiagnostics(R"(
    void target(int * _Nullable x) {
      *x; // [[unsafe]]
      if (!x) {
        *x; // [[unsafe]]
      } else {
        *x;
      }
      *x; // [[unsafe]]
    }
  )");
}

// TODO(b/233582219): Fix false negatives. Casting the pointer to boolean is
// evidence of the author considering null a possibility, hence the unnannotated
// pointer should be considered nullable and emit warnings where it fails or is
// not null checked.
TEST(PointerNullabilityTest, UnknownPtrImplicitCastToBool) {
  // x
  checkDiagnostics(R"(
    void target(int *x) {
      *x; // false-negative
      if (x) {
        *x;
      } else {
        *x; // false-negative
      }
      *x; // false-negative
    }
  )");

  // !x
  checkDiagnostics(R"(
    void target(int *x) {
      *x; // false-negative
      if (!x) {
        *x; // false-negative
      } else {
        *x;
      }
      *x; // false-negative
    }
  )");
}

TEST(PointerNullabilityTest, CompareNonNullPtrAndNonNullPtr) {
  // nonnull == nonnull
  checkDiagnostics(R"(
    void target(int * _Nonnull x, int * _Nonnull y) {
      *x;
      *y;
      if (x == y) {
        *x;
        *y;
      } else {
        *x;
        *y;
      }
      *x;
      *y;
    }
  )");

  // nonnull != nonnull
  checkDiagnostics(R"(
    void target(int * _Nonnull x, int * _Nonnull y) {
      *x;
      *y;
      if (x != y) {
        *x;
        *y;
      } else {
        *x;
        *y;
      }
      *x;
      *y;
    }
  )");
}

TEST(PointerNullabilityTest, CompareNullablePtrAndNullablePtr) {
  // nullable == nullable
  checkDiagnostics(R"(
    void target(int * _Nullable x, int * _Nullable y) {
      *x; // [[unsafe]]
      *y; // [[unsafe]]
      if (x == y) {
        *x; // [[unsafe]]
        *y; // [[unsafe]]
      } else {
        *x; // [[unsafe]]
        *y; // [[unsafe]]
      }
      *x; // [[unsafe]]
      *y; // [[unsafe]]
    }
  )");

  // nullable != nullable
  checkDiagnostics(R"(
    void target(int * _Nullable x, int * _Nullable y) {
      *x; // [[unsafe]]
      *y; // [[unsafe]]
      if (x != y) {
        *x; // [[unsafe]]
        *y; // [[unsafe]]
      } else {
        *x; // [[unsafe]]
        *y; // [[unsafe]]
      }
      *x; // [[unsafe]]
      *y; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, CompareUnknownPtrAndUnknownPtr) {
  // unknown == unknown
  checkDiagnostics(R"(
    void target(int *x, int *y) {
      *x;
      *y;
      if (x == y) {
        *x;
        *y;
      } else {
        *x;
        *y;
      }
      *x;
      *y;
    }
  )");

  // unknown != unknown
  checkDiagnostics(R"(
    void target(int *x, int *y) {
      *x;
      *y;
      if (x != y) {
        *x;
        *y;
      } else {
        *x;
        *y;
      }
      *x;
      *y;
    }
  )");
}

// TODO(b/233582219): Implement diagnosis of unreachable program points
TEST(PointerNullabilityTest, CompareNonNullPtrAndNullPtr) {
  // nonnull == nullptr
  checkDiagnostics(R"(
    void target(int * _Nonnull x) {
      *x;
      if (x == nullptr) {
        *x; // unreachable
      } else {
        *x;
      }
      *x;
    }
  )");

  // nullptr == nonnull
  checkDiagnostics(R"(
    void target(int * _Nonnull x) {
      *x;
      if (nullptr == x) {
        *x; // unreachable
      } else {
        *x;
      }
      *x;
    }
  )");

  // nonnull != nullptr
  checkDiagnostics(R"(
    void target(int * _Nonnull x) {
      *x;
      if (x != nullptr) {
        *x;
      } else {
        *x; // unreachable
      }
      *x;
    }
  )");

  // nullptr != nonnull
  checkDiagnostics(R"(
    void target(int * _Nonnull x) {
      *x;
      if (nullptr != x) {
        *x;
      } else {
        *x; // unreachable
      }
      *x;
    }
  )");
}

TEST(PointerNullabilityTest, CompareNullablePtrAndNullPtr) {
  // nullable == nullptr
  checkDiagnostics(R"(
    void target(int * _Nullable x) {
      *x; // [[unsafe]]
      if (x == nullptr) {
        *x; // [[unsafe]]
      } else {
        *x;
      }
      *x; // [[unsafe]]
    }
  )");

  // nullptr == nullable
  checkDiagnostics(R"(
    void target(int * _Nullable x) {
      *x; // [[unsafe]]
      if (nullptr == x) {
        *x; // [[unsafe]]
      } else {
        *x;
      }
      *x; // [[unsafe]]
    }
  )");

  // nullable != nullptr
  checkDiagnostics(R"(
    void target(int * _Nullable x) {
      *x; // [[unsafe]]
      if (x != nullptr) {
        *x;
      } else {
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
    }
  )");

  // nullptr != nullable
  checkDiagnostics(R"(
    void target(int * _Nullable x) {
      *x; // [[unsafe]]
      if (nullptr != x) {
        *x;
      } else {
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, CompareNullablePtrAndNonNullPtr) {
  // nullable == nonnull
  checkDiagnostics(R"(
    void target(int * _Nullable x, int * _Nonnull y) {
      *x; // [[unsafe]]
      *y;
      if (x == y) {
        *x;
        *y;
      } else {
        *x; // [[unsafe]]
        *y;
      }
      *x; // [[unsafe]]
      *y;
    }
  )");

  // nonnull == nullable
  checkDiagnostics(R"(
    void target(int * _Nullable x, int * _Nonnull y) {
      *x; // [[unsafe]]
      *y;
      if (y == x) {
        *x;
        *y;
      } else {
        *x; // [[unsafe]]
        *y;
      }
      *x; // [[unsafe]]
      *y;
    }
  )");

  // nullable != nonnull
  checkDiagnostics(R"(
    void target(int * _Nullable x, int * _Nonnull y) {
      *x; // [[unsafe]]
      *y;
      if (x != y) {
        *x; // [[unsafe]]
        *y;
      } else {
        *x;
        *y;
      }
      *x; // [[unsafe]]
      *y;
    }
  )");

  // nonnull != nullable
  checkDiagnostics(R"(
    void target(int * _Nullable x, int * _Nonnull y) {
      *x; // [[unsafe]]
      *y;
      if (y != x) {
        *x; // [[unsafe]]
        *y;
      } else {
        *x;
        *y;
      }
      *x; // [[unsafe]]
      *y;
    }
  )");
}

TEST(PointerNullabilityTest, CompareNullablePtrAndUnknownPtr) {
  // nullable == unknown
  checkDiagnostics(R"(
    void target(int * _Nullable x, int *y) {
      *x; // [[unsafe]]
      *y;
      if (x == y) {
        *x; // [[unsafe]]
        *y;
      } else {
        *x; // [[unsafe]]
        *y;
      }
      *x; // [[unsafe]]
      *y;
    }
  )");

  // unknown == nullable
  checkDiagnostics(R"(
    void target(int * _Nullable x, int *y) {
      *x; // [[unsafe]]
      *y;
      if (y == x) {
        *x; // [[unsafe]]
        *y;
      } else {
        *x; // [[unsafe]]
        *y;
      }
      *x; // [[unsafe]]
      *y;
    }
  )");

  // nullable != unknown
  checkDiagnostics(R"(
    void target(int * _Nullable x, int *y) {
      *x; // [[unsafe]]
      *y;
      if (x != y) {
        *x; // [[unsafe]]
        *y;
      } else {
        *x; // [[unsafe]]
        *y;
      }
      *x; // [[unsafe]]
      *y;
    }
  )");

  // unknown != nullable
  checkDiagnostics(R"(
    void target(int * _Nullable x, int *y) {
      *x; // [[unsafe]]
      *y;
      if (y != x) {
        *x; // [[unsafe]]
        *y;
      } else {
        *x; // [[unsafe]]
        *y;
      }
      *x; // [[unsafe]]
      *y;
    }
  )");
}

// TODO(b/233582219): Fix false negatives. The pointer is compared to nullptr,
// hence the unnannotated pointer should be considered nullable and emit
// warnings where it fails or is not null checked.
TEST(PointerNullabilityTest, CompareUnknownPtrAndNullPtr) {
  // unknown == nullptr
  checkDiagnostics(R"(
    void target(int *x) {
      *x; // false-negative
      if (x == nullptr) {
        *x; // false-negative
      } else {
        *x;
      }
      *x; // false-negative
    }
  )");

  // nullptr == unknown
  checkDiagnostics(R"(
    void target(int *x) {
      *x; // false-negative
      if (nullptr == x) {
        *x; // false-negative
      } else {
        *x;
      }
      *x; // false-negative
    }
  )");

  // unknown != nullptr
  checkDiagnostics(R"(
    void target(int *x) {
      *x; // false-negative
      if (x != nullptr) {
        *x;
      } else {
        *x; // false-negative
      }
      *x; // false-negative
    }
  )");

  // nullptr != unknown
  checkDiagnostics(R"(
    void target(int *x) {
      *x; // false-negative
      if (nullptr != x) {
        *x;
      } else {
        *x; // false-negative
      }
      *x; // false-negative
    }
  )");
}

TEST(PointerNullabilityTest, CompareUnknownPtrAndNonNullPtr) {
  // unknown == nonnull
  checkDiagnostics(R"(
    void target(int *x, int * _Nonnull y) {
      *x;
      *y;
      if (x == y) {
        *x;
        *y;
      } else {
        *x;
        *y;
      }
      *x;
      *y;
    }
  )");

  // nonnull == unknown
  checkDiagnostics(R"(
    void target(int *x, int * _Nonnull y) {
      *x;
      *y;
      if (y == x) {
        *x;
        *y;
      } else {
        *x;
        *y;
      }
      *x;
      *y;
    }
  )");

  // unknown != nonnull
  checkDiagnostics(R"(
    void target(int *x, int * _Nonnull y) {
      *x;
      *y;
      if (x != y) {
        *x;
        *y;
      } else {
        *x;
        *y;
      }
      *x;
      *y;
    }
  )");

  // nonnull != unknown
  checkDiagnostics(R"(
    void target(int *x, int * _Nonnull y) {
      *x;
      *y;
      if (y != x) {
        *x;
        *y;
      } else {
        *x;
        *y;
      }
      *x;
      *y;
    }
  )");
}

TEST(PointerNullabilityTest, TransitiveNullCheck) {
  checkDiagnostics(R"(
    void target(int * _Nullable x) {
      int *y = x;
      *x; // [[unsafe]]
      if (y) {
        *x;
      } else {
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    void target(int * _Nullable x) {
      int *y = x;
      *y; // [[unsafe]]
      if (x) {
        *y;
      } else {
        *y; // [[unsafe]]
      }
      *y; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, BinaryExpressions) {
  // x && y
  checkDiagnostics(R"(
    void target(int * _Nullable x, int * _Nullable y) {
      *x; // [[unsafe]]
      *y; // [[unsafe]]
      if (x && y) {
        *x;
        *y;
      } else {
        *x; // [[unsafe]]
        *y; // [[unsafe]]
      }
      *x; // [[unsafe]]
      *y; // [[unsafe]]
    }
  )");

  // x || y
  checkDiagnostics(R"(
    void target(int * _Nullable x, int * _Nullable y) {
      *x; // [[unsafe]]
      *y; // [[unsafe]]
      if (x || y) {
        *x; // [[unsafe]]
        *y; // [[unsafe]]
      } else {
        *x; // [[unsafe]]
        *y; // [[unsafe]]
      }
      *x; // [[unsafe]]
      *y; // [[unsafe]]
    }
  )");

  // !x && !y
  checkDiagnostics(R"(
    void target(int * _Nullable x, int * _Nullable y) {
      *x; // [[unsafe]]
      *y; // [[unsafe]]
      if (!x && !y) {
        *x; // [[unsafe]]
        *y; // [[unsafe]]
      } else {
        *x; // [[unsafe]]
        *y; // [[unsafe]]
      }
      *x; // [[unsafe]]
      *y; // [[unsafe]]
    }
  )");

  // !x || !y
  checkDiagnostics(R"(
    void target(int * _Nullable x, int * _Nullable y) {
      *x; // [[unsafe]]
      *y; // [[unsafe]]
      if (!x || !y) {
        *x; // [[unsafe]]
        *y; // [[unsafe]]
      } else {
        *x;
        *y;
      }
      *x; // [[unsafe]]
      *y; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, ArrowOperatorOnNonNullPtr) {
  // (->) member field
  checkDiagnostics(R"(
    struct Foo {
      Foo *foo;
    };
    void target(Foo * _Nonnull foo) {
      foo->foo;
    }
  )");

  // (->) member function
  checkDiagnostics(R"(
    struct Foo {
      Foo *foo();
    };
    void target(Foo * _Nonnull foo) {
      foo->foo();
    }
  )");
}

TEST(PointerNullabilityTest, ArrowOperatorOnNullablePtr) {
  // (->) member field
  checkDiagnostics(R"(
    struct Foo {
      Foo *foo;
    };
    void target(Foo * _Nullable foo) {
      foo->foo; // [[unsafe]]
      if (foo) {
        foo->foo;
      } else {
        foo->foo; // [[unsafe]]
      }
      foo->foo; // [[unsafe]]
    }
  )");

  // (->) member function
  checkDiagnostics(R"(
    struct Foo {
      Foo *foo();
    };
    void target(Foo * _Nullable foo) {
      foo->foo(); // [[unsafe]]
      if (foo) {
        foo->foo();
      } else {
        foo->foo(); // [[unsafe]]
      }
      foo->foo(); // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, ArrowOperatorOnUnknownPtr) {
  // (->) member field
  checkDiagnostics(R"(
    struct Foo {
      Foo *foo;
    };
    void target(Foo *foo) {
      foo->foo;
    }
  )");

  // (->) member function
  checkDiagnostics(R"(
    struct Foo {
      Foo *foo();
    };
    void target(Foo *foo) {
      foo->foo();
    }
  )");
}

TEST(PointerNullabilityTest, ThisPointer) {
  // (->) implicit `this`
  checkDiagnostics(R"(
    struct Foo {
      void foo();
      void target() {
        foo();
      }
    };
  )");

  // (->) explicit `this`
  checkDiagnostics(R"(
    struct Foo {
      void foo();
      void target() {
        this->foo();
      }
    };
  )");
}

TEST(PointerNullabilityTest, NonNullFieldsOfPointerType) {
  // dereference field of pointer type
  checkDiagnostics(R"(
    struct Foo {
      Foo * _Nonnull ptr;
    };
    void target(Foo foo) {
      *foo.ptr;
    }
  )");

  // dereference field of pointer type in member function
  checkDiagnostics(R"(
    struct Foo {
      Foo * _Nonnull ptr;
      void target() {
        *ptr;
      }
    };
  )");
}

TEST(PointerNullabilityTest, NullableFieldsOfPointerType) {
  // dereference field of pointer type
  checkDiagnostics(R"(
    struct Foo {
      Foo * _Nullable ptr;
    };
    void target(Foo foo) {
      *foo.ptr; // [[unsafe]]
      if (foo.ptr) {
        *foo.ptr;
      } else {
        *foo.ptr; // [[unsafe]]
      }
      *foo.ptr; // [[unsafe]]
    }
  )");

  // dereference field of pointer type in member function
  checkDiagnostics(R"(
    struct Foo {
      Foo * _Nullable ptr;
      void target() {
        *ptr; // [[unsafe]]
        if (ptr) {
          *ptr;
        } else {
          *ptr; // [[unsafe]]
        }
        *ptr; // [[unsafe]]
      }
    };
  )");
}

TEST(PointerNullabilityTest, UnknownFieldsOfPointerType) {
  // dereference field of pointer type
  checkDiagnostics(R"(
    struct Foo {
      Foo *ptr;
    };
    void target(Foo foo) {
      *foo.ptr;
    }
  )");

  // dereference field of pointer type in member function
  checkDiagnostics(R"(
    struct Foo {
      Foo *ptr;
      void target() {
        *ptr;
      }
    };
  )");
}

TEST(PointerNullabilityTest, MergeNullAndNonNull) {
  checkDiagnostics(R"(
    void target(int * _Nonnull y, bool b) {
      int *x = nullptr;
      *x; // [[unsafe]]
      if (b) {
        *x; // [[unsafe]]
        x = y;
        *x;
      }
      *x; // [[unsafe]]
      if (b) {
        *x;
      } else {
        *x; // [[unsafe]]
      }
    }
  )");
}

TEST(PointerNullabilityTest, MergeNullAndNullable) {
  checkDiagnostics(R"(
    void target(int * _Nullable y, bool b) {
      int *x = nullptr;
      *x; // [[unsafe]]
      if (b) {
        *x; // [[unsafe]]
        x = y;
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
      if (b) {
        *x; // [[unsafe]]
      } else {
        *x; // [[unsafe]]
      }
    }
  )");
}

TEST(PointerNullabilityTest, MergeNullAndUnknown) {
  checkDiagnostics(R"(
    void target(int *y, bool b) {
      int *x = nullptr;
      *x; // [[unsafe]]
      if (b) {
        *x; // [[unsafe]]
        x = y;
        *x;
      }
      *x; // [[unsafe]]
      if (b) {
        *x;
      } else {
        *x; // [[unsafe]]
      }
    }
  )");
}

TEST(PointerNullabilityTest, MergeNonNullAndNull) {
  checkDiagnostics(R"(
    void target(int * _Nonnull y, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = nullptr;
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
      if (b) {
        *x; // [[unsafe]]
      } else {
        *x;
      }
    }
  )");
}

TEST(PointerNullabilityTest, MergeNonNullAndNonNull) {
  checkDiagnostics(R"(
    void target(int * _Nonnull y, int * _Nonnull z, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = z;
        *x;
      }
      *x;
      if (b) {
        *x;
      } else {
        *x;
      }
    }
  )");
}

TEST(PointerNullabilityTest, MergeNonNullAndNullable) {
  checkDiagnostics(R"(
    void target(int * _Nonnull y, int * _Nullable z, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = z;
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
      if (b) {
        *x; // [[unsafe]]
      } else {
        *x;
      }
    }
  )");
}

TEST(PointerNullabilityTest, MergeNonNullAndUnknown) {
  checkDiagnostics(R"(
    void target(int * _Nonnull y, int *z, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = z;
        *x;
      }
      *x;
      if (b) {
        *x;
      } else {
        *x;
      }
    }
  )");
}

TEST(PointerNullabilityTest, MergeNullableAndNull) {
  checkDiagnostics(R"(
    void target(int * _Nullable y, bool b) {
      int *x = y;
      *x; // [[unsafe]]
      if (b) {
        *x; // [[unsafe]]
        x = nullptr;
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
      if (b) {
        *x; // [[unsafe]]
      } else {
        *x; // [[unsafe]]
      }
    }
  )");
}

TEST(PointerNullabilityTest, MergeNullableAndNonNull) {
  checkDiagnostics(R"(
    void target(int * _Nullable y, int * _Nonnull z, bool b) {
      int *x = y;
      *x; // [[unsafe]]
      if (b) {
        *x; // [[unsafe]]
        x = z;
        *x;
      }
      *x; // [[unsafe]]
      if (b) {
        *x;
      } else {
        *x; // [[unsafe]]
      }
    }
  )");
}

TEST(PointerNullabilityTest, MergeNullableAndNullable) {
  checkDiagnostics(R"(
    void target(int * _Nullable y, int * _Nullable z, bool b) {
      int *x = y;
      *x; // [[unsafe]]
      if (b) {
        *x; // [[unsafe]]
        x = z;
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
      if (b) {
        *x; // [[unsafe]]
      } else {
        *x; // [[unsafe]]
      }
    }
  )");
}

TEST(PointerNullabilityTest, MergeNullableAndUnknown) {
  checkDiagnostics(R"(
    void target(int * _Nullable y, int *z, bool b) {
      int *x = y;
      *x; // [[unsafe]]
      if (b) {
        *x; // [[unsafe]]
        x = z;
        *x;
      }
      *x; // [[unsafe]]
      if (b) {
        *x;
      } else {
        *x; // [[unsafe]]
      }
    }
  )");
}

TEST(PointerNullabilityTest, MergeUnknownAndNull) {
  checkDiagnostics(R"(
    void target(int *y, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = nullptr;
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
      if (b) {
        *x; // [[unsafe]]
      } else {
        *x;
      }
    }
  )");
}

TEST(PointerNullabilityTest, MergeUnknownAndNonNull) {
  checkDiagnostics(R"(
    void target(int *y, int * _Nonnull z, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = z;
        *x;
      }
      *x;
      if (b) {
        *x;
      } else {
        *x;
      }
    }
  )");
}

TEST(PointerNullabilityTest, MergeUnknownAndNullable) {
  checkDiagnostics(R"(
    void target(int *y, int * _Nullable z, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = z;
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
      if (b) {
        *x; // [[unsafe]]
      } else {
        *x;
      }
    }
  )");
}

TEST(PointerNullabilityTest, MergeUnknownAndUnknown) {
  checkDiagnostics(R"(
    void target(int *y, int *z, bool b) {
      int *x = y;
      if (b) {
        *x;
        x = z;
        *x;
      }
      *x;
      if (b) {
        *x;
      } else {
        *x;
      }
    }
  )");
}

TEST(PointerNullabilityTest, CallExprWithPointerReturnType) {
  // free function
  checkDiagnostics(R"(
    int * _Nonnull makeNonnull();
    int * _Nullable makeNullable();
    int *makeUnannotated();
    void target() {
      *makeNonnull();
      *makeNullable();    // [[unsafe]]
      *makeUnannotated();
    }
  )");

  // member function
  checkDiagnostics(R"(
    struct Foo {
      int * _Nonnull makeNonnull();
      int * _Nullable makeNullable();
      int *makeUnannotated();
    };
    void target(Foo foo) {
      *foo.makeNonnull();
      *foo.makeNullable();    // [[unsafe]]
      *foo.makeUnannotated();
    }
  )");

  // overloaded operator call
  checkDiagnostics(R"(
    struct MakeNonnull {
      int * _Nonnull operator()();
    };
    struct MakeNullable {
      int * _Nullable operator()();
    };
    struct MakeUnannotated {
      int *operator()();
    };
    void target() {
      MakeNonnull makeNonnull;
      *makeNonnull();

      MakeNullable makeNullable;
      *makeNullable();  // [[unsafe]]

      MakeUnannotated makeUnannotated;
      *makeUnannotated();
    }
  )");

  // function pointer
  checkDiagnostics(R"(
    void target(int * _Nonnull (*makeNonnull)(),
                int * _Nullable (*makeNullable)(),
                int * (*makeUnannotated)()) {
      *makeNonnull();
      *makeNullable();    // [[unsafe]]
      *makeUnannotated();
    }
  )");

  // pointer to function pointer
  checkDiagnostics(R"(
    void target(int * _Nonnull (**makeNonnull)(),
                int * _Nullable (**makeNullable)(),
                int * (**makeUnannotated)()) {
      *(*makeNonnull)();
      *(*makeNullable)();   // [[unsafe]]
      *(*makeUnannotated)();
    }
  )");

  // function returning a function pointer which returns a pointer
  checkDiagnostics(R"(
    typedef int * _Nonnull (*MakeNonnullT)();
    typedef int * _Nullable (*MakeNullableT)();
    typedef int * (*MakeUnannotatedT)();
    void target(MakeNonnullT (*makeNonnull)(),
                MakeNullableT (*makeNullable)(),
                MakeUnannotatedT (*makeUnannotated)()) {
      *(*makeNonnull)()();
      *(*makeNullable)()();   // [[unsafe]]
      *(*makeUnannotated)()();
    }
  )");

  // free function returns reference to pointer
  checkDiagnostics(R"(
    int * _Nonnull & makeNonnull();
    int * _Nullable & makeNullable();
    int *&makeUnannotated();
    void target() {
      *makeNonnull();
      *makeNullable();    // [[unsafe]]
      *makeUnannotated();
    }
  )");

  // function called in loop
  //
  // TODO(b/233582219): Fix false negative. The pointer is only null-checked and
  // therefore safe to dereference on the first iteration of the loop. On
  // subsequent iterations of the loop, the pointer dereference is unsafe due to
  // the lack of null check. The diagnoser currently fails to catch the
  // unsafe dereference as it only evaluates the statement once.
  checkDiagnostics(R"(
    int * _Nullable makeNullable();
    bool makeBool();
    void target() {
      bool first = true;
      while(true) {
        int *x = makeNullable();
        if (first && x == nullptr) return;
        first = false;
        *x; // false-negative
      }
    }
  )");
}

TEST(PointerNullabilityTest, CallExprParamAssignment) {
  // free function with single param
  checkDiagnostics(R"(
    void takeNonnull(int * _Nonnull);
    void takeNullable(int * _Nullable);
    void takeUnannotated(int *);
    void target(int * _Nonnull ptr_nonnull,
                int * _Nullable ptr_nullable,
                int *ptr_unannotated) {
      takeNonnull(nullptr);             // [[unsafe]]
      takeNonnull(ptr_nonnull);
      takeNonnull(ptr_nullable);        // [[unsafe]]
      takeNonnull(ptr_unannotated);

      takeNullable(nullptr);
      takeNullable(ptr_nonnull);
      takeNullable(ptr_nullable);
      takeNullable(ptr_unannotated);

      takeUnannotated(nullptr);
      takeUnannotated(ptr_nonnull);
      takeUnannotated(ptr_nullable);
      takeUnannotated(ptr_unannotated);
    }
  )");

  // overloaded operator with single param
  checkDiagnostics(R"(
    // map<int * _Nonnull, int>
    struct MapWithNonnullKeys {
      int &operator[](int * _Nonnull key);
    };
    // map<int * _Nullable, int>
    struct MapWithNullableKeys {
      int &operator[](int * _Nullable key);
    };
    // map<int *, int>
    struct MapWithUnannotatedKeys {
      int &operator[](int *key);
    };
    void target(int * _Nonnull ptr_nonnull,
                int * _Nullable ptr_nullable,
                int *ptr_unannotated) {
      MapWithNonnullKeys nonnull_keys;
      nonnull_keys[nullptr] = 42;             // [[unsafe]]
      nonnull_keys[ptr_nonnull] = 42;
      nonnull_keys[ptr_nullable] = 42;        // [[unsafe]]
      nonnull_keys[ptr_unannotated] = 42;

      MapWithNullableKeys nullable_keys;
      nullable_keys[nullptr] = 42;
      nullable_keys[ptr_nonnull] = 42;
      nullable_keys[ptr_nullable] = 42;
      nullable_keys[ptr_unannotated] = 42;

      MapWithUnannotatedKeys unannotated_keys;
      unannotated_keys[nullptr] = 42;
      unannotated_keys[ptr_nonnull] = 42;
      unannotated_keys[ptr_nullable] = 42;
      unannotated_keys[ptr_unannotated] = 42;
    }
  )");

  // free function with multiple params of mixed nullability
  checkDiagnostics(R"(
    void takeMixed(int *, int * _Nullable, int * _Nonnull);
    void target() {
      takeMixed(nullptr, nullptr, nullptr); // [[unsafe]]
    }
  )");

  // overloaded operator with multiple params of mixed nullability
  checkDiagnostics(R"(
    struct TakeMixed {
      void operator()(int *, int * _Nullable, int * _Nonnull);
    };
    void target() {
      TakeMixed takeMixed;
      takeMixed(nullptr, nullptr, nullptr); // [[unsafe]]
    }
  )");

  // member function
  checkDiagnostics(R"(
    struct Foo {
      void takeNonnull(int * _Nonnull);
      void takeNullable(int * _Nullable);
      void takeUnannotated(int *);
    };
    void target(Foo foo) {
      foo.takeNonnull(nullptr);     // [[unsafe]]
      foo.takeNullable(nullptr);
      foo.takeUnannotated(nullptr);
    }
  )");

  // function pointer
  checkDiagnostics(R"(
    void target(void (*takeNonnull)(int * _Nonnull),
                void (*takeNullable)(int * _Nullable),
                void (*takeUnannotated)(int *)) {
      takeNonnull(nullptr);     // [[unsafe]]
      takeNullable(nullptr);
      takeUnannotated(nullptr);
    }
  )");

  // pointer to function pointer
  //
  // TODO(b/233582219): Fix false negative. Implement support for retrieving
  // parameter types from a pointer to function pointer.
  checkDiagnostics(R"(
    void target(void (**takeNonnull)(int * _Nonnull),
                void (**takeNullable)(int * _Nullable),
                void (**takeUnannotated)(int *)) {
      (*takeNonnull)(nullptr);    // false-negative
      (*takeNullable)(nullptr);
      (*takeUnannotated)(nullptr);
    }
  )");

  // function returned from function
  //
  // TODO(b/233582219): Fix false negative. Implement support for retrieving
  // parameter types for functions returned by another function.
  checkDiagnostics(R"(
    typedef void (*takeNonnullF)(int * _Nonnull);
    typedef void (*takeNullableF)(int * _Nullable);
    typedef void (*takeUnannotatedF)(int *);
    void target(takeNonnullF (*takeNonnull)(),
                takeNullableF (*takeNullable)(),
                takeUnannotatedF (*takeUnannotated)()) {
      (*takeNonnull)()(nullptr);    // false-negative
      (*takeNullable)()(nullptr);
      (*takeUnannotated)()(nullptr);
    }
  )");

  // passing a reference to a nonnull pointer
  //
  // TODO(b/233582219): Fix false negative. When the nonnull pointer is passed
  // by reference into the callee which takes a nullable parameter, its value
  // may be changed to null, making it unsafe to dereference when we return from
  // the function call. Some possible approaches for handling this case:
  // (1) Disallow passing a nonnull pointer as a nullable reference - and warn
  // at the function call.
  // (2) Assume in worst case the nonnull pointer becomes nullable after the
  // call - and warn at the dereference.
  // (3) Sacrifice soundness for reduction in noise, and skip the warning.
  checkDiagnostics(R"(
    void takeNonnullRef(int * _Nonnull &);
    void takeNullableRef(int * _Nullable &);
    void takeUnannotatedRef(int *&);
    void target(int * _Nonnull ptr_nonnull) {
      takeNonnullRef(ptr_nonnull);
      *ptr_nonnull;

      // false-negative
      takeNullableRef(ptr_nonnull);
      *ptr_nonnull;

      takeUnannotatedRef(ptr_nonnull);
      *ptr_nonnull;
    }
  )");

  // passing a reference to a nullable pointer
  checkDiagnostics(R"(
    void takeNonnullRef(int * _Nonnull &);
    void takeNullableRef(int * _Nullable &);
    void takeUnannotatedRef(int *&);
    void target(int * _Nullable ptr_nullable) {
      takeNonnullRef(ptr_nullable);    // [[unsafe]]
      *ptr_nullable;                   // [[unsafe]]

      takeNullableRef(ptr_nullable);
      *ptr_nullable;                   // [[unsafe]]

      takeUnannotatedRef(ptr_nullable);
      *ptr_nullable;                   // [[unsafe]]
    }
  )");

  // passing a reference to an unannotated pointer
  //
  // TODO(b/233582219): Fix false negative. The unannotated pointer should be
  // considered nullable if it has been used as a nullable pointer.
  checkDiagnostics(R"(
    void takeNonnullRef(int * _Nonnull &);
    void takeNullableRef(int * _Nullable &);
    void takeUnannotatedRef(int *&);
    void target(int *ptr_unannotated) {
      takeNonnullRef(ptr_unannotated);
      *ptr_unannotated;

      takeNullableRef(ptr_unannotated);
      *ptr_unannotated;  // false-negative

      takeUnannotatedRef(ptr_unannotated);
      *ptr_unannotated;
    }
  )");
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang

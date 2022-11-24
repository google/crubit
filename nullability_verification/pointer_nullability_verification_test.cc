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
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

using dataflow::Environment;
using dataflow::TypeErasedDataflowAnalysisState;
using dataflow::test::AnalysisInputs;
using dataflow::test::AnalysisOutputs;
using dataflow::test::checkDataflow;
using ::testing::ContainerEq;
using ::testing::Test;

void checkDiagnostics(llvm::StringRef SourceCode) {
  std::vector<CFGElement> Diagnostics;
  PointerNullabilityDiagnoser Diagnoser;
  ASSERT_THAT_ERROR(
      checkDataflow<PointerNullabilityAnalysis>(
          AnalysisInputs<PointerNullabilityAnalysis>(
              SourceCode, ast_matchers::hasName("target"),
              [](ASTContext &ASTCtx, Environment &) {
                return PointerNullabilityAnalysis(ASTCtx);
              })
              .withPostVisitCFG(
                  [&Diagnostics, &Diagnoser](
                      ASTContext &Ctx, const CFGElement &Elt,
                      const TypeErasedDataflowAnalysisState &State) {
                    auto EltDiagnostics =
                        Diagnoser.diagnose(&Elt, Ctx, State.Env);
                    if (EltDiagnostics.has_value()) {
                      Diagnostics.push_back(EltDiagnostics.value());
                    }
                  })
              .withASTBuildArgs({"-fsyntax-only", "-std=c++17",
                                 "-Wno-unused-value", "-Wno-nonnull"}),
          [&Diagnostics](
              const llvm::DenseMap<unsigned, std::string> &Annotations,
              const AnalysisOutputs &AnalysisData) {
            llvm::DenseSet<unsigned> ExpectedLines, ActualLines;
            for (const auto &[Line, _] : Annotations) {
              ExpectedLines.insert(Line);
            }
            auto &SrcMgr = AnalysisData.ASTCtx.getSourceManager();
            for (auto Element : Diagnostics) {
              if (Optional<CFGStmt> stmt = Element.getAs<CFGStmt>()) {
                ActualLines.insert(SrcMgr.getPresumedLineNumber(
                    stmt->getStmt()->getBeginLoc()));
              } else if (Optional<CFGInitializer> init =
                             Element.getAs<CFGInitializer>()) {
                ActualLines.insert(SrcMgr.getPresumedLineNumber(
                    init->getInitializer()->getSourceLocation()));
              } else {
                ADD_FAILURE() << "this code should not be reached";
              }
            }
            EXPECT_THAT(ActualLines, ContainerEq(ExpectedLines));
          }),
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
  checkDiagnostics(R"(
    int * _Nullable makeNullable();
    bool makeBool();
    void target() {
      bool first = true;
      while(true) {
        int *x = makeNullable();
        if (first && x == nullptr) return;
        first = false;
        *x;  // [[unsafe]]
      }
    }
  )");
}

TEST(PointerNullabilityTest, MemberExpressionOfClassTemplateInstantiation) {
  // Struct with 2 arguments with nullable second argument.
  checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {
      T0 arg0;
      T1 arg1;
    };
    void target(Struct2Arg<int* _Nonnull, double* _Nullable> p) {
      *p.arg0;
      *p.arg1;  // [[unsafe]]
    }
  )cc");

  // Struct with 5 arguments with interleaved nullable/nonnull/unknown.
  checkDiagnostics(R"cc(
    template <typename T0, typename T1, typename T2, typename T3, typename T4>
    struct Struct5Arg {
      T0 arg0;
      T1 arg1;
      T2 arg2;
      T3 arg3;
      T4 arg4;
    };
    void target(Struct5Arg<int* _Nullable, double* _Nonnull, float*,
                           double* _Nullable, int* _Nonnull>
                    p) {
      *p.arg0;  // [[unsafe]]
      *p.arg1;
      *p.arg2;
      *p.arg3;  // [[unsafe]]
      *p.arg4;
    }
  )cc");

  // Struct with interleaved int and typename arguments.
  checkDiagnostics(R"cc(
    template <typename T0, int I1, typename T2, int T3, typename T4>
    struct Struct5Arg {
      T0 arg0;
      T2 arg2;
      T4 arg4;
    };
    void target(Struct5Arg<int* _Nullable, 0, float*, 1, int* _Nullable> p) {
      *p.arg0;  // [[unsafe]]
      *p.arg2;
      *p.arg4;  // [[unsafe]]
    }
  )cc");

  // Struct template that uses another struct template in a member variable.
  checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {
      T0 arg0;
      T1 arg1;
    };

    template <typename TN0, typename TN1>
    struct Struct2ArgNested {
      Struct2Arg<TN1, Struct2Arg<TN0, TN1>>* arg0;
      Struct2Arg<TN1, Struct2Arg<TN0, TN1>>* _Nullable arg1;
    };
    void target(Struct2ArgNested<int* _Nonnull, double* _Nullable> p) {
      *p.arg0;
      *p.arg1;  // [[unsafe]]

      // TODO: The following lines currently crash at getBaseType()
      //*p.arg0->arg0; // false-positive
      //*p.arg0->arg1.arg0;
      //*p.arg0->arg1.arg1; // false-positive
    }
  )cc");
}

TEST(PointerNullabilityTest, MemberCallExpressionOfClassTemplateInstantiation) {
  // Struct with one argument initialised as _Nullable.
  checkDiagnostics(R"cc(
    template <typename T0>
    struct Struct1Arg {
      T0 getT();
      T0 *getUnknownTPtr();
      T0 *_Nullable getNullableTPtr();
      T0 *_Nonnull getNonnullTPtr();
    };
    void target(Struct1Arg<int *_Nullable> &xs) {
      *xs.getT();  // [[unsafe]]
      *xs.getUnknownTPtr();
      // **xs.getUnknownTPtr();  // false-negative
      *xs.getNullableTPtr();  // [[unsafe]]
      // **xs.getNullableTPtr();  // false-negative
      *xs.getNonnullTPtr();
      // **xs.getNonnullTPtr();  // false-negative
    }
  )cc");

  // Struct with one argument initialised as _Nonnull.
  checkDiagnostics(R"cc(
    template <typename T0>
    struct Struct1Arg {
      T0 getT();
      T0 *getUnknownTPtr();
      T0 *_Nullable getNullableTPtr();
      T0 *_Nonnull getNonnullTPtr();
    };
    void target(Struct1Arg<int *_Nonnull> &xs) {
      *xs.getT();
      *xs.getUnknownTPtr();
      // **xs.getUnknownTPtr();
      *xs.getNullableTPtr();  // [[unsafe]]
      // **xs.getNullableTPtr();
      *xs.getNonnullTPtr();
      // **xs.getNonnullTPtr();
    }
  )cc");

  // Struct with one argument initialised without annotation.
  checkDiagnostics(R"cc(
    template <typename T0>
    struct Struct1Arg {
      T0 getT();
      T0 *getUnknownTPtr();
      T0 *_Nullable getNullableTPtr();
      T0 *_Nonnull getNonnullTPtr();
    };
    void target(Struct1Arg<int *> &xs) {
      *xs.getT();
      *xs.getUnknownTPtr();
      // **xs.getUnknownTPtr();
      *xs.getNullableTPtr();  // [[unsafe]]
      // **xs.getNullableTPtr();
      *xs.getNonnullTPtr();
      // **xs.getNonnullTPtr();
    }
  )cc");

  // Struct with multiple arguments.
  checkDiagnostics(R"cc(
    template <typename T0, typename T1, typename T2>
    struct Struct3Arg {
      T0 getT0();
      T1 getT1();
      T2 getT2();
    };
    void target(Struct3Arg<int *, int *_Nonnull, int *_Nullable> &x) {
      *x.getT0();
      *x.getT1();
      *x.getT2();  // [[unsafe]]
    }
  )cc");

  // Struct with multiple arguments and methods whose return types are pointers
  // to those arguments.
  checkDiagnostics(R"cc(
    template <typename T0, typename T1, typename T2>
    struct Struct3Arg {
      T0 *getUnknownT0Ptr();
      T1 *_Nullable getNullableT1Ptr();
      T2 *_Nonnull getNonnullT2Ptr();
    };
    void target(Struct3Arg<int *, int *_Nonnull, int *_Nullable> &x) {
      *x.getUnknownT0Ptr();
      // **x.getUnknownT0Ptr();
      *x.getNullableT1Ptr();  // [[unsafe]]
      // **x.getNullableT1Ptr();
      *x.getNonnullT2Ptr();
      // **x.getNonnullT2Ptr();  // false-negative
    }
  )cc");

  // Struct with int template parameters and with type as first argument.
  checkDiagnostics(R"cc(
    template <typename T0, int I1, int I2>
    struct Struct3ArgWithInt {
      T0 getT0();
    };
    void target(Struct3ArgWithInt<int *_Nullable, 0, 1> &x) {
      *x.getT0();  // [[unsafe]]
    }
  )cc");

  // Struct with int template parameters and with type as second argument.
  checkDiagnostics(R"cc(
    template <int I0, typename T1, int I2>
    struct Struct3ArgWithInt {
      T1 getT1();
    };
    void target(Struct3ArgWithInt<0, int *_Nullable, 1> &x) {
      *x.getT1();  // [[unsafe]]
    }
  )cc");

  // Struct with interleaved int and type template parameters.
  checkDiagnostics(R"cc(
    template <int I0, typename T1, int I2, typename T3, int I4, typename T5>
    struct Struct6ArgWithInt {
      T1 getT1();
      T3 getT3();
      T5 getT5();
    };
    void target(
        Struct6ArgWithInt<0, int *_Nullable, 1, int *_Nullable, 2, int *> &x) {
      *x.getT1();  // [[unsafe]]
      *x.getT3();  // [[unsafe]]
      *x.getT5();
    }
  )cc");
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

TEST(PointerNullabilityTest, ReturnStatements) {
  // nonnull return type
  checkDiagnostics(R"(
    int * _Nonnull target() {
      return nullptr; // [[unsafe]]
    }
  )");
  checkDiagnostics(R"(
    int * _Nonnull target(int * _Nonnull ptr_nonnull) {
      return ptr_nonnull;
    }
  )");
  checkDiagnostics(R"(
    int * _Nonnull target(int * _Nullable ptr_nullable) {
      return ptr_nullable; // [[unsafe]]
    }
  )");
  checkDiagnostics(R"(
    int * _Nonnull target(int *ptr_unannotated) {
      return ptr_unannotated;
    }
  )");

  // nullable return type
  checkDiagnostics(R"(
    int * _Nullable target() {
      return nullptr;
    }
  )");
  checkDiagnostics(R"(
    int * _Nullable target(int * _Nonnull ptr_nonnull) {
      return ptr_nonnull;
    }
  )");
  checkDiagnostics(R"(
    int * _Nullable target(int * _Nullable ptr_nullable) {
      return ptr_nullable;
    }
  )");
  checkDiagnostics(R"(
    int * _Nullable target(int *ptr_unannotated) {
      return ptr_unannotated;
    }
  )");

  // unannotated return type
  checkDiagnostics(R"(
    int * target() {
      return nullptr;
    }
  )");
  checkDiagnostics(R"(
    int * target(int * _Nonnull ptr_nonnull) {
      return ptr_nonnull;
    }
  )");
  checkDiagnostics(R"(
    int * target(int * _Nullable ptr_nullable) {
      return ptr_nullable;
    }
  )");
  checkDiagnostics(R"(
    int * target(int *ptr_unannotated) {
      return ptr_unannotated;
    }
  )");

  // multiple return statements
  checkDiagnostics(R"(
    int * _Nonnull target(bool b, int * _Nonnull ptr_nonnull) {
      if (b) {
        return nullptr; // [[unsafe]]
      }
      return ptr_nonnull;
    }
  )");
  checkDiagnostics(R"(
    int * _Nonnull target(int * _Nullable ptr_nullable,
                          int * _Nonnull ptr_nonnull) {
      if (ptr_nullable) {
        return ptr_nullable;
      }
      return ptr_nonnull;
    }
  )");
  checkDiagnostics(R"(
    int * _Nonnull target(int * _Nullable ptr_nullable_1,
                          int * _Nullable ptr_nullable_2) {
      if (ptr_nullable_1) {
        return ptr_nullable_2; // [[unsafe]]
      }
      return ptr_nullable_1; // [[unsafe]]
    }
  )");

  // return result of merging 2 pointer values
  checkDiagnostics(R"(
    int * _Nonnull target(bool b, int i) {
      int *ptr;
      if (b) {
        ptr = &i;
      } else {
        ptr = nullptr;
      }
      return ptr; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, ConstructExpr) {
  // Constructor call assigned to local variable.
  checkDiagnostics(R"(
    struct TakeNonnull {
      explicit TakeNonnull(int * _Nonnull) {}
    };
    struct TakeNullable {
      explicit TakeNullable(int * _Nullable) {}
    };
    struct TakeUnannotated {
      explicit TakeUnannotated(int *) {}
    };
    int * _Nonnull makeNonnull();
    int * _Nullable makeNullable();
    int *makeUnannotated();
    void target() {
      auto NN1 = TakeNonnull(makeNonnull());
      auto NN2 = TakeNonnull(makeNullable());        // [[unsafe]]
      auto NN3 = TakeNonnull(makeUnannotated());

      auto NB1 = TakeNullable(makeNonnull());
      auto NB2 = TakeNullable(makeNullable());
      auto NB3 = TakeNullable(makeUnannotated());

      auto UN1 = TakeUnannotated(makeNonnull());
      auto UN2 = TakeUnannotated(makeNullable());
      auto UN3 = TakeUnannotated(makeUnannotated());
    }
  )");

  // Constructor call in a base initializer.
  checkDiagnostics(R"(
    struct TakeNonnull {
      explicit TakeNonnull(int * _Nonnull);
    };
    struct target: TakeNonnull {
      target(int * _Nullable ptr_nullable): TakeNonnull(ptr_nullable) {} // [[unsafe]]
    };
  )");

  // Call to a delegating constructor
  checkDiagnostics(R"(
    int * _Nullable makeNullable();
    struct target {
      target(int * _Nonnull);
      target(): target(makeNullable()) {} // [[unsafe]]
    };
  )");
}

TEST(PointerNullabilityTest, ConstructorMemberInitializer) {
  checkDiagnostics(R"(
    int * _Nullable makeNullable();
    struct target {
      int * _Nonnull ptr_nonnull;
      int * _Nullable ptr_nullable;
      int * ptr_unannotated;
      target(): ptr_nonnull(makeNullable()), // [[unsafe]]
                ptr_nullable(makeNullable()),
                ptr_unannotated(makeNullable()) {}
    };
  )");

  checkDiagnostics(R"(
    int * _Nonnull makeNonnull();
    struct target {
      int * _Nonnull ptr_nonnull;
      int * _Nullable ptr_nullable;
      int * ptr_unannotated;
      target(): ptr_nonnull(makeNonnull()),
                ptr_nullable(makeNonnull()),
                ptr_unannotated(makeNonnull()) {}
    };
  )");

  checkDiagnostics(R"(
    int *makeUnannotated();
    struct target {
      int * _Nonnull ptr_nonnull;
      int * _Nullable ptr_nullable;
      int * ptr_unannotated;
      target(): ptr_nonnull(makeUnannotated()),
                ptr_nullable(makeUnannotated()),
                ptr_unannotated(makeUnannotated()) {}
    };
  )");
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang

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
using dataflow::TransferStateForDiagnostics;
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
              .withPostVisitCFG([&Diagnostics, &Diagnoser](
                                    ASTContext &Ctx, const CFGElement &Elt,
                                    const TransferStateForDiagnostics<
                                        PointerNullabilityLattice> &State) {
                auto EltDiagnostics = Diagnoser.diagnose(&Elt, Ctx, State);
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

TEST(PointerNullabilityTest, DoubleDereference) {
  checkDiagnostics(R"cc(
    void target(int** p) {
      *p;
      **p;
    }
  )cc");

  checkDiagnostics(R"cc(
    void target(int** _Nonnull p) {
      *p;
      **p;
    }
  )cc");

  checkDiagnostics(R"cc(
    void target(int* _Nonnull* p) {
      *p;
      **p;
    }
  )cc");

  checkDiagnostics(R"cc(
    void target(int* _Nonnull* _Nonnull p) {
      *p;
      **p;
    }
  )cc");

  checkDiagnostics(R"cc(
    void target(int** _Nullable p) {
      *p;   // [[unsafe]]
      **p;  // [[unsafe]]
    }
  )cc");

  checkDiagnostics(R"cc(
    void target(int* _Nullable* p) {
      *p;
      **p;  // [[unsafe]]
    }
  )cc");

  checkDiagnostics(R"cc(
    void target(int* _Nullable* _Nullable p) {
      *p;   // [[unsafe]]
      **p;  // [[unsafe]]
    }
  )cc");

  checkDiagnostics(R"cc(
    void target(int* _Nullable* _Nonnull p) {
      *p;
      **p;  // [[unsafe]]
    }
  )cc");

  checkDiagnostics(R"cc(
    void target(int* _Nonnull* _Nullable p) {
      *p;   // [[unsafe]]
      **p;  // [[unsafe]]
    }
  )cc");
}

// TODO: Fix false negatives.
TEST(PointerNullabilityTest, ClassTemplateInstantiation) {
  // Class template specialization with one argument initialised as _Nullable.
  // We test types that contain both nullability that is substituted into the
  // template argument and nullability that is spelt inside the template. That
  // is, we should be able to accurately store nullabilities from different
  // sources in a single nullability vector.
  checkDiagnostics(R"cc(
    template <typename T0>
    struct Struct1Arg {
      T0 arg0;
      T0 *unknownTPtr;
      T0 *_Nullable nullableTPtr;
      T0 *_Nonnull nonnullTPtr;

      T0 getT();
      T0 *getUnknownTPtr();
      T0 *_Nullable getNullableTPtr();
      T0 *_Nonnull getNonnullTPtr();
    };
    void target(Struct1Arg<int *_Nullable> p) {
      *p.arg0;  // [[unsafe]]
      *p.unknownTPtr;
      *p.nullableTPtr;  // [[unsafe]]
      *p.nonnullTPtr;
      **p.unknownTPtr;   // TODO: fix false negative.
      **p.nullableTPtr;  // [[unsafe]]
      **p.nonnullTPtr;   // TODO: fix false negative.

      *p.getT();  // [[unsafe]]
      *p.getUnknownTPtr();
      *p.getNullableTPtr();  // [[unsafe]]
      *p.getNonnullTPtr();
      **p.getUnknownTPtr();   // TODO: fix false negative.
      **p.getNullableTPtr();  // [[unsafe]]
      **p.getNonnullTPtr();   // TODO: fix false negative.
    }
  )cc");

  // Class template specialization with one argument initialised as _Nonnull.
  checkDiagnostics(R"cc(
    template <typename T0>
    struct Struct1Arg {
      T0 arg0;
      T0 *unknownTPtr;
      T0 *_Nullable nullableTPtr;
      T0 *_Nonnull nonnullTPtr;

      T0 getT();
      T0 *getUnknownTPtr();
      T0 *_Nullable getNullableTPtr();
      T0 *_Nonnull getNonnullTPtr();
    };

    void target(Struct1Arg<int *_Nonnull> p) {
      *p.getT();
      *p.getUnknownTPtr();
      *p.getNullableTPtr();  // [[unsafe]]
      *p.getNonnullTPtr();
      **p.getUnknownTPtr();
      **p.getNullableTPtr();  // [[unsafe]]
      **p.getNonnullTPtr();

      *p.arg0;
      *p.unknownTPtr;
      *p.nullableTPtr;  // [[unsafe]]
      *p.nonnullTPtr;
      **p.unknownTPtr;
      **p.nullableTPtr;  // [[unsafe]]
      **p.nonnullTPtr;
    }
  )cc");

  // Class template specialization with one argument initialised without
  // nullability annotation.
  checkDiagnostics(R"cc(
    template <typename T0>
    struct Struct1Arg {
      T0 arg0;
      T0 *unknownTPtr;
      T0 *_Nullable nullableTPtr;
      T0 *_Nonnull nonnullTPtr;
      T0 getT();

      T0 *getUnknownTPtr();
      T0 *_Nullable getNullableTPtr();
      T0 *_Nonnull getNonnullTPtr();
    };

    void target(Struct1Arg<int *> p) {
      *p.getT();
      *p.getUnknownTPtr();
      *p.getNullableTPtr();  // [[unsafe]]
      *p.getNonnullTPtr();
      **p.getUnknownTPtr();
      **p.getNullableTPtr();  // [[unasfe]]
      **p.getNonnullTPtr();

      *p.arg0;
      *p.unknownTPtr;
      *p.nullableTPtr;  // [[unsafe]]
      *p.nonnullTPtr;
      **p.unknownTPtr;
      **p.nullableTPtr;  // [[unsafe]]
      **p.nonnullTPtr;
    }
  )cc");

  // Class template specialization with two arguments, whose second argument is
  // initialized as nullable.
  checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {
      T0 arg0;
      T0 *unknownT0Ptr;
      T0 *_Nullable nullableT0Ptr;
      T0 *_Nonnull nonnullT0Ptr;

      T1 arg1;
      T1 *unknownT1Ptr;
      T1 *_Nullable nullableT1Ptr;
      T1 *_Nonnull nonnullT1Ptr;

      T0 getT0();
      T0 *getUnknownT0Ptr();
      T0 *_Nullable getNullableT0Ptr();
      T0 *_Nonnull getNonnullT0Ptr();

      T1 getT1();
      T1 *getUnknownT1Ptr();
      T1 *_Nullable getNullableT1Ptr();
      T1 *_Nonnull getNonnullT1Ptr();
    };

    void target(Struct2Arg<int *_Nonnull, double *_Nullable> p) {
      *p.arg0;
      *p.arg1;  // [[unsafe]]

      *p.unknownT0Ptr;
      *p.nullableT0Ptr;  // [[unsafe]]
      *p.nonnullT0Ptr;

      *p.unknownT1Ptr;
      *p.nullableT1Ptr;  // [[unsafe]]
      *p.nonnullT1Ptr;

      *p.getUnknownT0Ptr();
      *p.getNullableT0Ptr();  // [[unsafe]]
      *p.getNonnullT0Ptr();

      *p.getUnknownT1Ptr();
      *p.getNullableT1Ptr();  // [[unsafe]]
      *p.getNonnullT1Ptr();
    }
  )cc");

  // Class template specialization with 5 arguments with interleaved
  // nullable/nonnull/unknown.
  checkDiagnostics(R"cc(
    template <typename T0, typename T1, typename T2, typename T3, typename T4>
    struct Struct5Arg {
      T0 arg0;
      T1 arg1;
      T2 arg2;
      T3 arg3;
      T4 arg4;

      T0 getT0();
      T1 getT1();
      T2 getT2();
      T3 getT3();
      T4 getT4();
    };
    void target(Struct5Arg<int* _Nullable, double* _Nonnull, float*,
                           double* _Nullable, int* _Nonnull>
                    p) {
      *p.arg0;  // [[unsafe]]
      *p.arg1;
      *p.arg2;
      *p.arg3;  // [[unsafe]]
      *p.arg4;

      *p.getT0();  // [[unsafe]]
      *p.getT1();
      *p.getT2();
      *p.getT3();  // [[unsafe]]
      *p.getT4();
    }
  )cc");

  // Class template specialization with 5 arguments with interleaved
  // nullable/nonnull/unknown/const.
  checkDiagnostics(R"cc(
    template <typename T0, typename T1, typename T2, typename T3, typename T4>
    struct Struct5Arg {
      T0 arg0;
      T1 arg1;
      T2 arg2;
      T3 arg3;
      T4 arg4;

      T0 getT0();
      T1 getT1();
      T2 getT2();
      T3 getT3();
      T4 getT4();
    };
    void target(Struct5Arg<int* const _Nullable, double const* const _Nonnull,
                           float*, double const* const _Nullable, int* _Nonnull>
                    p) {
      *p.arg0;  // [[unsafe]]
      *p.arg1;
      *p.arg2;
      *p.arg3;  // [[unsafe]]
      *p.arg4;

      *p.getT0();  // TODO: fix false negative.
      *p.getT1();
      *p.getT2();
      *p.getT3();  // TODO: fix false negative.
      *p.getT4();
    }
  )cc");

  // Class template specialization with interleaved int and type template
  // parameters.
  checkDiagnostics(R"cc(
    template <int I0, typename T1, int I2, typename T3, int I4, typename T5>
    struct Struct6ArgWithInt {
      T1 arg1;
      T3 arg3;
      T5 arg5;

      T1 getT1();
      T3 getT3();
      T5 getT5();
    };
    void target(
        Struct6ArgWithInt<0, int *_Nullable, 1, int *_Nullable, 2, int *> &x) {
      *x.arg1;  // [[unsafe]]
      *x.arg3;  // [[unsafe]]
      *x.arg5;

      *x.getT1();  // [[unsafe]]
      *x.getT3();  // [[unsafe]]
      *x.getT5();
    }
  )cc");
}

// TODO: Fix false positives and false negatives.
TEST(PointerNullabilityTest,
     ClassTemplateInstantiationWithStructsAsParameters) {
  checkDiagnostics(R"cc(
    struct Struct3IntPtrs {
      int* unknown;
      int* _Nullable nullable;
      int* _Nonnull nonnull;

      int* getUnknown();
      int* _Nullable getNullable();
      int* _Nonnull getNonnull();
    };

    template <typename T0>
    struct Struct1Arg {
      T0 arg0;
      T0 getT0();
    };

    void target(Struct1Arg<Struct3IntPtrs> p) {
      *p.arg0.unknown;
      *p.arg0.nullable;  // [[unsafe]]
      *p.arg0.nonnull;

      *p.arg0.getUnknown();
      *p.arg0.getNullable();  // [[unsafe]]
      *p.arg0.getNonnull();

      *p.getT0().unknown;   // [[unsafe]] TODO: fix false positive.
      *p.getT0().nullable;  // [[unsafe]]
      *p.getT0().nonnull;   // [[unsafe]] TODO: fix false positive.

      *p.getT0().getUnknown();
      *p.getT0().getNullable();  // [[unsafe]]
      *p.getT0().getNonnull();
    }
  )cc");

  checkDiagnostics(R"cc(
    struct Struct1UnknownArg {
      char* unknownChar;

      char* getUnknownChar();
    };

    struct Struct1NullableArg {
      char* _Nullable nullableChar;

      char* _Nullable getNullableChar();
    };

    struct Struct1NonnullArg {
      char* _Nonnull nonnullChar;

      char* _Nonnull getNonnullChar();
    };

    struct StructLotsOfArgs {
      int num;
      long long* unknownLongLong;
      double* _Nullable nullableDouble;
      float* _Nonnull nonnullFloat;
      short* unknownShort;
      unsigned int* _Nullable nullableUInt;
      bool* _Nullable nullableBool;

      long long* getUnknownLongLong();
      double* _Nullable getNullableDouble();
      float* _Nonnull getNonnullFloat();
      short* getUnknownShort();
      unsigned int* _Nullable getNullableUInt();
      bool* _Nullable getNullableBool();
    };

    template <typename T0, typename T1, typename T2, typename T3>
    struct Struct4Arg {
      T0 arg0;
      T1 arg1;
      T2 arg2;
      T3 arg3;

      T0 getT0();
      T1 getT1();
      T2 getT2();
      T3 getT3();
    };

    void target(Struct4Arg<Struct1UnknownArg, Struct1NullableArg,
                           Struct1NonnullArg, StructLotsOfArgs>
                    p) {
      *p.arg0.unknownChar;
      *p.arg1.nullableChar;  // [[unsafe]]
      *p.arg2.nonnullChar;
      *p.arg3.unknownLongLong;
      *p.arg3.nullableDouble;  // [[unsafe]]
      *p.arg3.nonnullFloat;
      *p.arg3.unknownShort;
      *p.arg3.nullableUInt;  // [[unsafe]]
      *p.arg3.nullableBool;  // [[unsafe]]

      *p.arg0.getUnknownChar();
      *p.arg1.getNullableChar();  // [[unsafe]]
      *p.arg2.getNonnullChar();
      *p.arg3.getUnknownLongLong();
      *p.arg3.getNullableDouble();  // [[unsafe]]
      *p.arg3.getNonnullFloat();
      *p.arg3.getUnknownShort();
      *p.arg3.getNullableUInt();  // [[unsafe]]
      *p.arg3.getNullableBool();  // [[unsafe]]

      *p.getT0().unknownChar;      // [[unsafe]] TODO: fix false positive.
      *p.getT1().nullableChar;     // [[unsafe]]
      *p.getT2().nonnullChar;      // [[unsafe]] TODO: fix false positive.
      *p.getT3().unknownLongLong;  // [[unsafe]] TODO: fix false positive.
      *p.getT3().nullableDouble;   // [[unsafe]]
      *p.getT3().nonnullFloat;     // [[unsafe]] TODO: fix false positive.
      *p.getT3().unknownShort;     // [[unsafe]] TODO: fix false positive.
      *p.getT3().nullableUInt;     // [[unsafe]]
      *p.getT3().nullableBool;     // [[unsafe]]

      *p.getT0().getUnknownChar();
      *p.getT1().getNullableChar();  // [[unsafe]]
      *p.getT2().getNonnullChar();
      *p.getT3().getUnknownLongLong();
      *p.getT3().getNullableDouble();  // [[unsafe]]
      *p.getT3().getNonnullFloat();
      *p.getT3().getUnknownShort();
      *p.getT3().getNullableUInt();  // [[unsafe]]
      *p.getT3().getNullableBool();  // [[unsafe]]
    }
  )cc");

  // With const arguments and int template parameter.
  checkDiagnostics(R"cc(
    struct Struct1UnknownArg {
      char* const constUnknownChar;
      char const* unknownConstChar;
      char const* const constUnknownConstChar;

      char* const getConstUnknownChar();
      char const* getUnknownConstChar();
      char const* const getConstUnknownConstChar();
    };

    struct Struct1NullableArg {
      char* const _Nullable constNullableChar;
      char const* _Nullable nullableConstChar;
      char const* const _Nullable constNullableConstChar;

      char* const _Nullable getConstNullableChar();
      char const* _Nullable getNullableConstChar();
      char* const* _Nullable getConstNullableConstChar();
    };

    struct Struct1NonnullArg {
      char* const _Nonnull constNonnullChar;
      char const* _Nonnull nonnullConstChar;
      char const* const _Nonnull constNonnullConstChar;

      char* const _Nonnull getConstNonnullChar();
      char const* _Nonnull getNonnullConstChar();
      char const* const _Nonnull getConstNonnullConstChar();
    };

    template <int I0, typename T1, typename T2, typename T3>
    struct Struct4Arg {
      T1 arg1;
      T2 arg2;
      T3 arg3;

      T1 getT1();
      T2 getT2();
      T3 getT3();
    };

    void target(
        Struct4Arg<4, Struct1UnknownArg, Struct1NullableArg, Struct1NonnullArg>
            p) {
      *p.arg1.constUnknownChar;
      *p.arg1.unknownConstChar;
      *p.arg1.constUnknownConstChar;
      *p.arg2.constNullableChar;       // [[unsafe]]
      *p.arg2.nullableConstChar;       // [[unsafe]]
      *p.arg2.constNullableConstChar;  // [[unsafe]]
      *p.arg3.constNonnullChar;
      *p.arg3.nonnullConstChar;
      *p.arg3.constNonnullConstChar;

      *p.arg1.getConstUnknownChar();
      *p.arg1.getUnknownConstChar();
      *p.arg1.getConstUnknownConstChar();
      *p.arg2.getConstNullableChar();       // TODO: fix false negative.
      *p.arg2.getNullableConstChar();       // [[unsafe]]
      *p.arg2.getConstNullableConstChar();  // [[unsafe]]
      *p.arg3.getConstNonnullChar();
      *p.arg3.getNonnullConstChar();
      *p.arg3.getConstNonnullConstChar();

      *p.getT1().constUnknownChar;       // [[unsafe]] TODO: fix false positive.
      *p.getT1().unknownConstChar;       // [[unsafe]] TODO: fix false positive.
      *p.getT1().constUnknownConstChar;  // [[unsafe]] TODO: fix false positive.
      *p.getT2().constNullableChar;      // [[unsafe]]
      *p.getT2().nullableConstChar;      // [[unsafe]]
      *p.getT2().constNullableConstChar;  // [[unsafe]]
      *p.getT3().constNonnullChar;       // [[unsafe]] TODO: fix false positive.
      *p.getT3().nonnullConstChar;       // [[unsafe]] TODO: fix false positive.
      *p.getT3().constNonnullConstChar;  // [[unsafe]] TODO: fix false positive.

      *p.getT1().getConstUnknownChar();
      *p.getT1().getUnknownConstChar();
      *p.getT1().getConstUnknownConstChar();
      *p.getT2().getConstNullableChar();       // TODO: fix false negative.
      *p.getT2().getNullableConstChar();       // [[unsafe]]
      *p.getT2().getConstNullableConstChar();  // [[unsafe]]
      *p.getT3().getConstNonnullChar();
      *p.getT3().getNonnullConstChar();
      *p.getT3().getConstNonnullConstChar();
    }
  )cc");
}

// TODO: Fix false negatives.
TEST(PointerNullabilityTest, MemberFunctionTemplateOfConcreteStruct) {
  checkDiagnostics(R"cc(
    struct S {
      template <typename T0>
      T0 getT0();
    };

    void target(S p) {
      *p.getT0<int *>();
      *p.getT0<int *_Nonnull>();
      *p.getT0<int *_Nullable>();  // TODO: fix false negative.

      *p.getT0<int const *>();
      *p.getT0<int *const>();
      *p.getT0<int const *const>();
      *p.getT0<int const *_Nonnull>();
      *p.getT0<int *const _Nonnull>();
      *p.getT0<int const *const _Nonnull>();
      *p.getT0<int const *_Nullable>();        // TODO: fix false negative.
      *p.getT0<int *const _Nullable>();        // TODO: fix false negative.
      *p.getT0<int const *const _Nullable>();  // TODO: fix false negative.
    }
  )cc");

  checkDiagnostics(R"cc(
    struct S {
      template <int I0, typename T1, int I2>
      T1 getT1();
    };

    void target(S p) {
      *p.getT1<0, int *, 1>();
      *p.getT1<2147483647, int *_Nonnull, -2147483647>();
      *p.getT1<4, int *_Nullable, 4>();  // TODO: fix false negative.
    }
  )cc");
}

TEST(PointerNullabilityTest, MemberFunctionTemplateOfTemplateStruct) {
  checkDiagnostics(R"cc(
    template <typename T0>
    struct S {
      template <typename TN1>
      TN1 getTN1();
    };

    void target(S<int> p) {
      *p.getTN1<int *>();
      *p.getTN1<int *_Nonnull>();
      *p.getTN1<int *_Nullable>();  // TODO: fix false negative.

      *p.getTN1<int const *>();
      *p.getTN1<int *const>();
      *p.getTN1<int const *const>();
      *p.getTN1<int const *_Nonnull>();
      *p.getTN1<int *const _Nonnull>();
      *p.getTN1<int const *const _Nonnull>();
      *p.getTN1<int const *_Nullable>();        // TODO: fix false negative.
      *p.getTN1<int *const _Nullable>();        // TODO: fix false negative.
      *p.getTN1<int const *const _Nullable>();  // TODO: fix false negative.
    }
  )cc");

  checkDiagnostics(R"cc(
    template <typename T0>
    struct S {
      template <int IN1, typename TN2, int IN3>
      TN2 getTN2();
    };

    void target(S<int> p) {
      // *p.getTN2<0, int *, 1>(); // TODO: fix crash
      // *p.getTN2<2147483647, int * _Nonnull, -2147483647>(); // TODO: fix
      // crash *p.getTN2<4, int * _Nullable, 4>(); // TODO: fix crash
    }
  )cc");
}

// TODO: Fix false positives.
TEST(PointerNullabilityTest,
     ClassTemplateInstantiationWithTemplateStructsAsParameters) {
  // Class template with another class template as parameter
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

      *p.arg0->arg0;
      *p.arg0->arg1.arg0;
      *p.arg0->arg1.arg1;
    }
  )cc");

  // Class template with itself as parameter
  checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {
      T0 arg0;
      T1 arg1;
    };

    void target(Struct2Arg<Struct2Arg<int*, int* _Nullable>, int* _Nonnull> p) {
      *p.arg0.arg0;
      *p.arg0.arg1;  // [[unsafe]]
      *p.arg1;
    }
  )cc");

  checkDiagnostics(R"cc(
    template <typename T0, typename T1, typename T2, typename T3, typename T4>
    struct Struct5Arg {
      T0 arg0;
      T1 arg1;
      T2 arg2;
      T3 arg3;
      T4 arg4;
    };

    void
    target(Struct5Arg<
           Struct5Arg<
               Struct5Arg<Struct5Arg<int* _Nullable, int* _Nonnull,
                                     float* _Nullable, int*, double* _Nullable>,
                          int, int, int, int* _Nullable>,
               int, int* _Nullable, int, int>,
           int, int* _Nullable, int* _Nonnull, int>
               p) {
      *p.arg0.arg0.arg0.arg0;  // [[unsafe]]
      *p.arg0.arg0.arg0.arg1;  // [[unsafe]] TODO: fix false positive.
      *p.arg0.arg0.arg0.arg2;  // [[unsafe]]
      *p.arg0.arg0.arg0.arg3;  // [[unsafe]] TODO: fix false positive.
      *p.arg0.arg0.arg0.arg4;  // [[unsafe]]
      *p.arg0.arg0.arg4;       // [[unsafe]]
      *p.arg0.arg2;            // [[unsafe]]
      *p.arg2;                 // [[unsafe]]
      *p.arg3;
    }
  )cc");

  checkDiagnostics(R"cc(
    template <int I0, typename T1, typename T2, typename T3, int I4,
              typename T5, typename T6>
    struct Struct7ArgWithInt {
      T1 arg1;
      T2 arg2;
      T3 arg3;
      T5 arg5;
      T6 arg6;
    };

    void target(Struct7ArgWithInt<
                0,
                Struct7ArgWithInt<
                    2147483647,
                    Struct7ArgWithInt<
                        0,
                        Struct7ArgWithInt<-2147483647, int* _Nullable,
                                          int* _Nonnull, float* _Nullable, 0,
                                          int*, double* _Nullable>,
                        int, int, 1, int, int* _Nullable>,
                    int, int* _Nullable, 2147483647, int, int>,
                int, int* _Nullable, 2, int* _Nonnull, int>
                    p) {
      *p.arg1.arg1.arg1.arg1;  // [[unsafe]]
      *p.arg1.arg1.arg1.arg2;  // [[unsafe]] TODO: fix false positive.
      *p.arg1.arg1.arg1.arg3;  // [[unsafe]]
      *p.arg1.arg1.arg1.arg5;  // [[unsafe]] TODO: fix false positive.
      *p.arg1.arg1.arg1.arg6;  // [[unsafe]]
      *p.arg1.arg1.arg6;       // [[unsafe]]
      *p.arg1.arg3;            // [[unsafe]]
      *p.arg3;                 // [[unsafe]]
      *p.arg5;
    }
  )cc");
}

TEST(PointerNullabilityTest,
     ClassTemplateInstantiationWithPointersToStructsAsParameters) {
  checkDiagnostics(R"cc(
    struct Struct3IntPtrs {
      int* unknown;
      int* _Nullable nullable;
      int* _Nonnull nonnull;

      int* getUnknown();
      int* _Nullable getNullable();
      int* _Nonnull getNonnull();
    };

    template <typename T0>
    struct Struct1Arg {
      T0 arg0;

      T0 getT0();
    };

    void target(Struct1Arg<Struct3IntPtrs*> p) {
      *p.arg0->unknown;
      *p.arg0->nullable;  // [[unsafe]]
      *p.arg0->nonnull;

      *p.arg0->getUnknown();
      *p.arg0->getNullable();  // [[unsafe]]
      *p.arg0->getNonnull();

      *p.getT0()->unknown;
      *p.getT0()->nullable;  // [[unsafe]]
      *p.getT0()->nonnull;

      *p.getT0()->getUnknown();
      *p.getT0()->getNullable();  // [[unsafe]]
      *p.getT0()->getNonnull();
    }
  )cc");

  checkDiagnostics(R"cc(
    struct Struct3IntPtrs {
      int* unknown;
      int* _Nullable nullable;
      int* _Nonnull nonnull;

      int* getUnknown();
      int* _Nullable getNullable();
      int* _Nonnull getNonnull();
    };

    template <typename T0>
    struct Struct1Arg {
      T0 arg0;

      T0 getT0();
    };

    void target(Struct1Arg<Struct3IntPtrs* _Nullable> p) {
      *p.arg0->unknown;   // [[unsafe]]
      *p.arg0->nullable;  // [[unsafe]]
      *p.arg0->nonnull;   // [[unsafe]]

      *p.arg0->getUnknown();   // [[unsafe]]
      *p.arg0->getNullable();  // [[unsafe]]
      *p.arg0->getNonnull();   // [[unsafe]]

      *p.getT0()->unknown;   // [[unsafe]]
      *p.getT0()->nullable;  // [[unsafe]]
      *p.getT0()->nonnull;   // [[unsafe]]

      *p.getT0()->getUnknown();   // [[unsafe]]
      *p.getT0()->getNullable();  // [[unsafe]]
      *p.getT0()->getNonnull();   // [[unsafe]]
    }
  )cc");

  checkDiagnostics(R"cc(
    struct Struct3IntPtrs {
      int* unknown;
      int* _Nullable nullable;
      int* _Nonnull nonnull;

      int* getUnknown();
      int* _Nullable getNullable();
      int* _Nonnull getNonnull();
    };

    template <typename T0>
    struct Struct1Arg {
      T0 arg0;

      T0 getT0();
    };

    void target(Struct1Arg<Struct3IntPtrs* _Nonnull> p) {
      *p.arg0->unknown;
      *p.arg0->nullable;  // [[unsafe]]
      *p.arg0->nonnull;

      *p.arg0->getUnknown();
      *p.arg0->getNullable();  // [[unsafe]]
      *p.arg0->getNonnull();

      *p.getT0()->unknown;
      *p.getT0()->nullable;  // [[unsafe]]
      *p.getT0()->nonnull;

      *p.getT0()->getUnknown();
      *p.getT0()->getNullable();  // [[unsafe]]
      *p.getT0()->getNonnull();
    }
  )cc");

  checkDiagnostics(R"cc(
    struct Struct3IntPtrs {
      int* unknown;
      int* _Nullable nullable;
      int* _Nonnull nonnull;

      int* getUnknown();
      int* _Nullable getNullable();
      int* _Nonnull getNonnull();
    };

    template <int I0, typename T1>
    struct Struct2Arg {
      T1 arg1;

      T1 getT1();
    };

    void target(Struct2Arg<0, Struct3IntPtrs*> p) {
      *p.arg1->unknown;
      *p.arg1->nullable;  // [[unsafe]]
      *p.arg1->nonnull;

      *p.arg1->getUnknown();
      *p.arg1->getNullable();  // [[unsafe]]
      *p.arg1->getNonnull();

      *p.getT1()->unknown;
      *p.getT1()->nullable;  // [[unsafe]]
      *p.getT1()->nonnull;
      *p.getT1()->getUnknown();
      *p.getT1()->getNullable();  // [[unsafe]]
      *p.getT1()->getNonnull();
    }
  )cc");
}

TEST(PointerNullabilityTest,
     ClassTemplateInstantiationWithPointersToTemplateStructsAsParameters) {
  checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {
      T0 arg0;
      T1 arg1;

      T0 getT0();
      T1 getT1();
    };

    void target(Struct2Arg<Struct2Arg<int *, int *_Nullable> *_Nullable,
                           Struct2Arg<int, int *> *_Nonnull>
                    p) {
      *p.arg0;        // [[unsafe]]
      *p.arg0->arg0;  // [[unsafe]]
      *p.arg0->arg1;  // [[unsafe]]
      *p.arg1;
      *p.arg1->arg1;

      *p.arg0->getT0();  // [[unsafe]]
      *p.arg0->getT1();  // [[unsafe]]
      *p.arg1->getT1();

      *p.getT0();        // [[unsafe]]
      *p.getT0()->arg0;  // [[unsafe]]
      *p.getT0()->arg1;  // [[unsafe]]
      *p.getT1();
      *p.getT1()->arg1;

      *p.getT0()->getT0();  // [[unsafe]]
      *p.getT0()->getT1();  // [[unsafe]]
      *p.getT1()->getT1();
    }
  )cc");

  checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct StructNonnullUnknown {
      T0 nonnull;
      T1 unknown;

      T0 getNonnull();
      T1 getUnknown();
    };

    template <typename T0, typename T1>
    struct StructNonnullNullable {
      T0 nonnull;
      T1 nullable;

      T0 getNonnull();
      T1 getNullable();
    };

    template <typename T0, typename T1>
    struct StructNullableNonnull {
      T0 nullable;
      T1 nonnull;

      T0 getNullable();
      T1 getNonnull();
    };

    template <typename T0, typename T1>
    struct StructNullableNullable {
      T0 nullable0;
      T1 nullable1;

      T0 getNullable0();
      T1 getNullable1();
    };

    template <typename T0, typename T1>
    struct StructNullableUnknown {
      T0 nullable;
      T1 unknown;

      T0 getNullable();
      T1 getUnknown();
    };

    template <typename T0, typename T1>
    struct StructUnknownNullable {
      T0 unknown;
      T1 nullable;

      T0 getUnknown();
      T1 getNullable();
    };

    void
    target(StructNonnullUnknown<
           StructNonnullNullable<
               StructNullableNullable<int* _Nullable, int* _Nullable>* _Nonnull,
               StructUnknownNullable<int*,
                                     int* _Nullable>* _Nullable>* _Nonnull,
           StructUnknownNullable<
               StructUnknownNullable<int*, int* _Nullable>*,
               StructNullableNonnull<int* _Nullable,
                                     int* _Nonnull>* _Nullable>*>
               p) {
      *p.nonnull;
      *p.nonnull->nonnull;
      *p.nonnull->nonnull->nullable0;  // TODO: fix false negative.
      *p.nonnull->nonnull->nullable1;  // TODO: fix false negative.
      *p.nonnull->nullable;            // TODO: fix false negative.
      *p.nonnull->nullable->unknown;   // TODO: fix false negative.
      *p.nonnull->nullable->nullable;  // TODO: fix false negative.
      *p.unknown->unknown;
      *p.unknown->unknown->unknown;
      *p.unknown->unknown->nullable;  // TODO: fix false negative.
      *p.unknown;
      *p.unknown->nullable;            // TODO: fix false negative.
      *p.unknown->nullable->nullable;  // TODO: fix false negative.
      *p.unknown->nullable->nonnull;   // TODO: fix false negative.

      *p.nonnull->getNonnull();
      *p.nonnull->getNonnull()->nullable0;  // TODO: fix false negative.
      *p.nonnull->getNonnull()->nullable1;  // TODO: fix false negative.
      *p.nonnull->getNullable();
      *p.nonnull->getNullable()->unknown;   // TODO: fix false negative.
      *p.nonnull->getNullable()->nullable;  // TODO: fix false negative.
      *p.unknown->getUnknown();
      *p.unknown->getUnknown()->unknown;
      *p.unknown->getUnknown()->nullable;   // TODO: fix false negative.
      *p.unknown->getNullable();            // TODO: fix false negative.
      *p.unknown->getNullable()->nullable;  // TODO: fix false negative.
      *p.unknown->getNullable()->nonnull;   // TODO: fix false negative.

      *p.nonnull->getNonnull()->getNullable0();  // TODO: fix false negative.
      *p.nonnull->getNonnull()->getNullable1();  // TODO: fix false negative.
      *p.nonnull->getNullable()->getUnknown();   // TODO: fix false negative.
      *p.nonnull->getNullable()->getNullable();  // TODO: fix false negative.
      *p.unknown->getUnknown()->getUnknown();
      *p.unknown->getUnknown()->getNullable();   // TODO: fix false negative.
      *p.unknown->getNullable()->getNullable();  // TODO: fix false negative.
      *p.unknown->getNullable()->getNonnull();   // TODO: fix false negative.

      *p.nonnull->nonnull->getNullable0();  // TODO: fix false negative.
      *p.nonnull->nonnull->getNullable1();  // TODO: fix false negative.
      *p.nonnull->nullable->getUnknown();   // TODO: fix false negative.
      *p.nonnull->nullable->getNullable();  // TODO: fix false negative.
      *p.unknown->unknown->getUnknown();
      *p.unknown->unknown->getNullable();   // TODO: fix false negative.
      *p.unknown->nullable->getNullable();  // TODO: fix false negative.
      *p.unknown->nullable->getNonnull();   // TODO: fix false negative.

      *p.getNonnull();
      *p.getNonnull()->nonnull;
      *p.getNonnull()->nonnull->nullable0;  // TODO: fix false negative.
      *p.getNonnull()->nonnull->nullable1;  // TODO: fix false negative.
      *p.getNonnull()->nullable;            // TODO: fix false negative.
      *p.getNonnull()->nullable->unknown;   // TODO: fix false negative.
      *p.getNonnull()->nullable->nullable;  // TODO: fix false negative.
      *p.getUnknown()->unknown;
      *p.getUnknown()->unknown->unknown;
      *p.getUnknown()->unknown->nullable;  // TODO: fix false negative.
      *p.getUnknown();
      *p.getUnknown()->nullable;            // TODO: fix false negative.
      *p.getUnknown()->nullable->nullable;  // TODO: fix false negative.
      *p.getUnknown()->nullable->nonnull;   // TODO: fix false negative.

      *p.getNonnull()->getNonnull();
      *p.getNonnull()->getNonnull()->nullable0;  // TODO: fix false negative.
      *p.getNonnull()->getNonnull()->nullable1;  // TODO: fix false negative.
      *p.getNonnull()->getNullable();            // TODO: fix false negative.
      *p.getNonnull()->getNullable()->unknown;   // TODO: fix false negative.
      *p.getNonnull()->getNullable()->nullable;  // TODO: fix false negative.
      *p.getUnknown()->getUnknown();
      *p.getUnknown()->getUnknown()->unknown;
      *p.getUnknown()->getUnknown()->nullable;   // TODO: fix false negative.
      *p.getUnknown()->getNullable();            // TODO: fix false negative.
      *p.getUnknown()->getNullable()->nullable;  // TODO: fix false negative.
      *p.getUnknown()->getNullable()->nonnull;   // TODO: fix false negative.

      *p.getNonnull()->nonnull->getNullable0();  // TODO: fix false negative.
      *p.getNonnull()->nonnull->getNullable1();  // TODO: fix false negative.
      *p.getNonnull()->nullable->getUnknown();   // TODO: fix false negative.
      *p.getNonnull()->nullable->getNullable();  // TODO: fix false negative.
      *p.getUnknown()->unknown->getUnknown();
      *p.getUnknown()->unknown->getNullable();   // TODO: fix false negative.
      *p.getUnknown()->nullable->getNullable();  // TODO: fix false negative.
      *p.getUnknown()->nullable->getNonnull();   // TODO: fix false negative.

      *p.getNonnull()->getNonnull()->getNullable0();  // TODO: fix false
                                                      // negative.
      *p.getNonnull()->getNonnull()->getNullable1();  // TODO: fix false
                                                      // negative.
      *p.getNonnull()->getNullable()->getUnknown();   // TODO: fix false
                                                      // negative.
      *p.getNonnull()->getNullable()->getNullable();  // TODO: fix false
                                                      // negative.
      *p.getUnknown()->getUnknown()->getUnknown();
      *p.getUnknown()->getUnknown()->getNullable();   // TODO: fix false
                                                      // negative.
      *p.getUnknown()->getNullable()->getNullable();  // TODO: fix false
                                                      // negative.
      *p.getUnknown()->getNullable()->getNonnull();   // TODO: fix false
                                                      // negative.
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

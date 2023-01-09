// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <set>
#include <string>

#include "nullability_verification/pointer_nullability_analysis.h"
#include "nullability_verification/pointer_nullability_diagnosis.h"
#include "clang/Basic/SourceManager.h"
#include "third_party/llvm/llvm-project/clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/ADT/ArrayRef.h"
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
using dataflow::test::AnalysisInputs;
using dataflow::test::AnalysisOutputs;
using dataflow::test::checkDataflow;
using ::testing::ContainerEq;
using ::testing::Test;

bool checkDiagnostics(llvm::StringRef SourceCode) {
  std::vector<CFGElement> Diagnostics;
  PointerNullabilityDiagnoser Diagnoser;
  bool Failed = false;
  EXPECT_THAT_ERROR(
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
          [&Diagnostics, &Failed](
              const llvm::DenseMap<unsigned, std::string> &Annotations,
              const AnalysisOutputs &AnalysisData) {
            // Note: use sorted sets for expected and actual lines to improve
            // readability of the error output in case the test fails.
            std::set<unsigned> ExpectedLines, ActualLines;
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
            if (ActualLines != ExpectedLines) {
              Failed = true;
            }
          }),
      llvm::Succeeded());
  return !Failed;
}

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
    void target(int* _Nonnull x) { *x; }
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
    void target(int* _Nullable x) {
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

// TODO(b/233582219): Implement diagnosis of unreachable program points
TEST(PointerNullabilityTest, NonNullPtrImplicitCastToBool) {
  // x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x) {
      *x;
      if (x) {
        *x;
      } else {
        *x;  // unreachable
      }
      *x;
    }
  )cc"));

  // !x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x) {
      *x;
      if (!x) {
        *x;  // unreachable
      } else {
        *x;
      }
      *x;
    }
  )cc"));
}

TEST(PointerNullabilityTest, NullablePtrImplicitCastToBool) {
  // x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x) {
      *x;  // [[unsafe]]
      if (x) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
    }
  )cc"));

  // !x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x) {
      *x;  // [[unsafe]]
      if (!x) {
        *x;  // [[unsafe]]
      } else {
        *x;
      }
      *x;  // [[unsafe]]
    }
  )cc"));
}

// TODO(b/233582219): Fix false negatives. Casting the pointer to boolean is
// evidence of the author considering null a possibility, hence the unnannotated
// pointer should be considered nullable and emit warnings where it fails or is
// not null checked.
TEST(PointerNullabilityTest, UnknownPtrImplicitCastToBool) {
  // x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) {
      *x;  // false-negative
      if (x) {
        *x;
      } else {
        *x;  // false-negative
      }
      *x;  // false-negative
    }
  )cc"));

  // !x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) {
      *x;  // false-negative
      if (!x) {
        *x;  // false-negative
      } else {
        *x;
      }
      *x;  // false-negative
    }
  )cc"));
}

TEST(PointerNullabilityTest, CompareNonNullPtrAndNonNullPtr) {
  // nonnull == nonnull
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x, int* _Nonnull y) {
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
  )cc"));

  // nonnull != nonnull
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x, int* _Nonnull y) {
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
  )cc"));
}

TEST(PointerNullabilityTest, CompareNullablePtrAndNullablePtr) {
  // nullable == nullable
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nullable y) {
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
      if (x == y) {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      } else {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
    }
  )cc"));

  // nullable != nullable
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nullable y) {
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
      if (x != y) {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      } else {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, CompareUnknownPtrAndUnknownPtr) {
  // unknown == unknown
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  // unknown != unknown
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));
}

// TODO(b/233582219): Implement diagnosis of unreachable program points
TEST(PointerNullabilityTest, CompareNonNullPtrAndNullPtr) {
  // nonnull == nullptr
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x) {
      *x;
      if (x == nullptr) {
        *x;  // unreachable
      } else {
        *x;
      }
      *x;
    }
  )cc"));

  // nullptr == nonnull
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x) {
      *x;
      if (nullptr == x) {
        *x;  // unreachable
      } else {
        *x;
      }
      *x;
    }
  )cc"));

  // nonnull != nullptr
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x) {
      *x;
      if (x != nullptr) {
        *x;
      } else {
        *x;  // unreachable
      }
      *x;
    }
  )cc"));

  // nullptr != nonnull
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x) {
      *x;
      if (nullptr != x) {
        *x;
      } else {
        *x;  // unreachable
      }
      *x;
    }
  )cc"));
}

TEST(PointerNullabilityTest, CompareNullablePtrAndNullPtr) {
  // nullable == nullptr
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x) {
      *x;  // [[unsafe]]
      if (x == nullptr) {
        *x;  // [[unsafe]]
      } else {
        *x;
      }
      *x;  // [[unsafe]]
    }
  )cc"));

  // nullptr == nullable
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x) {
      *x;  // [[unsafe]]
      if (nullptr == x) {
        *x;  // [[unsafe]]
      } else {
        *x;
      }
      *x;  // [[unsafe]]
    }
  )cc"));

  // nullable != nullptr
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x) {
      *x;  // [[unsafe]]
      if (x != nullptr) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
    }
  )cc"));

  // nullptr != nullable
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x) {
      *x;  // [[unsafe]]
      if (nullptr != x) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, CompareNullablePtrAndNonNullPtr) {
  // nullable == nonnull
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nonnull y) {
      *x;  // [[unsafe]]
      *y;
      if (x == y) {
        *x;
        *y;
      } else {
        *x;  // [[unsafe]]
        *y;
      }
      *x;  // [[unsafe]]
      *y;
    }
  )cc"));

  // nonnull == nullable
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nonnull y) {
      *x;  // [[unsafe]]
      *y;
      if (y == x) {
        *x;
        *y;
      } else {
        *x;  // [[unsafe]]
        *y;
      }
      *x;  // [[unsafe]]
      *y;
    }
  )cc"));

  // nullable != nonnull
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nonnull y) {
      *x;  // [[unsafe]]
      *y;
      if (x != y) {
        *x;  // [[unsafe]]
        *y;
      } else {
        *x;
        *y;
      }
      *x;  // [[unsafe]]
      *y;
    }
  )cc"));

  // nonnull != nullable
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nonnull y) {
      *x;  // [[unsafe]]
      *y;
      if (y != x) {
        *x;  // [[unsafe]]
        *y;
      } else {
        *x;
        *y;
      }
      *x;  // [[unsafe]]
      *y;
    }
  )cc"));
}

TEST(PointerNullabilityTest, CompareNullablePtrAndUnknownPtr) {
  // nullable == unknown
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable x, int *y) {
      *x;  // [[unsafe]]
      *y;
      if (x == y) {
        *x;  // [[unsafe]]
        *y;
      } else {
        *x;  // [[unsafe]]
        *y;
      }
      *x;  // [[unsafe]]
      *y;
    }
  )cc"));

  // unknown == nullable
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable x, int *y) {
      *x;  // [[unsafe]]
      *y;
      if (y == x) {
        *x;  // [[unsafe]]
        *y;
      } else {
        *x;  // [[unsafe]]
        *y;
      }
      *x;  // [[unsafe]]
      *y;
    }
  )cc"));

  // nullable != unknown
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable x, int *y) {
      *x;  // [[unsafe]]
      *y;
      if (x != y) {
        *x;  // [[unsafe]]
        *y;
      } else {
        *x;  // [[unsafe]]
        *y;
      }
      *x;  // [[unsafe]]
      *y;
    }
  )cc"));

  // unknown != nullable
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable x, int *y) {
      *x;  // [[unsafe]]
      *y;
      if (y != x) {
        *x;  // [[unsafe]]
        *y;
      } else {
        *x;  // [[unsafe]]
        *y;
      }
      *x;  // [[unsafe]]
      *y;
    }
  )cc"));
}

// TODO(b/233582219): Fix false negatives. The pointer is compared to nullptr,
// hence the unnannotated pointer should be considered nullable and emit
// warnings where it fails or is not null checked.
TEST(PointerNullabilityTest, CompareUnknownPtrAndNullPtr) {
  // unknown == nullptr
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) {
      *x;  // false-negative
      if (x == nullptr) {
        *x;  // false-negative
      } else {
        *x;
      }
      *x;  // false-negative
    }
  )cc"));

  // nullptr == unknown
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) {
      *x;  // false-negative
      if (nullptr == x) {
        *x;  // false-negative
      } else {
        *x;
      }
      *x;  // false-negative
    }
  )cc"));

  // unknown != nullptr
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) {
      *x;  // false-negative
      if (x != nullptr) {
        *x;
      } else {
        *x;  // false-negative
      }
      *x;  // false-negative
    }
  )cc"));

  // nullptr != unknown
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) {
      *x;  // false-negative
      if (nullptr != x) {
        *x;
      } else {
        *x;  // false-negative
      }
      *x;  // false-negative
    }
  )cc"));
}

TEST(PointerNullabilityTest, CompareUnknownPtrAndNonNullPtr) {
  // unknown == nonnull
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x, int *_Nonnull y) {
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
  )cc"));

  // nonnull == unknown
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x, int *_Nonnull y) {
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
  )cc"));

  // unknown != nonnull
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x, int *_Nonnull y) {
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
  )cc"));

  // nonnull != unknown
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x, int *_Nonnull y) {
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
  )cc"));
}

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

TEST(PointerNullabilityTest, BinaryExpressions) {
  // x && y
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nullable y) {
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
      if (x && y) {
        *x;
        *y;
      } else {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
    }
  )cc"));

  // x || y
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nullable y) {
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
      if (x || y) {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      } else {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
    }
  )cc"));

  // !x && !y
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nullable y) {
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
      if (!x && !y) {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      } else {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
    }
  )cc"));

  // !x || !y
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x, int* _Nullable y) {
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
      if (!x || !y) {
        *x;  // [[unsafe]]
        *y;  // [[unsafe]]
      } else {
        *x;
        *y;
      }
      *x;  // [[unsafe]]
      *y;  // [[unsafe]]
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

TEST(PointerNullabilityTest, ThisPointer) {
  // (->) implicit `this`
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      void foo();
      void target() { foo(); }
    };
  )cc"));

  // (->) explicit `this`
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      void foo();
      void target() { this->foo(); }
    };
  )cc"));
}

TEST(PointerNullabilityTest, NonNullFieldsOfPointerType) {
  // dereference field of pointer type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo* _Nonnull ptr;
    };
    void target(Foo foo) { *foo.ptr; }
  )cc"));

  // dereference field of pointer type in member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo* _Nonnull ptr;
      void target() { *ptr; }
    };
  )cc"));
}

TEST(PointerNullabilityTest, NullableFieldsOfPointerType) {
  // dereference field of pointer type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo* _Nullable ptr;
    };
    void target(Foo foo) {
      *foo.ptr;  // [[unsafe]]
      if (foo.ptr) {
        *foo.ptr;
      } else {
        *foo.ptr;  // [[unsafe]]
      }
      *foo.ptr;  // [[unsafe]]
    }
  )cc"));

  // dereference field of pointer type in member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo* _Nullable ptr;
      void target() {
        *ptr;  // [[unsafe]]
        if (ptr) {
          *ptr;
        } else {
          *ptr;  // [[unsafe]]
        }
        *ptr;  // [[unsafe]]
      }
    };
  )cc"));
}

TEST(PointerNullabilityTest, UnknownFieldsOfPointerType) {
  // dereference field of pointer type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *ptr;
    };
    void target(Foo foo) { *foo.ptr; }
  )cc"));

  // dereference field of pointer type in member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *ptr;
      void target() { *ptr; }
    };
  )cc"));
}

TEST(PointerNullabilityTest, MergeNullAndNonNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull y, bool b) {
      int *x = nullptr;
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
        x = y;
        *x;
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, MergeNullAndNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable y, bool b) {
      int *x = nullptr;
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
        x = y;
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
      } else {
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, MergeNullAndUnknown) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *y, bool b) {
      int *x = nullptr;
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
        x = y;
        *x;
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, MergeNonNullAndNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull y, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = nullptr;
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
      } else {
        *x;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, MergeNonNullAndNonNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull y, int *_Nonnull z, bool b) {
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
  )cc"));
}

TEST(PointerNullabilityTest, MergeNonNullAndNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull y, int *_Nullable z, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = z;
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
      } else {
        *x;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, MergeNonNullAndUnknown) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull y, int *z, bool b) {
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
  )cc"));
}

TEST(PointerNullabilityTest, MergeNullableAndNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable y, bool b) {
      int *x = y;
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
        x = nullptr;
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
      } else {
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, MergeNullableAndNonNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable y, int *_Nonnull z, bool b) {
      int *x = y;
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
        x = z;
        *x;
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, MergeNullableAndNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable y, int *_Nullable z, bool b) {
      int *x = y;
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
        x = z;
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
      } else {
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, MergeNullableAndUnknown) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable y, int *z, bool b) {
      int *x = y;
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
        x = z;
        *x;
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, MergeUnknownAndNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *y, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = nullptr;
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
      } else {
        *x;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, MergeUnknownAndNonNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *y, int *_Nonnull z, bool b) {
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
  )cc"));
}

TEST(PointerNullabilityTest, MergeUnknownAndNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *y, int *_Nullable z, bool b) {
      int *x = y;
      *x;
      if (b) {
        *x;
        x = z;
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
      if (b) {
        *x;  // [[unsafe]]
      } else {
        *x;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, MergeUnknownAndUnknown) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));
}

TEST(PointerNullabilityTest, CallExprWithPointerReturnType) {
  // free function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull makeNonnull();
    int *_Nullable makeNullable();
    int *makeUnannotated();
    void target() {
      *makeNonnull();
      *makeNullable();  // [[unsafe]]
      *makeUnannotated();
    }
  )cc"));

  // member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      int *_Nonnull makeNonnull();
      int *_Nullable makeNullable();
      int *makeUnannotated();
    };
    void target(Foo foo) {
      *foo.makeNonnull();
      *foo.makeNullable();  // [[unsafe]]
      *foo.makeUnannotated();
    }
  )cc"));

  // overloaded operator call
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct MakeNonnull {
      int *_Nonnull operator()();
    };
    struct MakeNullable {
      int *_Nullable operator()();
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
  )cc"));

  // function pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull (*makeNonnull)(),
                int* _Nullable (*makeNullable)(), int* (*makeUnannotated)()) {
      *makeNonnull();
      *makeNullable();  // [[unsafe]]
      *makeUnannotated();
    }
  )cc"));

  // pointer to function pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull (**makeNonnull)(),
                int* _Nullable (**makeNullable)(), int* (**makeUnannotated)()) {
      *(*makeNonnull)();
      *(*makeNullable)();  // [[unsafe]]
      *(*makeUnannotated)();
    }
  )cc"));

  // function returning a function pointer which returns a pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    typedef int* _Nonnull (*MakeNonnullT)();
    typedef int* _Nullable (*MakeNullableT)();
    typedef int* (*MakeUnannotatedT)();
    void target(MakeNonnullT (*makeNonnull)(), MakeNullableT (*makeNullable)(),
                MakeUnannotatedT (*makeUnannotated)()) {
      *(*makeNonnull)()();
      *(*makeNullable)()();  // [[unsafe]]
      *(*makeUnannotated)()();
    }
  )cc"));

  // free function returns reference to pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull &makeNonnull();
    int *_Nullable &makeNullable();
    int *&makeUnannotated();
    void target() {
      *makeNonnull();
      *makeNullable();  // [[unsafe]]
      *makeUnannotated();
    }
  )cc"));

  // function called in loop
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nullable makeNullable();
    bool makeBool();
    void target() {
      bool first = true;
      while (true) {
        int *x = makeNullable();
        if (first && x == nullptr) return;
        first = false;
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, DoubleDereference) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int** p) {
      *p;
      **p;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int** _Nonnull p) {
      *p;
      **p;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull* p) {
      *p;
      **p;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull* _Nonnull p) {
      *p;
      **p;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int** _Nullable p) {
      *p;   // [[unsafe]]
      **p;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable* p) {
      *p;
      **p;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable* _Nullable p) {
      *p;   // [[unsafe]]
      **p;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable* _Nonnull p) {
      *p;
      **p;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull* _Nullable p) {
      *p;   // [[unsafe]]
      **p;  // [[unsafe]]
    }
  )cc"));
}

// TODO: Fix false negatives.
TEST(PointerNullabilityTest, ClassTemplateInstantiation) {
  // Class template specialization with one argument initialised as _Nullable.
  // We test types that contain both nullability that is substituted into the
  // template argument and nullability that is spelt inside the template. That
  // is, we should be able to accurately store nullabilities from different
  // sources in a single nullability vector.
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  // Class template specialization with one argument initialised as _Nonnull.
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  // Class template specialization with one argument initialised without
  // nullability annotation.
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  // Class template specialization with two arguments, whose second argument is
  // initialized as nullable.
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  // Class template specialization with 5 arguments with interleaved
  // nullable/nonnull/unknown.
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  // Class template specialization with 5 arguments with interleaved
  // nullable/nonnull/unknown/const.
  EXPECT_TRUE(checkDiagnostics(R"cc(
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

      *p.getT0();  // [[unsafe]]
      *p.getT1();
      *p.getT2();
      *p.getT3();  // [[unsafe]]
      *p.getT4();
    }
  )cc"));

  // Class template specialization with interleaved int and type template
  // parameters.
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));
}

// TODO: Fix false positives and false negatives.
TEST(PointerNullabilityTest,
     ClassTemplateInstantiationWithStructsAsParameters) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  // With const arguments and int template parameter.
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));
}

// TODO: Fix false negatives.
TEST(PointerNullabilityTest, MemberFunctionTemplateOfConcreteStruct) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      template <int I0, typename T1, int I2>
      T1 getT1();
    };

    void target(S p) {
      *p.getT1<0, int *, 1>();
      *p.getT1<2147483647, int *_Nonnull, -2147483647>();
      *p.getT1<4, int *_Nullable, 4>();  // TODO: fix false negative.
    }
  )cc"));
}

TEST(PointerNullabilityTest, MemberFunctionTemplateOfTemplateStruct) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0>
    struct S {
      template <int IN1, typename TN2, int IN3>
      TN2 getTN2();
    };

    void target(S<int> p) {
      *p.getTN2<0, int *, 1>();
      *p.getTN2<2147483647, int *_Nonnull, -2147483647>();
      *p.getTN2<4, int *_Nullable, 4>();  // TODO: fix false negative.
    }
  )cc"));
}

// TODO: Fix false positives.
TEST(PointerNullabilityTest,
     ClassTemplateInstantiationWithTemplateStructsAsParameters) {
  // Class template with another class template as parameter
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  // Class template with itself as parameter
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));
}

TEST(PointerNullabilityTest,
     ClassTemplateInstantiationWithPointersToStructsAsParameters) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));
}

TEST(PointerNullabilityTest,
     ClassTemplateInstantiationWithPointersToTemplateStructsAsParameters) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
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
  )cc"));
}

TEST(PointerNullabilityTest, CallExprParamAssignment) {
  // free function with single param
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void takeNonnull(int *_Nonnull);
    void takeNullable(int *_Nullable);
    void takeUnannotated(int *);
    void target(int *_Nonnull ptr_nonnull, int *_Nullable ptr_nullable,
                int *ptr_unannotated) {
      takeNonnull(nullptr);  // [[unsafe]]
      takeNonnull(ptr_nonnull);
      takeNonnull(ptr_nullable);  // [[unsafe]]
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
  )cc"));

  // overloaded operator with single param
  EXPECT_TRUE(checkDiagnostics(R"cc(
    // map<int * _Nonnull, int>
    struct MapWithNonnullKeys {
      int &operator[](int *_Nonnull key);
    };
    // map<int * _Nullable, int>
    struct MapWithNullableKeys {
      int &operator[](int *_Nullable key);
    };
    // map<int *, int>
    struct MapWithUnannotatedKeys {
      int &operator[](int *key);
    };
    void target(int *_Nonnull ptr_nonnull, int *_Nullable ptr_nullable,
                int *ptr_unannotated) {
      MapWithNonnullKeys nonnull_keys;
      nonnull_keys[nullptr] = 42;  // [[unsafe]]
      nonnull_keys[ptr_nonnull] = 42;
      nonnull_keys[ptr_nullable] = 42;  // [[unsafe]]
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
  )cc"));

  // free function with multiple params of mixed nullability
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void takeMixed(int *, int *_Nullable, int *_Nonnull);
    void target() {
      takeMixed(nullptr, nullptr, nullptr);  // [[unsafe]]
    }
  )cc"));

  // overloaded operator with multiple params of mixed nullability
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct TakeMixed {
      void operator()(int *, int *_Nullable, int *_Nonnull);
    };
    void target() {
      TakeMixed takeMixed;
      takeMixed(nullptr, nullptr, nullptr);  // [[unsafe]]
    }
  )cc"));

  // member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      void takeNonnull(int *_Nonnull);
      void takeNullable(int *_Nullable);
      void takeUnannotated(int *);
    };
    void target(Foo foo) {
      foo.takeNonnull(nullptr);  // [[unsafe]]
      foo.takeNullable(nullptr);
      foo.takeUnannotated(nullptr);
    }
  )cc"));

  // function pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(void (*takeNonnull)(int *_Nonnull),
                void (*takeNullable)(int *_Nullable),
                void (*takeUnannotated)(int *)) {
      takeNonnull(nullptr);  // [[unsafe]]
      takeNullable(nullptr);
      takeUnannotated(nullptr);
    }
  )cc"));

  // pointer to function pointer
  //
  // TODO(b/233582219): Fix false negative. Implement support for retrieving
  // parameter types from a pointer to function pointer.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(void (**takeNonnull)(int *_Nonnull),
                void (**takeNullable)(int *_Nullable),
                void (**takeUnannotated)(int *)) {
      (*takeNonnull)(nullptr);  // false-negative
      (*takeNullable)(nullptr);
      (*takeUnannotated)(nullptr);
    }
  )cc"));

  // function returned from function
  //
  // TODO(b/233582219): Fix false negative. Implement support for retrieving
  // parameter types for functions returned by another function.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    typedef void (*takeNonnullF)(int *_Nonnull);
    typedef void (*takeNullableF)(int *_Nullable);
    typedef void (*takeUnannotatedF)(int *);
    void target(takeNonnullF (*takeNonnull)(), takeNullableF (*takeNullable)(),
                takeUnannotatedF (*takeUnannotated)()) {
      (*takeNonnull)()(nullptr);  // false-negative
      (*takeNullable)()(nullptr);
      (*takeUnannotated)()(nullptr);
    }
  )cc"));

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
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void takeNonnullRef(int *_Nonnull &);
    void takeNullableRef(int *_Nullable &);
    void takeUnannotatedRef(int *&);
    void target(int *_Nonnull ptr_nonnull) {
      takeNonnullRef(ptr_nonnull);
      *ptr_nonnull;

      // false-negative
      takeNullableRef(ptr_nonnull);
      *ptr_nonnull;

      takeUnannotatedRef(ptr_nonnull);
      *ptr_nonnull;
    }
  )cc"));

  // passing a reference to a nullable pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void takeNonnullRef(int *_Nonnull &);
    void takeNullableRef(int *_Nullable &);
    void takeUnannotatedRef(int *&);
    void target(int *_Nullable ptr_nullable) {
      takeNonnullRef(ptr_nullable);  // [[unsafe]]
      *ptr_nullable;                 // [[unsafe]]

      takeNullableRef(ptr_nullable);
      *ptr_nullable;  // [[unsafe]]

      takeUnannotatedRef(ptr_nullable);
      *ptr_nullable;  // [[unsafe]]
    }
  )cc"));

  // passing a reference to an unannotated pointer
  //
  // TODO(b/233582219): Fix false negative. The unannotated pointer should be
  // considered nullable if it has been used as a nullable pointer.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void takeNonnullRef(int *_Nonnull &);
    void takeNullableRef(int *_Nullable &);
    void takeUnannotatedRef(int *&);
    void target(int *ptr_unannotated) {
      takeNonnullRef(ptr_unannotated);
      *ptr_unannotated;

      takeNullableRef(ptr_unannotated);
      *ptr_unannotated;  // false-negative

      takeUnannotatedRef(ptr_unannotated);
      *ptr_unannotated;
    }
  )cc"));
}

TEST(PointerNullabilityTest, ReturnStatements) {
  // nonnull return type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nonnull target() {
      return nullptr;  // [[unsafe]]
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nonnull target(int* _Nonnull ptr_nonnull) {
      return ptr_nonnull;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nonnull target(int* _Nullable ptr_nullable) {
      return ptr_nullable;  // [[unsafe]]
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull target(int *ptr_unannotated) {
      return ptr_unannotated;
    }
  )cc"));

  // nullable return type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable target() { return nullptr; }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable target(int* _Nonnull ptr_nonnull) {
      return ptr_nonnull;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable target(int* _Nullable ptr_nullable) {
      return ptr_nullable;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nullable target(int *ptr_unannotated) {
      return ptr_unannotated;
    }
  )cc"));

  // unannotated return type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* target() { return nullptr; }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* target(int* _Nonnull ptr_nonnull) {
      return ptr_nonnull;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* target(int* _Nullable ptr_nullable) {
      return ptr_nullable;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *target(int *ptr_unannotated) {
      return ptr_unannotated;
    }
  )cc"));

  // multiple return statements
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nonnull target(bool b, int* _Nonnull ptr_nonnull) {
      if (b) {
        return nullptr;  // [[unsafe]]
      }
      return ptr_nonnull;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nonnull target(int* _Nullable ptr_nullable,
                         int* _Nonnull ptr_nonnull) {
      if (ptr_nullable) {
        return ptr_nullable;
      }
      return ptr_nonnull;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nonnull target(int* _Nullable ptr_nullable_1,
                         int* _Nullable ptr_nullable_2) {
      if (ptr_nullable_1) {
        return ptr_nullable_2;  // [[unsafe]]
      }
      return ptr_nullable_1;  // [[unsafe]]
    }
  )cc"));

  // return result of merging 2 pointer values
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull target(bool b, int i) {
      int *ptr;
      if (b) {
        ptr = &i;
      } else {
        ptr = nullptr;
      }
      return ptr;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, ConstructExpr) {
  // Constructor call assigned to local variable.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct TakeNonnull {
      explicit TakeNonnull(int *_Nonnull) {}
    };
    struct TakeNullable {
      explicit TakeNullable(int *_Nullable) {}
    };
    struct TakeUnannotated {
      explicit TakeUnannotated(int *) {}
    };
    int *_Nonnull makeNonnull();
    int *_Nullable makeNullable();
    int *makeUnannotated();
    void target() {
      auto NN1 = TakeNonnull(makeNonnull());
      auto NN2 = TakeNonnull(makeNullable());  // [[unsafe]]
      auto NN3 = TakeNonnull(makeUnannotated());

      auto NB1 = TakeNullable(makeNonnull());
      auto NB2 = TakeNullable(makeNullable());
      auto NB3 = TakeNullable(makeUnannotated());

      auto UN1 = TakeUnannotated(makeNonnull());
      auto UN2 = TakeUnannotated(makeNullable());
      auto UN3 = TakeUnannotated(makeUnannotated());
    }
  )cc"));

  // Constructor call in a base initializer.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct TakeNonnull {
      explicit TakeNonnull(int* _Nonnull);
    };
    struct target : TakeNonnull {
      target(int* _Nullable ptr_nullable)  // Forced line break.
          : TakeNonnull(ptr_nullable)      // [[unsafe]]
      {}
    };
  )cc"));

  // Call to a delegating constructor.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable makeNullable();
    struct target {
      target(int* _Nonnull);
      target()                      // Forced line break.
          : target(makeNullable())  // [[unsafe]]
      {}
    };
  )cc"));
}

TEST(PointerNullabilityTest, ConstructorMemberInitializer) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable makeNullable();
    struct target {
      int* _Nonnull ptr_nonnull;
      int* _Nullable ptr_nullable;
      int* ptr_unannotated;
      target()
          : ptr_nonnull(makeNullable()),  // [[unsafe]]
            ptr_nullable(makeNullable()),
            ptr_unannotated(makeNullable()) {}
    };
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nonnull makeNonnull();
    struct target {
      int* _Nonnull ptr_nonnull;
      int* _Nullable ptr_nullable;
      int* ptr_unannotated;
      target()
          : ptr_nonnull(makeNonnull()),
            ptr_nullable(makeNonnull()),
            ptr_unannotated(makeNonnull()) {}
    };
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *makeUnannotated();
    struct target {
      int *_Nonnull ptr_nonnull;
      int *_Nullable ptr_nullable;
      int *ptr_unannotated;
      target()
          : ptr_nonnull(makeUnannotated()),
            ptr_nullable(makeUnannotated()),
            ptr_unannotated(makeUnannotated()) {}
    };
  )cc"));
}

// TODO: Move the definitions of `NullabilityKind` and `__assert_nullability()`
// into a preamble that `checkDiagnostics()` prepends to every test.
TEST(PointerNullabilityTest, AssertNullability) {
  const std::string Declarations = R"cc(
    enum NullabilityKind {
      NK_nonnull,
      NK_nullable,
      NK_unspecified,
    };

    template <NullabilityKind... NK, typename T>
    void __assert_nullability(T&);
  )cc";

  // Concrete struct.
  EXPECT_TRUE(checkDiagnostics(Declarations + R"cc(
    struct StructNonnullNullable {
      int* _Nonnull nonnull;
      int* _Nullable nullable;
    };

    void target(StructNonnullNullable p) {
      __assert_nullability<>(p);
      __assert_nullability<NK_nonnull>(p);                   // [[unsafe]]
      __assert_nullability<NK_nullable>(p);                  // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_nullable>(p);      // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_unspecified>(p);   // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_nonnull>(p);       // [[unsafe]]
      __assert_nullability<NK_nullable, NK_nullable>(p);     // [[unsafe]]
      __assert_nullability<NK_unspecified, NK_nullable>(p);  // [[unsafe]]
    }
  )cc"));

  // Struct with two template type parameters.
  EXPECT_TRUE(checkDiagnostics(Declarations + R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {};

    void target(Struct2Arg<int *, int *_Nullable> p) {
      __assert_nullability<NK_unspecified>(p);  // [[unsafe]]
      __assert_nullability<NK_nullable>(p);     // [[unsafe]]

      __assert_nullability<NK_unspecified, NK_nonnull>(p);  // [[unsafe]]
      __assert_nullability<NK_unspecified, NK_nullable>(p);
      __assert_nullability<NK_unspecified, NK_unspecified>(p);  // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_nullable>(p);         // [[unsafe]]
      __assert_nullability<NK_nullable, NK_nullable>(p);        // [[unsafe]]

      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_unspecified>(p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable>(p);
    }
  )cc"));

  // Struct with one type and non-type template parameters.
  EXPECT_TRUE(checkDiagnostics(Declarations + R"cc(
    template <int I0, typename T1, typename T2>
    struct Struct3ArgWithInt {};

    void target(Struct3ArgWithInt<2147483647, int* _Nullable, int* _Nonnull> p) {
      __assert_nullability<>(p);             // [[unsafe]]
      __assert_nullability<NK_nonnull>(p);   // [[unsafe]]
      __assert_nullability<NK_nullable>(p);  // [[unsafe]]

      __assert_nullability<NK_unspecified, NK_nonnull>(p);  // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_nonnull>(p);      // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_nullable>(p);     // [[unsafe]]
      __assert_nullability<NK_nullable, NK_nonnull>(p);
      __assert_nullability<NK_nullable, NK_nullable>(p);     // [[unsafe]]
      __assert_nullability<NK_nullable, NK_unspecified>(p);  // [[unsafe]]

      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nonnull>(p);
    }
  )cc"));

  // Nested template arguments.
  EXPECT_TRUE(checkDiagnostics(Declarations + R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {};

    void target(
        Struct2Arg<Struct2Arg<int *, int *_Nullable>,
                   Struct2Arg<Struct2Arg<int *_Nullable, int *_Nonnull>,
                              Struct2Arg<int *_Nullable, int *_Nullable>>>
            p) {
      __assert_nullability<>(p);  // [[unsafe]]

      __assert_nullability<NK_unspecified, NK_nullable, NK_nullable, NK_nonnull,
                           NK_nullable, NK_nullable>(p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable, NK_nonnull, NK_nullable,
           NK_nullable, NK_nullable>(p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable, NK_nonnull, NK_nullable>(
              p);
    }
  )cc"));

  // Struct with two template parameters substituted with concrete structs.
  EXPECT_TRUE(checkDiagnostics(Declarations + R"cc(
    struct StructUnknownNullable {
      int* unknown;
      int* _Nullable nullable;
    };

    struct StructNullableNonnull {
      int* _Nullable nullable;
      int* _Nonnull nonnull;
    };

    template <typename T1, typename T2>
    struct Struct2Arg {};

    void target(Struct2Arg<StructUnknownNullable, StructNullableNonnull> p) {
      __assert_nullability<>(p);

      __assert_nullability<NK_unspecified, NK_nullable>(p);  // [[unsafe]]

      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable>(p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable, NK_nonnull>(p);
      __assert_nullability  // [[unsafe]]
          <NK_nonnull, NK_nullable, NK_nullable, NK_nonnull>(p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable, NK_nonnull,
           NK_unspecified>(p);
    }
  )cc"));

  // Member variables and member calls on a struct with nested template
  // arguments.
  // TODO: Write non-flow sensitive transfer functions for cast expressions,
  // then change the argument type of `__assert_nullability` to take in a const
  // T&, which then allows for member call arguments. Further, add non-flow
  // sensitive transfer function for the dereference operator.
  EXPECT_TRUE(checkDiagnostics(Declarations + R"(
    template <typename T0, typename T1>
    struct Struct2Arg {
      T0 arg0;
      T1 arg1;

      T0 getT0();
      T1 getT1();
    };

    void target(
        Struct2Arg<Struct2Arg<int *, int *_Nullable>,
                   Struct2Arg<Struct2Arg<int *_Nullable, int *_Nonnull>,
                              Struct2Arg<int *_Nullable, int *_Nullable>>> *p) {
      __assert_nullability<NK_unspecified, NK_unspecified, NK_nullable,
                           NK_nullable, NK_nonnull, NK_nullable, NK_nullable>(
          p);

      __assert_nullability<>(p);  // [[unsafe]]
      __assert_nullability        // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable, NK_nonnull, NK_nullable,
           NK_nullable, NK_nullable>(p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable, NK_nonnull, NK_nullable>(
              p);

//      __assert_nullability<NK_unspecified, NK_nullable, NK_nullable, NK_nonnull,
//                           NK_nullable, NK_nullable>(*p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_unspecified, NK_nullable, NK_nullable, NK_nonnull,
           NK_nullable, NK_nullable>(*p);

//      __assert_nullability<NK_unspecified, NK_nullable>(p->getT0());
//        __assert_nullability<NK_unspecified, NK_nullable>(p->arg0);

//      __assert_nullability  // TODO: fix false negative.
//          <NK_unspecified, NK_nullable, NK_unspecified>(p->getT0());
//      __assert_nullability  // TODO: fix false negative.
//          <NK_unspecified>(p->getT0());
//      __assert_nullability  // TODO: fix false negative.
//          <NK_unspecified, NK_nullable, NK_nullable>(p->arg0);
//       __assert_nullability  // TODO: fix false negative.
//          <NK_nullable>(p->arg0);

//      __assert_nullability<NK_nonnull>(p->getT1().getT0().getT1());
//      __assert_nullability<NK_nonnull>(p->getT1().arg0.getT1());
//      __assert_nullability<NK_nonnull>(p->arg1.getT0().arg1);
//      __assert_nullability<NK_nonnull>(p->arg1.arg0.arg1);

//      __assert_nullability  // TODO: fix false negative.
//          <>(p->getT1().getT0().getT1());
//      __assert_nullability  // TODO: fix false negative.
//          <NK_nonnull, NK_nonnull>(p->arg1.getT0().arg1);
    }
  )"));
}
}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang

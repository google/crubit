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
              ASTContext &Ctx, const Stmt *Stmt,
              const TypeErasedDataflowAnalysisState &State) {
            auto StmtDiagnostics = Diagnoser.diagnose(Stmt, Ctx, State.Env);
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
          {"-fsyntax-only", "-std=c++17", "-Wno-unused-value"}),
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

TEST(PointerNullabilityTest, DerefNullablePtrWithoutACheck) {
  checkDiagnostics(R"(
    void target(int *x) {
      *x; // [[unsafe]]
    }
  )");

  // transitive
  checkDiagnostics(R"(
    void target(int *x) {
      int *y = x;
      *y; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, NullablePtrImplicitCastToBool) {
  // x
  checkDiagnostics(R"(
    void target(int *x) {
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
    void target(int *x) {
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

TEST(PointerNullabilityTest, CompareNullablePtrAndNullPtr) {
  // x == nullptr
  checkDiagnostics(R"(
    void target(int *x) {
      *x; // [[unsafe]]
      if (x == nullptr) {
        *x; // [[unsafe]]
      } else {
        *x;
      }
      *x; // [[unsafe]]
    }
  )");

  // nullptr == x
  checkDiagnostics(R"(
    void target(int *x) {
      *x; // [[unsafe]]
      if (nullptr == x) {
        *x; // [[unsafe]]
      } else {
        *x;
      }
      *x; // [[unsafe]]
    }
  )");

  // x != nullptr
  checkDiagnostics(R"(
    void target(int *x) {
      *x; // [[unsafe]]
      if (x != nullptr) {
        *x;
      } else {
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
    }
  )");

  // nullptr != x
  checkDiagnostics(R"(
    void target(int *x) {
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
  // x == nonnull
  checkDiagnostics(R"(
    void target(int *x) {
      int i = 0;
      int *y = &i;
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

  // nonnull == x
  checkDiagnostics(R"(
    void target(int *x) {
      int i = 0;
      int *y = &i;
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
}

TEST(PointerNullabilityTest, CheckByComparisonToOtherNullPtr) {
  checkDiagnostics(R"(
    void target(int *x) {
      int i = 0;
      int *y = &i;
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

  // nonnull != x
  checkDiagnostics(R"(
    void target(int *x) {
      int i = 0;
      int *y = &i;
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

TEST(PointerNullabilityTest, CompareNullablePtrAndNullablePtr) {
  // x == y
  checkDiagnostics(R"(
    void target(int *x, int *y) {
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

  // x != y
  checkDiagnostics(R"(
    void target(int *x, int *y) {
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

TEST(PointerNullabilityTest, TransitiveNullCheck) {
  checkDiagnostics(R"(
    void target(int *x) {
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
    void target(int *x) {
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
    void target(int *x, int *y) {
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
    void target(int *x, int *y) {
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
    void target(int *x, int *y) {
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
    void target(int *x, int *y) {
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

TEST(PointerNullabilityTest, ArrowOperator) {
  // (->) member field
  checkDiagnostics(R"(
    struct Foo {
      Foo *foo;
    };
    void target(Foo *foo) {
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
    void target(Foo *foo) {
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

TEST(PointerNullabilityTest, FieldsOfPointerType) {
  // dereference field of pointer type
  checkDiagnostics(R"(
    struct Foo {
      Foo *ptr;
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
      Foo *ptr;
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

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang

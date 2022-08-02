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

TEST(PointerNullabilityTest, DereferenceWithoutACheck) {
  checkDiagnostics(R"(
    void target(int* maybeNull) {
      *maybeNull; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, InitializedWithNullPtrLiteral) {
  checkDiagnostics(R"(
    void target() {
      int *null = nullptr;
      *null; // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    void target() {
      int *null = 0;
      *null; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, InitializedWithAddressOf) {
  checkDiagnostics(R"(
    void target(int x) {
      int *nonNull = &x;
      *nonNull;
    }
  )");
}

TEST(PointerNullabilityTest, InitializedWithOtherPointer) {
  checkDiagnostics(R"(
    void target(int x) {
      int *nonNull = &x;
      int *nonNullCopy = nonNull;
      *nonNullCopy;
    }
  )");

  checkDiagnostics(R"(
    void target(int* nullable) {
      int *nullableCopy = nullable;
      *nullableCopy; // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    void target(int* nullable) {
      int *nullableCopy = nullable;
      if (nullable) {
        *nullableCopy;
      } else {
        *nullableCopy; // [[unsafe]]
      }
      *nullableCopy; // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    void target(int* nullable) {
      int *nullableCopy = nullable;
      if (nullableCopy) {
        *nullable;
      } else {
        *nullable; // [[unsafe]]
      }
      *nullable; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, CheckByComparisonToNullPtr) {
  checkDiagnostics(R"(
    void target(int *maybeNull) {
      if (maybeNull != nullptr) {
        *maybeNull;
      } else {
        *maybeNull; // [[unsafe]]
      }
      *maybeNull; // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    void target(int *maybeNull) {
      if (nullptr != maybeNull) {
        *maybeNull;
      } else {
        *maybeNull; // [[unsafe]]
      }
      *maybeNull; // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    void target(int* maybeNull) {
      if (maybeNull == nullptr) {
        *maybeNull; // [[unsafe]]
      } else {
        *maybeNull;
      }
      *maybeNull; // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    void target(int* maybeNull) {
      if (nullptr == maybeNull) {
        *maybeNull; // [[unsafe]]
      } else {
        *maybeNull;
      }
      *maybeNull; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, CheckByImplicitCastToBool) {
  checkDiagnostics(R"(
    void target(int* maybeNull) {
      if (maybeNull) {
        *maybeNull;
      } else {
        *maybeNull; // [[unsafe]]
      }
      *maybeNull; // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    void target(int* maybeNull) {
      if (!maybeNull) {
        *maybeNull; // [[unsafe]]
      } else {
        *maybeNull;
      }
      *maybeNull; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, CheckByComparisonToOtherNullPtr) {
  checkDiagnostics(R"(
    void target(int* maybeNull) {
      int *null = nullptr;
      if (maybeNull != null) {
        *maybeNull;
      } else {
        *maybeNull; // [[unsafe]]
      }
      *maybeNull; // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    void target(int* maybeNull) {
      int *null = nullptr;
      if (maybeNull == null) {
        *maybeNull; // [[unsafe]]
      } else {
        *maybeNull;
      }
      *maybeNull; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, CheckByComparisonToOtherNonNullPtr) {
  checkDiagnostics(R"(
    void target(int* maybeNull, int x) {
      int* nonNull = &x;
      if (maybeNull != nonNull) {
        *maybeNull; // [[unsafe]]
      } else {
        *maybeNull;
      }
      *maybeNull; // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    void target(int* maybeNull, int x) {
      int* nonNull = &x;
      if (maybeNull == nonNull) {
        *maybeNull;
      } else {
        *maybeNull; // [[unsafe]]
      }
      *maybeNull; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, CheckByComparisonToOtherUnknownPtr) {
  checkDiagnostics(R"(
    void target(int* x, int* y) {
      if (x != y) {
        *x; // [[unsafe]]
      } else {
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    void target(int* x, int* y) {
      if (x == y) {
        *x; // [[unsafe]]
      } else {
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, BinaryExpressions) {
  checkDiagnostics(R"(
    void target(int* x, int* y) {
      if (x && y) {
        *x;
      } else {
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    void target(int* x, int* y) {
      if (x || y) {
        *x; // [[unsafe]]
      } else {
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    void target(int* x, int* y) {
      if (!x && !y) {
        *x; // [[unsafe]]
      } else {
        *x; // [[unsafe]]
      }
      *x; // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    void target(int* x, int* y) {
      if (!x || !y) {
        *x; // [[unsafe]]
      } else {
        *x;
      }
      *x; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, MemberPointers) {
  checkDiagnostics(R"(
    struct Foo {
      Foo* ptr;
    };
    void target(Foo foo) {
      if (foo.ptr) {
        *foo.ptr;
      } else {
        *foo.ptr; // [[unsafe]]
      }
      *foo.ptr; // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    struct Foo {
      Foo* ptr;
      void target() {
        if (ptr) {
          *ptr;
        } else {
          *ptr; // [[unsafe]]
        }
        *ptr; // [[unsafe]]
      }
    };
  )");

  checkDiagnostics(R"(
    class Foo {
     public:
      Foo* ptr;
    };
    void target(Foo foo) {
      if (foo.ptr) {
        *foo.ptr;
      } else {
        *foo.ptr; // [[unsafe]]
      }
      *foo.ptr; // [[unsafe]]
    }
  )");
}

TEST(PointerNullabilityTest, MemberAccessOnPointer) {
  checkDiagnostics(R"(
    struct Foo {
      void foo();
    };
    void target(Foo* foo) {
      if (foo) {
        foo->foo();
      } else {
        foo->foo(); // [[unsafe]]
      }
      foo->foo(); // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    struct Foo {
      void foo();
      void target() {
        foo();
      }
    };
  )");

  checkDiagnostics(R"(
    struct Foo {
      void foo();
      void target() {
        this->foo();
      }
    };
  )");

  checkDiagnostics(R"(
    struct Foo {
      void foo();
      void target() {
        Foo *thisCopy = this;
        thisCopy->foo();
      }
    };
  )");

  checkDiagnostics(R"(
    struct Foo {
      Foo* foo;
    };
    void target(Foo* foo) {
      if (foo) {
        foo->foo->foo; // [[unsafe]]
      } else {
        foo->foo->foo; // [[unsafe]]
      }
      foo->foo->foo; // [[unsafe]]
    }
  )");

  checkDiagnostics(R"(
    struct Foo {
      Foo* foo;
    };
    void target(Foo* foo) {
      if (foo && foo->foo) {
        foo->foo->foo;
      } else {
        foo->foo; // [[unsafe]]
      }
      foo->foo; // [[unsafe]]
    }
  )");
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang

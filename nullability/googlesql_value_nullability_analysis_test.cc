// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/googlesql_value_nullability_analysis.h"

#include <memory>
#include <set>
#include <string>

#include "nullability/googlesql_value_nullability_lattice.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/AdornedCFG.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "clang/Testing/CommandLineArgs.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Testing/Annotations/Annotations.h"
#include "external/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {

// Preamble providing a stub for googlesql::Value.
static const char* kPreamble = R"cpp(
  namespace googlesql {
  class Value {
   public:
    Value();
    static Value Int64(int v);
    static Value NullInt64();
    bool is_null() const;
    int value() const;
    int Int64Value() const;
    Value field(int i) const;
  };
  }  // namespace googlesql
)cpp";

static void runTest(llvm::StringRef TestCode) {
  std::string FullCode = std::string(kPreamble) + "\n" + TestCode.str();
  llvm::Annotations AnnotatedCode(FullCode);

  TestInputs Inputs(AnnotatedCode.code());
  Inputs.Language = TestLanguage::Lang_CXX17;

  TestAST AST(Inputs);
  ASTContext& Ctx = AST.context();

  // Find the target function.
  auto MatchResult = ast_matchers::match(
      ast_matchers::functionDecl(ast_matchers::hasName("target"))
          .bind("target"),
      Ctx);
  ASSERT_FALSE(MatchResult.empty()) << "Could not find function 'target'";
  const auto* Target = MatchResult.front().getNodeAs<FunctionDecl>("target");

  auto CFG = dataflow::AdornedCFG::build(*Target);
  ASSERT_TRUE(!!CFG);

  dataflow::DataflowAnalysisContext::Options Opts;
  dataflow::DataflowAnalysisContext DACtx(
      std::make_unique<dataflow::WatchedLiteralsSolver>(), Opts);
  dataflow::Environment Env(DACtx, *Target);

  GoogleSqlValueNullabilityAnalysis Analysis(Ctx);

  std::set<unsigned> ActualLines;
  dataflow::CFGEltCallbacks<GoogleSqlValueNullabilityAnalysis> Callbacks;
  Callbacks.Before = [&](const CFGElement& Elt,
                         const dataflow::DataflowAnalysisState<
                             GoogleSqlValueNullabilityLattice>& State) {
    auto Diags = diagnoseGoogleSqlValueNullability(Elt, Ctx, State.Env);
    for (const Stmt* S : Diags) {
      ActualLines.insert(
          Ctx.getSourceManager().getPresumedLineNumber(S->getBeginLoc()));
    }
  };

  auto Result = dataflow::runDataflowAnalysis(*CFG, Analysis, Env, Callbacks);
  ASSERT_TRUE(!!Result);

  // Get expected lines from annotations (points).
  std::set<unsigned> ExpectedLines;
  for (const auto& Point : AnnotatedCode.points()) {
    ExpectedLines.insert(Ctx.getSourceManager().getPresumedLineNumber(
        Ctx.getSourceManager()
            .getLocForStartOfFile(Ctx.getSourceManager().getMainFileID())
            .getLocWithOffset(Point)));
  }

  EXPECT_THAT(ActualLines, testing::ContainerEq(ExpectedLines))
      << "Diagnostics mismatch. Expected at lines: "
      << testing::PrintToString(ExpectedLines)
      << ", but found at lines: " << testing::PrintToString(ActualLines);
}

namespace {

TEST(GoogleSqlValueNullabilityTest, DefaultConstructorIsUnsafe) {
  runTest(R"cpp(
    void target() {
      googlesql::Value v;
      v.value();  // ^
    }
  )cpp");
}

TEST(GoogleSqlValueNullabilityTest, FactoryMethodIsSafe) {
  runTest(R"cpp(
    void target() {
      googlesql::Value v = googlesql::Value::Int64(1);
      v.value();  // Safe! No annotation.
    }
  )cpp");
}

TEST(GoogleSqlValueNullabilityTest, ExplicitCheckIsSafe) {
  runTest(R"cpp(
    void target(googlesql::Value v) {
      if (!v.is_null()) {
        v.value();  // Safe!
      }
    }
  )cpp");
}

TEST(GoogleSqlValueNullabilityTest, UncheckedAccessIsUnsafe) {
  runTest(R"cpp(
    void target(googlesql::Value v) {
      v.value();  // ^ Unsafe!
    }
  )cpp");
}

TEST(GoogleSqlValueNullabilityTest, EarlyReturnIsSafe) {
  runTest(R"cpp(
    void target(googlesql::Value v) {
      if (v.is_null()) return;
      v.value();  // Safe!
    }
  )cpp");
}

TEST(GoogleSqlValueNullabilityTest, CheckedInBranchAccessIsSafe) {
  runTest(R"cpp(
    void target(googlesql::Value v) {
      if (v.is_null()) {
        // do nothing
      } else {
        v.value();  // Safe!
      }
    }
  )cpp");
}

TEST(GoogleSqlValueNullabilityTest, LogicalAndCheckIsSafe) {
  runTest(R"cpp(
    void target(googlesql::Value v) {
      if (!v.is_null() && v.value() > 0) {  // Safe!
        // ...
      }
    }
  )cpp");
}

TEST(GoogleSqlValueNullabilityTest, LogicalOrCheckIsUnsafe) {
  runTest(R"cpp(
    void target(googlesql::Value v) {
      if (!v.is_null() || v.value() > 0) {  // ^
        // ...
      }
    }
  )cpp");
}

TEST(GoogleSqlValueNullabilityTest, TernaryOperatorIsSafe) {
  runTest(R"cpp(
    int target(googlesql::Value v) {
      return v.is_null() ? 0 : v.value();  // Safe!
    }
  )cpp");
}

TEST(GoogleSqlValueNullabilityTest, ReassignmentClearsNullability) {
  runTest(R"cpp(
    void target() {
      googlesql::Value v;
      v.value();  // ^ Unsafe!

      v = googlesql::Value::Int64(1);
      v.value();  // Safe!

      v = googlesql::Value();
      v.value();  // ^ Unsafe again!
    }
  )cpp");
}

TEST(GoogleSqlValueNullabilityTest, ChainedAccessIsUnsafe) {
  runTest(R"cpp(
    void target(googlesql::Value v) {
      if (!v.is_null()) {
        googlesql::Value f = v.field(0);
        f.value();  // ^
      }
    }
  )cpp");
}

TEST(GoogleSqlValueNullabilityTest, ReferenceAliasingIsSafe) {
  runTest(R"cpp(
    void target(googlesql::Value v) {
      googlesql::Value& r = v;
      if (!r.is_null()) {
        v.value();  // Safe!
      }
    }
  )cpp");
}

TEST(GoogleSqlValueNullabilityTest, CopyConstructorPropagatesSafety) {
  runTest(R"cpp(
    void target(googlesql::Value v1) {
      if (!v1.is_null()) {
        googlesql::Value v2 = v1;
        v2.value();  // Safe!
      }
    }
  )cpp");
}

TEST(GoogleSqlValueNullabilityTest, LoopConditionCheckIsSafe) {
  runTest(R"cpp(
    void target(googlesql::Value v) {
      while (v.is_null()) {
        v = googlesql::Value::Int64(1);
      }
      v.value();  // Safe!
    }
  )cpp");
}

TEST(GoogleSqlValueNullabilityTest, DoubleNegationIsHandled) {
  runTest(R"cpp(
    void target(googlesql::Value v) {
      if (!!v.is_null()) {
        v.value();  // ^
      } else {
        v.value();  // Safe!
      }
    }
  )cpp");
}

TEST(GoogleSqlValueNullabilityTest, NullFactoryMethodIsUnsafe) {
  runTest(R"cpp(
    void target() {
      googlesql::Value v = googlesql::Value::NullInt64();
      v.value();  // ^
    }
  )cpp");
}

TEST(GoogleSqlValueNullabilityTest, CopyConstructorSharesState) {
  runTest(R"cpp(
    void target(googlesql::Value v1) {
      googlesql::Value v2 = v1;
      if (!v1.is_null()) {
        v2.value();  // Safe!
      }
    }
  )cpp");
}

}  // namespace
}  // namespace clang::tidy::nullability

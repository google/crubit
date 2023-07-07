// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/safety_constraint_generator.h"

#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/log/check.h"
#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_analysis.h"
#include "clang/AST/Decl.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/LLVM.h"
#include "third_party/llvm/llvm-project/clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/ADT/STLExtras.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

using ::std::replace;
using ::testing::ElementsAre;
using ::testing::IsEmpty;
using ::testing::Not;
using ::testing::UnorderedElementsAre;

// Returns names for nullability atoms of a function's params.
// Given a function `void foo(int *x)`, returns: {
//   Env.getPointerNullState(x).first => "x.is_known",
//   Env.getPointerNullState(x).second => "x.is_null",
// }
dataflow::Formula::AtomNames getNullabilityVariableNames(
    const FunctionDecl &Func, const dataflow::Environment &Env) {
  dataflow::Formula::AtomNames Result;
  for (unsigned I = 0; I < Func.param_size(); ++I) {
    auto &Param = *Func.getParamDecl(I);
    if (auto *Val = dyn_cast_or_null<clang::dataflow::PointerValue>(
            Env.getValue(Param));
        Val && hasPointerNullState(*Val)) {
      std::string Name = Param.getName().str();

      auto [Known, Null] = getPointerNullState(*Val);
      Result[Known.getAtom()] = Name + ".is_known";
      Result[Null.getAtom()] = Name + ".is_null";
    }
  }
  return Result;
}

// Analyzes the "target" function in the code, and returns safety constraints.
// These are expressed as strings (AtomicBoolValue* won't live long enough).
std::vector<std::string> getSafetyConstraints(llvm::StringRef Code) {
  using namespace ast_matchers;
  std::vector<std::string> Result;
  SafetyConstraintGenerator Generator;
  auto Inputs =
      dataflow::test::AnalysisInputs<PointerNullabilityAnalysis>(
          Code,
          functionDecl(hasName("target"), hasBody(compoundStmt()))
              .bind("target"),
          [&](ASTContext &Ctx, const dataflow::Environment &E) {
            return PointerNullabilityAnalysis(Ctx);
          })
          .withPostVisitCFG([&](ASTContext &AST, const CFGElement &Elt,
                                auto &&State) {
            Generator.collectConstraints(Elt, State.Lattice, State.Env, AST);
          });
  auto Err = dataflow::test::checkDataflow(
      std::move(Inputs), [&](const dataflow::test::AnalysisOutputs &Out) {
        auto Names = getNullabilityVariableNames(*Out.Target, Out.InitEnv);
        for (const auto *Constraint : Generator.constraints()) {
          std::string Textual;
          llvm::raw_string_ostream OS(Textual);
          Constraint->formula().print(OS, &Names);
          Result.push_back(std::move(Textual));
        }
      });
  CHECK(!Err) << toString(std::move(Err));
  return Result;
}

TEST(SafetyConstraintGenerator, GeneratesNoConstraintsForEmptyFunctionDefn) {
  EXPECT_THAT(getSafetyConstraints("void target() {}"), IsEmpty());
}

TEST(SafetyConstraintGenerator, GeneratesNoConstraintsForUnusedParam) {
  EXPECT_THAT(getSafetyConstraints("void target(int *p) {}"), IsEmpty());
}

TEST(SafetyConstraintGenerator, GeneratesNotIsNullConstraintForDeref) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p) { *p; }
  )cc";
  EXPECT_THAT(getSafetyConstraints(Src), ElementsAre("!p.is_null"));
}

TEST(SafetyConstraintGenerator,
     GeneratesNotIsNullConstraintForImproperlyGuardedDeref) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p) {
      if (p == nullptr) *p;
    }
  )cc";
  EXPECT_THAT(getSafetyConstraints(Src), ElementsAre("!p.is_null"));
}

TEST(SafetyConstraintGenerator, GeneratesConstraintsForAllParams) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p, int *q, int *r) {
      *p;
      *q;
      *r;
    }
  )cc";
  EXPECT_THAT(getSafetyConstraints(Src),
              UnorderedElementsAre("!p.is_null", "!q.is_null", "!r.is_null"));
}

TEST(SafetyConstraintGenerator, DoesntGenerateConstraintForNullCheckedPtr) {
  static constexpr llvm::StringRef Src = R"cc(
    void target(int *p) {
      if (p) *p;
      if (p != nullptr) *p;
    }
  )cc";
  EXPECT_THAT(getSafetyConstraints(Src), IsEmpty());
}

TEST(SafetyConstraintGenerator,
     ConstrainsParameterIfDereferencedBeforeAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    int *getPtr();

    void target(int *p) {
      *p;
      p = getPtr();
    }
  )cc";
  EXPECT_THAT(getSafetyConstraints(Src), ElementsAre("!p.is_null"));
}

TEST(SafetyConstraintGenerator,
     DoesNotConstrainParameterIfDereferencedAfterAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    int *getPtr();

    void target(int *p) {
      p = getPtr();
      *p;
    }
  )cc";
  // TODO(b/268440048) Figure out how to access and assert
  // equality for the constraint that this is.
  // (We require the value that models the getPtr() result to be non-null)
  EXPECT_THAT(getSafetyConstraints(Src), ElementsAre(Not("!p.is_null")));
}
}  // namespace
}  // namespace clang::tidy::nullability

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/safety_constraint_generator.h"

#include <memory>
#include <optional>
#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/log/check.h"
#include "nullability/inference/analyze_target_for_test.h"
#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pointer_nullability_lattice.h"
#include "clang/AST/Decl.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/ControlFlowContext.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "clang/Basic/LLVM.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"

namespace clang::tidy::nullability {
namespace {

using ::testing::AllOf;
using ::testing::IsEmpty;
using ::testing::Not;
using ::testing::SizeIs;
using ::testing::UnorderedElementsAre;

// Analyzes the to-be-matched code snippet with PointerNullabilityAnalysis, and
// then matches against the constraints produced by SafetyConstraintGenerator
// for the snippet.
//
// `ConstraintsMatcherProducer` should be a function taking the Environment
// and a vector of PointerValue* representing the Target function's parameters
// and returning a matcher for the generated constraints. This allows for
// matching of the constraints against individual parameter null state
// properties.
MATCHER_P(ProducesSafetyConstraints, ConstraintsMatcherProducer, "") {
  bool Success = false;
  auto MatcherProducer = ConstraintsMatcherProducer;
  analyzeTargetForTest(
      arg, [&Success, &MatcherProducer, &result_listener](
               const clang::FunctionDecl& Func,
               const clang::ast_matchers::MatchFinder::MatchResult& Result) {
        QCHECK(Func.hasBody()) << "Matched function has no body.";
        QCHECK(Result.Context) << "Missing ASTContext from match result.";
        llvm::Expected<clang::dataflow::ControlFlowContext> ControlFlowContext =
            clang::dataflow::ControlFlowContext::build(Func);
        clang::dataflow::DataflowAnalysisContext AnalysisContext(
            std::make_unique<clang::dataflow::WatchedLiteralsSolver>());
        clang::dataflow::Environment Environment(AnalysisContext, Func);
        PointerNullabilityAnalysis Analysis(*Result.Context);
        SafetyConstraintGenerator Generator;
        llvm::Expected<std::vector<std::optional<
            clang::dataflow::DataflowAnalysisState<PointerNullabilityLattice>>>>
            BlockToOutputStateOrError = clang::dataflow::runDataflowAnalysis(
                *ControlFlowContext, Analysis, Environment,
                [&Generator, &Result](
                    const clang::CFGElement& Element,
                    const clang::dataflow::DataflowAnalysisState<
                        PointerNullabilityLattice>& State) {
                  Generator.collectConstraints(Element, State, *Result.Context);
                });

        QCHECK(BlockToOutputStateOrError)
            << "No output state from dataflow analysis.";

        // TODO(b/268440048) When we can retrieve the improved atoms
        // representing annotations, stop using the Environment to retrieve the
        // initial Environment PointerValues, which won't work for local
        // variables down the road.
        std::vector<clang::dataflow::PointerValue*> ParamDeclPointerValues;
        for (const auto* P : Func.parameters()) {
          CHECK(P != nullptr);
          auto* Val = clang::dyn_cast_or_null<clang::dataflow::PointerValue>(
              Environment.getValue(*P));
          if (Val) {
            ParamDeclPointerValues.push_back(Val);
          }
        }
        Success = ExplainMatchResult(
            MatcherProducer(Environment, ParamDeclPointerValues),
            Generator.constraints(), result_listener);
      });
  return Success;
}

TEST(SafetyConstraintGenerator, GeneratesNoConstraintsForEmptyFunctionDefn) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target() {}
  )cc";
  EXPECT_THAT(Src, ProducesSafetyConstraints(
                       [](const dataflow::Environment& Environment,
                          auto ParamPointerValues) { return IsEmpty(); }));
}

TEST(SafetyConstraintGenerator, GeneratesNoConstraintsForUnusedParam) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target(int* p) {}
  )cc";
  EXPECT_THAT(Src, ProducesSafetyConstraints(
                       [](const dataflow::Environment& Environment,
                          auto ParamPointerValues) { return IsEmpty(); }));
}

TEST(SafetyConstraintGenerator, GeneratesNotIsNullConstraintForDeref) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target(int* p) { *p; }
  )cc";
  EXPECT_THAT(
      Src,
      ProducesSafetyConstraints([](const dataflow::Environment& Environment,
                                   auto ParamPointerValues) {
        return UnorderedElementsAre(&Environment.makeNot(
            getPointerNullState(*ParamPointerValues[0]).second));
      }));
}

TEST(SafetyConstraintGenerator,
     GeneratesNotIsNullConstraintForImproperlyGuardedDeref) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target(int* p) {
      if (p == nullptr) *p;
    }
  )cc";
  EXPECT_THAT(
      Src,
      ProducesSafetyConstraints([](const dataflow::Environment& Environment,
                                   auto ParamPointerValues) {
        return UnorderedElementsAre(&Environment.makeNot(
            getPointerNullState(*ParamPointerValues[0]).second));
      }));
}

TEST(SafetyConstraintGenerator, GeneratesConstraintsForAllParams) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target(int* p, int* q, int* r) {
      *p;
      *q;
      *r;
    }
  )cc";
  EXPECT_THAT(Src, ProducesSafetyConstraints([](const dataflow::Environment&
                                                    Environment,
                                                auto ParamPointerValues) {
                return UnorderedElementsAre(
                    &Environment.makeNot(
                        getPointerNullState(*ParamPointerValues[0]).second),
                    &Environment.makeNot(
                        getPointerNullState(*ParamPointerValues[1]).second),
                    &Environment.makeNot(
                        getPointerNullState(*ParamPointerValues[2]).second));
              }));
}

TEST(SafetyConstraintGenerator, DoesntGenerateConstraintForNullCheckedPtr) {
  static constexpr llvm::StringRef Src = R"cc(
    void Target(int* p) {
      if (p) *p;
      if (p != nullptr) *p;
    }
  )cc";
  EXPECT_THAT(Src, ProducesSafetyConstraints(
                       [](const dataflow::Environment& Environment,
                          auto ParamPointerValues) { return IsEmpty(); }));
}

TEST(SafetyConstraintGenerator,
     ConstrainsParameterIfDereferencedBeforeAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    int* getPtr();

    void Target(int* p) {
      *p;
      p = getPtr();
    }
  )cc";
  EXPECT_THAT(
      Src,
      ProducesSafetyConstraints([](const dataflow::Environment& Environment,
                                   auto ParamPointerValues) {
        return UnorderedElementsAre(&Environment.makeNot(
            getPointerNullState(*ParamPointerValues[0]).second));
      }));
}

TEST(SafetyConstraintGenerator,
     DoesNotConstrainParameterIfDereferencedAfterAssignment) {
  static constexpr llvm::StringRef Src = R"cc(
    int* getPtr();

    void Target(int* p) {
      p = getPtr();
      *p;
    }
  )cc";
  EXPECT_THAT(
      Src,
      ProducesSafetyConstraints([](const dataflow::Environment& Environment,
                                   auto ParamPointerValues) {
        return AllOf(SizeIs(1),
                     // TODO(b/268440048) Figure out how to access and assert
                     // equality for the constraint that this is.
                     Not(UnorderedElementsAre(&Environment.makeNot(
                         getPointerNullState(*ParamPointerValues[0]).second))));
      }));
}
}  // namespace
}  // namespace clang::tidy::nullability

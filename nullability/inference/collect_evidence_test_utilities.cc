// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/collect_evidence_test_utilities.h"

#include <string>
#include <utility>
#include <vector>

#include "nullability/inference/augmented_test_inputs.h"
#include "nullability/inference/collect_evidence.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/usr_cache.h"
#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pragma.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/DeclBase.h"
#include "clang/Testing/TestAST.h"
#include "third_party/llvm/llvm-project/clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/Testing/Support/Error.h"
#include "external/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
MATCHER_P4(isEvidenceMatcher, SlotMatcher, KindMatcher, SymbolMatcher,
           CrossesFromTestToNontestMatcher, "") {
  return SlotMatcher.Matches(static_cast<Slot>(arg.slot())) &&
         KindMatcher.Matches(arg.kind()) &&
         SymbolMatcher.Matches(arg.symbol()) &&
         CrossesFromTestToNontestMatcher.Matches(
             arg.crosses_from_test_to_nontest());
}

MATCHER(notPropagated, "") { return !arg.has_propagated_from(); }

MATCHER_P(propagatedFrom, PropagatedFromMatcher, "") {
  return PropagatedFromMatcher.Matches(arg.propagated_from());
}
}  // namespace

testing::Matcher<const Evidence&> evidence(
    testing::Matcher<Slot> S, testing::Matcher<Evidence::Kind> Kind,
    testing::Matcher<const Symbol&> SymbolMatcher,
    testing::Matcher<bool> CrossesFromTestToNontest) {
  return AllOf(
      isEvidenceMatcher(S, Kind, SymbolMatcher, CrossesFromTestToNontest),
      notPropagated());
}

testing::Matcher<const Evidence&> evidencePropagatedFrom(
    testing::Matcher<const Symbol&> PropagatedFromMatcher,
    testing::Matcher<Slot> S, testing::Matcher<Evidence::Kind> Kind,
    testing::Matcher<const Symbol&> SymbolMatcher,
    testing::Matcher<bool> CrossesFromTestToNontest) {
  return AllOf(
      isEvidenceMatcher(S, Kind, SymbolMatcher, CrossesFromTestToNontest),
      propagatedFrom(PropagatedFromMatcher));
}

std::string printToString(DefinitionCollectionMode Mode) {
  switch (Mode) {
    case DefinitionCollectionMode::kTestWithSummaries:
      return "WithSummaries";
    case DefinitionCollectionMode::kTestDirectly:
      return "Directly";
  }
  llvm_unreachable("Unknown CollectionMode");
}

static llvm::Expected<CFGSummary> summarizeDefinitionNamed(
    llvm::StringRef TargetName, llvm::StringRef Source) {
  USRCache UsrCache;
  NullabilityPragmas Pragmas;
  clang::TestAST AST(getAugmentedTestInputs(Source, Pragmas));
  const Decl& Definition =
      *dataflow::test::findValueDecl(AST.context(), TargetName);
  return summarizeDefinition(Definition, UsrCache, Pragmas);
}

llvm::Expected<CFGSummary> summarizeTargetFuncDefinition(
    llvm::StringRef Source) {
  return summarizeDefinitionNamed("target", Source);
}

std::pair<llvm::Error, std::vector<Evidence>>
collectFromDefinitionViaSummaryWithErrors(
    clang::ASTContext& ASTCtx, const Decl& Definition,
    const NullabilityPragmas& Pragmas,
    const PreviousInferences& InputInferences,
    const SolverFactory& MakeSolver) {
  USRCache UsrCache;
  std::vector<Evidence> Results;
  auto Summary = summarizeDefinition(Definition, UsrCache, Pragmas, MakeSolver);
  if (!Summary) return {Summary.takeError(), Results};

  // In the context of a pipeline, the index would be created from the AST and
  // then serialized to proto, along with the summaries. We round-trip the index
  // here to ensure proper testing of the full save/restore flow.
  VirtualMethodIndex VMI = getVirtualMethodIndex(ASTCtx, UsrCache);
  VirtualMethodIndexSummary VMIProto = saveVirtualMethodsIndex(VMI);

  VirtualMethodIndex PostVMI = loadVirtualMethodsIndex(VMIProto);
  PostVMI.Overrides = std::move(VMI.Overrides);
  return {collectEvidenceFromSummary(
              *Summary,
              evidenceEmitterWithPropagation(
                  [&Results](const Evidence& E) { Results.push_back(E); },
                  std::move(PostVMI)),
              InputInferences, MakeSolver),
          Results};
}

static std::vector<Evidence> collectFromDefinitionViaSummary(
    clang::ASTContext& ASTCtx, const Decl& Definition,
    const NullabilityPragmas& Pragmas, PreviousInferences InputInferences) {
  auto [Err, Results] = collectFromDefinitionViaSummaryWithErrors(
      ASTCtx, Definition, Pragmas, InputInferences);
  if (Err) {
    // Can't assert from within a non-void helper function, so only ADD_FAILURE.
    ADD_FAILURE() << "Error encountered in collection via summary: " << Err;
    return {};
  }
  return Results;
}

static std::vector<Evidence> collectFromDefinitionDirectly(
    clang::ASTContext& ASTCtx, const Decl& Definition,
    const NullabilityPragmas& Pragmas,
    PreviousInferences InputInferences = {}) {
  std::vector<Evidence> Results;
  USRCache UsrCache;
  // Can't assert from within a non-void helper function, so only EXPECT.
  EXPECT_THAT_ERROR(
      collectEvidenceFromDefinition(
          Definition,
          evidenceEmitterWithPropagation(
              [&Results](Evidence E) { Results.push_back(std::move(E)); },
              UsrCache, ASTCtx),
          UsrCache, Pragmas, InputInferences),
      llvm::Succeeded());
  return Results;
}

std::vector<Evidence> collectFromDefinition(
    clang::ASTContext& ASTCtx, const Decl& Definition,
    const NullabilityPragmas& Pragmas, DefinitionCollectionMode Mode,
    PreviousInferences InputInferences) {
  switch (Mode) {
    case DefinitionCollectionMode::kTestWithSummaries:
      return collectFromDefinitionViaSummary(ASTCtx, Definition, Pragmas,
                                             InputInferences);
    case DefinitionCollectionMode::kTestDirectly:
      return collectFromDefinitionDirectly(ASTCtx, Definition, Pragmas,
                                           InputInferences);
  }
  llvm_unreachable("Unexpected collection mode");
}

std::vector<Evidence> collectFromDefinitionNamed(
    llvm::StringRef TargetName, llvm::StringRef Source,
    DefinitionCollectionMode Mode, PreviousInferences InputInferences) {
  NullabilityPragmas Pragmas;
  clang::TestAST AST(getAugmentedTestInputs(Source, Pragmas));
  const Decl& Definition =
      *dataflow::test::findValueDecl(AST.context(), TargetName);
  return collectFromDefinition(AST.context(), Definition, Pragmas, Mode,
                               InputInferences);
}

std::vector<Evidence> collectFromTargetFuncDefinition(
    llvm::StringRef Source, DefinitionCollectionMode Mode,
    PreviousInferences InputInferences) {
  return collectFromDefinitionNamed("target", Source, Mode, InputInferences);
}
std::vector<Evidence> collectFromDecl(llvm::StringRef Source,
                                      llvm::StringRef DeclName) {
  std::vector<Evidence> Results;
  NullabilityPragmas Pragmas;
  clang::TestAST AST(getAugmentedTestInputs(Source, Pragmas));
  USRCache USRCache;
  collectEvidenceFromTargetDeclaration(
      *dataflow::test::findValueDecl(AST.context(), DeclName),
      evidenceEmitterWithPropagation(
          [&Results](Evidence E) { Results.push_back(std::move(E)); }, USRCache,
          AST.context()),
      USRCache, Pragmas);
  return Results;
}

std::vector<Evidence> collectFromTargetVarDecl(llvm::StringRef Source) {
  return collectFromDecl(Source, "Target");
}

std::vector<Evidence> collectFromTargetFuncDecl(llvm::StringRef Source) {
  return collectFromDecl(Source, "target");
}
}  // namespace clang::tidy::nullability

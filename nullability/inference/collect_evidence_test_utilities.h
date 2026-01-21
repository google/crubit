// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_NULLABILITY_INFERENCE_COLLECT_EVIDENCE_TEST_UTILITIES_H_
#define THIRD_PARTY_CRUBIT_NULLABILITY_INFERENCE_COLLECT_EVIDENCE_TEST_UTILITIES_H_

#include <optional>
#include <utility>
#include <vector>

#include "nullability/inference/augmented_test_inputs.h"
#include "nullability/inference/collect_evidence.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pragma.h"
#include "clang/AST/DeclBase.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/Support/Regex.h"
#include "llvm/Support/raw_ostream.h"
#include "external/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {

/// Matches Symbols with the USR of a function with the given name.
MATCHER_P(functionNamed, Name, "") {
  return llvm::StringRef(arg.usr()).contains(
      ("@" + llvm::Twine(Name) + "#").str());
}

/// Matches Symbols with the USR of a function template with the given name.
MATCHER_P(functionTemplateNamed, Name, "") {
  return llvm::Regex((".*@FT@>[0-9]+(#.*)*" + llvm::Twine(Name) + "#.*").str())
      .match(arg.usr());
}

/// Matches Symbols with the USR of non-static field with the given name.
/// The name should be of the form "MyStruct::field", but it should be qualified
/// only by the enclosing type, not any namespaces.
MATCHER_P(fieldNamed, TypeQualifiedFieldName, "") {
  const auto [TypeName, FieldName] =
      llvm::StringRef(TypeQualifiedFieldName).split("::");
  return arg.usr().ends_with(("@S@" + TypeName + "@FI@" + FieldName).str()) ||
         arg.usr().ends_with(("@U@" + TypeName + "@FI@" + FieldName).str());
}

/// Matches Symbols with the USR of a static field with the given name.
/// The name should be of the form "MyStruct::field" (see also comment for
/// `fieldNamed()`).
MATCHER_P(staticFieldNamed, TypeQualifiedFieldName, "") {
  const auto [TypeName, FieldName] =
      llvm::StringRef(TypeQualifiedFieldName).split("::");
  return arg.usr().ends_with(("@S@" + TypeName + "@" + FieldName).str());
}

/// Matches Symbols with the USR of a global variable with the given name.
MATCHER_P(globalVarNamed, Name, "") {
  return arg.usr() == ("c:@" + llvm::Twine(Name)).str();
}

/// Matches Symbols with the USR of a local variable with the name `VarName`
/// declared inside a function named `FunctionName`.
MATCHER_P2(localVarNamedImpl, VarName, FunctionName, "") {
  return llvm::StringRef(arg.usr()).contains(
             ("@F@" + llvm::Twine(FunctionName) + "#").str()) &&
         arg.usr().ends_with(("@" + llvm::Twine(VarName)).str());
}

/// Returns a matcher that matches Symbols with the USR of a local variable with
/// the name `VarName` declared inside a function named `FunctionName`.
inline auto localVarNamed(llvm::StringRef VarName,
                          llvm::StringRef FunctionName = "target") {
  return localVarNamedImpl(VarName, FunctionName);
}

/// Returns a matcher for non-propagated Evidence, checking slot number, Kind,
/// Symbol, and whether the Evidence is collected from a test file but
/// contributes to the inference for a slot in a non-test file.
testing::Matcher<const Evidence&> evidence(
    testing::Matcher<Slot> S, testing::Matcher<Evidence::Kind> Kind,
    testing::Matcher<const Symbol&> SymbolMatcher = functionNamed("target"),
    testing::Matcher<bool> CrossesFromTestToNontest = false);

/// Returns a matcher for propagated Evidence, checking the propagation source,
/// slot number, Kind, Symbol, and whether the Evidence is collected from a test
/// file but contributes to the inference for a slot in a non-test file.
testing::Matcher<const Evidence&> evidencePropagatedFrom(
    testing::Matcher<const Symbol&> PropagatedFromMatcher,
    testing::Matcher<Slot> S, testing::Matcher<Evidence::Kind> Kind,
    testing::Matcher<const Symbol&> SymbolMatcher = functionNamed("target"),
    testing::Matcher<bool> CrossesFromTestToNontest = false);

/// Summarizes the definition in `Source` with the name "target" (a generic name
/// using the LLVM-style capitalization for function names).
llvm::Expected<std::optional<CFGSummary>> summarizeTargetFuncDefinition(
    llvm::StringRef Source);

/// Collect evidence from the given `Definition`.
/// Returns both an error and a vector to represent partial computations --
/// those that fail after producing some results.
std::pair<llvm::Error, std::vector<Evidence>> collectFromDefinitionWithErrors(
    ASTContext& ASTCtx, const Decl& Definition,
    const NullabilityPragmas& Pragmas,
    const PreviousInferences& InputInferences,
    const SolverFactory& MakeSolver = makeDefaultSolverForInference);

/// Collects evidence from the given `Definition`.
std::vector<Evidence> collectFromDefinition(
    ASTContext& ASTCtx, const Decl& Definition,
    const NullabilityPragmas& Pragmas, PreviousInferences InputInferences = {});

/// Constructs an AST from `Source` and the necessary matcher to retrieve the
/// definition named `TargetName`, then collects evidence from that definition.
std::vector<Evidence> collectFromDefinitionNamed(
    llvm::StringRef TargetName, llvm::StringRef Source,
    PreviousInferences InputInferences = {});

/// Provides a default LLVM-style function-name-cased value ("target") for
/// TargetName in `collectFromDefinitionNamed`, which puts `TargetName` first
/// for readability.
std::vector<Evidence> collectFromTargetFuncDefinition(
    llvm::StringRef Source, PreviousInferences InputInferences = {});

/// Constructs an AST from `Source` and collects evidence from the definition
/// retrieved using `Matcher`.
template <typename MatcherT>
std::vector<Evidence> collectFromDefinitionMatching(
    MatcherT Matcher, llvm::StringRef Source,
    PreviousInferences InputInferences = {}) {
  NullabilityPragmas Pragmas;
  TestAST AST(getAugmentedTestInputs(Source, Pragmas));
  const Decl& Definition = *selectFirst<Decl>(
      "d", ::clang::ast_matchers::match(Matcher.bind("d"), AST.context()));
  return collectFromDefinition(AST.context(), Definition, Pragmas,
                               InputInferences);
}

/// Collects evidence from the declaration in `Source` with the name `DeclName`.
std::vector<Evidence> collectFromDecl(llvm::StringRef Source,
                                      llvm::StringRef DeclName);

/// Provides a default LLVM-style variable-name-cased value ("Target") for
/// `DeclName` in `collectFromDecl`.
std::vector<Evidence> collectFromTargetVarDecl(llvm::StringRef Source);

/// Provides a default LLVM-style function-name-cased value ("target") for
/// `DeclName` in `collectFromDecl`.
std::vector<Evidence> collectFromTargetFuncDecl(llvm::StringRef Source);

}  // namespace clang::tidy::nullability

#endif  // THIRD_PARTY_CRUBIT_NULLABILITY_INFERENCE_COLLECT_EVIDENCE_TEST_UTILITIES_H_

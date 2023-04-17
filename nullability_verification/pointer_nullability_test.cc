// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/pointer_nullability.h"

#include "clang/Testing/TestAST.h"
#include "llvm/ADT/StringRef.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using testing::ElementsAre;

class GetNullabilityAnnotationsFromTypeTest : public ::testing::Test {
 protected:
  // C++ declarations prepended before parsing type in nullVec().
  std::string Preamble;

  // Parses `Type` and returns getNullabilityAnnotationsFromType().
  std::vector<NullabilityKind> nullVec(llvm::StringRef Type) {
    clang::TestAST AST((Preamble + "\nusing Target = " + Type + ";").str());
    auto Target = AST.context().getTranslationUnitDecl()->lookup(
        &AST.context().Idents.get("Target"));
    assert(Target.isSingleResult());
    QualType TargetType =
        AST.context().getTypedefType(Target.find_first<TypeAliasDecl>());
    return getNullabilityAnnotationsFromType(TargetType);
  }
};

TEST_F(GetNullabilityAnnotationsFromTypeTest, Pointers) {
  EXPECT_THAT(nullVec("int"), ElementsAre());
  EXPECT_THAT(nullVec("int *"), ElementsAre(NullabilityKind::Unspecified));
  EXPECT_THAT(nullVec("int **"), ElementsAre(NullabilityKind::Unspecified,
                                             NullabilityKind::Unspecified));
  EXPECT_THAT(nullVec("int *_Nullable*_Nonnull"),
              ElementsAre(NullabilityKind::NonNull, NullabilityKind::Nullable));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, Sugar) {
  Preamble = "using X = int* _Nonnull;";

  EXPECT_THAT(nullVec("X"), ElementsAre(NullabilityKind::NonNull));
  EXPECT_THAT(nullVec("X*"), ElementsAre(NullabilityKind::Unspecified,
                                         NullabilityKind::NonNull));

  EXPECT_THAT(nullVec("X(*)"), ElementsAre(NullabilityKind::Unspecified,
                                           NullabilityKind::NonNull));
}

}  // namespace
}  // namespace clang::tidy::nullability

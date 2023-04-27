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
    CHECK(Target.isSingleResult());
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

TEST_F(GetNullabilityAnnotationsFromTypeTest, AliasTemplates) {
  Preamble = R"cpp(
    template <typename T>
    using Nullable = T _Nullable;
    template <typename T>
    using Nonnull = T _Nonnull;
  )cpp";
  EXPECT_THAT(nullVec("Nullable<int*>"),
              ElementsAre(NullabilityKind::Nullable));

  EXPECT_THAT(
      nullVec("Nullable<Nullable<int*>*>"),
      ElementsAre(NullabilityKind::Nullable, NullabilityKind::Nullable));

  EXPECT_THAT(nullVec("Nullable<Nullable<Nonnull<int*>*>*>"),
              ElementsAre(NullabilityKind::Nullable, NullabilityKind::Nullable,
                          NullabilityKind::NonNull));

  Preamble = R"cpp(
    template <typename T, typename U>
    struct Pair;
    template <typename T>
    using Two = Pair<T, T>;
  )cpp";
  EXPECT_THAT(
      nullVec("Two<int* _Nullable>"),
      ElementsAre(NullabilityKind::Nullable, NullabilityKind::Nullable));

  Preamble = R"cpp(
    template <typename T1>
    using A = T1* _Nullable;
    template <typename T2>
    using B = A<T2>* _Nonnull;
  )cpp";
  EXPECT_THAT(nullVec("B<int>"),
              ElementsAre(NullabilityKind::NonNull, NullabilityKind::Nullable));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, TypesInClassTemplates) {
  Preamble = R"cpp(
    template <class T>
    struct Nullable {
      using type = T _Nullable;
    };
  )cpp";
  // TODO: should be [Nullable, Nonnull].
  // We're not making use of the template arg list from the ElaboratedType.
  EXPECT_THAT(
      nullVec("Nullable<int* _Nonnull *>::type"),
      ElementsAre(NullabilityKind::Nullable, NullabilityKind::Unspecified));
}

}  // namespace
}  // namespace clang::tidy::nullability

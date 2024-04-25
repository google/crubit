// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/type_nullability.h"

#include <algorithm>
#include <memory>
#include <optional>
#include <string>
#include <tuple>
#include <utility>
#include <vector>

#include "absl/log/check.h"
#include "nullability/pragma.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/AST/Decl.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeLoc.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Frontend/FrontendActions.h"
#include "clang/Lex/Lexer.h"
#include "clang/Testing/CommandLineArgs.h"
#include "clang/Testing/TestAST.h"
#include "clang/Tooling/Transformer/SourceCode.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Testing/Annotations/Annotations.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using ::llvm::Annotations;
using ::testing::ElementsAre;
using ::testing::FieldsAre;
using ::testing::IsEmpty;
using ::testing::Matcher;
using ::testing::Optional;

static test::EnableSmartPointers Enable;

class PointerTypeTest : public ::testing::Test {
 protected:
  QualType underlying(llvm::StringRef Name, TestAST& AST) {
    auto Lookup = AST.context().getTranslationUnitDecl()->lookup(
        &AST.context().Idents.get(Name));
    EXPECT_TRUE(Lookup.isSingleResult());
    return Lookup.find_first<TypeAliasDecl>()->getUnderlyingType();
  }
};

TEST_F(PointerTypeTest, IsSupportedRawPointerType) {
  TestAST AST(R"cpp(
    using NotPointer = int;
    using Pointer = NotPointer*;
    using FuncPointer = Pointer (*)(Pointer);
    using SugaredPointer = Pointer;

    struct S;
    using PointerDataMember = Pointer S::*;
    using PointerMemberFunction = Pointer (S::*)(Pointer);

    @class X;
    using ObjCPointer = X;

    template <class>
    struct Container;
    using ContainsPointers = Container<int*>;

    namespace std {
    template <typename T>
    class unique_ptr;
    }
    using UniquePointer = std::unique_ptr<NotPointer>;
  )cpp");

  EXPECT_FALSE(isSupportedRawPointerType(underlying("NotPointer", AST)));
  EXPECT_TRUE(isSupportedRawPointerType(underlying("Pointer", AST)));
  EXPECT_TRUE(isSupportedRawPointerType(underlying("FuncPointer", AST)));
  EXPECT_TRUE(isSupportedRawPointerType(underlying("SugaredPointer", AST)));
  EXPECT_FALSE(isSupportedRawPointerType(underlying("PointerDataMember", AST)));
  EXPECT_FALSE(
      isSupportedRawPointerType(underlying("PointerMemberFunction", AST)));
  EXPECT_FALSE(isSupportedRawPointerType(underlying("ObjCPointer", AST)));
  EXPECT_FALSE(isSupportedRawPointerType(underlying("ContainsPointers", AST)));
  EXPECT_FALSE(isSupportedRawPointerType(underlying("UniquePointer", AST)));
}

TEST_F(PointerTypeTest, IsSupportedSmartPointerType) {
  TestAST AST(R"cpp(
    namespace std {
    template <typename T>
    class unique_ptr;
    template <typename T>
    class shared_ptr;
    template <typename T>
    class weak_ptr;
    }  // namespace std
    template <typename T>
    class unique_ptr;

    using NotPointer = int;
    using UniquePointer = std::unique_ptr<NotPointer>;
    using SharedPointer = std::shared_ptr<NotPointer>;
    using WeakPointer = std::weak_ptr<NotPointer>;

    using UniquePointerWrongNamespace = ::unique_ptr<NotPointer>;

    using SugaredPointer = UniquePointer;

    template <typename T>
    struct PublicDerived : public std::unique_ptr<T> {};
    template <typename T>
    struct PrivateDerived : private std::unique_ptr<T> {};
    using PublicDerivedPointer = PublicDerived<int>;
    using PrivateDerivedPointer = PrivateDerived<int>;

    template <typename T>
    struct UserDefinedSmartPointer {
      using absl_nullability_compatible = void;
    };
    using UserDefined = UserDefinedSmartPointer<NotPointer>;

    template <class>
    struct Container;
    using ContainsPointers = Container<std::unique_ptr<int>>;
  )cpp");

  EXPECT_FALSE(isSupportedSmartPointerType(underlying("NotPointer", AST)));
  EXPECT_TRUE(isSupportedSmartPointerType(underlying("UniquePointer", AST)));
  EXPECT_TRUE(isSupportedSmartPointerType(underlying("SharedPointer", AST)));
  EXPECT_FALSE(isSupportedSmartPointerType(underlying("WeakPointer", AST)));
  EXPECT_FALSE(isSupportedSmartPointerType(
      underlying("UniquePointerWrongNamespace", AST)));
  EXPECT_TRUE(isSupportedSmartPointerType(underlying("SugaredPointer", AST)));
  EXPECT_TRUE(isSupportedSmartPointerType(underlying("UserDefined", AST)));
  EXPECT_TRUE(
      isSupportedSmartPointerType(underlying("PublicDerivedPointer", AST)));
  EXPECT_FALSE(
      isSupportedSmartPointerType(underlying("PrivateDerivedPointer", AST)));
  EXPECT_FALSE(
      isSupportedSmartPointerType(underlying("ContainsPointers", AST)));
}

using UnderlyingRawPointerTest = PointerTypeTest;

TEST_F(UnderlyingRawPointerTest, Instantiated) {
  // Test the case where the smart pointer type is instantiated and
  // `underlyingRawPointerType()` therefore looks at the type aliases `pointer`
  // or `element_type`.
  // To test that we're really looking at these type aliases, make them refer to
  // `char *` / `char`, independent of the template argument.
  TestAST AST(R"cpp(
    namespace std {
    template <typename T>
    class unique_ptr {
      using pointer = char *;
    };
    template <typename T>
    class shared_ptr {
      using element_type = char;
    };
    }  // namespace std

    template <typename T>
    struct PublicDerived : public std::unique_ptr<T> {};
    template <typename T>
    struct PrivateDerived : private std::unique_ptr<T> {};

    template <typename T>
    struct UserDefinedSmartPointer {
      using absl_nullability_compatible = void;
      using pointer = char *;
    };

    template <int i>
    struct Recursive : public Recursive<i - 1> {};
    template <>
    class Recursive<0> {};

    template <int i>
    struct IndirectRecursive;
    template <int i>
    struct Base : public IndirectRecursive<i - 1> {};
    template <int i>
    struct IndirectRecursive : public Base<i> {};
    template <>
    struct IndirectRecursive<0> {};

    using UniquePointer = std::unique_ptr<int>;
    using SharedPointer = std::shared_ptr<int>;
    using PublicDerivedPointer = PublicDerived<int>;
    using PrivateDerivedPointer = PrivateDerived<int>;
    using UserDefined = UserDefinedSmartPointer<int>;
    using Recursive2 = Recursive<2>;
    using IndirectRecursive2 = IndirectRecursive<2>;
    // Force the compiler to instantiate the templates. Otherwise, the
    // `ClassTemplateSpecializationDecl` won't contain a `TypedefNameDecl` for
    // `pointer` or `element_type`, and `underlyingRawPointerType()` will
    // use the fallback behavior of looking at the template argument.
    template class std::unique_ptr<int>;
    template class std::shared_ptr<int>;
    template class PublicDerived<int>;
    template class PrivateDerived<int>;
    template class UserDefinedSmartPointer<int>;
    template class Recursive<2>;
    template class IndirectRecursive<2>;
  )cpp");

  QualType PointerToCharTy = AST.context().getPointerType(AST.context().CharTy);
  EXPECT_EQ(underlyingRawPointerType(underlying("UniquePointer", AST)),
            PointerToCharTy);
  EXPECT_EQ(underlyingRawPointerType(underlying("SharedPointer", AST)),
            PointerToCharTy);
  EXPECT_EQ(underlyingRawPointerType(underlying("PublicDerivedPointer", AST)),
            PointerToCharTy);
  EXPECT_TRUE(underlyingRawPointerType(underlying("PrivateDerivedPointer", AST))
                  .isNull());
  EXPECT_EQ(underlyingRawPointerType(underlying("PrivateDerivedPointer", AST),
                                     AS_private),
            PointerToCharTy);
  EXPECT_EQ(underlyingRawPointerType(underlying("UserDefined", AST)),
            PointerToCharTy);
  EXPECT_TRUE(underlyingRawPointerType(underlying("Recursive2", AST)).isNull());
  EXPECT_TRUE(
      underlyingRawPointerType(underlying("IndirectRecursive2", AST)).isNull());
}

TEST_F(UnderlyingRawPointerTest, NotInstantiated) {
  // Test the fallback behavior for `underlyingRawPointerType()` where the smart
  // pointer type is not instantiated. (In fact, we can't even see the template
  // definition here.)
  TestAST AST(R"cpp(
    namespace std {
    template <typename T>
    class unique_ptr;
    template <typename T>
    class shared_ptr;
    }  // namespace std

    using UniquePointer = std::unique_ptr<int>;
    using ArrayUniquePointer = std::unique_ptr<int[]>;
    using SharedPointer = std::shared_ptr<int>;
    using ArraySharedPointer = std::shared_ptr<int[]>;

    template <typename T>
    struct PublicDerived : public std::unique_ptr<T> {};
    using PublicDerivedPointer = PublicDerived<int>;

    template <typename T>
    struct PrivateDerived : private std::unique_ptr<T> {};
    using PrivateDerivedPointer = PrivateDerived<int>;

    template <typename T>
    using Nullable [[clang::annotate("Nullable")]] = T;
    using NullableUniquePointer = Nullable<std::unique_ptr<int>>;

    template <int i>
    struct Recursive : public Recursive<i - 1> {};
    template <>
    class Recursive<0> {};
    using Recursive2 = Recursive<2>;

    template <int i>
    struct IndirectRecursive;
    template <int i>
    struct Base : public IndirectRecursive<i - 1> {};
    template <int i>
    struct IndirectRecursive : public Base<i> {};
    template <>
    struct IndirectRecursive<0> {};
    using IndirectRecursive2 = IndirectRecursive<2>;
  )cpp");

  QualType PointerToIntTy = AST.context().getPointerType(AST.context().IntTy);
  EXPECT_EQ(underlyingRawPointerType(underlying("UniquePointer", AST)),
            PointerToIntTy);
  EXPECT_EQ(underlyingRawPointerType(underlying("ArrayUniquePointer", AST)),
            PointerToIntTy);
  EXPECT_EQ(underlyingRawPointerType(underlying("SharedPointer", AST)),
            PointerToIntTy);
  EXPECT_EQ(underlyingRawPointerType(underlying("ArraySharedPointer", AST)),
            PointerToIntTy);

  EXPECT_EQ(underlyingRawPointerType(underlying("PublicDerivedPointer", AST)),
            PointerToIntTy);

  EXPECT_TRUE(underlyingRawPointerType(underlying("PrivateDerivedPointer", AST))
                  .isNull());
  EXPECT_EQ(underlyingRawPointerType(underlying("PrivateDerivedPointer", AST),
                                     AS_private),
            PointerToIntTy);

  EXPECT_EQ(underlyingRawPointerType(underlying("NullableUniquePointer", AST)),
            PointerToIntTy);

  EXPECT_TRUE(underlyingRawPointerType(underlying("Recursive2", AST)).isNull());
  EXPECT_TRUE(
      underlyingRawPointerType(underlying("IndirectRecursive2", AST)).isNull());
}

class GetTypeNullabilityTest : public ::testing::Test {
 protected:
  // C++ declarations prepended before parsing type in nullVec().
  TestInputs Inputs;
  std::string &Header;
  std::string Preamble;

  GetTypeNullabilityTest() : Header(Inputs.ExtraFiles["header.h"]) {
    Inputs.ExtraArgs.push_back("-include");
    Inputs.ExtraArgs.push_back("header.h");
  }

  // Parses `Type` and returns getTypeNullability().
  TypeNullability nullVec(llvm::StringRef Type) {
    NullabilityPragmas Pragmas;
    Inputs.Code = (Preamble + "\nusing Target = " + Type + ";").str();
    Inputs.MakeAction = [&] {
      struct Action : public SyntaxOnlyAction {
        NullabilityPragmas &Pragmas;
        Action(NullabilityPragmas &Pragmas) : Pragmas(Pragmas) {}

        std::unique_ptr<ASTConsumer> CreateASTConsumer(
            CompilerInstance &CI, llvm::StringRef File) override {
          registerPragmaHandler(CI.getPreprocessor(), Pragmas);
          return SyntaxOnlyAction::CreateASTConsumer(CI, File);
        }
      };
      return std::make_unique<Action>(Pragmas);
    };
    TestAST AST(Inputs);
    auto Target = AST.context().getTranslationUnitDecl()->lookup(
        &AST.context().Idents.get("Target"));
    CHECK(Target.isSingleResult());
    return getTypeNullability(*Target.find_first<TypeAliasDecl>(),
                              TypeNullabilityDefaults(AST.context(), Pragmas));
  }
};

// GetTypeNullabilityLocsTests below cover much of the same functionality as
// GetTypeNullabilityTest could cover, as long as they both use
// NullabilityWalker under the hood, so we only add additional
// GetTypeNullabilityTests for differences in their coverage,
// namely pragma consideration and Unspecified as a default nullability.

TEST_F(GetTypeNullabilityTest, UnannotatedGivesDefault) {
  EXPECT_THAT(nullVec("int *"), ElementsAre(NullabilityKind::Unspecified));
}

TEST_F(GetTypeNullabilityTest, Pragma) {
  EXPECT_THAT(nullVec("int*"), ElementsAre(NullabilityKind::Unspecified));
  Preamble = "#pragma nullability file_default nonnull";
  EXPECT_THAT(nullVec("int*"), ElementsAre(NullabilityKind::NonNull));
  Preamble = "#pragma nullability file_default nullable";
  EXPECT_THAT(nullVec("int*"), ElementsAre(NullabilityKind::Nullable));
  Preamble = "#pragma nullability file_default unspecified";
}

TEST_F(GetTypeNullabilityTest, PragmaTypedef) {
  Inputs.ExtraFiles["p.h"] = R"cpp(
#pragma nullability file_default nullable
    typedef int *P;
  )cpp";
  Header = R"cpp(
#include "p.h"
#pragma nullability file_default nonnull
    using PP = P*;
  )cpp";
  EXPECT_THAT(nullVec("PP*"),
              ElementsAre(NullabilityKind::Unspecified,
                          NullabilityKind::NonNull, NullabilityKind::Nullable));
}

TEST_F(GetTypeNullabilityTest, PragmaMacroUsesExpansionLoc) {
  Header = R"cpp(
#pragma nullability file_default nonnull
#define P int*
#define PTR(X) X*
  )cpp";
  Preamble = "#pragma nullability file_default nullable";
  // Ideally we'd track the spelling location of the `*`, but instead we just
  // use the expansion location.
  EXPECT_THAT(nullVec("P*"), ElementsAre(NullabilityKind::Nullable,
                                         NullabilityKind::Nullable));
  EXPECT_THAT(nullVec("PTR(int*)"), ElementsAre(NullabilityKind::Nullable,
                                                NullabilityKind::Nullable));
}

TEST_F(GetTypeNullabilityTest, PragmaTemplate) {
  Header = R"cpp(
#pragma nullability file_default nonnull

    template <class X>
    using P = X*;

    template <class X>
    struct S {
      using P = X*;
    };
  )cpp";
  // int* is written in the main file, so the main file's "unspecified" applies.
  EXPECT_THAT(nullVec("P<int*>"), ElementsAre(NullabilityKind::NonNull,
                                              NullabilityKind::Unspecified));
  EXPECT_THAT(nullVec("S<int*>::P"), ElementsAre(NullabilityKind::NonNull,
                                                 NullabilityKind::Unspecified));
}

TEST_F(GetTypeNullabilityTest, LostSugarCausesWrongType) {
  Preamble = "#pragma nullability file_default nonnull";
  Header = R"cpp(
#pragma nullability file_default nullable
    using NullablePointer = int*;

    auto identity(auto X) { return X; }
  )cpp";
  Inputs.ExtraArgs.push_back("-std=c++20");
  // identity() destroys sugar, so we incorrectly use main-file's "nonnull".
  EXPECT_THAT(nullVec("decltype(identity(NullablePointer{}))"),
              ElementsAre(NullabilityKind::NonNull));
}

class PrintWithNullabilityTest : public ::testing::Test {
 protected:
  // C++ declarations prepended before parsing type in nullVec().
  std::string Preamble;

  // Parses `Type`, augments it with Nulls, and prints the result.
  std::string print(llvm::StringRef Type, const TypeNullability &Nulls) {
    clang::TestAST AST((Preamble + "\n using Target = " + Type + ";").str());
    auto Target = AST.context().getTranslationUnitDecl()->lookup(
        &AST.context().Idents.get("Target"));
    CHECK(Target.isSingleResult());
    QualType TargetType =
        AST.context().getTypedefType(Target.find_first<TypeAliasDecl>());
    return printWithNullability(TargetType, Nulls, AST.context());
  }
};

TEST_F(PrintWithNullabilityTest, Pointers) {
  EXPECT_EQ(print("int*", {NullabilityKind::Nullable}), "int * _Nullable");
  EXPECT_EQ(
      print("int***", {NullabilityKind::Nullable, NullabilityKind::NonNull,
                       NullabilityKind::Unspecified}),
      "int ** _Nonnull * _Nullable");
}

TEST_F(PrintWithNullabilityTest, Sugar) {
  Preamble = R"cpp(
    template <class T>
    using Ptr = T *;
    using Int = int;
    using IntPtr = Ptr<Int>;
  )cpp";
  EXPECT_EQ(print("IntPtr", {NullabilityKind::Nullable}), "int * _Nullable");
}

TEST_F(PrintWithNullabilityTest, Templates) {
  Preamble = R"cpp(
    template <class>
    struct vector;
    template <class, class>
    struct pair;
  )cpp";
  EXPECT_EQ(print("vector<pair<int*, int*>*>",
                  {NullabilityKind::Nullable, NullabilityKind::NonNull,
                   NullabilityKind::Unspecified}),
            "vector<pair<int * _Nonnull, int *> * _Nullable>");
}

TEST_F(PrintWithNullabilityTest, Functions) {
  EXPECT_EQ(print("float*(*)(double*, double*)",
                  {NullabilityKind::Nullable, NullabilityKind::NonNull,
                   NullabilityKind::NonNull, NullabilityKind::Unspecified}),
            "float * _Nonnull (* _Nullable)(double * _Nonnull, double *)");
}

TEST_F(PrintWithNullabilityTest, Arrays) {
  EXPECT_EQ(print("int*[][2]", {NullabilityKind::Nullable}),
            "int * _Nullable[][2]");
  // variable length array not allowed at file scope, wrap in a function...
  Preamble = R"cpp(
    int n;
    auto &makeArray() {
      float *array[n];
      return array;
    }
  )cpp";
  EXPECT_EQ(print("decltype(makeArray())", {NullabilityKind::Nullable}),
            "float * _Nullable (&)[n]");
}

using MissingLocSlots = std::vector<
    std::pair<unsigned, Matcher<std::optional<clang::NullabilityKind>>>>;

using ComparableNullabilityLoc =
    std::tuple<unsigned, std::optional<clang::NullabilityKind>,
               std::optional<Annotations::Range>>;

std::optional<Annotations::Range> getRange(std::optional<TypeLoc> L,
                                           TestAST &AST) {
  if (!L) return std::nullopt;
  const auto &SM = AST.sourceManager();
  std::optional<CharSourceRange> ActualRange = tooling::getFileRange(
      CharSourceRange::getTokenRange(L->getSourceRange()), AST.context(),
      /*IncludeMacroExpansion=*/true);
  if (!ActualRange) {
    ADD_FAILURE() << "unable to retrieve source range for TypeLoc ";
    return std::nullopt;
  }
  return Annotations::Range{SM.getFileOffset(ActualRange->getBegin()),
                            SM.getFileOffset(ActualRange->getEnd())};
}

// Snippet should be a string of code contain a `using Target = ` typedef,
// with the aliased type annotated with ranges representing each expected
// TypeNullabilityLoc. Each range should have a numeric name equal to the Slot
// number expected for that TypeNullabilityLoc.
//
// Require passing of Snippet into this function instead of using the test
// fixture's snippet to make each expectation read more typically, i.e. in the
// format EXPECT_THAT(functionOf(Input), matchesExpectations()), as opposed to
// having Input also passed to the expectations or not passed to functionOf.
std::vector<ComparableNullabilityLoc> getComparableNullabilityLocs(
    llvm::StringRef Snippet, llvm::StringRef HeaderWithAttributes = "") {
  Annotations AnnotatedInput(Snippet);
  TestInputs Inputs(AnnotatedInput.code());
  Inputs.Language = TestLanguage::Lang_CXX17;
  if (!HeaderWithAttributes.empty()) {
    Inputs.ExtraFiles["header.h"] = HeaderWithAttributes;
    Inputs.ExtraArgs.push_back("-include");
    Inputs.ExtraArgs.push_back("header.h");
  }
  TestAST AST{Inputs};
  auto Target = AST.context().getTranslationUnitDecl()->lookup(
      &AST.context().Idents.get("Target"));
  CHECK(Target.isSingleResult());
  std::vector<TypeNullabilityLoc> NullabilityLocs = getTypeNullabilityLocs(
      Target.find_first<TypeAliasDecl>()->getTypeSourceInfo()->getTypeLoc());

  std::vector<ComparableNullabilityLoc> ComparableOutputs;
  for (const auto &Output : NullabilityLocs) {
    ComparableOutputs.push_back(
        {Output.Slot, Output.NK, getRange(Output.Loc, AST)});
  }
  return ComparableOutputs;
}

class GetTypeNullabilityLocsTest : public ::testing::Test {
 protected:
  // Snippet should be a string of code contain a `using Target = ` typedef,
  // with the aliased type annotated with ranges representing each expected
  // TypeNullabilityLoc. Each range should have a numeric name equal to the Slot
  // number expected for that TypeNullabilityLoc.
  std::string Snippet;

  Matcher<std::vector<ComparableNullabilityLoc>> matchesRanges(
      MissingLocSlots SlotsWithNoLocs = {}) {
    if (Snippet.empty()) {
      ADD_FAILURE() << "Snippet is empty. You probably need to set it before "
                       "creating this matcher.";
      return IsEmpty();
    }
    Annotations AnnotatedInput(Snippet);
    auto Ranges = AnnotatedInput.all_ranges();
    std::vector<std::pair<unsigned, Matcher<ComparableNullabilityLoc>>>
        MatchersBySlotNumber;
    for (const auto &RangesForKey : Ranges) {
      if (RangesForKey.getValue().size() != 1) {
        ADD_FAILURE()
            << "Input should contain ranges named with Slot numbers, e.g. "
               "$0[[int*]], with only one range for each name.";
        return IsEmpty();
      }
      unsigned Slot;
      if (RangesForKey.getKey().consumeInteger(10, Slot)) {
        ADD_FAILURE()
            << "Unable to parse Slot number from annotated range name: "
            << RangesForKey.getKey().str();
        return IsEmpty();
      }
      llvm::StringRef Payload =
          AnnotatedInput.rangeWithPayload(RangesForKey.getKey()).second;
      std::optional<clang::NullabilityKind> ExpectedNullabilityKind =
          std::nullopt;
      if (!Payload.empty()) {
        clang::NullabilityKind KindFromPayload;
        if (Payload == "Nullable") {
          KindFromPayload = NullabilityKind::Nullable;
        } else if (Payload == "NonNull") {
          KindFromPayload = NullabilityKind::NonNull;
        } else if (Payload == "Unspecified") {
          KindFromPayload = NullabilityKind::Unspecified;
        } else {
          ADD_FAILURE()
              << "Payload is not empty but does not match one of the expected "
                 "values indicating a nullability kind.";
        }
        ExpectedNullabilityKind = KindFromPayload;
      }
      Annotations::Range ExpectedRange = RangesForKey.getValue().front();
      MatchersBySlotNumber.push_back(
          {Slot, FieldsAre(Slot, ExpectedNullabilityKind, ExpectedRange)});
    }
    for (auto &[Slot, NKMatcher] : SlotsWithNoLocs) {
      MatchersBySlotNumber.push_back(
          {Slot, FieldsAre(Slot, NKMatcher, std::nullopt)});
    }
    // We get much better error messages with an ordered container matcher,
    // which means we need to first sort the Matchers by Slot number. Locs
    // should be assembled in Slot number order anyway.
    std::stable_sort(
        MatchersBySlotNumber.begin(), MatchersBySlotNumber.end(),
        [](const auto &A, const auto &B) { return A.first < B.first; });
    std::vector<Matcher<ComparableNullabilityLoc>> SortedMatchers;
    std::for_each(MatchersBySlotNumber.begin(), MatchersBySlotNumber.end(),
                  [&SortedMatchers](const auto &A) {
                    SortedMatchers.push_back(A.second);
                  });
    return ElementsAreArray(SortedMatchers);
  }
};

TEST_F(GetTypeNullabilityLocsTest, Pointers) {
  std::string Using = "using Target = ";
  Snippet = Using + R"(int;)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
  Snippet = Using + R"($0[[int *]];)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
  Snippet = Using + R"($0[[$1[[int *]]*]];)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
  Snippet = Using + R"($0[[$1[[$2[[int *]]*]]*]];)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  Snippet = Using + R"($0(Nullable)[[int *]] _Nullable;)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
  Snippet = Using + R"($0(NonNull)[[int *]] _Nonnull;)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
  Snippet = Using + R"($0(Unspecified)[[int *]] _Null_unspecified;)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  Snippet =
      Using + R"($0(Nullable)[[$1(NonNull)[[int *]] _Nonnull *]] _Nullable;)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, Sugar) {
  std::string Header = R"cpp(using X = int* _Nonnull;)cpp";
  Snippet = Header + R"(
    using Target = $0(NonNull)[[X]];
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  Snippet = Header + R"(
    using Target = $0(Nullable)[[$1(NonNull)[[X]] *]] _Nullable;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  Snippet = Header + R"(
    using Target = $0[[$1(NonNull)[[X]](*)]];
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, References) {
  std::string Using = "using Target = ";
  // Top-level references can't be expression types, but we support them anyway
  Snippet = Using + R"($0(NonNull)[[int *]] _Nonnull &;)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
  Snippet = Using + R"($0(NonNull)[[int *]] _Nonnull &&;)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  // ... and other types involving references can appear in expressions
  Snippet =
      Using +
      R"($0(NonNull)[[$1(Nullable)[[int *]] _Nullable & (* _Nonnull)()]];)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
  Snippet =
      Using +
      R"($0(NonNull)[[$1(Nullable)[[int *]] _Nullable && (* _Nonnull)()]];)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, Arrays) {
  Snippet = R"(using Target = $0(NonNull)[[int *]] _Nonnull [][2];)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, AliasTemplates) {
  Snippet = R"(
    template <typename T>
    using Nullable = T _Nullable;
    template <typename T>
    using Nonnull = T _Nonnull;

    using Target =
    Nullable<$0(Nullable)[[Nullable<$1(Nullable)[[Nonnull<$2(NonNull)[[int*]]>*]]>*]]>;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  Snippet = R"(
    template <typename T>
    using Alias = T;

    using Target = Alias<$0(Nullable)[[int *]] _Nullable>;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  Snippet = R"(
    template <typename T, typename U>
    struct Pair;
    template <typename T>
    using Two = Pair<T, T>;

    using Target = Two<$0(Nullable)[[$1(Nullable)[[int *]]]] _Nullable>;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  Snippet = R"(
    template <typename T1>
    using A = T1 * _Nullable;
    template <typename T2>
    using B = A<T2> * _Nonnull;

    using Target = $0[[$1(NonNull)[[B<int>]] *]];
  )";
  EXPECT_THAT(
      getComparableNullabilityLocs(Snippet),
      matchesRanges(MissingLocSlots{{2, Optional(NullabilityKind::Nullable)}}));

  Snippet = R"(
    template <typename T, typename U, typename V>
    struct Triple;
    template <typename A, typename... Rest>
    using TripleAlias = Triple<A _Nonnull, Rest...>;

    using Target = TripleAlias<$0(NonNull)[[int *]], $1(Nullable)[[int *]]
    _Nullable, $2[[int *]]>;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  Snippet = R"(
    template <class... Ts>
    using First = __type_pack_element<0, Ts...>;

    using Target = First<$0(NonNull)[[int *]] _Nonnull>;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, DependentAlias) {
  // Simple dependent type-aliases.
  Snippet = R"(
    template <typename T>
    struct Nullable {
      using type = T _Nullable;
    };

    using Target = Nullable<$0(Nullable)[[$1(NonNull)[[int *]] _Nonnull *]]>::type;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, NestedClassTemplate) {
  // Simple struct inside template.
  Snippet = R"(
    template <class T>
    struct Outer {
      struct Inner;
    };

    using Target = Outer<$0(NonNull)[[int *]] _Nonnull>::Inner;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, NestedClassInstantiation) {
  std::string Header = R"cpp(
    template <class T, class U>
    struct Pair;
    template <class T, class U>
    struct PairWrapper {
      using type = Pair<T _Nullable, U>;
    };
  )cpp";
  Snippet = Header + R"(
    using Target = PairWrapper<$0(Nullable)[[int *]],
                               $1(NonNull)[[int *]] _Nonnull>::type;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  Snippet = Header + R"(
    using Target = PairWrapper<$0(Nullable)[[int *]] _Nonnull,
                               $1[[int *]]>::type;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  Snippet = Header + R"(
    using Target = PairWrapper<$0(Nullable)[[PairWrapper<$1(Nullable)[[int *]],
                                                         $2(NonNull)[[int *]] _Nonnull
                                                        >::type *]], 
                               $3[[PairWrapper<$4(Nullable)[[int *]] _Nonnull,
                                               $5[[int *]]
                                              >::type *]]
                              >::type;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, ReferenceOuterTemplateParam) {
  // Referencing type-params from indirectly-enclosing template.
  Snippet = R"(
    template <class A, class B>
    struct Pair;

    template <class T>
    struct Outer {
      template <class U>
      struct Inner {
        using type = Pair<U, T>;
      };
    };

    using Target = Outer<$1(Nullable)[[int *]] _Nullable>::Inner<
        $0(NonNull)[[int *]] _Nonnull>::type;
   )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  // Same where Inner is an alias template.
  Snippet = R"(
    template <class A, class B>
    struct Pair;

    template <class T>
    struct Outer {
      template <class U>
      using Inner = Pair<U, T>;
    };

    using Target = Outer<$1(Nullable)[[int *]] _Nullable>::Inner<
        $0(NonNull)[[int *]] _Nonnull>;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, MixedQualiferChain) {
  std::string Header = R"cpp(
    template <class A, class B>
    class Pair;

    struct Outer1 {
      template <class T>
      struct Middle {
        template <class U>
        struct Inner {
          using type = Pair<T, U>;
        };
      };
    };

    template <class T>
    struct Outer2 {
      struct Middle {
        template <class U>
        struct Inner {
          using type = Pair<T, U>;
        };
      };
    };

    template <class T>
    struct Outer3 {
      template <class U>
      struct Middle {
        struct Inner {
          using type = Pair<T, U>;
        };
      };
    };
  )cpp";

  Snippet = Header + R"(
    using Target = Outer1::Middle<$0(Nullable)[[int *]] _Nullable>::Inner<
        $1(NonNull)[[int *]] _Nonnull>::type;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  Snippet = Header + R"(
    using Target = Outer2<$0(Nullable)[[int *]] _Nullable>::Middle::Inner<
        $1(NonNull)[[int *]] _Nonnull>::type;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  Snippet = Header + R"(
    using Target = Outer3<$0(Nullable)[[int *]] _Nullable>::Middle<
        $1(NonNull)[[int *]] _Nonnull>::Inner::type;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, DependentlyNamedTemplate) {
  // Instantiation of dependent-named template
  Snippet = R"(
    struct Wrapper {
      template <class T>
      using Nullable = T _Nullable;
    };

    template <class U, class WrapT>
    struct S {
      using type = typename WrapT::template Nullable<U> *_Nonnull;
    };
    
    using Target = $0(NonNull)[[S<$1(Nullable)[[int *]], Wrapper>::type]];
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, PartialSpecialization) {
  Snippet = R"(
    template <class>
    struct S;
    template <class T>
    struct S<T *> {
      using Alias = T;
    };

    using Target = S<int *>::Alias;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, TemplateTemplateParams) {
  // Template template params
  std::string Header = R"cpp(
    template <class X>
    struct Nullable {
      using type = X _Nullable;
    };
    template <class X>
    struct Nonnull {
      using type = X _Nonnull;
    };

    template <template <class> class Nullability, class T>
    struct Pointer {
      using type = typename Nullability<T *>::type;
    };
  )cpp";
  Snippet = Header +
            R"(using Target = $0(Nullable)[[Pointer<Nullable, int>::type]];)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  Snippet = Header +
            R"(using Target = $0(Nullable)[[Pointer<Nullable,
                  $1(NonNull)[[Pointer<Nonnull, int>::type]]>::type]];)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  // Same thing, but with alias templates.
  Header = R"cpp(
    template <class X>
    using Nullable = X _Nullable;
    template <class X>
    using Nonnull = X _Nonnull;

    template <template <class> class Nullability, class T>
    struct Pointer {
      using type = Nullability<T *>;
    };
  )cpp";
  Snippet = Header +
            R"(using Target = $0(Nullable)[[Pointer<Nullable, int>::type]];)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  Snippet = Header +
            R"(using Target = $0(Nullable)[[Pointer<Nullable,
                  $1(NonNull)[[Pointer<Nonnull, int>::type]]>::type]];)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, ClassTemplateParamPack) {
  // Parameter packs
  std::string Header = R"cpp(
    template <typename... X>
    struct TupleWrapper {
      class Tuple;
    };
  )cpp";
  Snippet = Header + R"(
    using Target = TupleWrapper<$0[[int *]],
                                $1(NonNull)[[int *]] _Nonnull>::Tuple;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  Snippet = Header + R"(
    template <typename... X>
    struct NullableTuple {
      using type = TupleWrapper<X _Nullable...>::Tuple;
    };

    using Target = NullableTuple<$0(Nullable)[[int *]],
                                 $1(Nullable)[[int *]] _Nonnull>::type;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, AliasTemplateWithDefaultArg) {
  // TODO(b/281474380): The NullabilityKind should be Nullable and the range
  // should only enclose the `int *`, but we don't yet handle default argument
  // sugar correctly.
  Snippet = R"(
    template <typename T1, typename T2 = T1>
    using AliasTemplate = T2;

  using Target =$0[[AliasTemplate<int * _Nullable>]];
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, ClassTemplateWithDefaultArg) {
  // TODO(b/281474380): This should be two nullable slots with the same range,
  // but we don't yet handle default argument sugar correctly.
  Snippet = R"(
    template <typename T1, typename T2 = T1>
    class ClassTemplate {};

    using Target = ClassTemplate<$0(Nullable)[[int *]] _Nullable>;
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet),
              matchesRanges(MissingLocSlots{
                  {1, Matcher<std::optional<NullabilityKind>>(std::nullopt)}}));
}

TEST_F(GetTypeNullabilityLocsTest, TemplateArgsBehindAlias) {
  // TODO: NullabilityKind should be Nullable, but we don't assemble template
  // contexts behind an alias.
  Snippet = R"(
    template <class X>
    struct Outer {
      using Inner = X;
    };
    using OuterNullable = Outer<int *_Nullable>;

    using Target = $0[[OuterNullable::Inner]];
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, AnnotateNullable) {
  std::string HeaderWithAttributes = R"cpp(
    namespace custom {
    template <class T>
    using Nullable [[clang::annotate("Nullable")]] = T;
    template <class T>
    using NonNull [[clang::annotate("Nonnull")]] = T;
    }  // namespace custom

    template <class T, class U>
    class pair;

    template <class X>
    using twice = pair<X, X>;
  )cpp";
  Snippet = R"(using Target = custom::Nullable<$0(Nullable)[[int *]]>;)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet, HeaderWithAttributes),
              matchesRanges());
  Snippet = R"(using Target = custom::NonNull<$0(NonNull)[[int *]]>;)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet, HeaderWithAttributes),
              matchesRanges());
  Snippet =
      R"(using Target = pair<custom::NonNull<$0(NonNull)[[int *]]>, custom::Nullable<$1(Nullable)[[int *]]>>;)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet, HeaderWithAttributes),
              matchesRanges());
  Snippet =
      R"(using Target = twice<custom::NonNull<$0(NonNull)[[$1(NonNull)[[int *]]]]>>;)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet, HeaderWithAttributes),
              matchesRanges());

  // Should still work if aliases *do* apply _Nullable.
  HeaderWithAttributes = R"cpp(
    namespace custom {
    template <class T>
    using Nullable [[clang::annotate("Nullable")]] = T _Nullable;
    template <class T>
    using NonNull [[clang::annotate("Nonnull")]] = T _Nonnull;
    }  // namespace custom
  )cpp";
  Snippet = R"(using Target = custom::Nullable<$0(Nullable)[[int *]]>;)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet, HeaderWithAttributes),
              matchesRanges());
  Snippet = R"(using Target = custom::NonNull<$0(NonNull)[[int *]]>;)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet, HeaderWithAttributes),
              matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, SmartPointers) {
  std::string Header = R"cpp(
    namespace std {
    template <typename T>
    class unique_ptr {};
    }  // namespace std
  )cpp";
  Snippet = Header + R"(using Target = int;)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
  Snippet = Header + R"(using Target = $0[[std::unique_ptr<int>]];)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
  Snippet =
      Header +
      R"(using Target = $0[[std::unique_ptr<$1[[std::unique_ptr<int>]]>]];)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());

  std::string MoreHeaderWithAttributes = R"cpp(
    template <typename T>
    using Nullable [[clang::annotate("Nullable")]] = T;
    template <typename T>
    using NonNull [[clang::annotate("Nonnull")]] = T;
  )cpp";
  Snippet =
      Header +
      R"(using Target = NonNull<$0(NonNull)[[std::unique_ptr<Nullable<$1(Nullable)[[std::unique_ptr<int>]]>>]]>;)";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet, MoreHeaderWithAttributes),
              matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest, ArrayBehindElaboratedAlias) {
  Snippet = R"(
    namespace ns { using T = int * _Nullable [5]; }
    using Target = ns::T;
  )";
  EXPECT_THAT(
      getComparableNullabilityLocs(Snippet),
      matchesRanges(MissingLocSlots{{0, Optional(NullabilityKind::Nullable)}}));
}

TEST_F(GetTypeNullabilityLocsTest,
       FunctionPrototypeBehindElaboratedAliasWithPointerReturn) {
  Snippet = R"(
    namespace ns { using T = int * _Nullable (* _Nonnull)(); }
    using Target = $0(NonNull)[[ns::T]];
  )";
  EXPECT_THAT(
      getComparableNullabilityLocs(Snippet),
      matchesRanges(MissingLocSlots{{1, Optional(NullabilityKind::Nullable)}}));
}

TEST_F(GetTypeNullabilityLocsTest,
       FunctionPrototypeBehindElaboratedAliasWithPointerParam) {
  Snippet = R"(
    namespace ns { using T = void (* _Nonnull)(int * _Nullable); }
    using Target = $0(NonNull)[[ns::T]];
  )";
  EXPECT_THAT(
      getComparableNullabilityLocs(Snippet),
      matchesRanges(MissingLocSlots{{1, Optional(NullabilityKind::Nullable)}}));
}

TEST_F(GetTypeNullabilityLocsTest,
       FunctionPrototypePointerParamAfterElaboratedNonPointerParam) {
  Snippet = R"(
    namespace ns {
    using MyInt = int;
    } //namespace ns

    using Target = $0(NonNull)[[void (* _Nonnull)(ns::MyInt,
        $1(Nullable)[[int *]] _Nullable)]];
  )";
  EXPECT_THAT(getComparableNullabilityLocs(Snippet), matchesRanges());
}

TEST_F(GetTypeNullabilityLocsTest,
       DeclContextTemplateArgsWithElaboratedAliases) {
  // We record the correct Locs for decl context template args when the decl
  // context is behind an elaborated alias.
  std::string Header = R"cpp(
    namespace ns {
    template <typename T>
    struct Outer {
      template <typename U>
      struct InnerTemplated {};

      struct Inner {};
    };

    using MyInnerTemplated = Outer<int* _Nullable>::InnerTemplated<int>;
    using MyInner = Outer<int* _Nullable>::Inner;
    }  // namespace ns
  )cpp";
  Snippet = Header + R"(using Target = ns::MyInnerTemplated;)";
  EXPECT_THAT(
      getComparableNullabilityLocs(Snippet),
      matchesRanges(MissingLocSlots{{0, Optional(NullabilityKind::Nullable)}}));
  Snippet = Header + R"(using Target = ns::MyInner;)";
  EXPECT_THAT(
      getComparableNullabilityLocs(Snippet),
      matchesRanges(MissingLocSlots{{0, Optional(NullabilityKind::Nullable)}}));

  // We record the correct Locs for decl context template args when we have
  // pointer args after elaborated alias args.
  Header = R"cpp(
    namespace ns {
    using MyInt = int;
    }  // namespace ns

    template <typename T, typename U, typename V, typename W = int* _Nonnull>
    struct Outer {
      template <typename X>
      struct InnerTemplated {};

      struct Inner {};
    };
  )cpp";
  Snippet = Header + R"(
    using Target = Outer<ns::MyInt, $0(Nullable)[[int *]] _Nullable,
        ns::MyInt>::InnerTemplated<int>;
  )";
  EXPECT_THAT(
      getComparableNullabilityLocs(Snippet),
      matchesRanges(
          // TODO(b/281474380) Should be NonNull, but we don't yet resugar
          // default arguments.
          MissingLocSlots{{1, std::optional<NullabilityKind>(std::nullopt)}}));
  Snippet = Header + R"(
    using Target = Outer<ns::MyInt, $0(Nullable)[[int *]] _Nullable,
        ns::MyInt>::Inner;
  )";
  EXPECT_THAT(
      getComparableNullabilityLocs(Snippet),
      matchesRanges(
          // TODO(b/281474380) Should be NonNull, but we don't yet resugar
          // default arguments.
          MissingLocSlots{{1, std::optional<NullabilityKind>(std::nullopt)}}));
}

}  // namespace
}  // namespace clang::tidy::nullability

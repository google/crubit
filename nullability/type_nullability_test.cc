// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/type_nullability.h"

#include "absl/log/check.h"
#include "clang/Basic/Specifiers.h"
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
  TypeNullability nullVec(llvm::StringRef Type) {
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

TEST_F(GetNullabilityAnnotationsFromTypeTest, References) {
  // Top-level references can't be expression types, but we support them anyway
  EXPECT_THAT(nullVec("int * _Nonnull &"),
              ElementsAre(NullabilityKind::NonNull));
  EXPECT_THAT(nullVec("int * _Nonnull &&"),
              ElementsAre(NullabilityKind::NonNull));

  // ... and other types involving references can appear in expressions
  EXPECT_THAT(nullVec("int * _Nullable& (* _Nonnull)()"),
              ElementsAre(NullabilityKind::NonNull, NullabilityKind::Nullable));
  EXPECT_THAT(nullVec("int * _Nullable&& (* _Nonnull)()"),
              ElementsAre(NullabilityKind::NonNull, NullabilityKind::Nullable));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, Arrays) {
  EXPECT_THAT(nullVec("int * _Nonnull[][2]"),
              ElementsAre(NullabilityKind::NonNull));
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

  Preamble = R"cpp(
    template <typename T, typename U, typename V>
    struct Triple;
    template <typename A, typename... Rest>
    using TripleAlias = Triple<A _Nonnull, Rest...>;
  )cpp";
  EXPECT_THAT(nullVec("TripleAlias<int *, int *_Nullable, int*>"),
              ElementsAre(NullabilityKind::NonNull, NullabilityKind::Nullable,
                          NullabilityKind::Unspecified));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, DependentAlias) {
  // Simple dependent type-aliases.
  Preamble = R"cpp(
    template <class T>
    struct Nullable {
      using type = T _Nullable;
    };
  )cpp";
  EXPECT_THAT(nullVec("Nullable<int* _Nonnull *>::type"),
              ElementsAre(NullabilityKind::Nullable, NullabilityKind::NonNull));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, NestedClassTemplate) {
  // Simple struct inside template.
  Preamble = R"cpp(
    template <class T>
    struct Outer {
      struct Inner;
    };
    using OuterNullableInner = Outer<int* _Nonnull>::Inner;
  )cpp";
  // TODO: should be [NonNull]
  EXPECT_THAT(nullVec("Outer<int* _Nonnull>::Inner"),
              ElementsAre(NullabilityKind::Unspecified));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, NestedClassInstantiation) {
  Preamble = R"cpp(
    template <class T, class U>
    struct Pair;
    template <class T, class U>
    struct PairWrapper {
      using type = Pair<T _Nullable, U>;
    };
  )cpp";

  EXPECT_THAT(nullVec("PairWrapper<int*, int* _Nonnull>::type"),
              ElementsAre(NullabilityKind::Nullable, NullabilityKind::NonNull));
  EXPECT_THAT(
      nullVec("PairWrapper<int* _Nonnull, int*>::type"),
      ElementsAre(NullabilityKind::Nullable, NullabilityKind::Unspecified));

  EXPECT_THAT(
      nullVec("PairWrapper<PairWrapper<int*, int* _Nonnull>::type*, "
              "            PairWrapper<int* _Nonnull, int*>::type*>::type"),
      ElementsAre(NullabilityKind::Nullable, NullabilityKind::Nullable,
                  NullabilityKind::NonNull,

                  NullabilityKind::Unspecified, NullabilityKind::Nullable,
                  NullabilityKind::Unspecified));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, ReferenceOuterTemplateParam) {
  // Referencing type-params from indirectly-enclosing template.
  Preamble = R"cpp(
    template <class A, class B>
    struct Pair;

    template <class T>
    struct Outer {
      template <class U>
      struct Inner {
        using type = Pair<U, T>;
      };
    };
  )cpp";
  EXPECT_THAT(nullVec("Outer<int *_Nullable>::Inner<int *_Nonnull>::type"),
              ElementsAre(NullabilityKind::NonNull, NullabilityKind::Nullable));
  // Same where Inner is an alias template.
  Preamble = R"cpp(
    template <class A, class B>
    struct Pair;

    template <class T>
    struct Outer {
      template <class U>
      using Inner = Pair<U, T>;
    };
  )cpp";
  EXPECT_THAT(nullVec("Outer<int *_Nullable>::Inner<int *_Nonnull>"),
              ElementsAre(NullabilityKind::NonNull, NullabilityKind::Nullable));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, MixedQualiferChain) {
  Preamble = R"cpp(
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

  EXPECT_THAT(
      nullVec("Outer1::Middle<int * _Nullable>::Inner<int * _Nonnull>::type"),
      ElementsAre(NullabilityKind::Nullable, NullabilityKind::NonNull));
  EXPECT_THAT(
      nullVec("Outer2<int * _Nullable>::Middle::Inner<int * _Nonnull>::type"),
      ElementsAre(NullabilityKind::Nullable, NullabilityKind::NonNull));
  EXPECT_THAT(
      nullVec("Outer3<int * _Nullable>::Middle<int * _Nonnull>::Inner::type"),
      ElementsAre(NullabilityKind::Nullable, NullabilityKind::NonNull));
};

TEST_F(GetNullabilityAnnotationsFromTypeTest, DependentlyNamedTemplate) {
  // Instantiation of dependent-named template
  Preamble = R"cpp(
    struct Wrapper {
      template <class T>
      using Nullable = T _Nullable;
    };

    template <class U, class WrapT>
    struct S {
      using type = typename WrapT::template Nullable<U>* _Nonnull;
    };
  )cpp";
  EXPECT_THAT(nullVec("S<int *, Wrapper>::type"),
              ElementsAre(NullabilityKind::NonNull, NullabilityKind::Nullable));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, TemplateTemplateParams) {
  // Template template params
  Preamble = R"cpp(
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
      using type = typename Nullability<T*>::type;
    };
  )cpp";
  EXPECT_THAT(nullVec("Pointer<Nullable, int>::type"),
              ElementsAre(NullabilityKind::Nullable));
  EXPECT_THAT(nullVec("Pointer<Nullable, Pointer<Nonnull, int>::type>::type"),
              ElementsAre(NullabilityKind::Nullable, NullabilityKind::NonNull));
  // Same thing, but with alias templates.
  Preamble = R"cpp(
    template <class X>
    using Nullable = X _Nullable;
    template <class X>
    using Nonnull = X _Nonnull;

    template <template <class> class Nullability, class T>
    struct Pointer {
      using type = Nullability<T*>;
    };
  )cpp";
  EXPECT_THAT(nullVec("Pointer<Nullable, int>::type"),
              ElementsAre(NullabilityKind::Nullable));
  EXPECT_THAT(nullVec("Pointer<Nullable, Pointer<Nonnull, int>::type>::type"),
              ElementsAre(NullabilityKind::Nullable, NullabilityKind::NonNull));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, ClassTemplateParamPack) {
  // Parameter packs
  Preamble = R"cpp(
    template <class... X>
    struct TupleWrapper {
      class Tuple;
    };

    template <class... X>
    struct NullableTuple {
      using type = TupleWrapper<X _Nullable...>::Tuple;
    };
  )cpp";
  // TODO: should be [Unspecified, Nonnull]
  EXPECT_THAT(
      nullVec("TupleWrapper<int*, int* _Nonnull>::Tuple"),
      ElementsAre(NullabilityKind::Unspecified, NullabilityKind::Unspecified));
  // TODO: should be [Nullable, Nullable]
  EXPECT_THAT(
      nullVec("NullableTuple<int*, int* _Nonnull>::type"),
      ElementsAre(NullabilityKind::Unspecified, NullabilityKind::Unspecified));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, AliasTemplateWithDefaultArg) {
  Preamble =
      "template <typename T1, typename T2 = T1> using AliasTemplate = T2;";

  // TODO(b/281474380): This should be [Nullable], but we don't yet handle
  // default arguments correctly.
  EXPECT_THAT(nullVec("AliasTemplate<int * _Nullable>"),
              ElementsAre(NullabilityKind::Unspecified));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, TemplateArgsBehindAlias) {
  Preamble = R"cpp(
    template <class X>
    struct Outer {
      using Inner = X;
    };
    using OuterNullable = Outer<int* _Nullable>;
  )cpp";
  // TODO: should be [Nullable]
  EXPECT_THAT(nullVec("OuterNullable::Inner"),
              ElementsAre(NullabilityKind::Unspecified));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, AnnotateNullable) {
  Preamble = R"cpp(
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
  EXPECT_THAT(nullVec("custom::Nullable<int*>"),
              ElementsAre(NullabilityKind::Nullable));
  EXPECT_THAT(nullVec("custom::NonNull<int*>"),
              ElementsAre(NullabilityKind::NonNull));
  EXPECT_THAT(nullVec("pair<custom::NonNull<int*>, custom::Nullable<int*>>"),
              ElementsAre(NullabilityKind::NonNull, NullabilityKind::Nullable));
  EXPECT_THAT(nullVec("twice<custom::NonNull<int*>>"),
              ElementsAre(NullabilityKind::NonNull, NullabilityKind::NonNull));

  // Should still work if aliases *do* apply _Nullable.
  Preamble = R"cpp(
    namespace custom {
    template <class T>
    using Nullable [[clang::annotate("Nullable")]] = T _Nullable;
    template <class T>
    using NonNull [[clang::annotate("Nonnull")]] = T _Nonnull;
    }  // namespace custom
  )cpp";
  EXPECT_THAT(nullVec("custom::Nullable<int*>"),
              ElementsAre(NullabilityKind::Nullable));
  EXPECT_THAT(nullVec("custom::NonNull<int*>"),
              ElementsAre(NullabilityKind::NonNull));
}

class PrintWithNullabilityTest : public ::testing::Test {
 protected:
  // C++ declarations prepended before parsing type in nullVec().
  std::string Preamble;

  // Parses `Type`, augments it with Nulls, and prints the result.
  std::string print(llvm::StringRef Type, const TypeNullability& Nulls) {
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
    using Ptr = T*;
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
    auto& makeArray() {
      float* array[n];
      return array;
    }
  )cpp";
  EXPECT_EQ(print("decltype(makeArray())", {NullabilityKind::Nullable}),
            "float * _Nullable (&)[n]");
}

}  // namespace
}  // namespace clang::tidy::nullability

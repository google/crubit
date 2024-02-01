// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/type_nullability.h"

#include <memory>
#include <string>

#include "absl/log/check.h"
#include "nullability/pragma.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/AST/Decl.h"
#include "clang/AST/Type.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Frontend/FrontendActions.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/StringRef.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using testing::ElementsAre;

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
    struct UserDefinedSmartPointer {
      using absl_nullability_compatible = void;
      using pointer = char *;
    };

    using UniquePointer = std::unique_ptr<int>;
    using SharedPointer = std::shared_ptr<int>;
    using PublicDerivedPointer = PublicDerived<int>;
    using UserDefined = UserDefinedSmartPointer<int>;
    // Force the compiler to instantiate the templates. Otherwise, the
    // `ClassTemplateSpecializationDecl` won't contain a `TypedefNameDecl` for
    // `pointer` or `element_type`, and `underlyingRawPointerType()` will
    // use the fallback behavior of looking at the template argument.
    template class std::unique_ptr<int>;
    template class std::shared_ptr<int>;
    template class PublicDerived<int>;
    template class UserDefinedSmartPointer<int>;
  )cpp");

  QualType PointerToCharTy = AST.context().getPointerType(AST.context().CharTy);
  EXPECT_EQ(underlyingRawPointerType(underlying("UniquePointer", AST)),
            PointerToCharTy);
  EXPECT_EQ(underlyingRawPointerType(underlying("SharedPointer", AST)),
            PointerToCharTy);
  EXPECT_EQ(underlyingRawPointerType(underlying("PublicDerivedPointer", AST)),
            PointerToCharTy);
  EXPECT_EQ(underlyingRawPointerType(underlying("UserDefined", AST)),
            PointerToCharTy);
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
    using Nullable [[clang::annotate("Nullable")]] = T;
    using NullableUniquePointer = Nullable<std::unique_ptr<int>>;
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

  EXPECT_EQ(underlyingRawPointerType(underlying("NullableUniquePointer", AST)),
            PointerToIntTy);
}

class GetNullabilityAnnotationsFromTypeTest : public ::testing::Test {
 protected:
  // C++ declarations prepended before parsing type in nullVec().
  TestInputs Inputs;
  std::string &Header;
  std::string Preamble;

  GetNullabilityAnnotationsFromTypeTest()
      : Header(Inputs.ExtraFiles["header.h"]) {
    Inputs.ExtraArgs.push_back("-include");
    Inputs.ExtraArgs.push_back("header.h");
  }

  // Parses `Type` and returns getNullabilityAnnotationsFromType().
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

TEST_F(GetNullabilityAnnotationsFromTypeTest, Pointers) {
  EXPECT_THAT(nullVec("int"), ElementsAre());
  EXPECT_THAT(nullVec("int *"), ElementsAre(NullabilityKind::Unspecified));
  EXPECT_THAT(nullVec("int **"), ElementsAre(NullabilityKind::Unspecified,
                                             NullabilityKind::Unspecified));
  EXPECT_THAT(nullVec("int *_Nullable*_Nonnull"),
              ElementsAre(NullabilityKind::NonNull, NullabilityKind::Nullable));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, Sugar) {
  Header = "using X = int* _Nonnull;";

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
  Header = R"cpp(
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

  Header = R"cpp(
    template <typename T, typename U>
    struct Pair;
    template <typename T>
    using Two = Pair<T, T>;
  )cpp";
  EXPECT_THAT(
      nullVec("Two<int* _Nullable>"),
      ElementsAre(NullabilityKind::Nullable, NullabilityKind::Nullable));

  Header = R"cpp(
    template <typename T1>
    using A = T1 *_Nullable;
    template <typename T2>
    using B = A<T2> *_Nonnull;
  )cpp";
  EXPECT_THAT(nullVec("B<int>"),
              ElementsAre(NullabilityKind::NonNull, NullabilityKind::Nullable));

  Header = R"cpp(
    template <typename T, typename U, typename V>
    struct Triple;
    template <typename A, typename... Rest>
    using TripleAlias = Triple<A _Nonnull, Rest...>;
  )cpp";
  EXPECT_THAT(nullVec("TripleAlias<int *, int *_Nullable, int*>"),
              ElementsAre(NullabilityKind::NonNull, NullabilityKind::Nullable,
                          NullabilityKind::Unspecified));

  Header = R"cpp(
    template <class... Ts>
    using First = __type_pack_element<0, Ts...>;
  )cpp";
  EXPECT_THAT(nullVec("First<int * _Nonnull>"),
              ElementsAre(NullabilityKind::NonNull));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, DependentAlias) {
  // Simple dependent type-aliases.
  Header = R"cpp(
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
  Header = R"cpp(
    template <class T>
    struct Outer {
      struct Inner;
    };
    using OuterNullableInner = Outer<int *_Nonnull>::Inner;
  )cpp";
  // TODO: should be [NonNull]
  EXPECT_THAT(nullVec("Outer<int* _Nonnull>::Inner"),
              ElementsAre(NullabilityKind::Unspecified));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, NestedClassInstantiation) {
  Header = R"cpp(
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
  Header = R"cpp(
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
  Header = R"cpp(
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
  Header = R"cpp(
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
  Header = R"cpp(
    struct Wrapper {
      template <class T>
      using Nullable = T _Nullable;
    };

    template <class U, class WrapT>
    struct S {
      using type = typename WrapT::template Nullable<U> *_Nonnull;
    };
  )cpp";
  EXPECT_THAT(nullVec("S<int *, Wrapper>::type"),
              ElementsAre(NullabilityKind::NonNull, NullabilityKind::Nullable));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, PartialSpecialization) {
  Header = R"cpp(
    template <class>
    struct S;
    template <class T>
    struct S<T *> {
      using Alias = T;
    };
  )cpp";
  EXPECT_THAT(nullVec("S<int*>::Alias"), testing::IsEmpty());
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, TemplateTemplateParams) {
  // Template template params
  Header = R"cpp(
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
  EXPECT_THAT(nullVec("Pointer<Nullable, int>::type"),
              ElementsAre(NullabilityKind::Nullable));
  EXPECT_THAT(nullVec("Pointer<Nullable, Pointer<Nonnull, int>::type>::type"),
              ElementsAre(NullabilityKind::Nullable, NullabilityKind::NonNull));
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
  EXPECT_THAT(nullVec("Pointer<Nullable, int>::type"),
              ElementsAre(NullabilityKind::Nullable));
  EXPECT_THAT(nullVec("Pointer<Nullable, Pointer<Nonnull, int>::type>::type"),
              ElementsAre(NullabilityKind::Nullable, NullabilityKind::NonNull));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, ClassTemplateParamPack) {
  // Parameter packs
  Header = R"cpp(
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
  Header = "template <typename T1, typename T2 = T1> using AliasTemplate = T2;";

  // TODO(b/281474380): This should be [Nullable], but we don't yet handle
  // default arguments correctly.
  EXPECT_THAT(nullVec("AliasTemplate<int * _Nullable>"),
              ElementsAre(NullabilityKind::Unspecified));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, ClassTemplateWithDefaultArg) {
  Header = "template <typename T1, typename T2 = T1> class ClassTemplate {};";

  // TODO(b/281474380): This should be [Nullable, Nullable], but we don't yet
  // handle default arguments correctly.
  EXPECT_THAT(
      nullVec("ClassTemplate<int * _Nullable>"),
      ElementsAre(NullabilityKind::Nullable, NullabilityKind::Unspecified));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, TemplateArgsBehindAlias) {
  Header = R"cpp(
    template <class X>
    struct Outer {
      using Inner = X;
    };
    using OuterNullable = Outer<int *_Nullable>;
  )cpp";
  // TODO: should be [Nullable]
  EXPECT_THAT(nullVec("OuterNullable::Inner"),
              ElementsAre(NullabilityKind::Unspecified));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, AnnotateNullable) {
  Header = R"cpp(
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
  Header = R"cpp(
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

TEST_F(GetNullabilityAnnotationsFromTypeTest, SmartPointers) {
  Header = R"cpp(
    namespace std {
    template <class T>
    class unique_ptr {};
    }  // namespace std
    template <class T>
    using Nullable [[clang::annotate("Nullable")]] = T;
    template <class T>
    using NonNull [[clang::annotate("Nonnull")]] = T;
  )cpp";
  EXPECT_THAT(nullVec("int"), ElementsAre());
  EXPECT_THAT(nullVec("std::unique_ptr<int>"),
              ElementsAre(NullabilityKind::Unspecified));
  EXPECT_THAT(
      nullVec("std::unique_ptr<std::unique_ptr<int>>"),
      ElementsAre(NullabilityKind::Unspecified, NullabilityKind::Unspecified));
  EXPECT_THAT(
      nullVec("NonNull<std::unique_ptr<Nullable<std::unique_ptr<int>>>>"),
      ElementsAre(NullabilityKind::NonNull, NullabilityKind::Nullable));
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, Pragma) {
  EXPECT_THAT(nullVec("int*"), ElementsAre(NullabilityKind::Unspecified));
  Preamble = "#pragma nullability file_default nonnull";
  EXPECT_THAT(nullVec("int*"), ElementsAre(NullabilityKind::NonNull));
  Preamble = "#pragma nullability file_default nullable";
  EXPECT_THAT(nullVec("int*"), ElementsAre(NullabilityKind::Nullable));
  Preamble = "#pragma nullability file_default unspecified";
}

TEST_F(GetNullabilityAnnotationsFromTypeTest, PragmaTypedef) {
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

TEST_F(GetNullabilityAnnotationsFromTypeTest, PragmaMacroUsesExpansionLoc) {
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

TEST_F(GetNullabilityAnnotationsFromTypeTest, PragmaTemplate) {
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

TEST_F(GetNullabilityAnnotationsFromTypeTest, LostSugarCausesWrongType) {
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

}  // namespace
}  // namespace clang::tidy::nullability

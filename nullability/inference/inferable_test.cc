// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/inferable.h"

#include <string>
#include <vector>

#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Basic/LLVM.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/StringRef.h"
#include "external/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
using ::clang::ast_matchers::anything;
using ::clang::ast_matchers::equalsNode;
using ::clang::ast_matchers::hasDeclContext;
using ::clang::ast_matchers::hasName;
using ::clang::ast_matchers::isImplicit;
using ::clang::ast_matchers::match;
using ::clang::ast_matchers::namedDecl;
using ::clang::ast_matchers::unless;
using ::testing::UnorderedElementsAreArray;

template <class T = NamedDecl>
static const T& lookup(llvm::StringRef Name, ASTContext& Ctx,
                       const Decl* DeclContext = nullptr) {
  const auto& ContextMatcher =
      DeclContext ? hasDeclContext(equalsNode(DeclContext)) : anything();
  const auto& BoundNodes =
      match(namedDecl(hasName(Name), ContextMatcher, unless(isImplicit()))
                .bind("decl"),
            Ctx);
  const T* Match = nullptr;
  for (const auto& N : BoundNodes) {
    if (const auto* NAsT = N.getNodeAs<T>("decl")) {
      if (Match)
        ADD_FAILURE() << "Found more than one matching node for " << Name;
      Match = NAsT;
    }
  }
  EXPECT_NE(Match, nullptr) << Name;
  return *Match;
}

namespace {
constexpr llvm::StringRef SmartPointerHeader = R"cc(
  namespace std {
  template <typename T>
  struct unique_ptr {
    using pointer = T*;
  };
  }  // namespace std

  template <typename T>
  struct _Nullable custom_smart_ptr {
    using pointer = T*;
  };

  class _Nullable custom_smart_int_ptr { using pointer = int*; };
)cc";

TEST(IsInferenceTargetTest, GlobalRawPointer) {
  TestAST AST(R"cc(
    int* Pointer;
  )cc");
  EXPECT_TRUE(isInferenceTarget(lookup("Pointer", AST.context())));
}

TEST(IsInferenceTargetTest, GlobalStdSmartPointer) {
  TestAST AST((SmartPointerHeader + R"cc(
                std::unique_ptr<int> StdSmartPointer;
              )cc")
                  .str());
  EXPECT_TRUE(isInferenceTarget(lookup("StdSmartPointer", AST.context())));
}

TEST(IsInferenceTargetTest, GlobalCustomSmartPointer) {
  TestAST AST((SmartPointerHeader + R"cc(
                custom_smart_ptr<int> CustomSmartPointer;
              )cc")
                  .str());
  EXPECT_TRUE(isInferenceTarget(lookup("CustomSmartPointer", AST.context())));
}

TEST(IsInferenceTargetTest, GlobalCustomSmartIntPointer) {
  TestAST AST((SmartPointerHeader + R"cc(
                custom_smart_int_ptr CustomSmartIntPointer;
              )cc")
                  .str());
  EXPECT_TRUE(
      isInferenceTarget(lookup("CustomSmartIntPointer", AST.context())));
}

TEST(IsInferenceTargetTest, GlobalNonPointer) {
  TestAST AST(R"cc(
    int NotPointer;
  )cc");
  EXPECT_FALSE(isInferenceTarget(lookup("NotPointer", AST.context())));
}

TEST(IsInferenceTargetTest, FunctionWithoutPointersIsNotTarget) {
  TestAST AST(R"cc(
    void func() {}
  )cc");
  EXPECT_FALSE(isInferenceTarget(lookup("func", AST.context())));
}

TEST(IsInferenceTargetTest, FunctionWithPointerReturnIsTarget) {
  TestAST AST(R"cc(
    int* func();
  )cc");
  EXPECT_TRUE(isInferenceTarget(lookup("func", AST.context())));
}

TEST(IsInferenceTargetTest, FunctionWithPointerParamIsTargetButParamIsNot) {
  TestAST AST(R"cc(
    void func(int* Param);
  )cc");
  ASTContext& Ctx = AST.context();
  EXPECT_TRUE(isInferenceTarget(lookup("func", Ctx)));
  EXPECT_FALSE(isInferenceTarget(lookup("Param", Ctx)));
}

TEST(IsInferenceTargetTest, FunctionLocalPointerIsTarget) {
  TestAST AST((SmartPointerHeader + R"cc(
                void func() { int* Local; }
              )cc")
                  .str());
  EXPECT_TRUE(isInferenceTarget(lookup("Local", AST.context())));
}

TEST(IsInferenceTargetTest, StaticFunctionLocalPointerIsTarget) {
  TestAST AST((SmartPointerHeader + R"cc(
                void func() {
                  static int* StaticLocal;
                }
              )cc")
                  .str());
  EXPECT_TRUE(isInferenceTarget(lookup("StaticLocal", AST.context())));
}

TEST(IsInferenceTargetTest, FunctionLocalStdSmartPointerIsTarget) {
  TestAST AST((SmartPointerHeader + R"cc(
                void func() {
                  std::unique_ptr<int> StdSmartLocal;
                }
              )cc")
                  .str());
  EXPECT_TRUE(isInferenceTarget(lookup("StdSmartLocal", AST.context())));
}

TEST(IsInferenceTargetTest, FunctionLocalCustomSmartPointerIsTarget) {
  TestAST AST((SmartPointerHeader + R"cc(
                void func() {
                  custom_smart_ptr<int> CustomSmartLocal;
                }
              )cc")
                  .str());
  EXPECT_TRUE(isInferenceTarget(lookup("CustomSmartLocal", AST.context())));
}

TEST(IsInferenceTargetTest, LambdaWithoutPointerIsNotTarget) {
  TestAST AST(R"cc(
    auto Lambda = []() {};
  )cc");
  ASTContext& Ctx = AST.context();
  auto& Lambda = lookup<VarDecl>("Lambda", Ctx);
  CXXRecordDecl* LambdaCtx =
      cast<LambdaExpr>(Lambda.getInit())->getLambdaClass();
  EXPECT_FALSE(isInferenceTarget(Lambda));
  EXPECT_FALSE(isInferenceTarget(lookup("operator()", Ctx, LambdaCtx)));
}

TEST(IsInferenceTargetTest, LambdaWithPointerIsTarget) {
  TestAST AST(R"cc(
    auto LambdaWithPtr = [](int*) {};
  )cc");
  ASTContext& Ctx = AST.context();
  auto& LambdaWithPtr = lookup<VarDecl>("LambdaWithPtr", Ctx);
  CXXRecordDecl* LambdaWithPtrCtx =
      cast<LambdaExpr>(LambdaWithPtr.getInit())->getLambdaClass();
  EXPECT_FALSE(isInferenceTarget(LambdaWithPtr));
  EXPECT_TRUE(isInferenceTarget(lookup("operator()", Ctx, LambdaWithPtrCtx)));
}

TEST(IsInferenceTargetTest, ClassDeclIsNotTarget) {
  TestAST AST(R"cc(
    class C {};
  )cc");
  EXPECT_FALSE(isInferenceTarget(lookup<CXXRecordDecl>("C", AST.context())));
}

TEST(IsInferenceTargetTest, MethodWithoutPointerIsNotTarget) {
  TestAST AST(R"cc(
    class C {
      void method();
    };
  )cc");
  EXPECT_FALSE(isInferenceTarget(lookup("method", AST.context())));
}

TEST(IsInferenceTargetTest, MethodWithPointerIsTarget) {
  TestAST AST(R"cc(
    class C {
      int* methodWithPtr();
    };
  )cc");
  EXPECT_TRUE(isInferenceTarget(lookup("methodWithPtr", AST.context())));
}

TEST(IsInferenceTargetTest, StaticMethodWithPointerIsTarget) {
  TestAST AST(R"cc(
    class C {
      static int* staticMethodWithPtr();
    };
  )cc");
  EXPECT_TRUE(isInferenceTarget(lookup("staticMethodWithPtr", AST.context())));
}

TEST(IsInferenceTargetTest, NonPointerFieldIsNotTarget) {
  TestAST AST(R"cc(
    class C {
      int NonPtrField;
    };
  )cc");
  EXPECT_FALSE(isInferenceTarget(lookup("NonPtrField", AST.context())));
}

TEST(IsInferenceTargetTest, PointerFieldIsTarget) {
  TestAST AST(R"cc(
    class C {
      int* PtrField;
    };
  )cc");
  EXPECT_TRUE(isInferenceTarget(lookup("PtrField", AST.context())));
}

TEST(IsInferenceTargetTest, StaticPointerFieldIsTarget) {
  TestAST AST(R"cc(
    class C {
      static int* StaticField;
    };
  )cc");
  EXPECT_TRUE(isInferenceTarget(lookup("StaticField", AST.context())));
}

TEST(IsInferenceTargetTest, StdSmartPointerFieldIsTarget) {
  TestAST AST((SmartPointerHeader + R"cc(
                class C {
                  std::unique_ptr<int> StdSmartField;
                };
              )cc")
                  .str());
  EXPECT_TRUE(isInferenceTarget(lookup("StdSmartField", AST.context())));
}

TEST(IsInferenceTargetTest, CustomSmartPointerFieldIsTarget) {
  TestAST AST((SmartPointerHeader + R"cc(
                class C {
                  custom_smart_ptr<int> CustomSmartField;
                };
              )cc")
                  .str());
  EXPECT_TRUE(isInferenceTarget(lookup("CustomSmartField", AST.context())));
}

TEST(IsInferenceTargetTest, FunctionTemplateDeclIsNotTarget) {
  TestAST AST(R"cc(
    template <int X>
    void funcTmpl(int*) {}

    // To demonstrate that the existence of an instantiation is not a factor.
    void instantiate() { funcTmpl<0>(nullptr); }
  )cc");
  const FunctionTemplateDecl& FuncTmpl =
      lookup<FunctionTemplateDecl>("funcTmpl", AST.context());
  EXPECT_FALSE(isInferenceTarget(FuncTmpl));
  EXPECT_FALSE(isInferenceTarget(*FuncTmpl.getTemplatedDecl()));
}

TEST(IsInferenceTargetTest, LocalInFunctionTemplateDeclIsNotTarget) {
  TestAST AST(R"cc(
    template <int X>
    void funcTmpl(int*) {
      int* LocalInTmpl;
    }

    // To demonstrate that the existence of an instantiation is not a factor.
    void instantiate() { funcTmpl<0>(nullptr); }
  )cc");
  ASTContext& Ctx = AST.context();
  const FunctionTemplateDecl& FuncTmpl =
      lookup<FunctionTemplateDecl>("funcTmpl", Ctx);
  EXPECT_FALSE(isInferenceTarget(
      lookup("LocalInTmpl", Ctx, FuncTmpl.getTemplatedDecl())));
}

TEST(IsInferenceTargetTest, MethodInFunctionTemplateDeclIsNotTarget) {
  TestAST AST(R"cc(
    template <int X>
    void funcTmpl(int*) {
      struct StructInTmpl {
        void funcInStructInTmpl(int* P) {}
      };
    }

    // To demonstrate that the existence of an instantiation is not a factor.
    void instantiate() { funcTmpl<0>(nullptr); }
  )cc");
  ASTContext& Ctx = AST.context();
  const FunctionTemplateDecl& FuncTmpl =
      lookup<FunctionTemplateDecl>("funcTmpl", Ctx);
  auto* StructInTmpl =
      &lookup<CXXRecordDecl>("StructInTmpl", Ctx, FuncTmpl.getTemplatedDecl());
  const NamedDecl& FuncInStructInTmpl =
      lookup("funcInStructInTmpl", Ctx, StructInTmpl);
  EXPECT_FALSE(isInferenceTarget(FuncInStructInTmpl));
}

TEST(IsInferenceTargetTest, LocalInMethodInFunctionTemplateDeclIsNotTarget) {
  TestAST AST(R"cc(
    template <int X>
    void funcTmpl(int*) {
      struct StructInTmpl {
        void funcInStructInTmpl() { int* LocalInStructInTmpl; }
      };
    }

    // To demonstrate that the existence of an instantiation is not a factor.
    void instantiate() { funcTmpl<0>(nullptr); }
  )cc");
  ASTContext& Ctx = AST.context();
  const FunctionTemplateDecl& FuncTmpl =
      lookup<FunctionTemplateDecl>("funcTmpl", Ctx);
  auto* StructInTmpl =
      &lookup<CXXRecordDecl>("StructInTmpl", Ctx, FuncTmpl.getTemplatedDecl());
  const NamedDecl& FuncInStructInTmpl =
      lookup("funcInStructInTmpl", Ctx, StructInTmpl);
  EXPECT_FALSE(isInferenceTarget(
      lookup("LocalInStructInTmpl", Ctx, &FuncInStructInTmpl)));
}

TEST(IsInferenceTargetTest, FieldInFunctionTemplateDeclIsNotTarget) {
  TestAST AST(R"cc(
    template <int X>
    void funcTmpl(int*) {
      struct StructInTmpl {
        int* FieldInStructInTmpl;
      };
    }

    // To demonstrate that the existence of an instantiation is not a factor.
    void instantiate() { funcTmpl<0>(nullptr); }
  )cc");
  ASTContext& Ctx = AST.context();
  auto& FuncTmpl = lookup<FunctionTemplateDecl>("funcTmpl", Ctx);
  auto* StructInTmpl =
      &lookup<CXXRecordDecl>("StructInTmpl", Ctx, FuncTmpl.getTemplatedDecl());
  EXPECT_FALSE(
      isInferenceTarget(lookup("FieldInStructInTmpl", Ctx, StructInTmpl)));
}

TEST(IsInferenceTargetTest, FunctionTemplateInstantiationIsTarget) {
  TestAST AST(R"cc(
    template <int X>
    void funcTmpl(int*) {}

    auto& FuncTmplInst = funcTmpl<2>;
  )cc");
  const ValueDecl& Instantiation =
      *cast<DeclRefExpr>(lookup<VarDecl>("FuncTmplInst", AST.context())
                             .getInit()
                             ->IgnoreImplicit())
           ->getDecl();
  EXPECT_TRUE(isInferenceTarget(Instantiation));
}

TEST(IsInferenceTargetTest, LocalInFunctionTemplateInstantiationIsTarget) {
  TestAST AST(R"cc(
    template <int X>
    void funcTmpl(int*) {
      int* LocalInTmpl;
    }

    auto& FuncTmplInst = funcTmpl<2>;
  )cc");
  ASTContext& Ctx = AST.context();
  const ValueDecl& Instantiation =
      *cast<DeclRefExpr>(
           lookup<VarDecl>("FuncTmplInst", Ctx).getInit()->IgnoreImplicit())
           ->getDecl();
  EXPECT_TRUE(isInferenceTarget(lookup("LocalInTmpl", Ctx, &Instantiation)));
}

TEST(IsInferenceTargetTest, MethodInFunctionTemplateInstantiationIsTarget) {
  TestAST AST(R"cc(
    template <int X>
    void funcTmpl(int*) {
      struct StructInTmpl {
        void funcInStructInTmpl(int* P) {}
      };
    }

    auto& FuncTmplInst = funcTmpl<2>;
  )cc");
  ASTContext& Ctx = AST.context();
  const ValueDecl& Instantiation =
      *cast<DeclRefExpr>(
           lookup<VarDecl>("FuncTmplInst", Ctx).getInit()->IgnoreImplicit())
           ->getDecl();
  auto* StructInInstantiation =
      &lookup<CXXRecordDecl>("StructInTmpl", Ctx, &Instantiation);
  const NamedDecl& FuncInStructInInstantiation =
      lookup("funcInStructInTmpl", Ctx, StructInInstantiation);
  EXPECT_TRUE(isInferenceTarget(FuncInStructInInstantiation));
}

TEST(IsInferenceTargetTest,
     LocalInMethodInFunctionTemplateInstantiationIsTarget) {
  TestAST AST(R"cc(
    template <int X>
    void funcTmpl(int*) {
      struct StructInTmpl {
        void funcInStructInTmpl() { int* LocalInStructInTmpl; }
      };
    }

    auto& FuncTmplInst = funcTmpl<2>;
  )cc");
  ASTContext& Ctx = AST.context();
  const ValueDecl& Instantiation =
      *cast<DeclRefExpr>(
           lookup<VarDecl>("FuncTmplInst", Ctx).getInit()->IgnoreImplicit())
           ->getDecl();
  auto* StructInInstantiation =
      &lookup<CXXRecordDecl>("StructInTmpl", Ctx, &Instantiation);
  const NamedDecl& FuncInStructInInstantiation =
      lookup("funcInStructInTmpl", Ctx, StructInInstantiation);
  EXPECT_TRUE(isInferenceTarget(
      lookup("LocalInStructInTmpl", Ctx, &FuncInStructInInstantiation)));
}

TEST(IsInferenceTargetTest, FieldInFunctionTemplateInstantiationIsTarget) {
  TestAST AST(R"cc(
    template <int X>
    void funcTmpl(int*) {
      struct StructInTmpl {
        int* FieldInStructInTmpl;
      };
    }

    auto& FuncTmplInst = funcTmpl<2>;
  )cc");
  ASTContext& Ctx = AST.context();
  const ValueDecl& Instantiation =
      *cast<DeclRefExpr>(
           lookup<VarDecl>("FuncTmplInst", Ctx).getInit()->IgnoreImplicit())
           ->getDecl();
  auto* StructInInstantiation =
      &lookup<CXXRecordDecl>("StructInTmpl", Ctx, &Instantiation);
  EXPECT_TRUE(isInferenceTarget(
      lookup("FieldInStructInTmpl", Ctx, StructInInstantiation)));
}

TEST(IsInferenceTargetTest, ClassTemplateDeclIsNotTarget) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {};

    // To demonstrate that the existence of an instantiation is not a factor.
    ClassTemplate<int*> Instantiation;
  )cc");
  auto& ClassTemplate =
      lookup<ClassTemplateDecl>("ClassTemplate", AST.context());
  EXPECT_FALSE(isInferenceTarget(ClassTemplate));
  EXPECT_FALSE(isInferenceTarget(*ClassTemplate.getTemplatedDecl()));
}

// We do not exhaustively test all the possible decls that can be inside a class
// template, as this would be repetitive with the function template test cases
// above.
TEST(IsInferenceTargetTest, FieldInClassTemplateDeclIsNotTarget) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {
      T* PtrField;
    };

    // To demonstrate that the existence of an instantiation is not a factor.
    ClassTemplate<int*> Instantiation;
  )cc");
  ASTContext& Ctx = AST.context();
  auto& ClassTemplate = lookup<ClassTemplateDecl>("ClassTemplate", Ctx);
  EXPECT_FALSE(isInferenceTarget(
      lookup("PtrField", Ctx, ClassTemplate.getTemplatedDecl())));
}

TEST(IsInferenceTargetTest, ClassTemplateInstantiationWithNonPointer) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {};

    ClassTemplate<int> Instantiation;
  )cc");
  EXPECT_FALSE(
      isInferenceTarget(*lookup<VarDecl>("Instantiation", AST.context())
                             .getType()
                             ->getAsRecordDecl()));
}

TEST(IsInferenceTargetTest, NonPointerFieldInClassTemplateInstantiation) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {
      T NonPtrField;
    };

    ClassTemplate<int> Instantiation;
    int Field = Instantiation.NonPtrField;
  )cc");
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("Field", AST.context()).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
}

TEST(IsInferenceTargetTest, PointerFieldInClassTemplateInstantiation) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {
      T* PtrField;
    };

    ClassTemplate<int> Instantiation;
    int* Field = Instantiation.PtrField;
  )cc");
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("Field", AST.context()).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
}

TEST(IsInferenceTargetTest, StaticPointerFieldInClassTemplateInstantiation) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {
      static T* StaticField;
    };

    ClassTemplate<int> Instantiation;
    int* Field = Instantiation.StaticField;
  )cc");
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("Field", AST.context()).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
}

TEST(IsInferenceTargetTest, MethodWithoutPointersInClassTemplateInstantiation) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {
      T method();
    };

    ClassTemplate<int> Instantiation;
    int MethodResult = Instantiation.method();
  )cc");
  EXPECT_FALSE(isInferenceTarget(
      *cast<CXXMemberCallExpr>(lookup<VarDecl>("MethodResult", AST.context())
                                   .getInit()
                                   ->IgnoreImplicit())
           ->getMethodDecl()));
}

TEST(IsInferenceTargetTest, MethodWithPointerInClassTemplateInstantiation) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {
      T* methodWithPtr();
    };

    ClassTemplate<int> Instantiation;
    int* MethodResult = Instantiation.methodWithPtr();
  )cc");
  EXPECT_TRUE(isInferenceTarget(
      *cast<CXXMemberCallExpr>(lookup<VarDecl>("MethodResult", AST.context())
                                   .getInit()
                                   ->IgnoreImplicit())
           ->getMethodDecl()));
}

TEST(IsInferenceTargetTest,
     PointerFieldInNestedStructInClassTemplateInstantiation) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {
      struct Nested {
        struct NestedTwo {
          T* NestedStructPtrField;
        };
        NestedTwo NestedStructTwo;
      };
      Nested NestedStruct;
    };

    ClassTemplate<int> Instantiation;
    int* Field = Instantiation.NestedStruct.NestedStructTwo.NestedStructPtrField;
  )cc");
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("Field", AST.context()).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
}

TEST(IsInferenceTargetTest,
     MethodWithNonDependentPointerInNestedStructInClassTemplateInstantiation) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {
      struct Nested {
        struct NestedTwo {
          bool* nestedStructMethod();
        };
        NestedTwo NestedStructTwo;
      };
      Nested NestedStruct;
    };

    ClassTemplate<int> Instantiation;
    bool* MethodResult =
        Instantiation.NestedStruct.NestedStructTwo.nestedStructMethod();
  )cc");
  EXPECT_TRUE(isInferenceTarget(
      *cast<CXXMemberCallExpr>(lookup<VarDecl>("MethodResult", AST.context())
                                   .getInit()
                                   ->IgnoreImplicit())
           ->getMethodDecl()));
}

TEST(IsInferenceTargetTest, ClassTemplateInstantiationWithPointer) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {};

    ClassTemplate<int*> Instantiation;
  )cc");
  EXPECT_FALSE(
      isInferenceTarget(*lookup<VarDecl>("Instantiation", AST.context())
                             .getType()
                             ->getAsRecordDecl()));
}

TEST(IsInferenceTargetTest,
     FieldMadePointerOnlyByTemplateArgumentInClassTemplateInstantiation) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {
      T Field;
    };

    ClassTemplate<int*> Instantiation;
    int* F = Instantiation.Field;
  )cc");
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("F", AST.context()).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
}

TEST(
    IsInferenceTargetTest,
    MethodContainingPointerOnlyByTemplateArgumentInClassTemplateInstantiation) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {
      T method();
    };

    ClassTemplate<int*> Instantiation;
    int* MethodResult = Instantiation.method();
  )cc");
  EXPECT_FALSE(isInferenceTarget(
      *cast<CXXMemberCallExpr>(lookup<VarDecl>("MethodResult", AST.context())
                                   .getInit()
                                   ->IgnoreImplicit())
           ->getMethodDecl()));
}

TEST(IsInferenceTargetTest,
     FieldMadePointerThroughAliasToTemplateArgInClassTemplateInstantiation) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {
      using U = T;
      U NotInferableThroughAlias;
    };

    ClassTemplate<int*> Instantiation;
    int* F = Instantiation.NotInferableThroughAlias;
  )cc");
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("F", AST.context()).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
}

TEST(IsInferenceTargetTest,
     TypeAliasTemplateUsedWithPointerPresentInClassTemplate) {
  TestAST AST(R"cc(
    template <typename T>
    using Alias = T;

    template <typename U>
    struct ClassTemplate {
      Alias<U*> FieldOfAliasTypeWithPointerInClassTemplate;
    };

    ClassTemplate<int> Instantiation;
    int* Field = Instantiation.FieldOfAliasTypeWithPointerInClassTemplate;
  )cc");
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("Field", AST.context()).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
}

TEST(IsInferenceTargetTest,
     TypeAliasTemplateUsedWithPointerInClassTemplateArgument) {
  TestAST AST(R"cc(
    template <typename T>
    using Alias = T;

    template <typename U>
    struct ClassTemplate {
      Alias<U> FieldOfAliasTypeWithoutPointerInClassTemplate;
    };

    ClassTemplate<int*> Instantiation;
    int* Field = Instantiation.FieldOfAliasTypeWithoutPointerInClassTemplate;
  )cc");
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("Field", AST.context()).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
}

TEST(IsInferenceTargetTest, TypeAliasTemplateWithPointerInAliasTemplate) {
  TestAST AST(R"cc(
    template <typename T>
    using PtrAlias = T*;

    template <typename U>
    struct ClassTemplate {
      PtrAlias<U> FieldOfAliasTypeWithPointerInAlias;
    };

    ClassTemplate<int> Instantiation;
    int* Field = Instantiation.FieldOfAliasTypeWithPointerInAlias;
  )cc");
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("Field", AST.context()).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
}

TEST(IsInferenceTargetTest, NestedTemplatesWithPointerInNestedTemplate) {
  TestAST AST(R"cc(
    template <typename T>
    struct Outer {
      template <typename U>
      struct Inner {
        U* FieldWithPointerInNestedTemplate;
      };
      Inner<bool> AnInner;
    };

    Outer<int> AnOuter;
    bool* Field = AnOuter.AnInner.FieldWithPointerInNestedTemplate;
  )cc");
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("Field", AST.context()).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
}

TEST(IsInferenceTargetTest,
     NestedTemplatesWithPointerInNestedTemplateArgument) {
  TestAST AST(R"cc(
    template <typename T>
    struct Outer {
      template <typename U>
      struct Inner {
        U FieldWithoutPointerInNestedTemplate;
      };
      Inner<bool*> AnInner;
    };

    Outer<int> AnOuter;
    bool* Field = AnOuter.AnInner.FieldWithoutPointerInNestedTemplate;
  )cc");
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("Field", AST.context()).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
}

TEST(IsInferenceTargetTest,
     NestedTemplatesWithPointerUsingOuterTemplateParameter) {
  TestAST AST(R"cc(
    template <typename T>
    struct Outer {
      template <typename U>
      struct Inner {
        T* FieldwithPointerUsingOuterTemplateParameter;
      };
      Inner<bool> AnInner;
    };

    Outer<int> AnOuter;
    int* Field = AnOuter.AnInner.FieldwithPointerUsingOuterTemplateParameter;
  )cc");
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("Field", AST.context()).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
}

struct CountInferableSlotsTestInput {
  std::string TestCaseName;
  std::string InputCode;
  int ExpectedSlots;
  std::string TargetName = "target";
};

class CountInferableSlotsTest
    : public testing::TestWithParam<CountInferableSlotsTestInput> {};

TEST_P(CountInferableSlotsTest, CountsInferableSlots) {
  TestAST AST(GetParam().InputCode);
  EXPECT_EQ(countInferableSlots(lookup(GetParam().TargetName, AST.context())),
            GetParam().ExpectedSlots);
}

INSTANTIATE_TEST_SUITE_P(
    CountInferableSlotsTests, CountInferableSlotsTest,
    testing::Values(
        CountInferableSlotsTestInput{.TestCaseName = "ParamRawPointer",
                                     .InputCode = R"cc(
                                       void target(int*);
                                     )cc",
                                     .ExpectedSlots = 1},
        CountInferableSlotsTestInput{.TestCaseName = "ParamAlias",
                                     .InputCode = R"cc(
                                       using Pointer = int*;
                                       void target(Pointer);
                                     )cc",
                                     .ExpectedSlots = 1},
        CountInferableSlotsTestInput{.TestCaseName = "ParamNested",
                                     .InputCode = R"cc(
                                       void target(int**);
                                     )cc",
                                     .ExpectedSlots = 1},
        CountInferableSlotsTestInput{.TestCaseName = "ParamAliasNested",
                                     .InputCode = R"cc(
                                       using Pointer = int*;
                                       void target(Pointer*);
                                     )cc",
                                     .ExpectedSlots = 1},
        CountInferableSlotsTestInput{.TestCaseName = "ParamReference",
                                     .InputCode = R"cc(
                                       void target(int*&);
                                     )cc",
                                     .ExpectedSlots = 1},
        CountInferableSlotsTestInput{.TestCaseName = "ParamFunctionPointer",
                                     .InputCode = R"cc(
                                       void target(int (*)());
                                     )cc",
                                     .ExpectedSlots = 1},
        CountInferableSlotsTestInput{
            .TestCaseName = "ParamStdSmartPointer",
            .InputCode = (SmartPointerHeader + R"cc(
                           void target(std::unique_ptr<int>);
                         )cc")
                             .str(),
            .ExpectedSlots = 1},
        CountInferableSlotsTestInput{
            .TestCaseName = "ParamCustomSmartPointer",
            .InputCode = (SmartPointerHeader + R"cc(
                           void target(custom_smart_ptr<int>);
                         )cc")
                             .str(),
            .ExpectedSlots = 1},
        CountInferableSlotsTestInput{.TestCaseName = "ReturnRawPointer",
                                     .InputCode = R"cc(
                                       int* target();
                                     )cc",
                                     .ExpectedSlots = 1},
        CountInferableSlotsTestInput{.TestCaseName = "ReturnAlias",
                                     .InputCode = R"cc(
                                       using Pointer = int*;
                                       Pointer target();
                                     )cc",
                                     .ExpectedSlots = 1},
        CountInferableSlotsTestInput{.TestCaseName = "ReturnNested",
                                     .InputCode = R"cc(
                                       int** target();
                                     )cc",
                                     .ExpectedSlots = 1},
        CountInferableSlotsTestInput{.TestCaseName = "ReturnAliasNested",
                                     .InputCode = R"cc(
                                       using Pointer = int*;
                                       Pointer* target();
                                     )cc",
                                     .ExpectedSlots = 1},
        CountInferableSlotsTestInput{.TestCaseName = "ReturnReference",
                                     .InputCode = R"cc(
                                       int*& target();
                                     )cc",
                                     .ExpectedSlots = 1},
        CountInferableSlotsTestInput{.TestCaseName = "ReturnFunctionPointer",
                                     .InputCode = R"cc(
                                       using FnPtr = int (*)();
                                       FnPtr target();
                                     )cc",
                                     .ExpectedSlots = 1},
        CountInferableSlotsTestInput{
            .TestCaseName = "ReturnStdSmartPointer",
            .InputCode = (SmartPointerHeader + R"cc(
                           std::unique_ptr<int> target();
                         )cc")
                             .str(),
            .ExpectedSlots = 1},
        CountInferableSlotsTestInput{
            .TestCaseName = "ReturnCustomSmartPointer",
            .InputCode = (SmartPointerHeader + R"cc(
                           custom_smart_ptr<int> target();
                         )cc")
                             .str(),
            .ExpectedSlots = 1},
        CountInferableSlotsTestInput{.TestCaseName = "TemplateOfPointer",
                                     .InputCode = R"cc(
                                       template <typename T>
                                       struct S {};
                                       S<int*> Target;
                                     )cc",
                                     .ExpectedSlots = 0,
                                     .TargetName = "Target"},
        CountInferableSlotsTestInput{.TestCaseName = "PointerToDataMember",
                                     .InputCode = R"cc(
                                       struct T;
                                       int T::* Target;
                                     )cc",
                                     .ExpectedSlots = 0,
                                     .TargetName = "Target"},
        CountInferableSlotsTestInput{.TestCaseName = "PointerToMemberFunction",
                                     .InputCode = R"cc(
                                       struct T;
                                       int (T::*Target)();
                                     )cc",
                                     .ExpectedSlots = 0,
                                     .TargetName = "Target"}),
    [](testing::TestParamInfo<CountInferableSlotsTestInput> Info) {
      return Info.param.TestCaseName;
    });

struct GetInferableSlotIndicesTestInput {
  std::string TestCaseName;
  std::string InputCode;
  std::vector<int> ExpectedSlotIndices;
  std::string TargetName = "target";
};

class GetInferableSlotIndicesTest
    : public testing::TestWithParam<GetInferableSlotIndicesTestInput> {};

TEST_P(GetInferableSlotIndicesTest, GetsInferableSlotIndices) {
  TestAST AST(GetParam().InputCode);
  EXPECT_THAT(
      getInferableSlotIndices(lookup(GetParam().TargetName, AST.context())),
      UnorderedElementsAreArray(GetParam().ExpectedSlotIndices));
}

INSTANTIATE_TEST_SUITE_P(
    GetInferableSlotIndicesTest, GetInferableSlotIndicesTest,
    testing::Values(
        GetInferableSlotIndicesTestInput{
            .TestCaseName = "PointerAndNonPointerParams",
            .InputCode = R"cc(
              void target(int**, int, char, char*);
            )cc",
            .ExpectedSlotIndices = {1, 4},
        },
        GetInferableSlotIndicesTestInput{
            .TestCaseName = "PointerReturnWithPointerAndNonPointerParams",
            .InputCode = R"cc(
              int* target(bool, bool*);
            )cc",
            .ExpectedSlotIndices = {0, 2},
        },
        GetInferableSlotIndicesTestInput{
            .TestCaseName = "StdSmartPointerParam",
            .InputCode = (SmartPointerHeader + R"cc(
                           void target(std::unique_ptr<int>);
                         )cc")
                             .str(),
            .ExpectedSlotIndices = {1},
        },
        GetInferableSlotIndicesTestInput{
            .TestCaseName = "CustomSmartPointerParam",
            .InputCode = (SmartPointerHeader + R"cc(
                           void target(custom_smart_ptr<int>);
                         )cc")
                             .str(),
            .ExpectedSlotIndices = {1},
        },
        GetInferableSlotIndicesTestInput{
            .TestCaseName = "NonPointerVariable",
            .InputCode = R"cc(
              int Target;
            )cc",
            .ExpectedSlotIndices = {},
            .TargetName = "Target",
        },
        GetInferableSlotIndicesTestInput{
            .TestCaseName = "PointerVariable",
            .InputCode = R"cc(
              int* Target;
            )cc",
            .ExpectedSlotIndices = {0},
            .TargetName = "Target",
        },
        GetInferableSlotIndicesTestInput{
            .TestCaseName = "NestedPointerVariable",
            .InputCode = R"cc(
              int** Target;
            )cc",
            .ExpectedSlotIndices = {0},
            .TargetName = "Target",
        },
        GetInferableSlotIndicesTestInput{
            .TestCaseName = "NonPointerClassMember",
            .InputCode = R"cc(
              class C {
                int Target;
              };
            )cc",
            .ExpectedSlotIndices = {},
            .TargetName = "Target",
        },
        GetInferableSlotIndicesTestInput{
            .TestCaseName = "PointerClassMember",
            .InputCode = R"cc(
              class C {
                int* Target;
              };
            )cc",
            .ExpectedSlotIndices = {0},
            .TargetName = "Target",
        },
        GetInferableSlotIndicesTestInput{
            .TestCaseName = "NestedPointerClassMember",
            .InputCode = R"cc(
              class C {
                int** Target;
              };
            )cc",
            .ExpectedSlotIndices = {0},
            .TargetName = "Target",
        },
        GetInferableSlotIndicesTestInput{
            .TestCaseName = "PointerContainingClassMemberFunction",
            .InputCode = R"cc(
              class C {
                int* target(bool, bool*);
              };
            )cc",
            .ExpectedSlotIndices = {0, 2},
        }),
    [](testing::TestParamInfo<GetInferableSlotIndicesTestInput> Info) {
      return Info.param.TestCaseName;
    });

TEST(HasInferableTest,
     LocalInFunctionTemplateInstantiationWithTemplateArgumentType) {
  TestAST AST(R"cc(
    template <typename T>
    void funcTmpl() {
      T LocalInTmpl;
    }

    auto& FuncTmplInst = funcTmpl<int*>;
  )cc");
  ASTContext& Ctx = AST.context();
  const ValueDecl& Instantiation =
      *cast<DeclRefExpr>(
           lookup<VarDecl>("FuncTmplInst", Ctx).getInit()->IgnoreImplicit())
           ->getDecl();
  EXPECT_FALSE(hasInferable(
      lookup<VarDecl>("LocalInTmpl", Ctx, &Instantiation).getType()));
}

TEST(HasInferableTest,
     LocalInFunctionTemplateInstantiationWithPointerToTemplateArgumentType) {
  TestAST AST(R"cc(
    template <typename T>
    void funcTmpl() {
      T* LocalInTmpl;
    }

    auto& FuncTmplInst = funcTmpl<int>;
  )cc");
  ASTContext& Ctx = AST.context();
  const ValueDecl& Instantiation =
      *cast<DeclRefExpr>(
           lookup<VarDecl>("FuncTmplInst", Ctx).getInit()->IgnoreImplicit())
           ->getDecl();
  EXPECT_TRUE(hasInferable(
      lookup<VarDecl>("LocalInTmpl", Ctx, &Instantiation).getType()));
}

TEST(HasInferableTest,
     ParamInFunctionTemplateInstantiationWithTemplateArgumentType) {
  TestAST AST(R"cc(
    template <typename T>
    void funcTmpl(T ParamInTmpl) {}

    auto& FuncTmplInst = funcTmpl<int*>;
  )cc");
  ASTContext& Ctx = AST.context();
  const ValueDecl& Instantiation =
      *cast<DeclRefExpr>(
           lookup<VarDecl>("FuncTmplInst", Ctx).getInit()->IgnoreImplicit())
           ->getDecl();
  EXPECT_FALSE(hasInferable(
      lookup<ParmVarDecl>("ParamInTmpl", Ctx, &Instantiation).getType()));
}

TEST(HasInferableTest,
     ParamInFunctionTemplateInstantiationWithUniversalRefTemplateArgumentType) {
  TestAST AST(R"cc(
    template <typename T>
    void funcTmpl(T&& ParamInTmpl) {}

    // Instantiations with an lvalue reference result in a parameter type that
    // does not contain a SubstTemplateTypeParmType node, so we don't detect
    // that it is a substituted template parameter.
    auto& FuncTmplInstLValue = funcTmpl<int*&>;

    // Without the reference component of the template argument, the type of
    // ParamInTmpl is an rvalue reference.
    auto& FuncTmplInstRValue = funcTmpl<int*>;
  )cc");
  ASTContext& Ctx = AST.context();
  const ValueDecl& LValueInstantiation =
      *cast<DeclRefExpr>(lookup<VarDecl>("FuncTmplInstLValue", Ctx)
                             .getInit()
                             ->IgnoreImplicit())
           ->getDecl();
  // Ideally, this would be false.
  EXPECT_TRUE(hasInferable(
      lookup<ParmVarDecl>("ParamInTmpl", Ctx, &LValueInstantiation).getType()));

  const ValueDecl& RValueInstantiation =
      *cast<DeclRefExpr>(lookup<VarDecl>("FuncTmplInstRValue", Ctx)
                             .getInit()
                             ->IgnoreImplicit())
           ->getDecl();
  EXPECT_FALSE(hasInferable(
      lookup<ParmVarDecl>("ParamInTmpl", Ctx, &RValueInstantiation).getType()));
}

TEST(HasInferableTest,
     ParamInFunctionTemplateInstantiationWithPointerToTemplateArgumentType) {
  TestAST AST(R"cc(
    template <typename T>
    void funcTmpl(T* ParamInTmpl) {}

    auto& FuncTmplInst = funcTmpl<int>;
  )cc");
  ASTContext& Ctx = AST.context();
  const ValueDecl& Instantiation =
      *cast<DeclRefExpr>(
           lookup<VarDecl>("FuncTmplInst", Ctx).getInit()->IgnoreImplicit())
           ->getDecl();
  EXPECT_TRUE(hasInferable(
      lookup<ParmVarDecl>("ParamInTmpl", Ctx, &Instantiation).getType()));
}

TEST(HasInferableTest,
     FunctionTemplateInstantiationReturnOfTemplateArgumentType) {
  TestAST AST(R"cc(
    template <typename T>
    T funcTmpl() {}

    auto& FuncTmplInst = funcTmpl<int*>;
  )cc");
  const FunctionDecl* Instantiation = cast<FunctionDecl>(
      cast<DeclRefExpr>(lookup<VarDecl>("FuncTmplInst", AST.context())
                            .getInit()
                            ->IgnoreImplicit())
          ->getDecl());
  EXPECT_FALSE(hasInferable(Instantiation->getReturnType()));
}

TEST(HasInferableTest,
     FunctionTemplateInstantiationReturnOfPointerToTemplateArgumentType) {
  TestAST AST(R"cc(
    template <typename T>
    T* funcTmpl() {}

    auto& FuncTmplInst = funcTmpl<int*>;
  )cc");
  const FunctionDecl* Instantiation = cast<FunctionDecl>(
      cast<DeclRefExpr>(lookup<VarDecl>("FuncTmplInst", AST.context())
                            .getInit()
                            ->IgnoreImplicit())
          ->getDecl());
  EXPECT_TRUE(hasInferable(Instantiation->getReturnType()));
}

TEST(HasInferableTest, TypeInsideTemplateTypeParamIsNotInferableBecauseOfName) {
  TestAST AST(R"cc(
    template <class T>
    // Being named Ptr does not guarantee that the return type will always be a
    // pointer. So, we still don't annotate unless the pointer-ness is part of
    // the template declaration.
    T::Ptr ReturnsTypeNamedPtr();

    class S {
     public:
      typedef const char* Ptr;
    };

    auto& FuncTmplInst = ReturnsTypeNamedPtr<S>;
  )cc");
  const FunctionDecl* Instantiation = cast<FunctionDecl>(
      cast<DeclRefExpr>(lookup<VarDecl>("FuncTmplInst", AST.context())
                            .getInit()
                            ->IgnoreImplicit())
          ->getDecl());
  EXPECT_FALSE(hasInferable(Instantiation->getReturnType()));
}

TEST(HasInferableTest,
     TypeInsideTemplateTypeParamIsInferableIfPointerIsInsideTemplate) {
  TestAST AST(R"cc(
    template <class T>
    T::Ptr* ReturnsPtr();

    class S {
     public:
      typedef const char* Ptr;
    };

    auto& FuncTmplInst = ReturnsPtr<S>;
  )cc");
  const FunctionDecl* Instantiation = cast<FunctionDecl>(
      cast<DeclRefExpr>(lookup<VarDecl>("FuncTmplInst", AST.context())
                            .getInit()
                            ->IgnoreImplicit())
          ->getDecl());
  EXPECT_TRUE(hasInferable(Instantiation->getReturnType()));
}

TEST(HasInferableTest, ClassTemplateInstanceWithPointerTemplateArgument) {
  TestAST AST(R"cc(
    template <typename T>
    struct S {};

    S<int*> Instance;
  )cc");

  EXPECT_FALSE(
      hasInferable(lookup<VarDecl>("Instance", AST.context()).getType()));
}

TEST(HasInferableTest, StdVectorOfPointerNotInferableWithoutFlagEnabling) {
  TestAST AST(R"cc(
    namespace std {
    template <typename T>
    class vector {};
    }  // namespace std

    std::vector<int*> Instance;
  )cc");

  bool PreviousState = selectTemplatesOfPointersInferable();
  setSelectTemplatesOfPointersInferable(false);
  EXPECT_FALSE(
      hasInferable(lookup<VarDecl>("Instance", AST.context()).getType()));

  // Reset global state for other tests.
  setSelectTemplatesOfPointersInferable(PreviousState);
}

TEST(HasInferableTest, AbslStatusOrOfPointerNotInferableWithoutFlagEnabling) {
  TestAST AST(R"cc(
    namespace absl {
    template <typename T>
    class StatusOr {};
    }  // namespace absl

    absl::StatusOr<int*> Instance;
  )cc");

  bool PreviousState = selectTemplatesOfPointersInferable();
  setSelectTemplatesOfPointersInferable(false);
  EXPECT_FALSE(
      hasInferable(lookup<VarDecl>("Instance", AST.context()).getType()));

  // Reset global state for other tests.
  setSelectTemplatesOfPointersInferable(PreviousState);
}

class HasInferableTestWithSelectTemplatesOfPointersInferable
    : public testing::Test {
 public:
  static void SetUpTestSuite() {
    PreviousState = selectTemplatesOfPointersInferable();
    setSelectTemplatesOfPointersInferable(true);
  }

  static void TearDownTestSuite() {
    setSelectTemplatesOfPointersInferable(PreviousState);
  }

  static bool PreviousState;
};

// Initialize to false arbitrarily.
bool HasInferableTestWithSelectTemplatesOfPointersInferable::PreviousState =
    false;

TEST_F(HasInferableTestWithSelectTemplatesOfPointersInferable,
       NonSelectClassTemplateInstanceWithPointerTemplateArgument) {
  TestAST AST(R"cc(
    template <typename T>
    struct S {};

    S<int*> Instance;
  )cc");

  // The pointer in this arbitrary template type is not one of the "select"
  // inferable pointers.
  EXPECT_FALSE(
      hasInferable(lookup<VarDecl>("Instance", AST.context()).getType()));
}

TEST_F(HasInferableTestWithSelectTemplatesOfPointersInferable,
       VectorOutsideStdIsNotASelectTemplate) {
  TestAST AST(R"cc(
    template <typename T>
    class vector {};

    vector<int*> Instance;
  )cc");

  EXPECT_FALSE(
      hasInferable(lookup<VarDecl>("Instance", AST.context()).getType()));
}

TEST_F(HasInferableTestWithSelectTemplatesOfPointersInferable,
       StatusOrOutsideAbslIsNotASelectTemplate) {
  TestAST AST(R"cc(
    template <typename T>
    class StatusOr {};

    StatusOr<int*> Instance;
  )cc");

  EXPECT_FALSE(
      hasInferable(lookup<VarDecl>("Instance", AST.context()).getType()));
}

TEST_F(HasInferableTestWithSelectTemplatesOfPointersInferable,
       StdVectorIsASelectTemplate) {
  TestAST AST(R"cc(
    namespace std {
    template <typename T>
    class vector {};
    }  // namespace std

    std::vector<int*> Instance;
  )cc");

  EXPECT_TRUE(
      hasInferable(lookup<VarDecl>("Instance", AST.context()).getType()));
}

TEST_F(HasInferableTestWithSelectTemplatesOfPointersInferable,
       AbslStatusOrIsASelectTemplate) {
  TestAST AST(R"cc(
    namespace absl {
    template <typename T>
    class StatusOr {};
    }  // namespace absl

    absl::StatusOr<int*> Instance;
  )cc");

  EXPECT_TRUE(
      hasInferable(lookup<VarDecl>("Instance", AST.context()).getType()));
}

TEST_F(HasInferableTestWithSelectTemplatesOfPointersInferable,
       ReferenceToSelectTemplateIsInferable) {
  TestAST AST(R"cc(
    namespace std {
    template <typename T>
    class vector {};
    }  // namespace std

    std::vector<int*>& GetRef();
  )cc");

  EXPECT_TRUE(hasInferable(
      lookup<FunctionDecl>("GetRef", AST.context()).getReturnType()));
}

TEST_F(HasInferableTestWithSelectTemplatesOfPointersInferable,
       NestedSelectTemplatesAreNotInferable) {
  TestAST AST(R"cc(
    namespace std {
    template <typename T>
    class vector {};
    }  // namespace std

    std::vector<std::vector<int*>> Instance;
  )cc");

  EXPECT_FALSE(
      hasInferable(lookup<VarDecl>("Instance", AST.context()).getType()));
}

TEST_F(HasInferableTestWithSelectTemplatesOfPointersInferable,
       OtherTypesInStdAreNotSelectTemplates) {
  TestAST AST(R"cc(
    namespace std {
    template <typename T>
    class not_vector {};
    }  // namespace std

    std::not_vector<int*> Instance;
  )cc");

  EXPECT_FALSE(
      hasInferable(lookup<VarDecl>("Instance", AST.context()).getType()));
}

TEST_F(HasInferableTestWithSelectTemplatesOfPointersInferable,
       SelectTemplateOfNonPointerIsNotInferable) {
  TestAST AST(R"cc(
    namespace std {
    template <typename T>
    class vector {};
    }  // namespace std

    std::vector<int> Instance;
  )cc");

  EXPECT_FALSE(
      hasInferable(lookup<VarDecl>("Instance", AST.context()).getType()));
}

TEST_F(HasInferableTestWithSelectTemplatesOfPointersInferable,
       SelectTemplateOfPointerIsInferableWhenUsedThroughAlias) {
  TestAST AST(R"cc(
    namespace std {
    template <typename T>
    class vector {};
    }  // namespace std

    using VectorOfIntPtr = std::vector<int*>;

    VectorOfIntPtr Instance;
  )cc");

  EXPECT_TRUE(
      hasInferable(lookup<VarDecl>("Instance", AST.context()).getType()));
}

TEST_F(HasInferableTestWithSelectTemplatesOfPointersInferable,
       SelectTemplateOfPointerIsInferableWhenUsedThroughAliasTemplate) {
  TestAST AST(R"cc(
    namespace std {
    template <typename T>
    class vector {};
    }  // namespace std

    template <typename T>
    using VectorAliasTemplate = std::vector<T>;

    VectorAliasTemplate<int*> Instance;
  )cc");

  EXPECT_TRUE(
      hasInferable(lookup<VarDecl>("Instance", AST.context()).getType()));
}

TEST_F(HasInferableTestWithSelectTemplatesOfPointersInferable,
       SelectTemplateOfPointerIsInferableWhenUsedThroughUsingDecl) {
  TestAST AST(R"cc(
    namespace std {
    template <typename T>
    class vector {};
    }  // namespace std

    using std::vector;

    vector<int*> Instance;
  )cc");

  EXPECT_TRUE(
      hasInferable(lookup<VarDecl>("Instance", AST.context()).getType()));
}

// This is a reproduction of a crash seen on methods inside std::vector.
TEST_F(HasInferableTestWithSelectTemplatesOfPointersInferable,
       SelectTemplateOfPointerIsInferableWhenReferencedInsideTheTemplate) {
  TestAST AST(R"cc(
    namespace std {
    template <typename T>
    class vector {
     public:
      int method(vector);
    };
    }  // namespace std

    std::vector<int*> Instance;
    int MethodResult = Instance.method(Instance);
  )cc");

  EXPECT_TRUE(hasInferable(
      cast<CXXMemberCallExpr>(lookup<VarDecl>("MethodResult", AST.context())
                                  .getInit()
                                  ->IgnoreImplicit())
          ->getMethodDecl()
          ->getParamDecl(0)
          ->getType()));
}

}  // namespace
}  // namespace clang::tidy::nullability

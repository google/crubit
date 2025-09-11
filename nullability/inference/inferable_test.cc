// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/inferable.h"

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
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using ::clang::ast_matchers::anything;
using ::clang::ast_matchers::equalsNode;
using ::clang::ast_matchers::functionDecl;
using ::clang::ast_matchers::hasDeclContext;
using ::clang::ast_matchers::hasName;
using ::clang::ast_matchers::isImplicit;
using ::clang::ast_matchers::isTemplateInstantiation;
using ::clang::ast_matchers::match;
using ::clang::ast_matchers::namedDecl;
using ::clang::ast_matchers::selectFirst;
using ::clang::ast_matchers::unless;

template <class T = NamedDecl>
const T &lookup(llvm::StringRef Name, ASTContext &Ctx,
                const Decl *DeclContext = nullptr) {
  const auto &ContextMatcher =
      DeclContext ? hasDeclContext(equalsNode(DeclContext)) : anything();
  const auto &BoundNodes =
      match(namedDecl(hasName(Name), ContextMatcher, unless(isImplicit()))
                .bind("decl"),
            Ctx);
  const T *Match = nullptr;
  for (const auto &N : BoundNodes) {
    if (const auto *NAsT = N.getNodeAs<T>("decl")) {
      if (Match)
        ADD_FAILURE() << "Found more than one matching node for " << Name;
      Match = NAsT;
    }
  }
  EXPECT_NE(Match, nullptr) << Name;
  return *Match;
}

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

TEST(IsInferenceTargetTest, GlobalVariables) {
  TestAST AST((SmartPointerHeader + R"cc(
                int* Pointer;
                std::unique_ptr<int> StdSmartPointer;
                custom_smart_ptr<int> CustomSmartPointer;
                custom_smart_int_ptr CustomSmartIntPointer;
                int NotPointer;
              )cc")
                  .str());

  auto &Ctx = AST.context();
  EXPECT_TRUE(isInferenceTarget(lookup("Pointer", Ctx)));
  EXPECT_TRUE(isInferenceTarget(lookup("StdSmartPointer", Ctx)));
  EXPECT_TRUE(isInferenceTarget(lookup("CustomSmartPointer", Ctx)));
  EXPECT_TRUE(isInferenceTarget(lookup("CustomSmartIntPointer", Ctx)));
  EXPECT_FALSE(isInferenceTarget(lookup("NotPointer", Ctx)));
}

TEST(IsInferenceTargetTest, Functions) {
  TestAST AST((SmartPointerHeader + R"cc(
                int* func(int* Param, int** NestedParam,
                          std::unique_ptr<int> StdSmartParam,
                          custom_smart_ptr<int> CustomSmartParam) {
                  int* Local;
                  static int* StaticLocal;
                  std::unique_ptr<int> StdSmartLocal;
                  custom_smart_ptr<int> CustomSmartLocal;
                }
                void empty() {}
                auto Lambda = []() {};
                auto LambdaWithPtr = [](int*) {};
              )cc")
                  .str());

  auto &Ctx = AST.context();
  EXPECT_TRUE(isInferenceTarget(lookup("func", Ctx)));
  EXPECT_FALSE(isInferenceTarget(lookup("Param", Ctx)));
  EXPECT_FALSE(isInferenceTarget(lookup("NestedParam", Ctx)));
  EXPECT_FALSE(isInferenceTarget(lookup("StdSmartParam", Ctx)));
  EXPECT_FALSE(isInferenceTarget(lookup("CustomSmartParam", Ctx)));
  EXPECT_TRUE(isInferenceTarget(lookup("Local", Ctx)));
  EXPECT_TRUE(isInferenceTarget(lookup("StaticLocal", Ctx)));
  EXPECT_TRUE(isInferenceTarget(lookup("StdSmartLocal", Ctx)));
  EXPECT_TRUE(isInferenceTarget(lookup("CustomSmartLocal", Ctx)));
  EXPECT_FALSE(isInferenceTarget(lookup("empty", Ctx)));
  auto &Lambda = lookup<VarDecl>("Lambda", Ctx);
  auto *LambdaCtx = cast<LambdaExpr>(Lambda.getInit())->getLambdaClass();
  EXPECT_FALSE(isInferenceTarget(Lambda));
  EXPECT_FALSE(isInferenceTarget(lookup("operator()", Ctx, LambdaCtx)));
  auto &LambdaWithPtr = lookup<VarDecl>("LambdaWithPtr", Ctx);
  auto *LambdaWithPtrCtx =
      cast<LambdaExpr>(LambdaWithPtr.getInit())->getLambdaClass();
  EXPECT_FALSE(isInferenceTarget(LambdaWithPtr));
  EXPECT_TRUE(isInferenceTarget(lookup("operator()", Ctx, LambdaWithPtrCtx)));
}

TEST(IsInferenceTargetTest, ClassAndMembers) {
  TestAST AST((SmartPointerHeader + R"cc(
                class C {
                  void method();
                  int* methodWithPtr();
                  int NonPtrField;
                  int* PtrField;
                  static int* StaticField;
                  std::unique_ptr<int> StdSmartField;
                  custom_smart_ptr<int> CustomSmartField;
                };
              )cc")
                  .str());

  auto &Ctx = AST.context();
  EXPECT_FALSE(isInferenceTarget(lookup<CXXRecordDecl>("C", Ctx)));
  EXPECT_FALSE(isInferenceTarget(lookup("method", Ctx)));
  EXPECT_TRUE(isInferenceTarget(lookup("methodWithPtr", Ctx)));
  EXPECT_FALSE(isInferenceTarget(lookup("NonPtrField", Ctx)));
  EXPECT_TRUE(isInferenceTarget(lookup("PtrField", Ctx)));
  EXPECT_TRUE(isInferenceTarget(lookup("StaticField", Ctx)));
  EXPECT_TRUE(isInferenceTarget(lookup("StdSmartField", Ctx)));
  EXPECT_TRUE(isInferenceTarget(lookup("CustomSmartField", Ctx)));
}

TEST(IsInferenceTargetTest, FunctionTemplate) {
  TestAST AST(R"cc(
    template <int X>
    void funcTmpl(int*) {
      int* LocalInTmpl;

      struct StructInTmpl {
        void funcInStructInTmpl(int* P) { int* LocalInStructInTmpl; }
        int* FieldInStructInTmpl;
      };
    }

    auto& FuncTmplInst = funcTmpl<2>;
  )cc");

  auto &Ctx = AST.context();
  // A function template is not an inference target, nor are functions or fields
  // or local variables contained within.
  const FunctionTemplateDecl &FuncTmpl =
      lookup<FunctionTemplateDecl>("funcTmpl", Ctx);
  EXPECT_FALSE(isInferenceTarget(FuncTmpl));
  EXPECT_FALSE(isInferenceTarget(*FuncTmpl.getTemplatedDecl()));
  EXPECT_FALSE(isInferenceTarget(
      lookup("LocalInTmpl", Ctx, FuncTmpl.getTemplatedDecl())));
  auto *StructInTmpl =
      &lookup<CXXRecordDecl>("StructInTmpl", Ctx, FuncTmpl.getTemplatedDecl());
  EXPECT_FALSE(
      isInferenceTarget(lookup("FieldInStructInTmpl", Ctx, StructInTmpl)));
  auto &FuncInStructInTmpl = lookup("funcInStructInTmpl", Ctx, StructInTmpl);
  EXPECT_FALSE(isInferenceTarget(FuncInStructInTmpl));
  EXPECT_FALSE(isInferenceTarget(
      lookup("LocalInStructInTmpl", Ctx, &FuncInStructInTmpl)));
  // The function template instantiation *is* an inference target, as are
  // functions, fields, and local variables within.
  const ValueDecl &Instantiation =
      *cast<DeclRefExpr>(
           lookup<VarDecl>("FuncTmplInst", Ctx).getInit()->IgnoreImplicit())
           ->getDecl();
  EXPECT_TRUE(isInferenceTarget(Instantiation));
  EXPECT_TRUE(isInferenceTarget(lookup("LocalInTmpl", Ctx, &Instantiation)));
  auto *StructInInstantiation =
      &lookup<CXXRecordDecl>("StructInTmpl", Ctx, &Instantiation);
  EXPECT_TRUE(isInferenceTarget(
      lookup("FieldInStructInTmpl", Ctx, StructInInstantiation)));
  auto &FuncInStructInInstantiation =
      lookup("funcInStructInTmpl", Ctx, StructInInstantiation);
  EXPECT_TRUE(isInferenceTarget(FuncInStructInInstantiation));
  EXPECT_TRUE(isInferenceTarget(
      lookup("LocalInStructInTmpl", Ctx, &FuncInStructInInstantiation)));
}

TEST(IsInferenceTargetTest, ClassTemplateAndMembers) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {
      T NonPtrField;
      T* PtrField;
      static T* StaticField;
      T method();
      T* methodWithPtr();
      struct Nested {
        struct NestedTwo {
          T* NestedStructPtrField;
          bool* nestedStructMethod();
        };
        NestedTwo NestedStructTwo;
      };
      Nested NestedStruct;
    };

    ClassTemplate<int> IntClass;
    int IntA = IntClass.NonPtrField;
    int* IntB = IntClass.PtrField;
    int* IntC = IntClass.StaticField;
    int IntD = IntClass.method();
    int* IntE = IntClass.methodWithPtr();
    int* IntF = IntClass.NestedStruct.NestedStructTwo.NestedStructPtrField;
    bool* IntG = IntClass.NestedStruct.NestedStructTwo.nestedStructMethod();

    ClassTemplate<int*> IntStarClass;
    int* IntStarA = IntStarClass.NonPtrField;
    int** IntStarB = IntStarClass.PtrField;
    int** IntStarC = IntStarClass.StaticField;
    int* IntStarD = IntStarClass.method();
    int** IntStarE = IntStarClass.methodWithPtr();
    int** IntStarF = IntStarClass.NestedStruct.NestedStructTwo.NestedStructPtrField;
    bool* IntStarG = IntStarClass.NestedStruct.NestedStructTwo.nestedStructMethod();
  )cc");

  auto &Ctx = AST.context();
  // Class templates and fields and functions inside them are not inference
  // targets.
  auto &ClassTemplate = lookup<ClassTemplateDecl>("ClassTemplate", Ctx);
  EXPECT_FALSE(isInferenceTarget(ClassTemplate));
  EXPECT_FALSE(isInferenceTarget(*ClassTemplate.getTemplatedDecl()));
  EXPECT_FALSE(isInferenceTarget(
      lookup("NonPtrField", Ctx, ClassTemplate.getTemplatedDecl())));
  EXPECT_FALSE(isInferenceTarget(
      lookup("PtrField", Ctx, ClassTemplate.getTemplatedDecl())));
  EXPECT_FALSE(isInferenceTarget(
      lookup("StaticField", Ctx, ClassTemplate.getTemplatedDecl())));
  EXPECT_FALSE(isInferenceTarget(
      lookup("method", Ctx, ClassTemplate.getTemplatedDecl())));
  EXPECT_FALSE(isInferenceTarget(
      lookup("methodWithPtr", Ctx, ClassTemplate.getTemplatedDecl())));
  auto *NestedTwoInClassTemplate = &lookup<CXXRecordDecl>(
      "NestedTwo", Ctx,
      &lookup<CXXRecordDecl>("Nested", Ctx, ClassTemplate.getTemplatedDecl()));
  EXPECT_FALSE(isInferenceTarget(
      lookup("NestedStructPtrField", Ctx, NestedTwoInClassTemplate)));
  EXPECT_FALSE(isInferenceTarget(
      lookup("nestedStructMethod", Ctx, NestedTwoInClassTemplate)));

  // Class template instantiations are inference targets, as are fields
  // and functions inside, if they have inferable types, i.e. they are pointers
  // and the pointer-ness is specified by the template.
  // For the int template argument:
  EXPECT_FALSE(isInferenceTarget(
      *lookup<VarDecl>("IntClass", Ctx).getType()->getAsRecordDecl()));
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("IntA", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("IntB", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("IntC", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_FALSE(isInferenceTarget(
      *cast<CXXMemberCallExpr>(
           lookup<VarDecl>("IntD", Ctx).getInit()->IgnoreImplicit())
           ->getMethodDecl()));
  EXPECT_TRUE(isInferenceTarget(
      *cast<CXXMemberCallExpr>(
           lookup<VarDecl>("IntE", Ctx).getInit()->IgnoreImplicit())
           ->getMethodDecl()));
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("IntF", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_TRUE(isInferenceTarget(
      *cast<CXXMemberCallExpr>(
           lookup<VarDecl>("IntG", Ctx).getInit()->IgnoreImplicit())
           ->getMethodDecl()));

  // For the int* template argument, notably, we get the same results, even
  // though the canonical types of more fields are pointers:
  EXPECT_FALSE(isInferenceTarget(
      *lookup<VarDecl>("IntStarClass", Ctx).getType()->getAsRecordDecl()));
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("IntStarA", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("IntStarB", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("IntStarC", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_FALSE(isInferenceTarget(
      *cast<CXXMemberCallExpr>(
           lookup<VarDecl>("IntStarD", Ctx).getInit()->IgnoreImplicit())
           ->getMethodDecl()));
  EXPECT_TRUE(isInferenceTarget(
      *cast<CXXMemberCallExpr>(
           lookup<VarDecl>("IntStarE", Ctx).getInit()->IgnoreImplicit())
           ->getMethodDecl()));
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("IntStarF", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_TRUE(isInferenceTarget(
      *cast<CXXMemberCallExpr>(
           lookup<VarDecl>("IntStarG", Ctx).getInit()->IgnoreImplicit())
           ->getMethodDecl()));
}

TEST(InferableTest, CountInferableSlots) {
  TestAST AST((SmartPointerHeader + R"cc(
                using Pointer = int *;
                template <class T>
                struct S;
                struct T;

                void f1(int *);
                void f2(Pointer);
                void f3(int **);
                void f4(Pointer *);
                void f5(int *&);
                void f6(int (*)());  // function pointer
                void f7(std::unique_ptr<int>);
                void f8(custom_smart_ptr<int>);

                int *g1(int);
                Pointer g2(int);
                std::unique_ptr<int> g3(int);
                custom_smart_ptr<int> g4(int);

                void h1(S<int *>);
                void h2(int T::*);      // pointer to data member
                void h3(int (T::*)());  // pointer to member function
              )cc")
                  .str());
  auto &Ctx = AST.context();

  // All the 'f's have a single pointer arg.
  EXPECT_EQ(1, countInferableSlots(lookup("f1", Ctx)));
  EXPECT_EQ(1, countInferableSlots(lookup("f2", Ctx)));
  EXPECT_EQ(1, countInferableSlots(lookup("f3", Ctx)));
  EXPECT_EQ(1, countInferableSlots(lookup("f4", Ctx)));
  EXPECT_EQ(1, countInferableSlots(lookup("f5", Ctx)));
  EXPECT_EQ(1, countInferableSlots(lookup("f6", Ctx)));
  EXPECT_EQ(1, countInferableSlots(lookup("f7", Ctx)));
  EXPECT_EQ(1, countInferableSlots(lookup("f8", Ctx)));

  // All the 'g's have a pointer return.
  EXPECT_EQ(1, countInferableSlots(lookup("g1", Ctx)));
  EXPECT_EQ(1, countInferableSlots(lookup("g2", Ctx)));
  EXPECT_EQ(1, countInferableSlots(lookup("g3", Ctx)));
  EXPECT_EQ(1, countInferableSlots(lookup("g4", Ctx)));

  // The 'h's have types that aren't really pointers.
  EXPECT_EQ(0, countInferableSlots(lookup("h1", Ctx)));
  EXPECT_EQ(0, countInferableSlots(lookup("h2", Ctx)));
  EXPECT_EQ(0, countInferableSlots(lookup("h3", Ctx)));
}

TEST(InferableTest, InnerPointersNotInferable) {
  TestAST AST(R"cc(
    int*** ThreePointersOneInferable;
  )cc");
  auto &Ctx = AST.context();
  EXPECT_EQ(1, countInferableSlots(lookup("ThreePointersOneInferable", Ctx)));
}

TEST(InferableTest, TemplateArgumentPointersNotInferable) {
  TestAST AST(R"cc(
    template <typename T>
    struct S {};

    S<int*> NotInferable;
  )cc");
  auto &Ctx = AST.context();
  EXPECT_EQ(0, countInferableSlots(lookup("NotInferable", Ctx)));
  EXPECT_FALSE(isInferenceTarget(lookup("NotInferable", Ctx)));
}

TEST(InferableTest, TemplateInstantiationOnlyArgWithStarIsInferable) {
  TestAST AST((SmartPointerHeader + R"cc(
                template <typename T>
                struct S {
                  T* Inferable;
                  T NotInferable;
                  using U = T;
                  U NotInferableThroughAlias;
                };

                S<int*> AnS;
                int** FromTStarField = AnS.Inferable;
                int* FromTField = AnS.NotInferable;
                int* FromTFieldAlias = AnS.NotInferableThroughAlias;

                S<std::unique_ptr<int>> AnSSmart;
                std::unique_ptr<int>* FromTStarFieldSmart = AnSSmart.Inferable;
                std::unique_ptr<int>& FromTFieldSmart = AnSSmart.NotInferable;
                std::unique_ptr<int>& FromTFieldAliasSmart = AnSSmart.NotInferableThroughAlias;
              )cc")
                  .str());
  auto &Ctx = AST.context();
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("FromTStarField", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("FromTField", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("FromTFieldAlias", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(lookup<VarDecl>("FromTStarFieldSmart", Ctx)
                            .getInit()
                            ->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("FromTFieldSmart", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(lookup<VarDecl>("FromTFieldAliasSmart", Ctx)
                            .getInit()
                            ->IgnoreImplicit())
           ->getMemberDecl()));
}

TEST(InferableTest, TypeAliasTemplate) {
  TestAST AST(R"cc(
    template <typename T>
    using PtrAlias = T*;
    template <typename T>
    using Alias = T;

    template <typename U>
    struct S {
      Alias<U*> Inferable;
      Alias<U> NotInferable;
      PtrAlias<U> InferablePointerInAlias;
    };

    S<int*> AnS;
    int** FromInferable = AnS.Inferable;
    int* FromNotInferable = AnS.NotInferable;
    int** FromInferablePointerInAlias = AnS.InferablePointerInAlias;
  )cc");
  auto &Ctx = AST.context();
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("FromInferable", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("FromNotInferable", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(lookup<VarDecl>("FromInferablePointerInAlias", Ctx)
                            .getInit()
                            ->IgnoreImplicit())
           ->getMemberDecl()));
}

TEST(InferableTest, NestedTemplates) {
  TestAST AST(R"cc(
    template <typename T>
    struct S {
      template <typename U>
      struct Nested {
        U* InferableU;
        U NotInferableU;
        T* InferableT;
        T NotInferableT;
      };
      Nested<bool*> ANested;
    };

    S<int*> AnS;
    bool** FromInferableU = AnS.ANested.InferableU;
    bool* FromNotInferableU = AnS.ANested.NotInferableU;
    int** FromInferableT = AnS.ANested.InferableT;
    int* FromNotInferableT = AnS.ANested.NotInferableT;
  )cc");
  auto &Ctx = AST.context();
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("FromInferableU", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(lookup<VarDecl>("FromNotInferableU", Ctx)
                            .getInit()
                            ->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_TRUE(isInferenceTarget(
      *cast<MemberExpr>(
           lookup<VarDecl>("FromInferableT", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(lookup<VarDecl>("FromNotInferableT", Ctx)
                            .getInit()
                            ->IgnoreImplicit())
           ->getMemberDecl()));
}

TEST(InferableTest, TemplateParamTypedDeclsInFunctionTemplate) {
  TestAST AST(R"cc(
    template <typename T>
    void func(T* ParamStar, T Param) {
      T* LocalStar;
      T Local;
    }

    template <typename T>
    T* returnStar() {}

    template <typename T>
    T returnNoStar() {}

    void instantiate() {
      func<int*>(nullptr, nullptr);
      returnStar<bool*>();
      returnNoStar<char*>();
    }
  )cc");

  auto& Ctx = AST.context();

  {
    const FunctionDecl* FuncInstantiation = selectFirst<FunctionDecl>(
        "func_instantiation",
        match(functionDecl(isTemplateInstantiation(), hasName("func"))
                  .bind("func_instantiation"),
              Ctx));
    ASSERT_NE(FuncInstantiation, nullptr);
    EXPECT_TRUE(hasInferable(FuncInstantiation->getParamDecl(0)->getType()));
    EXPECT_FALSE(hasInferable(FuncInstantiation->getParamDecl(1)->getType()));

    const VarDecl& LocalStar =
        lookup<VarDecl>("LocalStar", Ctx, FuncInstantiation);
    EXPECT_TRUE(hasInferable(LocalStar.getType()));
    const VarDecl& Local = lookup<VarDecl>("Local", Ctx, FuncInstantiation);
    EXPECT_FALSE(hasInferable(Local.getType()));
  }

  {
    const FunctionDecl* ReturnStarInstantiation = selectFirst<FunctionDecl>(
        "returnStar_instantiation",
        match(functionDecl(isTemplateInstantiation(), hasName("returnStar"))
                  .bind("returnStar_instantiation"),
              Ctx));
    ASSERT_NE(ReturnStarInstantiation, nullptr);
    EXPECT_TRUE(hasInferable(ReturnStarInstantiation->getReturnType()));
  }

  {
    const FunctionDecl* ReturnNoStarInstantiation = selectFirst<FunctionDecl>(
        "returnNoStar_instantiation",
        match(functionDecl(isTemplateInstantiation(), hasName("returnNoStar"))
                  .bind("returnNoStar_instantiation"),
              Ctx));
    ASSERT_NE(ReturnNoStarInstantiation, nullptr);
    EXPECT_FALSE(hasInferable(ReturnNoStarInstantiation->getReturnType()));
  }
}

TEST(InferableTest, TypeInsideTemplateTypeParamIsNotInferable) {
  TestAST AST(R"cc(
    template <class T>
    // Being named Ptr does not guarantee that the return type will always be a
    // pointer. So, we still don't annotate unless the pointer-ness is part of
    // the template declaration.
    T::Ptr ReturnsTypeNamedPtr();
    template <class T>
    T::Ptr* ReturnsPtr();

    class S {
     public:
      typedef const char* Ptr;
    };

    int main() {
      ReturnsTypeNamedPtr<S>();
      ReturnsPtr<S>();
    }
  )cc");

  {
    const FunctionDecl* ReturnsTypeNamedPtr = selectFirst<FunctionDecl>(
        "func", match(functionDecl(isTemplateInstantiation(),
                                   hasName("ReturnsTypeNamedPtr"))
                          .bind("func"),
                      AST.context()));
    ASSERT_NE(ReturnsTypeNamedPtr, nullptr);
    EXPECT_FALSE(hasInferable(ReturnsTypeNamedPtr->getReturnType()));
  }

  {
    const FunctionDecl* ReturnsPtr = selectFirst<FunctionDecl>(
        "func",
        match(functionDecl(isTemplateInstantiation(), hasName("ReturnsPtr"))
                  .bind("func"),
              AST.context()));
    ASSERT_NE(ReturnsPtr, nullptr);
    EXPECT_TRUE(hasInferable(ReturnsPtr->getReturnType()));
  }
}

}  // namespace
}  // namespace clang::tidy::nullability

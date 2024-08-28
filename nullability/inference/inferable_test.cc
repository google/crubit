// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/inferable.h"

#include "nullability/type_nullability.h"
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
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using ::clang::ast_matchers::anything;
using ::clang::ast_matchers::equalsNode;
using ::clang::ast_matchers::hasDeclContext;
using ::clang::ast_matchers::hasName;
using ::clang::ast_matchers::isImplicit;
using ::clang::ast_matchers::match;
using ::clang::ast_matchers::namedDecl;
using ::clang::ast_matchers::unless;

test::EnableSmartPointers Enable;

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
  struct custom_smart_ptr {
    using absl_nullability_compatible = void;
    using pointer = T*;
  };
)cc";

TEST(IsInferenceTargetTest, GlobalVariables) {
  TestAST AST((SmartPointerHeader + R"cc(
                int* Pointer;
                std::unique_ptr<int> StdSmartPointer;
                custom_smart_ptr<int> CustomSmartPointer;
                int NotPointer;
              )cc")
                  .str());

  auto &Ctx = AST.context();
  EXPECT_TRUE(isInferenceTarget(lookup("Pointer", Ctx)));
  EXPECT_TRUE(isInferenceTarget(lookup("StdSmartPointer", Ctx)));
  EXPECT_TRUE(isInferenceTarget(lookup("CustomSmartPointer", Ctx)));
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
    }

    auto& FuncTmplSpec = funcTmpl<2>;
  )cc");

  auto &Ctx = AST.context();
  // A function template is not an inference target, nor are local variables
  // contained within.
  const FunctionTemplateDecl &FuncTmpl =
      lookup<FunctionTemplateDecl>("funcTmpl", Ctx);
  EXPECT_FALSE(isInferenceTarget(FuncTmpl));
  EXPECT_FALSE(isInferenceTarget(*FuncTmpl.getTemplatedDecl()));
  EXPECT_FALSE(isInferenceTarget(
      lookup("LocalInTmpl", Ctx, FuncTmpl.getTemplatedDecl())));
  // The function template specialization is *also* not an inference target, nor
  // are local variables contained within.
  const ValueDecl &Specialization =
      *cast<DeclRefExpr>(
           lookup<VarDecl>("FuncTmplSpec", Ctx).getInit()->IgnoreImplicit())
           ->getDecl();
  EXPECT_FALSE(isInferenceTarget(Specialization));
  EXPECT_FALSE(isInferenceTarget(
      lookup("LocalInTmpl", Ctx, cast<FunctionDecl>(&Specialization))));
}

TEST(IsInferenceTargetTest, ClassTemplateAndMembers) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {
      T NonPtrField;
      T* PtrField;
      static T* StaticField;
    };

    ClassTemplate<int> I;
    int A = I.NonPtrField;
    int* B = I.PtrField;
    int* C = I.StaticField;
  )cc");

  auto &Ctx = AST.context();
  // Class templates and their fields are not inference targets.
  auto &ClassTemplate = lookup<ClassTemplateDecl>("ClassTemplate", Ctx);
  EXPECT_FALSE(isInferenceTarget(ClassTemplate));
  EXPECT_FALSE(isInferenceTarget(*ClassTemplate.getTemplatedDecl()));
  EXPECT_FALSE(isInferenceTarget(
      lookup("NonPtrField", Ctx, ClassTemplate.getTemplatedDecl())));
  EXPECT_FALSE(isInferenceTarget(
      lookup("PtrField", Ctx, ClassTemplate.getTemplatedDecl())));
  EXPECT_FALSE(isInferenceTarget(
      lookup("StaticField", Ctx, ClassTemplate.getTemplatedDecl())));

  // Class template specializations and their fields are also not inference
  // targets.
  EXPECT_FALSE(isInferenceTarget(
      *lookup<VarDecl>("I", Ctx).getType()->getAsRecordDecl()));
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(lookup<VarDecl>("A", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(lookup<VarDecl>("B", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(lookup<VarDecl>("C", Ctx).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
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

}  // namespace
}  // namespace clang::tidy::nullability

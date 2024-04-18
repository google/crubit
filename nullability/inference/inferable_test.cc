// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/inferable.h"

#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Expr.h"
#include "clang/Basic/LLVM.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/StringRef.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

template <class T = NamedDecl>
const T &lookup(llvm::StringRef Name, const DeclContext &DC) {
  auto L = DC.lookup(&DC.getParentASTContext().Idents.get(Name));
  EXPECT_TRUE(L.isSingleResult()) << Name;
  auto *Result = L.find_first<T>();
  EXPECT_NE(Result, nullptr) << Name;
  return *Result;
}

TEST(IsInferenceTargetTest, GlobalVariables) {
  TestAST AST(R"cc(
    int* Pointer;
    int NotPointer;
  )cc");

  auto &TU = *AST.context().getTranslationUnitDecl();
  EXPECT_FALSE(isInferenceTarget(lookup("Pointer", TU)));
  EXPECT_FALSE(isInferenceTarget(lookup("NotPointer", TU)));
}

TEST(IsInferenceTargetTest, Functions) {
  TestAST AST(R"cc(
    int* func(int*, int**);
    void empty() {}
  )cc");

  auto &TU = *AST.context().getTranslationUnitDecl();
  EXPECT_TRUE(isInferenceTarget(lookup("func", TU)));
  EXPECT_TRUE(isInferenceTarget(lookup("empty", TU)));
}

TEST(IsInferenceTargetTest, ClassAndMembers) {
  TestAST AST(R"cc(
    class C {
      void method();
      int NonPtrField;
      int* PtrField;
    };
  )cc");

  auto &TU = *AST.context().getTranslationUnitDecl();
  auto &Class = lookup<CXXRecordDecl>("C", TU);
  EXPECT_FALSE(isInferenceTarget(Class));
  EXPECT_TRUE(isInferenceTarget(lookup("method", Class)));
  EXPECT_TRUE(isInferenceTarget(lookup("PtrField", Class)));
  EXPECT_FALSE(isInferenceTarget(lookup("NonPtrField", Class)));
}

TEST(IsInferenceTargetTest, FunctionTemplate) {
  TestAST AST(R"cc(
    template <int X>
    void funcTmpl(int*) {}

    auto& FuncTmplSpec = funcTmpl<2>;
  )cc");

  auto &TU = *AST.context().getTranslationUnitDecl();
  // A function template is not an inference target.
  const FunctionTemplateDecl &FuncTmpl =
      lookup<FunctionTemplateDecl>("funcTmpl", TU);
  EXPECT_FALSE(isInferenceTarget(FuncTmpl));
  EXPECT_FALSE(isInferenceTarget(*FuncTmpl.getTemplatedDecl()));
  // The template specialization is *also* not an inference target.
  const VarDecl &FuncTmplSpec = lookup<VarDecl>("FuncTmplSpec", TU);
  EXPECT_FALSE(isInferenceTarget(FuncTmplSpec));
  const ValueDecl &FuncTmpl2 =
      *cast<DeclRefExpr>(FuncTmplSpec.getInit()->IgnoreImplicit())->getDecl();
  EXPECT_FALSE(isInferenceTarget(FuncTmpl2));
}

TEST(IsInferenceTargetTest, ClassTemplateAndMembers) {
  TestAST AST(R"cc(
    template <typename T>
    struct ClassTemplate {
      T NonPtrField;
      T* PtrField;
    };

    ClassTemplate<int> I;
    int A = I.NonPtrField;
    int* B = I.PtrField;
  )cc");

  auto &TU = *AST.context().getTranslationUnitDecl();
  // Class templates and their fields are not inference targets.
  auto &ClassTemplate = lookup<ClassTemplateDecl>("ClassTemplate", TU);
  EXPECT_FALSE(isInferenceTarget(ClassTemplate));
  EXPECT_FALSE(isInferenceTarget(*ClassTemplate.getTemplatedDecl()));
  EXPECT_FALSE(isInferenceTarget(
      lookup("NonPtrField", *ClassTemplate.getTemplatedDecl())));
  EXPECT_FALSE(
      isInferenceTarget(lookup("PtrField", *ClassTemplate.getTemplatedDecl())));

  // Class template specializations and their fields are also not inference
  // targets.
  EXPECT_FALSE(isInferenceTarget(
      *lookup<VarDecl>("I", TU).getType()->getAsRecordDecl()));
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(lookup<VarDecl>("A", TU).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
  EXPECT_FALSE(isInferenceTarget(
      *cast<MemberExpr>(lookup<VarDecl>("B", TU).getInit()->IgnoreImplicit())
           ->getMemberDecl()));
}

TEST(InferableTest, CountInferableSlots) {
  TestAST AST(R"cc(
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

    int *g1(int);
    Pointer g2(int);

    void h1(S<int *>);
    void h2(int T::*);      // pointer to data member
    void h3(int (T::*)());  // pointer to member function
  )cc");
  auto &TU = *AST.context().getTranslationUnitDecl();

  // All the 'f's have a single pointer arg.
  EXPECT_EQ(1, countInferableSlots(lookup("f1", TU)));
  EXPECT_EQ(1, countInferableSlots(lookup("f2", TU)));
  EXPECT_EQ(1, countInferableSlots(lookup("f3", TU)));
  EXPECT_EQ(1, countInferableSlots(lookup("f4", TU)));
  EXPECT_EQ(1, countInferableSlots(lookup("f5", TU)));
  EXPECT_EQ(1, countInferableSlots(lookup("f6", TU)));

  // All the 'g's have a pointer return.
  EXPECT_EQ(1, countInferableSlots(lookup("g1", TU)));
  EXPECT_EQ(1, countInferableSlots(lookup("g2", TU)));

  // The 'h's have types that aren't really pointers.
  EXPECT_EQ(0, countInferableSlots(lookup("h1", TU)));
  EXPECT_EQ(0, countInferableSlots(lookup("h2", TU)));
  EXPECT_EQ(0, countInferableSlots(lookup("h3", TU)));
}

}  // namespace
}  // namespace clang::tidy::nullability

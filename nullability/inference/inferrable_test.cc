// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/inferrable.h"

#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
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

TEST(InferrableTest, IsInferenceTarget) {
  TestAST AST(R"cc(
    int* Pointer;
    int* func(int*, int**);
    void empty() {}

    class Cls {
      void method();
    };

    template <int X>
    void funcTmpl(int*) {}

    auto& FuncTmplSpec = funcTmpl<2>;
  )cc");

  auto &TU = *AST.context().getTranslationUnitDecl();
  EXPECT_FALSE(isInferenceTarget(lookup("Pointer", TU)));
  EXPECT_TRUE(isInferenceTarget(lookup("func", TU)));
  EXPECT_TRUE(isInferenceTarget(lookup("empty", TU)));

  auto &Cls = lookup<CXXRecordDecl>("Cls", TU);
  EXPECT_FALSE(isInferenceTarget(Cls));
  EXPECT_TRUE(isInferenceTarget(lookup("method", Cls)));

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

TEST(InferrableTest, CountInferrableSlots) {
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
  EXPECT_EQ(1, countInferrableSlots(lookup("f1", TU)));
  EXPECT_EQ(1, countInferrableSlots(lookup("f2", TU)));
  EXPECT_EQ(1, countInferrableSlots(lookup("f3", TU)));
  EXPECT_EQ(1, countInferrableSlots(lookup("f4", TU)));
  EXPECT_EQ(1, countInferrableSlots(lookup("f5", TU)));
  EXPECT_EQ(1, countInferrableSlots(lookup("f6", TU)));

  // All the 'g's have a pointer return.
  EXPECT_EQ(1, countInferrableSlots(lookup("g1", TU)));
  EXPECT_EQ(1, countInferrableSlots(lookup("g2", TU)));

  // The 'h's have types that aren't really pointers.
  EXPECT_EQ(0, countInferrableSlots(lookup("h1", TU)));
  EXPECT_EQ(0, countInferrableSlots(lookup("h2", TU)));
  EXPECT_EQ(0, countInferrableSlots(lookup("h3", TU)));
}

}  // namespace
}  // namespace clang::tidy::nullability

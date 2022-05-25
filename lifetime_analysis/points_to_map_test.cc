// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/points_to_map.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/test/run_on_code.h"
#include "clang/AST/ASTContext.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

const clang::CallExpr* getFirstCallExpr(const clang::ASTContext& ast_context) {
  using clang::ast_matchers::callExpr;
  using clang::ast_matchers::match;
  using clang::ast_matchers::selectFirst;

  return selectFirst<clang::CallExpr>(
      "call", match(callExpr().bind("call"),
                    const_cast<clang::ASTContext&>(ast_context)));
}

TEST(PointsToMapTest, Equality) {
  runOnCodeWithLifetimeHandlers(
      "int *return_int_ptr();"
      "int* p = return_int_ptr();",
      [](const clang::ASTContext& ast_context,
         const LifetimeAnnotationContext&) {
        Object p1 = Object::Create(Lifetime::CreateLocal(), ast_context.IntTy);
        Object p2 = Object::Create(Lifetime::CreateLocal(), ast_context.IntTy);
        Object p3 = Object::Create(Lifetime::CreateLocal(), ast_context.IntTy);
        const clang::CallExpr* expr = getFirstCallExpr(ast_context);

        {
          PointsToMap map1, map2;
          map1.SetPointerPointsToSet(p1, {p2});
          map2.SetPointerPointsToSet(p1, {p3});
          EXPECT_EQ(map1, PointsToMap(map1));
          EXPECT_NE(map1, PointsToMap());
          EXPECT_NE(map1, map2);
        }

        {
          PointsToMap map1, map2;
          map1.SetExprObjectSet(expr, {p1});
          map2.SetExprObjectSet(expr, {p2});
          EXPECT_EQ(map1, PointsToMap(map1));
          EXPECT_NE(map1, PointsToMap());
          EXPECT_NE(map1, map2);
        }
      },
      {});
}

TEST(PointsToMapTest, Union) {
  runOnCodeWithLifetimeHandlers(
      "int *return_int_ptr();"
      "int* p = return_int_ptr();",
      [](const clang::ASTContext& ast_context,
         const LifetimeAnnotationContext&) {
        Object p1 = Object::Create(Lifetime::CreateLocal(), ast_context.IntTy);
        Object p2 = Object::Create(Lifetime::CreateLocal(), ast_context.IntTy);
        Object p3 = Object::Create(Lifetime::CreateLocal(), ast_context.IntTy);
        const clang::CallExpr* expr = getFirstCallExpr(ast_context);

        PointsToMap map1, map2;
        map1.SetPointerPointsToSet(p1, {p2});
        map2.SetPointerPointsToSet(p1, {p3});

        map1.SetExprObjectSet(expr, {p2});
        map2.SetExprObjectSet(expr, {p3});

        PointsToMap union_map = map1.Union(map2);

        EXPECT_EQ(union_map.GetPointerPointsToSet(p1), ObjectSet({p2, p3}));
        EXPECT_EQ(union_map.GetExprObjectSet(expr), ObjectSet({p2, p3}));
      },
      {});
}

TEST(PointsToMapTest, GetPointerPointsToSet) {
  runOnCodeWithLifetimeHandlers(
      "",
      [](const clang::ASTContext& ast_context,
         const LifetimeAnnotationContext&) {
        Object p1 = Object::Create(Lifetime::CreateLocal(), ast_context.IntTy);
        Object p2 = Object::Create(Lifetime::CreateLocal(), ast_context.IntTy);
        Object p3 = Object::Create(Lifetime::CreateLocal(), ast_context.IntTy);
        Object p4 = Object::Create(Lifetime::CreateLocal(), ast_context.IntTy);

        PointsToMap map;

        EXPECT_EQ(map.GetPointerPointsToSet(p1), ObjectSet());

        map.SetPointerPointsToSet(p1, {p3});
        map.SetPointerPointsToSet(p2, {p4});

        EXPECT_EQ(map.GetPointerPointsToSet(p1), ObjectSet({p3}));
        EXPECT_EQ(map.GetPointerPointsToSet({p1, p2}), ObjectSet({p3, p4}));
      },
      {});
}

TEST(PointsToMapTest, ExtendPointerPointsToSet) {
  runOnCodeWithLifetimeHandlers(
      "",
      [](const clang::ASTContext& ast_context,
         const LifetimeAnnotationContext&) {
        Object p1 = Object::Create(Lifetime::CreateLocal(), ast_context.IntTy);
        Object p2 = Object::Create(Lifetime::CreateLocal(), ast_context.IntTy);
        Object p3 = Object::Create(Lifetime::CreateLocal(), ast_context.IntTy);

        PointsToMap map;

        EXPECT_EQ(map.GetPointerPointsToSet(p1), ObjectSet());

        map.ExtendPointerPointsToSet(p1, {p2});

        EXPECT_EQ(map.GetPointerPointsToSet(p1), ObjectSet({p2}));

        map.ExtendPointerPointsToSet(p1, {p3});

        EXPECT_EQ(map.GetPointerPointsToSet(p1), ObjectSet({p2, p3}));
      },
      {});
}

TEST(PointsToMapTest, GetExprObjectSet) {
  runOnCodeWithLifetimeHandlers(
      "int *return_int_ptr();"
      "int* p = return_int_ptr();",
      [](const clang::ASTContext& ast_context,
         const LifetimeAnnotationContext&) {
        Object p1 = Object::Create(Lifetime::CreateLocal(), ast_context.IntTy);
        const clang::CallExpr* expr = getFirstCallExpr(ast_context);

        PointsToMap map;

        map.SetExprObjectSet(expr, {p1});
        EXPECT_EQ(map.GetExprObjectSet(expr), ObjectSet({p1}));
      },
      {});
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

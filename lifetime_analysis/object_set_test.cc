// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/object_set.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/object.h"
#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/lifetime_annotations.h"
#include "lifetime_annotations/test/run_on_code.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

using testing::UnorderedElementsAre;

TEST(ObjectSet, AccessObjects) {
  runOnCodeWithLifetimeHandlers(
      "",
      [](const clang::ASTContext& ast_context,
         const LifetimeAnnotationContext&) {
        Object object_static(Lifetime::Static(), ast_context.IntTy);
        ObjectSet object_set = {&object_static};

        EXPECT_THAT(object_set, UnorderedElementsAre(object_static));
      },
      {});
}

TEST(ObjectSet, Contains) {
  runOnCodeWithLifetimeHandlers(
      "",
      [](const clang::ASTContext& ast_context,
         const LifetimeAnnotationContext&) {
        Object o1(Lifetime::CreateLocal(), ast_context.IntTy);
        Object o2(Lifetime::CreateLocal(), ast_context.IntTy);

        EXPECT_TRUE(ObjectSet({&o1, &o2}).Contains(&o1));
        EXPECT_TRUE(ObjectSet({&o1, &o2}).Contains(&o2));
        EXPECT_FALSE(ObjectSet({&o1}).Contains(&o2));

        EXPECT_TRUE(ObjectSet({&o1, &o2}).Contains(ObjectSet()));
        EXPECT_TRUE(ObjectSet({&o1, &o2}).Contains(ObjectSet{&o1}));
        EXPECT_TRUE(ObjectSet({&o1, &o2}).Contains(ObjectSet{&o2}));
        EXPECT_TRUE(ObjectSet({&o1, &o2}).Contains({&o1, &o2}));
        EXPECT_TRUE(ObjectSet({&o1}).Contains(ObjectSet{&o1}));
        EXPECT_FALSE(ObjectSet({&o1}).Contains(ObjectSet{&o2}));
        EXPECT_TRUE(ObjectSet().Contains(ObjectSet()));
      },
      {});
}

TEST(ObjectSet, Union) {
  runOnCodeWithLifetimeHandlers(
      "",
      [](const clang::ASTContext& ast_context,
         const LifetimeAnnotationContext&) {
        Object object_static(Lifetime::Static(), ast_context.IntTy);
        ObjectSet set_1 = {&object_static};
        Object object_local(Lifetime::CreateLocal(), ast_context.IntTy);
        ObjectSet set_2 = {&object_local};

        ObjectSet set_union = set_1.Union(set_2);

        EXPECT_THAT(set_union,
                    UnorderedElementsAre(object_static, object_local));
      },
      {});
}

TEST(ObjectSet, Add) {
  runOnCodeWithLifetimeHandlers(
      "",
      [](const clang::ASTContext& ast_context,
         const LifetimeAnnotationContext&) {
        Object o1(Lifetime::CreateLocal(), ast_context.IntTy);
        Object o2(Lifetime::CreateLocal(), ast_context.IntTy);
        Object o3(Lifetime::CreateLocal(), ast_context.IntTy);

        {
          ObjectSet object_set = {&o1};
          object_set.Add(&o2);
          EXPECT_THAT(object_set, UnorderedElementsAre(o1, o2));
        }
        {
          ObjectSet object_set = {&o1, &o2};
          object_set.Add(&o2);
          EXPECT_THAT(object_set, UnorderedElementsAre(o1, o2));
        }
        {
          ObjectSet object_set = {&o1};
          object_set.Add({&o2, &o3});
          EXPECT_THAT(object_set, UnorderedElementsAre(o1, o2, o3));
        }
        {
          ObjectSet object_set = {&o1, &o2};
          object_set.Add({&o2, &o3});
          EXPECT_THAT(object_set, UnorderedElementsAre(o1, o2, o3));
        }
      },
      {});
}

TEST(ObjectSet, Equality) {
  runOnCodeWithLifetimeHandlers(
      "",
      [](const clang::ASTContext& ast_context,
         const LifetimeAnnotationContext&) {
        Object object_static(Lifetime::Static(), ast_context.IntTy);
        Object object_local(Lifetime::CreateLocal(), ast_context.IntTy);
        ObjectSet set_1 = {&object_static};
        ObjectSet set_2 = {&object_static};
        ObjectSet set_3 = {&object_static, &object_local};

        EXPECT_EQ(set_1, set_2);
        EXPECT_NE(set_1, set_3);
      },
      {});
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests involving function templates.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST_F(LifetimeAnalysisTest, FunctionTemplatePtr) {
  EXPECT_THAT(GetLifetimesWithPlaceholder(R"(
    template <typename T>
    T* target(T* t) {
      return t;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, FunctionTemplatePtrWithTwoArgs) {
  EXPECT_THAT(GetLifetimesWithPlaceholder(R"(
    template <typename T, typename U>
    T* target(T* t, U* u1, U& u2) {
      u1 = &u2;
      return t;
    }
  )"),
              LifetimesAre({{"target", "a, b, c -> a"}}));
}

TEST_F(LifetimeAnalysisTest, FunctionTemplatePtrWithTemplatedStruct) {
  EXPECT_THAT(GetLifetimesWithPlaceholder(R"(
    template <typename T>
    struct S {
      T t;
    };

    template <typename T>
    T* target(S<T*>* s) {
      return s->t;
    }
  )"),
              LifetimesAre({{"target", "(a, b) -> a"}}));
}

TEST_F(LifetimeAnalysisTest, FunctionTemplatePtrWithMultipleFunctions) {
  // The code has both template and non-template functions/code.
  EXPECT_THAT(GetLifetimesWithPlaceholder(R"(
    static int x = 3;
    template <typename T>
    struct A {
      T x;
      T y;
    };
    template <typename T>
    T* target(T* t) {
      return t;
    }
    template <typename U>
    U* target2(U* u) {
      return u;
    }
    int foo(A<int>* a) {
      return a->x + a->y + x;
    }
  )"),
              LifetimesAre(
                  {{"target", "a -> a"}, {"target2", "a -> a"}, {"foo", "a"}}));
}

TEST_F(LifetimeAnalysisTest, FunctionTemplateCall) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    T* t(T* a, T* b) {
      if (*a > *b) {
        return a;
      }
      return b;
    }
    int* target(int* a, int* b) {
      return t(a, b);
    }
  )"),
              LifetimesContain({{"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, FunctionTemplateCallIgnoreArg) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    T* t(T* a, T* b) {
      return a;
    }
    int* target(int* a, int* b) {
      return t(a, b);
    }
  )"),
              LifetimesContain({{"target", "a, b -> a"}}));
}

TEST_F(LifetimeAnalysisTest, FunctionTemplateCallPtrInstantiation) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    T* t(T* a, T* b) {
      if (*a > *b) {
        return a;
      }
      return b;
    }
    int** target(int** a, int** b) {
      return t(a, b);
    }
  )"),
              LifetimesContain({{"target", "(a, b), (a, b) -> (a, b)"}}));
}

TEST_F(LifetimeAnalysisTest, FunctionTemplateCallIgnoreArgPtrInstantiation) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    T* t(T* a, T* b) {
      return a;
    }
    int** target(int** a, int** b) {
      return t(a, b);
    }
  )"),
              LifetimesContain({{"target", "(a, b), (c, d) -> (a, b)"}}));
}

TEST_F(LifetimeAnalysisTest, FunctionTemplateInsideClassTemplate) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      template <typename U>
      U f(T t, U u) {
        return u;
      }
    };
    int* target(S<int *>& s, int* p1, int* p2) {
      return s.f(p1, p2);
    }
  )"),
              LifetimesContain({{"target", "(a, b), c, d -> d"}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

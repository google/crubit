// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests that control flow is taken into account correctly.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST_F(LifetimeAnalysisTest, ReturnArgumentWithControlFlow) {
  EXPECT_THAT(GetLifetimes(R"(
    int* get_lesser_of(int* a, int* b) {
      if (*a < *b) {
        return a;
      }
      return b;
    }
  )"),
              LifetimesAre({{"get_lesser_of", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnPtrArgumentWithConditionalOperator) {
  EXPECT_THAT(GetLifetimes(R"(
    int* get_lesser_of(int* a, int* b) {
      return *a < *b? a : b;
    }
  )"),
              LifetimesAre({{"get_lesser_of", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnRefArgumentWithConditionalOperator) {
  EXPECT_THAT(GetLifetimes(R"(
    int& get_lesser_of(int& a, int& b) {
      return a < b? a : b;
    }
  )"),
              LifetimesAre({{"get_lesser_of", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ControlFlowExceptionsWithUnconditionalThrow) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* a, int* b) {
      try {
        throw 42;
        return a;
      } catch(...) {
        return b;
      }
    }
  )"),
              LifetimesAre({{"target", "a, b -> b"}}));
}

TEST_F(LifetimeAnalysisTest, ControlFlowExceptionsWithUnknownControlFlow) {
  EXPECT_THAT(GetLifetimes(R"(
    void may_throw() {
      throw 42;
    }
    int* target(int* a, int* b) {
      try {
        may_throw();
        return a;
      } catch(...) {
        return b;
      }
    }
  )"),
              LifetimesContain({{"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, DoublePointerWithConditionalAssignment) {
  // This is a regression test for a bug where we were not taking all
  // substitutions into account in the return value lifetimes.

  EXPECT_THAT(GetLifetimes(R"(
    int** target(int** pp1, int** pp2) {
      if (**pp1 > **pp2) {
        *pp1 = *pp2;
      }
      return pp2;
    }
  )"),
              LifetimesAre({{"target", "(a, b), (a, c) -> (a, c)"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnArgumentWithControlFlowAndJoin) {
  EXPECT_THAT(GetLifetimes(R"(
    int* get_lesser_of(int* a, int* b) {
      int* p = a;
      if (*a < *b) {
        p = b;
      }
      return p;
    }
  )"),
              LifetimesAre({{"get_lesser_of", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnArgumentWithUnnecessaryAssignment) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* p, int* a, int* b) {
      for (int i=0; i<*a; i++) {
        p = a;
        p = b;
      }
      return p;
    }
  )"),
              LifetimesAre({{"target", "a, b, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, TakeAReferenceInControlFlow) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* p, int* a) {
      int local = 42;
      int** pp = &a;
      if (*a < *p) {
        pp = &p;
      }
      p = &local;
      return *pp;
    }
  )"),
              LifetimesAre({{"target",
                             "ERROR: function returns reference to a local"}}));
}

TEST_F(LifetimeAnalysisTest, TakeAReferenceEndOfBlock) {
  // Make sure that the analysis handles statement ordering correctly.

  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* a) {
      int local = 42;
      int** pp = &a;
      int* b = a;
      int* p = a;
      if (*p < *a) {
        p = b;
        pp = &p;
        b = &local;
      }
      return *pp;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, TakeAReferenceSneaky) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* p, int* a) {
      int local = 42;
      int** pp = &a;
      int* b = a;
      for (int i=0; i<*a; i++) {
        p = b;
        pp = &p;
        b = &local;
      }
      return *pp;
    }
  )"),
              LifetimesAre({{"target",
                             "ERROR: function returns reference to a local"}}));
}

TEST_F(LifetimeAnalysisTest, TakeAReferenceSneakyParam) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* p, int* a, int* c) {
      int** pp = &a;
      int* b = a;
      for (int i=0; i<*a; i++) {
        p = b;
        pp = &p;
        b = c;
      }
      return *pp;
    }
  )"),
              LifetimesAre({{"target", "a, a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, TakeAReferenceAndOverwrite) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* p, int* a, int* b) {
      int** pp = &a;
      if (*a < *p) {
        pp = &p;
      }
      p = b;
      return *pp;
    }
  )"),
              LifetimesAre({{"target", "a, b, b -> b"}}));
}

TEST_F(LifetimeAnalysisTest, TakeAReferenceTooStrict) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* p, int* a) {
      int** pp = &a;
      if (*p < *a) {
        p = a;
        pp = &p;
      }
      return *pp;
    }
  )"),
              LifetimesAre({{"target", "a, a -> a"}}));
  // TODO(mboehme): This result is too strict. This is because at the
  // return statement, the analysis concludes that
  // - pp may be pointing at either p or a, and
  // - p may either still have its original value or it may be pointing at a
  // The analysis doesn't "know" that the combination "p has original value and
  // pp points at p" can never occur. It may be possible to solve this with path
  // conditions -- IIUC, this is exactly what they are for.
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

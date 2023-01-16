// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for propagating pointees through function calls.
//
// Not every test that contains a function call should go here -- just those
// that test some specific aspect of the logic that propagates pointees through
// function calls.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST_F(LifetimeAnalysisTest, SimpleFnIdentity) {
  EXPECT_THAT(GetLifetimes(R"(
    int* f(int* a) {
      return a;
    }
    int* target(int* a) {
      return f(a);
    }
  )"),
              LifetimesAre({{"f", "a -> a"}, {"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, SimpleFnStatic) {
  EXPECT_THAT(GetLifetimes(R"(
    int* f() {
      static int i = 42;
      return &i;
    }
    int* target() {
      return f();
    }
  )"),
              LifetimesAre({{"f", "-> a"}, {"target", "-> a"}}));
}

TEST_F(LifetimeAnalysisTest, SimpleFnStaticOutParam) {
  EXPECT_THAT(GetLifetimes(R"(
    void f(int** p) {
      static int i = 42;
      *p = &i;
    }
    int* target() {
      int* p;
      f(&p);
      return p;
    }
  )"),
              LifetimesAre({{"f", "(a, b)"}, {"target", "-> a"}}));
}

TEST_F(LifetimeAnalysisTest, SimpleFnIdentityArg1) {
  EXPECT_THAT(GetLifetimes(R"(
    int* f(int* a, int* b) {
      return a;
    }
    int* target(int* a, int* b) {
      return f(b, a);
    }
  )"),
              LifetimesAre({{"f", "a, b -> a"}, {"target", "a, b -> b"}}));
}

TEST_F(LifetimeAnalysisTest, SimpleFnCall) {
  EXPECT_THAT(
      GetLifetimes(R"(
    int* get_lesser_of(int* a, int* b) {
      if (*a < *b) {
        return a;
      }
      return b;
    }
    int* target(int* a, int* b) {
      return get_lesser_of(a, b);
    }
  )"),
      LifetimesAre({{"get_lesser_of", "a, a -> a"}, {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, RefFnCall) {
  EXPECT_THAT(GetLifetimes(R"(
    int* get_as_ptr(int& a) {
      return &a;
    }
    int* target(int& a) {
      return get_as_ptr(a);
    }
  )"),
              LifetimesAre({{"get_as_ptr", "a -> a"}, {"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, FnCall_NonPointerParameter) {
  EXPECT_THAT(GetLifetimes(R"(
    int id(int i) {
      return i;
    }
    int target() {
      return id(42);
    }
  )"),
              LifetimesAre({{"id", "()"}, {"target", ""}}));
}

TEST_F(LifetimeAnalysisTest, FnCall_DoublePointer) {
  EXPECT_THAT(GetLifetimes(R"(
    int** f(int** pp) {
      return pp;
    }
    int** target(int** pp) {
      return f(pp);
    }
  )"),
              LifetimesAre(
                  {{"f", "(a, b) -> (a, b)"}, {"target", "(a, b) -> (a, b)"}}));
}

TEST_F(LifetimeAnalysisTest, FnCall_DoublePointerDeref) {
  EXPECT_THAT(GetLifetimes(R"(
    int* f(int** pp) {
      return *pp;
    }
    int* target(int** pp) {
      return f(pp);
    }
  )"),
              LifetimesAre({{"f", "(a, b) -> a"}, {"target", "(a, b) -> a"}}));
}

TEST_F(LifetimeAnalysisTest, FnCall_MultipleDoublePointer) {
  EXPECT_THAT(GetLifetimes(R"(
    int** f(int** pp1, int** pp2) {
      return pp1;
    }
    int* target(int* p1, int* p2) {
      return *f(&p1, &p2);
    }
  )"),
              LifetimesAre({{"f", "(a, b), (c, d) -> (a, b)"},
                            {"target", "a, b -> a"}}));
}

TEST_F(LifetimeAnalysisTest, FnCall_MultipleDoublePointerWithControlFlow) {
  EXPECT_THAT(GetLifetimes(R"(
    int** f(int** pp1, int** pp2) {
      if (**pp1 < **pp2) {
        *pp1 = *pp2;
      }
      return pp1;
    }
    int* target(int* p1, int* p2) {
      return *f(&p1, &p2);
    }
  )"),
              LifetimesAre({{"f", "(a, b), (a, c) -> (a, b)"},
                            {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, FnCall_MultipleDoublePointerWithOuterConst) {
  EXPECT_THAT(GetLifetimes(R"(
    void f(int** pp1, int** const pp2) {
      *pp1 = *pp2;
    }
    int* target1(int* p1, int* p2) {
      // Making this call can cause p1 to be overwritten with p2...
      f(&p1, &p2);
      return p1;
    }
    int* target2(int* p1, int* p2) {
      // ...and it can also cause p2 to be overwritten with p1.
      //
      // The `const` only causes `pp2` itself to be const, but `*pp2` and
      // `**pp2` are both non-const. In other words, from the lifetimes of `f()`
      // alone, it would be entirely possible for it to do `*pp2 = *pp1`.
      f(&p1, &p2);
      return p2;
    }
  )"),
              LifetimesAre({{"f", "(a, b), (a, c)"},
                            {"target1", "a, a -> a"},
                            {"target2", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, FnCall_MultipleDoublePointerWithMiddleConst) {
  EXPECT_THAT(GetLifetimes(R"(
    void f(int** pp1, int* const * pp2) {
      *pp1 = *pp2;
    }
    int* target1(int* p1, int* p2) {
      // Making this call can cause p1 to be overwritten with p2...
      f(&p1, &p2);
      return p1;
    }
    int* target2(int* p1, int* p2) {
      // ...but it can't cause p2 to be overwritten with p1.
      f(&p1, &p2);
      return p2;
    }
  )"),
              LifetimesAre({{"f", "(a, b), (a, c)"},
                            {"target1", "a, a -> a"},
                            {"target2", "a, b -> b"}}));
}

TEST_F(LifetimeAnalysisTest, FnCall_MultipleDoublePointerWithInnerConst) {
  EXPECT_THAT(GetLifetimes(R"(
    void f(const int** pp1, int** pp2) {
      *pp1 = *pp2;
    }
    const int* target1(const int* p1, int* p2) {
      // Making this call can cause p1 to be overwritten with p2.
      f(&p1, &p2);
      return p1;
    }
    const int* target2(const int* p1, int* p2) {
      // The analysis concludes that p2 could also be overwritten by p1,
      // despite the fact that a const int* cannot be converted to an int*.
      // This is because, when determining what objects the callee might copy,
      // the analysis looks only at lifetimes in the function signature but not
      // at whether the objects that these lifetimes refer to can be converted
      // into one another.
      // As a result, the lifetimes we infer for target2() are stricter than
      // they would need to be.
      f(&p1, &p2);
      return p2;
    }
  )"),
              LifetimesAre({{"f", "(a, b), (a, c)"},
                            {"target1", "a, a -> a"},
                            {"target2", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, FnCall_TriplePointerWithConst_1) {
  EXPECT_THAT(GetLifetimes(R"(
    void f(int*** ppp1, int** const * ppp2) {
      *ppp1 = *ppp2;
    }
    int** target(int* p1, int** pp2) {
      // - `pp2` cannot be overwritten because of the `const` in the signature
      //   of `f()`. (Without this, we would infer a local lifetime for the
      //   return value.)
      // - `*pp2` can be overwritten.
      int** pp1 = &p1;
      f(&pp1, &pp2);
      return pp2;
    }
  )"),
              LifetimesAre({{"f", "(a, b, c), (a, b, d)"},
                            {"target", "a, (a, b) -> (a, b)"}}));
}

TEST_F(LifetimeAnalysisTest, FnCall_TriplePointerWithConst_2) {
  EXPECT_THAT(GetLifetimes(R"(
    void f(int*** ppp1, int* const ** ppp2) {
      **ppp1 = **ppp2;
    }
    int* const * target(int* p1, int* const * pp2) {
      // - `pp2` cannot be overwritten because of the lifetimes in the signature
      //   of `f()`.
      // - `*pp2` cannot be overwritten because of the `const` in the signature
      //   of `f()`.
      int** pp1 = &p1;
      f(&pp1, &pp2);
      return pp2;
    }
  )"),
              LifetimesAre({{"f", "(a, b, c), (a, d, e)"},
                            {"target", "a, (b, c) -> (b, c)"}}));
}

TEST_F(LifetimeAnalysisTest, FnCall_TriplePointerWithConst_3) {
  EXPECT_THAT(GetLifetimes(R"(
    void f(int* const ** ppp1, int* const ** ppp2) {
      *ppp1 = *ppp2;
    }
    int* const * target(int* const p1, int* const * pp2) {
      // - `pp2` can be overwritten (hence the return value has local lifetime)
      // - `*pp2` can be overwritten (hence both `p1` and `pp2` have lifetime a)
      int* const * pp1 = &p1;
      f(&pp1, &pp2);
      return pp2;
    }
  )"),
              LifetimesAre({{"f", "(a, b, c), (a, b, d)"},
                            {"target",
                             "ERROR: function returns reference to a local"}}));
}

TEST_F(LifetimeAnalysisTest, FnCall_OutputParam) {
  EXPECT_THAT(GetLifetimes(R"(
    void f(int* in, int** out) {
      *out = in;
    }
    int* target(int* p) {
      int* result;
      f(p, &result);
      return result;
    }
  )"),
              LifetimesAre({{"f", "a, (a, b)"}, {"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, FnCall_Operator) {
  EXPECT_THAT(GetLifetimes(R"(
    struct S {};
    bool operator<(const S& s1, const S& s2) {
      return false;
    }
    bool target(const S& s) {
      return s < s;
    }
  )"),
              LifetimesAre({{"operator<", "a, b"}, {"target", "a"}}));
}

TEST_F(LifetimeAnalysisTest, FnCall_PassLambda) {
  // This test doesn't do anything interesting from a lifetimes point of view.
  // It's just intended to test that we can instantiate a capture-less lambda
  // and convert it to a function pointer.
  EXPECT_THAT(GetLifetimes(R"(
    void call_callback(void(*callback)()) {
      // TODO(mboehme): Can't actually call the callback yet because we don't
      // have support for indirect callees.
      // callback();
    }

    void target() {
      call_callback([] {});
    }
  )"),
              LifetimesContain({{"call_callback", "a"}, {"target", ""}}));
}

TEST_F(LifetimeAnalysisTest, SimpleIndirectFnCall) {
  EXPECT_THAT(
      GetLifetimes(R"(
    int* get_lesser_of(int* a, int* b) {
      if (*a < *b) {
        return a;
      }
      return b;
    }
    int* target(int* a, int* b) {
      auto fp = get_lesser_of;
      return fp(a, b);
    }
  )"),
      LifetimesAre({{"get_lesser_of", "a, a -> a"}, {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, SimpleIndirectFnCallFwdDecl) {
  // Tests that the analysis correctly identifies dependencies due to non-call
  // uses of a function.
  EXPECT_THAT(
      GetLifetimes(R"(
    int* get_lesser_of(int* a, int* b);
    int* target(int* a, int* b) {
      auto fp = get_lesser_of;
      return fp(a, b);
    }
    int* get_lesser_of(int* a, int* b) {
      if (*a < *b) {
        return a;
      }
      return b;
    }
  )"),
      LifetimesAre({{"get_lesser_of", "a, a -> a"}, {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ConditionalIndirectFnCall) {
  EXPECT_THAT(GetLifetimes(R"(
    int* get_first(int* a, int* b) {
      return a;
    }
    int* get_second(int* a, int* b) {
      return b;
    }
    int* target(int* a, int* b) {
      auto fp = *a < *b ? get_first : get_second;
      return fp(a, b);
    }
  )"),
              LifetimesAre({{"get_first", "a, b -> a"},
                            {"get_second", "a, b -> b"},
                            {"target", "a, a -> a"}}));
}

// TODO(mboehme): Add a test where we're calling a function with lifetime
// signature `static -> a`. The analysis should realize that f could return
// its input pointee. Creating such a test is currently difficult because we
// don't have lifetime annotations and the inferred lifetime for the return
// value of f will always be static in this case.

TEST_F(LifetimeAnalysisTest, ComplexFnCallGraph) {
  EXPECT_THAT(GetLifetimes(R"(
    int* f(int* a, int* b) {
      if (*a < *b) {
        return a;
      }
      return b;
    }
    int* g(int* a, int* b) {
      return f(a, b);
    }
    int* h(int* a, int* b) {
      return f(a, b);
    }
    int* target(int* a, int* b, int* c, int* d) {
      return f(g(a, b), h(c, d));
    }
  )"),
              LifetimesAre({{"f", "a, a -> a"},
                            {"g", "a, a -> a"},
                            {"h", "a, a -> a"},
                            {"target", "a, a, a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ComplexFnCallGraphUnusedArgs) {
  EXPECT_THAT(GetLifetimes(R"(
    int* f(int* a, int* b) {
      if (*a < *b) {
        return a;
      }
      return b;
    }
    int* g(int* a, int* b) {
      return f(a, b);
    }
    int* h(int* a, int* b) {
      return f(a, b);
    }
    int* target(int* a, int* b, int* c, int* d) {
      return f(g(a, b), h(a, b));
    }
  )"),
              LifetimesAre({{"f", "a, a -> a"},
                            {"g", "a, a -> a"},
                            {"h", "a, a -> a"},
                            {"target", "a, a, b, c -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructCall) {
  // Tests that lifetimes of structs are properly propagated (in both
  // directions) through function calls.
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    void f(S* s, int* a) {
      s->a = a;
    }
    int* target(S* s, int* a) {
      f(s, a);
      return s->a;
    }
  )"),
              LifetimesAre({{"f", "(a, b), a"}, {"target", "(a, b), a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructDoubleCall) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    void f(S* s, int* a) {
      s->a = a;
    }
    int* g(S* s) {
      return s->a;
    }
    int* target(S* s, int* a) {
      f(s, a);
      return g(s);
    }
  )"),
              LifetimesAre({{"f", "(a, b), a"},
                            {"g", "(a, b) -> a"},
                            {"target", "(a, b), a -> a"}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

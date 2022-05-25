// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests involving initializers.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST_F(LifetimeAnalysisTest,
       ReturnStructFieldFromMultipleInitializersConstructor) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      S(T i) : i(i) {}
      T i;
    };
    int* ConstructorSyntax(int* a, int* b, bool cond) {
      return (cond ? S<int*>{a} : S<int*>{b}).i;
    }
    int* CastSyntax(int* a, int* b, bool cond) {
      return (cond ? S<int*>(a) : S<int*>(b)).i;
    }
  )"),
              LifetimesAre({
                  {"S<int *>::S", "(a, b): a"},
                  {"ConstructorSyntax", "a, a, () -> a"},
                  {"CastSyntax", "a, a, () -> a"},
              }));
}

TEST_F(LifetimeAnalysisTest,
       ReturnStructFieldFromMultipleInitializersInitList) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T i;
    };
    int* target(int* a, int* b, bool cond) {
      return (cond ? S<int*>{a} : S<int*>{b}).i;
    }
  )"),
              LifetimesAre({{"target", "a, a, () -> a"}}));
}

TEST_F(LifetimeAnalysisTest,
       ReturnStructFromMultipleInitializersConstructSyntax) {
  EXPECT_THAT(
      GetLifetimes(R"(
    template <typename T>
    struct S {
      S(T i) : i(i) {}
      T i;
    };
    S<int*> target(int* a, int* b) {
      return true ? S<int*>{a} : S<int*>{b};
    }
  )"),
      LifetimesAre({{"S<int *>::S", "(a, b): a"}, {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnStructFromMultipleInitializersCastSyntax) {
  EXPECT_THAT(
      GetLifetimes(R"(
    template <typename T>
    struct S {
      S(T i) : i(i) {}
      T i;
    };
    S<int*> target(int* a, int* b) {
      return true ? S<int*>(a) : S<int*>(b);
    }
  )"),
      LifetimesAre({{"S<int *>::S", "(a, b): a"}, {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest,
       ReturnStructFromMultipleInitializersInitListSyntax) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T i;
    };
    S<int*> target(int* a, int* b) {
      return true ? S<int*>{a} : S<int*>{b};
    }
  )"),
              LifetimesAre({{"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructWithMultipleInitializersConstructorSyntax) {
  EXPECT_THAT(
      GetLifetimes(R"(
    template <typename T>
    struct S {
      S(T i) : i(i) {}
      T i;
    };
    int* target(int* a, int* b) {
      S<int*> s = true ? S{a} : S{b};
      return s.i;
    }
  )"),
      LifetimesAre({{"S<int *>::S", "(a, b): a"}, {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructWithMultipleInitializersCastSyntax) {
  EXPECT_THAT(
      GetLifetimes(R"(
    template <typename T>
    struct S {
      S(T i) : i(i) {}
      T i;
    };
    int* target(int* a, int* b) {
      S<int*> s = true ? S(a) : S(b);
      return s.i;
    }
  )"),
      LifetimesAre({{"S<int *>::S", "(a, b): a"}, {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructWithMultipleInitializersInitListSyntax) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T i;
    };
    int* target(int* a, int* b) {
      S<int*> s = true ? S<int*>{a} : S<int*>{b};
      return s.i;
    }
  )"),
              LifetimesAre({{"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest,
       ConstructorInitWithMultipleInitializersConstructorSyntax) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct R {
      R(T i) : i(i) {}
      T i;
    };
    template <typename T>
    struct S {
      S(T a, T b) : r(true ? R{a} : R{b}) {}
      R<T> r;
    };
    int* target(int* a, int* b) {
      S<int*> s(a, b);
      return s.r.i;
    }
  )"),
              LifetimesAre({{"R<int *>::R", "(a, b): a"},
                            {"S<int *>::S", "(a, b): a, a"},
                            {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest,
       ConstructorInitWithMultipleInitializersCastSyntax) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct R {
      R(T i) : i(i) {}
      T i;
    };
    template <typename T>
    struct S {
      S(T a, T b) : r(true ? R(a) : R(b)) {}
      R<T> r;
    };
    int* target(int* a, int* b) {
      S<int*> s(a, b);
      return s.r.i;
    }
  )"),
              LifetimesAre({{"R<int *>::R", "(a, b): a"},
                            {"S<int *>::S", "(a, b): a, a"},
                            {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest,
       ConstructorInitWithMultipleInitializersInitListSyntax) {
  EXPECT_THAT(
      GetLifetimes(R"(
    template <typename T>
    struct R {
      T i;
    };
    template <typename T>
    struct S {
      S(T a, T b) : r(true ? R<T>{a} : R<T>{b}) {}
      R<T> r;
    };
    int* target(int* a, int* b) {
      S<int*> s(a, b);
      return s.r.i;
    }
  )"),
      LifetimesAre({{"S<int *>::S", "(a, b): a, a"}, {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest,
       MemberInitWithMultipleInitializersConstructorSyntax) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct R {
      R(T i) : i(i) {}
      T i;
    };
    template <typename T>
    struct S {
      S(T a, T b) : a(a), b(b) {}
      T a;
      T b;
      R<T> r{true ? R{a} : R{b}};
    };
    int* target(int* a, int* b) {
      S<int*> s(a, b);
      return s.r.i;
    }
  )"),
              LifetimesAre({{"R<int *>::R", "(a, b): a"},
                            {"S<int *>::S", "(a, b): a, a"},
                            {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, MemberInitWithMultipleInitializersCastSyntax) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct R {
      R(T i) : i(i) {}
      T i;
    };
    template <typename T>
    struct S {
      S(T a, T b) : a(a), b(b) {}
      T a;
      T b;
      R<T> r{true ? R(a) : R(b)};
    };
    int* target(int* a, int* b) {
      S<int*> s(a, b);
      return s.r.i;
    }
  )"),
              LifetimesAre({{"R<int *>::R", "(a, b): a"},
                            {"S<int *>::S", "(a, b): a, a"},
                            {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ConstructorInitWithMultiplePointers) {
  EXPECT_THAT(
      GetLifetimes(R"(
    template <typename T>
    struct R {
      R(T i) : i(i) {}
      T i;
    };
    template <typename T, typename U, typename V>
    struct S {
      S(T a, U b) : r(true ? a : b) {}
      R<V> r;
    };
    int* target(int* a, int* b) {
      S<int*, int*, int*> s(a, b);
      return s.r.i;
    }
  )"),
      LifetimesAre({{"R<int *>::R", "(a, b): a"},
                    {"S<int *, int *, int *>::S", "(<b, c, a>, d): a, a"},
                    {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest,
       ConstructorInitWithMultiplePointersAndStoresFields) {
  EXPECT_THAT(
      GetLifetimes(R"(
    template <typename T>
    struct R {
      R(T i) : i(i) {}
      T i;
    };
    template <typename T, typename U, typename V>
    struct S {
      S(T a, U b) : a_(a), b_(b), r(true ? a : b) {}
      T a_;
      U b_;
      R<V> r;
    };
    int* target(int* a, int* b) {
      S<int*, int*, int*> s(a, b);
      return s.r.i;
    }
  )"),
      LifetimesAre({{"R<int *>::R", "(a, b): a"},
                    {"S<int *, int *, int *>::S", "(<a, a, a>, b): a, a"},
                    {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, MemberInitWithMultiplePointers) {
  EXPECT_THAT(
      GetLifetimes(R"(
    template <typename T>
    struct R {
      R(T i) : i(i) {}
      T i;
    };
    template <typename T, typename U, typename V>
    struct S {
      S(T a, U b) : a(a), b(b) {}
      T a;
      U b;
      R<V> r{true ? a : b};
    };
    int* target(int* a, int* b) {
      S<int*, int*, int*> s(a, b);
      return s.r.i;
    }
  )"),
      LifetimesAre({{"R<int *>::R", "(a, b): a"},
                    {"S<int *, int *, int *>::S", "(<a, a, a>, b): a, a"},
                    {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, MemberInitWithMultipleInitializersInitListSyntax) {
  EXPECT_THAT(
      GetLifetimes(R"(
    template <typename T>
    struct R {
      T i;
    };
    template <typename T>
    struct S {
      S(T a, T b) : a(a), b(b) {}
      T a;
      T b;
      R<T> r{true ? R<T>{a} : R<T>{b}};
    };
    int* target(int* a, int* b) {
      S<int*> s(a, b);
      return s.r.i;
    }
  )"),
      LifetimesAre({{"S<int *>::S", "(a, b): a, a"}, {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, DeclStructInitializerWithConversionOperator) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct R {
      T a;
    };
    template <typename T>
    struct S {
      T a;
      operator R<T>() { return {a}; }
    };
    int* target(int* a) {
      R<int*> r = S<int*>{a};
      return r.a;
    }
  )"),
              LifetimesAre({{"S<int *>::operator R", "(a, b): -> a"},
                            {"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, DeclStructInitializerFromCall) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct R {
      T a;
    };
    template <typename T>
    struct R<T> f(T a) {
      return R<T>{a};
    }
    int* target(int* a) {
      R<int*> r = f<int*>(a);
      return r.a;
    }
  )"),
              LifetimesAre({{"f", "a -> a"}, {"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnStructInitializerWithConversionOperator) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct R {
      T a;
    };
    template <typename T>
    struct S {
      T a;
      operator R<T>() { return {a}; }
    };
    R<int*> target(int* a) {
      return S<int*>{a};
    }
  )"),
              LifetimesAre({{"S<int *>::operator R", "(a, b): -> a"},
                            {"target", "a -> a"}}));
}

// TODO(danakj): Crashes due to operator() not being a CXXConstructExpr, but
// SetExprObjectSetRespectingType only handles CXXConstructExpr for record
// types.
TEST_F(LifetimeAnalysisTest,
       DISABLED_ConstructorInitializerWithConversionOperator) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T a;
    };
    template <typename T>
    struct R {
      T a;
      operator S<T>() { return {a}; }
    };

    // This initializes the `s` field from a constructor initializer.
    template <typename T>
    struct QConstructor {
      QQConstructor(T a) : s(R<T>{a}) {}
      S<T> s;
    };
    int* constructor(int* a) {
      return QQConstructor<int*>{a}.s.a;
    }

    // This initializes the `s` field from a transparent InitListExpr on a
    // member initializer.
    template <typename T>
    struct QMember {
      QMember(T a) : a(a) {}
      T a;
      S<T> s{S<T>(R<T>{a})};
    };
    int* member(int* a) {
      return QMember<int*>{a}.s.a;
    }
)"),
              LifetimesAre({{"QConstructor<int *>::QConstructor", "(a, b): a"},
                            {"QMember<int *>::QMember", "(a, b): a"},
                            {"constructor", "a -> a"},
                            {"member", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructInitializerWithCtorCall) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      S(T a) : a(a) {}
      T a;
    };
    int* TransparentInitListExpr(int* a) {
      S<int*> s{S<int*>(a)};
      return s.a;
    }
    int* CastSyntax(int* a) {
      S<int*> s((S<int*>(a)));
      return s.a;
    }
  )"),
              LifetimesAre({{"S<int *>::S", "(a, b): a"},
                            {"TransparentInitListExpr", "a -> a"},
                            {"CastSyntax", "a -> a"}}));
}

// TODO(danakj): Crashes because the initializer expression is a
// CXXStaticCastExpr, and operator() is not a CXXConstructExpr, but
// SetExprObjectSetRespectingType only handles CXXConstructExpr for record
// types.
TEST_F(LifetimeAnalysisTest, DISABLED_StaticCastInitializer) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T a;
    };
    template <typename T>
    struct R {
      T a;
      operator S<T>() { return {a}; }
    };
    int* target(int* a) {
      return static_cast<S<int*>>(R<int*>{a}).a;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

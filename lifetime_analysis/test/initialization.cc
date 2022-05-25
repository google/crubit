// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for initialization.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

// TODO(danakj): Crashes trying to find the initializer expression under
// MaterializeTemporaryExpr. Should be improved by cl/414032764.
TEST_F(LifetimeAnalysisTest, DISABLED_VarDeclReferenceToRecordTemporary) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T a;
    };
    int* target(int* a) {
      const S<int*>& s = S<int*>{a};
      return s.a;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, VarDeclReferenceToRecordTemplate) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T a;
    };
    S<int*>* target(S<int*>* a) {
      S<int*>& b = *a;
      return &b;
    }
  )"),
              LifetimesAre({{"target", "(a, b) -> (a, b)"}}));
}

TEST_F(LifetimeAnalysisTest, VarDeclReferenceToRecordNoTemplate) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    S* target(S* a) {
      S& b = *a;
      return &b;
    }
  )"),
              LifetimesAre({{"target", "(a, b) -> (a, b)"}}));
}

TEST_F(LifetimeAnalysisTest, ConstructorInitReferenceToRecord) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    template <class Ref>
    struct R {
      R(S& s): s(s) {}
      Ref s;
    };
    int* target(S* a) {
      R<S&> r(*a);
      return r.s.a;
    }
  )"),
              LifetimesAre({{"R<S &>::R", "(a, b, c): (a, b)"},
                            {"target", "(a, b) -> a"}}));
}

// TODO(danakj): Fails because a nested TransferMemberExpr() ends up looking for
// the field from the outer expr on the object of the inner expr.
//
// The code:
// ObjectSet struct_points_to =
//     points_to_map.GetExprObjectSet(member->getBase());
//
// The AST:
// MemberExpr 0x4027d3f2628 'int *':'int *' lvalue .p 0x4027d3f7338
// `-MemberExpr 0x4027d3f25f8 'S<int *>':'struct S<int *>' lvalue .s
//   0x4027d3f74c0
//   `-DeclRefExpr 0x4027d3f25d8 'R<int *>':'struct R<int *>' lvalue Var
//     0x4027d3f6cd0 'r' 'R<int *>':'struct R<int *>'
//
// The p field is on struct S, but the code tries to find it on an object
// of type R<int *>.
TEST_F(LifetimeAnalysisTest, MemberInitReferenceToRecord) {
  EXPECT_THAT(
      GetLifetimes(R"(
    template <typename P>
    struct S {
      P p;
    };
    template<typename P>
    struct [[clang::annotate("lifetime_params", "a")]] R {
      R(P p): ss{p} {}
      S<P> ss;
      [[clang::annotate("member_lifetimes", "a")]]
      S<P>& s{ss};
    };
    int* target(int* a) {
      R<int*> r(a);
      return r.s.p;
    }
  )"),
      LifetimesAre({{"R<int *>::R", "(<a> [b], b): a"}, {"target", "a -> a"}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

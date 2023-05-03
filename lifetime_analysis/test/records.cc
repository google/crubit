// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests involving (non-template) records (structs, classes, unions).

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST_F(LifetimeAnalysisTest, MembersWithSameAnnotationMergeLifetimes) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* i;
      [[clang::annotate("member_lifetimes", "a")]]
      int* j;
    };
    void target(S* s, int* p, int* q) {
      s->i = p;
      s->j = q;
    }
  )"),
              LifetimesAre({{"target", "(a, b), a, a"}}));
}

TEST_F(LifetimeAnalysisTest, StructsWithTemplateFieldsDoesNotMergeLifetimes) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename A, typename B>
    struct S { A i; B j; };
    void target(S<int*, int*>* s, int* p, int* q) {
      s->i = p;
      s->j = q;
    }
  )"),
              LifetimesAre({{"target", "(<a, b>, c), a, b"}}));
}

TEST_F(LifetimeAnalysisTest, StructWithArrayMergesLifetimes) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename A>
    struct S { A array; };
    void target(S<int**>* s, int* p, int* q) {
      s->array[0] = p;
      s->array[1] = q;
    }
  )"),
              LifetimesAre({{"target", "(a, b, c), a, a"}}));
}

TEST_F(LifetimeAnalysisTest, DeclRecordWithConditionalOperator) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename P>
    struct S { P p; };
    int* target(int* a, int* b, bool cond) {
      S<int*> s = cond ? S<int*>{a} : S<int*>{b};
      return s.p;
    }
  )"),
              LifetimesAre({{"target", "a, a, () -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnRecordWithConditionalOperator) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename P>
    struct S { P p; };
    S<int*> target(int* a, int* b, bool cond) {
      return cond ? S<int*>{a} : S<int*>{b};
    }
  )"),
              LifetimesAre({{"target", "a, a, () -> a"}}));
}

TEST_F(LifetimeAnalysisTest, MaterializeRecordWithConditionalOperator) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename P>
    struct S { P p; };
    int* target(int* a, int* b, bool cond) {
      return (cond ? S<int*>{a} : S<int*>{b}).p;
    }
  )"),
              LifetimesAre({{"target", "a, a, () -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ConstructorInitRecordWithConditionalOperator) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename P>
    struct S { P p; };
    template <typename P>
    struct T {
      T(int* a, int* b, bool cond) : s(cond ? S<int*>{a} : S<int*>{b}) {}
      S<P> s;
    };
    int* target(int* a, int* b, bool cond) {
      T<int*> t(a, b, cond);
      return t.s.p;
    }
  )"),
              LifetimesAre({{"T<int *>::T", "(a, b): a, a, ()"},
                            {"target", "a, a, () -> a"}}));
}

TEST_F(LifetimeAnalysisTest, MemberInitRecordWithConditionalOperator) {
  EXPECT_THAT(
      GetLifetimes(R"(
    template <typename P>
    struct S { P p; };
    template <typename A, typename B, typename P>
    struct T {
      T(int* a, int* b, bool cond) : a(a), b(b), cond(cond) {}
      A a;
      B b;
      bool cond;
      S<P> s{cond ? S<int*>{a} : S<int*>{b}};
    };
    int* target(int* a, int* b, bool cond) {
      T<int*, int*, int*> t(a, b, cond);
      return t.s.p;
    }
  )"),
      LifetimesAre({{"T<int *, int *, int *>::T", "(<a, a, a>, b): a, a, ()"},
                    {"target", "a, a, () -> a"}}));
}

TEST_F(LifetimeAnalysisTest, SimpleStruct) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
      [[clang::annotate("member_lifetimes", "a")]]
      int* b;
    };
    void target(S* s, int* a, int* b) {
      s->a = a;
      s->b = b;
    }
  )"),
              LifetimesAre({{"target", "(a, b), a, a"}}));
}

TEST_F(LifetimeAnalysisTest, SimpleUnion) {
  EXPECT_THAT(GetLifetimes(R"(
    union [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
      [[clang::annotate("member_lifetimes", "a")]]
      int* b;
    };
    void target(S* s, int* a, int* b) {
      s->a = a;
      s->b = b;
    }
  )"),
              LifetimesAre({{"target", "(a, b), a, a"}}));
}

TEST_F(LifetimeAnalysisTest, StructReference) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
      [[clang::annotate("member_lifetimes", "a")]]
      int* b;
    };
    void target(S& s, int* a, int* b) {
      s.a = a;
      s.b = b;
    }
  )"),
              LifetimesAre({{"target", "(a, b), a, a"}}));
}

TEST_F(LifetimeAnalysisTest, StructValue) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    int* target(S s) {
      return s.a;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructMultiplePtrsSameLifetime) {
  // TODO(veluca): here, we correctly deduce *once f gets called* that something
  // fishy is going on, namely, that `s.a` could be pointing to a local
  // variable. However, we should already know this from the initialization of
  // `s`.
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a", "a")]]
      int** a;
      [[clang::annotate("member_lifetimes", "a")]]
      int* b;
    };
    void f(S& s) {
      *s.a = s.b;
    }
    void target(int* a, int b) {
       S s{&a, &b};
       f(s);
    }
  )"),
              LifetimesContain({{"target",
                                 "ERROR: function returns reference to a local "
                                 "through parameter 'a'"}}));
}

TEST_F(LifetimeAnalysisTest, StructNonLocalPtr) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    int* target(S* s) {
      return s->a;
    }
  )"),
              LifetimesAre({{"target", "(a, b) -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructMemberStructInitializedWithInitializerList) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] T {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    struct [[clang::annotate("lifetime_params", "a")]] S {
      S(int* a): t{a} { }
      [[clang::annotate("member_lifetimes", "a")]]
      T t;
    };
    int* target(int* a) {
      return S{a}.t.a;
    }
  )"),
              LifetimesAre({{"S::S", "(a, b): a"}, {"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructMemberPtr) {
  EXPECT_THAT(GetLifetimes(R"(
    struct S {
      int a;
    };
    int* target(S* s) {
      return &s->a;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructReferenceMember) {
  // This is a regression test for a bug where we were not treating accesses to
  // member variables of reference type correctly.
  EXPECT_THAT(GetLifetimes(R"(
    struct S1 {
      int a;
    };
    struct [[clang::annotate("lifetime_params", "a")]] S2 {
      [[clang::annotate("member_lifetimes", "a")]]
      S1 &s1;
    };
    int& target(S2* s2) {
      // Make sure we can find the field S1::a. This is to ensure that our
      // member access for s2->s1 is in fact returning an object of type S1
      // (not S1&).
      s2->s1.a = 5;
      return s2->s1.a;
    }
  )"),
              LifetimesAre({{"target", "(a, b) -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructStaticMemberFunction) {
  EXPECT_THAT(GetLifetimes(R"(
    struct S {
      static int* f(int* x) { return x; }
    };
    int* target(int* a) {
      return S::f(a);
    }
  )"),
              LifetimesAre({{"S::f", "a -> a"}, {"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructMemberFunction) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
      int* f() { return a; }
    };
  )"),
              LifetimesAre({{"S::f", "(a, b): -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructMemberFunctionExplicitThis) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
      int* f() { return this->a; }
    };
  )"),
              LifetimesAre({{"S::f", "(a, b): -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructMemberFunctionCall) {
  EXPECT_THAT(
      GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
      int* f() { return a; }
    };
    int* target(S* s) {
      return s->f();
    }
  )"),
      LifetimesAre({{"S::f", "(a, b): -> a"}, {"target", "(a, b) -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructMemberFunctionCallDot) {
  EXPECT_THAT(
      GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
      int* f() { return a; }
    };
    int* target(S* s) {
      return (*s).f();
    }
  )"),
      LifetimesAre({{"S::f", "(a, b): -> a"}, {"target", "(a, b) -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructMemberFunctionComplexCall) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
      void set(int* x) { a = x; }
      int* f() { return a; }
    };
    int* target(S* s, int* b) {
      s->set(b);
      return (*s).f();
    }
  )"),
              LifetimesAre({{"S::set", "(a, b): a"},
                            {"S::f", "(a, b): -> a"},
                            {"target", "(a, b), a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructReturnAddressOfMemberFunction) {
  EXPECT_THAT(GetLifetimes(R"(
    struct S {
      static void f();
    };
    typedef void (*funtype)();
    funtype target() {
      S s;
      return s.f;
    }
  )"),
              LifetimesContain({{"target", "-> ()"}}));
}

TEST_F(LifetimeAnalysisTest, StructDefaultConstructor) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    void target() {
      S s;
    }
  )"),
              LifetimesAre({{"target", ""}}));
}

TEST_F(LifetimeAnalysisTest, StructDefaultConstructor_ExplicitCall) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    void target() {
      S();
    }
  )"),
              LifetimesAre({{"target", ""}}));
}

TEST_F(LifetimeAnalysisTest, StructConstructorStatic) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      S(int* a) { this->a = a; }
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    void target(int* a) {
      static S s{a};
    }
  )"),
              LifetimesAre({{"S::S", "(a, b): a"}, {"target", "static"}}));
}

TEST_F(LifetimeAnalysisTest, StructCopyConstructorStatic) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    void target(S* x) {
      static S s = *x;
    }
  )"),
              LifetimesAre({{"target", "(static, a)"}}));
}

TEST_F(LifetimeAnalysisTest, StructConstructorOutputsFieldPointer) {
  EXPECT_THAT(GetLifetimes(R"(
    struct S {
      S(int** field_out) {
        *field_out = &i;
      }
      int i;
    };
    int* target() {
     int* i_out;
     S s(&i_out);
     return i_out;
    }
  )"),
              LifetimesAre({{"S::S", "a: (a, b)"},
                            {"target",
                             "ERROR: function returns reference to a local"}}));
}

TEST_F(LifetimeAnalysisTest, StructConstructorOutputsThisPointer) {
  EXPECT_THAT(GetLifetimes(R"(
    struct S {
      S(S** this_out) {
        *this_out = this;
      }
    };
    S* target() {
     S* s_out;
     S s(&s_out);
     return s_out;
    }
  )"),
              LifetimesAre({{"S::S", "a: (a, b)"},
                            {"target",
                             "ERROR: function returns reference to a local"}}));
}

TEST_F(LifetimeAnalysisTest,
       StructConstructorOutputsFieldPointerConstructorInitializer) {
  EXPECT_THAT(GetLifetimes(R"(
    struct S {
      S(int** field_out) {
        *field_out = &i;
      }
      int i;
    };
    struct T {
      T(int** int_out): s(int_out) {}
      S s;
    };
  )"),
              LifetimesAre({{"S::S", "a: (a, b)"}, {"T::T", "a: (a, b)"}}));
}

TEST_F(LifetimeAnalysisTest,
       StructConstructorOutputsThisPointerConstructorInitializer) {
  EXPECT_THAT(GetLifetimes(R"(
    struct S {
      S(S** this_out) {
        *this_out = this;
      } 
    };
    struct T {
      T(S** this_out): s(this_out) {}
      S s;
    };
  )"),
              LifetimesAre({{"S::S", "a: (a, b)"}, {"T::T", "a: (a, b)"}}));
}

TEST_F(LifetimeAnalysisTest, StructConstructorOutputsThisPointerInitMember) {
  EXPECT_THAT(GetLifetimes(R"(
    struct S {
      S(S** this_out) {
        *this_out = this;
      }
    };
    static S* static_s_ptr;
    struct T {
      T() {}
      S s{&static_s_ptr};
    };
  )"),
              LifetimesAre({{"S::S", "a: (a, b)"}, {"T::T", "static:"}}));
}

TEST_F(LifetimeAnalysisTest, StructConstructorInitializers) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      S(int* a): a(a) { }
      [[clang::annotate("member_lifetimes", "a")]]
      int* a = nullptr;
      // The following members don't affect lifetimes, but we keep them
      // around to make sure that the related code is exercised.
      int b = 0;
      // This member points into the struct itself, forcing the lifetime
      // parameter in the constructor to be the same as the lifetime of the
      // object itself.
      [[clang::annotate("member_lifetimes", "a")]]
      int* c = &b;
      int d;
    };
  )"),
              LifetimesAre({{"S::S", "(a, a): a"}}));
}

TEST_F(LifetimeAnalysisTest, StructConstructorStaticPtr) {
  EXPECT_THAT(GetLifetimes(R"(
    static int x;
    struct [[clang::annotate("lifetime_params", "a")]] S {
      S() { a = &x; }
      [[clang::annotate("member_lifetimes", "a")]]
      int* a = nullptr;
    };
  )"),
              LifetimesAre({{"S::S", "(a, b):"}}));
}

TEST_F(LifetimeAnalysisTest, StructConstructorStaticPtrInitializer) {
  EXPECT_THAT(GetLifetimes(R"(
    static int x;
    struct [[clang::annotate("lifetime_params", "a")]] S {
      S(): a(&x) { }
      [[clang::annotate("member_lifetimes", "a")]]
      int* a = nullptr;
    };
  )"),
              LifetimesAre({{"S::S", "(a, b):"}}));
}

TEST_F(LifetimeAnalysisTest, StructConstructorStaticPtrMemberInitializer) {
  EXPECT_THAT(GetLifetimes(R"(
    static int x;
    struct [[clang::annotate("lifetime_params", "a")]] S {
      S() { }
      [[clang::annotate("member_lifetimes", "a")]]
      int* a = &x;
    };
  )"),
              LifetimesAre({{"S::S", "(a, b):"}}));
}

TEST_F(LifetimeAnalysisTest, StructMemberFreeFunction) {
  // Check that calling a method behaves in the same way as a free function.
  EXPECT_THAT(GetLifetimes(R"(
    static int x;
    struct [[clang::annotate("lifetime_params", "a")]] S {
      void f() { a = &x; }
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    void f(S& a) {
      a.a = &x;
    }
  )"),
              LifetimesAre({{"S::f", "(a, b):"}, {"f", "(a, b)"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnFieldFromTemporaryStructConstructor) {
  // S(i) with a single argument produces a clang::CXXFunctionalCastExpr around
  // a clang::CXXConstructExpr.
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      S(int* i) : f(i) {}
      [[clang::annotate("member_lifetimes", "a")]]
      int* f;
    };
    int* ConstructorSyntax(int* i) {
      return S{i}.f;
    }
    int* CastSyntax(int* i) {
      return S(i).f;
    }
  )"),
              LifetimesAre({{"S::S", "(a, b): a"},
                            {"ConstructorSyntax", "a -> a"},
                            {"CastSyntax", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest,
       ReturnFieldFromTemporaryStructConstructorInitList) {
  // S has no constructors so S{i} produces a clang::InitListExpr.
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* f;
    };
    int* target(int* i) {
      return S{i}.f;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnFieldFromTemporaryUnion) {
  // S has no constructors so S{i} produces a clang::InitListExpr.
  EXPECT_THAT(GetLifetimes(R"(
    union [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* f;
    };
    int* target(int* i) {
      return S{i}.f;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructInitList) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    void target(int* a) {
      S s{a};
    }
  )"),
              LifetimesAre({{"target", "a"}}));
}

TEST_F(LifetimeAnalysisTest, UnionInitList) {
  EXPECT_THAT(GetLifetimes(R"(
    union [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    void target(int* a) {
      S s{a};
    }
  )"),
              LifetimesAre({{"target", "a"}}));
}

TEST_F(LifetimeAnalysisTest, StructCopy) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    void target(S* a, S* b) {
      *a = *b;
    }
  )"),
              LifetimesAre({{"target", "(a, b), (a, c)"}}));
}

TEST_F(LifetimeAnalysisTest, StructCopyStatic) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    void target(S* x) {
      static S s;
      s = *x;
    }
  )"),
              LifetimesAre({{"target", "(static, a)"}}));
}

// We fail to initialize the temporary object in a CXXOperatorCallExpr argument,
// which causes us to assert when we visit the MaterializeTemporaryExpr later.
TEST_F(LifetimeAnalysisTest, DISABLED_CallExprWithRecordInitializedArguments) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* i;
    };
    S callee(const S& s1, const S& s2) {
      return S{s2.i};
    }
    S target(int* a, int* b) {
      return callee(S{a}, S{b});
    }
  )"),
              LifetimesAre({{"target", "a, b -> b"}}));
}

TEST_F(LifetimeAnalysisTest, StructAssignMemberStatic) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    void target(int* x) {
      static S s;
      s.a = x;
    }
  )"),
              LifetimesAre({{"target", "static"}}));
}

TEST_F(LifetimeAnalysisTest, StructCopyExplicit) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
      S& operator=(const S& other) {
        a = other.a;
        return *this;
      }
    };
    void target(S* a, S* b) {
      *a = *b;
    }
  )"),
              LifetimesAre({{"S::operator=", "(a, c): (a, b) -> (a, c)"},
                            {"target", "(a, b), (a, c)"}}));
}

TEST_F(LifetimeAnalysisTest, StructCopyExplicitNoop) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
      S& operator=(const S& other) {
        return *this;
      }
    };
    void target(S* a, S* b) {
      *a = *b;
    }
  )"),
              LifetimesAre({{"S::operator=", "(c, d): (a, b) -> (c, d)"},
                            {"target", "(a, b), (c, d)"}}));
}

TEST_F(LifetimeAnalysisTest, StructWithStruct) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] T {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      T t;
    };
    int* target(S* s) {
      return s->t.a;
    }
  )"),
              LifetimesAre({{"target", "(a, b) -> a"}}));
}

TEST_F(LifetimeAnalysisTest, NonReferenceLikeStruct) {
  EXPECT_THAT(GetLifetimes(R"(
    struct S {
      int a;
    };
    void target(S* a, S* b) {
      a->a = b->a;
    }
  )"),
              LifetimesAre({{"target", "a, b"}}));
}

TEST_F(LifetimeAnalysisTest, StructNonReferenceLikeField) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      int a;
      [[clang::annotate("member_lifetimes", "a")]]
      int* b;
    };
    void target(S* a, S* b) {
      a->a = b->a;
    }
  )"),
              LifetimesAre({{"target", "(a, b), (c, d)"}}));
}

TEST_F(LifetimeAnalysisTest, StructAssignToReference) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      int a;
      [[clang::annotate("member_lifetimes", "a")]]
      int& b;
    };
    void target(S* a, S* b) {
      a->a = b->a;
      a->b = b->b;
    }
  )"),
              LifetimesAre({{"target", "(a, b), (c, d)"}}));
}

TEST_F(LifetimeAnalysisTest, NonReferenceLikeStructCopyAssignment) {
  EXPECT_THAT(GetLifetimes(R"(
    struct S {
      int a;
    };
    void target(S* a, S* b) {
      *a = *b;
    }
  )"),
              LifetimesAre({{"target", "a, b"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnReferenceLikeStruct) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* p;
    };
    S target() {
      int i = 42;
      S s = { &i };
      return s;
    }
  )"),
              LifetimesAre({{"target",
                             "ERROR: function returns reference to a local"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnNonReferenceLikeStruct) {
  EXPECT_THAT(GetLifetimes(R"(
    struct S {
      int a;
    };
    S target() {
      int i = 42;
      S s = { i };
      return s;
    }
  )"),
              LifetimesAre({{"target", ""}}));
}

TEST_F(LifetimeAnalysisTest, ReturnNonReferenceLikeStructCopy) {
  EXPECT_THAT(GetLifetimes(R"(
    struct S {
      int a;
    };
    S target(S& s) {
      return s;
    }
  )"),
              LifetimesAre({{"target", "a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnNonReferenceLikeStructFromTemporary) {
  // This is a repro for a crash observed on b/228325046.
  EXPECT_THAT(GetLifetimes(R"(
    struct S {};
    S target() {
      return S();
    }
  )"),
              LifetimesAre({{"target", ""}}));
}

TEST_F(LifetimeAnalysisTest, StructInnerDoublePtrInitList) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a", "a")]]
      int** x;
    };

    void f(int** b) {
      S s{b};
      int i = 0;
      *s.x = &i;
    }
  )"),
              LifetimesAre({{"f",
                             "ERROR: function returns reference to a local "
                             "through parameter 'b'"}}));
}

TEST_F(LifetimeAnalysisTest, StructInnerDoublePtr) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a", "a")]]
      int** x;
    };

    void g(S* s, int* a) {
      *s->x = a;
    }

    void f(int* a, int** b) {
      S s{b};
      g(&s, a);
    }
  )"),
              LifetimesAre({{"f", "a, (a, a)"}, {"g", "(a, b), a"}}));
}

TEST_F(LifetimeAnalysisTest, StructInnerDoublePtrAssign) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a", "a")]]
      int** x;
    };

    void g(S* s, int* a) {
      *s->x = a;
    }

    int* f(int* a, int** b) {
      S s{b};
      g(&s, a);
      return *b;
    }
  )"),
              LifetimesAre({{"f", "a, (a, a) -> a"}, {"g", "(a, b), a"}}));
}

TEST_F(LifetimeAnalysisTest, StructInnerDoublePtrParam) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a", "a")]]
      int** x;
    };

    void g(S* s, int* a) {
      *s->x = a;
    }

    void f(S& s, int* a, int** b) {
      s.x = b;
      g(&s, a);
    }
  )"),
              LifetimesAre({{"f", "(a, b), a, (a, a)"}, {"g", "(a, b), a"}}));
}

TEST_F(LifetimeAnalysisTest, List) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] List {
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
      [[clang::annotate("member_lifetimes", "a", "a")]]
      List* next;
      void Append(List& oth) {
        next = &oth;
      }
      int* Get() const {
        return a;
      }
    };
    int* target(List* l, int* a) {
      if (l->next) {
        l->next->a = a;
      }
      return l->Get();
    }
  )"),
              LifetimesAre({{"List::Append", "(a, b): (a, a)"},
                            {"List::Get", "(a, b): -> a"},
                            {"target", "(a, b), a -> a"}}));
}

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

TEST_F(LifetimeAnalysisTest, StructTemplateReturn) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T a;
    };
    S<int*> target(S<int*>& s) {
      return s;
    }
  )"),
              LifetimesAre({{"target", "(a, b) -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructTemplateReturnXvalue) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T a;
    };
    S<int*> target(S<int*> s) {
      return s;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructTemplateReturnCall) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T a;
    };
    S<int*> take_by_ref(S<int*>& s) {
      return s;
    }
    S<int*> take_by_value(S<int*> s) {
      return s;
    }
  )"),
              LifetimesAre({{"take_by_ref", "(a, b) -> a"},
                            {"take_by_value", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructTemplateReturnLocal) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T a;
    };
    S<int*> target(int* a) {
      int i = 42;
      return { &i };
    }
  )"),
              LifetimesAre({{"target",
                             "ERROR: function returns reference to a local"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnStructTemporaryConstructor) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      S(T a) : a(a) {}
      T a;
    };
    S<int*> ConstructorCastSyntax(int* a) {
      return S(a);
    }
    S<int*> ConstructTemporarySyntax(int* a) {
      return S{a};
    }
  )"),
              LifetimesAre({{"S<int *>::S", "(a, b): a"},
                            {"ConstructorCastSyntax", "a -> a"},
                            {"ConstructTemporarySyntax", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnStructTemporaryInitializerList) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T a;
    };
    S<int*> InitListExpr(int* a) {
      return {a};
    }
    S<int*> CastWithInitListExpr(int* a) {
      return S<int*>{a};
    }
  )"),
              LifetimesAre({{"InitListExpr", "a -> a"},
                            {"CastWithInitListExpr", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReturnUnionTemporaryInitializerList) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    union S {
      T a;
    };
    S<int*> target(int* a) {
      return {a};
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructWithUnionMember) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] T {
      [[clang::annotate("member_lifetimes", "a")]]
      union [[clang::annotate("lifetime_params", "a")]] U {
        [[clang::annotate("member_lifetimes", "a")]]
        int* i;
      } u;
    };
    int* target(T* t) {
      return t->u.i;
    }
  )"),
              LifetimesAre({{"target", "(a, b) -> a"}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

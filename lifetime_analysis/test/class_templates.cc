// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests involving class templates.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST_F(LifetimeAnalysisTest, StructTemplate) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T a;
    };
    int* target(S<int*> s) {
      return s.a;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructTemplatePtr) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T a;
    };
    int* target(S<int*>* s) {
      return s->a;
    }
  )"),
              LifetimesAre({{"target", "(a, b) -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructTemplateInnerDoubleUsage) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T a;
      T b;
    };
    int* target(S<int**>* s) {
      int l = 0;
      *s->b = &l;
      return *s->a;
    }
  )"),
              LifetimesAre({{"target",
                             "ERROR: function returns reference to a local "
                             "through parameter 's'"}}));
}

TEST_F(LifetimeAnalysisTest, StructTwoTemplateArguments) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T, typename U>
    struct S {
      T t;
      U u;
    };

    int* return_t(S<int*, int*>& v) {
      return v.t;
    }

    int* return_u(S<int*, int*>& v) {
      return v.u;
    }
  )"),
              LifetimesAre({{"return_t", "(<a, b>, c) -> a"},
                            {"return_u", "(<a, b>, c) -> b"}}));
}

// Disabled due to b/253992761.
TEST_F(LifetimeAnalysisTest, DISABLED_StructTwoTemplateArgumentsNestedClasses) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct Outer {
      template <typename U>
      struct Inner {
        T t;
        U u;
      };
    };

    int* return_t(Outer<int*>::Inner<int*>& inner) {
      return inner.t;
    }

    int* return_u(Outer<int*>::Inner<int*>& inner) {
      return inner.u;
    }
  )"),
              LifetimesAre({{"return_t", "(<a>::<b>, c) -> a"},
                            {"return_u", "(<a>::<b>, c) -> b"}}));
}

TEST_F(LifetimeAnalysisTest, StructTwoTemplateArgumentsConstructInner) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct Inner {
      Inner (T a): a(a) {}
      T a;
    };
    template <typename T, typename U>
    struct Outer {
      Outer(T a, U& b): a(a), b(b) {}
      T a;
      U b;
    };
    int* target(int* a, int* b) {
      Inner<int*> is(b);
      Outer<int*, Inner<int*>> s(a, is);
      return s.b.a;
    }
  )"),
              LifetimesContain({{"target", "a, b -> b"}}));
}

TEST_F(LifetimeAnalysisTest, StructTwoTemplateArgumentsTernary) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T, typename U>
    struct S {
      T t;
      U u;
    };

    int* f(S<int*, int*>& v) {
      return *v.t < *v.u ? v.t : v.u;
    }
  )"),
              LifetimesAre({{"f", "(<a, a>, b) -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructTemplateLocalVariable) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T a;
    };
    const int* target(S<int*> s) {
      S<const int*> t;
      t.a = s.a;
      return t.a;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructTemplatePointerToMember) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T, typename U>
    struct S {
      T a;
      U b;
    };
    int** target(S<int*, int*>& s) {
      return &s.b;
    }
  )"),
              LifetimesAre({{"target", "(<a, b>, c) -> (b, c)"}}));
}

TEST_F(LifetimeAnalysisTest, StructTemplateWithPointer) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      T* a;
    };
    int** target(S<int*>& s) {
      return s.a;
    }
  )"),
              LifetimesAre({{"target", "(<a> [b], c) -> (a, b)"}}));
}

TEST_F(LifetimeAnalysisTest, StructTemplateWithTemplate) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T a;
    };
    int* target(S<S<int*>> s) {
      return s.a.a;
    }
  )"),
              LifetimesAre({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructTemplateInnerTemplate) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct U {
      T a;
    };
    template <typename T>
    struct S {
      U<T> a;
    };
    int* target(S<int*>* s) {
      return s->a.a;
    }
  )"),
              LifetimesAre({{"target", "(a, b) -> a"}}));
}

TEST_F(LifetimeAnalysisTest, DISABLED_StructTemplateInnerTemplatePtr) {
  // TODO(veluca): we don't correctly propagate lifetime arguments when creating
  // template arguments for fields that use the template argument indirectly,
  // such as behind a pointer or as template arguments to a struct passed as a
  // template argument to the member.
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct U {
      T a;
    };
    template <typename T>
    struct S {
      U<T*> a;
    };
    int* target(S<int*>* s) {
      return *s->a.a;
    }
  )"),
              LifetimesAre({{"target", "(a, b) -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructTemplateSwapArguments) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T, typename U>
    struct [[clang::annotate("lifetime_params", "a")]] S {
      T a;
      U b;
      [[clang::annotate("member_lifetimes", "a", "a")]]
      S<U, T>* next;
    };
    int* target(S<int*, int*>* s) {
      return s->next->a;
    }
    int* target_swtwice(S<int*, int*>* s) {
      return s->next->next->a;
    }
  )"),
              LifetimesAre({{"target", "(<a, b> [c], d) -> b"},
                            {"target_swtwice", "(<a, b> [c], d) -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructTemplateMemberCall) {
  EXPECT_THAT(
      GetLifetimes(R"(
    template <typename T>
    // TODO(mboehme): The real `vector` doesn't have lifetime parameters, but
    // we use these here as we don't have the ability to do `lifetime_cast`s
    // yet.
    struct [[clang::annotate("lifetime_params", "a")]] vector {
      T& operator[](int i) { return a[i]; }
      [[clang::annotate("member_lifetimes", "a")]]
      T* a;
    };

    int* get(vector<int*>& v, int i) {
      return v[i];
    }
  )"),
      LifetimesAre({{"vector<int *>::operator[]", "(<a> [b], c): () -> (a, b)"},
                    {"get", "(<a> [b], c), () -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructTwoTemplateArgumentsCall) {
  EXPECT_THAT(
      GetLifetimes(R"(
    template <typename T, typename U>
    struct S {
      T t;
      U u;
    };

    int* f(S<int*, int*>& v) {
      return *v.t < *v.u ? v.t : v.u;
    }

    int* g(S<int*, int*>& v) {
      return f(v);
    }
  )"),
      LifetimesAre({{"f", "(<a, a>, b) -> a"}, {"g", "(<a, a>, b) -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructNoTemplateInnerTemplate) {
  EXPECT_THAT(
      GetLifetimes(R"(
    template <typename T>
    struct X {
      T field;
    };

    struct Y {
     X<int*> field;
    };

    int* target_byref(Y& s) {
      return s.field.field;
    }

    int* target_byvalue(Y s) {
      return s.field.field;
    }
  )"),
      LifetimesContain({{"target_byref", "a -> a"},
                        {"target_byvalue",
                         "ERROR: function returns reference to a local"}}));
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

TEST_F(LifetimeAnalysisTest, DISABLED_StructTemplateReturnPassByValue) {
  // TODO(veluca): disabled because calling a function with a pass-by-value
  // struct is not yet supported -- see TODO in TransferLifetimesForCall.
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      T a;
    };
    S<int*> t(S<int*> s) {
      return s;
    }
    S<int*> target(S<int*> s) {
      return t(s);
    }
  )"),
              LifetimesAre({{"t", "a -> a"}, {"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructWithTemplateArgs) {
  EXPECT_THAT(GetLifetimes(R"(
template <typename T, typename U>
struct S {
  T t;
  U u;
};

int* target(S<int*, int*>* s, int* t, int* u) {
  s->t = t;
  s->u = u;
  return s->t;
}
  )"),
              // With template arguments, now the struct and its fields can
              // have different lifetimes.
              LifetimesAre({{"target", "(<a, b>, c), a, b -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ExampleFromRFC) {
  // This is an example from the lifetimes RFC.
  EXPECT_THAT(GetLifetimes(R"(
template <typename T>
struct R {
  R(T t) : t(t) {}
  T t;
};

bool some_condition();

template <typename T>
struct S {
  S(T a, T b) : r(some_condition() ? R(a) : R(b)) {}
  R<T> r;
};

int* target(int* a, int* b) {
  S<int*> s(a, b);
  return s.r.t;
}
  )"),
              LifetimesContain({{"R<int *>::R", "(a, b): a"},
                                {"S<int *>::S", "(a, b): a, a"},
                                {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, VariadicTemplate) {
  EXPECT_THAT(GetLifetimes(R"(
    template <int idx, typename... Args> struct S {};
    template <int idx, typename T, typename... Args>
    struct S<idx, T, Args...> {
      T t;
      S<idx+1, Args...> nested;
    };

    template <typename... Args>
    struct tuple: public S<0, Args...> {};

    int* target(tuple<int*, int*>& s) {
      return s.nested.t;
    }
  )"),
              LifetimesAre({{"target", "(<a, b>, c) -> b"}}));
}

TEST_F(LifetimeAnalysisTest, DISABLED_VariadicTemplateConstructTrivial) {
  EXPECT_THAT(GetLifetimes(R"(
    template <int idx, typename... Args> struct S {};
    template <int idx, typename T, typename... Args>
    struct S<idx, T, Args...> {
      T t;
      S<idx+1, Args...> nested;
    };

    template <typename... Args>
    struct tuple: public S<0, Args...> {};

    void target(int* a, int* b) {
      tuple<int*, int*> s = {a, b};
    }
  )"),
              LifetimesAre({{"target", "a, b"}}));
}

TEST_F(LifetimeAnalysisTest, VariadicTemplateConstruct) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename... Args> struct S { S() {} };
    template <typename T, typename... Args>
    struct S<T, Args...> {
      T t;
      S<Args...> nested;
      S(T t, Args... args): t(t), nested(args...) {}
    };

    void target(int* a, int* b) {
      S<int*, int*> s = {a, b};
    }
  )"),
              LifetimesContain({{"target", "a, b"}}));
}

// Disabled due to b/253992761.
TEST_F(LifetimeAnalysisTest, DISABLED_NoexceptTemplate) {
  EXPECT_THAT(GetLifetimes(R"(
    template <typename T>
    struct S {
      S() noexcept(isnoexcept<T>()) {}
      template <typename U>
      static constexpr bool isnoexcept() { return true; }
    };

    void f() {
      S<int> s;
    }
  )"),
              LifetimesContain({{"f", ""}}));
}

TEST_F(LifetimeAnalysisTest, TypeTemplateArgAfterNonType) {
  // Minimized repro for a crash from b/228325046.
  EXPECT_THAT(GetLifetimes(R"(
    template<int _Idx, typename _Head>
    struct _Head_base
    {
      constexpr _Head_base(_Head&& __h)
        : _M_head_impl(__h) { }

      _Head _M_head_impl;
    };

    void f() {
      _Head_base<0, void*> head_base(nullptr);
    }
  )"),
              LifetimesContain({{"f", ""}}));
}

TEST_F(LifetimeAnalysisTest,
       TemplateContainingTypedefInstantiatedAnotherTemplate) {
  // Minimized repro for a crash from b/228325046.
  // The scenario that triggered the crash is:
  // - We have a template (in this case `remove_reference`) containing a typedef
  // - That typedef depends on a template parameter
  // - We instantiate the template with an argument that is another template
  // The bug was that we weren't desugaring the typedef and hence coming up with
  // a different value for the depth of the template argument than
  // TemplateTypeParmType::getDepth() uses.
  EXPECT_THAT(GetLifetimes(R"(
    namespace std {
      template <typename T1, typename T2> struct pair {
        T1 t1;
        T2 t2;
      };

      template<typename _Tp>
        struct remove_reference
        { typedef _Tp   type; };

      template<typename _Tp>
        constexpr _Tp&&
        forward(typename std::remove_reference<_Tp>::type& __t) noexcept
        { return static_cast<_Tp&&>(__t); }
    }

    void f() {
      std::pair<int, int> p;
      std::forward<decltype(p)>(p);
    }
  )"),
              LifetimesContain({{"f", ""}}));
}

TEST_F(LifetimeAnalysisTest, DISABLED_ReturnPointerToTemplate) {
  EXPECT_THAT(
      GetLifetimes(R"(
    template <class T> struct S { T t; };
    S<int*>* target(S<int*>* s) {
      return s;
    }
  )"),
      // TODO(b/230456778): This currently erroneously returns (a, b) -> (c, b)
      LifetimesAre({{"f", "(a, b) -> (a, b)"}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests involving inheritance.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST_F(LifetimeAnalysisTest, StructSimpleInheritance) {
  EXPECT_THAT(GetLifetimes(R"(
struct [[clang::annotate("lifetime_params", "a")]] B {
  [[clang::annotate("member_lifetimes", "a")]]
  int* a;
};
struct S : public B {
};
int* target(S* s, int* a) {
  s->a = a;
  return s->a;
}
  )"),
              LifetimesAre({{"target", "(a, b), a -> a"}}));
}

TEST_F(LifetimeAnalysisTest,
       DISABLED_StructInheritanceCallTrivialDefaultConstructor) {
  EXPECT_THAT(GetLifetimes(R"(
    struct T {};
    struct S: public T {
      S(): T() {}
      int* a;
    };
    void target() {
      S s;
    }
  )"),
              LifetimesAre({{"target", ""}}));
}

TEST_F(LifetimeAnalysisTest, StructInheritanceCallBaseConstructor) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] T {
      [[clang::annotate("member_lifetimes", "a")]]
      int* b;
      T(int* b): b(b) {}
    };
    struct S: public T {
      S(int* a, int* b): a(a), T(b) {}
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    int* target(int* a, int* b) {
      S s(a, b);
      return s.b;
    }
  )"),
              LifetimesContain({{"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructInheritanceCallBaseConstructorTypedef) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] T {
      [[clang::annotate("member_lifetimes", "a")]]
      int* b;
      T(int* b): b(b) {}
    };
    using U = T;
    struct S: public U {
      S(int* a, int* b): a(a), T(b) {}
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    int* target(int* a, int* b) {
      S s(a, b);
      return s.b;
    }
  )"),
              LifetimesContain({{"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest,
       StructInheritanceCallBaseConstructorTypedefBaseInit) {
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] T {
      [[clang::annotate("member_lifetimes", "a")]]
      int* b;
      T(int* b): b(b) {}
    };
    using U = T;
    struct S: public T {
      S(int* a, int* b): a(a), U(b) {}
      [[clang::annotate("member_lifetimes", "a")]]
      int* a;
    };
    int* target(int* a, int* b) {
      S s(a, b);
      return s.b;
    }
  )"),
              LifetimesContain({{"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructSimpleInheritanceWithMethod) {
  EXPECT_THAT(
      GetLifetimes(R"(
struct [[clang::annotate("lifetime_params", "a")]] B {
  [[clang::annotate("member_lifetimes", "a")]]
  int* a;
  int* f() { return a; }
};
struct S : public B {
};
int* target(S* s, int* a) {
  s->a = a;
  return s->f();
}
  )"),
      LifetimesAre({{"B::f", "(a, b): -> a"}, {"target", "(a, b), a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructSimpleInheritanceWithMethodInDerived) {
  EXPECT_THAT(
      GetLifetimes(R"(
struct [[clang::annotate("lifetime_params", "a")]] B {
  [[clang::annotate("member_lifetimes", "a")]]
  int* a;
};
struct S : public B {
  int* f() { return a; }
};
int* target(S* s, int* a) {
  s->a = a;
  return s->f();
}
  )"),
      LifetimesAre({{"S::f", "(a, b): -> a"}, {"target", "(a, b), a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructSimpleInheritanceChained) {
  EXPECT_THAT(
      GetLifetimes(R"(
struct [[clang::annotate("lifetime_params", "a")]] A {
  [[clang::annotate("member_lifetimes", "a")]]
  int* a;
};
struct B : public A {
  int* f() { return a; }
};
struct S : public B {
};
int* target(S* s, int* a) {
  s->a = a;
  return s->f();
}
  )"),
      LifetimesAre({{"B::f", "(a, b): -> a"}, {"target", "(a, b), a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructSimpleInheritanceWithSwappedTemplateArgs) {
  // Base and Derived have template arguments where the order is swapped, so
  // if the code reuse the same vector representation for the lifetimes
  // Derived (T, U) for the base class where Base has (U, T) this code fails.
  EXPECT_THAT(GetLifetimes(R"(
template <typename U, typename T>
struct Base {
  T base_t;
  U base_u;
};

template <typename T, typename U>
struct Derived : public Base<U, T> {
  T derived_t;
  U derived_u;
};

int* target(Derived<int*, float*>* d, int* t1, int* t2) {
  d->derived_t = t1;
  d->base_t = t2;
  return d->derived_t;
}
  )"),
              // The lifetime for Derived::derived_t should also be
              // Base::base_t. See discussions at cl/411724984.
              LifetimesAre({{"target", "(<a, b>, c), a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, StructSimpleInheritanceWithDoubledTemplateArgs) {
  // Base and Derived have different number of template arguments.
  // Similar test case as StructSimpleInheritanceWithSwappedTemplateArgs.
  EXPECT_THAT(GetLifetimes(R"(
template <typename T, typename U>
struct Base {
  T base_t;
  U base_u;
};

template <typename T>
struct Derived : public Base<T, T> {
  T derived_t;
};

int* target(Derived<int*>* d, int* t1, int* t2, int* t3) {
  d->derived_t = t1;
  d->base_t = t2;
  d->base_u = t3;
  return d->derived_t;
}
  )"),
              LifetimesAre({{"target", "(a, b), a, a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest,
       StructSimpleInheritanceWithTemplateSubstitutedAndArgs) {
  // Base is a template type and has different number of template arguments from
  // Derived. Similar test case as
  // StructSimpleInheritanceWithSwappedTemplateArgs.
  EXPECT_THAT(GetLifetimes(R"(
template <typename T>
struct Base {
  T base_t;
};

template <typename B, typename T>
struct Derived : public B {
  T derived_t;
};

int* target(Derived<Base<int*>, int*>* d, int* t1, int* t2) {
  d->derived_t = t1;
  d->base_t = t2;
  return d->derived_t;
}
  )"),
              LifetimesAre({{"target", "(<a, b>, c), b, a -> b"}}));
}

TEST_F(LifetimeAnalysisTest, PassDerivedByValue) {
  EXPECT_THAT(GetLifetimes(R"(
struct [[clang::annotate("lifetime_params", "a")]] B {
  [[clang::annotate("member_lifetimes", "a")]]
  int* a;
  int* f() { return a; }
};
struct S : public B {
};
int* target(S s) {
  return s.f();
}
  )"),
              LifetimesAre({{"B::f", "(a, b): -> a"}, {"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, PassDerivedByValue_BaseIsTemplate) {
  EXPECT_THAT(
      GetLifetimes(R"(
template <class T>
struct B {
  T a;
  T f() { return a; }
};
template <class T>
struct S : public B<T> {
};
int* target(S<int *> s) {
  return s.f();
}
  )"),
      LifetimesAre({{"B<int *>::f", "(a, b): -> a"}, {"target", "a -> a"}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

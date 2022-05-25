// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests involving lifetime propagation between virtual functions.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST_F(LifetimeAnalysisTest, WithPureVirtual) {
  EXPECT_THAT(GetLifetimes(R"(
struct Base {
  virtual ~Base() {}
  virtual int* f(int* a) = 0;
};

struct Derived : public Base {
  int* f(int* a) override { return a; }
};
  )"),
              LifetimesContain(
                  {{"Base::f", "b: a -> a"}, {"Derived::f", "b: a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, FunctionVirtualInheritanceWithStatic) {
  EXPECT_THAT(GetLifetimes(R"(
struct Base {
  virtual ~Base() {}
  virtual int* f(int* a) = 0;
};

struct Derived1 : public Base {
  int* f(int* a) override { return a; }
};

struct Derived2 : public Base {
  int* f(int* a) override {
    static int i = 42;
    return &i;
  }
};
  )"),
              LifetimesContain({
                  {"Base::f", "b: a -> a"},
                  {"Derived1::f", "b: a -> a"},
                  {"Derived2::f", "b: a -> static"},
              }));
}

TEST_F(LifetimeAnalysisTest, FunctionVirtualInheritanceWithTwoDeriveds) {
  EXPECT_THAT(GetLifetimes(R"(
struct Base {
  virtual ~Base() {}
  virtual int* f(int* a, int* b) = 0;
};

struct Derived1 : public Base {
  int* f(int* a, int* b) override { return a; }
};

struct Derived2 : public Base {
  int* f(int* a, int* b) override { return b; }
};
  )"),
              LifetimesContain({
                  {"Base::f", "b: a, a -> a"},
                  {"Derived1::f", "c: a, b -> a"},
                  {"Derived2::f", "c: a, b -> b"},
              }));
}

TEST_F(LifetimeAnalysisTest, FunctionVirtualInheritanceWithBaseReturnStatic) {
  EXPECT_THAT(GetLifetimes(R"(
struct Base {
  virtual ~Base() {}
  virtual int* f(int* a) { return a; }
};

struct Derived : public Base {
  int* f(int* a) override {
    static int i = 42;
    return &i;
  }
};
  )"),
              LifetimesContain({
                  {"Base::f", "b: a -> a"},
                  {"Derived::f", "b: a -> static"},
              }));
}

TEST_F(LifetimeAnalysisTest, FunctionVirtualInheritanceChained) {
  EXPECT_THAT(GetLifetimes(R"(
struct Base {
  virtual ~Base() {}
  virtual int* f(int* a) = 0;
};

struct Derived1 : public Base {
  int* f(int* a) override {
    static int i = 42;
    return &i;
  }
};

struct Derived2 : public Derived1 {
  int* f(int* a) override { return a; }
};
  )"),
              LifetimesContain({
                  {"Base::f", "b: a -> a"},
                  {"Derived1::f", "b: a -> a"},
                  {"Derived2::f", "b: a -> a"},
              }));
}

TEST_F(LifetimeAnalysisTest, FunctionVirtualInheritanceWithControlFlow) {
  EXPECT_THAT(GetLifetimes(R"(
struct Base {
  virtual ~Base() {}
  virtual int* f(int* a, int* b) { return a; }
};

struct Derived : public Base {
  int* f(int* a, int* b) override {
    if (*a < *b)
      return a;
    return b;
  }
};
  )"),
              LifetimesContain({
                  {"Base::f", "b: a, a -> a"},
                  {"Derived::f", "b: a, a -> a"},
              }));
}

TEST_F(LifetimeAnalysisTest, FunctionVirtualInheritanceWithLocal) {
  EXPECT_THAT(
      GetLifetimes(R"(
struct Base {
  virtual ~Base() {}
  virtual int* f(int* a) { return a; }
};

struct Derived : public Base {
  int* f(int* a) override {
    int i = 42;
    return &i;
  }
};
  )"),
      LifetimesContain({
          {"Base::f", "b: a -> a"},
          {"Derived::f", "ERROR: function returns reference to a local"},
      }));
}

TEST_F(LifetimeAnalysisTest, FunctionVirtualInheritanceWithStaticPtr) {
  EXPECT_THAT(GetLifetimes(R"(
struct Base {
  virtual ~Base() {}
  virtual void f(int** a) {}
};

struct Derived : public Base {
  void f(int** a) override {
    static int i = 42;
    *a = &i;
  }
};
  )"),
              LifetimesContain({
                  {"Base::f", "b: (static, a)"},
                  {"Derived::f", "b: (static, a)"},
              }));
}

TEST_F(LifetimeAnalysisTest, FunctionVirtualInheritanceWithRecursion) {
  EXPECT_THAT(GetLifetimes(R"(
struct Base {
  virtual ~Base() {}
  virtual int* f(int* a, int* b) { return a; }
};

struct Derived : public Base {
  int* f(int* a, int* b) override {
    if (*a > *b)
      return b;
    *a -= 1;
    return f(a, b);
  }
};
  )"),
              LifetimesContain({
                  {"Base::f", "b: a, a -> a"},
                  {"Derived::f", "c: a, b -> b"},
              }));
}

TEST_F(LifetimeAnalysisTest, FunctionVirtualInheritanceWithExplicitBaseCall) {
  EXPECT_THAT(GetLifetimes(R"(
struct Base {
  virtual ~Base() {}
  virtual int* f(int* a, int* b) { return a; }
};

struct Derived : public Base {
  int* f(int* a, int* b) override {
    if (*a > *b)
      return b;
    return Base::f(a, b);
  }
};
  )"),
              LifetimesContain({
                  {"Base::f", "b: a, a -> a"},
                  {"Derived::f", "b: a, a -> a"},
              }));
}

TEST_F(LifetimeAnalysisTest,
       DISABLED_FunctionVirtualInheritanceWithComplexRecursion) {
  // TODO(kinuko): Fix this. Currently this doesn't work because in
  // AnalyzeFunctionRecursive() the recursion cycle check
  // (FindAndMarkCycleWithFunc) happens before the code expands the possible
  // overrides, and let it return early when it finds f() in Base::f() even if
  // it has overrides. Later in AnalyzeRecursiveFunctions Base::f() is analyzed
  // but it doesn't expand the overrides there. See the TODO in
  // AnalyzeFunctionRecursive.
  EXPECT_THAT(GetLifetimes(R"(
struct Base {
  virtual ~Base() {}
  virtual int* f(int* a, int* b) {
    if (*a > *b)
      return b;
    *a -= 1;
    return f(a, b);
  }
};

struct Derived : public Base {
  int* f(int* a, int* b) override {
    if (*a == *b)
      return a;
    return Base::f(a, b);
  }
};
  )"),
              LifetimesContain({
                  {"Base::f", "b: a, a -> a"},
                  {"Derived::f", "b: a, a -> a"},
              }));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests involving casts.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST_F(LifetimeAnalysisTest, DISABLED_StaticCast) {
  // TODO(veluca): the `Object` we create for the base struct does not know
  // about the derived struct, so this test will fail when trying to access the
  // base on the object of the derived class. See also DynamicCastAccessField.
  EXPECT_THAT(GetLifetimes(R"(
    struct Base {
      virtual bool is_derived() { return false; }
    };
    struct Derived : public Base {
      bool is_derived() override {
        return true;
      }
    };
    Derived* test_static_cast_ptr(Base* base, Derived* derived) {
      if (base->is_derived()) {
        return static_cast<Derived*>(base);
      }
      return derived;
    }
    Derived& test_static_cast_ref(Base& base, Derived& derived) {
      if (base.is_derived()) {
        return static_cast<Derived&>(base);
      }
      return derived;
    }
  )"),
              LifetimesContain({{"test_static_cast_ptr", "a, a -> a"},
                                {"test_static_cast_ref", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, DISABLED_DynamicCastWithFnCall) {
  // TODO(veluca): the `Object` we create for the base struct does not know
  // about the derived struct, so this test will fail when trying to access the
  // base on the object of the derived class. See also DynamicCastAccessField.
  EXPECT_THAT(GetLifetimes(R"(
    struct Base {
      virtual bool is_derived() { return false; }
    };
    struct Derived : public Base {
      bool is_derived() override {
        return true;
      }
    };
    Derived* test_dynamic_cast_ptr(Base* base, Derived* derived) {
      if (Derived* derived_from_base = dynamic_cast<Derived*>(base)) {
        return derived_from_base;
      }
      return derived;
    }
    Derived& test_dynamic_cast_ref(Base& base, Derived& derived) {
      // We don't have support for exceptions enabled, so we can't use
      // dynamic_cast unconditionally and check whether it succeeded or failed
      // by catching std::bad_cast. Instead, we call is_derived() like we do in
      // StaticCast test above. This makes the dynamic_cast somewhat pointless,
      // but at least we can test that we do propagate the points-to set through
      // it correctly.
      if (base.is_derived()) {
        return dynamic_cast<Derived&>(base);
      }
      return derived;
    }
    // Also test that we handle function calls to test_dynamic_cast_...()
    // correctly.  Our logic for function calls should realize that
    // test_dynamic_cast_...() may return not just `derived` but also `base` and
    // that therefore all three lifetimes should be the same.
    Derived* call_dynamic_cast_ptr(Base* base, Derived* derived) {
      return test_dynamic_cast_ptr(base, derived);
    }
    Derived& call_dynamic_cast_ref(Base& base, Derived& derived) {
      return test_dynamic_cast_ref(base, derived);
    }
  )"),
              LifetimesContain({{"test_dynamic_cast_ptr", "a, a -> a"},
                                {"test_dynamic_cast_ref", "a, a -> a"},
                                {"call_dynamic_cast_ptr", "a, a -> a"},
                                {"call_dynamic_cast_ref", "a, a -> a"}}));
}

// TODO(mboehme): This test currently fails when trying to access `derived->a`
// because it can't find the field. This is because we set up the object as a
// `Base` and only gave it the fields that are present on `Base`.
// There are several ways we could resolve this:
// a) When setting up the object initially, proactively give it the fields of
//    all transitive derived classes. This can, however, be very costly if the
//    object type is the base class of a large object hierarchy.
// b) When the object becomes accessible through a pointer to the derived class,
//    add all of the fields of that derived class if they aren't present yet.
// c) When we perform a field access, add the field if it isn't present yet.
// Of these, c) may be the easiest to implement, and it also avoids
// speculatively adding fields to the object that may never be accessed.
TEST_F(LifetimeAnalysisTest, DISABLED_DynamicCastAccessField) {
  EXPECT_THAT(
      GetLifetimes(R"(
    struct Base {
      virtual ~Base() {}
    };
    struct Derived : public Base {
      int* p;
    };
    Derived* DerivedFromBase(Base* base) {
      return dynamic_cast<Derived*>(base);
    }
    int* target(Base* base) {
      if (auto* derived = DerivedFromBase(base)) {
        return derived->p;
      }
      return nullptr;
    }
  )"),
      LifetimesContain({{"DerivedFromBase", "a -> a"}, {"target", "a -> a"}}));
}

// TODO(mboehme): This example demonstrates an issue related to field access on
// derived classes that may be hard to overcome in a principled way. In a
// multi-TU setting, neither the definition of these functions nor that of the
// class Derived need be visible within the TU that contains target().
// Currently, this example fails because SetFieldIfPresent() and
// GetFieldIfPresent() cannot access Derived::p. But even when this is resolved,
// we face these issues:
// - The call to SetFieldIfPresent() is a no-op with respect to the points-to
//   map. Even though `base` and `p` share the same lifetime, the logic for
//   performing function calls doesn't see any object of type `int*` that could
//   be modified by the callee.
// - There is no existing object for GetFieldIfPresent() to return.
// We could fix the second issue by creating a new object and giving it the same
// lifetime as `base` (which we know to do because of the signature of
// GetFieldIfPresent()). However, we would still infer incorrect lifetimes of
// "a, b -> b" for target() because we would not understand that `p` potentially
// gets propagated to the return value of target().
// The alternative function call algorithm that veluca@ is working on might
// resolve this issue and infer correct lifetimes in this case.
TEST_F(LifetimeAnalysisTest, DISABLED_DynamicCastFieldAccessBehindFnCall) {
  EXPECT_THAT(GetLifetimes(R"(
    struct Base {
      virtual ~Base() {}
    };
    struct Derived : public Base {
      int* p;
    };
    void SetFieldIfPresent(Base* base, int* p) {
      if (auto* derived = dynamic_cast<Derived*>(base)) {
        derived->p = p;
      }
    }
    int* GetFieldIfPresent(Base* base) {
      if (auto* derived = dynamic_cast<Derived*>(base)) {
        return derived->p;
      }
      return nullptr;
    }
    int* target(Base* base, int* p) {
      SetFieldIfPresent(base, p);
      return GetFieldIfPresent(base);
    }
  )"),
              LifetimesContain({{"SetFieldIfPresent", "a, a"},
                                {"GetFieldIfPresent", "a -> a"},
                                {"target", "a, a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, ReinterpretCastPtr) {
  EXPECT_THAT(
      GetLifetimes(R"(
    double* target(int* p) {
      return reinterpret_cast<double*>(p);
    }
  )"),
      LifetimesAre({{"target", "ERROR: type-unsafe cast prevents analysis"}}));
}

TEST_F(LifetimeAnalysisTest, ReinterpretCastRef) {
  EXPECT_THAT(
      GetLifetimes(R"(
    double& target(int& p) {
      return reinterpret_cast<double&>(p);
    }
  )"),
      LifetimesAre({{"target", "ERROR: type-unsafe cast prevents analysis"}}));
}

TEST_F(LifetimeAnalysisTest, IntegralToPointerCast) {
  EXPECT_THAT(
      GetLifetimes(R"(
    // We want to avoid including <cstdint>, so just assume `long long` is big
    // enough to hold a pointer.
    int* target(long long i) {
      return reinterpret_cast<int*>(i);
    }
  )"),
      LifetimesAre({{"target", "ERROR: type-unsafe cast prevents analysis"}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

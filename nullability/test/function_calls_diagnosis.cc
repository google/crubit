// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for diagnosis on function calls.

#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, CallExprParamAssignment) {
  // free function with single param
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void takeNonnull(int *_Nonnull);
    void takeNullable(int *_Nullable);
    void takeUnannotated(int *);
    void target(int *_Nonnull ptr_nonnull, int *_Nullable ptr_nullable,
                int *ptr_unannotated) {
      takeNonnull(nullptr);  // [[unsafe]]
      takeNonnull(ptr_nonnull);
      takeNonnull(ptr_nullable);  // [[unsafe]]
      takeNonnull(ptr_unannotated);

      takeNullable(nullptr);
      takeNullable(ptr_nonnull);
      takeNullable(ptr_nullable);
      takeNullable(ptr_unannotated);

      takeUnannotated(nullptr);
      takeUnannotated(ptr_nonnull);
      takeUnannotated(ptr_nullable);
      takeUnannotated(ptr_unannotated);
    }
  )cc"));

  // free function with multiple params of mixed nullability
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void takeMixed(int *, int *_Nullable, int *_Nonnull);
    void target() {
      takeMixed(nullptr, nullptr, nullptr);  // [[unsafe]]
    }
  )cc"));

  // member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      void takeNonnull(int *_Nonnull);
      void takeNullable(int *_Nullable);
      void takeUnannotated(int *);
    };
    void target(Foo foo) {
      foo.takeNonnull(nullptr);  // [[unsafe]]
      foo.takeNullable(nullptr);
      foo.takeUnannotated(nullptr);
    }
  )cc"));

  // function pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(void (*takeNonnull)(int *_Nonnull),
                void (*takeNullable)(int *_Nullable),
                void (*takeUnannotated)(int *)) {
      takeNonnull(nullptr);  // [[unsafe]]
      takeNullable(nullptr);
      takeUnannotated(nullptr);
    }
  )cc"));

  // pointer to function pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(void (**takeNonnull)(int *_Nonnull),
                void (**takeNullable)(int *_Nullable),
                void (**takeUnannotated)(int *)) {
      (*takeNonnull)(nullptr);  // [[unsafe]]
      (*takeNullable)(nullptr);
      (*takeUnannotated)(nullptr);
    }
  )cc"));

  // function returned from function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    typedef void (*takeNonnullF)(int *_Nonnull);
    typedef void (*takeNullableF)(int *_Nullable);
    typedef void (*takeUnannotatedF)(int *);
    void target(takeNonnullF (*takeNonnull)(), takeNullableF (*takeNullable)(),
                takeUnannotatedF (*takeUnannotated)()) {
      (*takeNonnull)()(nullptr);  // [[unsafe]]
      (*takeNullable)()(nullptr);
      (*takeUnannotated)()(nullptr);
    }
  )cc"));

  // passing a reference to a nonnull pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void takeNonnullRef(int* _Nonnull&);
    void takeNullableRef(int* _Nullable&);
    void takeUnannotatedRef(int*&);
    void target(int* _Nonnull ptr_nonnull) {
      takeNonnullRef(ptr_nonnull);
      *ptr_nonnull;

      takeNullableRef(ptr_nonnull);  // [[unsafe]]
      *ptr_nonnull;

      takeUnannotatedRef(ptr_nonnull);
      *ptr_nonnull;
    }
  )cc"));

  // passing a reference to a nullable pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void takeNonnullRef(int *_Nonnull &);
    void takeNullableRef(int *_Nullable &);
    void takeUnannotatedRef(int *&);
    void target(int *_Nullable ptr_nullable) {
      takeNonnullRef(ptr_nullable);  // [[unsafe]]
      *ptr_nullable;                 // [[unsafe]]

      takeNullableRef(ptr_nullable);
      *ptr_nullable;  // [[unsafe]]

      takeUnannotatedRef(ptr_nullable);
      *ptr_nullable;
    }
  )cc"));

  // passing a reference to an unannotated pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void takeNonnullRef(int* _Nonnull&);
    void takeNullableRef(int* _Nullable&);
    void takeUnannotatedRef(int*&);
    void target(int* ptr_unannotated) {
      int* UnannotatedNullValue = nullptr;
      takeNonnullRef(UnannotatedNullValue);  // [[unsafe]]

      takeNonnullRef(ptr_unannotated);
      *ptr_unannotated;

      takeNullableRef(ptr_unannotated);
      *ptr_unannotated;  // false-negative? The unannotated pointer could be
                         // considered nullable if it has been used as a
                         // nullable pointer.

      takeUnannotatedRef(ptr_unannotated);
      *ptr_unannotated;
    }
  )cc"));
}

// Test that relevant diagnostics are produced for declarations with templated
// annotations.
TEST(PointerNullabilityTest, CallExprParamAssignmentTemplateAnnotations) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T>
    using Nonnull = T _Nonnull;
    template <typename T>
    using Nullable = T _Nullable;
    template <typename T>
    using NullabilityUnknown = T;

    void takeNonnull(Nonnull<int *>);
    void takeNullable(Nullable<int *>);
    void takeUnknown(NullabilityUnknown<int *>);

    void target(Nonnull<int *> ptr_nonnull, Nullable<int *> ptr_nullable,
                NullabilityUnknown<int *> ptr_unknown) {
      takeNonnull(nullptr);  // [[unsafe]]
      takeNonnull(ptr_nonnull);
      takeNonnull(ptr_nullable);  // [[unsafe]]
      takeNonnull(ptr_unknown);

      takeNullable(nullptr);
      takeNullable(ptr_nonnull);
      takeNullable(ptr_nullable);
      takeNullable(ptr_unknown);

      takeUnknown(nullptr);
      takeUnknown(ptr_nonnull);
      takeUnknown(ptr_nullable);
      takeUnknown(ptr_unknown);
    }
  )cc"));
}

// Test that templated annotations work interchangeably, in diagnosis, with the
// built-in Clang annotations.
TEST(PointerNullabilityTest, CallExprParamAssignmentTemplateBuiltinMixed) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T>
    using Nonnull = T _Nonnull;
    template <typename T>
    using Nullable = T _Nullable;
    template <typename T>
    using NullabilityUnknown = T;

    void takeNonnull(int *_Nonnull);
    void takeNullable(int *_Nullable);
    void takeUnannotated(int *);

    void target(Nonnull<int *> ptr_nonnull, Nullable<int *> ptr_nullable,
                NullabilityUnknown<int *> ptr_unknown) {
      takeNonnull(ptr_nonnull);
      takeNonnull(ptr_nullable);  // [[unsafe]]
      takeNonnull(ptr_unknown);

      takeNullable(ptr_nonnull);
      takeNullable(ptr_nullable);
      takeNullable(ptr_unknown);

      takeUnannotated(ptr_nonnull);
      takeUnannotated(ptr_nullable);
      takeUnannotated(ptr_unknown);
    }
  )cc"));
}

TEST(PointerNullabilityTest, CallExprMultiNonnullParams) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void take(int *_Nonnull, int *_Nullable, int *_Nonnull);
    void target() {
      take(nullptr,  // [[unsafe]]
           nullptr,
           nullptr);  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, CallVariadicFunction) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void variadic(int *_Nonnull, ...);
    void target() {
      int i = 0;
      variadic(&i, nullptr, &i);
      variadic(nullptr, nullptr, &i);  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, CallVariadicConstructor) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      S(int* _Nonnull, ...);
    };
    void target() {
      int i = 0;
      S(&i, nullptr, &i);
      S(nullptr, nullptr, &i);  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, CallMemberOperatorOneParam) {
  // overloaded operator with single param
  EXPECT_TRUE(checkDiagnostics(R"cc(
    // map<int * _Nonnull, int>
    struct MapWithNonnullKeys {
      int &operator[](int *_Nonnull key);
    };
    // map<int * _Nullable, int>
    struct MapWithNullableKeys {
      int &operator[](int *_Nullable key);
    };
    // map<int *, int>
    struct MapWithUnannotatedKeys {
      int &operator[](int *key);
    };
    void target(int *_Nonnull ptr_nonnull, int *_Nullable ptr_nullable,
                int *ptr_unannotated) {
      MapWithNonnullKeys nonnull_keys;
      nonnull_keys[nullptr] = 42;  // [[unsafe]]
      nonnull_keys[ptr_nonnull] = 42;
      nonnull_keys[ptr_nullable] = 42;  // [[unsafe]]
      nonnull_keys[ptr_unannotated] = 42;

      MapWithNullableKeys nullable_keys;
      nullable_keys[nullptr] = 42;
      nullable_keys[ptr_nonnull] = 42;
      nullable_keys[ptr_nullable] = 42;
      nullable_keys[ptr_unannotated] = 42;

      MapWithUnannotatedKeys unannotated_keys;
      unannotated_keys[nullptr] = 42;
      unannotated_keys[ptr_nonnull] = 42;
      unannotated_keys[ptr_nullable] = 42;
      unannotated_keys[ptr_unannotated] = 42;
    }
  )cc"));
}

TEST(PointerNullabilityTest, CallMemberOperatorMultipleParams) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct TakeMixed {
      void operator()(int *, int *_Nullable, int *_Nonnull);
    };
    void target() {
      TakeMixed takeMixed;
      takeMixed(nullptr, nullptr, nullptr);  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, CallNoreturnDestructor) {
  // This test demonstrates that the check considers program execution to end
  // when a `noreturn` destructor is called.
  // Among other things, this is intended to demonstrate that Abseil's logging
  // instruction `LOG(FATAL)` (which creates an object with a `noreturn`
  // destructor) is correctly interpreted as terminating the program.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Fatal {
      __attribute__((noreturn)) ~Fatal();
      void method(int);
    };
    void target(int* _Nullable p) {
      // Do warn here, as the `*p` dereference happens before the `Fatal` object
      // is destroyed.
      Fatal().method(*p);  // [[unsafe]]
      // Don't warn here: We know that this code never gets executed.
      *p;
    }
  )cc"));
}

// This is a crash repro.
TEST(PointerNullabilityTest, NonConstMethodClearsPointerMembersInExpr) {
  EXPECT_TRUE(checkDiagnosticsHasUntracked(R"cc(
    void f(char* _Nonnull const&, char* const&);

    template <class T>
    struct S {
      void target() {
        // This used to cause a crash because of a very specific sequence of
        // events:
        // - We visit `p` and initialize its nullability properties.
        // - We visit `returnsPtr()`, causing us to reset all pointer-type
        //   fields (in this case, `p`). When we did this, we used to create
        //   fresh `PointerValue`s for the fields, but without nullability
        //   properties. This would cause a crash in the next step (see below).
        //   (Instead, we now simply clear the values associated with the
        //   fields.)
        // - We visit the function call and check that `p` is non-null, which
        //   used to crash because `p` had a `PointerValue` associated with it
        //   that didn't have nullability properties.
        // Diagnosis produces a "pointer value not modeled" warning on this line
        // because the value for `p` has been cleared. (This only happens if
        // `S` is a template, which is why `S` has a template parameter that is
        // otherwise unused.)
        f(p, returnsPtr());  // [[unsafe]]
      }

      char* returnsPtr();

      char* p;
    };

    template class S<int>;
  )cc"));
}

TEST(SmartPointerTest, JoinCausesLossOfNullabilityPropertiesAtExit) {
  // This is a regression test for the crash seen in b/414348238.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    class A {
     public:
      void target(bool b) {
        if (b) return;
        // Calling a non-const member function clears out the framework-produced
        // value for `p_` but (crucially) without mentioning `p_` in an
        // expression (which would initialize the nullability properties for the
        // previous value of `p_`).
        non_const_member_fn();
        // We now mention `p_` in an expression to make sure the nullability
        // properties for `p_` are initialized.
        p_;
        // At function exit, we see the joined state from two blocks:
        // - The block containing the return statement above. Here, `p_` has the
        //   value that the framework initialized it with. Because we never
        //   saw the expression `p_` on this path, we didn't initialize the
        //   nullability properties for this value.
        // - The block that follows the if statement. Here, `p_` has a different
        //   value, which is associated with nullablility properties, as
        //   explained above.
        // When we join these two values, because one of them does not have
        // nullability properties, we also don't associate nullability
        // properties with the joined value. It's important to test for this;
        // our failure to do this previously resulted in the crash.
      }

      void non_const_member_fn();

     private:
      int* _Nonnull p_;
    };
  )cc"));
}

TEST(PointerNullabilityTest, Accessor_BaseObjectReturnedByReference) {
  // Crash repro:
  // If the base object of the accessor call expression is a reference returned
  // from a function call, we have a storage location for the object but no
  // values for its fields. Check that we don't crash in this case.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct C {
      int *_Nullable property() const { return x; }
      int *_Nullable x = nullptr;
    };
    C &foo();
    void target() {
      if (foo().property() != nullptr) int x = *foo().property();  // [[unsafe]]
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability

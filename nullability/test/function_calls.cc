// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for function calls.

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, CallExprWithPointerReturnType) {
  // free function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull makeNonnull();
    int *_Nullable makeNullable();
    int *makeUnannotated();
    void target() {
      *makeNonnull();
      *makeNullable();  // [[unsafe]]
      *makeUnannotated();
    }
  )cc"));

  // member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      int *_Nonnull makeNonnull();
      int *_Nullable makeNullable();
      int *makeUnannotated();
    };
    void target(Foo foo) {
      *foo.makeNonnull();
      *foo.makeNullable();  // [[unsafe]]
      *foo.makeUnannotated();
    }
  )cc"));

  // function pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull (*makeNonnull)(),
                int *_Nullable (*makeNullable)(), int *(*makeUnannotated)()) {
      *makeNonnull();
      *makeNullable();  // [[unsafe]]
      *makeUnannotated();
    }
  )cc"));

  // pointer to function pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull (**makeNonnull)(),
                int *_Nullable (**makeNullable)(), int *(**makeUnannotated)()) {
      *(*makeNonnull)();
      *(*makeNullable)();  // [[unsafe]]
      *(*makeUnannotated)();
    }
  )cc"));

  // function returning a function pointer which returns a pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    typedef int *_Nonnull (*MakeNonnullT)();
    typedef int *_Nullable (*MakeNullableT)();
    typedef int *(*MakeUnannotatedT)();
    void target(MakeNonnullT (*makeNonnull)(), MakeNullableT (*makeNullable)(),
                MakeUnannotatedT (*makeUnannotated)()) {
      *(*makeNonnull)()();
      *(*makeNullable)()();  // [[unsafe]]
      *(*makeUnannotated)()();
    }
  )cc"));

  // free function returns reference to pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull &makeNonnull();
    int *_Nullable &makeNullable();
    int *&makeUnannotated();
    void target() {
      *makeNonnull();
      *makeNullable();  // [[unsafe]]
      *makeUnannotated();

      // Check that we can take the address of the returned reference and still
      // see the correct nullability "behind" the resulting pointer.
      __assert_nullability<NK_nonnull, NK_nonnull>(&makeNonnull());
      __assert_nullability<NK_nonnull, NK_nullable>(&makeNullable());
      __assert_nullability<NK_nonnull, NK_unspecified>(&makeUnannotated());
    }
  )cc"));

  // function called in loop
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nullable makeNullable();
    bool makeBool();
    void target() {
      bool first = true;
      while (true) {
        int *x = makeNullable();
        if (first && x == nullptr) return;
        first = false;
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

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
  //
  // TODO(b/233582219): Fix false negative. Implement support for retrieving
  // parameter types from a pointer to function pointer.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(void (**takeNonnull)(int *_Nonnull),
                void (**takeNullable)(int *_Nullable),
                void (**takeUnannotated)(int *)) {
      (*takeNonnull)(nullptr);  // false-negative
      (*takeNullable)(nullptr);
      (*takeUnannotated)(nullptr);
    }
  )cc"));

  // function returned from function
  //
  // TODO(b/233582219): Fix false negative. Implement support for retrieving
  // parameter types for functions returned by another function.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    typedef void (*takeNonnullF)(int *_Nonnull);
    typedef void (*takeNullableF)(int *_Nullable);
    typedef void (*takeUnannotatedF)(int *);
    void target(takeNonnullF (*takeNonnull)(), takeNullableF (*takeNullable)(),
                takeUnannotatedF (*takeUnannotated)()) {
      (*takeNonnull)()(nullptr);  // false-negative
      (*takeNullable)()(nullptr);
      (*takeUnannotated)()(nullptr);
    }
  )cc"));

  // passing a reference to a nonnull pointer
  //
  // TODO(b/233582219): Fix false negative. When the nonnull pointer is passed
  // by reference into the callee which takes a nullable parameter, its value
  // may be changed to null, making it unsafe to dereference when we return from
  // the function call. Some possible approaches for handling this case:
  // (1) Disallow passing a nonnull pointer as a nullable reference - and warn
  // at the function call.
  // (2) Assume in worst case the nonnull pointer becomes nullable after the
  // call - and warn at the dereference.
  // (3) Sacrifice soundness for reduction in noise, and skip the warning.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void takeNonnullRef(int *_Nonnull &);
    void takeNullableRef(int *_Nullable &);
    void takeUnannotatedRef(int *&);
    void target(int *_Nonnull ptr_nonnull) {
      takeNonnullRef(ptr_nonnull);
      *ptr_nonnull;

      // false-negative
      takeNullableRef(ptr_nonnull);
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
      *ptr_nullable;  // [[unsafe]]
    }
  )cc"));

  // passing a reference to an unannotated pointer
  //
  // TODO(b/233582219): Fix false negative. The unannotated pointer should be
  // considered nullable if it has been used as a nullable pointer.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void takeNonnullRef(int *_Nonnull &);
    void takeNullableRef(int *_Nullable &);
    void takeUnannotatedRef(int *&);
    void target(int *ptr_unannotated) {
      takeNonnullRef(ptr_unannotated);
      *ptr_unannotated;

      takeNullableRef(ptr_unannotated);
      *ptr_unannotated;  // false-negative

      takeUnannotatedRef(ptr_unannotated);
      *ptr_unannotated;
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

TEST(PointerNullabilityTest, CanOverwritePtrWithPtrCreatedFromRefReturnType) {
  // Test that if we create a pointer from a function returning a reference, we
  // can use that pointer to overwrite an existing nullable pointer and make it
  // nonnull.

  EXPECT_TRUE(checkDiagnostics(R"cc(
    int &get_int();

    void target(int *_Nullable i) {
      i = &get_int();
      *i;
    }
  )cc"));
}

TEST(PointerNullabilityTest, CanOverwritePtrWithPtrReturnedByFunction) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull get_int();

    void target(int *_Nullable i) {
      i = get_int();
      *i;
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

TEST(PointerNullabilityTest, CallMemberOperatorNoParams) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct MakeNonnull {
      int *_Nonnull operator()();
    };
    struct MakeNullable {
      int *_Nullable operator()();
    };
    struct MakeUnannotated {
      int *operator()();
    };
    void target() {
      MakeNonnull makeNonnull;
      *makeNonnull();

      MakeNullable makeNullable;
      *makeNullable();  // [[unsafe]]

      MakeUnannotated makeUnannotated;
      *makeUnannotated();
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

TEST(PointerNullabilityTest, CallFreeOperator) {
  // No nullability involved. This is just a regression test to make sure we can
  // process a call to a free overloaded operator.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct A {};
    A operator+(A, A);
    void target() {
      A a;
      a = a + a;
    }
  )cc"));
}

// Check that we distinguish between the nullability of the return type and
// parameters.
TEST(PointerNullabilityTest, DistinguishFunctionReturnTypeAndParams) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nullable callee(int *_Nonnull);

    void target() {
      int i = 0;
      __assert_nullability<NK_nullable>(callee(&i));
    }
  )cc"));
}

TEST(PointerNullabilityTest, DistinguishMethodReturnTypeAndParams) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int *_Nullable callee(int *_Nonnull);
    };

    void target(S s) {
      int i = 0;
      __assert_nullability<NK_nullable>(s.callee(&i));
    }
  )cc"));
}

TEST(PointerNullabilityTest,
     ClassTemplate_DistinguishMethodReturnTypeAndParams) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct S {
      T0 callee(T1);
    };

    void target(S<int *_Nullable, int *_Nonnull> s) {
      int i = 0;
      __assert_nullability<NK_nullable>(s.callee(&i));
    }
  )cc"));
}

TEST(PointerNullabilityTest,
     CallFunctionTemplate_TemplateArgInReturnTypeHasNullTypeSourceInfo) {
  // This test sets up a function call where we don't have a `TypeSourceInfo`
  // for the argument to a template parameter used in the return type.
  // This is a regression test for a crash that we observed on real-world code.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <class T>
    struct A {
      using Type = T;
    };
    template <int, class T>
    typename A<T>::Type f(T);
    void target() { f<0>(1); }
  )cc"));
}

TEST(PointerNullabilityTest, CallFunctionTemplate_PartiallyDeduced) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <int, class T>
    T f(T);
    void target() { f<0>(1); }
  )cc"));
}

TEST(PointerNullabilityTest, CallBuiltinFunction) {
  // Crash repro.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() { __builtin_operator_new(0); }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <optional>
#include <set>
#include <string>

#include "nullability_verification/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
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

  // free function with multiple params of mixed nullability
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void takeMixed(int *, int *_Nullable, int *_Nonnull);
    void target() {
      takeMixed(nullptr, nullptr, nullptr);  // [[unsafe]]
    }
  )cc"));

  // overloaded operator with multiple params of mixed nullability
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct TakeMixed {
      void operator()(int *, int *_Nullable, int *_Nonnull);
    };
    void target() {
      TakeMixed takeMixed;
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

TEST(PointerNullabilityTest, ReturnStatements) {
  // nonnull return type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nonnull target() {
      return nullptr;  // [[unsafe]]
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nonnull target(int* _Nonnull ptr_nonnull) {
      return ptr_nonnull;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nonnull target(int* _Nullable ptr_nullable) {
      return ptr_nullable;  // [[unsafe]]
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull target(int *ptr_unannotated) {
      return ptr_unannotated;
    }
  )cc"));

  // nullable return type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable target() { return nullptr; }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable target(int* _Nonnull ptr_nonnull) {
      return ptr_nonnull;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable target(int* _Nullable ptr_nullable) {
      return ptr_nullable;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nullable target(int *ptr_unannotated) {
      return ptr_unannotated;
    }
  )cc"));

  // unannotated return type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* target() { return nullptr; }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* target(int* _Nonnull ptr_nonnull) {
      return ptr_nonnull;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* target(int* _Nullable ptr_nullable) {
      return ptr_nullable;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *target(int *ptr_unannotated) {
      return ptr_unannotated;
    }
  )cc"));

  // multiple return statements
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nonnull target(bool b, int* _Nonnull ptr_nonnull) {
      if (b) {
        return nullptr;  // [[unsafe]]
      }
      return ptr_nonnull;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nonnull target(int* _Nullable ptr_nullable,
                         int* _Nonnull ptr_nonnull) {
      if (ptr_nullable) {
        return ptr_nullable;
      }
      return ptr_nonnull;
    }
  )cc"));
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nonnull target(int* _Nullable ptr_nullable_1,
                         int* _Nullable ptr_nullable_2) {
      if (ptr_nullable_1) {
        return ptr_nullable_2;  // [[unsafe]]
      }
      return ptr_nullable_1;  // [[unsafe]]
    }
  )cc"));

  // return result of merging 2 pointer values
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull target(bool b, int i) {
      int *ptr;
      if (b) {
        ptr = &i;
      } else {
        ptr = nullptr;
      }
      return ptr;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, ConstructExpr) {
  // Constructor call assigned to local variable.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct TakeNonnull {
      explicit TakeNonnull(int *_Nonnull) {}
    };
    struct TakeNullable {
      explicit TakeNullable(int *_Nullable) {}
    };
    struct TakeUnannotated {
      explicit TakeUnannotated(int *) {}
    };
    int *_Nonnull makeNonnull();
    int *_Nullable makeNullable();
    int *makeUnannotated();
    void target() {
      auto NN1 = TakeNonnull(makeNonnull());
      auto NN2 = TakeNonnull(makeNullable());  // [[unsafe]]
      auto NN3 = TakeNonnull(makeUnannotated());

      auto NB1 = TakeNullable(makeNonnull());
      auto NB2 = TakeNullable(makeNullable());
      auto NB3 = TakeNullable(makeUnannotated());

      auto UN1 = TakeUnannotated(makeNonnull());
      auto UN2 = TakeUnannotated(makeNullable());
      auto UN3 = TakeUnannotated(makeUnannotated());
    }
  )cc"));

  // Constructor call in a base initializer.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct TakeNonnull {
      explicit TakeNonnull(int* _Nonnull);
    };
    struct target : TakeNonnull {
      target(int* _Nullable ptr_nullable)  // Forced line break.
          : TakeNonnull(ptr_nullable)      // [[unsafe]]
      {}
    };
  )cc"));

  // Call to a delegating constructor.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable makeNullable();
    struct target {
      target(int* _Nonnull);
      target()                      // Forced line break.
          : target(makeNullable())  // [[unsafe]]
      {}
    };
  )cc"));
}

TEST(PointerNullabilityTest, ConstructorMemberInitializer) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable makeNullable();
    struct target {
      int* _Nonnull ptr_nonnull;
      int* _Nullable ptr_nullable;
      int* ptr_unannotated;
      target()
          : ptr_nonnull(makeNullable()),  // [[unsafe]]
            ptr_nullable(makeNullable()),
            ptr_unannotated(makeNullable()) {}
    };
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nonnull makeNonnull();
    struct target {
      int* _Nonnull ptr_nonnull;
      int* _Nullable ptr_nullable;
      int* ptr_unannotated;
      target()
          : ptr_nonnull(makeNonnull()),
            ptr_nullable(makeNonnull()),
            ptr_unannotated(makeNonnull()) {}
    };
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *makeUnannotated();
    struct target {
      int *_Nonnull ptr_nonnull;
      int *_Nullable ptr_nullable;
      int *ptr_unannotated;
      target()
          : ptr_nonnull(makeUnannotated()),
            ptr_nullable(makeUnannotated()),
            ptr_unannotated(makeUnannotated()) {}
    };
  )cc"));
}

TEST(PointerNullabilityTest, AssertNullability) {
  // Concrete struct.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct StructNonnullNullable {
      int* _Nonnull nonnull;
      int* _Nullable nullable;
    };

    void target(StructNonnullNullable p) {
      __assert_nullability<>(p);
      __assert_nullability<NK_nonnull>(p);                   // [[unsafe]]
      __assert_nullability<NK_nullable>(p);                  // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_nullable>(p);      // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_unspecified>(p);   // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_nonnull>(p);       // [[unsafe]]
      __assert_nullability<NK_nullable, NK_nullable>(p);     // [[unsafe]]
      __assert_nullability<NK_unspecified, NK_nullable>(p);  // [[unsafe]]
    }
  )cc"));

  // Struct with two template type parameters.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {};

    void target(Struct2Arg<int *, int *_Nullable> p) {
      __assert_nullability<NK_unspecified>(p);  // [[unsafe]]
      __assert_nullability<NK_nullable>(p);     // [[unsafe]]

      __assert_nullability<NK_unspecified, NK_nonnull>(p);  // [[unsafe]]
      __assert_nullability<NK_unspecified, NK_nullable>(p);
      __assert_nullability<NK_unspecified, NK_unspecified>(p);  // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_nullable>(p);         // [[unsafe]]
      __assert_nullability<NK_nullable, NK_nullable>(p);        // [[unsafe]]

      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_unspecified>(p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable>(p);
    }
  )cc"));

  // Struct with one type and non-type template parameters.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <int I0, typename T1, typename T2>
    struct Struct3ArgWithInt {};

    void target(Struct3ArgWithInt<2147483647, int* _Nullable, int* _Nonnull> p) {
      __assert_nullability<>(p);             // [[unsafe]]
      __assert_nullability<NK_nonnull>(p);   // [[unsafe]]
      __assert_nullability<NK_nullable>(p);  // [[unsafe]]

      __assert_nullability<NK_unspecified, NK_nonnull>(p);  // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_nonnull>(p);      // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_nullable>(p);     // [[unsafe]]
      __assert_nullability<NK_nullable, NK_nonnull>(p);
      __assert_nullability<NK_nullable, NK_nullable>(p);     // [[unsafe]]
      __assert_nullability<NK_nullable, NK_unspecified>(p);  // [[unsafe]]

      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nonnull>(p);
    }
  )cc"));

  // Nested template arguments.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {};

    void target(
        Struct2Arg<Struct2Arg<int *, int *_Nullable>,
                   Struct2Arg<Struct2Arg<int *_Nullable, int *_Nonnull>,
                              Struct2Arg<int *_Nullable, int *_Nullable>>>
            p) {
      __assert_nullability<>(p);  // [[unsafe]]

      __assert_nullability<NK_unspecified, NK_nullable, NK_nullable, NK_nonnull,
                           NK_nullable, NK_nullable>(p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable, NK_nonnull, NK_nullable,
           NK_nullable, NK_nullable>(p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable, NK_nonnull, NK_nullable>(
              p);
    }
  )cc"));

  // Struct with two template parameters substituted with concrete structs.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct StructUnknownNullable {
      int* unknown;
      int* _Nullable nullable;
    };

    struct StructNullableNonnull {
      int* _Nullable nullable;
      int* _Nonnull nonnull;
    };

    template <typename T1, typename T2>
    struct Struct2Arg {};

    void target(Struct2Arg<StructUnknownNullable, StructNullableNonnull> p) {
      __assert_nullability<>(p);

      __assert_nullability<NK_unspecified, NK_nullable>(p);  // [[unsafe]]

      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable>(p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable, NK_nonnull>(p);
      __assert_nullability  // [[unsafe]]
          <NK_nonnull, NK_nullable, NK_nullable, NK_nonnull>(p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable, NK_nonnull,
           NK_unspecified>(p);
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {
      T0 arg0;
      T1 arg1;

      T0 getT0();
      T1 getT1();
    };

    void target(
        Struct2Arg<Struct2Arg<int *, int *_Nullable>,
                   Struct2Arg<Struct2Arg<int *_Nullable, int *_Nonnull>,
                              Struct2Arg<int *_Nullable, int *_Nullable>>>
            p) {
      __assert_nullability<NK_unspecified, NK_nullable, NK_nullable, NK_nonnull,
                           NK_nullable, NK_nullable>(p);
      __assert_nullability<NK_unspecified, NK_nullable>(p.arg0);
      __assert_nullability<NK_unspecified>(p.arg0.arg0);
      __assert_nullability<NK_nullable>(p.arg0.arg1);
      __assert_nullability<NK_nullable, NK_nonnull, NK_nullable, NK_nullable>(
          p.arg1);
      __assert_nullability<NK_nullable, NK_nonnull>(p.arg1.arg0);
      __assert_nullability<NK_nullable>(p.arg1.arg0.arg0);
      __assert_nullability<NK_nonnull>(p.arg1.arg0.arg1);
      __assert_nullability<NK_nullable, NK_nullable>(p.arg1.arg1);
      __assert_nullability<NK_nullable>(p.arg1.arg1.arg0);
      __assert_nullability<NK_nullable>(p.arg1.arg1.arg1);

      __assert_nullability<>(p.arg0.arg0);           // [[unsafe]]
      __assert_nullability<NK_unspecified>(p.arg0);  // [[unsafe]]
      __assert_nullability                           // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nonnull, NK_nullable, NK_nullable>(
              p.arg1);

      __assert_nullability<NK_unspecified, NK_nullable>(p.getT0());
      __assert_nullability<NK_nonnull>(p.getT1().getT0().getT1());

      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_unspecified>(p.getT0());
      __assert_nullability  // [[unsafe]]
          <NK_unspecified>(p.getT0());

      __assert_nullability<NK_nonnull>(p.getT1().arg0.getT1());
      __assert_nullability<NK_nonnull>(p.arg1.getT0().arg1);
      __assert_nullability<NK_nonnull>(p.arg1.arg0.arg1);

      __assert_nullability  // [[unsafe]]
          <>(p.getT1().getT0().getT1());
      __assert_nullability  // [[unsafe]]
          <NK_nonnull, NK_nonnull>(p.arg1.getT0().arg1);
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable p, int* _Nonnull q, int* r) {
      __assert_nullability<NK_nonnull, NK_nullable>(&p);
      __assert_nullability<NK_nonnull, NK_nonnull>(&q);
      __assert_nullability<NK_nonnull>(&*p);  // [[unsafe]]
      __assert_nullability<NK_nonnull>(&*q);
      __assert_nullability<NK_nonnull>(&*r);
    }
  )cc"));
}

TEST(PointerNullabilityTest, CastExpression) {
  // TODO: We currently do not warn on local variables
  // whose annotations conflict with the initializer. Decide whether to do so,
  // and then treat static casts in an equivalent manner.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable p) {
      static_cast<int *_Nonnull>(p);  // TODO: To warn, or not to warn, that is
                                      // the question.
      static_cast<int *>(p);
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <int I0, typename T1, typename T2>
    struct Struct3Arg {
      T1 arg1;
      T2 arg2;
    };

    void target(Struct3Arg<1, int *_Nullable, int *> &p) {
      *static_cast<const Struct3Arg<1, int *, int *> &>(p).arg1;  // [[unsafe]]
      *static_cast<const Struct3Arg<1, int *, int *> &>(p).arg2;
      *static_cast<int *>(p.arg1);  // [[unsafe]]
      *static_cast<int *>(p.arg2);
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Base {};
    struct Derived : public Base {};

    void target(Derived *_Nullable x, Derived *_Nonnull y) {
      *static_cast<Base *>(x);  // [[unsafe]]
      *static_cast<Base *>(y);  // [[unsafe]] TODO: Fix false positive.
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <int I0, typename T1, typename T2>
    struct Struct3Arg {
      T1 arg1;
      T2 arg2;
    };

    void target(Struct3Arg<1, int *_Nullable, int *> &p) {
      *((const Struct3Arg<1, int *, int *> &)p).arg1;  // [[unsafe]]
      *((const Struct3Arg<1, int *, int *> &)p)        // [[unsafe]]
           .arg2;        // TODO: Fix false positive.
      *(int *)p.arg1;    // [[unsafe]]
      *(int *)p.arg2;    // [[unsafe]] TODO: Fix false positive.
      *(float *)p.arg1;  // [[unsafe]]
      *(char *)p.arg2;   // [[unsafe]] TODO: Fix false positive.
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {
      T0 arg0;
      T1 arg1;
    };

    void target(Struct2Arg<const int *, const int *_Nullable> &p) {
      *const_cast<int *>(p.arg0);  // [[unsafe]] TODO: Fix false positive.
      *const_cast<int *>(p.arg1);  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, NonFlowSensitiveMaterializeTemporaryExpr) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull makeNonnull();
    int *_Nullable makeNullable();
    int *makeUnannotated();

    template <typename T>
    T identity(const T &);

    void target() {
      {
        *identity<int *_Nonnull>(makeNonnull());
        int *const &p = makeNonnull();
        *p;
      }
      {
        *identity<int *_Nullable>(makeNullable());  // [[unsafe]]
        int *const &p = makeNullable();
        *p;  // [[unsafe]]
      }
      {
        *identity<int *>(makeUnannotated());
        int *const &p = makeUnannotated();
        *p;
      }
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {
      T0 getT0();
      T1 getT1();
    };

    template <typename T>
    T make();

    template <typename T>
    T identity(const T &);

    void target(Struct2Arg<int *, int *_Nullable> &p) {
      *identity<Struct2Arg<int *, int *_Nullable>>(p).getT0();
      *identity<Struct2Arg<int *, int *_Nullable>>(
           make<Struct2Arg<int *, int *_Nullable>>())
           .getT0();
      *identity<Struct2Arg<int *, int *_Nullable>>(
           Struct2Arg<int *, int *_Nullable>(p))
           .getT0();
      *identity<int *>(p.getT0());
      *identity<Struct2Arg<int *, int *_Nullable>>(p).getT1();  // [[unsafe]]
      *identity<Struct2Arg<int *, int *_Nullable>>(             // [[unsafe]]
           make<Struct2Arg<int *, int *_Nullable>>())
           .getT1();
      *identity<Struct2Arg<int *, int *_Nullable>>(  // [[unsafe]]
           Struct2Arg<int *, int *_Nullable>(p))
           .getT1();
      *identity<int *_Nullable>(p.getT1());  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, FunctionTemplates) {
  // Call expression that returns the first of two type parameters.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    T0 returnFirst();

    void target() {
      *returnFirst<int *_Nonnull, int *_Nullable>();
      *returnFirst<int *, int *_Nullable>();
      *returnFirst<int *_Nullable, int *_Nonnull>();  // [[unsafe]]
    }
  )cc"));

  // Call expression that returns the second of two type parameters.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    T1 returnSecond();

    void target() {
      *returnSecond<int *_Nullable, int *_Nonnull>();
      *returnSecond<int *_Nullable, int *>();
      *returnSecond<int *, int *_Nullable>();  // [[unsafe]]
    }
  )cc"));

  // Call expression that has an int parameter and two type parameters,
  // returning the first type parameter.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <int I0, typename T1, typename T2>
    T1 fn3ArgWithInt();

    void target() {
      *fn3ArgWithInt<1, int *_Nullable, int *>();  // [[unsafe]]
      *fn3ArgWithInt<1, int *, int *_Nullable>();
    }
  )cc"));

  // Call expression with template parameter substituted with a concrete struct.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct StructUnknownNullable {
      int *var0;
      int *_Nullable var1;

      int *getVar0();
      int *_Nullable getVar1();
    };

    template <typename T0, typename T1>
    T1 returnSecond();

    void target() {
      *returnSecond<StructUnknownNullable, int *_Nullable>();  // [[unsafe]]
      *returnSecond<int *_Nonnull, StructUnknownNullable *>();
      // TODO: The following line is a false positive. We correctly compute the
      // nullability of the expression, as confirmed by the call to
      // `assert_nullability`. However, the dataflow framework currently does
      // not model pointer values for this expression, which results in a (in
      // this case incorrect) nullptr value.
      *returnSecond<int *_Nonnull, StructUnknownNullable>()  // [[unsafe]]
           .var0;  // TODO: Fix false positive.
      __assert_nullability<NK_unspecified>(
          returnSecond<int *_Nonnull, StructUnknownNullable>().var0);
      *returnSecond<int *_Nonnull, StructUnknownNullable>().var1;  // [[unsafe]]
      *returnSecond<int *_Nonnull, StructUnknownNullable>().getVar0();
      *returnSecond<int *_Nonnull, StructUnknownNullable>()  // [[unsafe]]
           .getVar1();
    }
  )cc"));
}

TEST(PointerNullabilityTest, ParenthesizedExpressions) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0>
    struct Struct1Arg {
      T0 arg0;
      T0 getT0();
    };

    void target(Struct1Arg<int *_Nullable> p) {
      *(p).arg0;         // [[unsafe]]
      *((p)).arg0;       // [[unsafe]]
      *(p).getT0();      // [[unsafe]]
      *(((p))).getT0();  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <int I0, typename T1, typename T2>
    struct Struct3ArgWithInt {
      T1 arg1;
      T2 arg2;

      T1 getT1();
      T2 getT2();
    };

    void target(Struct3ArgWithInt<1, int *, int *_Nullable> p) {
      *(((p)).arg1);
      *(((p))).getT1();
      (*((p)).arg2);         // [[unsafe]]
      *(((((p)))).getT2());  // [[unsafe]]
    }
  )cc"));
}

// TODO: fix false positives due to unsupported PointerValues in the framework.
TEST(PointerNullabilityTest, PointerArithmetic) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable p, int *_Nonnull q, int *r) {
      *++p;  // [[unsafe]]
      *p++;  // [[unsafe]]
      *--p;  // [[unsafe]]
      *p--;  // [[unsafe]]
      *+p;   // [[unsafe]]

      *++q;  // [[unsafe]] TODO: fix false positive
      *q++;  // [[unsafe]] TODO: fix false positive
      *--q;  // [[unsafe]] TODO: fix false positive
      *q--;  // [[unsafe]] TODO: fix false positive
      *+q;   // [[unsafe]] TODO: fix false positive

      *++r;  // [[unsafe]] TODO: fix false positive
      *r++;  // [[unsafe]] TODO: fix false positive
      *--r;  // [[unsafe]] TODO: fix false positive
      *r--;  // [[unsafe]] TODO: fix false positive
      *+r;   // [[unsafe]] TODO: fix false positive
    }
  )cc"));
}

// TODO: fix false positives due to unsupported PointerValues in the framework.
TEST(PointerNullabilityTest, Deref) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      S* _Nonnull nonnull;
      S* _Nullable nullable;
      S* unknown;
    };
    void target(S& s) {
      *(*s.nonnull).nonnull;   // [[unsafe]] TODO: fix false positive
      *(*s.nonnull).nullable;  // [[unsafe]]
      *(*s.nonnull).unknown;   // [[unsafe]] TODO: fix false positive

      s.nonnull->nonnull->nonnull;   // [[unsafe]] TODO: fix false positive
      s.nonnull->nonnull->nullable;  // [[unsafe]] TODO: fix false positive
      s.nonnull->nullable->nonnull;  // [[unsafe]]
      s.nonnull->unknown->nonnull;   // [[unsafe]] TODO: fix false positive

      *&s;
    }
  )cc"));
}

TEST(PointerNullabilityTest, NonPointerReturnType) {
  checkDiagnostics(R"cc(
    struct S {
      int* p;
      int*& target() { return p; }
    };
  )cc");

  checkDiagnostics(R"cc(
    struct S {
      int* _Nullable p;
      int* _Nonnull& target() {
        return p;  // TODO: Fix false negative.
      }
    };
  )cc");
}

TEST(PointerNullabilityTest, ParenTypeInTemplate) {
  checkDiagnostics(R"cc(
    template <typename T>
    struct S {
      T(a);
      T(*(b));

      T (*f)();
      T(((*g)))();
    };

    void targetNullable(S<int* _Nullable> s) {
      *s.a;   // [[unsafe]]
      **s.b;  // [[unsafe]]
      *s.f;
      *s.g;
      // TODO: Handle function pointers. The analysis currently crashes.
      // *s.f();
      // *s.g();
    }

    void targetNonnull(S<int* _Nonnull> s) {
      *s.a;
      **s.b;
      *s.f;
      *s.g;
      // TODO: Handle function pointers. The analysis currently crashes.
      // *s.f();
      // *s.g();
    }
  )cc");

  checkDiagnostics(R"cc(
    template <typename T>
    struct S {
      T arg;
    };

    void targetNullable(S<int* _Nullable>(a), S<int* _Nullable>(*(b)),
                        S<int(*_Nullable)> c, S<int*(*(*_Nullable))> d,
                        S<int* _Nullable (*)()> e) {
      *a.arg;    // [[unsafe]]
      *b->arg;   // [[unsafe]]
      *c.arg;    // [[unsafe]]
      ***d.arg;  // [[unsafe]]
      *e.arg;    // [[unsafe]]

      // TODO: Handle function pointers. The analysis currently crashes.
      // *e.arg();
    }

    void targetNonnull(S<int* _Nonnull>(a), S<int* _Nonnull>(*(b)),
                       S<int(*_Nonnull)> c, S<int*(*(*_Nonnull))> d,
                       S<int* _Nonnull (*)()> e) {
      *a.arg;
      *b->arg;
      *c.arg;
      ***d.arg;
      *e.arg;

      // TODO: Handle function pointers. The analysis currently crashes.
      // *e.arg();
    }
  )cc");
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang

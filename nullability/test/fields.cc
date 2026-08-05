// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for nullability annotations on fields.

#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, NonNullFieldsOfPointerType) {
  // dereference field of pointer type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *_Nonnull ptr;
    };
    void target(Foo foo) { *foo.ptr; }
  )cc"));

  // dereference field of pointer type in member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *_Nonnull ptr;
      void target() { *ptr; }
    };
  )cc"));
}

TEST(PointerNullabilityTest, NullableFieldsOfPointerType) {
  // dereference field of pointer type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *_Nullable ptr;
    };
    void target(Foo foo) {
      *foo.ptr;  // [[unsafe]]
      if (foo.ptr) {
        *foo.ptr;
      } else {
        *foo.ptr;  // [[unsafe]]
      }
      *foo.ptr;  // [[unsafe]]
    }
  )cc"));

  // dereference field of pointer type in member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *_Nullable ptr;
      void target() {
        *ptr;  // [[unsafe]]
        if (ptr) {
          *ptr;
        } else {
          *ptr;  // [[unsafe]]
        }
        *ptr;  // [[unsafe]]
      }
    };
  )cc"));
}

TEST(PointerNullabilityTest, UnknownFieldsOfPointerType) {
  // dereference field of pointer type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *ptr;
    };
    void target(Foo foo) { *foo.ptr; }
  )cc"));

  // dereference field of pointer type in member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *ptr;
      void target() { *ptr; }
    };
  )cc"));
}

TEST(PointerNullabilityTest, ChainedFieldDeref) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      S *_Nonnull nonnull;
      S *_Nullable nullable;
      S *unknown;
    };
    void target(S &s) {
      *(*s.nonnull).nonnull;
      *(*s.nonnull).nullable;  // [[unsafe]]
      *(*s.nonnull).unknown;

      s.nonnull->nonnull->nonnull;
      s.nonnull->nonnull->nullable;
      s.nonnull->nullable->nonnull;  // [[unsafe]]
      s.nonnull->unknown->nonnull;

      *&s;
    }
  )cc"));
}

// This is a crash repro. It sets up a situation where we're merging pointers
// that don't have a null state to check that we don't crash in this case.
TEST(PointerNullabilityTest, MergePointersWithoutNullState) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      void *p;
    };
    void target(bool cond) {
      S src, dst;
      if (cond) dst = src;

      // `dst` has different values in the two branches that merge here, so we
      // will attempt to merge the values of `dst.p` from the two branches.

      // These lines are only here to ensure that `p` is modeled.
      S unrelated;
      unrelated.p;
    }
  )cc"));
}

TEST(PointerNullabilityTest, CreatesConsistentPointerValueForField) {
  // This is a repro for a false positive.
  // The call to `some_func()` clears the value of `p_`.
  // Our logic for creating new pointer values used to work only on prvalues,
  // so it would create two independent pointer values for the two accesses of
  // `p_`, and hence we would not be able to conclude that `p_` was null in the
  // `p_->target()` call.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      S* _Nullable const p_;
      void target() {
        some_func();

        if (p_ != nullptr)
          p_->target();  // p_ needs to be a member variable to repro.
      }
      void some_func();
    };
  )cc"));
}

TEST(PointerNullabilityTest,
     NonnullRawPointerFieldNullableAtDestructorEntryViaRValueRefMethod) {
  // An `&&`-qualified method may null out `some_resource_` before destruction,
  // so it is modeled as nullable at destructor entry and dereferencing it in
  // the destructor body is unsafe.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct SomeResource {};
    class target {
     public:
      void Finalize() && { some_resource_ = nullptr; }
      ~target() {
        *some_resource_;  // [[unsafe]]
      }

     private:
      SomeResource* _Nonnull some_resource_;
    };
  )cc"));
}

TEST(PointerNullabilityTest, NonnullRawPointerFieldCheckedAtDestructorEntry) {
  // The field is modeled as nullable at destructor entry, so a null check
  // narrows it back to nonnull and the guarded dereference is safe.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct SomeResource {};
    class target {
     public:
      void Finalize() && { some_resource_ = nullptr; }
      ~target() {
        if (some_resource_) *some_resource_;
      }

     private:
      SomeResource* _Nonnull some_resource_;
    };
  )cc"));
}

TEST(PointerNullabilityTest,
     UnknownRawPointerFieldNotNullableAtDestructorEntry) {
  // A pointer field with unknown (unannotated) nullability is NOT modeled as
  // nullable at destructor entry, even in a movable class. The user has not
  // opted into nullability checking for such a field, so dereferencing it in
  // the destructor -- without a null check -- must not be diagnosed as unsafe.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct SomeResource {};
    class target {
     public:
      void Finalize() && { some_resource_ = nullptr; }
      ~target() {
        *some_resource_;  // safe: unknown nullability, not downgraded
      }

     private:
      SomeResource* some_resource_;
    };
  )cc"));
}

TEST(PointerNullabilityTest,
     NonnullRawPointerFieldNullableAtDestructorEntryViaConsumingMethod) {
  // A method taking an rvalue reference to the same class (`Consume(target&&)`)
  // may move from its argument, nulling the argument's members before
  // destruction -- even though this class declares no move constructor or move
  // assignment operator (the user-declared destructor suppresses the implicit
  // ones). So `some_resource_` may be null at destructor entry and
  // dereferencing it is unsafe.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct SomeResource {};
    class target {
     public:
      void Consume(target&& other);
      ~target() {
        *some_resource_;  // [[unsafe]]
      }

     private:
      SomeResource* _Nonnull some_resource_;
    };
  )cc"));
}

TEST(PointerNullabilityTest,
     NonnullConstRawPointerFieldNonnullAtDestructorEntryInMovableClass) {
  // A `const` pointer field is copied (not moved) by a defaulted move operation
  // and cannot be reassigned by an `&&`-qualified method, so it is never
  // moved-from and remains nonnull at destructor entry. Only the non-const
  // sibling is downgraded and flagged.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct SomeResource {};
    class target {
     public:
      void Finalize() && { movable_resource_ = nullptr; }
      ~target() {
        *const_resource_;    // safe: const field, not downgraded
        *movable_resource_;  // [[unsafe]]
      }

     private:
      SomeResource* _Nonnull const const_resource_;
      SomeResource* _Nonnull movable_resource_;
    };
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability

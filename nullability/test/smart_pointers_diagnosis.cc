// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for diagnostics on smart pointers.

#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(SmartPointerTest, Dereference) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target() {
      *std::unique_ptr<int>();  // [[unsafe]]
      *std::make_unique<int>();
    }
  )cc"));
}

TEST(SmartPointerTest, ArrowOp) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct S {
      int i = 0;
    };
    void target() {
      std::unique_ptr<S>()->i;  // [[unsafe]]
      std::make_unique<S>()->i;
    }
  )cc"));
}

TEST(SmartPointerTest, Subscript) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target() {
      std::unique_ptr<int[]>()[0];  // [[unsafe]]
      std::make_unique<int[]>(1)[0];
    }
  )cc"));
}

TEST(SmartPointerTest, Assignment) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target(Nonnull<std::unique_ptr<int>> NonnullPtr,
                Nullable<std::unique_ptr<int>> NullablePtr,
                std::unique_ptr<int> UnannotatedPtr) {
      NonnullPtr = std::make_unique<int>();
      NonnullPtr = nullptr;  // [[unsafe]]
      NullablePtr = nullptr;
      UnannotatedPtr = nullptr;
    }
  )cc"));
}

TEST(SmartPointerTest, ResetUniquePtr) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target(Nonnull<std::unique_ptr<int>> NonnullPtr,
                Nullable<std::unique_ptr<int>> NullablePtr,
                std::unique_ptr<int> UnannotatedPtr) {
      NonnullPtr.reset();  // [[unsafe]]
      NullablePtr.reset();
      UnannotatedPtr.reset();

      (&NonnullPtr)->reset();  // [[unsafe]]
      (&NullablePtr)->reset();
      (&UnannotatedPtr)->reset();

      NonnullPtr.reset(nullptr);  // [[unsafe]]
      NullablePtr.reset(nullptr);
      UnannotatedPtr.reset(nullptr);

      NonnullPtr.reset(new int);
    }
  )cc"));
}

TEST(SmartPointerTest, ResetSharedPtr) {
  // `shared_ptr` has different `reset()` overloads than `unique_ptr`, so test
  // it too.
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target(Nonnull<std::shared_ptr<int>> NonnullPtr,
                Nullable<std::shared_ptr<int>> NullablePtr,
                std::shared_ptr<int> UnannotatedPtr) {
      NonnullPtr.reset();  // [[unsafe]]
      NullablePtr.reset();
      UnannotatedPtr.reset();

      (&NonnullPtr)->reset();  // [[unsafe]]
      (&NullablePtr)->reset();
      (&UnannotatedPtr)->reset();

      // Cannot pass `nullptr` directly to `shared_ptr::reset()` because it is
      // not a pointer type.
      int *NullIntPtr = nullptr;
      NonnullPtr.reset(NullIntPtr);  // [[unsafe]]
      NullablePtr.reset(NullIntPtr);
      UnannotatedPtr.reset(NullIntPtr);

      NonnullPtr.reset(new int);
    }
  )cc"));
}

TEST(SmartPointerTest, FunctionParameters) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void TakeNonnull(Nonnull<std::shared_ptr<int>>);
    void TakeNullable(Nullable<std::shared_ptr<int>>);
    void TakeUnannotated(std::shared_ptr<int>);
    void target(Nonnull<std::shared_ptr<int>> NonnullPtr,
                Nullable<std::shared_ptr<int>> NullablePtr,
                std::shared_ptr<int> UnannotatedPtr) {
      TakeNonnull(std::shared_ptr<int>());  // [[unsafe]]
      TakeNonnull(NonnullPtr);
      TakeNonnull(NullablePtr);  // [[unsafe]]
      TakeNonnull(UnannotatedPtr);

      TakeNullable(std::shared_ptr<int>());
      TakeNullable(NonnullPtr);
      TakeNullable(NullablePtr);
      TakeNullable(UnannotatedPtr);

      TakeUnannotated(std::shared_ptr<int>());
      TakeUnannotated(NonnullPtr);
      TakeUnannotated(NullablePtr);
      TakeUnannotated(UnannotatedPtr);
    }
  )cc"));
}

TEST(SmartPointerTest, ConstructorParameters) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct TakeNonnull {
      TakeNonnull(Nonnull<std::shared_ptr<int>>);
    };
    struct TakeNullable {
      TakeNullable(Nullable<std::shared_ptr<int>>);
    };
    struct TakeUnannotated {
      TakeUnannotated(std::shared_ptr<int>);
    };
    void target(Nonnull<std::shared_ptr<int>> NonnullPtr,
                Nullable<std::shared_ptr<int>> NullablePtr,
                std::shared_ptr<int> UnannotatedPtr) {
      TakeNonnull{std::shared_ptr<int>()};  // [[unsafe]]
      TakeNonnull{NonnullPtr};
      TakeNonnull{NullablePtr};  // [[unsafe]]
      TakeNonnull{UnannotatedPtr};

      TakeNullable{std::shared_ptr<int>()};
      TakeNullable{NonnullPtr};
      TakeNullable{NullablePtr};
      TakeNullable{UnannotatedPtr};

      TakeUnannotated{std::shared_ptr<int>()};
      TakeUnannotated{NonnullPtr};
      TakeUnannotated{NullablePtr};
      TakeUnannotated{UnannotatedPtr};
    }
  )cc"));
}

TEST(SmartPointerTest, ReturnValue_Nullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    bool cond();
    Nullable<std::unique_ptr<int>> target() {
      if (cond())
        return std::make_unique<int>(0);
      else
        return std::unique_ptr<int>();
    }
  )cc"));
}

TEST(SmartPointerTest, ReturnValue_Unknown) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    bool cond();
    std::unique_ptr<int> target() {
      if (cond())
        return std::make_unique<int>(0);
      else
        return std::unique_ptr<int>();
    }
  )cc"));
}

TEST(SmartPointerTest, InitializeMemberWithNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct target {
      target(Nonnull<std::shared_ptr<int>> NonnullPtr)
          : NonnullMember(NonnullPtr),
            NullableMember(NonnullPtr),
            UnannotatedMember(NonnullPtr) {}
      Nonnull<std::shared_ptr<int>> NonnullMember;
      Nullable<std::shared_ptr<int>> NullableMember;
      std::shared_ptr<int> UnannotatedMember;
    };
  )cc"));
}

TEST(SmartPointerTest, InitializeMemberWithNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct target {
      target(Nullable<std::shared_ptr<int>> NullablePtr)
          : NonnullMember(NullablePtr),  // [[unsafe]]
            NullableMember(NullablePtr),
            UnannotatedMember(NullablePtr) {
      // We get a warning on the following line because the constructor leaves
      // `NonnullMember` in a null state.
      /* [[unsafe]] */ }
          Nonnull<std::shared_ptr<int>> NonnullMember;
          Nullable<std::shared_ptr<int>> NullableMember;
          std::shared_ptr<int> UnannotatedMember;
    };
  )cc"));
}

TEST(SmartPointerTest, InitializeMemberWithUnannotated) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct target {
      target(std::shared_ptr<int> UnannotatedPtr)
          : NonnullMember(UnannotatedPtr),
            NullableMember(UnannotatedPtr),
            UnannotatedMember(UnannotatedPtr) {}
      Nonnull<std::shared_ptr<int>> NonnullMember;
      Nullable<std::shared_ptr<int>> NullableMember;
      std::shared_ptr<int> UnannotatedMember;
    };
  )cc"));
}

TEST(SmartPointerTest, AccessSmartPointerReturnedByReference) {
  // This is a repro for an assertion failure.
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct S {
      void f();
    };
    // The `const` is important for the repro (so that the AST at the callsite
    // below doesn't contain an `ImplicitCastExpr` to remove the const).
    const Nonnull<std::unique_ptr<S>>& ReturnNonnull();
    const Nullable<std::unique_ptr<S>>& ReturnNullable();
    const std::unique_ptr<S>& ReturnUnannotated();
    void target() {
      ReturnNonnull()->f();
      ReturnNullable()->f();  // [[unsafe]]
      ReturnUnannotated()->f();
    }
  )cc"));
}

TEST(SmartPointerTest, AccessSmartPointerReturnedByPointerAlias) {
  // This is a crash repro.
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>

    // When checking whether the base of a `->` member expression is a
    // pointer-to-smart-pointer, it used to be that we didn't canonicalize the
    // type. This test wraps such a return type in a type alias, which used to
    // cause a crash. The `const` is important because, without it, the AST
    // contains an `ImplicitCastExpr` that adds a `const`, desugaring the type
    // in the process.
    template <typename T>
    using Alias = T;
    Alias<const std::unique_ptr<int> *> getPtr();
    void target() { *getPtr()->get(); }
  )cc"));
}

TEST(SmartPointerTest, SmartPointerFlowSensitive) {
  // Simple flow-sensitive test with a smart pointer.
  // This is a repro for a false positive that we used to encounter in C++20
  // mode.
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target(Nullable<std::shared_ptr<int>> NullablePtr) {
      *NullablePtr;  // [[unsafe]]
      if (NullablePtr != nullptr) *NullablePtr;
    }
  )cc"));
}

TEST(SmartPointerTest, SimpleIfFpRepro) {
  // This is a repro for a false positive.
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    bool cond();

    void target() {
      std::shared_ptr<int> p;
      while (cond()) {
        if (p != nullptr) {
          *p;  // False positive here
        } else {
          p = std::make_shared<int>();
        }
        if (cond()) continue;
        p.reset();
      }
    }
  )cc"));
}

TEST(SmartPointerTest, NestedPointersArrowOperatorOnInner) {
  // This is a crash repro.
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>

    struct S {
      int i;
    };

    void target() {
      // The `const` is important because, without it, the AST for the arrow
      // access contains an `ImplicitCastExpr` that adds a `const` and is seen
      // as a smart pointer expression that initializes null state for the inner
      // smart pointer.
      std::unique_ptr<const std::unique_ptr<S>> p = nullptr;

      (void)(*p)->i;  // [[unsafe]]
    }
  )cc"));
}

TEST(SmartPointerTest, ConstructSmartPointerFromTemporarySmartPointer) {
  // This is a crash repro.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct _Nullable OtherS {
      using pointer = char *;
    };

    struct _Nullable S {
      using pointer = bool *;

      // S needs to be constructed by value from another smart pointer,
      // otherwise, didn't crash.
      explicit S(OtherS) {}
    };

    void target() { S{OtherS()}; }
  )cc"));
}

TEST(SmartPointerTest, ConditionalSmartPointer) {
  // This is a crash repro.
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>

    void takesNonnull(Nonnull<std::unique_ptr<int>> a);

    void target(bool b) {
      takesNonnull(b ? nullptr  // [[unsafe]]
                     : std::make_unique<int>());
    }
  )cc"));
}

TEST(SmartPointerTest, MovedFromNonnullSmartPointer) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
#include <utility>
    struct S {
      int i;
    };
    void target(Nonnull<std::unique_ptr<S>> NonnullPtr) {
      // Moving from a nonnull smart pointer is legal.
      std::unique_ptr<S> Local = std::move(NonnullPtr);

      // Moving from a moved-from nonnull smart pointer is not legal.
      // This also serves as a more general test that passing a moved-from
      // nonnull smart pointer as an argument is always disallowed, even if
      // the parameter is not annotated nonnull.
      Local = std::move(NonnullPtr);  // [[unsafe]]

      // Calling any member function or operator on a moved-from nonnull
      // smart pointer is not legal, with two exceptions:
      // - operator=
      // - reset() (non-`nullptr_t` overload)
      // We don't test all member functions and operators exhaustively, but we
      // test a few to satisfy ourselves that we have the necessary generality.
      NonnullPtr.get();  // [[unsafe]]
      // Do a test with a pointer receiver.
      (&NonnullPtr)->get();       // [[unsafe]]
      NonnullPtr.release();       // [[unsafe]]
      NonnullPtr.reset(nullptr);  // [[unsafe]]
      *NonnullPtr;                // [[unsafe]]
      NonnullPtr->i;              // [[unsafe]]
      NonnullPtr == nullptr;      // [[unsafe]]
      nullptr == NonnullPtr;      // [[unsafe]]
      // Test `operator bool`.
      if (NonnullPtr)  // [[unsafe]]
        ;

      // Assigning to a moved-from nonnull smart pointer is legal.
      // Make sure that we also allow this if the left-hand side contains
      // additional nodes such as parentheses or casts.
      NonnullPtr = std::make_unique<S>();
      Local = std::move(NonnullPtr);
      (NonnullPtr) = std::make_unique<S>();
      Local = std::move(NonnullPtr);
      static_cast<std::unique_ptr<S> &>(NonnullPtr) = std::make_unique<S>();

      // Calling non-nullptr reset() on a moved-from nonnull smart pointer is
      // legal.
      Local = std::move(NonnullPtr);
      NonnullPtr.reset(new S());
      Local = std::move(NonnullPtr);
      (NonnullPtr).reset(new S());
      Local = std::move(NonnullPtr);
      static_cast<std::unique_ptr<S> &>(NonnullPtr).reset(new S());
    }
  )cc"));
}

TEST(SmartPointerTest, UserDefinedSmartPointer) {
  // This is a regression test for a false negative where we were not issuing
  // diagnostics on user-defined smart pointer types when the smart pointer type
  // did not have a `pointer` or `element_type` type alias.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <class T>
    struct _Nullable MySmartPtr {
      // The false negative happens whether or not we have this deprecated
      // tag in place.
      using absl_nullability_compatible = void;
    };
    Nonnull<MySmartPtr<int>> target(Nullable<MySmartPtr<int>> p) {
      return p;  // [[unsafe]]
    }
  )cc"));
}

TEST(SmartPointerTest, UserDefinedSmartPointerComplexAssignmentOperator) {
  // This is a regression test for a crash involving an operator= where
  // - the LHS is a smart pointer without `pointer` or `element_type` type
  // aliases.
  // - the RHS is not a smart pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct OtherType {};

    template <class T>
    struct _Nullable MySmartPtr {
      MySmartPtr& operator=(const OtherType& other);
    };

    void target(OtherType& other) {
      Nonnull<MySmartPtr<int>> p;  // [[unsafe]]
      p = other;
    }
  )cc"));
}

TEST(SmartPointerTest, DereferenceViaNonMemberStar) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
#include <utility>
    template <class T>
    struct _Nullable MySmartPtr {
      using pointer = T*;
    };

    // A bit unusual, but one can define a non-member operator*.
    template <typename T>
    static T& operator*(MySmartPtr<T> P);

    void target() {
      MySmartPtr<int> Null;
      *Null;  // [[unsafe]]
    }
  )cc"));
}

TEST(SmartPointerTest, ArrowOperatorReturnsPointerThatNeedsNullState) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    // Take by reference to avoid an implicit cast of the argument to rvalue.
    void bar(Nonnull<int *> &A) {}

    class _Nullable CustomSmartIntPtr {
      using pointer = int *;
      int *P;

     public:
      void target() {
        // We used to crash when requesting the null state for `P` here, which
        // is implicitly accessed via the arrow operator.
        bar(P);
      }
    };
  )cc"));
}

TEST(SmartPointerTest, UnusualSmartPointerTypes) {
  // This smart pointer type is unusual in that expects its template argument to
  // be the underlying pointer type, rather than the type that the underlying
  // smart pointer points to.
  // Smart pointers that are "unusual" in this way should define a `pointer`
  // type alias to make it clear what the underlying pointer type is, but if
  // they omit this, we shouldn't crash.
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <type_traits>

    template <class T>
    class _Nullable UnusualSmartPointer {
     public:
      T operator->() const;
      std::remove_pointer_t<T> operator*() const;
      T get() const;
      T release();
    };

    struct S {
      void nonConstMemberFunction();
    };

    void target() {
      // We'll interpret the default constructor as initializing the smart
      // pointer to nullptr.
      // So we know at least (*ptr) is dereferencing a nullptr.
      // For the other expressions, without the `pointer` type alias, our guess
      // for the underlying pointer type is incorrect, so we bail out.
      UnusualSmartPointer<S *> ptr;
      (*ptr).nonConstMemberFunction();  // [[unsafe]]
      ptr->nonConstMemberFunction();
      ptr.get()->nonConstMemberFunction();
      ptr.release()->nonConstMemberFunction();
    }
  )cc"));
}

TEST(SmartPointerTest, OperatorStarReturnsPointer) {
  // This smart pointer type is unusual in that `operator*` returns a pointer
  // (while standard smart pointers like `std::unique_ptr` return a reference to
  // the pointee)
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <class T>
    class _Nullable UnusualSmartPointer {
      using pointer = T*;

     public:
      T* operator*() const;
    };

    struct S {
      void nonConstMemberFunction();
    };

    void target(UnusualSmartPointer<S> ptr) { (*ptr)->nonConstMemberFunction(); }
  )cc"));
}

TEST(SmartPointerTest, DerivedFromSmartPointerTemplateInstantiation) {
  // This is a crash repro. We did not recognize `target` as a smart pointer
  // type because we did not check the template argument of the smart pointer
  // base class, and it had no other indication of the underlying raw pointer
  // type.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename Handle>
    class _Nullable SmartPtr {
     public:
      SmartPtr(SmartPtr &&) noexcept;
    };

    // Public inheritance or private inheritance doesn't matter.
    struct target : SmartPtr<int *> {
      target(target &&Other)
          // The built-in transfer function running over the implicit
          // CXXConstructExpr in the initializer expression would crash when
          // trying to copy the synthetic smart pointer field from the source
          // RecordStorageLocation. The assignment of the result object location
          // for the derived class to be the result object location for the
          // base-class-typed initializer expression was removing the synthetic
          // field because the derived class was not seen as a smart pointer and
          // did not have the synthetic field.
          : target::SmartPtr(static_cast<target::SmartPtr &&>(Other)) {}
    };
  )cc"));
}

TEST(SmartPointerTest, NonnullSmartPointerFieldMovedFromAtExit) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct SomeResource {};
    class A {
     public:
      void target() {
        std::unique_ptr<SomeResource> some_resource = std::move(some_resource_);
      /* [[unsafe]] */ }

         private:
          Nonnull<std::unique_ptr<SomeResource>> some_resource_;
    };
  )cc"));
}

TEST(SmartPointerTest, NonnullSmartPointerFieldNotMovedFromAtExit) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct SomeResource {};
    class A {
     public:
      void target() {
        // The function doesn't move from the nonnull smart pointer field (in
        // fact, it doesn't even access it), so this is fine.
        // This also tests that the analysis doesn't try to access the field
        // when it isn't modeled.
      }

     private:
      Nonnull<std::unique_ptr<SomeResource>> some_resource_;
    };
  )cc"));
}

TEST(SmartPointerTest,
     NonnullSmartPointerFieldMovedFromThenResetToNonnullAtExit) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct SomeResource {};
    class A {
     public:
      void target() {
        // It's permissible to move from the nonnull smart pointer field during
        // execution of the method, as long as it is reset to a nonnull state
        // before exiting the method.
        std::unique_ptr<SomeResource> some_resource = std::move(some_resource_);
        some_resource_ = std::make_unique<SomeResource>();
      }

     private:
      Nonnull<std::unique_ptr<SomeResource>> some_resource_;
    };
  )cc"));
}

TEST(SmartPointerTest, NonnullSmartPointerFieldMovedFromAtDestructorExit) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct SomeResource {};
    class target {
     public:
      ~target() {
        // Don't warn if the nonnull smart pointer field is moved from at
        // destructor exit, because it's not possible to access the field after
        // this point.
        std::unique_ptr<SomeResource> some_resource = std::move(some_resource_);
      }

     private:
      Nonnull<std::unique_ptr<SomeResource>> some_resource_;
    };
  )cc"));
}

TEST(SmartPointerTest, NullableSmartPointerFieldMovedFromAtExit) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct SomeResource {};
    class A {
     public:
      void target() {
        // `some_resource_` is nullable, so it's fine for it to be in the
        // moved-from state when this member function exits.
        std::unique_ptr<SomeResource> some_resource = std::move(some_resource_);
      }

     private:
      Nullable<std::unique_ptr<SomeResource>> some_resource_;
    };
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for smart pointers.
//
// Where `unique_ptr` and `shared_ptr` provide same API, we test the API only on
// `unique_ptr`. Duplicating the tests for `shared_ptr` would not give us
// additional coverage, as we know the implementation is shared between all
// smart pointer types. (This can be viewed as a form of white-box testing.)
// We therefore only test `shared_ptr` APIs that do not exist in equivalent form
// in `unique_ptr`.

#include <memory>
#include <type_traits>
#include <utility>

#include "nullability_test.h"

_Nonnull std::unique_ptr<int> makeNonnull();
_Nullable std::unique_ptr<int> makeNullable();
std::unique_ptr<int> makeUnknown();

int *_Nonnull makeNonnullRaw();
int *_Nullable makeNullableRaw();
int *makeUnknownRaw();

const _Nonnull std::unique_ptr<int> &returnNonnullRef();
const _Nullable std::unique_ptr<int> &returnNullableRef();
const std::unique_ptr<int> &returnUnknownRef();

// Add an extra wrinkle for the following functions by wrapping the return type
// in a type alias. It used to be that we didn't canonicalize these types,
// leading to crashes.
template <typename T>
using Alias = T;
Alias<const _Nonnull std::unique_ptr<int> *> returnPtrToNonnull();
Alias<const _Nullable std::unique_ptr<int> *> returnPtrToNullable();
Alias<const std::unique_ptr<int> *> returnPtrToUnknown();

// Provided for various tests that require a base and derived class.
struct Base {
  virtual ~Base();
};
struct Derived : public Base {
  ~Derived() override;
};

TEST void parameterAnnotations(_Nonnull std::unique_ptr<int> NonnullParam,
                               _Nullable std::unique_ptr<int> NullableParam,
                               std::unique_ptr<int> UnknownParam) {
  nonnull(NonnullParam);
  nullable(NullableParam);
  unknown(UnknownParam);
}

TEST void returnValueAnnotations() {
  nonnull(makeNonnull());
  nullable(makeNullable());
  unknown(makeUnknown());
}

TEST void returnValueAnnotationsRef() {
  nonnull(returnNonnullRef());
  nullable(returnNullableRef());
  unknown(returnUnknownRef());
}

TEST void returnValueAnnotationsPtrToSmartPtr() {
  nonnull(*returnPtrToNonnull());
  nullable(*returnPtrToNullable());
  unknown(*returnPtrToUnknown());
}

TEST void returnValueAnnotationsPtrToSmartPtrGet() {
  // This is a crash repro.
  // Call `get()` on the smart pointer with `->` syntax, so that there isn't
  // a glvalue of smart pointer type in the AST. We used to crash on this
  // because we didn't initialize the nullability properties on the contained
  // raw pointer.
  // Don't merge this test with the test above, or the `*` dereferences will
  // cause the smart pointers to get initialized.
  nonnull(returnPtrToNonnull()->get());
  nullable(returnPtrToNullable()->get());
  unknown(returnPtrToUnknown()->get());
}

TEST void outputParameters() {
  // This test checks only a few of the most common cases for output parameters.
  // The tests for raw pointers cover a broader set of cases. Because we know
  // that the implementation is shared between raw pointers and smart pointers,
  // we chose not to duplicate all of those tests here.

  {
    void maybeModifyPtr(std::unique_ptr<int> * P);
    std::unique_ptr<int> P;
    nullable(P);
    maybeModifyPtr(&P);
    unknown(P);
  }

  {
    void maybeModifyPtr(std::unique_ptr<int> & P);
    std::unique_ptr<int> P;
    nullable(P);
    maybeModifyPtr(P);
    unknown(P);
  }

  {
    void doesntModifyPtr(const std::unique_ptr<int> *P);
    std::unique_ptr<int> P;
    nullable(P);
    doesntModifyPtr(&P);
    nullable(P);
  }

  {
    void doesntModifyPtr(const std::unique_ptr<int> &P);
    std::unique_ptr<int> P;
    nullable(P);
    doesntModifyPtr(P);
    nullable(P);
  }
}

TEST void defaultConstructor() { nullable(std::unique_ptr<int>()); }

TEST void nullptrConstructor() {
  nullable(std::unique_ptr<int>(nullptr));

  nullable(std::shared_ptr<int>(nullptr, std::default_delete<int>()));
  nullable(std::shared_ptr<int>(nullptr, std::default_delete<int>(),
                                std::allocator<int>()));
}

TEST void constructorTakingPointer() {
  nonnull(std::unique_ptr<int>(makeNonnullRaw()));
  nullable(std::unique_ptr<int>(makeNullableRaw()));
  unknown(std::unique_ptr<int>(makeUnknownRaw()));

  nonnull(std::unique_ptr<int>(makeNonnullRaw(), std::default_delete<int>()));
  nullable(std::unique_ptr<int>(makeNullableRaw(), std::default_delete<int>()));
  unknown(std::unique_ptr<int>(makeUnknownRaw(), std::default_delete<int>()));

  nonnull(std::shared_ptr<int>(makeNonnullRaw(), std::default_delete<int>(),
                               std::allocator<int>()));
  nullable(std::shared_ptr<int>(makeNullableRaw(), std::default_delete<int>(),
                                std::allocator<int>()));
  unknown(std::shared_ptr<int>(makeUnknownRaw(), std::default_delete<int>(),
                               std::allocator<int>()));
}

TEST void constructorTakingPointer_ArrayVersion() {
  nonnull(std::unique_ptr<int[]>(makeNonnullRaw()));
  nullable(std::unique_ptr<int[]>(makeNullableRaw()));
  unknown(std::unique_ptr<int[]>(makeUnknownRaw()));

  nonnull(
      std::unique_ptr<int[]>(makeNonnullRaw(), std::default_delete<int[]>()));
  nullable(
      std::unique_ptr<int[]>(makeNullableRaw(), std::default_delete<int[]>()));
  unknown(
      std::unique_ptr<int[]>(makeUnknownRaw(), std::default_delete<int[]>()));
}

TEST void constructorTakingPointer_DerivedToBaseConversion() {
  // Test the `shared_ptr` constructor taking a pointer that can be converted
  // to the underlying pointer type.
  std::shared_ptr<Base> base(new Derived());
  nonnull(base);
  nonnull(base.get());
}

TEST void moveConstructor(_Nonnull std::unique_ptr<int> NonnullParam,
                          _Nullable std::unique_ptr<int> NullableParam,
                          std::unique_ptr<int> UnknownParam) {
  nonnull(std::unique_ptr<int>(std::move(NonnullParam)));
  nullable(std::unique_ptr<int>(std::move(NullableParam)));
  unknown(std::unique_ptr<int>(std::move(UnknownParam)));

  nullable(NonnullParam);
  nullable(NullableParam);
  nullable(UnknownParam);
}

TEST void sharedPtrFromUniquePtr(_Nonnull std::unique_ptr<int> NonnullParam,
                                 _Nullable std::unique_ptr<int> NullableParam,
                                 std::unique_ptr<int> UnknownParam) {
  nonnull(std::shared_ptr<int>(std::move(NonnullParam)));
  nullable(std::shared_ptr<int>(std::move(NullableParam)));
  unknown(std::shared_ptr<int>(std::move(UnknownParam)));

  nullable(NonnullParam);
  nullable(NullableParam);
  nullable(UnknownParam);
}

TEST void copyConstructor(_Nonnull std::shared_ptr<int> NonnullParam,
                          _Nullable std::shared_ptr<int> NullableParam,
                          std::shared_ptr<int> UnknownParam) {
  nonnull(std::shared_ptr<int>(NonnullParam));
  nullable(std::shared_ptr<int>(NullableParam));
  unknown(std::shared_ptr<int>(UnknownParam));

  nonnull(NonnullParam);
  nullable(NullableParam);
  unknown(UnknownParam);
}

TEST void aliasingConstructor(_Nonnull std::shared_ptr<int> NonnullParam) {
  nullable(std::shared_ptr<int>(NonnullParam, nullptr));
  nonnull(NonnullParam);

  nullable(std::shared_ptr<int>(std::move(NonnullParam), nullptr));
  nullable(NonnullParam);
}

TEST void sharedPtrFromWeakPtr(std::weak_ptr<int> Weak) {
  nonnull(std::shared_ptr<int>(Weak));
}

TEST void nullptrAssignment() {
  std::unique_ptr<int> P = makeUnknown();
  unknown(P);
  P = nullptr;
  nullable(P);
}

TEST void moveAssignment(_Nonnull std::unique_ptr<int> NonnullParam,
                         _Nullable std::unique_ptr<int> NullableParam,
                         std::unique_ptr<int> UnknownParam) {
  std::unique_ptr<int> NonnullLocal;
  nonnull(NonnullLocal = std::move(NonnullParam));
  std::unique_ptr<int> NullableLocal;
  nullable(NullableLocal = std::move(NullableParam));
  std::unique_ptr<int> UnknownLocal;
  unknown(UnknownLocal = std::move(UnknownParam));

  nullable(NonnullParam);
  nullable(NullableParam);
  nullable(UnknownParam);
}

TEST void copyAssignment(_Nonnull std::shared_ptr<int> NonnullParam,
                         _Nullable std::shared_ptr<int> NullableParam,
                         std::shared_ptr<int> UnknownParam) {
  std::shared_ptr<int> NonnullLocal;
  nonnull(NonnullLocal = NonnullParam);
  std::shared_ptr<int> NullableLocal;
  nullable(NullableLocal = NullableParam);
  std::shared_ptr<int> UnknownLocal;
  unknown(UnknownLocal = UnknownParam);

  nonnull(NonnullParam);
  nullable(NullableParam);
  unknown(UnknownParam);
}

TEST void release(_Nonnull std::unique_ptr<int> NonnullParam,
                  _Nullable std::unique_ptr<int> NullableParam,
                  std::unique_ptr<int> UnknownParam) {
  nonnull(NonnullParam.release());
  nullable(NullableParam.release());
  unknown(UnknownParam.release());

  nullable(NonnullParam);
  nullable(NullableParam);
  nullable(UnknownParam);
}

TEST void reset() {
  {
    auto P = std::make_unique<int>();
    P.reset();
    provable(P.get() == nullptr);
  }

  {
    std::unique_ptr<int> P;
    int *Raw = new int();
    P.reset(Raw);
    provable(P.get() == Raw);
  }

  {
    auto P = std::make_unique<int[]>(1);
    P.reset();
    provable(P.get() == nullptr);
  }

  {
    auto P = std::make_unique<int[]>(1);
    P.reset(nullptr);
    provable(P.get() == nullptr);
  }

  {
    std::unique_ptr<int[]> P;
    int *Raw = new int[1];
    P.reset(Raw);
    provable(P.get() == Raw);
  }

  {
    auto P = std::make_shared<int>();
    P.reset();
    provable(P.get() == nullptr);
  }

  {
    std::shared_ptr<int> P;
    int *Raw = new int();
    P.reset(Raw);
    provable(P.get() == Raw);
  }

  {
    std::shared_ptr<int> P;
    int *Raw = new int();
    P.reset(Raw, std::default_delete<int>());
    provable(P.get() == Raw);
  }

  {
    std::shared_ptr<int> P;
    int *Raw = new int();
    P.reset(Raw, std::default_delete<int>(), std::allocator<int>());
    provable(P.get() == Raw);
  }
}

TEST void swap() {
  {
    auto P1 = std::make_unique<int>();
    auto P2 = std::make_unique<int>();
    int *Raw1 = P1.get();
    int *Raw2 = P2.get();
    P1.swap(P2);
    provable(P1.get() == Raw2);
    provable(P2.get() == Raw1);
  }

  {
    auto P1 = std::make_unique<int>();
    auto P2 = std::make_unique<int>();
    int *Raw1 = P1.get();
    int *Raw2 = P2.get();
    std::swap(P1, P2);
    provable(P1.get() == Raw2);
    provable(P2.get() == Raw1);
  }
}

TEST void get(int *Raw) {
  std::unique_ptr<int> Null;
  provable(Null.get() == nullptr);

  std::unique_ptr<int> P(Raw);
  provable(P.get() == Raw);

  // Test `->method()` call syntax.
  provable((&Null)->get() == nullptr);
  provable((&P)->get() == Raw);
}

TEST void operatorBool() {
  provable(!std::unique_ptr<int>());
  provable(static_cast<bool>(std::make_unique<int>()));

  // Test `->method()` call syntax.
  auto P = std::make_unique<int>();
  provable((&P)->operator bool());
}

TEST void operatorStar() {
  auto P = std::make_unique<int>();
  provable(P.get() == &*P);
}

namespace operator_arrow {

struct S {
  int I;
};

TEST void operatorArrow() {
  auto P = std::make_unique<S>();
  provable(&P.get()->I == &P->I);
}

}  // namespace operator_arrow

TEST void makeUnique() {
  nonnull(std::make_unique<int>());
  nonnull(std::make_unique<int>(42));
  nonnull(std::make_unique_for_overwrite<int>());
  nonnull(std::make_unique_for_overwrite<int[]>(5));
}

TEST void makeShared() {
  nonnull(std::make_shared<int>());
  nonnull(std::make_shared<int>(42));
  nonnull(std::make_shared_for_overwrite<int>());
  nonnull(std::make_shared_for_overwrite<int[]>(5));
}

TEST void makeSmartPointerToRawPointer() {
  nullable(*std::make_shared<int *>(nullptr));
  nullable(*std::make_shared<int *>(nullptr).get());
  int *Raw = nullptr;
  nullable(*std::make_shared<int *>(Raw));
  nullable(*std::make_shared<int *>(Raw).get());
  int I;
  Raw = &I;
  nonnull(*std::make_shared<int *>(Raw));
  nonnull(*std::make_shared<int *>(Raw).get());
}

TEST void allocateShared() {
  nonnull(std::allocate_shared<int>(std::allocator<int>()));
  nonnull(std::allocate_shared<int>(std::allocator<int>(), 42));
  nonnull(std::allocate_shared_for_overwrite<int>(std::allocator<int>()));
  nonnull(
      std::allocate_shared_for_overwrite<int[]>(std::allocator<int[]>(), 5));
}

TEST void staticPointerCast(_Nonnull std::shared_ptr<Base> NonnullParam,
                            _Nullable std::shared_ptr<Base> NullableParam,
                            std::shared_ptr<Base> UnknownParam) {
  provable(std::static_pointer_cast<Derived>(std::shared_ptr<Base>()) ==
           nullptr);

  nonnull(std::static_pointer_cast<Derived>(NonnullParam));
  nullable(std::static_pointer_cast<Derived>(NullableParam));
  unknown(std::static_pointer_cast<Derived>(UnknownParam));

  // Arguments are unchanged after calling const lvalue reference overload.
  nonnull(NonnullParam);
  nullable(NullableParam);
  unknown(UnknownParam);

  nonnull(std::static_pointer_cast<Derived>(std::move(NonnullParam)));
  nullable(std::static_pointer_cast<Derived>(std::move(NullableParam)));
  unknown(std::static_pointer_cast<Derived>(std::move(UnknownParam)));

  // Arguments are empty after calling rvalue reference overload.
  provable(!NonnullParam);
  provable(!NullableParam);
  provable(!UnknownParam);
}

TEST void dynamicPointerCast(_Nonnull std::shared_ptr<Base> NonnullParam,
                             _Nullable std::shared_ptr<Base> NullableParam,
                             std::shared_ptr<Base> UnknownParam) {
  provable(std::dynamic_pointer_cast<Derived>(std::shared_ptr<Base>()) ==
           nullptr);

  nullable(std::dynamic_pointer_cast<Derived>(NonnullParam));
  nullable(std::dynamic_pointer_cast<Derived>(NullableParam));
  nullable(std::dynamic_pointer_cast<Derived>(UnknownParam));

  // Arguments are unchanged after calling const lvalue reference overload.
  nonnull(NonnullParam);
  nullable(NullableParam);
  unknown(UnknownParam);

  nullable(std::dynamic_pointer_cast<Derived>(std::move(NonnullParam)));
  nullable(std::dynamic_pointer_cast<Derived>(std::move(NullableParam)));
  nullable(std::dynamic_pointer_cast<Derived>(std::move(UnknownParam)));

  // Arguments are nullable (but not provably Null) after calling rvalue
  // reference overload (because they may or may not have been moved from).
  nullable(NonnullParam);
  nullable(NullableParam);
  nullable(UnknownParam);
  possible(NonnullParam != nullptr);
  possible(NullableParam != nullptr);
  possible(UnknownParam != nullptr);

  // However, if the argument was Null, then it should remain Null (and not just
  // nullable) after calling the rvalue reference overload.
  std::shared_ptr<Base> Null;
  provable(std::dynamic_pointer_cast<Derived>(Null) == nullptr);
  provable(Null == nullptr);
}

TEST void constPointerCast() {
  // A `const_pointer_cast`, unlike the other cast types, will definitely
  // produce a pointer with the same storage location as the source, so we can
  // test this cast more easily than the others.

  provable(std::const_pointer_cast<int>(std::shared_ptr<const int>()) ==
           nullptr);

  auto P = std::make_shared<const int>();
  provable(std::const_pointer_cast<int>(P).get() == P.get());
  provable(P != nullptr);
  std::const_pointer_cast<int>(std::move(P));
  provable(!P);
}

namespace reinterpret_pointer_cast {

// `S` and `S::I` are pointer-interconvertible.
struct S {
  int I;
};

TEST void reinterpretPointerCast(_Nonnull std::shared_ptr<S> NonnullParam,
                                 _Nullable std::shared_ptr<S> NullableParam,
                                 std::shared_ptr<S> UnknownParam) {
  // By the standard, the pointers we produce through `reinterpret_pointer_cast`
  // in this test should have the same address, but the dataflow framework does
  // not allow us to express this (as it requires different `StorageLocation`s
  // for different types). Therefore, we need to test `reinterpret_pointer_cast`
  // more indirectly, similar to `static_pointer_cast` and
  // `dynamic_pointer_cast` above.

  provable(std::reinterpret_pointer_cast<int>(std::shared_ptr<S>()) == nullptr);

  nonnull(std::reinterpret_pointer_cast<int>(NonnullParam));
  nullable(std::reinterpret_pointer_cast<int>(NullableParam));
  unknown(std::reinterpret_pointer_cast<int>(UnknownParam));

  // Arguments are unchanged after calling const lvalue reference overload.
  nonnull(NonnullParam);
  nullable(NullableParam);
  unknown(UnknownParam);

  nonnull(std::reinterpret_pointer_cast<int>(std::move(NonnullParam)));
  nullable(std::reinterpret_pointer_cast<int>(std::move(NullableParam)));
  unknown(std::reinterpret_pointer_cast<int>(std::move(UnknownParam)));

  // Arguments are empty after calling rvalue reference overload.
  provable(!NonnullParam);
  provable(!NullableParam);
  provable(!UnknownParam);
}

}  // namespace reinterpret_pointer_cast

TEST void operatorEqualsAndNotEquals() {
  // We perform this test on `shared_ptr` rather than `unique_ptr` because it
  // allows us the test to be stronger: We can check that two different
  // `shared_ptr`s with the same underlying Raw pointer compare equal. We can't
  // test this with `unique_ptr` because it is, well, unique.
  auto P1 = std::make_shared<int>();
  auto P2 = std::make_shared<int>();
  std::shared_ptr<int> Null;

  provable(P1 == P1);
  provable(P1 == std::shared_ptr<int>(P1));
  provable(Null == std::shared_ptr<int>());

  provable(P1 != P2);
  provable(P1 != Null);
  provable(P2 != Null);

  provable(Null == nullptr);
  provable(P1 != nullptr);
  provable(nullptr == Null);
  provable(nullptr != P1);
}

TEST void weakPtrLocReturnsNullable(std::shared_ptr<int> Shared) {
  std::weak_ptr<int> Weak(Shared);
  nullable(Weak.lock());
}

namespace user_defined_smart_pointers {

template <typename T>
struct _Nullable UserDefinedSmartPointer {
  using pointer = T *;

  pointer get() const;
};

TEST void userDefinedSmartPointers(
    _Nonnull UserDefinedSmartPointer<int> NonnullParam,
    _Nullable UserDefinedSmartPointer<int> NullableParam,
    UserDefinedSmartPointer<int> UnknownParam) {
  // Just spot-check some basic behaviors, as the implementation treats
  // user-defined smart pointers like standard smart pointers, so the tests for
  // standard smart pointers provide sufficient coverage. This also tests that
  // we can put nullability specifiers on a smart pointer class that has the
  // `_Nullable` attribute.

  nonnull(NonnullParam);
  nullable(NullableParam);
  unknown(UnknownParam);

  nonnull(NonnullParam.get());
  nullable(NullableParam.get());
  unknown(UnknownParam.get());
}

template <typename T>
using Nonnull = _Nonnull T;
template <typename T>
using Nullable = _Nullable T;

TEST void userDefinedSmartPointersWithAliasAnnotation(
    Nonnull<UserDefinedSmartPointer<int>> NonnullParam,
    Nullable<UserDefinedSmartPointer<int>> NullableParam,
    UserDefinedSmartPointer<int> UnknownParam) {
  // Spot-check that the nullability is set correctly when the specifier is
  // applied to a user-defined smart pointer by a template alias.

  nonnull(NonnullParam);
  nullable(NullableParam);
  unknown(UnknownParam);
}

struct NonPointer {};

template <typename T>
struct _Nullable UserDefinedSmartPointerWithNonPointerRHSAssignOp {
  UserDefinedSmartPointerWithNonPointerRHSAssignOp &operator=(
      const NonPointer &);
};

TEST void userDefinedSmartPointersWithNonPointerRHSAssignOp(NonPointer Param) {
  UserDefinedSmartPointerWithNonPointerRHSAssignOp<int> Local;
  unknown(Local = Param);
}

// An example where operator== takes a raw pointer argument instead of
// std::nullptr_t for comparison to `nullptr` literals.
struct _Nullable WithCompareRawPointer {
  using pointer = int*;

  WithCompareRawPointer();
};
bool operator==(const WithCompareRawPointer& X, const WithCompareRawPointer& Y);
bool operator==(const WithCompareRawPointer& X, int* Y);
bool operator!=(const WithCompareRawPointer& X, int* Y);

TEST void userDefinedSmartPointersWithCompareRawPointer(
    _Nonnull WithCompareRawPointer NonnullParam,
    _Nullable WithCompareRawPointer NullableParam,
    int* _Nonnull NonnullRawPointer) {
  // This is a crash repro -- make sure that when the argument is literal
  // `nullptr`, the analysis looks through the cast from `nullptr_t` to `int*`.
  WithCompareRawPointer Null;
  provable(Null == nullptr);
  provable(NonnullParam != nullptr);
  possible(NullableParam == nullptr);
  possible(NullableParam != nullptr);

  // Other things can prove.
  provable(Null == WithCompareRawPointer());
  provable(NonnullParam == NonnullParam);

  // Compare to a raw pointer that isn't literal nullptr
  // (we don't model for now, just don't want to crash)
  NonnullParam == NonnullRawPointer;
  NonnullParam != NonnullRawPointer;
}

}  // namespace user_defined_smart_pointers

namespace derived_from_unique_ptr {

struct Allocator {};
struct S {};

template <typename T>
struct _Nullable DerivedPtr : public std::unique_ptr<T> {
  // Allocates a new object using the given allocator.
  DerivedPtr(Allocator *);
};

// This is a crash repro. Make sure we can handle classes that are derived from
// smart pointers.
TEST void derivedFromUniquePtr(_Nonnull DerivedPtr<int> Ptr) {
  nonnull(Ptr);
  nonnull(Ptr.get());
}

// This is a crash repro. Make sure we don't assume that `A` is the underlying
// pointer for the smart pointer.
TEST void constructorTakingUnrelatedPointer(Allocator *A) {
  DerivedPtr<S> Ptr(A);
  // This used to cause a crash because we would attempt to copy a record of
  // type `S` to a record of type `Allocator`.
  *Ptr = S();
}

template <typename T>
struct PrivatelyDerivedPtr : private std::unique_ptr<T> {
  PrivatelyDerivedPtr();

  using std::unique_ptr<T>::operator=;
};

// This is a crash repro. Make sure we can handle classes that are privately
// derived from smart pointers. We don't consider such a class itself to be a
// supported smart pointer type, but we need to model the pointer field because
// copy and assignment operations may copy to it.
TEST void privatelyDerivedFromUniquePtr(_Nonnull std::unique_ptr<int> Ptr) {
  PrivatelyDerivedPtr<int> Dest;
  Dest = std::move(Ptr);
}

}  // namespace derived_from_unique_ptr

namespace underlying_type_is_not_raw_pointer {

struct Deleter {
  // Use a `shared_ptr` as the underlying pointer type. This wouldn't make a lot
  // of sense in production code, but we use it in the test because we already
  // have it available.
  using pointer = std::shared_ptr<int>;
  void operator()(pointer);
};

// This is a crash repro. Make sure we don't crash on a `unique_ptr` whose
// underlying `pointer` type is not a raw pointer.
// For the time being, we don't check these but silently ignore them; this seems
// acceptable, as this case is rare.
TEST Deleter::pointer underlyingTypeIsNotRawPointer() {
  std::unique_ptr<int, Deleter> Ptr;
  // TODO: Should be nullable, but we don't model this case.
  unknown(Ptr.get());
  return Ptr.get();
}

}  // namespace underlying_type_is_not_raw_pointer

namespace unusual_smart_pointer_type {

// This is a crash repro.
// This smart pointer type is unusual in that expects its template argument to
// be the underlying pointer type, rather than the type that the underlying
// smart pointer points to.
// Smart pointers that are "unusual" in this way should define a `pointer` type
// alias to make it clear what the underlying pointer type is, but if they omit
// this, we shouldn't crash.
template <class T>
struct _Nullable UnusualSmartPointer {
  // A more "usual" smart pointer type, such as `std::unique_ptr`, would return
  // `T*` from `operator->()`, `get()`, and `release()`, and `T&` from
  // `operator*()`.
  T operator->() const;
  std::remove_pointer_t<T> operator*() const;
  T get() const;
  T release();
};

struct S {
  void nonConstMemberFunction();
};

TEST void unusualSmartPointerType() {
  UnusualSmartPointer<S *> Ptr;
  // We shouldn't crash while analyzing these calls.
  Ptr->nonConstMemberFunction();
  (*Ptr).nonConstMemberFunction();
  Ptr.get()->nonConstMemberFunction();
  Ptr.release()->nonConstMemberFunction();
}

// Similar to `UnusualSmartPointer`, but define the operators in a base class
// and only mark the child class as a smart pointer.
template <class T>
class UnusualSmartPointerBase {
 public:
  T operator->() const;
  std::remove_pointer_t<T> operator*() const;
  T get() const;
  T release();
};

template <class T>
class _Nullable UnusualSmartPointerOperatorsInBase
    : public UnusualSmartPointerBase<T>{};

TEST void unusualSmartPointerTypeOperatorsInBase() {
  UnusualSmartPointerOperatorsInBase<S *> Ptr;
  // We shouldn't crash while analyzing these calls.
  Ptr->nonConstMemberFunction();
  (*Ptr).nonConstMemberFunction();
  Ptr.get()->nonConstMemberFunction();
  Ptr.release()->nonConstMemberFunction();
}

// An unusual smart pointer type which indicates that the `pointer` type
// is `void*`. Perhaps later it is possible to convert to a `T*`, but
// that conversion is hidden from our analysis.
template <class T>
struct _Nullable VoidStarSmartPointer {
  using pointer = void*;

  void* get() const;
};

// An unusual smart pointer type that can be constructed from
// a VoidStarSmartPointer (or assigned, swapped, or reset).
// Likely, to make this work, the operations do more than just copy the
// underlying pointers around, but that is invisible to the analysis.
//
// The operator*, operator-> would have type checked, but if we naively copied
// the underlying pointers directly, it would break assumptions about the types.
template <class T>
struct _Nullable NonVoidStarSmartPointer {
  using pointer = T*;

  NonVoidStarSmartPointer(VoidStarSmartPointer<T> & Other);
  NonVoidStarSmartPointer& operator=(const VoidStarSmartPointer<T>&);
  NonVoidStarSmartPointer& operator=(const VoidStarSmartPointer<T>&&);
  NonVoidStarSmartPointer();

  void swap(VoidStarSmartPointer<T> & Other);
  void reset(void* P = nullptr);

  T* operator->() const;
  T& operator*() const;
  T* get() const;
};

TEST void copyFromVoidStarSmartPointerType(VoidStarSmartPointer<S> Void) {
  NonVoidStarSmartPointer<S> NonVoid(Void);
  // We shouldn't crash while analyzing these calls (copying the incorrectly
  // typed `void*` pointer from Arg to Ptr, and using it in subsequent
  // analysis steps).
  NonVoid->nonConstMemberFunction();
  (*NonVoid).nonConstMemberFunction();
  NonVoid.get()->nonConstMemberFunction();

  NonVoid = Void;
  NonVoid->nonConstMemberFunction();

  NonVoid = std::move(Void);
  NonVoid->nonConstMemberFunction();

  VoidStarSmartPointer<S> FreshVoid;
  NonVoid.reset(FreshVoid.get());
  NonVoid->nonConstMemberFunction();

  NonVoid.swap(FreshVoid);
  NonVoid->nonConstMemberFunction();
}

}  // namespace unusual_smart_pointer_type

// Check non-member operator overloading (check if we have assumptions
// about presence of receiver arg that may be a pointer type, etc.).
namespace free_standing_operator_calls {

struct A {
  explicit A(int X) : X(X) {}
  int X;
};

static _Nonnull std::unique_ptr<A> operator*(std::unique_ptr<A> L,
                                             std::unique_ptr<A> R) {
  return std::make_unique<A>(L->X * R->X);
}

TEST void nonMemberBinaryOperatorStar() {
  auto P1 = std::make_unique<A>(1);
  auto P2 = std::make_unique<A>(2);
  nonnull(std::move(P1) * std::move(P2));
}

template <class T>
struct _Nullable MySmartPtr {
  using pointer = T *;

  T *get() const;
  T *Ptr;
};

// A bit unusual, but one can define a non-member operator*.
template <typename T>
static T &operator*(MySmartPtr<T> P) {
  return *P.Ptr;
}

TEST void nonMemberUnaryOperatorStar() {
  int X = 42;
  int *_Nonnull PtrToX = &X;
  MySmartPtr<int *> NonNull = {&PtrToX};
  provable(NonNull.get() == &*NonNull);
}

}  // namespace free_standing_operator_calls

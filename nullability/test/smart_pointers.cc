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
#include <utility>

#include "nullability_test.h"

Nonnull<std::unique_ptr<int>> makeNonnull();
Nullable<std::unique_ptr<int>> makeNullable();
std::unique_ptr<int> makeUnknown();

Nonnull<int *> makeNonnullRaw();
Nullable<int *> makeNullableRaw();
int *makeUnknownRaw();

const Nonnull<std::unique_ptr<int>> &returnNonnullRef();
const Nullable<std::unique_ptr<int>> &returnNullableRef();
const std::unique_ptr<int> &returnUnknownRef();

TEST void parameterAnnotations(Nonnull<std::unique_ptr<int>> NonnullParam,
                               Nullable<std::unique_ptr<int>> NullableParam,
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

TEST void moveConstructor(Nonnull<std::unique_ptr<int>> NonnullParam,
                          Nullable<std::unique_ptr<int>> NullableParam,
                          std::unique_ptr<int> UnknownParam) {
  nonnull(std::unique_ptr<int>(std::move(NonnullParam)));
  nullable(std::unique_ptr<int>(std::move(NullableParam)));
  unknown(std::unique_ptr<int>(std::move(UnknownParam)));

  nullable(NonnullParam);
  nullable(NullableParam);
  nullable(UnknownParam);
}

TEST void sharedPtrFromUniquePtr(Nonnull<std::unique_ptr<int>> NonnullParam,
                                 Nullable<std::unique_ptr<int>> NullableParam,
                                 std::unique_ptr<int> UnknownParam) {
  nonnull(std::shared_ptr<int>(std::move(NonnullParam)));
  nullable(std::shared_ptr<int>(std::move(NullableParam)));
  unknown(std::shared_ptr<int>(std::move(UnknownParam)));

  nullable(NonnullParam);
  nullable(NullableParam);
  nullable(UnknownParam);
}

TEST void copyConstructor(Nonnull<std::shared_ptr<int>> NonnullParam,
                          Nullable<std::shared_ptr<int>> NullableParam,
                          std::shared_ptr<int> UnknownParam) {
  nonnull(std::shared_ptr<int>(NonnullParam));
  nullable(std::shared_ptr<int>(NullableParam));
  unknown(std::shared_ptr<int>(UnknownParam));

  nonnull(NonnullParam);
  nullable(NullableParam);
  unknown(UnknownParam);
}

TEST void aliasingConstructor(Nonnull<std::shared_ptr<int>> NonnullParam) {
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

TEST void moveAssignment(Nonnull<std::unique_ptr<int>> NonnullParam,
                         Nullable<std::unique_ptr<int>> NullableParam,
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

TEST void copyAssignment(Nonnull<std::shared_ptr<int>> NonnullParam,
                         Nullable<std::shared_ptr<int>> NullableParam,
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

TEST void release(Nonnull<std::unique_ptr<int>> NonnullParam,
                  Nullable<std::unique_ptr<int>> NullableParam,
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
  auto P = std::make_unique<bool>();
  *P = false;
  provable(!*P);
  *P = true;
  provable(*P);

  // TODO(mboehme): We'd like to be able to write the following, but the
  // dataflow framework currently doesn't consider two different `PointerValue`s
  // with the same location to be the same.
  // Instead, we need to take a slightly more indirect approach to testing
  // `operator*()`, see above. Once we have better pointer comparisons in the
  // framework, replace the test above with the test below.
  // auto P = std::make_unique<int>();
  // provable(P.get() == &*P);
}

TEST void operatorArrow() {
  std::unique_ptr<std::unique_ptr<int>> PP;
  int *Raw = new int();
  PP->reset(Raw);
  provable(PP->get() == Raw);

  // TODO(mboehme): Replace the more indirect test above with this test below
  // once the framework considers two different `PointerValue`s with the same
  // location to be the same.
#if 0
  struct S {
    int i;
  };
  auto P = std::make_unique<S>();
  provable(&P.get()->b == &P->b);
#endif
}

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

TEST void allocateShared() {
  nonnull(std::allocate_shared<int>(std::allocator<int>()));
  nonnull(std::allocate_shared<int>(std::allocator<int>(), 42));
  nonnull(std::allocate_shared_for_overwrite<int>(std::allocator<int>()));
  nonnull(
      std::allocate_shared_for_overwrite<int[]>(std::allocator<int[]>(), 5));
}

// Tests for `shared_ptr::..._pointer_cast`. We put these in a namespace so that
// the types we create for them don't "leak" out beyond the tests.
namespace pointer_casts {

struct Base {
  virtual ~Base();
};
struct Derived : public Base {
  ~Derived() override;
};

TEST void staticPointerCast(Nonnull<std::shared_ptr<Base>> NonnullParam,
                            Nullable<std::shared_ptr<Base>> NullableParam,
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

TEST void dynamicPointerCast(Nonnull<std::shared_ptr<Base>> NonnullParam,
                             Nullable<std::shared_ptr<Base>> NullableParam,
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

// `S` and `S::I` are pointer-interconvertible.
struct S {
  int I;
};

TEST void reinterpretPointerCast(Nonnull<std::shared_ptr<S>> NonnullParam,
                                 Nullable<std::shared_ptr<S>> NullableParam,
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

}  // namespace pointer_casts

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
struct UserDefinedSmartPointer {
  using absl_nullability_compatible = void;
  using pointer = T *;

  pointer get() const;
};

TEST void userDefinedSmartPointers(
    Nonnull<UserDefinedSmartPointer<int>> NonnullParam,
    Nullable<UserDefinedSmartPointer<int>> NullableParam,
    UserDefinedSmartPointer<int> UnknownParam) {
  // Just spot-check some basic behaviors, as the implementation treats
  // user-defined smart pointers like standard smart pointers, so the tests for
  // standard smart pointers provide sufficient coverage.

  nonnull(NonnullParam);
  nullable(NullableParam);
  unknown(UnknownParam);

  nonnull(NonnullParam.get());
  nullable(NullableParam.get());
  unknown(UnknownParam.get());
}

}  // namespace user_defined_smart_pointers

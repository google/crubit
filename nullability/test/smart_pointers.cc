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

#include "nullability_test.h"

Nonnull<std::unique_ptr<int>> makeNonnull();
Nullable<std::unique_ptr<int>> makeNullable();
std::unique_ptr<int> makeUnknown();

Nonnull<int *> makeNonnullRaw();
Nullable<int *> makeNullableRaw();
int *makeUnknownRaw();

TEST void parameterAnnotations(Nonnull<std::unique_ptr<int>> nonnullParam,
                               Nullable<std::unique_ptr<int>> nullableParam,
                               std::unique_ptr<int> unknownParam) {
  nonnull(nonnullParam);
  nullable(nullableParam);
  unknown(unknownParam);
}

TEST void returnValueAnnotations() {
  nonnull(makeNonnull());
  nullable(makeNullable());
  unknown(makeUnknown());
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

TEST void moveConstructor(Nonnull<std::unique_ptr<int>> nonnullParam,
                          Nullable<std::unique_ptr<int>> nullableParam,
                          std::unique_ptr<int> unknownParam) {
  nonnull(std::unique_ptr<int>(std::move(nonnullParam)));
  nullable(std::unique_ptr<int>(std::move(nullableParam)));
  unknown(std::unique_ptr<int>(std::move(unknownParam)));

  nullable(nonnullParam);
  nullable(nullableParam);
  nullable(unknownParam);
}

TEST void sharedPtrFromUniquePtr(Nonnull<std::unique_ptr<int>> nonnullParam,
                                 Nullable<std::unique_ptr<int>> nullableParam,
                                 std::unique_ptr<int> unknownParam) {
  nonnull(std::shared_ptr<int>(std::move(nonnullParam)));
  nullable(std::shared_ptr<int>(std::move(nullableParam)));
  unknown(std::shared_ptr<int>(std::move(unknownParam)));

  nullable(nonnullParam);
  nullable(nullableParam);
  nullable(unknownParam);
}

TEST void copyConstructor(Nonnull<std::shared_ptr<int>> nonnullParam,
                          Nullable<std::shared_ptr<int>> nullableParam,
                          std::shared_ptr<int> unknownParam) {
  nonnull(std::shared_ptr<int>(nonnullParam));
  nullable(std::shared_ptr<int>(nullableParam));
  unknown(std::shared_ptr<int>(unknownParam));

  nonnull(nonnullParam);
  nullable(nullableParam);
  unknown(unknownParam);
}

TEST void aliasingConstructor(Nonnull<std::shared_ptr<int>> nonnullParam) {
  nullable(std::shared_ptr<int>(nonnullParam, nullptr));
  nonnull(nonnullParam);

  nullable(std::shared_ptr<int>(std::move(nonnullParam), nullptr));
  nullable(nonnullParam);
}

TEST void sharedPtrFromWeakPtr(std::weak_ptr<int> weak) {
  nonnull(std::shared_ptr<int>(weak));
}

TEST void nullptrAssignment() {
  std::unique_ptr<int> p = makeUnknown();
  unknown(p);
  p = nullptr;
  nullable(p);
}

TEST void moveAssignment(Nonnull<std::unique_ptr<int>> nonnullParam,
                         Nullable<std::unique_ptr<int>> nullableParam,
                         std::unique_ptr<int> unknownParam) {
  std::unique_ptr<int> nonnullLocal;
  nonnull(nonnullLocal = std::move(nonnullParam));
  std::unique_ptr<int> nullableLocal;
  nullable(nullableLocal = std::move(nullableParam));
  std::unique_ptr<int> unknownLocal;
  unknown(unknownLocal = std::move(unknownParam));

  nullable(nonnullParam);
  nullable(nullableParam);
  nullable(unknownParam);
}

TEST void copyAssignment(Nonnull<std::shared_ptr<int>> nonnullParam,
                         Nullable<std::shared_ptr<int>> nullableParam,
                         std::shared_ptr<int> unknownParam) {
  std::shared_ptr<int> nonnullLocal;
  nonnull(nonnullLocal = nonnullParam);
  std::shared_ptr<int> nullableLocal;
  nullable(nullableLocal = nullableParam);
  std::shared_ptr<int> unknownLocal;
  unknown(unknownLocal = unknownParam);

  nonnull(nonnullParam);
  nullable(nullableParam);
  unknown(unknownParam);
}

TEST void release(Nonnull<std::unique_ptr<int>> nonnullParam,
                  Nullable<std::unique_ptr<int>> nullableParam,
                  std::unique_ptr<int> unknownParam) {
  nonnull(nonnullParam.release());
  nullable(nullableParam.release());
  unknown(unknownParam.release());

  nullable(nonnullParam);
  nullable(nullableParam);
  nullable(unknownParam);
}

TEST void reset() {
  {
    auto p = std::make_unique<int>();
    p.reset();
    provable(p.get() == nullptr);
  }

  {
    std::unique_ptr<int> p;
    int *raw = new int();
    p.reset(raw);
    provable(p.get() == raw);
  }

  {
    auto p = std::make_unique<int[]>(1);
    p.reset();
    provable(p.get() == nullptr);
  }

  {
    auto p = std::make_unique<int[]>(1);
    p.reset(nullptr);
    provable(p.get() == nullptr);
  }

  {
    std::unique_ptr<int[]> p;
    int *raw = new int[1];
    p.reset(raw);
    provable(p.get() == raw);
  }

  {
    auto p = std::make_shared<int>();
    p.reset();
    provable(p.get() == nullptr);
  }

  {
    std::shared_ptr<int> p;
    int *raw = new int();
    p.reset(raw);
    provable(p.get() == raw);
  }

  {
    std::shared_ptr<int> p;
    int *raw = new int();
    p.reset(raw, std::default_delete<int>());
    provable(p.get() == raw);
  }

  {
    std::shared_ptr<int> p;
    int *raw = new int();
    p.reset(raw, std::default_delete<int>(), std::allocator<int>());
    provable(p.get() == raw);
  }
}

TEST void swap() {
  {
    auto p1 = std::make_unique<int>();
    auto p2 = std::make_unique<int>();
    int *raw1 = p1.get();
    int *raw2 = p2.get();
    p1.swap(p2);
    provable(p1.get() == raw2);
    provable(p2.get() == raw1);
  }

  {
    auto p1 = std::make_unique<int>();
    auto p2 = std::make_unique<int>();
    int *raw1 = p1.get();
    int *raw2 = p2.get();
    std::swap(p1, p2);
    provable(p1.get() == raw2);
    provable(p2.get() == raw1);
  }
}

TEST void get(int *raw) {
  std::unique_ptr<int> null;
  provable(null.get() == nullptr);

  std::unique_ptr<int> p(raw);
  provable(p.get() == raw);

  // Test `->method()` call syntax.
  provable((&null)->get() == nullptr);
  provable((&p)->get() == raw);
}

TEST void operatorBool() {
  provable(!std::unique_ptr<int>());
  provable(static_cast<bool>(std::make_unique<int>()));

  // Test `->method()` call syntax.
  auto p = std::make_unique<int>();
  provable((&p)->operator bool());
}

TEST void operatorStar() {
  auto p = std::make_unique<bool>();
  *p = false;
  provable(!*p);
  *p = true;
  provable(*p);

  // TODO(mboehme): We'd like to be able to write the following, but the
  // dataflow framework currently doesn't consider two different `PointerValue`s
  // with the same location to be the same.
  // Instead, we need to take a slightly more indirect approach to testing
  // `operator*()`, see above. Once we have better pointer comparisons in the
  // framework, replace the test above with the test below.
  // auto p = std::make_unique<int>();
  // provable(p.get() == &*p);
}

TEST void operatorArrow() {
  std::unique_ptr<std::unique_ptr<int>> pp;
  int *raw = new int();
  pp->reset(raw);
  provable(pp->get() == raw);

  // TODO(mboehme): Replace the more indirect test above with this test below
  // once the framework considers two different `PointerValue`s with the same
  // location to be the same.
#if 0
  struct S {
    int i;
  };
  auto p = std::make_unique<S>();
  provable(&p.get()->b == &p->b);
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

TEST void staticPointerCast(Nonnull<std::shared_ptr<Base>> nonnullParam,
                            Nullable<std::shared_ptr<Base>> nullableParam,
                            std::shared_ptr<Base> unknownParam) {
  provable(std::static_pointer_cast<Derived>(std::shared_ptr<Base>()) ==
           nullptr);

  nonnull(std::static_pointer_cast<Derived>(nonnullParam));
  nullable(std::static_pointer_cast<Derived>(nullableParam));
  unknown(std::static_pointer_cast<Derived>(unknownParam));

  // Arguments are unchanged after calling const lvalue reference overload.
  nonnull(nonnullParam);
  nullable(nullableParam);
  unknown(unknownParam);

  nonnull(std::static_pointer_cast<Derived>(std::move(nonnullParam)));
  nullable(std::static_pointer_cast<Derived>(std::move(nullableParam)));
  unknown(std::static_pointer_cast<Derived>(std::move(unknownParam)));

  // Arguments are empty after calling rvalue reference overload.
  provable(!nonnullParam);
  provable(!nullableParam);
  provable(!unknownParam);
}

TEST void dynamicPointerCast(Nonnull<std::shared_ptr<Base>> nonnullParam,
                             Nullable<std::shared_ptr<Base>> nullableParam,
                             std::shared_ptr<Base> unknownParam) {
  provable(std::dynamic_pointer_cast<Derived>(std::shared_ptr<Base>()) ==
           nullptr);

  nullable(std::dynamic_pointer_cast<Derived>(nonnullParam));
  nullable(std::dynamic_pointer_cast<Derived>(nullableParam));
  nullable(std::dynamic_pointer_cast<Derived>(unknownParam));

  // Arguments are unchanged after calling const lvalue reference overload.
  nonnull(nonnullParam);
  nullable(nullableParam);
  unknown(unknownParam);

  nullable(std::dynamic_pointer_cast<Derived>(std::move(nonnullParam)));
  nullable(std::dynamic_pointer_cast<Derived>(std::move(nullableParam)));
  nullable(std::dynamic_pointer_cast<Derived>(std::move(unknownParam)));

  // Arguments are nullable (but not provably null) after calling rvalue
  // reference overload (because they may or may not have been moved from).
  nullable(nonnullParam);
  nullable(nullableParam);
  nullable(unknownParam);
  possible(nonnullParam != nullptr);
  possible(nullableParam != nullptr);
  possible(unknownParam != nullptr);

  // However, if the argument was null, then it should remain null (and not just
  // nullable) after calling the rvalue reference overload.
  std::shared_ptr<Base> null;
  provable(std::dynamic_pointer_cast<Derived>(null) == nullptr);
  provable(null == nullptr);
}

TEST void constPointerCast() {
  // A `const_pointer_cast`, unlike the other cast types, will definitely
  // produce a pointer with the same storage location as the source, so we can
  // test this cast more easily than the others.

  provable(std::const_pointer_cast<int>(std::shared_ptr<const int>()) ==
           nullptr);

  auto p = std::make_shared<const int>();
  provable(std::const_pointer_cast<int>(p).get() == p.get());
  provable(p != nullptr);
  std::const_pointer_cast<int>(std::move(p));
  provable(!p);
}

// `S` and `S::i` are pointer-interconvertible.
struct S {
  int i;
};

TEST void reinterpretPointerCast(Nonnull<std::shared_ptr<S>> nonnullParam,
                                 Nullable<std::shared_ptr<S>> nullableParam,
                                 std::shared_ptr<S> unknownParam) {
  // By the standard, the pointers we produce through `reinterpret_pointer_cast`
  // in this test should have the same address, but the dataflow framework does
  // not allow us to express this (as it requires different `StorageLocation`s
  // for different types). Therefore, we need to test `reinterpret_pointer_cast`
  // more indirectly, similar to `static_pointer_cast` and
  // `dynamic_pointer_cast` above.

  provable(std::reinterpret_pointer_cast<int>(std::shared_ptr<S>()) == nullptr);

  nonnull(std::reinterpret_pointer_cast<int>(nonnullParam));
  nullable(std::reinterpret_pointer_cast<int>(nullableParam));
  unknown(std::reinterpret_pointer_cast<int>(unknownParam));

  // Arguments are unchanged after calling const lvalue reference overload.
  nonnull(nonnullParam);
  nullable(nullableParam);
  unknown(unknownParam);

  nonnull(std::reinterpret_pointer_cast<int>(std::move(nonnullParam)));
  nullable(std::reinterpret_pointer_cast<int>(std::move(nullableParam)));
  unknown(std::reinterpret_pointer_cast<int>(std::move(unknownParam)));

  // Arguments are empty after calling rvalue reference overload.
  provable(!nonnullParam);
  provable(!nullableParam);
  provable(!unknownParam);
}

}  // namespace pointer_casts

TEST void operatorEqualsAndNotEquals() {
  // We perform this test on `shared_ptr` rather than `unique_ptr` because it
  // allows us the test to be stronger: We can check that two different
  // `shared_ptr`s with the same underlying raw pointer compare equal. We can't
  // test this with `unique_ptr` because it is, well, unique.
  auto p1 = std::make_shared<int>();
  auto p2 = std::make_shared<int>();
  std::shared_ptr<int> null;

  provable(p1 == p1);
  provable(p1 == std::shared_ptr<int>(p1));
  provable(null == std::shared_ptr<int>());

  provable(p1 != p2);
  provable(p1 != null);
  provable(p2 != null);

  provable(null == nullptr);
  provable(p1 != nullptr);
  provable(nullptr == null);
  provable(nullptr != p1);
}

// TODO(b/316410576): re-enable after bug fix.
// TEST void weakPtrLocReturnsNullable(std::shared_ptr<int> shared) {
//  std::weak_ptr<int> weak(shared);
//  nullable(weak.lock());
// }

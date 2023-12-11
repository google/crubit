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

TEST void get(int *raw) {
  std::unique_ptr<int> null;
  provable(null.get() == nullptr);

  std::unique_ptr<int> p(raw);
  provable(p.get() == raw);
}

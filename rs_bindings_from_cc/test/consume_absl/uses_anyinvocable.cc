// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/consume_absl/uses_anyinvocable.h"

#include <optional>
#include <utility>

#include "absl/functional/any_invocable.h"

void CallVoidVoid(absl::AnyInvocable<void() &&> f) { std::move(f)(); }

bool CallWithAnyInvocableParam(
    absl::AnyInvocable<void(absl::AnyInvocable<void() &&>)> f) {
  bool inner_called = false;
  f([&inner_called]() { inner_called = true; });
  return inner_called;
}

absl::AnyInvocable<int(int) const> ReturnIntMapper() {
  return [](int x) -> int { return x + 1; };
}
int CallIntInt(absl::AnyInvocable<int(int) const> f, int i) { return f(i); }

absl::AnyInvocable<std::optional<int>(std::optional<int>) const>
ReturnOptionalIntMapper() {
  return [](std::optional<int> x) -> std::optional<int> {
    if (x.has_value()) {
      return x.value() + 1;
    }
    return std::nullopt;
  };
}
std::optional<int> CallOptionalIntMapper(
    absl::AnyInvocable<std::optional<int>(std::optional<int>) const> f,
    std::optional<int> i) {
  return f(i);
}

absl::AnyInvocable<int*(int*) const> ReturnPointerMapper() {
  return [](int* x) -> int* { return x; };
}
int* CallPointerMapper(absl::AnyInvocable<int*(int*) const> f, int* i) {
  return f(i);
}

absl::AnyInvocable<int&(int&) const> ReturnReferenceMapper() {
  return [](int& x) -> int& { return x; };
}
int& CallReferenceMapper(absl::AnyInvocable<int&(int&) const> f, int& i) {
  return f(i);
}

absl::AnyInvocable<int(int&&) const> ReturnRvalueRefConsumer() {
  return [](int&& x) -> int { return x + 1; };
}
int CallRvalueRefConsumer(absl::AnyInvocable<int(int&&) const> f, int&& i) {
  return f(std::move(i));
}

absl::AnyInvocable<FuncType(FuncType) const> ReturnFuncPtrMapper() {
  return [](FuncType f) -> FuncType { return f; };
}
FuncType CallFuncPtrMapper(absl::AnyInvocable<FuncType(FuncType) const> f,
                           FuncType i) {
  return f(i);
}

absl::AnyInvocable<MyStruct(MyStruct) const> ReturnRecordMapper() {
  return [](MyStruct x) -> MyStruct { return MyStruct{x.value + 1}; };
}
MyStruct CallRecordMapper(absl::AnyInvocable<MyStruct(MyStruct) const> f,
                          MyStruct i) {
  return f(i);
}

absl::AnyInvocable<int(MyStruct&&) const> ReturnRecordRvalueRefConsumer() {
  return [](MyStruct&& x) -> int { return x.value + 1; };
}
int CallRecordRvalueRefConsumer(absl::AnyInvocable<int(MyStruct&&) const> f,
                                MyStruct&& i) {
  return f(std::move(i));
}

absl::AnyInvocable<MyEnum(MyEnum) const> ReturnEnumMapper() {
  return [](MyEnum x) -> MyEnum {
    return x == MyEnum::kZero ? MyEnum::kOne : MyEnum::kZero;
  };
}
MyEnum CallEnumMapper(absl::AnyInvocable<MyEnum(MyEnum) const> f, MyEnum i) {
  return f(i);
}

absl::AnyInvocable<MyAlias(MyAlias) const> ReturnTypeAliasMapper() {
  return [](MyAlias x) -> MyAlias { return x + 1; };
}
MyAlias CallTypeAliasMapper(absl::AnyInvocable<MyAlias(MyAlias) const> f,
                            MyAlias i) {
  return f(i);
}

absl::AnyInvocable<Incomplete*(Incomplete*) const>
ReturnIncompletePointerMapper() {
  return [](Incomplete* x) -> Incomplete* { return x; };
}
Incomplete* CallIncompletePointerMapper(
    absl::AnyInvocable<Incomplete*(Incomplete*) const> f, Incomplete* i) {
  return f(i);
}

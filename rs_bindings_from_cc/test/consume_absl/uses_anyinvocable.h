// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_USES_ANYINVOCABLE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_USES_ANYINVOCABLE_H_

#include <optional>

#include "absl/functional/any_invocable.h"

// Calls the invocable and returns void.
void CallVoidVoid(absl::AnyInvocable<void() &&> f);

bool CallWithAnyInvocableParam(
    absl::AnyInvocable<void(absl::AnyInvocable<void() &&>)> f);

// Returns an invocable that increments its argument.
absl::AnyInvocable<int(int) const> ReturnIntMapper();
int CallIntInt(absl::AnyInvocable<int(int) const> f, int i);

absl::AnyInvocable<std::optional<int>(std::optional<int>) const>
ReturnOptionalIntMapper();

std::optional<int> CallOptionalIntMapper(
    absl::AnyInvocable<std::optional<int>(std::optional<int>) const> f,
    std::optional<int> i);

// Pointer
absl::AnyInvocable<int*(int*) const> ReturnPointerMapper();
int* CallPointerMapper(absl::AnyInvocable<int*(int*) const> f, int* i);

// Reference
absl::AnyInvocable<int&(int&) const> ReturnReferenceMapper();
int& CallReferenceMapper(absl::AnyInvocable<int&(int&) const> f, int& i);

// RvalueReference
absl::AnyInvocable<int(int&&) const> ReturnRvalueRefConsumer();
int CallRvalueRefConsumer(absl::AnyInvocable<int(int&&) const> f, int&& i);

// FuncPtr
using FuncType = int (*)(int);
absl::AnyInvocable<FuncType(FuncType) const> ReturnFuncPtrMapper();
FuncType CallFuncPtrMapper(absl::AnyInvocable<FuncType(FuncType) const> f,
                           FuncType i);

// Record
struct MyStruct {
  int value;
};
absl::AnyInvocable<MyStruct(MyStruct) const> ReturnRecordMapper();
MyStruct CallRecordMapper(absl::AnyInvocable<MyStruct(MyStruct) const> f,
                          MyStruct i);
absl::AnyInvocable<int(MyStruct&&) const> ReturnRecordRvalueRefConsumer();
int CallRecordRvalueRefConsumer(absl::AnyInvocable<int(MyStruct&&) const> f,
                                MyStruct&& i);

// Enum
enum class MyEnum {
  kZero = 0,
  kOne = 1,
};
absl::AnyInvocable<MyEnum(MyEnum) const> ReturnEnumMapper();
MyEnum CallEnumMapper(absl::AnyInvocable<MyEnum(MyEnum) const> f, MyEnum i);

// TypeAlias
using MyAlias = int;
absl::AnyInvocable<MyAlias(MyAlias) const> ReturnTypeAliasMapper();
MyAlias CallTypeAliasMapper(absl::AnyInvocable<MyAlias(MyAlias) const> f,
                            MyAlias i);

// IncompleteRecord
struct Incomplete;
absl::AnyInvocable<Incomplete*(Incomplete*) const>
ReturnIncompletePointerMapper();
Incomplete* CallIncompletePointerMapper(
    absl::AnyInvocable<Incomplete*(Incomplete*) const> f, Incomplete* i);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_USES_ANYINVOCABLE_H_

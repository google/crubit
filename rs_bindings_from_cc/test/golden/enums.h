// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ENUMS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ENUMS_H_

enum Color {
  kRed,
  kBlue,
  kGreen,
};

enum [[clang::annotate("crubit_rust_name", "RenamedEnum")]] EnumToRename {};
enum EnumWithRenamedVariants {
  kFoo [[clang::annotate("crubit_rust_name", "FOO")]],
  kBar [[clang::annotate("crubit_rust_name", "BAR")]],
};

enum Empty {};
enum EmptyBool : bool {};
enum EmptyInt : unsigned int {};
enum EmptyChar : char {};

enum NonEmptyBool : bool { kBool1, kBool2 = true };
enum NonEmptyInt : unsigned int { kInt1, kInt2 = 4294967295 };
enum NonEmptyChar : char { kChar1, kChar2 = 'a' };

enum class EmptyClass {};
enum class EmptyBoolClass : bool {};
enum class EmptyIntClass : int {};
enum class EmptyCharClass : char {};

enum class NonEmptyBoolClass : bool { k1, k2 = true };
enum class NonEmptyIntClass : unsigned int { k1, k2 = 4294967295 };
enum class NonEmptyCharClass : char { k1, k2 = 'a' };

enum ForwardDeclared : int;

ForwardDeclared do_not_generate_bindings_for_me();

enum { kAnonRed, kAnonBlue, kAnonGreen };

enum EnumWithUnbindableEnumerator {
  kBindable,

  // `[[deprecated]]` is the only enumerator attribute that Crubit understands
  // (see `EnumConstantDeclImporter::Import`), so any other attribute makes an
  // enumerator unbindable.  `maybe_unused` is arbitrarily selected for being
  // accepted by our toolchain on an enumerator and unknown to Crubit.
  //
  // Keep in sync with `test_supported_unknown_attr_enumerator` in
  // `generate_bindings_test.rs`: that test fails loudly if Crubit ever starts
  // to recognize `maybe_unused` - the signal to pick another attribute here.
  kUnbindable [[maybe_unused]],
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ENUMS_H_

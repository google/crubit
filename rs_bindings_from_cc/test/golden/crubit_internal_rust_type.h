// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CRUBIT_INTERNAL_RS_TYPE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CRUBIT_INTERNAL_RS_TYPE_H_

#pragma clang lifetime_elision

// These types should be suppressed due to the rust type override, as should
// any methods they have.

struct [[clang::annotate("crubit_internal_rust_type", "i8")]] MyI8Struct final {
  signed char x;

  void Method();
};

inline void MyI8Struct::Method() {
  // Note that this is potentially visited, even if the original declaration is
  // skipped due to crubit_internal_rust_type.
}

struct [[clang::annotate("crubit_internal_rust_type", "i8")]] MyI8Class final {
  signed char x;
};

enum [[clang::annotate("crubit_internal_rust_type",
                       "i8")]] MyI8Enum : unsigned char{kX};

using MyI8Alias [[clang::annotate("crubit_internal_rust_type", "i8")]] =
    unsigned char;

// Invalid annotations cause bindings to fail to be generated.
// (It's important not to fall back to the underlying type, since the user
// intent was to override it.)
// Uncomment these invalid annotations to observe the build-time errors.
// TODO: b/402989591 - Use compile-fail UI test to check these outputs.
using TooFewArgs /*[[clang::annotate("crubit_internal_rust_type")]]*/ =
    unsigned char;
using TooManyArgs
    /*[[clang::annotate("crubit_internal_rust_type", "i8", "i8")]]*/
    = unsigned char;
using NonStringArg /*[[clang::annotate("crubit_internal_rust_type", 8)]]*/ =
    unsigned char;
using BadSameAbiAttr
    /*[[clang::annotate("crubit_internal_rust_type", "i8")]] [[clang::annotate(
        "crubit_internal_same_abi", true)]]*/
    = unsigned char;

struct ExistingRustTypeFieldTypes final {
  MyI8Struct my_i8_struct;
  MyI8Class my_i8_class;
  MyI8Enum my_i8_enum;
  MyI8Alias my_i8_alias;

  TooFewArgs error;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CRUBIT_INTERNAL_RS_TYPE_H_

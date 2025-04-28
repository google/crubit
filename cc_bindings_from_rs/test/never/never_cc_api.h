// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// never_golden
// Features: supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_NEVER_NEVER_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_NEVER_NEVER_GOLDEN

namespace never {

// Generated from:
// cc_bindings_from_rs/test/never/never.rs;l=8
[[noreturn]] void never_return();

// Generated from:
// cc_bindings_from_rs/test/never/never.rs;l=13
extern "C" [[noreturn]] void extern_never_return();

namespace __crubit_internal {
extern "C" [[noreturn]] void __crubit_thunk_never_ureturn();
}
inline void never_return() {
  __crubit_internal::__crubit_thunk_never_ureturn();
}

}  // namespace never
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_NEVER_NEVER_GOLDEN

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_C_ALLOWED_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_C_ALLOWED_H_

namespace crubit::has_bindings {
extern "C" {

inline void crubit_void_function() {}
inline const void* crubit_void_ptr_identity(const void* x) { return x; }
}
}  // namespace crubit::has_bindings
#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_C_ALLOWED_H_

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_THUNK_INCLUDES_USES_STD_STRING_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_THUNK_INCLUDES_USES_STD_STRING_H_

#include <string>

// With `template_instantiation`, this target's thunks also cover std templates
// that `<string>` instantiates (e.g. `std::reverse_iterator`, declared in a
// libc++ implementation header). The thunk file `#include`s the public headers
// (e.g. `<iterator>`) that provide them.
inline void TakeString(const std::string& s) {}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_THUNK_INCLUDES_USES_STD_STRING_H_

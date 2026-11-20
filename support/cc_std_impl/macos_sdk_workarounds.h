// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_MACOS_SDK_WORKAROUNDS_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_MACOS_SDK_WORKAROUNDS_H_

#if defined(__APPLE__)

#include <netinet/in.h>
#include <stdint.h>

// The macOS SDK declares these outside of `__BEGIN_DECLS`/`__END_DECLS`, so in
// C++ they have C++ linkage and calls to them fail to link against libSystem's
// C symbols. Don't generate bindings (and thunks) for them.
__attribute__((annotate("crubit_do_not_bind"))) int setipv4sourcefilter(
    int, struct in_addr, struct in_addr, uint32_t, uint32_t, struct in_addr*);
__attribute__((annotate("crubit_do_not_bind"))) int getipv4sourcefilter(
    int, struct in_addr, struct in_addr, uint32_t*, uint32_t*, struct in_addr*);

#endif  // defined(__APPLE__)

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_MACOS_SDK_WORKAROUNDS_H_

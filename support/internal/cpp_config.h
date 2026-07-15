// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Header controls for C++ that go out with the release.
//
// (Note: this is only relevant for codebases that separately release Crubit
// and its internal runtime libraries, while using the support libraries at
// head.)
//
// To release a change to a C++ support library in concert with the release, add
// a new `#define` to this file, and use `#ifdef` in C++ code to guard against
// it. That `#ifdef` will be active for testing at head, and for the next stable
// release. It will be inactive for the old release. This works because
// `cpp_config.h` itself is a part of the release.

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_CPP_CONFIG_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_CPP_CONFIG_H_

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_CPP_CONFIG_H_

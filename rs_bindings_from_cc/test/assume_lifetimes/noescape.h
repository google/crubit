// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_NOESCAPE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_NOESCAPE_H_

// `[[clang::noescape]]` on parameters, under `assume_lifetimes`.
//
// The importer records the attribute in the IR (`FuncParam::clang_noescape`),
// but nothing reads it yet: it is also still an unknown attribute, so a
// function with such a parameter is not bound. This golden pins that
// behaviour, so that the CL that starts reading the attribute shows up as a
// diff here.

// Bound: the control, without the attribute.
void Plain(int& x);

// Not bound today: unknown attribute `clang::noescape`.
void NoEscapeRef([[clang::noescape]] int& x);

// Not bound today, although only the pointer parameter carries it.
void NoEscapePtr([[clang::noescape]] int* p, int& q);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_NOESCAPE_H_

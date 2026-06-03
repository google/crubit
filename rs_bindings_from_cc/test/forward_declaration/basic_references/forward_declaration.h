// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_BASIC_REFERENCES_FORWARD_DECLARATION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_BASIC_REFERENCES_FORWARD_DECLARATION_H_

#define $(l) [[clang::annotate_type("lifetime", #l)]]

struct A;
A& $(static) fwd_source();
A& fwd_ident(A& a);
const A& fwd_ident_const(const A& a);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_BASIC_REFERENCES_FORWARD_DECLARATION_H_

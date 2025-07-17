// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This file contains shared functions that consume annotations, do various
// checks, and return the annotation or some derived value. Annotations should
// be defined in support/annotations.h

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_ANNOTATIONS_CONSUMER_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_ANNOTATIONS_CONSUMER_H_

#include <optional>

#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/DeclBase.h"

namespace crubit {

// Returns the `crubit_rust_name` annotation as an identifier for the given
// declaration, if present.
std::optional<Identifier> CrubitRustName(const clang::Decl& decl);

}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_ANNOTATIONS_CONSUMER_H_

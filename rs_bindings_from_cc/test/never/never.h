// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_NEVER_NEVER_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_NEVER_NEVER_H_

#include <optional>

#include "support/rs/never.h"

struct CRUBIT_INTERNAL_RUST_TYPE("!") CustomNever {
  CustomNever() = delete;
  CustomNever(const CustomNever&) = default;
  CustomNever& operator=(const CustomNever&) = default;
};

CustomNever MakeCustomNever();
void AcceptCustomNever(CustomNever);

rs::never_t MakeNeverT();
void AcceptNeverT(rs::never_t);

std::optional<CustomNever> ReturnOptionalCustomNever();
std::optional<rs::never_t> ReturnOptionalNeverT();

const rs::never_t* PassPtrNeverT(const rs::never_t* p);
void AcceptRefNeverT(const rs::never_t&);

#endif

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/never/never.h"

#include <cstdlib>
#include <optional>

#include "support/rs/never.h"

CustomNever MakeCustomNever() { std::abort(); }
void AcceptCustomNever(CustomNever) {}

rs::never_t MakeNeverT() { std::abort(); }
void AcceptNeverT(rs::never_t) {}

std::optional<CustomNever> ReturnOptionalCustomNever() { return std::nullopt; }
std::optional<rs::never_t> ReturnOptionalNeverT() { return std::nullopt; }

const rs::never_t* PassPtrNeverT(const rs::never_t* p) { return p; }
void AcceptRefNeverT(const rs::never_t&) {}

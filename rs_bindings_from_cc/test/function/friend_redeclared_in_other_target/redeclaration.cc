// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/function/friend_redeclared_in_other_target/redeclaration.h"

#include <string>

#include "rs_bindings_from_cc/test/function/friend_redeclared_in_other_target/friend.h"

std::string Describe(const Befriending& b) { return std::to_string(b.value_); }

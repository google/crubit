// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/manual_bridge_vocabulary_types/unique_ptr_lib.h"

#include <memory>

std::unique_ptr<VirtualBase> MakeVirtualBase() { return nullptr; }

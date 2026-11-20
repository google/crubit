// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "top.h"

int Visible() { return 4; }
int UsesHidden(Hidden h) { return h.z; }

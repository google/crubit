// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/test/bidirectional_deps/leaf_cc_lib.h"  // IWYU pragma: keep
#include "common/test/bidirectional_deps/middle_rs_lib_cc_api.h"  // IWYU pragma: keep

namespace {

// TODO(b/274834739): Test that CcType(RsType(X)) == X, and remove the IWYU
// pragmas.

}  // namespace

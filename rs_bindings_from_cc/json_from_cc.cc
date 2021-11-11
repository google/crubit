// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <string>

#include "base/logging.h"
#include "rs_bindings_from_cc/ffi_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_from_cc.h"
#include "third_party/absl/status/statusor.h"
#include "third_party/json/src/json.hpp"
#include "util/task/status.h"

namespace rs_bindings_from_cc {

// This is intended to be called from Rust.
extern "C" FfiU8SliceBox json_from_cc(FfiU8Slice cc_source) {
  absl::StatusOr<IR> ir = IrFromCc({StringViewFromFfiU8Slice(cc_source)});
  // TODO(forster): For now it is good enough to just exit: We are just using
  // this from tests, which are ok to just fail. Clang has already printed error
  // messages. If we start using this for production, then we should bridge the
  // error code into Rust.
  CHECK_OK(ir);
  std::string json = ir->ToJson().dump();
  return AllocFfiU8SliceBox(MakeFfiU8Slice(json));
}

}  // namespace rs_bindings_from_cc

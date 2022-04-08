// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <string>

#include "third_party/absl/status/statusor.h"
#include "common/ffi_types.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_from_cc.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/ErrorHandling.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/FormatVariadic.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/JSON.h"

namespace rs_bindings_from_cc {

// LINT.IfChange
static constexpr absl::string_view kDependencyTarget = "//test:dependency";

static constexpr absl::string_view kDependencyHeaderName =
    "test/dependency_header.h";
// LINT.ThenChange(//depot/rs_bindings_from_cc/ir_testing.rs)

// This is intended to be called from Rust.
extern "C" FfiU8SliceBox json_from_cc_dependency(
    FfiU8Slice header_source, FfiU8Slice dependency_header_source) {
  absl::StatusOr<IR> ir = IrFromCc(
      StringViewFromFfiU8Slice(header_source),
      BazelLabel{"//test:testing_target"},
      /* public_headers= */ {},
      {{HeaderName(std::string(kDependencyHeaderName)),
        std::string(StringViewFromFfiU8Slice(dependency_header_source))}},
      {{HeaderName(std::string(kDependencyHeaderName)),
        BazelLabel{std::string(kDependencyTarget)}}});
  // TODO(forster): For now it is good enough to just exit: We are just using
  // this from tests, which are ok to just fail. Clang has already printed error
  // messages. If we start using this for production, then we should bridge the
  // error code into Rust.
  if (!ir.ok()) {
    llvm::report_fatal_error(llvm::formatv("IrFromCc reported an error: {0}",
                                           ir.status().message()));
  }
  std::string json = llvm::formatv("{0}", ir->ToJson());
  return AllocFfiU8SliceBox(MakeFfiU8Slice(json));
}

}  // namespace rs_bindings_from_cc

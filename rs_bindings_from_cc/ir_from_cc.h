// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IR_FROM_CC_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IR_FROM_CC_H_

#include <string>

#include "third_party/absl/container/flat_hash_map.h"
#include "third_party/absl/status/statusor.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/absl/types/span.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"

namespace rs_bindings_from_cc {

// Parses C++ source code into IR.
//
// Parameters:
// * `extra_source_code`: C++ source code to be written into a virtual header
//   and included in binding generation. Intended for testing only.
// * `current_target`: full label of the target for which we generate bindings.
//   If not specified `//test:testing_target` is used.
// * `public_headers`: names of headers from which we build the Clang AST. If
//   `extra_source_code` is specified its header will be added automatically.
// * `virtual_headers_contents`: names and contents of virtual headers that
//   will be created in the virtual filesystem. These headers have to be
//   manually added to `public_headers` if needed.
// * `headers_to_targets`: mapping of headers to the label of the owning target.
//   If `extra_source_code` is specified it's added automatically under
//   `//test:testing_target`. Headers from `virtual_headers_contents` are not
//   added automatically.
// * `args`: additional command line arguments for Clang
//
absl::StatusOr<IR> IrFromCc(
    absl::string_view extra_source_code,
    BazelLabel current_target = BazelLabel{"//test:testing_target"},
    absl::Span<const HeaderName> public_headers = {},
    absl::flat_hash_map<const HeaderName, const std::string>
        virtual_headers_contents = {},
    absl::flat_hash_map<const HeaderName, const BazelLabel> headers_to_targets =
        {},
    absl::Span<const absl::string_view> args = {});

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_FROM_CC_H_

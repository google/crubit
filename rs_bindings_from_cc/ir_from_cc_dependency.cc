// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir_from_cc_dependency.h"

#include <string>
#include <utility>

#include "absl/container/flat_hash_set.h"
#include "absl/status/statusor.h"
#include "common/ffi_types.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_from_cc.h"

namespace crubit {

absl::StatusOr<IR> IrFromCcDependency(FfiU8Slice target_triple,
                                      FfiU8Slice header_source,
                                      FfiU8Slice dependency_header_source,
                                      FfiU8Slice extra_feature,
                                      bool kythe_annotations) {
  absl::flat_hash_set<std::string> features = {"supported"};
  if (extra_feature.size != 0) {
    features.insert(std::string(StringViewFromFfiU8Slice(extra_feature)));
  }
  return IrFromCc({
      .extra_source_code_for_testing = StringViewFromFfiU8Slice(header_source),
      .current_target = BazelLabel{"//test:testing_target"},
      .virtual_headers_contents_for_testing =
          {{HeaderName(std::string(kDependencyHeaderName)),
            std::string(StringViewFromFfiU8Slice(dependency_header_source))}},
      .headers_to_targets = {{HeaderName(std::string(kDependencyHeaderName)),
                              BazelLabel{std::string(kDependencyTarget)}}},
      .clang_args =
          {
              // The version should be consistent with the one passed by the C++
              // toolchain.
              "-std=gnu++20",
              "-target",
              StringViewFromFfiU8Slice(target_triple),
          },
      .crubit_features = {{BazelLabel{std::string(kDependencyTarget)},
                           features},
                          {BazelLabel{"//test:testing_target"},
                           std::move(features)}},
      .kythe_annotations = kythe_annotations,
  });
}

}  // namespace crubit

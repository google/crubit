// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "migrator/rs_from_cc/rs_from_cc_lib.h"

#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "absl/strings/substitute.h"
#include "absl/types/span.h"
#include "migrator/rs_from_cc/converter.h"
#include "migrator/rs_from_cc/frontend_action.h"
#include "clang/include/clang/Basic/FileManager.h"
#include "clang/include/clang/Basic/FileSystemOptions.h"
#include "clang/include/clang/Frontend/FrontendAction.h"
#include "clang/include/clang/Tooling/Tooling.h"

namespace crubit_rs_from_cc {

absl::StatusOr<std::string> RsFromCc(const absl::string_view cc_file_content,
                                     const absl::string_view cc_file_name,
                                     absl::Span<const absl::string_view> args) {
  std::vector<std::string> args_as_strings{
      // Parse non-doc comments that are used as documention
      "-fparse-all-comments"};
  args_as_strings.insert(args_as_strings.end(), args.begin(), args.end());

  Converter::Invocation invocation;
  if (clang::tooling::runToolOnCodeWithArgs(
          std::make_unique<FrontendAction>(invocation), cc_file_content,
          args_as_strings, cc_file_name, "rs_from_cc",
          std::make_shared<clang::PCHContainerOperations>(),
          clang::tooling::FileContentMappings())) {
    return invocation.rs_code_;
  } else {
    return absl::Status(absl::StatusCode::kInvalidArgument,
                        "Could not compile source file contents");
  }
}

}  // namespace crubit_rs_from_cc

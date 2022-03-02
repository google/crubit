// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/cmdline.h"

#include <algorithm>
#include <iterator>
#include <string>
#include <utility>
#include <vector>

#include "third_party/absl/flags/flag.h"
#include "third_party/absl/strings/substitute.h"
#include "third_party/json/include/nlohmann/json.hpp"
#include "util/task/status_macros.h"

ABSL_FLAG(bool, do_nothing, false,
          "if set to true the tool will produce empty files "
          "(useful for testing Blaze integration)");
ABSL_FLAG(std::string, rs_out, "",
          "output path for the Rust source file with bindings");
ABSL_FLAG(std::string, cc_out, "",
          "output path for the C++ source file with bindings implementation");
ABSL_FLAG(std::string, ir_out, "",
          "(optional) output path for the JSON IR. If not present, the JSON IR "
          "will not be dumped.");
ABSL_FLAG(std::vector<std::string>, public_headers, std::vector<std::string>(),
          "public headers of the cc_library this tool should generate bindings "
          "for, in a format suitable for usage in google3-relative quote "
          "include (#include \"\").");
ABSL_FLAG(std::string, targets_and_headers, std::string(),
          "Information about which headers belong to which targets, encoded as "
          "a JSON array. For example: "
          "[\n"
          "  {\n"
          "     \"t\": \"//foo/bar:baz\",\n"
          "     \"h\": [\"foo/bar/header1.h\", \"foo/bar/header2.h\"]\n"
          "  },\n"
          "...\n"
          "]");

namespace rs_bindings_from_cc {

absl::StatusOr<Cmdline> Cmdline::Create() {
  return CreateFromArgs(
      absl::GetFlag(FLAGS_cc_out), absl::GetFlag(FLAGS_rs_out),
      absl::GetFlag(FLAGS_ir_out), absl::GetFlag(FLAGS_do_nothing),
      absl::GetFlag(FLAGS_public_headers),
      absl::GetFlag(FLAGS_targets_and_headers));
}

absl::StatusOr<Cmdline> Cmdline::CreateFromArgs(
    std::string cc_out, std::string rs_out, std::string ir_out, bool do_nothing,
    std::vector<std::string> public_headers,
    std::string targets_and_headers_str) {
  Cmdline cmdline;

  if (rs_out.empty()) {
    return absl::InvalidArgumentError("please specify --rs_out");
  }
  cmdline.rs_out_ = std::move(rs_out);

  if (cc_out.empty()) {
    return absl::InvalidArgumentError("please specify --cc_out");
  }
  cmdline.cc_out_ = std::move(cc_out);

  cmdline.ir_out_ = std::move(ir_out);
  cmdline.do_nothing_ = do_nothing;

  if (public_headers.empty()) {
    return absl::InvalidArgumentError("please specify --public_headers");
  }
  std::transform(public_headers.begin(), public_headers.end(),
                 std::back_inserter(cmdline.public_headers_),
                 [](const std::string& s) { return HeaderName(s); });

  if (targets_and_headers_str.empty()) {
    return absl::InvalidArgumentError("please specify --targets_and_headers");
  }
  nlohmann::json targets_and_headers =
      nlohmann::json::parse(std::move(targets_and_headers_str),
                            /* cb= */ nullptr,
                            /* allow_exceptions= */ false);
  if (!targets_and_headers.is_array()) {
    return absl::InvalidArgumentError(
        "Expected `--targets_and_headers` to be a JSON array of objects");
  }
  for (const auto& target_and_headers : targets_and_headers) {
    if (!target_and_headers.contains("t")) {
      return absl::InvalidArgumentError(
          "Missing `t` field in an `--targets_and_headers` object");
    }
    if (!target_and_headers["t"].is_string()) {
      return absl::InvalidArgumentError(
          "Expected `t` fields of `--targets_and_headers` to be a string");
    }
    if (!target_and_headers.contains("h")) {
      return absl::InvalidArgumentError(
          "Missing `h` field in an `--targets_and_headers` object");
    }
    if (!target_and_headers["h"].is_array()) {
      return absl::InvalidArgumentError(
          "Expected `h` fields of `--targets_and_headers` to be an array");
    }
    BlazeLabel target{std::string(target_and_headers["t"])};
    if (target.value().empty()) {
      return absl::InvalidArgumentError(
          "Expected `t` fields of `--targets_and_headers` to be a non-empty "
          "string");
    }
    for (const auto& header : target_and_headers["h"]) {
      if (!header.is_string()) {
        return absl::InvalidArgumentError(
            "Expected `h` fields of `--targets_and_headers` to be an array of "
            "strings");
      }
      std::string header_str(header);
      if (header_str.empty()) {
        return absl::InvalidArgumentError(
            "Expected `h` fields of `--targets_and_headers` to be an array of "
            "non-empty strings");
      }
      const auto [it, inserted] = cmdline.headers_to_targets_.insert(
          std::make_pair(HeaderName(header_str), target));
      if (!inserted) {
        return absl::InvalidArgumentError(absl::Substitute(
            "The `--targets_and_headers` cmdline argument assigns "
            "`$0` header to two conflicting targets: `$1` vs `$2`",
            header_str, target.value(), it->second.value()));
      }
    }
  }

  ASSIGN_OR_RETURN(cmdline.current_target_,
                   cmdline.FindHeader(cmdline.public_headers_[0]));
  for (const HeaderName& public_header : cmdline.public_headers_) {
    ASSIGN_OR_RETURN(BlazeLabel header_target,
                     cmdline.FindHeader(public_header));

    if (cmdline.current_target_ != header_target) {
      return absl::InvalidArgumentError(absl::Substitute(
          "Expected all public headers to belong to the current target '$0', "
          "but header '$1' belongs to '$2'",
          cmdline.current_target_.value(), public_header.IncludePath(),
          header_target.value()));
    }
  }

  return cmdline;
}

absl::StatusOr<BlazeLabel> Cmdline::FindHeader(const HeaderName& header) const {
  auto it = headers_to_targets_.find(header);
  if (it == headers_to_targets_.end()) {
    return absl::InvalidArgumentError(absl::Substitute(
        "Couldn't find header '$0' in the `headers_to_target` map "
        "derived from the --targets_and_headers cmdline argument",
        header.IncludePath()));
  }
  return it->second;
}

Cmdline::Cmdline() = default;

}  // namespace rs_bindings_from_cc

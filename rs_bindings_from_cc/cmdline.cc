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
#include "third_party/absl/strings/str_cat.h"
#include "third_party/absl/strings/substitute.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/JSON.h"
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

namespace {

struct TargetAndHeaders {
  std::string target;
  std::vector<std::string> headers;
};

bool fromJSON(const llvm::json::Value& json, TargetAndHeaders& out,
              llvm::json::Path path) {
  llvm::json::ObjectMapper mapper(json, path);
  return mapper && mapper.map("t", out.target) && mapper.map("h", out.headers);
}

}  // namespace

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
  auto targets_and_headers = llvm::json::parse<std::vector<TargetAndHeaders>>(
      std::move(targets_and_headers_str));
  if (auto err = targets_and_headers.takeError()) {
    return absl::InvalidArgumentError(
        absl::StrCat("Malformed `--targets_and_headers` argument: ",
                     toString(std::move(err))));
  }
  for (const TargetAndHeaders& it : *targets_and_headers) {
    const std::string& target = it.target;
    if (target.empty()) {
      return absl::InvalidArgumentError(
          "Expected `t` fields of `--targets_and_headers` to be a non-empty "
          "string");
    }
    for (const std::string& header : it.headers) {
      if (header.empty()) {
        return absl::InvalidArgumentError(
            "Expected `h` fields of `--targets_and_headers` to be an array of "
            "non-empty strings");
      }
      const auto [it, inserted] = cmdline.headers_to_targets_.insert(
          std::make_pair(HeaderName(header), BlazeLabel(target)));
      if (!inserted) {
        return absl::InvalidArgumentError(absl::Substitute(
            "The `--targets_and_headers` cmdline argument assigns "
            "`$0` header to two conflicting targets: `$1` vs `$2`",
            header, target, it->second.value()));
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

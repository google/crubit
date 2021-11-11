// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Parses C++ headers and generates:
// * a Rust source file with bindings for the C++ API
// * a C++ source file with the implementation of the bindings

#include <string>
#include <utility>
#include <vector>

#include "base/init_google.h"
#include "base/logging.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_from_cc.h"
#include "rs_bindings_from_cc/src_code_gen.h"
#include "file/base/filesystem.h"
#include "file/base/helpers.h"
#include "file/base/options.h"
#include "third_party/absl/container/flat_hash_map.h"
#include "third_party/absl/flags/flag.h"
#include "third_party/absl/meta/type_traits.h"
#include "third_party/absl/status/status.h"
#include "third_party/absl/status/statusor.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/absl/strings/substitute.h"
#include "third_party/json/src/json.hpp"
#include "util/task/status.h"

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

int main(int argc, char* argv[]) {
  InitGoogle(argv[0], &argc, &argv, true);

  auto rs_out = absl::GetFlag(FLAGS_rs_out);
  QCHECK(!rs_out.empty()) << "please specify --rs_out";
  auto cc_out = absl::GetFlag(FLAGS_cc_out);
  QCHECK(!cc_out.empty()) << "please specify --cc_out";
  auto public_headers = absl::GetFlag(FLAGS_public_headers);
  QCHECK(!public_headers.empty())
      << "please specify at least one header in --public_headers";
  auto targets_and_headers_json = absl::GetFlag(FLAGS_targets_and_headers);
  QCHECK(!targets_and_headers_json.empty())
      << "please specify --targets_and_headers";
  absl::flat_hash_map<const rs_bindings_from_cc::HeaderName,
                      const rs_bindings_from_cc::Label>
      headers_to_targets;
  if (!targets_and_headers_json.empty()) {
    nlohmann::json targets_and_headers =
        nlohmann::json::parse(targets_and_headers_json);
    QCHECK(targets_and_headers.is_array())
        << "Expected `--targets_and_headers` to be a Json array of objects";
    for (const auto& target_and_headers : targets_and_headers) {
      rs_bindings_from_cc::Label target =
          rs_bindings_from_cc::Label{target_and_headers["t"]};
      QCHECK(target_and_headers["h"].is_array())
          << "Expected `h` fields of `--targets_and_headers` "
             "to be an array of strings";
      for (std::string header : target_and_headers["h"]) {
        headers_to_targets.insert(
            std::pair<const rs_bindings_from_cc::HeaderName,
                      const rs_bindings_from_cc::Label>(
                rs_bindings_from_cc::HeaderName(header), target));
      }
    }
  }

  rs_bindings_from_cc::Label current_target =
      headers_to_targets
          .find(rs_bindings_from_cc::HeaderName(public_headers[0]))
          ->second;
  for (const auto& public_header : public_headers) {
    rs_bindings_from_cc::Label header_target =
        headers_to_targets.find(rs_bindings_from_cc::HeaderName(public_header))
            ->second;
    QCHECK(current_target == header_target)
        << "Expected all public headers to belong to the current target '"
        << current_target << "', but header '" << public_header
        << "' belongs to '" << header_target << "'";
  }

  auto ir_out = absl::GetFlag(FLAGS_ir_out);  // Optional.
  if (absl::StatusOr<rs_bindings_from_cc::IR> ir =
          rs_bindings_from_cc::IrFromCc(
              /* extra_source_code= */ "", current_target,
              std::vector<rs_bindings_from_cc::HeaderName>(
                  public_headers.begin(), public_headers.end()),
              /* virtual_headers_contents= */ {}, std::move(headers_to_targets),
              std::vector<absl::string_view>(argv, argv + argc));
      ir.ok()) {
    if (!ir_out.empty()) {
      CHECK_OK(file::SetContents(ir_out, ir->ToJson().dump(/*indent=*/2),
                                 file::Defaults()));
    }
    rs_bindings_from_cc::Bindings bindings =
        rs_bindings_from_cc::GenerateBindings(*ir);
    CHECK_OK(file::SetContents(rs_out, bindings.rs_api, file::Defaults()));
    CHECK_OK(file::SetContents(cc_out, bindings.rs_api_impl, file::Defaults()));
    return 0;
  }

  file::Delete(rs_out, file::Defaults()).IgnoreError();
  file::Delete(cc_out, file::Defaults()).IgnoreError();
  if (!ir_out.empty()) {
    file::Delete(ir_out, file::Defaults()).IgnoreError();
  }
  return 1;
}

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/cmdline.h"

#include <algorithm>
#include <fstream>
#include <iterator>
#include <optional>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#include "absl/debugging/leak_check.h"
#include "absl/flags/flag.h"
#include "absl/log/log.h"
#include "absl/status/status.h"
#include "absl/strings/match.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "absl/strings/substitute.h"
#include "common/ffi_types.h"
#include "common/status_macros.h"
#include "common/string_view_conversion.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/cmdline_flags.h"
#include "rs_bindings_from_cc/ir.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/JSON.h"

ABSL_FLAG(bool, do_nothing, false,
          "if set to true the tool will produce empty files "
          "(useful for testing Bazel integration)");
ABSL_FLAG(std::string, driver_path, "",
          "path to the rs_bindings_from_cc_driver script");
ABSL_FLAG(std::string, rs_out, "",
          "output path for the Rust source file with bindings");
ABSL_FLAG(std::string, cc_out, "",
          "output path for the C++ source file with bindings implementation");
ABSL_FLAG(std::string, ir_out, "",
          "(optional) output path for the JSON IR. If not present, the JSON IR "
          "will not be dumped.");
ABSL_FLAG(std::string, crubit_support_path_format, "",
          "the format of `#include` for including Crubit C++ support library "
          "headers in the "
          "generated .cc files, in the format specifier, use `{header}` as the "
          "placeholder. For "
          "example, to include `support_header.h` as "
          "`#include <crubit/support/support_header.h>, specify "
          "`<crubit/support/{header}>`; for "
          "`#include \"crubit/support/support_header.h\", specify "
          "`\"crubit/support/{header}`,");
ABSL_FLAG(std::string, clang_format_exe_path, "",
          "Path to a clang-format executable that will be used to format the "
          ".cc files generated by the tool.");
ABSL_FLAG(std::string, rustfmt_exe_path, "",
          "Path to a rustfmt executable that will be used to format the "
          ".rs files generated by the tool.");
ABSL_FLAG(std::string, rustfmt_config_path, "",
          "(optional) path to a rustfmt.toml file that should replace the "
          "default formatting of the .rs files generated by the tool.");
ABSL_FLAG(std::vector<std::string>, public_headers, std::vector<std::string>(),
          "public headers of the cc_library this tool should generate bindings "
          "for, in a format suitable for usage in a quote "
          "include (#include \"\"). Note that these can be a subset of headers "
          "attributed to the target via target_args. For example, "
          "unparseable headers may be removed frp, public_headers, but kept "
          "attributed to that target in target_args.");
ABSL_FLAG(std::string, target, "", "The target to generate bindings for.");
ABSL_FLAG(std::string, target_args, "",
          "Per-target Crubit arguments, encoded as a JSON array. This contains "
          "both the list of headers assigned to the target (h), and the set of "
          "enabled features (f). For example:"
          "[\n"
          "  {\n"
          "     \"t\": \"//foo/bar:baz\",\n"
          "     \"h\": [\"foo/bar/header1.h\", \"foo/bar/header2.h\"],\n"
          "     \"f\": [\"supported\"]\n"
          "  },\n"
          "...\n"
          "]");
ABSL_FLAG(std::vector<std::string>, extra_rs_srcs, std::vector<std::string>(),
          "Additional Rust source files to include into the crate.");
ABSL_FLAG(std::vector<std::string>, srcs_to_scan_for_instantiations,
          std::vector<std::string>(),
          "[template instantiation mode only] all Rust source files of a crate "
          "for which we are instantiating templates.");
ABSL_FLAG(std::optional<std::vector<std::string>>, do_not_bind_allowlist,
          std::nullopt,
          "List of decls that are allowed to be omitted from bindings."
          " If omitted, `CRUBIT_DO_NOT_BIND` can be used on any decl.");
ABSL_FLAG(std::string, instantiations_out, "",
          "[template instantiation mode only] output path for the JSON file "
          "with mapping from a template instantiation to a generated Rust "
          "struct name. This file is used by cc_template! macro expansion.");
ABSL_FLAG(std::string, namespaces_out, "",
          "(optional) output path for the JSON file containing the target's"
          "namespace hierarchy.");
ABSL_FLAG(std::string, error_report_out, "",
          "(optional) output path for the JSON error report");
ABSL_FLAG(std::string, environment, "production",
          "The environment that the bindings are generated for. When set to "
          "'production', non mandatory (but potentially useful) information is "
          "generated. When set to 'golden_test', unnecessary information is "
          "omitted to reduce noise.");
// TODO(okabayashi): This is now an alias for --environment.
// Remove this flag once the alias is no longer used.
ABSL_FLAG(bool, generate_source_location_in_doc_comment, true,
          "add the source code location from which the binding originates in"
          "the doc comment of the binding")
    .OnUpdate([] {
      absl::SetFlag(&FLAGS_environment,
                    absl::GetFlag(FLAGS_generate_source_location_in_doc_comment)
                        ? "production"
                        : "golden_test");
    });

namespace crubit {

namespace {

struct TargetArgs {
  std::string target;
  std::vector<std::string> headers;
  std::vector<std::string> features;
};

bool fromJSON(const llvm::json::Value& json, TargetArgs& out,
              llvm::json::Path path) {
  llvm::json::ObjectMapper mapper(json, path);
  return mapper && mapper.map("t", out.target) &&
         mapper.mapOptional("h", out.headers) &&
         mapper.mapOptional("f", out.features);
}

std::vector<HeaderName> PublicHeaders() {
  std::vector<HeaderName> public_headers;
  const std::vector<std::string>& public_headers_string =
      absl::GetFlag(FLAGS_public_headers);
  std::transform(public_headers_string.begin(), public_headers_string.end(),
                 std::back_inserter(public_headers),
                 [](const std::string& s) { return HeaderName(s); });
  return public_headers;
}
}  // namespace

namespace internal {
absl::Status ParseTargetArgs(absl::string_view target_args_str,
                             CmdlineArgs& args) {
  if (target_args_str.empty()) {
    return absl::InvalidArgumentError("please specify --target_args");
  }
  auto target_args = llvm::json::parse<std::vector<TargetArgs>>(
      StringRefFromStringView(target_args_str));
  if (auto err = target_args.takeError()) {
    return absl::InvalidArgumentError(absl::StrCat(
        "Malformed `--target_args` argument: ", toString(std::move(err))));
  }
  for (const TargetArgs& it : *target_args) {
    const std::string& target = it.target;
    if (target.empty()) {
      return absl::InvalidArgumentError(
          "Expected `t` fields of `--target_args` to be a non-empty "
          "string");
    }
    for (const std::string& header : it.headers) {
      if (header.empty()) {
        return absl::InvalidArgumentError(
            "Expected `h` (header) fields of `--target_args` to be an "
            "array of non-empty strings");
      }
      BazelLabel target_label(target);
      auto [it, inserted] = args.headers_to_targets.try_emplace(
          HeaderName(header), std::move(target_label));
      if (!inserted) {
        LOG(WARNING) << "The `--target_args` cmdline argument assigns `"
                     << header << "` header to two conflicting targets: `"
                     << target << "` vs `" << it->second.value() << "`";
        // Assign the one that comes first alphabetically, to get a consistent
        // result.
        if (target_label.value() < it->second.value()) {
          it->second = std::move(target_label);
        }
      }
    }
    for (const std::string& feature : it.features) {
      if (feature.empty()) {
        return absl::InvalidArgumentError(
            "Expected `f` (feature) fields of `--target_args` to be an "
            "array of non-empty strings");
      }
      args.target_to_features[BazelLabel(target)].insert(feature);
    }
  }
  return absl::OkStatus();
}

absl::Status ParseEnvironment(absl::string_view environment_str,
                              CmdlineArgs& args) {
  if (environment_str == "production") {
    args.environment = Environment::Production;
    return absl::OkStatus();
  } else if (environment_str == "golden_test") {
    args.environment = Environment::GoldenTest;
    return absl::OkStatus();
  } else {
    return absl::InvalidArgumentError(
        absl::StrCat("Unknown environment: ", environment_str));
  }
}

}  // namespace internal

absl::StatusOr<Cmdline> Cmdline::FromFlags() {
  auto args = CmdlineArgs{
      .current_target = BazelLabel(absl::GetFlag(FLAGS_target)),
      .driver_path = absl::GetFlag(FLAGS_driver_path),
      .cc_out = absl::GetFlag(FLAGS_cc_out),
      .rs_out = absl::GetFlag(FLAGS_rs_out),
      .ir_out = absl::GetFlag(FLAGS_ir_out),
      .namespaces_out = absl::GetFlag(FLAGS_namespaces_out),
      .crubit_support_path_format =
          absl::GetFlag(FLAGS_crubit_support_path_format),
      .clang_format_exe_path = absl::GetFlag(FLAGS_clang_format_exe_path),
      .rustfmt_exe_path = absl::GetFlag(FLAGS_rustfmt_exe_path),
      .rustfmt_config_path = absl::GetFlag(FLAGS_rustfmt_config_path),
      .error_report_out = absl::GetFlag(FLAGS_error_report_out),
      .do_nothing = absl::GetFlag(FLAGS_do_nothing),
      .public_headers = PublicHeaders(),
      .extra_rs_srcs = absl::GetFlag(FLAGS_extra_rs_srcs),
      .srcs_to_scan_for_instantiations =
          absl::GetFlag(FLAGS_srcs_to_scan_for_instantiations),
      .instantiations_out = absl::GetFlag(FLAGS_instantiations_out),
      .do_not_bind_allowlist = absl::GetFlag(FLAGS_do_not_bind_allowlist)};
  absl::Status parse_environment_status =
      internal::ParseEnvironment(absl::GetFlag(FLAGS_environment), args);

  absl::Status parse_target_args_status =
      internal::ParseTargetArgs(absl::GetFlag(FLAGS_target_args), args);

  absl::StatusOr<Cmdline> cmdline = Cmdline::Create(std::move(args));
  if (!parse_target_args_status.ok() || !parse_environment_status.ok() ||
      !cmdline.ok()) {
    return absl::InvalidArgumentError(
        absl::StrCat(cmdline.status().message(), cmdline.ok() ? "" : "\n",
                     parse_target_args_status.message(),
                     parse_target_args_status.ok() ? "" : "\n",
                     parse_environment_status.message()));
  }
  return cmdline;
}

absl::StatusOr<Cmdline> Cmdline::Create(CmdlineArgs args) {
  std::string error;
  if (args.current_target.empty()) {
    absl::StrAppend(&error, "please specify --target\n");
  }
  if (args.rs_out.empty()) {
    absl::StrAppend(&error, "please specify --rs_out\n");
  }
  if (args.cc_out.empty()) {
    absl::StrAppend(&error, "please specify --cc_out\n");
  }
  if (args.public_headers.empty()) {
    absl::StrAppend(&error, "please specify --public_headers\n");
  }
  if (args.clang_format_exe_path.empty()) {
    absl::StrAppend(&error, "please specify --clang_format_exe_path\n");
  }
  if (args.rustfmt_exe_path.empty()) {
    absl::StrAppend(&error, "please specify --rustfmt_exe_path\n");
  }

  if (args.crubit_support_path_format.empty()) {
    absl::StrAppend(&error, "please specify --crubit_support_path_format\n");
  } else if (!absl::StrContains(args.crubit_support_path_format, "{header}")) {
    absl::StrAppend(
        &error,
        "cannot find `{header}` placeholder in crubit_support_path_format\n");
  }
  if (args.srcs_to_scan_for_instantiations.empty() !=
      args.instantiations_out.empty()) {
    absl::StrAppend(
        &error,
        "please specify both --rust_sources and --instantiations_out when "
        "requesting a template instantiation mode\n");
  }
  for (const HeaderName& header : args.public_headers) {
    if (auto it = args.headers_to_targets.find(header);
        it == args.headers_to_targets.end()) {
      absl::StrAppend(
          &error,
          absl::Substitute(
              "Couldn't find header '$0' in the `headers_to_target` map "
              "derived from the --target_args cmdline argument\n",
              header.IncludePath()));
    }
  }
  if (!error.empty()) {
    error.erase(error.size() - 1);  // remove trailing \n
    return absl::InvalidArgumentError(error);
  }
  return Cmdline(std::move(args));
}

void ExpandParamfiles(int& argc, char**& argv) {
  std::vector<char*> new_argv;  // Will be leaked if we find a paramfile.
  char** begin = argv;
  char** end = begin + argc;
  for (;;) {
    char** next_paramfile =
        std::find_if(begin, end, [](char* c) { return c[0] == '@'; });
    if (next_paramfile == end) break;
    new_argv.insert(new_argv.end(), begin, next_paramfile);
    begin = next_paramfile + 1;
    const char* paramfile = *next_paramfile + 1;
    std::ifstream in(paramfile);
    std::stringstream ss;
    ss << in.rdbuf();
    std::string s = ss.str();
    std::string next_arg;
    // Unfortunately, we can't just use something like StrReplaceAll, because an
    // escaped newline should be part of the value, while an unescaped newline
    // should not be.
    for (auto it = s.begin(); it != s.end(); ++it) {
      if (*it == '\\') {
        ++it;
        if (it == s.end()) {
          absl::StrAppend(&next_arg, "\\");
          break;
        } else {
          absl::StrAppend(&next_arg, absl::string_view(&*it, 1));
        }
      } else if (*it == '\n') {
        new_argv.push_back(
            absl::IgnoreLeak(new auto(std::move(next_arg)))->data());
        next_arg.clear();
      } else {
        absl::StrAppend(&next_arg, absl::string_view(&*it, 1));
      }
    }
    if (!next_arg.empty()) {
      new_argv.push_back(
          absl::IgnoreLeak(new auto(std::move(next_arg)))->data());
    }
  }
  if (begin == argv) return;
  new_argv.insert(new_argv.end(), begin, end);
  argv = new_argv.data();
  argc = new_argv.size();
  absl::IgnoreLeak(new auto(std::move(new_argv)));  // nowhere else to put it.
}

void PreprocessTargetArgs(int& argc, char** argv) {
  // TODO(jeanpierreda): Now that flag parsing logic is no longer in a bash script,
  // this should probably skip target_args entirely. (For now, the target_args
  // flag is left in place for compatibility and to avoid test churn.) We put it
  // in the place of the first --target_to_arg flag, because we can't put it at
  // the end, because of `--`.
  char** end = argv + argc;
  static constexpr absl::string_view kTargetToArg = "--target_to_arg";
  char** first_target_to_arg = std::find(argv, end, kTargetToArg);
  if (first_target_to_arg == end) {
    return;
  }
  std::string& target_args =
      *absl::IgnoreLeak(new std::string("--target_args=["));
  bool is_target_arg = true;
  bool is_done = false;
  auto new_end = std::remove_if(first_target_to_arg + 1, end, [&](char* arg) {
    if (is_done) {
      return false;
    }
    if (is_target_arg) {
      absl::StrAppend(&target_args, arg, ",");
      is_target_arg = false;
      return true;
    } else if (arg == kTargetToArg) {
      is_target_arg = true;
      return true;
    } else if (arg == absl::string_view("--")) {
      is_done = true;
      return false;
    } else {
      return false;
    }
  });
  target_args.pop_back();  // remove trailing comma.
  absl::StrAppend(&target_args, "]");
  *first_target_to_arg = target_args.data();
  argc = new_end - argv;
}

}  // namespace crubit

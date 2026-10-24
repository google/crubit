// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/cmdline.h"

#include <algorithm>
#include <iterator>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "common/logging.h"
#include "common/status.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "llvm/ADT/STLExtras.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Allocator.h"
#include "llvm/Support/CommandLine.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/JSON.h"
#include "llvm/Support/StringSaver.h"
#include "llvm/Support/raw_ostream.h"

namespace crubit {

namespace {

struct TargetArgs {
  std::string target;
  std::vector<std::string> headers;
  std::vector<std::string> features;
  std::optional<std::string> crate_name;
};

bool fromJSON(const llvm::json::Value& json, TargetArgs& out,
              llvm::json::Path path) {
  llvm::json::ObjectMapper mapper(json, path);
  return mapper && mapper.map("t", out.target) &&
         mapper.mapOptional("h", out.headers) &&
         mapper.mapOptional("f", out.features) &&
         mapper.mapOptional("c", out.crate_name);
}

llvm::cl::OptionCategory rs_bindings_from_cc_category(
    "rs_bindings_from_cc options");

// Parses bool flags with the same spellings Abseil flags accepted
// (case-insensitive `true`/`t`/`yes`/`y`/`1` and `false`/`f`/`no`/`n`/`0`), so
// that existing `rust_bindings_from_cc_cli_flag` users keep working.
struct BoolFlagParser : llvm::cl::parser<bool> {
  using llvm::cl::parser<bool>::parser;

  bool parse(llvm::cl::Option& option, llvm::StringRef arg_name,
             llvm::StringRef arg, bool& value) {
    // A bare `--flag` is passed as a null `arg`.
    if (arg.data() == nullptr) {
      value = true;
      return false;
    }
    std::string lower = arg.lower();
    if (lower == "true" || lower == "t" || lower == "yes" || lower == "y" ||
        lower == "1") {
      value = true;
      return false;
    }
    if (lower == "false" || lower == "f" || lower == "no" || lower == "n" ||
        lower == "0") {
      value = false;
      return false;
    }
    return option.error("'" + arg + "' is invalid value for boolean argument");
  }
};
using BoolFlag = llvm::cl::opt<bool, /*ExternalStorage=*/false, BoolFlagParser>;

// A flag holding a comma-separated list of strings. Each occurrence is stored
// unsplit; see `ListFlagValue`.
using StringListFlag = llvm::cl::list<std::string>;

// Returns the value of a `StringListFlag` with the semantics of Abseil's
// `std::vector<std::string>` flags: the last occurrence wins, an empty value is
// an empty list, and empty elements are otherwise preserved.
std::vector<std::string> ListFlagValue(const StringListFlag& flag) {
  std::vector<std::string> result;
  if (flag.empty()) return result;
  llvm::StringRef last_value = *std::prev(flag.end());
  if (last_value.empty()) return result;
  llvm::SmallVector<llvm::StringRef> elements;
  last_value.split(elements, ',');
  for (llvm::StringRef element : elements) result.push_back(element.str());
  return result;
}

BoolFlag do_nothing(
    "do_nothing", llvm::cl::init(false),
    llvm::cl::desc("if set to true the tool will produce empty files "
                   "(useful for testing Bazel integration)"),
    llvm::cl::cat(rs_bindings_from_cc_category));
llvm::cl::opt<std::string> driver_path(
    "driver_path", llvm::cl::init(""),
    llvm::cl::desc("path to the rs_bindings_from_cc_driver script"),
    llvm::cl::cat(rs_bindings_from_cc_category));
llvm::cl::opt<std::string> rs_out(
    "rs_out", llvm::cl::init(""),
    llvm::cl::desc("output path for the Rust source file with bindings"),
    llvm::cl::cat(rs_bindings_from_cc_category));
llvm::cl::opt<std::string> cc_out(
    "cc_out", llvm::cl::init(""),
    llvm::cl::desc(
        "output path for the C++ source file with bindings implementation"),
    llvm::cl::cat(rs_bindings_from_cc_category));
llvm::cl::opt<std::string> ir_out(
    "ir_out", llvm::cl::init(""),
    llvm::cl::desc("(optional) output path for the debug-formatted IR. If not "
                   "present, the "
                   "debug-formatted IR will not be dumped."),
    llvm::cl::cat(rs_bindings_from_cc_category));
llvm::cl::opt<std::string> crubit_support_path_format(
    "crubit_support_path_format", llvm::cl::init(""),
    llvm::cl::desc(
        "the format of `#include` for including Crubit C++ support library "
        "headers in the "
        "generated .cc files. in the format specifier, use `{header}` as the "
        "placeholder. For "
        "example, to include `support_header.h` as "
        "`#include \"support/support_header.h\", specify "
        "`\"support/{header}\";"),
    llvm::cl::cat(rs_bindings_from_cc_category));
llvm::cl::opt<std::string> crubit_support_versioned_path_format(
    "crubit_support_versioned_path_format", llvm::cl::init(""),
    llvm::cl::desc(
        "the format of `#include` for including Crubit's versioned `internal`"
        "support library headers in the generated .cc files. "
        "In the format specifier, use `{header}` as the "
        "placeholder. For "
        "example, to include `support_header.h` as "
        "`#include <crubit/support/internal/support_header.h>, specify "
        "`<crubit/support/internal/{header}>`;"),
    llvm::cl::cat(rs_bindings_from_cc_category));
llvm::cl::opt<std::string> clang_format_exe_path(
    "clang_format_exe_path", llvm::cl::init(""),
    llvm::cl::desc(
        "(optional) Path to a clang-format executable that will be used to "
        "format the "
        ".cc files generated by the tool."),
    llvm::cl::cat(rs_bindings_from_cc_category));
llvm::cl::opt<std::string> rustfmt_exe_path(
    "rustfmt_exe_path", llvm::cl::init(""),
    llvm::cl::desc("(optional) Path to a rustfmt executable that will be used "
                   "to format the "
                   ".rs files generated by the tool."),
    llvm::cl::cat(rs_bindings_from_cc_category));
llvm::cl::opt<std::string> rustfmt_config_path(
    "rustfmt_config_path", llvm::cl::init(""),
    llvm::cl::desc(
        "(optional) path to a rustfmt.toml file that should replace the "
        "default formatting of the .rs files generated by the tool."),
    llvm::cl::cat(rs_bindings_from_cc_category));
StringListFlag public_headers(
    "public_headers",
    llvm::cl::desc(
        "public headers of the cc_library this tool should generate bindings "
        "for, in a format suitable for usage in a quote "
        "include (#include \"\"). Note that these can be a subset of headers "
        "attributed to the target via target_args. For example, "
        "unparseable headers may be removed frp, public_headers, but kept "
        "attributed to that target in target_args."),
    llvm::cl::cat(rs_bindings_from_cc_category));
llvm::cl::opt<std::string> target(
    "target", llvm::cl::init(""),
    llvm::cl::desc("The target to generate bindings for."),
    llvm::cl::cat(rs_bindings_from_cc_category));
llvm::cl::opt<std::string> target_args(
    "target_args", llvm::cl::init(""),
    llvm::cl::desc(
        "Per-target Crubit arguments, encoded as a JSON array. This contains "
        "the list of headers assigned to the target (h), the set of "
        "enabled features (f), and the (optional) crate name (c). For example:"
        "[\n"
        "  {\n"
        "     \"t\": \"//foo/bar:baz\",\n"
        "     \"h\": [\"foo/bar/header1.h\", \"foo/bar/header2.h\"],\n"
        "     \"f\": [\"supported\"],\n"
        "     \"c\": \"some_not_target_name_derived_crate_name\"\n"
        "  },\n"
        "...\n"
        "]"),
    llvm::cl::cat(rs_bindings_from_cc_category));
StringListFlag extra_rs_srcs(
    "extra_rs_srcs",
    llvm::cl::desc("Additional Rust source files to include into the crate."),
    llvm::cl::cat(rs_bindings_from_cc_category));
StringListFlag extra_cpp_srcs(
    "extra_cpp_srcs",
    llvm::cl::desc(
        "Additional C++ source files whose contents are parsed by Clang to "
        "generate Rust bindings, and prepended to the generated C++ "
        "implementation (`rs_api_impl.cc`)."),
    llvm::cl::cat(rs_bindings_from_cc_category));
StringListFlag reexported_namespaces(
    "reexported_namespaces",
    llvm::cl::desc(
        "List of C++ namespaces to re-export at the root of the generated "
        "Rust bindings."),
    llvm::cl::cat(rs_bindings_from_cc_category));
StringListFlag unstable_rust_features(
    "unstable_rust_features",
    llvm::cl::desc(
        "Additional unstable rustc features to enable via `#![feature(...)]`."),
    llvm::cl::cat(rs_bindings_from_cc_category));
StringListFlag srcs_to_scan_for_instantiations(
    "srcs_to_scan_for_instantiations",
    llvm::cl::desc(
        "[template instantiation mode only] all Rust source files of a crate "
        "for which we are instantiating templates."),
    llvm::cl::cat(rs_bindings_from_cc_category));
llvm::cl::opt<std::string> instantiations_out(
    "instantiations_out", llvm::cl::init(""),
    llvm::cl::desc(
        "[template instantiation mode only] output path for the JSON file "
        "with mapping from a template instantiation to a generated Rust "
        "struct name. This file is used by cc_template! macro expansion."),
    llvm::cl::cat(rs_bindings_from_cc_category));
llvm::cl::opt<std::string> namespaces_out(
    "namespaces_out", llvm::cl::init(""),
    llvm::cl::desc(
        "(optional) output path for the JSON file containing the target's"
        "namespace hierarchy."),
    llvm::cl::cat(rs_bindings_from_cc_category));
llvm::cl::opt<std::string> error_report_out(
    "error_report_out", llvm::cl::init(""),
    llvm::cl::desc("(optional) output path for the JSON error report"),
    llvm::cl::cat(rs_bindings_from_cc_category));
BoolFlag is_golden_test(
    "is_golden_test", llvm::cl::init(false),
    llvm::cl::desc(
        "If true, unnecessary information (such as source locations) is "
        "omitted from the generated bindings to reduce noise in golden "
        "tests."),
    llvm::cl::cat(rs_bindings_from_cc_category));

llvm::cl::list<std::string> target_to_arg(
    "target_to_arg",
    llvm::cl::desc("Per-target Crubit arguments for one target; may be "
                   "repeated. See --target_args."),
    llvm::cl::cat(rs_bindings_from_cc_category));

StringListFlag do_not_bind_allowlist(
    "do_not_bind_allowlist",
    llvm::cl::desc(
        "List of decls that are allowed to be omitted from bindings."
        " If omitted, `CRUBIT_DO_NOT_BIND` can be used on any decl."),
    llvm::cl::cat(rs_bindings_from_cc_category));

llvm::cl::opt<std::string> template_blocklist_path_regex(
    "template_blocklist_path_regex", llvm::cl::init(""),
    llvm::cl::desc(
        "If nonempty, do not instantiate templates defined in files with "
        "paths matching this llvm::Regex."),
    llvm::cl::cat(rs_bindings_from_cc_category));

llvm::cl::opt<std::string> carcinize(
    "carcinize", llvm::cl::init("off"),
    llvm::cl::desc("Mode for the carcinize pipeline for automated porting. "
                   "Options: off, strict, incomplete."),
    llvm::cl::cat(rs_bindings_from_cc_category));

BoolFlag kythe_annotations(
    "kythe_annotations", llvm::cl::init(false),
    llvm::cl::desc(
        "Emit extra source information for generating cross-references."),
    llvm::cl::cat(rs_bindings_from_cc_category));
llvm::cl::opt<std::string> kythe_default_corpus(
    "kythe_default_corpus", llvm::cl::init("corpus"),
    llvm::cl::desc("Default corpus to use for Kythe vnames."),
    llvm::cl::cat(rs_bindings_from_cc_category));

BoolFlag lazy_import_alien_decls(
    "lazy_import_alien_decls", llvm::cl::init(false),
    llvm::cl::desc(
        "If true, declarations from targets other than the current target "
        "are only imported when they are (transitively) referenced from "
        "the current target, instead of importing all of them eagerly."),
    llvm::cl::cat(rs_bindings_from_cc_category));

BoolFlag check_importer_invariants(
    "check_importer_invariants", llvm::cl::init(false),
    llvm::cl::desc(
        "If true, run (potentially expensive) consistency checks on the "
        "importer's internal state and crash if they fail. Intended for "
        "debugging."),
    llvm::cl::cat(rs_bindings_from_cc_category));

llvm::cl::list<std::string> clang_args(
    llvm::cl::Positional, llvm::cl::desc("<clang args>"),
    llvm::cl::cat(rs_bindings_from_cc_category));

// Consumed by `InitGoogle` before the switch to `llvm::cl`; Bazel passes it to
// control logging verbosity (see `generate_bindings.bzl`).
llvm::cl::opt<int> stderrthreshold(
    "stderrthreshold", llvm::cl::init(2), llvm::cl::Hidden,
    llvm::cl::desc("Log messages at or above this severity (0=INFO, 1=WARNING, "
                   "2=ERROR, 3=FATAL) are written to stderr."),
    llvm::cl::cat(rs_bindings_from_cc_category));

bool ParseCarcinizeMode(llvm::StringRef text, CarcinizeMode* mode,
                        std::string* error) {
  if (text == "off" || text == "false" || text == "0") {
    *mode = CarcinizeMode::kOff;
    return true;
  }
  if (text == "strict") {
    *mode = CarcinizeMode::kStrict;
    return true;
  }
  if (text == "incomplete" || text == "true" || text == "1") {
    *mode = CarcinizeMode::kIncomplete;
    return true;
  }
  *error = "CarcinizeMode must be one of: off, strict, incomplete";
  return false;
}

}  // namespace

namespace internal {

llvm::Error ParseTargetArgs(llvm::StringRef target_args_str,
                            CmdlineArgs& args) {
  if (target_args_str.empty()) {
    return InvalidArgumentError("please specify --target_args");
  }
  auto target_args =
      llvm::json::parse<std::vector<TargetArgs>>(target_args_str);
  if (!target_args) {
    return InvalidArgumentError("Malformed `--target_args` argument: " +
                                llvm::toString(target_args.takeError()));
  }
  for (const TargetArgs& it : *target_args) {
    const std::string& tgt = it.target;
    if (tgt.empty()) {
      return InvalidArgumentError(
          "Expected `t` fields of `--target_args` to be a non-empty "
          "string");
    }
    for (const std::string& header : it.headers) {
      if (header.empty()) {
        return InvalidArgumentError(
            "Expected `h` (header) fields of `--target_args` to be an "
            "array of non-empty strings");
      }
      BazelLabel target_label(tgt);
      auto [map_it, inserted] = args.headers_to_targets.try_emplace(
          HeaderName(header), std::move(target_label));
      if (!inserted) {
        CRUBIT_LOG(Warning)
            << "The `--target_args` cmdline argument assigns `" << header
            << "` header to two conflicting targets: `" << tgt << "` vs `"
            << map_it->second.value() << "`";
        if (target_label.value() < map_it->second.value()) {
          map_it->second = std::move(target_label);
        }
      }
    }
    for (const std::string& feature : it.features) {
      if (feature.empty()) {
        return InvalidArgumentError(
            "Expected `f` (feature) fields of `--target_args` to be an "
            "array of non-empty strings");
      }
      args.target_to_features[BazelLabel(tgt)].insert(feature);
    }
    if (it.crate_name.has_value()) {
      args.target_to_crate_name[BazelLabel(tgt)] = *it.crate_name;
    }
  }
  return llvm::Error::success();
}

}  // namespace internal

llvm::Expected<Cmdline> Cmdline::FromArgv(int argc, const char* const* argv) {
  // Hides the options registered by LLVM/Clang libraries from `--help`.
  [[maybe_unused]] static const bool unrelated_options_hidden = [] {
    llvm::cl::HideUnrelatedOptions(rs_bindings_from_cc_category);
    return true;
  }();
  // Allows tests to parse multiple command lines in one process.
  llvm::cl::ResetAllOptionOccurrences();

  // Bazel may pass the arguments in a gcc-style `@paramfile`.
  llvm::BumpPtrAllocator allocator;
  llvm::StringSaver saver(allocator);
  llvm::SmallVector<const char*> expanded_argv(argv, argv + argc);
  llvm::cl::ExpandResponseFiles(saver, llvm::cl::TokenizeGNUCommandLine,
                                expanded_argv);

  // Errors about unknown options are reported through `parse_error`; errors
  // about invalid option values are printed to stderr by the option parsers.
  std::string parse_error;
  llvm::raw_string_ostream parse_error_stream(parse_error);
  if (!llvm::cl::ParseCommandLineOptions(
          expanded_argv.size(), expanded_argv.data(),
          "Generates Rust bindings for C++ headers.", &parse_error_stream)) {
    if (parse_error.empty()) {
      parse_error = "Failed to parse command line arguments.";
    }
    return InvalidArgumentError(parse_error);
  }

  SetMinLogSeverity(static_cast<LogSeverity>(
      std::clamp<int>(stderrthreshold, static_cast<int>(LogSeverity::kInfo),
                      static_cast<int>(LogSeverity::kFatal))));

  CmdlineArgs args = {
      .current_target = BazelLabel(target),
      .driver_path = driver_path,
      .cc_out = cc_out,
      .rs_out = rs_out,
      .ir_out = ir_out,
      .namespaces_out = namespaces_out,
      .crubit_support_path_format = crubit_support_path_format,
      .crubit_support_versioned_path_format =
          crubit_support_versioned_path_format,
      .clang_format_exe_path = clang_format_exe_path,
      .rustfmt_exe_path = rustfmt_exe_path,
      .rustfmt_config_path = rustfmt_config_path,
      .error_report_out = error_report_out,
      .do_nothing = do_nothing,
      .is_golden_test = is_golden_test,
      .kythe_annotations = kythe_annotations,
      .kythe_default_corpus = kythe_default_corpus,
      .lazy_import_alien_decls = lazy_import_alien_decls,
      .check_importer_invariants = check_importer_invariants,
      .extra_rs_srcs = ListFlagValue(extra_rs_srcs),
      .extra_cpp_srcs = ListFlagValue(extra_cpp_srcs),
      .reexported_namespaces = ListFlagValue(reexported_namespaces),
      .unstable_rust_features = ListFlagValue(unstable_rust_features),
      .srcs_to_scan_for_instantiations =
          ListFlagValue(srcs_to_scan_for_instantiations),
      .instantiations_out = instantiations_out,
      .template_blocklist_path_regex = template_blocklist_path_regex,
      .clang_args = clang_args,
  };
  for (const std::string& header : ListFlagValue(public_headers)) {
    args.public_headers.push_back(HeaderName(header));
  }
  if (do_not_bind_allowlist.getNumOccurrences() > 0) {
    args.do_not_bind_allowlist = ListFlagValue(do_not_bind_allowlist);
  }
  std::string carcinize_error;
  if (!ParseCarcinizeMode(carcinize, &args.carcinize_mode, &carcinize_error)) {
    return InvalidArgumentError(carcinize_error);
  }

  // `--target_to_arg` may be repeated; each value is one element of the JSON
  // array otherwise passed as `--target_args`.
  std::string target_args_json = target_args;
  if (!target_to_arg.empty()) {
    target_args_json = "[" + llvm::join(target_to_arg, ",") + "]";
  }
  llvm::Error parse_target_args_error =
      internal::ParseTargetArgs(target_args_json, args);

  llvm::Expected<Cmdline> cmdline = Cmdline::Create(std::move(args));
  if (parse_target_args_error || !cmdline) {
    std::string create_error =
        cmdline ? "" : llvm::toString(cmdline.takeError());
    return InvalidArgumentError(
        create_error + (create_error.empty() ? "" : "\n") +
        llvm::toString(std::move(parse_target_args_error)));
  }
  return std::move(*cmdline);
}

llvm::Expected<Cmdline> Cmdline::Create(CmdlineArgs args) {
  std::string error;
  if (args.current_target.empty()) {
    error += "please specify --target\n";
  }
  if (args.rs_out.empty()) {
    error += "please specify --rs_out\n";
  }
  if (args.cc_out.empty()) {
    error += "please specify --cc_out\n";
  }
  if (args.public_headers.empty()) {
    error += "please specify --public_headers\n";
  }

  if (args.crubit_support_path_format.empty()) {
    error += "please specify --crubit_support_path_format\n";
  } else if (args.crubit_support_path_format.find("{header}") ==
             std::string::npos) {
    error +=
        "cannot find `{header}` placeholder in crubit_support_path_format\n";
  }
  if (args.srcs_to_scan_for_instantiations.empty() !=
      args.instantiations_out.empty()) {
    error +=
        "please specify both --rust_sources and --instantiations_out when "
        "requesting a template instantiation mode\n";
  }
  for (const HeaderName& header : args.public_headers) {
    if (args.headers_to_targets.find(header) == args.headers_to_targets.end()) {
      error += "Couldn't find header '" + header.IncludePath().str() +
               "' in the `headers_to_target` map "
               "derived from the --target_args cmdline argument\n";
    }
  }
  if (!error.empty()) {
    error.erase(error.size() - 1);  // remove trailing \n
    return InvalidArgumentError(error);
  }
  return Cmdline(std::move(args));
}

}  // namespace crubit

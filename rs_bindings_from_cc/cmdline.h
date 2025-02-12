// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_CMDLINE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_CMDLINE_H_

#include <string>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "common/ffi_types.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"

namespace crubit {

// The command line arguments to Crubit.
struct CmdlineArgs {
  BazelLabel current_target;
  std::string driver_path;
  std::string cc_out;
  std::string rs_out;
  std::string ir_out;
  std::string namespaces_out;
  std::string crubit_support_path_format;
  std::string clang_format_exe_path;
  std::string rustfmt_exe_path;
  std::string rustfmt_config_path;
  std::string error_report_out;
  bool do_nothing = true;
  SourceLocationDocComment generate_source_location_in_doc_comment =
      SourceLocationDocComment::Enabled;

  std::vector<HeaderName> public_headers;
  absl::flat_hash_map<HeaderName, BazelLabel> headers_to_targets;

  std::vector<std::string> extra_rs_srcs;

  std::vector<std::string> srcs_to_scan_for_instantiations;
  std::string instantiations_out;

  absl::flat_hash_map<BazelLabel, absl::flat_hash_set<std::string>>
      target_to_features;
};

// A valid command line invocation.
class Cmdline {
 public:
  // Creates a validated Cmdline based on the given `args`.
  //
  // Returns an error if `args` is invalid.
  static absl::StatusOr<Cmdline> Create(CmdlineArgs args);
  // Creates `Cmdline` based on the actual flags.
  //
  // Returns an error if the flags are invalid.
  static absl::StatusOr<Cmdline> FromFlags();
  const CmdlineArgs& args() const& { return args_; }
  CmdlineArgs args() && { return std::move(args_); }

 private:
  explicit Cmdline(CmdlineArgs args) : args_(std::move(args)) {}

  CmdlineArgs args_;
};

namespace internal {
// Parses --target_args into CmdlineArgs. Only exposed so it can be unit tested.
absl::Status ParseTargetArgs(absl::string_view target_args_str,
                             CmdlineArgs& args);
}  // namespace internal

// Expands paramfiles (@path/to/file) in-place in argv.
//
// This must be called before flag parsing.
//
// A paramfile is a newline-delimited list of arguments, with some characters
// escaped by a backslash. See:
// https://github.com/bazelbuild/bazel/blob/818c5c8693c43fe490c9f6b2c05149eb8f45cf52/src/main/java/com/google/devtools/build/lib/util/GccParamFileEscaper.java#L24-L30
//
// Everywhere a `@param/file` is encountered in argv, it is replaced by the
// list of arguments within that file.
//
// Paramfiles cannot include other paramfiles. (Can they?)
void ExpandParamfiles(int& argc, char**& argv);

// Moves `--target_to_arg` arguments into `--target_args`.
//
// This must be called before flag parsing.
//
// Abseil does not allow for repeated flags, so we need to concatenate the
// --target_to_args values before moving them to the --target_args flag.
void PreprocessTargetArgs(int& argc, char** argv);

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_CMDLINE_H_

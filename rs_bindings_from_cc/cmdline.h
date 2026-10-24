// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_CMDLINE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_CMDLINE_H_

#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/StringSet.h"
#include "llvm/Support/Error.h"

namespace crubit {

enum class CarcinizeMode {
  kOff,     // Standard Crubit build (Carcinize is 100% OFF)
  kStrict,  // Carcinize Mode: fails if 100% native Crubit coverage is missing
  kIncomplete,  // Carcinize Mode: emits global_cpp! fallbacks for unsupported
                // items
};

// The command line arguments to Crubit.
struct CmdlineArgs {
  BazelLabel current_target;
  std::string driver_path;
  std::string cc_out;
  std::string rs_out;
  std::string ir_out;
  std::string namespaces_out;
  std::string crubit_support_path_format;
  std::string crubit_support_versioned_path_format;
  std::string clang_format_exe_path;
  std::string rustfmt_exe_path;
  std::string rustfmt_config_path;
  std::string error_report_out;
  bool do_nothing = true;
  bool is_golden_test = false;
  bool kythe_annotations = false;
  std::string kythe_default_corpus;
  bool lazy_import_alien_decls = false;
  bool check_importer_invariants = false;

  std::vector<HeaderName> public_headers;
  llvm::DenseMap<HeaderName, BazelLabel> headers_to_targets;

  std::vector<std::string> extra_rs_srcs;
  std::vector<std::string> extra_cpp_srcs;
  std::vector<std::string> reexported_namespaces;
  std::vector<std::string> unstable_rust_features;

  std::vector<std::string> srcs_to_scan_for_instantiations;
  std::string instantiations_out;

  llvm::DenseMap<BazelLabel, llvm::StringSet<>> target_to_features;

  llvm::DenseMap<BazelLabel, std::string> target_to_crate_name;

  std::optional<std::vector<std::string>> do_not_bind_allowlist;

  std::string template_blocklist_path_regex;
  CarcinizeMode carcinize_mode = CarcinizeMode::kOff;

  std::vector<std::string> clang_args;
};

// A valid command line invocation.
class Cmdline {
 public:
  // Creates a validated Cmdline based on the given `args`.
  //
  // Returns an error if `args` is invalid.
  static llvm::Expected<Cmdline> Create(CmdlineArgs args);
  // Creates `Cmdline` based on the actual flags.
  //
  // Returns an error if the flags are invalid.
  static llvm::Expected<Cmdline> FromArgv(int argc, const char* const* argv);

  const CmdlineArgs& args() const& { return args_; }
  CmdlineArgs args() && { return std::move(args_); }

 private:
  explicit Cmdline(CmdlineArgs args) : args_(std::move(args)) {}

  CmdlineArgs args_;
};

namespace internal {
// Parses --target_args into CmdlineArgs. Only exposed so it can be unit tested.
llvm::Error ParseTargetArgs(llvm::StringRef target_args_str, CmdlineArgs& args);

}  // namespace internal

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_CMDLINE_H_

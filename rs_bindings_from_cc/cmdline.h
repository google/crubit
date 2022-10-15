// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_CMDLINE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_CMDLINE_H_

#include <string>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"

namespace crubit {

// Parses and validates command line arguments.
class Cmdline {
 public:
  // Creates `Cmdline` based on the actual cmdline arguments.
  static absl::StatusOr<Cmdline> Create();

  // Creates `Cmdline` based on the provided cmdline arguments - `cc_out`,
  // `rs_out`, and so forth.
  static absl::StatusOr<Cmdline> CreateForTesting(
      std::string cc_out, std::string rs_out, std::string ir_out,
      std::string namespaces_out, std::string crubit_support_path,
      std::string rustfmt_exe_path, std::string rustfmt_config_path,
      bool do_nothing, std::vector<std::string> public_headers,
      std::string targets_and_headers_str,
      std::vector<std::string> extra_rs_sources,
      std::vector<std::string> srcs_to_scan_for_instantiations,
      std::string instantiations_out, std::string error_report_out) {
    return CreateFromArgs(
        std::move(cc_out), std::move(rs_out), std::move(ir_out),
        std::move(namespaces_out), std::move(crubit_support_path),
        std::move(rustfmt_exe_path), std::move(rustfmt_config_path), do_nothing,
        std::move(public_headers), std::move(targets_and_headers_str),
        std::move(extra_rs_sources), std::move(srcs_to_scan_for_instantiations),
        std::move(instantiations_out), std::move(error_report_out));
  }

  Cmdline(const Cmdline&) = delete;
  Cmdline& operator=(const Cmdline&) = delete;
  Cmdline(Cmdline&&) = default;
  Cmdline& operator=(Cmdline&&) = default;

  absl::string_view cc_out() const { return cc_out_; }
  absl::string_view rs_out() const { return rs_out_; }
  absl::string_view ir_out() const { return ir_out_; }
  absl::string_view namespaces_out() const { return namespaces_out_; }
  absl::string_view crubit_support_path() const { return crubit_support_path_; }
  absl::string_view rustfmt_exe_path() const { return rustfmt_exe_path_; }
  absl::string_view rustfmt_config_path() const { return rustfmt_config_path_; }
  absl::string_view instantiations_out() const { return instantiations_out_; }
  absl::string_view error_report_out() const { return error_report_out_; }
  bool do_nothing() const { return do_nothing_; }

  const std::vector<HeaderName>& public_headers() const {
    return public_headers_;
  }

  const std::vector<std::string>& extra_rs_srcs() const {
    return extra_rs_srcs_;
  }

  const std::vector<std::string>& srcs_to_scan_for_instantiations() const {
    return srcs_to_scan_for_instantiations_;
  }

  const BazelLabel& current_target() const { return current_target_; }

  const absl::flat_hash_map<const HeaderName, const BazelLabel>&
  headers_to_targets() const {
    return headers_to_targets_;
  }

 private:
  Cmdline();

  static absl::StatusOr<Cmdline> CreateFromArgs(
      std::string cc_out, std::string rs_out, std::string ir_out,
      std::string namespaces_out, std::string crubit_support_path,
      std::string rustfmt_exe_path, std::string rustfmt_config_path,
      bool do_nothing, std::vector<std::string> public_headers,
      std::string targets_and_headers_str,
      std::vector<std::string> extra_rs_sources,
      std::vector<std::string> srcs_to_scan_for_instantiations,
      std::string instantiations_out, std::string error_report_out);

  absl::StatusOr<BazelLabel> FindHeader(const HeaderName& header) const;

  std::string cc_out_;
  std::string rs_out_;
  std::string ir_out_;
  std::string crubit_support_path_;
  std::string rustfmt_exe_path_;
  std::string rustfmt_config_path_;
  std::string error_report_out_;
  bool do_nothing_ = true;

  BazelLabel current_target_;
  std::vector<HeaderName> public_headers_;
  absl::flat_hash_map<const HeaderName, const BazelLabel> headers_to_targets_;

  std::vector<std::string> extra_rs_srcs_;

  std::vector<std::string> srcs_to_scan_for_instantiations_;
  std::string instantiations_out_;

  std::string namespaces_out_;
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_CMDLINE_H_

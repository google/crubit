// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_CMDLINE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_CMDLINE_H_

#include <string>
#include <vector>

#include "third_party/absl/container/flat_hash_map.h"
#include "third_party/absl/status/statusor.h"
#include "third_party/absl/strings/string_view.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"

namespace rs_bindings_from_cc {

// Parses and validates command line arguments.
class Cmdline {
 public:
  // Creates `Cmdline` based on the actual cmdline arguments.
  static absl::StatusOr<Cmdline> Create();

  // Creates `Cmdline` based on the provided cmdline arguments - `cc_out`,
  // `rs_out`, and so forth.
  static absl::StatusOr<Cmdline> CreateForTesting(
      std::string cc_out, std::string rs_out, std::string ir_out,
      bool do_nothing, std::vector<std::string> public_headers,
      std::string targets_and_headers_str) {
    return CreateFromArgs(cc_out, rs_out, ir_out, do_nothing, public_headers,
                          targets_and_headers_str);
  }

  Cmdline(const Cmdline&) = delete;
  Cmdline& operator=(const Cmdline&) = delete;
  Cmdline(Cmdline&&) = default;
  Cmdline& operator=(Cmdline&&) = default;

  absl::string_view cc_out() const { return cc_out_; }
  absl::string_view rs_out() const { return rs_out_; }
  absl::string_view ir_out() const { return ir_out_; }
  bool do_nothing() const { return do_nothing_; }

  const std::vector<HeaderName>& public_headers() const {
    return public_headers_;
  }

  const BlazeLabel& current_target() const { return current_target_; }

  const absl::flat_hash_map<const HeaderName, const BlazeLabel>&
  headers_to_targets() const {
    return headers_to_targets_;
  }

 private:
  Cmdline();

  static absl::StatusOr<Cmdline> CreateFromArgs(
      std::string cc_out, std::string rs_out, std::string ir_out,
      bool do_nothing, std::vector<std::string> public_headers,
      std::string targets_and_headers_str);

  absl::StatusOr<BlazeLabel> FindHeader(const HeaderName& header) const;

  std::string cc_out_;
  std::string rs_out_;
  std::string ir_out_;
  bool do_nothing_ = true;

  BlazeLabel current_target_;
  std::vector<HeaderName> public_headers_;
  absl::flat_hash_map<const HeaderName, const BlazeLabel> headers_to_targets_;
};

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_CMDLINE_H_

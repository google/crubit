// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_CMDLINE_FLAGS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_CMDLINE_FLAGS_H_
#include <string>
#include <vector>

#include "absl/flags/declare.h"

ABSL_DECLARE_FLAG(bool, do_nothing);
ABSL_DECLARE_FLAG(std::string, rs_out);
ABSL_DECLARE_FLAG(std::string, cc_out);
ABSL_DECLARE_FLAG(std::string, ir_out);
ABSL_DECLARE_FLAG(std::string, crubit_support_path_format);
ABSL_DECLARE_FLAG(std::string, clang_format_exe_path);
ABSL_DECLARE_FLAG(std::string, rustfmt_exe_path);
ABSL_DECLARE_FLAG(std::string, rustfmt_config_path);
ABSL_DECLARE_FLAG(std::vector<std::string>, public_headers);
ABSL_DECLARE_FLAG(std::string, target);
ABSL_DECLARE_FLAG(std::string, target_args);
ABSL_DECLARE_FLAG(std::vector<std::string>, extra_rs_srcs);
ABSL_DECLARE_FLAG(std::vector<std::string>, srcs_to_scan_for_instantiations);
ABSL_DECLARE_FLAG(std::string, instantiations_out);
ABSL_DECLARE_FLAG(std::string, namespaces_out);
ABSL_DECLARE_FLAG(std::string, error_report_out);
ABSL_DECLARE_FLAG(std::string, environment);
ABSL_DECLARE_FLAG(bool, generate_source_location_in_doc_comment);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_CMDLINE_FLAGS_H_

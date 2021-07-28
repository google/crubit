// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Parses C++ headers and generates:
// * a Rust source file with bindings for the C++ API
// * a C++ source file with the implementation of the bindings

#include <memory>
#include <string>

#include "base/callback.h"
#include "base/init_google.h"
#include "base/logging.h"
#include "devtools/cymbal/common/clang_google3_tool.h"
#include "rs_bindings_from_cc/ast_consumer_factory.h"
#include "file/base/helpers.h"
#include "file/base/options.h"
#include "third_party/absl/flags/flag.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Tooling/Tooling.h"
#include "util/task/status.h"

ABSL_FLAG(std::string, rs_out, "",
          "output path for the Rust source file with bindings");
ABSL_FLAG(std::string, cc_out, "",
          "output path for the C++ source file with bindings implementation");

int main(int argc, char* argv[]) {
  InitGoogle(argv[0], &argc, &argv, true);

  auto rs_out = absl::GetFlag(FLAGS_rs_out);
  QCHECK(!rs_out.empty()) << "please specify --rs_out";
  auto cc_out = absl::GetFlag(FLAGS_cc_out);
  QCHECK(!cc_out.empty()) << "please specify --cc_out";

  std::string rs_api;
  std::string rs_api_impl;
  rs_bindings_from_cc::AstConsumerFactory ast_consumer_factory(rs_api,
                                                               rs_api_impl);
  std::unique_ptr<clang::tooling::FrontendActionFactory> action_factory =
      clang::tooling::newFrontendActionFactory(&ast_consumer_factory);

  devtools::cymbal::StandaloneClangTool tool(argc, argv, "rs_bindings_from_cc");
  int result = tool.Run(NewPermanentCallback(
      action_factory.get(), &clang::tooling::FrontendActionFactory::create));

  if (result == 0) {
    CHECK_OK(file::SetContents(rs_out, rs_api, file::Defaults()));
    CHECK_OK(file::SetContents(cc_out, rs_api_impl, file::Defaults()));
  }

  return result;
}

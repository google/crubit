// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <memory>
#include <string>
#include <vector>

#include "rs_bindings_from_cc/ast_consumer_factory.h"
#include "testing/base/public/gunit.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Tooling/Tooling.h"

namespace rs_bindings_from_cc {
namespace {

struct Bindings {
  std::string rs_api;
  std::string rs_api_impl;
};

Bindings GenerateBindings(const absl::string_view code,
                          const std::vector<absl::string_view> &args) {
  Bindings outputs;
  AstConsumerFactory ast_consumer_factory(outputs.rs_api, outputs.rs_api_impl);
  std::unique_ptr<clang::tooling::FrontendActionFactory> action_factory =
      clang::tooling::newFrontendActionFactory(&ast_consumer_factory);
  std::vector<std::string> args_as_strings(args.begin(), args.end());
  clang::tooling::runToolOnCodeWithArgs(action_factory->create(), code,
                                        args_as_strings);
  return outputs;
}

TEST(AstVisitorTest, TestHelloWorld) {
  Bindings bindings = GenerateBindings("int hello();", {});
  EXPECT_EQ(bindings.rs_api, "// rs api");
  EXPECT_EQ(bindings.rs_api_impl, "// rs api impl");
}

}  // namespace
}  // namespace rs_bindings_from_cc

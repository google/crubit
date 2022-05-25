// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_TEST_LIFETIME_ANALYSIS_TEST_H_
#define DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_TEST_LIFETIME_ANALYSIS_TEST_H_

#include <string>

#include "gtest/gtest.h"
#include "absl/container/flat_hash_map.h"
#include "lifetime_analysis/analyze.h"
#include "lifetime_annotations/test/named_func_lifetimes.h"

namespace clang {
namespace tidy {
namespace lifetimes {

class LifetimeAnalysisTest : public testing::Test {
 protected:
  void TearDown() override;

  static std::string QualifiedName(const clang::FunctionDecl* func);

  struct GetLifetimesOptions {
    GetLifetimesOptions()
        : with_template_placeholder(false), include_implicit_methods(false) {}
    bool with_template_placeholder;
    bool include_implicit_methods;
  };

  NamedFuncLifetimes GetLifetimes(
      llvm::StringRef source_code,
      const GetLifetimesOptions& options = GetLifetimesOptions());

  NamedFuncLifetimes GetLifetimesWithPlaceholder(llvm::StringRef source_code);

  void AnalyzeBrokenCode() { analyze_broken_code_ = true; }

 private:
  absl::flat_hash_map<std::string, FunctionDebugInfo> debug_info_map_;
  bool analyze_broken_code_ = false;
};

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_TEST_LIFETIME_ANALYSIS_TEST_H_

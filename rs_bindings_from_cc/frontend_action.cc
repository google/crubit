// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/frontend_action.h"

#include <memory>
#include <string>
#include <utility>

#include "absl/container/flat_hash_set.h"
#include "common/string_view_conversion.h"
#include "lifetime_annotations/lifetime_annotations.h"
#include "nullability/pragma.h"
#include "rs_bindings_from_cc/ast_consumer.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Frontend/DependencyOutputOptions.h"
#include "clang/Frontend/FrontendAction.h"
#include "clang/Frontend/Utils.h"
#include "llvm/ADT/StringRef.h"

namespace crubit {

namespace {

// Like `clang::DependencyFileGenerator` (which implements `-MD`, `-MF`, etc.),
// but omits `excluded_files` from the depfile.
class DependencyFileGeneratorWithExclusions final
    : public clang::DependencyFileGenerator {
 public:
  DependencyFileGeneratorWithExclusions(
      const clang::DependencyOutputOptions& opts,
      absl::flat_hash_set<std::string> excluded_files)
      : clang::DependencyFileGenerator(opts),
        excluded_files_(std::move(excluded_files)) {}

  void maybeAddDependency(llvm::StringRef filename, bool from_module,
                          bool is_system, bool is_module_file,
                          bool is_direct_module_import,
                          bool is_missing) override {
    if (excluded_files_.contains(StringViewFromStringRef(filename))) {
      return;
    }
    clang::DependencyFileGenerator::maybeAddDependency(
        filename, from_module, is_system, is_module_file,
        is_direct_module_import, is_missing);
  }

 private:
  absl::flat_hash_set<std::string> excluded_files_;
};

}  // namespace

bool FrontendAction::BeginInvocation(clang::CompilerInstance& instance) {
  // Use `DependencyFileGeneratorWithExclusions` instead of the default
  // `clang::DependencyFileGenerator`.
  clang::DependencyOutputOptions& opts = instance.getDependencyOutputOpts();
  if (!opts.OutputFile.empty()) {
    instance.addDependencyCollector(
        std::make_shared<DependencyFileGeneratorWithExclusions>(
            opts, in_memory_files_));
    // Stop `CompilerInstance::createPreprocessor` from also adding the default
    // generator (which would write the same depfile without the exclusions).
    // Modifying `opts` is okay: they belong to the `CompilerInvocation` of this
    // action, and `BeginInvocation` is meant for such modifications.
    opts.OutputFile.clear();
  }
  return clang::ASTFrontendAction::BeginInvocation(instance);
}

std::unique_ptr<clang::ASTConsumer> FrontendAction::CreateASTConsumer(
    clang::CompilerInstance& instance, llvm::StringRef) {
  AddLifetimeAnnotationHandlers(instance.getPreprocessor(),
                                invocation_.lifetime_context_);
  clang::tidy::nullability::registerPragmaHandler(
      instance.getPreprocessor(), invocation_.nullability_pragmas_);
  return std::make_unique<AstConsumer>(instance, invocation_);
}

}  // namespace crubit

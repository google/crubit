// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/frontend_action.h"

#include <memory>
#include <string>
#include <utility>

#include "absl/cleanup/cleanup.h"
#include "common/check.h"
#include "common/string_view_conversion.h"
#include "lifetime_annotations/lifetime_annotations.h"
#include "nullability/pragma.h"
#include "rs_bindings_from_cc/ast_consumer.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclGroup.h"
#include "clang/AST/ExternalASTSource.h"
#include "clang/Basic/Stack.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Frontend/DependencyOutputOptions.h"
#include "clang/Frontend/FrontendAction.h"
#include "clang/Frontend/Utils.h"
#include "clang/Lex/Preprocessor.h"
#include "clang/Parse/Parser.h"
#include "clang/Sema/EnterExpressionEvaluationContext.h"
#include "clang/Sema/Sema.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/StringSet.h"

namespace crubit {

namespace {

// Like `clang::DependencyFileGenerator` (which implements `-MD`, `-MF`, etc.),
// but omits `excluded_files` from the depfile.
class DependencyFileGeneratorWithExclusions final
    : public clang::DependencyFileGenerator {
 public:
  DependencyFileGeneratorWithExclusions(
      const clang::DependencyOutputOptions& opts,
      llvm::StringSet<> excluded_files)
      : clang::DependencyFileGenerator(opts),
        excluded_files_(std::move(excluded_files)) {}

  void maybeAddDependency(llvm::StringRef filename, bool from_module,
                          bool is_system, bool is_module_file,
                          bool is_direct_module_import,
                          bool is_missing) override {
    if (excluded_files_.contains(filename)) {
      return;
    }
    clang::DependencyFileGenerator::maybeAddDependency(
        filename, from_module, is_system, is_module_file,
        is_direct_module_import, is_missing);
  }

 private:
  llvm::StringSet<> excluded_files_;
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
  return std::make_unique<AstConsumer>(instance, invocation_, parser_);
}

void FrontendAction::ExecuteAction() {
  // Like `clang::ASTFrontendAction::ExecuteAction` followed by
  // `clang::ParseAST` (without code completion or statistics), except for
  // incremental processing and the parser being passed to the `AstConsumer`.
  clang::CompilerInstance& instance = getCompilerInstance();
  if (!instance.hasPreprocessor()) return;
  clang::noteBottomOfStack();
  if (!instance.hasSema()) {
    instance.createSema(getTranslationUnitKind(),
                        /*CompletionConsumer=*/nullptr);
  }
  clang::Sema& sema = instance.getSema();
  clang::Preprocessor& preprocessor = sema.getPreprocessor();
  clang::ASTConsumer& consumer = sema.getASTConsumer();

  // Incremental processing (as in clang-repl, which parses each input after
  // the end of the previous one) keeps the translation unit usable after it
  // ends: `Sema` keeps the translation unit's scope, and the preprocessor
  // keeps the lexer of the main file, which keeps returning `eof`. This lets
  // the `AstConsumer` parse macro constants after the end of the translation
  // unit. For C++, nothing else changes (see
  // `Preprocessor::isIncrementalProcessingEnabled`).
  preprocessor.enableIncrementalProcessing();

  // Like `clang::ParseAST`, destroys the parser before returning, while
  // `Sema` is still alive.
  CRUBIT_CHECK(parser_ == nullptr);
  const bool skip_function_bodies =
      instance.getFrontendOpts().SkipFunctionBodies;
  parser_ =
      std::make_unique<clang::Parser>(preprocessor, sema, skip_function_bodies);
  absl::Cleanup reset_parser = [this] { parser_.reset(); };
  clang::Parser& parser = *parser_;
  preprocessor.EnterMainSourceFile();
  if (clang::ExternalASTSource* external =
          sema.getASTContext().getExternalSource()) {
    external->StartTranslationUnit(&consumer);
  }

  // If a PCH through header is specified that does not have an include in the
  // source, or a PCH is being created with `#pragma hdrstop` with nothing
  // after the pragma, there won't be any tokens or a lexer.
  if (preprocessor.getCurrentLexer() != nullptr) {
    parser.Initialize();
    clang::Parser::DeclGroupPtrTy decl_group;
    clang::Sema::ModuleImportState import_state;
    clang::EnterExpressionEvaluationContext potentially_evaluated(
        sema, clang::Sema::ExpressionEvaluationContext::PotentiallyEvaluated);
    for (bool at_eof = parser.ParseFirstTopLevelDecl(decl_group, import_state);
         !at_eof; at_eof = parser.ParseTopLevelDecl(decl_group, import_state)) {
      // If we got a null return and something *was* parsed, ignore it. This
      // is due to a top-level semicolon, an action override, or a parse error
      // skipping something.
      if (decl_group && !consumer.HandleTopLevelDecl(decl_group.get())) {
        return;
      }
    }
  }

  // Process any top-level decls generated by `#pragma weak`.
  for (clang::Decl* decl : sema.WeakTopLevelDecls()) {
    consumer.HandleTopLevelDecl(clang::DeclGroupRef(decl));
  }

  consumer.HandleTranslationUnit(sema.getASTContext());
}

}  // namespace crubit

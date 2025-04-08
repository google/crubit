// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/template_placeholder_support.h"

#include <functional>
#include <iterator>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/str_cat.h"
#include "absl/strings/str_join.h"
#include "clang/include/clang/AST/Decl.h"
#include "clang/include/clang/AST/DeclTemplate.h"
#include "clang/include/clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/include/clang/ASTMatchers/ASTMatchers.h"
#include "clang/include/clang/Analysis/CFG.h"
#include "clang/include/clang/Basic/SourceLocation.h"
#include "clang/include/clang/Lex/Lexer.h"
#include "clang/include/clang/Tooling/Refactoring/AtomicChange.h"
#include "clang/include/clang/Tooling/Tooling.h"
#include "clang/include/clang/Tooling/Transformer/RangeSelector.h"
#include "clang/include/clang/Tooling/Transformer/RewriteRule.h"
#include "clang/include/clang/Tooling/Transformer/Stencil.h"
#include "clang/include/clang/Tooling/Transformer/Transformer.h"
#include "llvm/include/llvm/ADT/ArrayRef.h"
#include "llvm/include/llvm/ADT/DenseMap.h"
#include "llvm/include/llvm/ADT/DenseSet.h"
#include "llvm/include/llvm/ADT/IntrusiveRefCntPtr.h"
#include "llvm/include/llvm/ADT/SmallVector.h"
#include "llvm/include/llvm/Support/Error.h"
#include "llvm/include/llvm/Support/VirtualFileSystem.h"
#include "llvm/include/llvm/Support/raw_ostream.h"

namespace clang {
namespace tidy {
namespace lifetimes {

namespace {

using clang::ast_matchers::MatchFinder;

class TranslationUnitMatcherCallback : public MatchFinder::MatchCallback {
 public:
  explicit TranslationUnitMatcherCallback(
      std::function<void(clang::ASTContext&)> operation)
      : operation_{operation} {}

  void run(const MatchFinder::MatchResult& Result) override {
    const auto* tu = Result.Nodes.getNodeAs<clang::TranslationUnitDecl>("tu");
    if (!tu) return;
    operation_(tu->getASTContext());
  }

  std::function<void(clang::ASTContext&)> operation_;
};

}  // namespace

llvm::Expected<GeneratedCode> GenerateTemplateInstantiationCode(
    const clang::TranslationUnitDecl* tu,
    const llvm::DenseMap<clang::FunctionTemplateDecl*,
                         const clang::FunctionDecl*>& templates) {
  using clang::ast_matchers::asString;
  using clang::ast_matchers::decl;
  using clang::ast_matchers::equalsNode;
  using clang::ast_matchers::functionDecl;
  using clang::ast_matchers::functionTemplateDecl;
  using clang::ast_matchers::hasBody;
  using clang::ast_matchers::hasParent;
  using clang::ast_matchers::loc;
  using clang::ast_matchers::qualType;
  using clang::ast_matchers::stmt;
  using clang::ast_matchers::typeLoc;
  using clang::tooling::Transformer;
  using clang::transformer::cat;
  using clang::transformer::charRange;
  using clang::transformer::edit;
  using clang::transformer::EditGenerator;
  using clang::transformer::name;
  using clang::transformer::node;
  using clang::transformer::remove;

  auto& context = tu->getASTContext();
  auto file_id = tu->getASTContext().getSourceManager().getMainFileID();
  auto& source_manager = context.getSourceManager();
  auto source_filename =
      source_manager.getFilename(source_manager.getLocForStartOfFile(file_id));

  auto source_code = clang::Lexer::getSourceText(
      clang::CharSourceRange::getTokenRange(
          source_manager.getLocForStartOfFile(file_id),
          source_manager.getLocForEndOfFile(file_id)),
      source_manager, context.getLangOpts());

  llvm::Error err = llvm::Error::success();
  clang::tooling::AtomicChanges changes;
  std::vector<std::unique_ptr<Transformer>> transformers;

  auto consumer =
      [&changes,
       &err](llvm::Expected<llvm::MutableArrayRef<clang::tooling::AtomicChange>>
                 c) {
        if (c) {
          changes.insert(changes.end(), std::make_move_iterator(c->begin()),
                         std::make_move_iterator(c->end()));
        } else {
          err = c.takeError();
          llvm::errs() << llvm::toString(c.takeError()) << "\n";
        }
      };

  clang::TranslationUnitDecl* translation_unit =
      context.getTranslationUnitDecl();
  llvm::DenseSet<const clang::Decl*> toplevels(translation_unit->decls_begin(),
                                               translation_unit->decls_end());

  int placeholder_suffix_idx = 0;
  std::vector<std::string> placeholder_classes;
  for (const auto& [tmpl, func] : templates) {
    toplevels.erase(tmpl);
    auto* params = tmpl->getTemplateParameters();
    std::vector<std::string> parameters;
    llvm::SmallVector<EditGenerator, 2> edits;
    std::string func_name = func->getNameAsString();

    for (auto param : *params) {
      // TODO(kinuko): check the template parameter types, this only assumes
      // type parameters for now.
      std::string placeholder_class = absl::StrCat(
          func_name, "_type_placeholder_", placeholder_suffix_idx++);

      placeholder_classes.push_back(placeholder_class);
      parameters.push_back(placeholder_class);

      auto change_type_rule =
          makeRule(typeLoc(loc(qualType(asString(param->getNameAsString())))),
                   changeTo(cat(placeholder_class)));
      edits.push_back(rewriteDescendants(func_name, change_type_rule));
    }

    edits.push_back(edit(changeTo(node("body"), cat(";"))));
    edits.push_back(edit(
        changeTo(name(func_name),
                 cat(absl::StrCat(func->getNameAsString(), "<",
                                  absl::StrJoin(parameters, ", "), ">")))));
    edits.push_back(edit(remove(charRange(clang::CharSourceRange::getCharRange(
        params->getLAngleLoc(), params->getRAngleLoc().getLocWithOffset(1))))));

    auto rule =
        makeRule(functionDecl(equalsNode(func), hasBody(stmt().bind("body")),
                              hasParent(functionTemplateDecl()))
                     .bind(func_name),
                 flattenVector(edits));
    transformers.push_back(std::make_unique<Transformer>(rule, consumer));
  }

  for (const auto* node_to_delete : toplevels) {
    // Delete all other top-level nodes (we only need the instantiation code as
    // original code is to be included separately)
    auto rule = makeRule(decl(equalsNode(node_to_delete)), changeTo(cat("")));
    transformers.push_back(std::make_unique<Transformer>(rule, consumer));
  }

  std::string instantiation_code;
  MatchFinder match_finder;
  for (const auto& transformer : transformers) {
    transformer->registerMatchers(&match_finder);
  }
  match_finder.matchAST(context);

  // `consumer` might have produced an error.
  if (err) return std::move(err);

  if ((err = clang::tooling::applyAtomicChanges(
                 source_filename, source_code, changes,
                 clang::tooling::ApplyChangesSpec())
                 .moveInto(instantiation_code))) {
    return std::move(err);
  }

  // insertBefore or other transform edits don't work quite well, so simply
  // concat and add the string.
  std::vector<std::string> placeholder_definitions;
  for (auto& c : placeholder_classes) {
    placeholder_definitions.push_back("struct ");
    placeholder_definitions.push_back(c);
    placeholder_definitions.push_back(" {};\n");
  }

  GeneratedCode generated;
  generated.filename = (source_filename + "-with-placeholders.cc").str();
  generated.code = absl::StrCat("#include \"", source_filename.str(), "\"\n",
                                absl::StrJoin(placeholder_definitions, ""),
                                instantiation_code);
  return generated;
}

void RunToolOnCodeWithOverlay(
    clang::ASTContext& original_context, const std::string& filename,
    const std::string& code,
    const std::function<void(clang::ASTContext&)> operation) {
  using clang::ast_matchers::MatchFinder;
  using clang::ast_matchers::translationUnitDecl;

  // Set up an overlay filesystem and add the `code` as a virtual file of it.
  llvm::IntrusiveRefCntPtr<llvm::vfs::FileSystem> fs(
      &original_context.getSourceManager()
           .getFileManager()
           .getVirtualFileSystem());
  auto overlay = llvm::makeIntrusiveRefCnt<llvm::vfs::OverlayFileSystem>(fs);
  auto memory_fs = llvm::makeIntrusiveRefCnt<llvm::vfs::InMemoryFileSystem>();
  overlay->pushOverlay(memory_fs);
  memory_fs->addFile(filename, 0, llvm::MemoryBuffer::getMemBuffer(code));

  clang::ast_matchers::MatchFinder match_finder;
  TranslationUnitMatcherCallback callback(operation);

  match_finder.addMatcher(translationUnitDecl().bind("tu"), &callback);
  std::unique_ptr<clang::tooling::FrontendActionFactory> factory(
      (clang::tooling::newFrontendActionFactory(&match_finder)));

  // TODO(kinuko): get the args from the current ASTContext.
  clang::tooling::runToolOnCodeWithArgs(factory->create(), code, overlay,
                                        {"-fsyntax-only", "-std=c++17"},
                                        filename, "lifetime-with-placedholder");
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

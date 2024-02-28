// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// infer_tu_main infers nullability within a single translation unit.
//
// By default (-diagnostics=1) it shows findings as diagnostics.
// It can optionally (-protos=1) print the Inference proto.
//
// This is not the intended way to fully analyze a real codebase.
// e.g. it can't jointly inspect all callsites of a function (in different TUs).

#include <memory>
#include <string>
#include <utility>

#include "absl/base/nullability.h"
#include "absl/log/check.h"
#include "absl/strings/str_cat.h"
#include "nullability/inference/ctn_replacement_macros.h"
#include "nullability/inference/infer_tu.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/replace_macros.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclarationName.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Frontend/FrontendAction.h"
#include "clang/Frontend/FrontendActions.h"
#include "clang/Index/USRGeneration.h"
#include "clang/Tooling/ArgumentsAdjusters.h"
#include "clang/Tooling/Execution.h"
#include "clang/Tooling/Tooling.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/STLExtras.h"
#include "llvm/ADT/SmallString.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/Twine.h"
#include "llvm/Support/Casting.h"
#include "llvm/Support/CommandLine.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/Regex.h"
#include "llvm/Support/raw_ostream.h"

using ::clang::tidy::nullability::ReplacementMacrosHeaderFileName;

llvm::cl::OptionCategory Opts("infer_tu_main options");
llvm::cl::opt<bool> PrintProtos{
    "protos",
    llvm::cl::desc("Print the Inference protos"),
    llvm::cl::init(false),
};
llvm::cl::opt<bool> Diagnostics{
    "diagnostics",
    llvm::cl::desc("Print inference results as diagnostics"),
    llvm::cl::init(true),
};
llvm::cl::opt<bool> PrintEvidence{
    "evidence",
    llvm::cl::desc("Print sample evidence as notes (requires -diagnostics)"),
    llvm::cl::init(true),
};
llvm::cl::opt<bool> IncludeTrivial{
    "trivial",
    llvm::cl::desc("Include trivial inferences (annotated, no conflicts)"),
    llvm::cl::init(false),
};
llvm::cl::opt<std::string> FileFilter{
    "file-filter",
    llvm::cl::desc("Regular expression filenames must match to be analyzed. "
                   "May be negated with - prefix."),
};
llvm::cl::opt<std::string> NameFilter{
    "name-filter",
    llvm::cl::desc("Regular expression decl names must match to be analyzed. "
                   "May be negated with - prefix."),
};
llvm::cl::opt<unsigned> Iterations{
    "iterations",
    llvm::cl::desc("Number of inference iterations"),
    llvm::cl::init(1),
};

namespace clang::tidy::nullability {
namespace {

// Walks the AST looking for declarations of symbols we inferred.
// When it finds them, prints the inference as diagnostics.
class DiagnosticPrinter : public RecursiveASTVisitor<DiagnosticPrinter> {
  llvm::DenseMap<llvm::StringRef, absl::Nonnull<const Inference *>>
      InferenceByUSR;
  DiagnosticsEngine &Diags;
  unsigned DiagInferHere;
  unsigned DiagSample;

  void render(const Inference &I, const Decl &D) {
    for (const auto &Slot : I.slot_inference()) {
      Diags.Report(D.getLocation(), DiagInferHere)
          << slotName(Slot.slot(), D)
          << Inference::Nullability_Name(Slot.nullability());
      if (PrintEvidence) {
        for (const auto &Sample : Slot.sample_evidence()) {
          if (SourceLocation Loc = parseLoc(Sample.location()); Loc.isValid())
            Diags.Report(Loc, DiagSample) << Evidence::Kind_Name(Sample.kind());
        }
      }
    }
  }

  std::string slotName(unsigned S, const Decl &D) {
    if (S == SLOT_RETURN_TYPE) return "return type";
    unsigned ParamIdx = S - SLOT_PARAM;
    llvm::StringRef ParamName;
    if (const auto *FD = dyn_cast<FunctionDecl>(&D)) {
      const ParmVarDecl *Param = FD->getParamDecl(ParamIdx);
      if (Param->getDeclName().isIdentifier()) ParamName = Param->getName();
    }
    llvm::Twine Name = "parameter " + llvm::Twine(ParamIdx);
    if (ParamName.empty()) return Name.str();
    return (Name + " ('" + ParamName + "')").str();
  }

  // Terrible hack: parse "foo.cc:4:2" back into a SourceLocation.
  SourceLocation parseLoc(llvm::StringRef LocStr) {
    auto &SM = Diags.getSourceManager();
    auto &FM = SM.getFileManager();
    auto [Rest, ColStr] = llvm::StringRef(LocStr).rsplit(':');
    auto [Name, LineStr] = Rest.rsplit(':');
    auto File = FM.getOptionalFileRef(Name);
    unsigned Line, Col;
    if (!File || LineStr.getAsInteger(10, Line) || ColStr.getAsInteger(10, Col))
      return SourceLocation();
    return SM.translateFileLineCol(&File->getFileEntry(), Line, Col);
  }

 public:
  DiagnosticPrinter(llvm::ArrayRef<Inference> All, DiagnosticsEngine &Diags)
      : Diags(Diags) {
    for (const auto &I : All) InferenceByUSR.try_emplace(I.symbol().usr(), &I);
    DiagInferHere = Diags.getCustomDiagID(DiagnosticsEngine::Remark,
                                          "would mark %0 as %1 here");
    DiagSample = Diags.getCustomDiagID(DiagnosticsEngine::Note, "%0 here");
  }

  bool VisitDecl(absl::Nonnull<const Decl *> FD) {
    llvm::SmallString<128> USR;
    if (!index::generateUSRForDecl(FD, USR))
      if (auto *I = InferenceByUSR.lookup(USR)) render(*I, *FD);
    return true;
  }
};

// Selects which declarations to analyze based on filter flags.
struct DeclFilter {
  bool operator()(const Decl &D) const {
    auto &SM = D.getDeclContext()->getParentASTContext().getSourceManager();
    if (!checkLocation(D.getLocation(), SM)) return false;
    if (auto *ND = llvm::dyn_cast<NamedDecl>(&D))
      if (!checkName(*ND)) return false;
    return true;
  }

  bool checkLocation(SourceLocation Loc, const SourceManager &SM) const {
    if (!FileFilter.getNumOccurrences()) return true;
    auto ID = SM.getFileID(SM.getFileLoc(Loc));
    auto [It, Inserted] = FileCache.try_emplace(ID);
    if (Inserted) {
      static auto &Pattern = *new RegexFlagFilter(FileFilter);
      auto FID = SM.getFileEntryRefForID(ID);
      It->second = !FID.has_value() || Pattern(FID->getName());
    }
    return It->second;
  }

  bool checkName(const NamedDecl &ND) const {
    if (!NameFilter.getNumOccurrences()) return true;
    static auto &Pattern = *new RegexFlagFilter(NameFilter);
    return Pattern(ND.getQualifiedNameAsString());
  }

  mutable llvm::DenseMap<FileID, bool> FileCache;
  struct RegexFlagFilter {
    RegexFlagFilter(llvm::StringRef Regex)
        : Negative(Regex.consume_front("-")), Pattern(Regex) {
      std::string Err;
      CHECK(Pattern.isValid(Err)) << Regex.str() << ": " << Err;
    }

    bool operator()(llvm::StringRef Text) {
      bool Match = Pattern.match(Text);
      return Negative ? !Match : Match;
    }

    bool Negative;
    llvm::Regex Pattern;
  };
};

class Action : public SyntaxOnlyAction {
  absl::Nonnull<std::unique_ptr<ASTConsumer>> CreateASTConsumer(
      CompilerInstance &, llvm::StringRef) override {
    class Consumer : public ASTConsumer {
      void HandleTranslationUnit(ASTContext &Ctx) override {
        llvm::errs() << "Running inference...\n";

        auto Results = inferTU(Ctx, Iterations, DeclFilter());
        if (!IncludeTrivial)
          llvm::erase_if(Results, [](Inference &I) {
            llvm::erase_if(
                *I.mutable_slot_inference(),
                [](const Inference::SlotInference &S) { return S.trivial(); });
            return I.slot_inference_size() == 0;
          });
        if (PrintProtos)
          for (const auto &I : Results) llvm::outs() << absl::StrCat(I) << "\n";
        if (Diagnostics)
          DiagnosticPrinter(Results, Ctx.getDiagnostics()).TraverseAST(Ctx);
      }
    };
    return std::make_unique<Consumer>();
  }

  bool BeginSourceFileAction(clang::CompilerInstance &CI) override {
    if (!ASTFrontendAction::BeginSourceFileAction(CI)) return false;
    if (!!CI.getLangOpts().CPlusPlus) return true;

    CI.getPreprocessor().addPPCallbacks(
        std::make_unique<ReplaceMacrosCallbacks>(CI.getPreprocessor()));
    return true;
  }
};

}  // namespace
}  // namespace clang::tidy::nullability

int main(int argc, absl::Nonnull<const char **> argv) {
  using namespace clang::tooling;
  auto Exec = createExecutorFromCommandLineArgs(argc, argv, Opts);
  QCHECK(Exec) << toString(Exec.takeError());

  CHECK_EQ(ctn_replacement_macros_size(), 1);
  llvm::StringRef MacroReplacementText =
      ctn_replacement_macros_create()[0].data;
  (*Exec)->mapVirtualFile(ReplacementMacrosHeaderFileName,
                          MacroReplacementText);

  auto Err = (*Exec)->execute(
      newFrontendActionFactory<clang::tidy::nullability::Action>(),

      getInsertArgumentAdjuster(
          {// Disable warnings, test cases are full of unused expressions etc.
           "-w",
           // Include the file containing macro replacements that enable
           // additional inference.
           "-include", std::string(ReplacementMacrosHeaderFileName)},
          ArgumentInsertPosition::BEGIN));
  QCHECK(!Err) << toString(std::move(Err));
}

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
#include "absl/container/flat_hash_map.h"
#include "absl/log/check.h"
#include "absl/strings/str_cat.h"
#include "nullability/inference/ctn_replacement_macros.h"
#include "nullability/inference/infer_tu.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/replace_macros.h"
#include "nullability/pragma.h"
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
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/SmallString.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/Twine.h"
#include "llvm/Support/Casting.h"
#include "llvm/Support/CommandLine.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/Format.h"
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
llvm::cl::opt<bool> PrintMetrics{
    "metrics",
    llvm::cl::desc("Print inference metrics"),
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
  InferenceResults InferencesByUSR;
  DiagnosticsEngine &Diags;
  unsigned DiagInferHere;
  unsigned DiagSample;

  void render(const absl::flat_hash_map<Slot, SlotInference> &InferencesBySlot,
              const Decl &D) {
    for (const auto &[Slot, SlotInference] : InferencesBySlot) {
      if (!IncludeTrivial && SlotInference.trivial()) continue;
      Diags.Report(D.getLocation(), DiagInferHere)
          << slotName(Slot, D) << Nullability_Name(SlotInference.nullability());
      if (PrintEvidence) {
        for (const auto &Sample : SlotInference.sample_evidence()) {
          if (SourceLocation Loc = parseLoc(Sample.location()); Loc.isValid())
            Diags.Report(Loc, DiagSample) << Evidence::Kind_Name(Sample.kind());
        }
      }
    }
  }

  std::string slotName(unsigned S, const Decl &D) {
    if (const auto *Field = dyn_cast<FieldDecl>(&D))
      return Field->getName().str();
    if (const auto *Var = dyn_cast<VarDecl>(&D)) return Var->getName().str();
    if (S == SLOT_RETURN_TYPE) return "return type";
    unsigned ParamIdx = S - SLOT_PARAM;
    llvm::StringRef ParamName;
    if (const auto *Func = dyn_cast<FunctionDecl>(&D)) {
      const ParmVarDecl *Param = Func->getParamDecl(ParamIdx);
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
  DiagnosticPrinter(InferenceResults All, DiagnosticsEngine &Diags)
      : InferencesByUSR(std::move(All)), Diags(Diags) {
    DiagInferHere = Diags.getCustomDiagID(DiagnosticsEngine::Remark,
                                          "would mark %0 as %1 here");
    DiagSample = Diags.getCustomDiagID(DiagnosticsEngine::Note, "%0 here");
  }

  bool VisitDecl(absl::Nonnull<const Decl *> FD) {
    llvm::SmallString<128> USR;
    if (!index::generateUSRForDecl(FD, USR)) {
      if (auto It = InferencesByUSR.find(USR.str());
          It != InferencesByUSR.end()) {
        render(It->second, *FD);
      }
    }
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
  NullabilityPragmas Pragmas;

  absl::Nonnull<std::unique_ptr<ASTConsumer>> CreateASTConsumer(
      CompilerInstance &, llvm::StringRef) override {
    class Consumer : public ASTConsumer {
     public:
      NullabilityPragmas &Pragmas;
      Consumer(NullabilityPragmas &Pragmas) : Pragmas(Pragmas) {}

     private:
      void HandleTranslationUnit(ASTContext &Ctx) override {
        if (Ctx.getDiagnostics().hasErrorOccurred()) {
          llvm::errs() << "An error has occurred; not running inference.\n";
          return;
        }
        llvm::errs() << "Running inference...\n";

        InferenceResults Results =
            inferTU(Ctx, Pragmas, Iterations, DeclFilter());
        if (PrintProtos) {
          for (const auto &[USR, InferencesBySlot] : Results) {
            llvm::outs() << "USR: " << absl::StrCat(USR) << "\n";
            for (const auto &[Slot, SlotInference] : InferencesBySlot) {
              llvm::outs() << "Slot: " << Slot << "\n";
              llvm::outs() << "Inference:\n"
                           << absl::StrCat(SlotInference) << "\n";
            }
          }
        }
        if (PrintMetrics) {
          unsigned Nonnull = 0;
          unsigned Nullable = 0;
          unsigned Unknown = 0;
          unsigned Conflict = 0;
          for (const auto &[_, InferencesBySlot] : Results) {
            for (const auto &[Slot, SlotInference] : InferencesBySlot) {
              if (SlotInference.conflict()) {
                ++Conflict;
                continue;
              }
              switch (SlotInference.nullability()) {
                case Nullability::NULLABLE:
                  ++Nullable;
                  break;
                case Nullability::NONNULL:
                  ++Nonnull;
                  break;
                case Nullability::UNKNOWN:
                  ++Unknown;
                  break;
              }
            }
          }
          llvm::outs() << "Inferred " << Nonnull + Nullable + Unknown + Conflict
                       << " symbols\n";
          llvm::outs() << "Nonnull: " << Nonnull << "\n";
          llvm::outs() << "Nullable: " << Nullable << "\n";
          llvm::outs() << "Unknown: " << Unknown << "\n";
          llvm::outs() << "Conflicts: " << Conflict << "\n";
          llvm::outs() << "Percent not Unknown and not Conflict: "
                       << llvm::format("%0.2f", 100.0 * (Nonnull + Nullable) /
                                                    (Nonnull + Nullable +
                                                     Unknown + Conflict))
                       << "%\n";
        }
        if (Diagnostics)
          DiagnosticPrinter(std::move(Results), Ctx.getDiagnostics())
              .TraverseAST(Ctx);
      }
    };
    return std::make_unique<Consumer>(Pragmas);
  }

  bool BeginSourceFileAction(clang::CompilerInstance &CI) override {
    if (!ASTFrontendAction::BeginSourceFileAction(CI) ||
        !CI.getLangOpts().CPlusPlus)
      return false;

    registerPragmaHandler(CI.getPreprocessor(), Pragmas);
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
           "-include", std::string(ReplacementMacrosHeaderFileName),
           // TODO: b/357760487 -- use the flag until the issue is resolved or
           // we find a workaround.
           "-Xclang", "-fretain-subst-template-type-parm-type-ast-nodes"},
          ArgumentInsertPosition::BEGIN));
  QCHECK(!Err) << toString(std::move(Err));
}

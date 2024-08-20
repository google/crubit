// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/augmented_test_inputs.h"

#include <memory>
#include <utility>

#include "nullability/inference/ctn_replacement_macros.h"
#include "nullability/inference/replace_macros.h"
#include "nullability/pragma.h"
#include "nullability/test/test_headers.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Frontend/FrontendAction.h"
#include "clang/Testing/CommandLineArgs.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/StringRef.h"

namespace clang::tidy::nullability {
TestInputs getAugmentedTestInputs(llvm::StringRef Source,
                                  NullabilityPragmas& Pragmas) {
  TestInputs Inputs = Source;
  Inputs.Language = TestLanguage::Lang_CXX20;
  for (const auto& Entry :
       llvm::ArrayRef(test_headers_create(), test_headers_size()))
    Inputs.ExtraFiles.try_emplace(Entry.name, Entry.data);
  for (const auto& Entry : llvm::ArrayRef(ctn_replacement_macros_create(),
                                          ctn_replacement_macros_size()))
    Inputs.ExtraFiles.try_emplace(Entry.name, Entry.data);
  // TODO: b/357760487 -- use the flag until the issue is resolved or we find a
  // workaround.
  Inputs.ExtraArgs.push_back(
      "-fretain-subst-template-type-parm-type-ast-nodes");
  Inputs.ExtraArgs.push_back("-I.");
  Inputs.ExtraArgs.push_back("-include");
  Inputs.ExtraArgs.push_back("nullability_annotations.h");
  Inputs.ExtraArgs.push_back("-include");
  Inputs.ExtraArgs.emplace_back(ReplacementMacrosHeaderFileName);

  Inputs.MakeAction = [&]() {
    struct RegisterPragmasWrapperAction : public WrapperFrontendAction {
      NullabilityPragmas& Pragmas;
      RegisterPragmasWrapperAction(std::unique_ptr<FrontendAction> Wrapped,
                                   NullabilityPragmas& Pragmas)
          : WrapperFrontendAction(std::move(Wrapped)), Pragmas(Pragmas) {}

      bool BeginSourceFileAction(clang::CompilerInstance& CI) override {
        if (!WrapperFrontendAction::BeginSourceFileAction(CI)) return false;
        registerPragmaHandler(CI.getPreprocessor(), Pragmas);
        return true;
      }
    };

    return std::make_unique<RegisterPragmasWrapperAction>(
        std::make_unique<ReplaceMacrosAction>(), Pragmas);
  };
  return Inputs;
}
}  // namespace clang::tidy::nullability

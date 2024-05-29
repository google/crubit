// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/augmented_test_inputs.h"

#include <memory>

#include "nullability/inference/ctn_replacement_macros.h"
#include "nullability/inference/replace_macros.h"
#include "nullability/test/test_headers.h"
#include "clang/Testing/CommandLineArgs.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/StringRef.h"

namespace clang::tidy::nullability {
TestInputs getAugmentedTestInputs(llvm::StringRef Source) {
  TestInputs Inputs = Source;
  Inputs.Language = TestLanguage::Lang_CXX20;
  for (const auto& Entry :
       llvm::ArrayRef(test_headers_create(), test_headers_size()))
    Inputs.ExtraFiles.try_emplace(Entry.name, Entry.data);
  for (const auto& Entry : llvm::ArrayRef(ctn_replacement_macros_create(),
                                          ctn_replacement_macros_size()))
    Inputs.ExtraFiles.try_emplace(Entry.name, Entry.data);
  Inputs.ExtraArgs.push_back("-I.");
  Inputs.ExtraArgs.push_back("-include");
  Inputs.ExtraArgs.push_back("nullability_annotations.h");
  Inputs.ExtraArgs.push_back("-include");
  Inputs.ExtraArgs.emplace_back(ReplacementMacrosHeaderFileName);

  Inputs.MakeAction = [&]() { return std::make_unique<ReplaceMacrosAction>(); };
  return Inputs;
}
}  // namespace clang::tidy::nullability

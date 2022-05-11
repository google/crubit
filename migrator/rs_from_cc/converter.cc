// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "migrator/rs_from_cc/converter.h"

#include <string>

#include "absl/strings/str_split.h"
#include "clang/AST/CXXInheritance.h"
#include "clang/AST/Decl.h"
#include "clang/AST/RecordLayout.h"
#include "clang/Basic/FileManager.h"
#include "third_party/re2/re2.h"

namespace crubit_rs_from_cc {

void Converter::Convert(
    const clang::TranslationUnitDecl* translation_unit_decl) {
  ConvertUnsupported(translation_unit_decl);
}

void Converter::ConvertUnsupported(const clang::Decl* decl) {
  std::string ast;
  llvm::raw_string_ostream os(ast);
  decl->dump(os);
  os.flush();
  result_ += "\n";
  result_ += "// Unsupported decl:\n//\n";
  // Remove addresses since they're not useful and add non-determinism that
  // would break golden testing.
  // Also remove spaces at the end of each line, those are a pain in golden
  // tests since IDEs often strip spaces at end of line.
  RE2::GlobalReplace(&ast, "(?m) 0x[a-z0-9]+| +$", "");
  for (auto line : absl::StrSplit(ast, '\n')) {
    if (line.empty()) {
      continue;
    }
    result_ += "// ";
    result_ += line;
    result_ += '\n';
  }
}

}  // namespace crubit_rs_from_cc

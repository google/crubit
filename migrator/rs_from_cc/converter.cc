// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "migrator/rs_from_cc/converter.h"

#include "absl/strings/str_split.h"
#include "clang/AST/CXXInheritance.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclFriend.h"
#include "clang/AST/RecordLayout.h"
#include "clang/Basic/FileManager.h"
#include "third_party/re2/re2.h"

namespace crubit_rs_from_cc {

void Converter::Convert(const clang::TranslationUnitDecl* translation_unit) {
  for (clang::Decl* decl : translation_unit->decls()) {
    if (decl->getBeginLoc().isInvalid()) {
      // Skip declarations with invalid locations, e.g. builtins that Clang
      // generates.
      continue;
    }
    Convert(decl);
  }
}

void Converter::Convert(const clang::Decl* decl) {
  switch (decl->getKind()) {
    case clang::Decl::TranslationUnit:
      Convert(dynamic_cast<const clang::TranslationUnitDecl*>(decl));
      break;

    default:
      ConvertUnhandled(decl);
  }
}

void Converter::ConvertUnhandled(const clang::Decl* decl) {
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

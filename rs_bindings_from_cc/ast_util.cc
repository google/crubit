// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ast_util.h"

#include <optional>
#include <string>

#include "absl/functional/function_ref.h"
#include "absl/strings/str_cat.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/Basic/LLVM.h"

namespace crubit {

bool IsFullClassTemplateSpecializationOrChild(const clang::Decl* decl) {
  if (clang::isa<clang::ClassTemplatePartialSpecializationDecl>(decl)) {
    return false;
  }
  if (clang::isa<clang::ClassTemplateSpecializationDecl>(decl)) {
    return true;
  }

  if (const auto* decl_context = decl->getDeclContext()) {
    return IsFullClassTemplateSpecializationOrChild(
        clang::dyn_cast<clang::Decl>(decl_context));
  }

  return false;
}

std::optional<std::string> CollectUnknownAttrs(
    const clang::Decl& decl,
    absl::FunctionRef<bool(const clang::Attr&)> is_known) {
  std::optional<std::string> unknown_attr;
  if (!decl.hasAttrs()) {
    // Surprisingly, getAttrs() does not return an empty vec if there are no
    // attrs, it crashes.
    return unknown_attr;
  }
  for (clang::Attr* attr : decl.getAttrs()) {
    if (is_known(*attr)) {
      continue;
    }
    if (unknown_attr.has_value()) {
      absl::StrAppend(&*unknown_attr, ", ");
    } else {
      unknown_attr.emplace("");
    }
    absl::StrAppend(&*unknown_attr, attr->getAttrName()
                                        ? attr->getNormalizedFullName()
                                        : attr->getSpelling());
  }
  return unknown_attr;
}

}  // namespace crubit

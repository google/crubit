// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ast_visitor.h"

#include <memory>
#include <string>
#include <vector>

#include "base/logging.h"
#include "rs_bindings_from_cc/ir.h"
#include "third_party/absl/container/flat_hash_set.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Decl.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Mangle.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Type.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/Casting.h"

namespace rs_bindings_from_cc {

bool AstVisitor::TraverseDecl(clang::Decl* decl) {
  if (seen_decls_.insert(decl->getCanonicalDecl()).second) {
    return Base::TraverseDecl(decl);
  }
  return false;
}

bool AstVisitor::TraverseTranslationUnitDecl(
    clang::TranslationUnitDecl* translation_unit_decl) {
  mangler_.reset(translation_unit_decl->getASTContext().createMangleContext());

  // TODO(hlopko): Make the generated C++ code include-what-you-use clean.
  // Currently we pass public headers of the library to the src_code_gen.
  // Through those Clang has access to all declarations needed by the public API
  // of the library. However the code violates IWYU - it will not directly
  // include all the headers declaring names used in the generated source. This
  // could be fixed by passing not only public headers of the library to the
  // tool, but also all public headers of the direct dependencies of the
  // library. This way if the library was IWYU clean, the generated code will be
  // too.
  for (const absl::string_view header_name : public_header_names_) {
    ir_.UsedHeaders().emplace_back(HeaderName(std::string(header_name)));
  }

  return Base::TraverseTranslationUnitDecl(translation_unit_decl);
}

bool AstVisitor::VisitFunctionDecl(clang::FunctionDecl* function_decl) {
  // TODO(hlopko): Skip decls from other headers
  // TODO(hlopko): Handle lowercased snakecased conflicts
  // TODO(hlopko): Convert primitive types (bool -> bool, int -> i64 (?) and
  // so on)
  // TODO(hlopko): Import return type properly
  // TODO(hlopko): Import parameter types properly
  // TODO(hlopko): Import clang doc comment
  // TODO(hlopko): Handle member functions
  // TODO(hlopko): Handle static member functions
  // TODO(hlopko): Handle constructors/operators/special members
  // TODO(hlopko): Handle destructors
  // TODO(hlopko): Do not import deleted members
  // TODO(hlopko): Handle function templates
  // TODO(hlopko): Do not import private/protected members
  // TODO(hlopko): Handle (?) variadic functions
  // TODO(hlopko): Fail when exceptions enabled?
  std::vector<FuncParam> params;
  for (const clang::ParmVarDecl* param : function_decl->parameters()) {
    params.push_back({ConvertType(param->getType()), GetTranslatedName(param)});
  }

  ir_.Functions().push_back(Func{
      .identifier = GetTranslatedName(function_decl),
      .mangled_name = GetMangledName(function_decl),
      .return_type = ConvertType(function_decl->getReturnType()),
      .params = std::move(params),
      .is_inline = function_decl->isInlined(),
  });
  return true;
}

bool AstVisitor::VisitRecordDecl(clang::RecordDecl* record_decl) {
  // TODO(hlopko): Check access control for members
  // TODO(hlopko): Import nested types
  // TODO(hlopko): Import methods
  // TODO(hlopko): Import constructors
  // TODO(hlopko): Import destructor
  // TODO(hlopko): Import operators
  // TODO(hlopko): Handle class template specializations
  // TODO(hlopko): Handle partial class template specializations
  // TODO(hlopko): Handle unions
  // TODO(hlopko): Handle non-trivially movable types
  // TODO(hlopko): Make trivially copyable types copyable in Rust too
  // TODO(hlopko): Handle dependent types
  // TODO(hlopko): Handle opaque types (for example types with a field we cannot
  // yet import)
  // TODO(hlopko): Handle named bitfields
  // TODO(hlopko): Handle unnamed bitfields (used only for padding)
  // TODO(hlopko): Collect and cross check field offsets in C++ and in Rust

  std::vector<Field> fields;
  for (const clang::FieldDecl* field_decl : record_decl->fields()) {
    fields.emplace_back(GetTranslatedName(field_decl),
                        ConvertType(field_decl->getType()));
  }
  ir_.Records().emplace_back(GetTranslatedName(record_decl), std::move(fields));
  return true;
}

Type AstVisitor::ConvertType(clang::QualType qual_type) const {
  // TODO(hlopko): Handle all builtin types
  // TODO(hlopko): Handle user-defined types
  // TODO(hlopko): Handle user-defined types defined elsewhere (with fully
  // qualified paths)
  if (const clang::BuiltinType* builtin_type =
          qual_type->getAs<clang::BuiltinType>()) {
    if (builtin_type->isIntegerType()) {
      // TODO(hlopko): look at the actual width of the type.
      return Type{std::string("i32"), std::string("int")};
    }
    if (builtin_type->isVoidType()) {
      return Type::Void();
    }
  }
  LOG(FATAL) << "Unsupported type " << qual_type.getAsString() << "\n";
}

std::string AstVisitor::GetMangledName(
    const clang::NamedDecl* named_decl) const {
  std::string name;
  llvm::raw_string_ostream stream(name);
  mangler_->mangleName(named_decl, stream);
  stream.flush();
  return name;
}

Identifier AstVisitor::GetTranslatedName(
    const clang::NamedDecl* named_decl) const {
  // TODO(hlopko): handle the case where the name is not a simple identifier.
  return Identifier(std::string(named_decl->getName()));
}

}  // namespace rs_bindings_from_cc

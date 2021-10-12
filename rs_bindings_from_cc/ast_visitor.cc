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
#include "third_party/absl/strings/substitute.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Decl.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/DeclCXX.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Mangle.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/RawCommentList.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/RecordLayout.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Type.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/SourceLocation.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/SourceManager.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/Specifiers.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/Casting.h"

namespace rs_bindings_from_cc {

constexpr std::string_view kTypeStatusPayloadUrl =
    "type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type";

bool AstVisitor::TraverseDecl(clang::Decl* decl) {
  if (seen_decls_.insert(decl->getCanonicalDecl()).second) {
    return Base::TraverseDecl(decl);
  }
  return true;
}

bool AstVisitor::TraverseTranslationUnitDecl(
    clang::TranslationUnitDecl* translation_unit_decl) {
  ctx_ = &translation_unit_decl->getASTContext();
  mangler_.reset(ctx_->createMangleContext());

  for (const absl::string_view header_name : public_header_names_) {
    ir_.used_headers.push_back(HeaderName(std::string(header_name)));
  }

  return Base::TraverseTranslationUnitDecl(translation_unit_decl);
}

bool AstVisitor::VisitFunctionDecl(clang::FunctionDecl* function_decl) {
  std::vector<FuncParam> params;
  bool success = true;

  for (const clang::ParmVarDecl* param : function_decl->parameters()) {
    auto param_type = ConvertType(param->getType());
    if (!param_type.ok()) {
      ir_.items.push_back(UnsupportedItem{
          .name = function_decl->getQualifiedNameAsString(),
          .message = absl::Substitute("Parameter type '$0' is not supported",
                                      param->getType().getAsString()),
          .source_loc = ConvertSourceLoc(param->getBeginLoc())});
      success = false;
      continue;
    }
    std::optional<Identifier> param_name = GetTranslatedName(param);
    if (!param_name.has_value()) {
      ir_.items.push_back(UnsupportedItem{
          .name = function_decl->getQualifiedNameAsString(),
          .message = "Empty parameter names are not supported",
          .source_loc = ConvertSourceLoc(param->getBeginLoc())});
      success = false;
      continue;
    }
    params.push_back({*param_type, *std::move(param_name)});
  }

  auto return_type = ConvertType(function_decl->getReturnType());
  if (!return_type.ok()) {
    ir_.items.push_back(UnsupportedItem{
        .name = function_decl->getQualifiedNameAsString(),
        .message =
            absl::Substitute("Return type '$0' is not supported",
                             function_decl->getReturnType().getAsString()),
        .source_loc = ConvertSourceLoc(
            function_decl->getReturnTypeSourceRange().getBegin())});
    success = false;
  }
  std::optional<Identifier> translated_name = GetTranslatedName(function_decl);
  // For example, the destructor doesn't have a name.
  if (success && translated_name.has_value()) {
    ir_.items.push_back(Func{
        .identifier = *translated_name,
        .doc_comment = GetComment(function_decl),
        .mangled_name = GetMangledName(function_decl),
        .return_type = *return_type,
        .params = std::move(params),
        .is_inline = function_decl->isInlined(),
    });
  }

  return true;
}

static AccessSpecifier TranslateAccessSpecifier(clang::AccessSpecifier access) {
  switch (access) {
    case clang::AS_public:
      return kPublic;
    case clang::AS_protected:
      return kProtected;
    case clang::AS_private:
      return kPrivate;
    case clang::AS_none:
      // We should never be encoding a "none" access specifier in IR.
      assert(false);
      // We have to return something. Conservatively return private so we don't
      // inadvertently make a private member variable accessible in Rust.
      return kPrivate;
  }
}

bool AstVisitor::VisitRecordDecl(clang::RecordDecl* record_decl) {
  std::vector<Field> fields;
  clang::AccessSpecifier default_access = clang::AS_public;
  // The definition is always rewritten, but default access to `kPublic` in case
  // it is implicitly defined.
  SpecialMemberFunc copy_ctor = {
      .definition = SpecialMemberFunc::Definition::kTrivial,
      .access = kPublic,
  };
  SpecialMemberFunc move_ctor = {
      .definition = SpecialMemberFunc::Definition::kTrivial,
      .access = kPublic,
  };
  SpecialMemberFunc dtor = {
      .definition = SpecialMemberFunc::Definition::kTrivial,
      .access = kPublic,
  };
  if (const auto* cxx_record_decl =
          clang::dyn_cast<clang::CXXRecordDecl>(record_decl)) {
    if (cxx_record_decl->isClass()) {
      default_access = clang::AS_private;
    }

    if (cxx_record_decl->hasTrivialCopyConstructor()) {
      copy_ctor.definition = SpecialMemberFunc::Definition::kTrivial;
    } else if (cxx_record_decl->hasNonTrivialCopyConstructor()) {
      copy_ctor.definition = SpecialMemberFunc::Definition::kNontrivial;
    } else {
      // I don't think the copy ctor can be implicitly deleted, but just in
      // case...
      copy_ctor.definition = SpecialMemberFunc::Definition::kDeleted;
    }

    if (cxx_record_decl->hasTrivialMoveConstructor()) {
      move_ctor.definition = SpecialMemberFunc::Definition::kTrivial;
    } else if (cxx_record_decl->hasNonTrivialMoveConstructor()) {
      move_ctor.definition = SpecialMemberFunc::Definition::kNontrivial;
    } else {
      // The move constructor can be **implicitly deleted** (and so not subject
      // to the below loop over ctors), e.g. by the presence by a copy ctor.
      move_ctor.definition = SpecialMemberFunc::Definition::kDeleted;
    }

    if (cxx_record_decl->hasTrivialDestructor()) {
      dtor.definition = SpecialMemberFunc::Definition::kTrivial;
    } else {
      dtor.definition = SpecialMemberFunc::Definition::kNontrivial;
    }

    for (clang::CXXConstructorDecl* ctor_decl : cxx_record_decl->ctors()) {
      if (ctor_decl->isCopyConstructor()) {
        copy_ctor.access = TranslateAccessSpecifier(ctor_decl->getAccess());
        if (ctor_decl->isDeleted()) {
          copy_ctor.definition = SpecialMemberFunc::Definition::kDeleted;
        }
      } else if (ctor_decl->isMoveConstructor()) {
        move_ctor.access = TranslateAccessSpecifier(ctor_decl->getAccess());
        if (ctor_decl->isDeleted()) {
          move_ctor.definition = SpecialMemberFunc::Definition::kDeleted;
        }
      }
    }
    clang::CXXDestructorDecl* dtor_decl = cxx_record_decl->getDestructor();
    if (dtor_decl != nullptr) {
      dtor.access = TranslateAccessSpecifier(dtor_decl->getAccess());
      if (dtor_decl->isDeleted()) {
        dtor.definition = SpecialMemberFunc::Definition::kDeleted;
      }
    }
  }
  const clang::ASTRecordLayout& layout = ctx_->getASTRecordLayout(record_decl);
  for (const clang::FieldDecl* field_decl : record_decl->fields()) {
    auto type = ConvertType(field_decl->getType());
    if (!type.ok()) {
      // TODO(b/200239975):  Add diagnostics for declarations we can't import
      return true;
    }
    clang::AccessSpecifier access = field_decl->getAccess();
    if (access == clang::AS_none) {
      access = default_access;
    }

    std::optional<Identifier> field_name = GetTranslatedName(field_decl);
    if (!field_name.has_value()) {
      return true;
    }
    fields.push_back(
        {.identifier = *std::move(field_name),
         .doc_comment = GetComment(field_decl),
         .type = *type,
         .access = TranslateAccessSpecifier(access),
         .offset = layout.getFieldOffset(field_decl->getFieldIndex())});
  }
  std::optional<Identifier> record_name = GetTranslatedName(record_decl);
  if (!record_name.has_value()) {
    return true;
  }
  ir_.items.push_back(
      Record{.identifier = *record_name,
             .doc_comment = GetComment(record_decl),
             .fields = std::move(fields),
             .size = layout.getSize().getQuantity(),
             .alignment = layout.getAlignment().getQuantity(),
             .copy_constructor = copy_ctor,
             .move_constructor = move_ctor,
             .destructor = dtor,
             .is_trivial_abi = record_decl->canPassInRegisters()});
  return true;
}

std::optional<std::string> AstVisitor::GetComment(
    const clang::Decl* decl) const {
  // This does currently not distinguish between different types of comments.
  // In general it is not possible in C++ to reliably only extract doc comments.
  // This is going to be a heuristic that needs to be tuned over time.

  clang::SourceManager& sm = ctx_->getSourceManager();
  clang::RawComment* raw_comment = ctx_->getRawCommentForDeclNoCache(decl);

  if (raw_comment == nullptr) {
    return {};
  } else {
    return raw_comment->getFormattedText(sm, sm.getDiagnostics());
  }
}

SourceLoc AstVisitor::ConvertSourceLoc(clang::SourceLocation loc) const {
  auto& sm = ctx_->getSourceManager();

  auto filename = sm.getFileEntryForID(sm.getFileID(loc))->getName();
  if (filename.startswith("./")) {
    filename = filename.substr(2);
  }

  return SourceLoc{.filename = filename.str(),
                   .line = sm.getSpellingLineNumber(loc),
                   .column = sm.getSpellingColumnNumber(loc)};
}

absl::StatusOr<MappedType> AstVisitor::ConvertType(
    clang::QualType qual_type) const {
  std::optional<MappedType> type = std::nullopt;
  std::string type_string = qual_type.getAsString();

  if (const clang::PointerType* pointer_type =
          qual_type->getAs<clang::PointerType>()) {
    auto pointee_type = ConvertType(pointer_type->getPointeeType());
    if (pointee_type.ok()) {
      type = MappedType::PointerTo(*pointee_type);
    }
  } else if (const clang::BuiltinType* builtin_type =
                 qual_type->getAs<clang::BuiltinType>()) {
    switch (builtin_type->getKind()) {
      case clang::BuiltinType::Bool:
        type = MappedType::Simple("bool", "bool");
        break;
      case clang::BuiltinType::Float:
        type = MappedType::Simple("f32", "float");
        break;
      case clang::BuiltinType::Double:
        type = MappedType::Simple("f64", "double");
        break;
      case clang::BuiltinType::Void:
        type = MappedType::Void();
        break;
      default:
        if (builtin_type->isIntegerType()) {
          auto size = ctx_->getTypeSize(builtin_type);
          if (size == 64 &&
              (type_string == "ptrdiff_t" || type_string == "intptr_t")) {
            type = MappedType::Simple("isize", type_string);
          } else if (size == 64 &&
                     (type_string == "size_t" || type_string == "uintptr_t")) {
            type = MappedType::Simple("usize", type_string);
          } else if (size == 8 || size == 16 || size == 32 || size == 64) {
            type = MappedType::Simple(
                absl::Substitute(
                    "$0$1", builtin_type->isSignedInteger() ? 'i' : 'u', size),
                type_string);
          }
        }
    }
  }

  if (!type.has_value()) {
    absl::Status error = absl::UnimplementedError(
        absl::Substitute("Unsupported type '$0'", type_string));
    error.SetPayload(kTypeStatusPayloadUrl, absl::Cord(type_string));
    return error;
  }

  // Add cv-qualification.
  type->cc_type.is_const = qual_type.isConstQualified();
  // Not doing volatile for now -- note that volatile pointers do not exist in
  // Rust, though volatile reads/writes still do.

  return *std::move(type);
}

std::string AstVisitor::GetMangledName(
    const clang::NamedDecl* named_decl) const {
  std::string name;
  llvm::raw_string_ostream stream(name);
  mangler_->mangleName(named_decl, stream);
  stream.flush();
  return name;
}

std::optional<Identifier> AstVisitor::GetTranslatedName(
    const clang::NamedDecl* named_decl) const {
  clang::IdentifierInfo* id = named_decl->getIdentifier();
  if (id == nullptr) {
    return std::nullopt;
  }
  return Identifier(std::string(id->getName()));
}

}  // namespace rs_bindings_from_cc

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ast_visitor.h"

#include <stdint.h>

#include <algorithm>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "base/logging.h"
#include "rs_bindings_from_cc/ast_convert.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "third_party/absl/container/flat_hash_map.h"
#include "third_party/absl/container/flat_hash_set.h"
#include "third_party/absl/status/status.h"
#include "third_party/absl/status/statusor.h"
#include "third_party/absl/strings/cord.h"
#include "third_party/absl/strings/str_cat.h"
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
#include "third_party/llvm/llvm-project/clang/include/clang/Sema/Sema.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/Casting.h"

namespace rs_bindings_from_cc {

constexpr std::string_view kTypeStatusPayloadUrl =
    "type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type";

static DeclId GenerateDeclId(const clang::Decl* decl) {
  return DeclId(reinterpret_cast<uintptr_t>(decl->getCanonicalDecl()));
}

bool AstVisitor::TraverseDecl(clang::Decl* decl) {
  if (!seen_decls_.insert(decl->getCanonicalDecl()).second) {
    return true;
  }

  const clang::DeclContext* decl_context = decl->getDeclContext();
  if (decl_context && decl_context->isNamespace()) {
    PushUnsupportedItem(decl,
                        "Items contained in namespaces are not supported yet",
                        decl->getBeginLoc());

    return true;
  }

  // Emit all comments in the current file before the decl
  comment_manager_.TraverseDecl(decl);

  return Base::TraverseDecl(decl);
}

bool AstVisitor::TraverseTranslationUnitDecl(
    clang::TranslationUnitDecl* translation_unit_decl) {
  ctx_ = &translation_unit_decl->getASTContext();
  mangler_.reset(ctx_->createMangleContext());

  for (const HeaderName& header_name : public_header_names_) {
    ir_.used_headers.push_back(header_name);
  }

  ir_.current_target = current_target_;

  bool result = Base::TraverseTranslationUnitDecl(translation_unit_decl);

  // Emit comments after the last decl
  comment_manager_.FlushComments();

  return result;
}

bool AstVisitor::VisitFunctionDecl(clang::FunctionDecl* function_decl) {
  if (!IsFromCurrentTarget(function_decl)) return true;
  if (function_decl->isDeleted()) return true;

  devtools_rust::LifetimeSymbolTable lifetime_symbol_table;
  llvm::Expected<devtools_rust::FunctionLifetimes> lifetimes =
      devtools_rust::GetLifetimeAnnotations(function_decl, lifetime_context_,
                                            &lifetime_symbol_table);
  llvm::DenseSet<devtools_rust::Lifetime> all_lifetimes;

  std::vector<FuncParam> params;
  bool success = true;
  // non-static member functions receive an implicit `this` parameter.
  if (auto* method_decl = llvm::dyn_cast<clang::CXXMethodDecl>(function_decl)) {
    if (method_decl->isInstance()) {
      std::optional<devtools_rust::TypeLifetimes> this_lifetimes;
      if (lifetimes) {
        this_lifetimes = lifetimes->this_lifetimes;
        all_lifetimes.insert(this_lifetimes->begin(), this_lifetimes->end());
      }
      auto param_type = ConvertType(method_decl->getThisType(), this_lifetimes,
                                    /*nullable=*/false);
      if (!param_type.ok()) {
        PushUnsupportedItem(function_decl, param_type.status().ToString(),
                            method_decl->getBeginLoc());

        success = false;
      } else {
        params.push_back({*std::move(param_type), Identifier("__this")});
      }
    }
  }

  if (lifetimes) {
    CHECK_EQ(lifetimes->param_lifetimes.size(), function_decl->getNumParams());
  }
  for (unsigned i = 0; i < function_decl->getNumParams(); ++i) {
    const clang::ParmVarDecl* param = function_decl->getParamDecl(i);
    std::optional<devtools_rust::TypeLifetimes> param_lifetimes;
    if (lifetimes) {
      param_lifetimes = lifetimes->param_lifetimes[i];
      all_lifetimes.insert(param_lifetimes->begin(), param_lifetimes->end());
    }
    auto param_type = ConvertType(param->getType(), param_lifetimes);
    if (!param_type.ok()) {
      PushUnsupportedItem(
          function_decl,
          absl::Substitute("Parameter type '$0' is not supported",
                           param->getType().getAsString()),
          param->getBeginLoc());
      success = false;
      continue;
    }

    if (const clang::RecordType* record_type =
            llvm::dyn_cast<clang::RecordType>(param->getType())) {
      if (clang::RecordDecl* record_decl =
              llvm::dyn_cast<clang::RecordDecl>(record_type->getDecl())) {
        // TODO(b/200067242): non-trivial_abi structs, when passed by value,
        // have a different representation which needs special support. We
        // currently do not support it.
        if (!record_decl->canPassInRegisters()) {
          PushUnsupportedItem(
              function_decl,
              absl::Substitute("Non-trivial_abi type '$0' is not "
                               "supported by value as a parameter",
                               param->getType().getAsString()),
              param->getBeginLoc());
          success = false;
        }
      }
    }

    std::optional<Identifier> param_name = GetTranslatedIdentifier(param);
    CHECK(param_name.has_value());  // No known cases where the above can fail.
    params.push_back({*param_type, *std::move(param_name)});
  }

  if (const clang::RecordType* record_return_type =
          llvm::dyn_cast<clang::RecordType>(function_decl->getReturnType())) {
    if (clang::RecordDecl* record_decl =
            llvm::dyn_cast<clang::RecordDecl>(record_return_type->getDecl())) {
      // TODO(b/200067242): non-trivial_abi structs, when passed by value,
      // have a different representation which needs special support. We
      // currently do not support it.
      if (!record_decl->canPassInRegisters()) {
        PushUnsupportedItem(
            function_decl,
            absl::Substitute("Non-trivial_abi type '$0' is not supported "
                             "by value as a return type",
                             function_decl->getReturnType().getAsString()),
            function_decl->getReturnTypeSourceRange());
        success = false;
      }
    }
  }

  std::optional<devtools_rust::TypeLifetimes> return_lifetimes;
  if (lifetimes) {
    return_lifetimes = lifetimes->return_lifetimes;
    all_lifetimes.insert(return_lifetimes->begin(), return_lifetimes->end());
  }
  auto return_type =
      ConvertType(function_decl->getReturnType(), return_lifetimes);
  if (!return_type.ok()) {
    PushUnsupportedItem(
        function_decl,
        absl::Substitute("Return type '$0' is not supported",
                         function_decl->getReturnType().getAsString()),
        function_decl->getReturnTypeSourceRange());
    success = false;
  }

  std::vector<Lifetime> lifetime_params;
  for (devtools_rust::Lifetime lifetime : all_lifetimes) {
    std::optional<llvm::StringRef> name =
        lifetime_symbol_table.LookupLifetime(lifetime);
    CHECK(name.has_value());
    lifetime_params.push_back(
        {.name = name->str(), .id = LifetimeId(lifetime.Id())});
  }
  std::sort(
      lifetime_params.begin(), lifetime_params.end(),
      [](const Lifetime& l1, const Lifetime& l2) { return l1.name < l2.name; });

  std::optional<MemberFuncMetadata> member_func_metadata;
  if (auto* method_decl = llvm::dyn_cast<clang::CXXMethodDecl>(function_decl)) {
    switch (method_decl->getAccess()) {
      case clang::AS_public:
        break;
      case clang::AS_protected:
      case clang::AS_private:
      case clang::AS_none:
        // No need for IR to include Func representing private methods.
        // TODO(lukasza): Revisit this for protected methods.
        return true;
    }
    std::optional<MemberFuncMetadata::InstanceMethodMetadata> instance_metadata;
    if (method_decl->isInstance()) {
      MemberFuncMetadata::ReferenceQualification reference;
      switch (method_decl->getRefQualifier()) {
        case clang::RQ_LValue:
          reference = MemberFuncMetadata::kLValue;
          break;
        case clang::RQ_RValue:
          reference = MemberFuncMetadata::kRValue;
          break;
        case clang::RQ_None:
          reference = MemberFuncMetadata::kUnqualified;
          break;
      }
      instance_metadata = MemberFuncMetadata::InstanceMethodMetadata{
          .reference = reference,
          .is_const = method_decl->isConst(),
          .is_virtual = method_decl->isVirtual(),
      };
    }

    member_func_metadata = MemberFuncMetadata{
        .record_id = GenerateDeclId(method_decl->getParent()),
        .instance_method_metadata = instance_metadata};
  }

  std::optional<UnqualifiedIdentifier> translated_name =
      GetTranslatedName(function_decl);
  if (success && translated_name.has_value()) {
    ir_.items.push_back(Func{
        .name = *translated_name,
        .owning_target = GetOwningTarget(function_decl),
        .doc_comment = GetComment(function_decl),
        .mangled_name = GetMangledName(function_decl),
        .return_type = *return_type,
        .params = std::move(params),
        .lifetime_params = std::move(lifetime_params),
        .is_inline = function_decl->isInlined(),
        .member_func_metadata = std::move(member_func_metadata),
    });
  }

  return true;
}

Label AstVisitor::GetOwningTarget(const clang::Decl* decl) const {
  clang::SourceManager& source_manager = ctx_->getSourceManager();
  llvm::StringRef filename = source_manager.getFilename(decl->getLocation());

  if (filename.startswith("./")) filename = filename.substr(2);
  auto target_iterator = headers_to_targets_.find(HeaderName(filename.str()));
  if (target_iterator == headers_to_targets_.end()) {
    // TODO(b/208377928): replace this hack with a
    // CHECK(target_iterator != headers_to_targets_.end()) once we generate
    // bindings for headers in Clang's resource dir.
    return Label("//:virtual_clang_resource_dir_target");
  }
  return target_iterator->second;
}

bool AstVisitor::IsFromCurrentTarget(const clang::Decl* decl) const {
  return current_target_ == GetOwningTarget(decl);
}

bool AstVisitor::VisitRecordDecl(clang::RecordDecl* record_decl) {
  const clang::DeclContext* decl_context = record_decl->getDeclContext();
  if (decl_context && decl_context->isRecord()) {
    PushUnsupportedItem(record_decl, "Nested classes are not supported yet",
                        record_decl->getBeginLoc());
    return true;
  }

  clang::AccessSpecifier default_access = clang::AS_public;

  bool is_final = true;
  if (auto* cxx_record_decl =
          clang::dyn_cast<clang::CXXRecordDecl>(record_decl)) {
    sema_.ForceDeclarationOfImplicitMembers(cxx_record_decl);
    if (cxx_record_decl->isClass()) {
      default_access = clang::AS_private;
    }
    is_final = cxx_record_decl->isEffectivelyFinal();
  }
  std::optional<std::vector<Field>> fields =
      ImportFields(record_decl, default_access);
  if (!fields.has_value()) {
    return true;
  }
  std::optional<Identifier> record_name = GetTranslatedIdentifier(record_decl);
  if (!record_name.has_value()) {
    return true;
  }
  const clang::ASTRecordLayout& layout = ctx_->getASTRecordLayout(record_decl);
  ir_.items.push_back(
      Record{.identifier = *record_name,
             .id = GenerateDeclId(record_decl),
             .owning_target = GetOwningTarget(record_decl),
             .doc_comment = GetComment(record_decl),
             .fields = *std::move(fields),
             .size = layout.getSize().getQuantity(),
             .alignment = layout.getAlignment().getQuantity(),
             .copy_constructor = GetCopyCtorSpecialMemberFunc(*record_decl),
             .move_constructor = GetMoveCtorSpecialMemberFunc(*record_decl),
             .destructor = GetDestructorSpecialMemberFunc(*record_decl),
             .is_trivial_abi = record_decl->canPassInRegisters(),
             .is_final = is_final});
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

void AstVisitor::PushUnsupportedItem(const clang::Decl* decl,
                                     std::string message,
                                     clang::SourceLocation source_location) {
  if (!IsFromCurrentTarget(decl)) return;

  std::string name = "unnamed";
  if (const auto* named_decl = llvm::dyn_cast<clang::NamedDecl>(decl)) {
    name = named_decl->getQualifiedNameAsString();
  }
  ir_.items.push_back(UnsupportedItem{
      .name = std::move(name),
      .message = std::move(message),
      .source_loc = ConvertSourceLocation(std::move(source_location))});
}

void AstVisitor::PushUnsupportedItem(const clang::Decl* decl,
                                     std::string message,
                                     clang::SourceRange source_range) {
  PushUnsupportedItem(decl, message, source_range.getBegin());
}

SourceLoc AstVisitor::ConvertSourceLocation(clang::SourceLocation loc) const {
  auto& sm = ctx_->getSourceManager();

  clang::StringRef filename = sm.getFilename(loc);
  if (filename.startswith("./")) {
    filename = filename.substr(2);
  }

  return SourceLoc{.filename = filename.str(),
                   .line = sm.getSpellingLineNumber(loc),
                   .column = sm.getSpellingColumnNumber(loc)};
}

absl::StatusOr<MappedType> AstVisitor::ConvertType(
    clang::QualType qual_type,
    std::optional<devtools_rust::TypeLifetimes> lifetimes,
    bool nullable) const {
  std::optional<MappedType> type = std::nullopt;
  std::string type_string = qual_type.getAsString();

  if (const auto* pointer_type = qual_type->getAs<clang::PointerType>()) {
    std::optional<LifetimeId> lifetime;
    if (lifetimes.has_value()) {
      CHECK(!lifetimes->empty());
      lifetime = LifetimeId(lifetimes->back().Id());
      lifetimes->pop_back();
    }
    auto pointee_type = ConvertType(pointer_type->getPointeeType(), lifetimes);
    if (pointee_type.ok()) {
      type = MappedType::PointerTo(*pointee_type, lifetime, nullable);
    }
  } else if (const auto* lvalue_ref_type =
                 qual_type->getAs<clang::LValueReferenceType>()) {
    std::optional<LifetimeId> lifetime;
    if (lifetimes.has_value()) {
      CHECK(!lifetimes->empty());
      lifetime = LifetimeId(lifetimes->back().Id());
      lifetimes->pop_back();
    }
    auto pointee_type =
        ConvertType(lvalue_ref_type->getPointeeType(), lifetimes);
    if (pointee_type.ok()) {
      type = MappedType::LValueReferenceTo(*pointee_type, lifetime);
    }
  } else if (const auto* builtin_type =
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
  } else if (const auto* tag_type = qual_type->getAs<clang::TagType>()) {
    // TODO(b/202692734): If tag_type is un-importable, fail here.
    clang::TagDecl* tag_decl = tag_type->getDecl();

    if (std::optional<Identifier> id = GetTranslatedIdentifier(tag_decl)) {
      std::string ident(id->Ident());
      DeclId decl_id = GenerateDeclId(tag_decl);
      type = MappedType::WithDeclIds(ident, decl_id, ident, decl_id);
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

std::optional<std::vector<Field>> AstVisitor::ImportFields(
    clang::RecordDecl* record_decl, clang::AccessSpecifier default_access) {
  std::vector<Field> fields;
  const clang::ASTRecordLayout& layout = ctx_->getASTRecordLayout(record_decl);
  for (const clang::FieldDecl* field_decl : record_decl->fields()) {
    auto type = ConvertType(field_decl->getType());
    if (!type.ok()) {
      // TODO(b/200239975):  Add diagnostics for declarations we can't import
      return std::nullopt;
    }
    clang::AccessSpecifier access = field_decl->getAccess();
    if (access == clang::AS_none) {
      access = default_access;
    }

    std::optional<Identifier> field_name = GetTranslatedIdentifier(field_decl);
    if (!field_name.has_value()) {
      return std::nullopt;
    }
    fields.push_back(
        {.identifier = *std::move(field_name),
         .doc_comment = GetComment(field_decl),
         .type = *type,
         .access = TranslateAccessSpecifier(access),
         .offset = layout.getFieldOffset(field_decl->getFieldIndex())});
  }
  return fields;
}

std::string AstVisitor::GetMangledName(
    const clang::NamedDecl* named_decl) const {
  clang::GlobalDecl decl;

  // There are only three named decl types that don't work with the GlobalDecl
  // unary constructor: GPU kernels (which do not exist in standard C++, so we
  // ignore), constructors, and destructors. GlobalDecl does not support
  // constructors and destructors from the unary constructor because there is
  // more than one global declaration for a given constructor or destructor!
  //
  //   * (Ctor|Dtor)_Complete is a function which constructs / destroys the
  //     entire object. This is what we want. :)
  //   * Dtor_Deleting is a function which additionally calls operator delete.
  //   * (Ctor|Dtor)_Base is a function which constructs/destroys the object but
  //     NOT including virtual base class subobjects.
  //   * (Ctor|Dtor)_Comdat: I *believe* this is the identifier used to
  //     deduplicate inline functions, and is not callable.
  //   * Dtor_(Copying|Default)Closure: These only exist in the MSVC++ ABI,
  //     which we don't support for now. I don't know when they are used.
  //
  // It was hard to piece this together, so writing it down here to explain why
  // we magically picked the *_Complete variants.
  if (auto dtor = llvm::dyn_cast<clang::CXXDestructorDecl>(named_decl)) {
    decl = clang::GlobalDecl(dtor, clang::CXXDtorType::Dtor_Complete);
  } else if (auto ctor =
                 llvm::dyn_cast<clang::CXXConstructorDecl>(named_decl)) {
    decl = clang::GlobalDecl(ctor, clang::CXXCtorType::Ctor_Complete);
  } else {
    decl = clang::GlobalDecl(named_decl);
  }

  std::string name;
  llvm::raw_string_ostream stream(name);
  mangler_->mangleName(decl, stream);
  stream.flush();
  return name;
}

std::optional<UnqualifiedIdentifier> AstVisitor::GetTranslatedName(
    const clang::NamedDecl* named_decl) const {
  switch (named_decl->getDeclName().getNameKind()) {
    case clang::DeclarationName::Identifier: {
      auto name = std::string(named_decl->getName());
      if (name.empty()) {
        if (const clang::ParmVarDecl* param_decl =
                clang::dyn_cast<clang::ParmVarDecl>(named_decl)) {
          int param_pos = param_decl->getFunctionScopeIndex();
          return {Identifier(absl::StrCat("__param_", param_pos))};
        }
        // TODO(lukasza): Handle anonymous structs (probably this won't be an
        // issue until nested types are handled - b/200067824).
        return std::nullopt;
      }
      return {Identifier(std::move(name))};
    }
    case clang::DeclarationName::CXXConstructorName:
      return {SpecialName::kConstructor};
    case clang::DeclarationName::CXXDestructorName:
      return {SpecialName::kDestructor};
    default:
      // To be implemented later: operators, conversion functions.
      // There are also e.g. literal operators, deduction guides, etc., but
      // we might not need to implement them at all. Full list at:
      // https://clang.llvm.org/doxygen/classclang_1_1DeclarationName.html#a9ab322d434446b43379d39e41af5cbe3
      return std::nullopt;
  }
}

void AstVisitor::CommentManager::TraverseDecl(clang::Decl* decl) {
  ctx_ = &decl->getASTContext();

  // When we go to a new file we flush the comments from the previous file,
  // because source locations won't be comparable by '<' any more.
  clang::FileID file = ctx_->getSourceManager().getFileID(decl->getBeginLoc());
  // For example, we hit an invalid FileID for virtual destructor definitions.
  if (file.isInvalid()) {
    return;
  }
  if (file != current_file_) {
    FlushComments();
    current_file_ = file;
    LoadComments();
  }

  // Visit all comments from the current file up to the current decl.
  clang::RawComment* decl_comment = ctx_->getRawCommentForDeclNoCache(decl);
  while (next_comment_ != file_comments_.end() &&
         (*next_comment_)->getBeginLoc() < decl->getBeginLoc()) {
    // Skip the decl's doc comment, which will be emitted as part of the decl.
    if (*next_comment_ != decl_comment) {
      VisitTopLevelComment(*next_comment_);
    }
    ++next_comment_;
  }

  // Skip comments that are within the decl, e.g., comments in the body of an
  // inline function
  // TODO(forster): We should retain floating comments within `Record`s
  if (!clang::isa<clang::NamespaceDecl>(decl)) {
    while (next_comment_ != file_comments_.end() &&
           (*next_comment_)->getBeginLoc() < decl->getEndLoc()) {
      ++next_comment_;
    }
  }
}

void AstVisitor::CommentManager::LoadComments() {
  auto comments = ctx_->Comments.getCommentsInFile(current_file_);
  if (comments) {
    for (auto [_, comment] : *comments) {
      file_comments_.push_back(comment);
    }
  }
  next_comment_ = file_comments_.begin();
}

void AstVisitor::CommentManager::FlushComments() {
  while (next_comment_ != file_comments_.end()) {
    VisitTopLevelComment(*next_comment_);
    next_comment_++;
  }
  file_comments_.clear();
}

void AstVisitor::CommentManager::VisitTopLevelComment(
    clang::RawComment* comment) {
  clang::SourceManager& sm = ctx_->getSourceManager();
  ir_.items.push_back(
      Comment{.text = comment->getFormattedText(sm, sm.getDiagnostics())});
}

}  // namespace rs_bindings_from_cc

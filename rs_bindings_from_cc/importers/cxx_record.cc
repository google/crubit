// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/cxx_record.h"

#include <optional>

#include "clang/Basic/SourceLocation.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/check.h"
#include "absl/log/die_if_null.h"
#include "absl/log/log.h"
#include "rs_bindings_from_cc/ast_convert.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/CXXInheritance.h"
#include "clang/AST/Decl.h"
#include "clang/AST/PrettyPrinter.h"
#include "clang/AST/RecordLayout.h"
#include "clang/AST/Type.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Sema/Sema.h"
#include "llvm/Support/ErrorHandling.h"
namespace crubit {

namespace {

// Types which are overridden to pretend to be final.
//
// WARNING: marking a non-final type as final is very dangerous!
// See docs/unpin.md
//
// In particular, only include a type in `FinalOverrides` if the type has no
// usable tail padding -- for example, if
/// `std::has_unique_object_representations_v<T>`, or 3if the type itself is
// POD for the purpose of layout (in the Itanium ABI).
//
// This should be enforced by asserting in `override_final_test.cc` that it
// has no tail padding.
const absl::flat_hash_set<absl::string_view>& FinalOverrides() {
  static auto& final_overrides = *new absl::flat_hash_set<absl::string_view>{
      // string_view only has a pointer and a size_t, which are both the same
      // size, and so has no usable tail padding.
      "std::string_view",
  };
  return final_overrides;
}

std::string GetClassTemplateSpecializationCcName(
    const clang::ASTContext& ast_context,
    const clang::ClassTemplateSpecializationDecl* specialization_decl,
    bool use_preferred_names) {
  clang::PrintingPolicy policy(ast_context.getLangOpts());
  policy.IncludeTagDefinition = false;
  // Canonicalize types -- in particular, the template parameter types must be
  // desugared out of an `ElaboratedType` so that their namespaces are written
  // down.
  policy.PrintCanonicalTypes = true;
  policy.UsePreferredNames = use_preferred_names;
  // Use type suffix (e.g. `123u` rather than just `123`) to avoid the
  // `-Wimplicitly-unsigned-literal` warning.  See also b/244616557.
  policy.AlwaysIncludeTypeForTemplateArgument = true;

  return clang::QualType(specialization_decl->getTypeForDecl(), 0)
      .getAsString(policy);
}

AccessSpecifier TranslateAccessSpecifier(clang::AccessSpecifier access) {
  switch (access) {
    case clang::AS_public:
      return kPublic;
    case clang::AS_protected:
      return kProtected;
    case clang::AS_private:
      return kPrivate;
    case clang::AS_none:
      LOG(FATAL)
          << "We should never be encoding a 'none' access specifier in IR.";
      // We have to return something. Conservatively return private so we don't
      // inadvertently make a private member variable accessible in Rust.
      return kPrivate;
  }
}

absl::StatusOr<RecordType> TranslateRecordType(
    const clang::RecordDecl& record_decl) {
  switch (record_decl.getTagKind()) {
    case clang::TTK_Struct:
      return RecordType::kStruct;
    case clang::TTK_Union:
      return RecordType::kUnion;
    case clang::TTK_Class:
      return RecordType::kClass;
    case clang::TTK_Enum:
      llvm::report_fatal_error(
          "clang::RecordDecl::getTagKind shouldn't return TTK_Enum");
    case clang::TTK_Interface:
      // Some docs about `__interface` can be found here:
      // https://docs.microsoft.com/en-us/cpp/cpp/interface?view=msvc-170
      return absl::UnimplementedError(
          "`__interface` / clang::TTK_Interface is not supported");
  }

  llvm::report_fatal_error("Unrecognized clang::TagKind");
}

}  // namespace

std::optional<Identifier> CXXRecordDeclImporter::GetTranslatedFieldName(
    const clang::FieldDecl* field_decl) {
  if (field_decl->getName().empty()) {
    CHECK(!field_decl->hasAttr<clang::NoUniqueAddressAttr>() &&
          "Unnamed fields can't be annotated with [[no_unique_address]]");
    // We don't just conjure an artificial name for an unnamed field, because
    // in the future such fields may be elided entirely - see unnamed members
    // in:
    // - https://en.cppreference.com/w/c/language/struct
    // - https://rust-lang.github.io/rfcs/2102-unnamed-fields.html
    return std::nullopt;
  }

  absl::StatusOr<Identifier> name = ictx_.GetTranslatedIdentifier(field_decl);
  if (!name.ok()) {
    unsigned field_pos = field_decl->getFieldIndex();
    return {Identifier(absl::StrCat("__field_", field_pos))};
  }
  return *name;
}

std::optional<IR::Item> CXXRecordDeclImporter::Import(
    clang::CXXRecordDecl* record_decl) {
  const clang::DeclContext* decl_context = record_decl->getDeclContext();
  if (decl_context->isFunctionOrMethod()) {
    return std::nullopt;
  }
  if (ictx_.HasBeenAlreadySuccessfullyImported(record_decl)) {
    return ictx_.ImportUnsupportedItem(
        record_decl,
        "THIS IS A BUG: the type was marked as imported, so we short-circuited "
        "evaluation here. However, instead of the fully imported type being "
        "used, apparently this empty stub was used instead. Report this "
        "upstream.");
  }
  if (record_decl->isInjectedClassName()) {
    return std::nullopt;
  }

  if (record_decl->isImplicit()) {
    return std::nullopt;
  }
  if (decl_context->isRecord()) {
    return ictx_.ImportUnsupportedItem(record_decl,
                                       "Nested classes are not supported yet");
  }
  if (clang::isa<clang::ClassTemplatePartialSpecializationDecl>(record_decl)) {
    return ictx_.ImportUnsupportedItem(
        record_decl, "Partially-specialized class templates are not supported");
  }
  if (record_decl->isDependentContext()) {
    // We can't pass this to getASTRecordLayout() or it'll segfault.
    // TODO(jeanpierreda): investigate what we can do to support dependent records?
    // All I know is that I saw other code calling getASTRecordLayout() do the
    // same check. But getASTRecordLayout() itself doesn't actually document
    // this.
    return ictx_.ImportUnsupportedItem(record_decl,
                                       "Dependent records are not supported");
  }
  if (record_decl->isInvalidDecl()) {
    return std::nullopt;
  }
  if (record_decl->isInStdNamespace() &&
      record_decl->hasAttr<clang::VisibilityAttr>()) {
    auto visibility = record_decl->getAttr<clang::VisibilityAttr>();
    if (visibility->getVisibility() ==
        clang::VisibilityAttr::VisibilityType::Hidden) {
      return ictx_.ImportUnsupportedItem(
          record_decl,
          "Records from the standard library with hidden visibility are not "
          "supported");
    }
  }

  // The less expensive `getName` comparison is done first, because the
  // documentation of `NamedDecl::getQualifiedNameAsString` says that "it should
  // be called only when performance doesn't matter".
  if (record_decl->getName() == "rs_char" &&
      record_decl->getQualifiedNameAsString() == "rs_std::rs_char") {
    return ictx_.ImportUnsupportedItem(
        record_decl,
        "Round-tripping of `rs_char` is not supported yet (b/270160530)");
  }

  absl::StatusOr<RecordType> record_type = TranslateRecordType(*record_decl);
  if (!record_type.ok()) {
    return ictx_.ImportUnsupportedItem(
        record_decl, std::string(record_type.status().message()));
  }

  if (record_decl->hasAttr<clang::PackedAttr>() ||
      std::any_of(record_decl->field_begin(), record_decl->field_end(),
                  [](const clang::FieldDecl* field_decl) {
                    return field_decl->hasAttr<clang::PackedAttr>();
                  })) {
    return ictx_.ImportUnsupportedItem(
        record_decl, "Records with packed layout are not supported");
  }

  std::string rs_name, cc_name, preferred_cc_name;
  clang::SourceLocation source_loc;
  std::optional<std::string> doc_comment;
  bool is_explicit_class_template_instantiation_definition = false;
  if (auto* specialization_decl =
          clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(
              record_decl)) {
    is_explicit_class_template_instantiation_definition =
        specialization_decl->getSpecializationKind() ==
        clang::TSK_ExplicitInstantiationDefinition;
    rs_name = ictx_.GetMangledName(specialization_decl);
    // use_preferred_names = false so that this returns e.g.
    // `basic_string_view<char16_t>` instead of 'u16string_view' despite
    // `_LIBCPP_PREFERRED_NAME(u16string_view)`.  See also b/244350186.
    cc_name = GetClassTemplateSpecializationCcName(
        ictx_.ctx_, specialization_decl, /*use_preferred_names=*/false);
    preferred_cc_name = GetClassTemplateSpecializationCcName(
        ictx_.ctx_, specialization_decl, /*use_preferred_names=*/true);
    doc_comment = ictx_.GetComment(specialization_decl);
    if (!doc_comment.has_value()) {
      doc_comment =
          ictx_.GetComment(specialization_decl->getSpecializedTemplate());
    }
    source_loc = specialization_decl->getBeginLoc();
  } else {
    const clang::NamedDecl* named_decl = record_decl;
    if (record_decl->getName().empty()) {
      if (auto* typedef_decl = record_decl->getTypedefNameForAnonDecl()) {
        named_decl = typedef_decl;
      } else {
        // Skip anonymous structs that don't get a name via typedecl.
        return std::nullopt;
      }
    }
    CHECK(!named_decl->getName().empty());

    absl::StatusOr<Identifier> record_name =
        ictx_.GetTranslatedIdentifier(named_decl);
    if (record_name.ok()) {
      rs_name = cc_name = record_name->Ident();
      doc_comment = ictx_.GetComment(record_decl);
      source_loc = record_decl->getBeginLoc();
    } else {
      return ictx_.ImportUnsupportedItem(
          record_decl, absl::StrCat("Record name is not supported: ",
                                    record_name.status().message()));
    }
  }

  if (clang::CXXRecordDecl* complete = record_decl->getDefinition()) {
    record_decl = complete;
  } else {
    CHECK(!record_decl->isCompleteDefinition());
    ictx_.MarkAsSuccessfullyImported(record_decl);
    return IncompleteRecord{
        .cc_name = std::move(cc_name),
        .rs_name = std::move(rs_name),
        .id = GenerateItemId(record_decl),
        .owning_target = ictx_.GetOwningTarget(record_decl),
        .record_type = *record_type,
        .enclosing_namespace_id = GetEnclosingNamespaceId(record_decl)};
  }

  // At this point we know that the import of `record_decl` will succeed /
  // cannot fail.
  ictx_.MarkAsSuccessfullyImported(record_decl);

  ictx_.sema_.ForceDeclarationOfImplicitMembers(record_decl);

  const clang::ASTRecordLayout& layout =
      ictx_.ctx_.getASTRecordLayout(record_decl);

  bool is_derived_class = record_decl->getNumBases() != 0;
  bool override_alignment = record_decl->hasAttr<clang::AlignedAttr>() ||
                            is_derived_class || layout.hasOwnVFPtr();

  std::vector<Field> fields = ImportFields(record_decl);
  for (const Field& field : fields) {
    if (field.is_no_unique_address || !field.type.ok()) {
      override_alignment = true;
      break;
    }
  }

  bool is_effectively_final = record_decl->isEffectivelyFinal() ||
                              record_decl->isUnion() ||
                              FinalOverrides().contains(preferred_cc_name);
  auto item_ids = ictx_.GetItemIdsInSourceOrder(record_decl);
  const clang::TypedefNameDecl* anon_typedef =
      record_decl->getTypedefNameForAnonDecl();
  auto record = Record{
      .rs_name = std::move(rs_name),
      .cc_name = std::move(cc_name),
      .mangled_cc_name = ictx_.GetMangledName(record_decl),
      .id = GenerateItemId(record_decl),
      .owning_target = ictx_.GetOwningTarget(record_decl),
      .doc_comment = std::move(doc_comment),
      .source_loc = ictx_.ConvertSourceLocation(source_loc),
      .unambiguous_public_bases = GetUnambiguousPublicBases(*record_decl),
      .fields = std::move(fields),
      .size = layout.getSize().getQuantity(),
      .original_cc_size = layout.getSize().getQuantity(),
      .alignment = layout.getAlignment().getQuantity(),
      .is_derived_class = is_derived_class,
      .override_alignment = override_alignment,
      .copy_constructor = GetCopyCtorSpecialMemberFunc(*record_decl),
      .move_constructor = GetMoveCtorSpecialMemberFunc(*record_decl),
      .destructor = GetDestructorSpecialMemberFunc(*record_decl),
      .is_trivial_abi = record_decl->canPassInRegisters(),
      .is_inheritable = !is_effectively_final,
      .is_abstract = record_decl->isAbstract(),
      .record_type = *record_type,
      .is_aggregate = record_decl->isAggregate(),
      .is_anon_record_with_typedef = anon_typedef != nullptr,
      .is_explicit_class_template_instantiation_definition =
          is_explicit_class_template_instantiation_definition,
      .child_item_ids = std::move(item_ids),
      .enclosing_namespace_id = GetEnclosingNamespaceId(record_decl),
  };

  // If the align attribute was attached to the typedef decl, we should
  // apply it to the generated record.
  //
  // TODO(jeanpierreda): We also need this logic for non-anonymous structs, where we
  // instead copy the struct into a new decl with this typedef's decl id.
  // So this part probably needs to be factored out somewhere that
  // typedef_name.cc can get at it.
  if (anon_typedef != nullptr) {
    auto* aligned = anon_typedef->getAttr<clang::AlignedAttr>();
    if (aligned) {
      record.alignment =
          ictx_.ctx_.toCharUnitsFromBits(aligned->getAlignment(ictx_.ctx_))
              .getQuantity();
      record.override_alignment = true;

      // If it has alignment, update the `record->size` to the aligned
      // one, because that size is going to be used as this record's
      // canonical size in IR and in the binding code.

      // Make sure that `alignment` is a power of 2.
      CHECK(!(record.alignment & (record.alignment - 1)));

      // Given that `alignment` is a power of 2, we can round it up by
      // a bit arithmetic: `alignment - 1` clears the single bit of it
      // while turning all the zeros in the right to 1s. Adding
      // `alignment - 1` and doing &~ with it effectively rounds it up
      // to the next multiple of the alignment.
      record.size =
          (record.size + record.alignment - 1) & ~(record.alignment - 1);
    }
  }
  return record;
}

std::vector<Field> CXXRecordDeclImporter::ImportFields(
    clang::CXXRecordDecl* record_decl) {
  clang::AccessSpecifier default_access =
      record_decl->isClass() ? clang::AS_private : clang::AS_public;
  std::vector<Field> fields;
  const clang::ASTRecordLayout& layout =
      ictx_.ctx_.getASTRecordLayout(record_decl);
  for (const clang::FieldDecl* field_decl : record_decl->fields()) {
    clang::AccessSpecifier access = field_decl->getAccess();
    if (access == clang::AS_none) {
      access = default_access;
    }

    std::optional<clang::tidy::lifetimes::ValueLifetimes> no_lifetimes;
    absl::StatusOr<MappedType> type;
    switch (access) {
      case clang::AS_public:
        // TODO(mboehme): Once lifetime_annotations supports retrieving
        // lifetimes in field types, pass these to ConvertQualType().
        type = ictx_.ConvertQualType(field_decl->getType(), no_lifetimes,
                                     std::nullopt);
        break;
      case clang::AS_protected:
      case clang::AS_private:
      case clang::AS_none:
        // As a performance optimization (i.e. to keep the generated code
        // small) we can emit private fields as opaque blobs of bytes.  This
        // may avoid the need to include supporting types in the generated
        // code (e.g. avoiding extra template instantiations).  See also
        // b/226580208 and <internal link>.
        type = absl::UnavailableError(
            "Types of non-public C++ fields can be elided away");
        break;
    }

    bool is_inheritable = false;
    auto* field_record = field_decl->getType()->getAsCXXRecordDecl();
    if (field_record) {
      // If it is a record as a direct member, its item must be already
      // imported.
      auto item = ictx_.GetImportedItem(field_record);
      if (item.has_value()) {
        if (const auto* record = std::get_if<Record>(&item.value())) {
          is_inheritable = record->is_inheritable;
        }
      }
    }

    fields.push_back(
        {.identifier = GetTranslatedFieldName(field_decl),
         .doc_comment = ictx_.GetComment(field_decl),
         .type = std::move(type),
         .access = TranslateAccessSpecifier(access),
         .offset = layout.getFieldOffset(field_decl->getFieldIndex()),
         .size = field_decl->isBitField()
                     ? field_decl->getBitWidthValue(ictx_.ctx_)
                     : ictx_.ctx_.getTypeSize(field_decl->getType()),
         .is_no_unique_address =
             field_decl->hasAttr<clang::NoUniqueAddressAttr>(),
         .is_bitfield = field_decl->isBitField(),
         .is_inheritable = is_inheritable});
  }
  return fields;
}

std::vector<BaseClass> CXXRecordDeclImporter::GetUnambiguousPublicBases(
    const clang::CXXRecordDecl& record_decl) const {
  // This function is unfortunate: the only way to correctly get information
  // about the bases is lookupInBases. It runs a complex O(N^3) algorithm for
  // e.g. correctly determining virtual base paths, etc.
  //
  // However, lookupInBases does not recurse into a class once it's found.
  // So we need to call lookupInBases once per class, making this O(N^4).

  llvm::SmallPtrSet<const clang::CXXRecordDecl*, 4> seen;
  std::vector<BaseClass> bases;
  clang::CXXBasePaths paths;
  // the const cast is a common pattern, apparently, see e.g.
  // https://clang.llvm.org/doxygen/CXXInheritance_8cpp_source.html#l00074
  paths.setOrigin(const_cast<clang::CXXRecordDecl*>(&record_decl));

  auto next_class = [&]() {
    const clang::CXXRecordDecl* found = nullptr;

    // Matches the first new class it encounters (and adds it to `seen`, so
    // that future runs don't rediscover it.)
    auto is_new_class = [&](const clang::CXXBaseSpecifier* base_specifier,
                            clang::CXXBasePath&) {
      const auto* record_decl = base_specifier->getType()->getAsCXXRecordDecl();
      if (found) {
        return record_decl == found;
      }

      if (record_decl && seen.insert(record_decl).second) {
        found = record_decl;
        return true;
      }
      return false;
    };
    return record_decl.lookupInBases(is_new_class, paths);
  };

  for (; next_class(); paths.clear()) {
    for (const clang::CXXBasePath& path : paths) {
      if (path.Access != clang::AS_public) {
        continue;
      }
      const clang::CXXBaseSpecifier& base_specifier =
          *path[path.size() - 1].Base;
      const clang::QualType& base = base_specifier.getType();
      if (paths.isAmbiguous(ictx_.ctx_.getCanonicalType(base))) {
        continue;
      }

      clang::CXXRecordDecl* base_record_decl =
          ABSL_DIE_IF_NULL(base_specifier.getType()->getAsCXXRecordDecl());
      if (!ictx_.HasBeenAlreadySuccessfullyImported(base_record_decl)) {
        continue;
      }

      std::optional<int64_t> offset = {0};
      for (const clang::CXXBasePathElement& base_path_element : path) {
        if (base_path_element.Base->isVirtual()) {
          offset.reset();
          break;
        }
        *offset +=
            {ictx_.ctx_.getASTRecordLayout(base_path_element.Class)
                 .getBaseClassOffset(ABSL_DIE_IF_NULL(
                     base_path_element.Base->getType()->getAsCXXRecordDecl()))
                 .getQuantity()};
      }
      CHECK((!offset.has_value() || *offset >= 0) &&
            "Concrete base classes should have non-negative offsets.");
      bases.push_back(
          BaseClass{.base_record_id = GenerateItemId(base_record_decl),
                    .offset = offset});
      break;
    }
  }
  return bases;
}

}  // namespace crubit

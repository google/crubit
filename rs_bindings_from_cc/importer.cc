// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importer.h"

#include <stdint.h>

#include <cassert>
#include <iterator>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <tuple>
#include <utility>
#include <variant>
#include <vector>

#include "absl/base/no_destructor.h"
#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/check.h"
#include "absl/log/log.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/cord.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_format.h"
#include "absl/strings/str_join.h"
#include "absl/strings/string_view.h"
#include "absl/strings/substitute.h"
#include "common/annotation_reader.h"
#include "common/status_macros.h"
#include "common/string_view_conversion.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "rs_bindings_from_cc/annotations_consumer.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/recording_diagnostic_consumer.h"
#include "rs_bindings_from_cc/type_map.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclFriend.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/DeclarationName.h"
#include "clang/AST/Mangle.h"
#include "clang/AST/PrettyPrinter.h"
#include "clang/AST/RawCommentList.h"
#include "clang/AST/Type.h"
#include "clang/Basic/AttrKinds.h"
#include "clang/Basic/Diagnostic.h"
#include "clang/Basic/FileManager.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/OperatorKinds.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Sema/Sema.h"
#include "llvm/ADT/STLExtras.h"
#include "llvm/Support/Casting.h"
#include "llvm/Support/FormatVariadic.h"
#include "llvm/Support/Regex.h"

namespace crubit {
namespace {

constexpr absl::string_view kTypeStatusPayloadUrl =
    "type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type";

// Checks if the return value from `GetDeclItem` indicates that the import was
// successful.
absl::Status CheckImportStatus(const std::optional<IR::Item>& item) {
  if (!item.has_value()) {
    return absl::InvalidArgumentError("The import has been skipped");
  }
  if (auto* unsupported = std::get_if<UnsupportedItem>(&*item)) {
    std::vector<absl::string_view> messages;
    messages.reserve(unsupported->errors.size());
    for (const auto& error : unsupported->errors) {
      messages.push_back(error.message());
    }
    return absl::InvalidArgumentError(absl::StrJoin(messages, "\n\n"));
  }
  return absl::OkStatus();
}

// Returns true if the comment is boilerplate that should be filtered out.
bool IsFilteredComment(const clang::SourceManager& sm,
                       const clang::RawComment& comment) {
  static absl::NoDestructor<llvm::Regex> kHeaderGuard("^// [A-Z_]*_H_ *$");
  if (kHeaderGuard->match(comment.getRawText(sm))) {
    return true;
  }

  // This one is a special case -- inside Crubit, we use a boilerplate license
  // header at the top of all files. It's added to the top of the file of both
  // the input and the output, and so we don't need to _repeat_ the version
  // that originated in the input.
  if (comment.getRawText(sm) ==
      "// Part of the Crubit project, under the Apache License v2.0 with "
      "LLVM\n// Exceptions. See /LICENSE for license information.\n// "
      "SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception") {
    return true;
  }
  return false;
}

bool IsBuiltinFunction(const clang::Decl* decl) {
  const auto* function = clang::dyn_cast<clang::FunctionDecl>(decl);
  if (function == nullptr) {
    return false;
  }
  return function->getBuiltinID() != 0;
}

}  // namespace

namespace {

// Reduces a clang::CallingConv into a crubit::CallingConv, which is a subset.
// If the variant isn't in the subset, returns an error.
absl::StatusOr<CallingConv> ConvertCcCallConvToSupportedCallingConv(
    clang::CallingConv cc_call_conv) {
  switch (cc_call_conv) {
    case clang::CC_C:  // __attribute__((cdecl))
      return CallingConv::kC;
    case clang::CC_X86FastCall:  // __attribute__((fastcall))
      return CallingConv::kX86FastCall;
    case clang::CC_X86VectorCall:  // __attribute__((vectorcall))
      return CallingConv::kX86VectorCall;
    case clang::CC_X86ThisCall:  // __attribute__((thiscall))
      return CallingConv::kX864ThisCall;
    case clang::CC_X86StdCall:  // __attribute__((stdcall))
      return CallingConv::kX86StdCall;
    case clang::CC_Win64:  // __attribute__((ms_abi))
      return CallingConv::kWin64;
    case clang::CC_AAPCS:      // __attribute__((pcs("aapcs")))
    case clang::CC_AAPCS_VFP:  // __attribute__((pcs("aapcs-vfp")))
      // TODO(lukasza): Should both map to "aapcs"?
      break;
    case clang::CC_X86_64SysV:  // __attribute__((sysv_abi))
      // TODO(lukasza): Maybe this is "sysv64"?
      break;
    case clang::CC_X86Pascal:     // __attribute__((pascal))
    case clang::CC_X86RegCall:    // __attribute__((regcall))
    case clang::CC_IntelOclBicc:  // __attribute__((intel_ocl_bicc))
    case clang::CC_SpirFunction:  // default for OpenCL functions on SPIR target
    case clang::CC_DeviceKernel:  // __attribute__((device_kernel))
    case clang::CC_Swift:         // __attribute__((swiftcall))
    case clang::CC_SwiftAsync:    // __attribute__((swiftasynccall))
    case clang::CC_PreserveMost:  // __attribute__((preserve_most))
    case clang::CC_PreserveAll:   // __attribute__((preserve_all))
    case clang::CC_AArch64VectorCall:  // __attribute__((aarch64_vector_pcs))
      // TODO(hlopko): Uncomment once we integrate the upstream change that
      // introduced it:
      // case clang::CC_AArch64SVEPCS: __attribute__((aarch64_sve_pcs))

      // These don't seem to have any Rust equivalents.
      break;
    default:
      break;
  }
  return absl::UnimplementedError(
      absl::StrCat("Unsupported calling convention: ",
                   StringViewFromStringRef(
                       clang::FunctionType::getNameForCallConv(cc_call_conv))));
}

}  // namespace

// Multiple IR items can be associated with the same source location (e.g. the
// implicitly defined constructors and assignment operators). To produce
// deterministic output, we order such items based on GetDeclOrder.  The order
// is somewhat arbitrary, but we still try to make it aesthetically pleasing
// (e.g. constructors go before assignment operators;  default constructor goes
// first, etc.).
static int GetDeclOrder(const clang::Decl* decl) {
  if (clang::isa<clang::RecordDecl>(decl)) {
    return decl->getDeclContext()->isRecord() ? 101 : 100;
  }

  if (auto* ctor = clang::dyn_cast<clang::CXXConstructorDecl>(decl)) {
    return ctor->isDefaultConstructor() ? 202
           : ctor->isCopyConstructor()  ? 203
           : ctor->isMoveConstructor()  ? 204
                                        : 299;
  }

  if (clang::isa<clang::CXXDestructorDecl>(decl)) {
    return 306;
  }

  if (auto* method = clang::dyn_cast<clang::CXXMethodDecl>(decl)) {
    return method->isCopyAssignmentOperator()   ? 401
           : method->isMoveAssignmentOperator() ? 402
                                                : 499;
  }

  return 999;
}

class Importer::SourceOrderKey {
 public:
  explicit SourceOrderKey(clang::SourceRange source_range, int decl_order = 0,
                          std::string name = "")
      : source_range_(source_range), decl_order_(decl_order), name_(name) {}

  SourceOrderKey(const SourceOrderKey&) = default;
  SourceOrderKey& operator=(const SourceOrderKey&) = default;

  bool isBefore(const SourceOrderKey& other,
                const clang::SourceManager& sm) const {
    if (!source_range_.isValid() || !other.source_range_.isValid()) {
      if (source_range_.isValid() != other.source_range_.isValid())
        return !source_range_.isValid() && other.source_range_.isValid();
    } else {
      if (source_range_.getBegin() != other.source_range_.getBegin()) {
        return sm.isBeforeInTranslationUnit(source_range_.getBegin(),
                                            other.source_range_.getBegin());
      }
      if (source_range_.getEnd() != other.source_range_.getEnd()) {
        return sm.isBeforeInTranslationUnit(source_range_.getEnd(),
                                            other.source_range_.getEnd());
      }
    }

    if (decl_order_ < other.decl_order_) {
      return true;
    } else if (decl_order_ > other.decl_order_) {
      return false;
    }
    return name_ < other.name_;
  }

 private:
  clang::SourceRange source_range_;
  int decl_order_;
  std::string name_;
};

Importer::SourceOrderKey Importer::GetSourceOrderKey(
    const clang::Decl* decl) const {
  return SourceOrderKey(decl->getSourceRange(), GetDeclOrder(decl),
                        GetNameForSourceOrder(decl));
}

Importer::SourceOrderKey Importer::GetSourceOrderKey(
    const clang::RawComment* comment) const {
  return SourceOrderKey(comment->getSourceRange());
}

class Importer::SourceLocationComparator {
 public:
  bool operator()(const clang::SourceLocation& a,
                  const clang::SourceLocation& b) const {
    return b.isValid() && a.isValid() && sm_.isBeforeInTranslationUnit(a, b);
  }
  bool operator()(const clang::RawComment* a,
                  const clang::SourceLocation& b) const {
    return this->operator()(a->getBeginLoc(), b);
  }
  bool operator()(const clang::SourceLocation& a,
                  const clang::RawComment* b) const {
    return this->operator()(a, b->getBeginLoc());
  }
  bool operator()(const clang::RawComment* a,
                  const clang::RawComment* b) const {
    return this->operator()(a->getBeginLoc(), b->getBeginLoc());
  }

  using OrderedItemId = std::pair<SourceOrderKey, ItemId>;
  using OrderedItem = std::pair<SourceOrderKey, IR::Item>;

  template <typename OrderedItemOrId>
  bool operator()(const OrderedItemOrId& a, const OrderedItemOrId& b) const {
    auto a_source_order = a.first;
    auto b_source_order = b.first;
    return a_source_order.isBefore(b_source_order, sm_);
  }
  explicit SourceLocationComparator(const clang::SourceManager& sm) : sm_(sm) {}

 private:
  const clang::SourceManager& sm_;
};

std::vector<clang::Decl*> Importer::GetCanonicalChildren(
    const clang::DeclContext* decl_context) const {
  std::vector<clang::Decl*> result;
  for (clang::Decl* decl : decl_context->decls()) {
    if (const auto* linkage_spec_decl =
            llvm::dyn_cast<clang::LinkageSpecDecl>(decl)) {
      llvm::move(GetCanonicalChildren(linkage_spec_decl),
                 std::back_inserter(result));
      continue;
    }

    // `CXXRecordDeclImporter::Import` supports class template specializations
    // but such import should only be triggered when
    // `Importer::ConvertTemplateSpecializationType` is called (which means that
    // the specialization is actually used in an explicit instantiation via
    // `cc_template!` macro, in a type alias, or as a parameter type of a
    // function, etc.).
    if (clang::isa<clang::ClassTemplateSpecializationDecl>(decl)) continue;

    // In general we only import (and include as children) canonical decls.
    // Namespaces are exempted to ensure that we process every one of
    // (potential) multiple namespace blocks with the same name.
    // CXXRecordDecls are exempted because we use the _definition_, not
    // the "canonical" decl (which may be a forward declaration).
    if (clang::Decl* canonical_decl = CanonicalizeDecl(decl);
        canonical_decl == decl) {
      result.push_back(decl);
    }
  }
  return result;
}

const clang::Decl* Importer::CanonicalizeDecl(const clang::Decl* decl) const {
  if (auto* namespace_decl = llvm::dyn_cast<clang::NamespaceDecl>(decl)) {
    return namespace_decl;
  }
  auto is_injected_class_name = [](const clang::Decl* decl) {
    if (auto* cxx_record_decl = llvm::dyn_cast<clang::CXXRecordDecl>(decl)) {
      return cxx_record_decl->isInjectedClassName();
    }
    return false;
  };
  if (llvm::isa<clang::TagDecl>(decl)) {
    if (is_injected_class_name(decl)) {
      return nullptr;
    }
    auto owning_target = GetOwningTarget(decl);
    const auto& source_manager = sema_.getSourceManager();
    clang::Decl* canonical = nullptr;
    for (auto iter = decl->redecls_begin(); iter != decl->redecls_end();
         ++iter) {
      auto* redecl = llvm::dyn_cast<clang::TagDecl>(*iter);
      CHECK(redecl);
      if (is_injected_class_name(redecl)) {
        continue;
      }
      if (GetOwningTarget(redecl) != owning_target) {
        continue;
      }
      if (redecl->isThisDeclarationADefinition()) {
        canonical = redecl;
        break;  // Multiple definitions are not allowed.
      }
      if (!canonical) {
        canonical = redecl;
        continue;
      }
      if (source_manager.isBeforeInTranslationUnit(
              redecl->getSourceRange().getBegin(),
              canonical->getSourceRange().getBegin())) {
        canonical = redecl;
      }
    }
    CHECK(canonical != nullptr);
    return canonical;
  }
  return decl->getCanonicalDecl();
}

clang::Decl* Importer::CanonicalizeDecl(clang::Decl* decl) const {
  const clang::Decl* decl_const = const_cast<clang::Decl*>(decl);
  const clang::Decl* ret = CanonicalizeDecl(decl_const);
  return const_cast<clang::Decl*>(ret);
}

ItemId Importer::GenerateItemId(const clang::Decl* decl) const {
  const clang::Decl* canonicalized = CanonicalizeDecl(decl);
  return ItemId(reinterpret_cast<uintptr_t>(canonicalized));
}

bool Importer::IsUnsupportedAndAlien(ItemId item_id) const {
  auto it = import_cache_.find(reinterpret_cast<clang::Decl*>(item_id.value()));
  return it != import_cache_.end() && it->second.has_value() &&
         std::holds_alternative<UnsupportedItem>(*it->second) &&
         !IsFromCurrentTarget(it->first);
}

ItemId Importer::GenerateItemId(const clang::RawComment* comment) const {
  return ItemId(reinterpret_cast<uintptr_t>(comment));
}

absl::StatusOr<std::optional<ItemId>> Importer::GetEnclosingItemId(
    clang::Decl* decl) {
  for (clang::DeclContext* decl_context = decl->getDeclContext();;
       decl_context = decl_context->getParent()) {
    if (decl_context->isTranslationUnit()) {
      return std::nullopt;
    }
    // Class template specializations are always emitted in the top-level
    // namespace.  See also Importer::GetOrderedItemIdsOfTemplateInstantiations.
    if (clang::isa<clang::ClassTemplateSpecializationDecl>(decl))
      return std::nullopt;

    if (decl_context->isFunctionOrMethod()) {
      return std::nullopt;
    }
    if (auto* record_decl = clang::dyn_cast<clang::RecordDecl>(decl_context)) {
      if (!EnsureSuccessfullyImported(record_decl)) {
        return absl::InvalidArgumentError("Couldn't import the parent");
      }
      return GenerateItemId(record_decl);
    }
    if (auto* namespace_decl =
            clang::dyn_cast<clang::NamespaceDecl>(decl_context)) {
      return GenerateItemId(namespace_decl);
    }
  }
}

Importer::DeclItems Importer::GetDeclItems(const clang::Decl* decl) {
  DeclItems decl_items;
  clang::SourceManager& sm = ctx_.getSourceManager();
  auto compare_locations = SourceLocationComparator(sm);

  // We are only interested in comments within this decl context.
  std::vector<const clang::RawComment*> comments_in_range(
      llvm::lower_bound(comments_, decl->getBeginLoc(), compare_locations),
      llvm::upper_bound(comments_, decl->getEndLoc(), compare_locations));

  std::map<clang::SourceLocation, const clang::RawComment*,
           SourceLocationComparator>
      ordered_comments(compare_locations);
  for (auto& comment : comments_in_range) {
    if (IsFilteredComment(sm, *comment)) continue;
    ordered_comments.insert({comment->getBeginLoc(), comment});
  }

  absl::flat_hash_set<ItemId> visited_item_ids;

  auto* decl_context = clang::cast<clang::DeclContext>(decl);
  for (auto decl : GetCanonicalChildren(decl_context)) {
    // Only add item ids for decls that can be successfully imported.
    if (auto item = GetDeclItem(decl); item.has_value()) {
      auto item_id = GenerateItemId(decl);
      // TODO(rosica): Drop this check when we start importing also other
      // redecls, not just the canonical
      if (visited_item_ids.find(item_id) == visited_item_ids.end()) {
        visited_item_ids.insert(item_id);
        decl_items.canonical_children.push_back({decl, item_id});
      }
    }

    // We remove comments attached to a child decl or that are within a child
    // decl.
    if (auto raw_comment = ctx_.getRawCommentForDeclNoCache(decl)) {
      ordered_comments.erase(raw_comment->getBeginLoc());
    }
    if (decl->getLocation().isValid()) {
      ordered_comments.erase(ordered_comments.lower_bound(decl->getBeginLoc()),
                             ordered_comments.upper_bound(decl->getEndLoc()));
    }
  }

  decl_items.comments.reserve(ordered_comments.size());
  for (auto& [_, comment] : ordered_comments) {
    decl_items.comments.push_back(comment);
  }
  return decl_items;
}

absl::flat_hash_map<BazelLabel, std::vector<ItemId>>
Importer::GetTopLevelItemIdsInSourceOrder(
    const clang::TranslationUnitDecl* translation_unit_decl) {
  Importer::DeclItems decl_items = GetDeclItems(translation_unit_decl);

  absl::flat_hash_map<BazelLabel,
                      std::vector<SourceLocationComparator::OrderedItemId>>
      items;

  // Push the comments first
  std::vector<SourceLocationComparator::OrderedItemId>& invocation_items =
      items[invocation_.target_];
  invocation_items.reserve(decl_items.comments.size());
  for (auto& comment : decl_items.comments) {
    invocation_items.push_back(
        {GetSourceOrderKey(comment), GenerateItemId(comment)});
  }

  // Push all the other items
  for (auto& [decl, item_id] : decl_items.canonical_children) {
    items[GetOwningTarget(decl)].push_back({GetSourceOrderKey(decl), item_id});
  }

  clang::SourceManager& sm = ctx_.getSourceManager();
  auto compare_locations = SourceLocationComparator(sm);

  absl::flat_hash_map<BazelLabel, std::vector<ItemId>> ordered_items;
  for (auto& [target, item_ids_in_target] : items) {
    llvm::sort(item_ids_in_target, compare_locations);

    std::vector<ItemId>& ordered_item_ids = ordered_items[target];
    ordered_item_ids.reserve(item_ids_in_target.size());
    for (auto& ordered_item : item_ids_in_target) {
      ordered_item_ids.push_back(ordered_item.second);
    }
  }
  return ordered_items;
}

std::vector<ItemId> Importer::GetItemIdsInSourceOrder(
    clang::Decl* parent_decl) {
  Importer::DeclItems decl_items = GetDeclItems(parent_decl);

  std::vector<SourceLocationComparator::OrderedItemId> items;

  items.reserve(decl_items.comments.size() +
                decl_items.canonical_children.size());
  for (auto& comment : decl_items.comments) {
    items.push_back({GetSourceOrderKey(comment), GenerateItemId(comment)});
  }
  for (auto& [decl, item_id] : decl_items.canonical_children) {
    items.push_back({GetSourceOrderKey(decl), item_id});
  }

  clang::SourceManager& sm = ctx_.getSourceManager();
  auto compare_locations = SourceLocationComparator(sm);

  llvm::sort(items, compare_locations);
  std::vector<ItemId> ordered_item_ids;

  ordered_item_ids.reserve(items.size());
  for (auto& ordered_item : items) {
    ordered_item_ids.push_back(ordered_item.second);
  }
  return ordered_item_ids;
}

std::vector<ItemId> Importer::GetOrderedItemIdsOfTemplateInstantiations()
    const {
  std::vector<SourceLocationComparator::OrderedItemId> items;
  items.reserve(class_template_instantiations_.size());
  for (const auto* decl : class_template_instantiations_) {
    items.push_back({GetSourceOrderKey(decl), GenerateItemId(decl)});
  }

  clang::SourceManager& sm = ctx_.getSourceManager();
  auto compare_locations = SourceLocationComparator(sm);
  llvm::sort(items, compare_locations);

  std::vector<ItemId> ordered_item_ids;
  ordered_item_ids.reserve(items.size());
  for (const auto& ordered_item : items) {
    ordered_item_ids.push_back(ordered_item.second);
  }
  return ordered_item_ids;
}

void Importer::ImportFreeComments() {
  clang::SourceManager& sm = ctx_.getSourceManager();
  for (const auto& header : invocation_.public_headers_) {
    if (auto file = sm.getFileManager().getFileRef(
            StringRefFromStringView(header.IncludePath()))) {
      if (auto comments_in_file = ctx_.Comments.getCommentsInFile(
              sm.getOrCreateFileID(*file, clang::SrcMgr::C_User))) {
        for (const auto& [_, comment] : *comments_in_file) {
          comments_.push_back(comment);
        }
      }
    }
  }
  llvm::sort(comments_, SourceLocationComparator(sm));
}

void Importer::Import(clang::TranslationUnitDecl* translation_unit_decl) {
  ImportFreeComments();
  clang::SourceManager& sm = ctx_.getSourceManager();
  std::vector<SourceLocationComparator::OrderedItem> ordered_items;

  ordered_items.reserve(comments_.size());
  for (auto& comment : comments_) {
    ordered_items.push_back(
        {GetSourceOrderKey(comment),
         Comment{.text = comment->getFormattedText(sm, sm.getDiagnostics()),
                 .id = GenerateItemId(comment)}});
  }

  ImportDeclsFromDeclContext(translation_unit_decl);
  for (const auto& [decl, item] : import_cache_) {
    if (!item.has_value() || IsUnsupportedAndAlien(GenerateItemId(decl))) {
      continue;
    }
    ordered_items.push_back({GetSourceOrderKey(decl), *item});
  }

  llvm::sort(ordered_items, SourceLocationComparator(sm));

  invocation_.ir_.items.reserve(ordered_items.size());
  for (auto& ordered_item : ordered_items) {
    invocation_.ir_.items.push_back(ordered_item.second);
  }
  invocation_.ir_.top_level_item_ids =
      GetTopLevelItemIdsInSourceOrder(translation_unit_decl);

  // TODO(b/257302656): Consider placing the generated template instantiations
  // into a separate namespace (maybe `crubit::instantiated_templates` ?).
  llvm::copy(GetOrderedItemIdsOfTemplateInstantiations(),
             std::back_inserter(
                 invocation_.ir_.top_level_item_ids[invocation_.target_]));
}

void Importer::ImportDeclsFromDeclContext(
    const clang::DeclContext* decl_context) {
  for (auto decl : GetCanonicalChildren(decl_context)) {
    GetDeclItem(decl);
  }
}

std::optional<IR::Item> Importer::GetDeclItem(clang::Decl* decl) {
  // TODO(jeanpierreda): Move `decl->getCanonicalDecl()` from callers into here.
  if (auto it = import_cache_.find(decl); it != import_cache_.end()) {
    return it->second;
  }
  // Here, we need to be careful. Recursive imports break cycles as follows:
  // an item which may, in the process of being imported, then import itself,
  // will mark itself as being successfully imported in the future via
  // `MarkAsSuccessfullyImported()` during its own import process. Then later
  // attempts to import it will, instead of trying to import it again (causing
  // an infinite loop), short-circuit and return a null item at that time.
  //
  // This means that import_cache_ can change *during the call to ImportDecl*,
  // and in particular, because `GetDeclItem` caches, and because recursive
  // calls return null, it might specifically have been changed to have a
  // null entry for the decl we are currently importing.
  //
  // For example, consider the following type:
  //
  // ```c++
  // struct Foo{ Foo* x; }
  // ```
  //
  // 1. First, we call `GetDeclItem(mystruct)`
  // 2. If importing `x` itself attempts an import of `Foo*`, then that would
  //    call `GetDeclItem(mystruct)` inside of an existing call to
  //    `GetDeclItem(mystruct)`.
  // 3. The nested call returns early, returning null (because this is a
  //    cyclic invocation), and `GetDeclItem` **caches the null entry**.
  // 4. finally, the original `GetDeclItem` call finishes its call to
  //    `ImportDecl`. It must now overwrite the nulled cache entry from the
  //    earlier import to instead use the real entry.
  //
  // TODO(jeanpierreda): find and eliminate all re-entrant imports, and replace with
  // a CHECK(inserted).

  // Note: insert_or_assign, not insert, in case a record, so as to overwrite
  // any null entries introduced by cycles.

  std::optional<IR::Item> result = ImportDecl(decl);
  auto [it, inserted] = import_cache_.try_emplace(decl, result);
  if (!inserted) {
    // TODO(jeanpierreda): Fix and promote to CHECK.
    // At least one cycle occurs with Typedef, where a typedef will import
    // itself during its own import. This isn't an infinite loop, because the
    // recursive cycle gets broken between the two by CXXRecordDecl, but the
    // result is that we get this typedef inserted while we were attempting to
    // insert it.
    //
    // Alternatively, maybe it's sufficient to check that they're _equal_.
    // It's not a bug at all to import it twice if it has no effect.
    LOG_IF(INFO, !it->second.has_value())
        << "re-entrant import discovered, where the re-entrant import had a "
           "non-null value."
        << "\n  trying to import a " << decl->getDeclKindName()
        << "\n  present entry: " << ItemToString(it->second)
        << "\n  was going to be inserted: " << ItemToString(result);
    it->second = result;
  }
  if (auto* record_decl = clang::dyn_cast<clang::CXXRecordDecl>(decl)) {
    // TODO(forster): Should we even visit the nested decl if we couldn't
    // import the parent? For now we have tests that check that we generate
    // error messages for those decls, so we're visiting.
    ImportDeclsFromDeclContext(record_decl);
  }

  // Logic for `ClassTemplateSpecializationDecl`: insert them into
  // `class_template_instantiations_`, so that they will get included in
  // IR::top_level_item_ids.
  // Note: The 'gating' logic here needs to be consistent with the 'gating'
  // logic in `GetItemIdsInSourceOrder`, or else the class template
  // instantiation decl ID is inserted into the IR::top_level_item_ids but does
  // not have its corresponding IR item, resulting in lookup failures (crashes)
  // when generating bindings.
  if (result.has_value()) {
    if (auto* specialization_decl =
            llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(decl);
        specialization_decl && IsFromCurrentTarget(specialization_decl)) {
      class_template_instantiations_.insert(specialization_decl);
    }
  }
  return result;
}

/// Returns true if a decl is inside a private section, or is inside a
/// RecordDecl which is IsTransitivelyInPrivate.
bool IsTransitivelyInPrivate(clang::Decl* decl_to_check) {
  while (true) {
    auto* parent =
        llvm::dyn_cast<clang::CXXRecordDecl>(decl_to_check->getDeclContext());
    if (parent == nullptr) {
      return false;
    }
    switch (decl_to_check->getAccess()) {
      case clang::AccessSpecifier::AS_public:
        break;
      case clang::AccessSpecifier::AS_none:
        if (!parent->isClass()) {
          break;
        }
        [[fallthrough]];
      case clang::AccessSpecifier::AS_private:
      case clang::AccessSpecifier::AS_protected:
        return true;
    }

    decl_to_check = parent;
  }
}

std::optional<IR::Item> Importer::ImportDecl(clang::Decl* decl) {
  if (IsTransitivelyInPrivate(decl)) return std::nullopt;

  const absl::StatusOr<bool> must_bind =
      HasAnnotationWithoutArgs(*decl, "crubit_must_bind");
  if (!must_bind.ok()) {
    return HardError(*decl, FormattedError::FromStatus(must_bind.status()));
  }

  const absl::StatusOr<bool> do_not_bind =
      HasAnnotationWithoutArgs(*decl, "crubit_do_not_bind");
  if (!do_not_bind.ok()) {
    return HardError(*decl, FormattedError::FromStatus(do_not_bind.status()));
  }
  if (*do_not_bind) {
    const std::optional<absl::flat_hash_set<std::string>>&
        do_not_bind_allowlist = invocation_.do_not_bind_allowlist_;
    const clang::NamedDecl* named_decl =
        clang::dyn_cast<clang::NamedDecl>(decl);
    // Function declarations do not need to be allowlisted.
    if (named_decl && !clang::isa<clang::FunctionDecl>(decl) &&
        do_not_bind_allowlist.has_value()) {
      std::string decl_name = named_decl->getQualifiedNameAsString();
      if (!do_not_bind_allowlist->contains(decl_name)) {
        return HardError(
            *decl, FormattedError::PrefixedStrCat(
                       "CRUBIT_DO_NOT_BIND annotation on non-allowlisted decl",
                       std::move(decl_name),
                       "\nOmitted bindings must be pre-registered using "
                       "`do_not_bind_allowlist`"));
      }
    }
    return std::nullopt;
  }

  std::string unavailable_error;
  if (decl->isUnavailable(&unavailable_error)) {
    return ImportUnsupportedItem(
        *decl, std::nullopt,
        {FormattedError::PrefixedStrCat("Decl is unavailable: ",
                                        std::move(unavailable_error))},
        /*is_hard_error=*/*must_bind);
  }

  for (auto& importer : decl_importers_) {
    std::optional<IR::Item> result = importer->ImportDecl(decl);
    if (result.has_value()) {
      if (*must_bind) {
        SetMustBindItem(*result);
      }
      return result;
    }
  }

  if (*must_bind) {
    return HardError(
        *decl,
        FormattedError::Static(
            "No importer found for decl with CRUBIT_MUST_BIND annotation"));
  }
  return std::nullopt;
}

std::optional<IR::Item> Importer::GetImportedItem(
    const clang::Decl* decl) const {
  auto it = import_cache_.find(decl);
  if (it != import_cache_.end()) {
    return it->second;
  }
  return std::nullopt;
}

BazelLabel Importer::GetOwningTarget(const clang::Decl* decl) const {
  // Template instantiations need to be generated in the target that triggered
  // the instantiation (not in the target where the template is defined).
  if (IsFullClassTemplateSpecializationOrChild(decl)) {
    return invocation_.target_;
  }

  // Built-in functions are defined by the compiler and are not associated with
  // any target. Without this check, the decl item ID will show up in the IR
  // as the ID of a top-level item, but the function itself will be missing from
  // the IR's list of items, resulting in a crash when generating bindings.
  if (IsBuiltinFunction(decl)) {
    return BazelLabel("//:virtual_clang_resource_dir_target");
  }

  clang::SourceManager& source_manager = ctx_.getSourceManager();
  auto source_location = decl->getLocation();

  // If the header this decl comes from is not associated with a target,
  // and appears to be a textual header, then we go up the include stack
  // until we find a header that has an owning target.

  std::optional<llvm::StringRef> filename;

  while (source_location.isValid()) {
    if (source_location.isMacroID()) {
      source_location = source_manager.getExpansionLoc(source_location);
    }
    auto id = source_manager.getFileID(source_location);
    filename = source_manager.getNonBuiltinFilenameForID(id);
    if (!filename) {
      return BazelLabel("//:_nothing_should_depend_on_private_builtin_hdrs");
    }
    if (filename->starts_with("./")) {
      filename = filename->substr(2);
    }

    if (auto target = invocation_.header_target(HeaderName(filename->str()))) {
      return *target;
    }

    if (!filename->ends_with(".inc") && !filename->ends_with(".rs.h")) {
      // .inc files and (cxx-generated) .rs.h files are textual headers.
      // Otherwise, we assume it is not a textual header.
      // TODO(b/438538035): Instead, mark .rs.h files as textual headers in
      // the C++ library.
      // TODO(b/438560038): Make this configurable by flag.
      break;
    }

    source_location = source_manager.getIncludeLoc(id);
  }

  // If we get here, we couldn't find a bazel target that claims ownership of
  // this header. For example, no targets enabling crubit contained the header.
  // In that case, we use the filename as part of the target name, so that
  // we can use it in error messages and have a useful action item for users.
  // (For instance, to tell them to enable Crubit for that file!)
  if (!filename) filename = "<unknown>";
  return BazelLabel(
      absl::StrCat("//_unknown_target:", absl::string_view(*filename)));
}

bool Importer::IsFromCurrentTarget(const clang::Decl* decl) const {
  return invocation_.target_ == GetOwningTarget(decl);
}

bool Importer::IsFromProtoTarget(const clang::Decl& decl) const {
  // TODO(b/b/441343672): This is probably not a good way to detect if something
  // is from a proto target, and we should do something more durable.
  clang::SourceManager& source_manager = ctx_.getSourceManager();
  std::optional<llvm::StringRef> filename =
      source_manager.getNonBuiltinFilenameForID(
          source_manager.getFileID(decl.getLocation()));
  return filename.has_value() && filename->ends_with(".proto.h");
}

IR::Item Importer::HardError(const clang::Decl& decl, FormattedError error) {
  return ImportUnsupportedItem(decl, std::nullopt, {error},
                               /*is_hard_error=*/true);
}

IR::Item Importer::ImportUnsupportedItem(
    const clang::Decl& decl, std::optional<UnsupportedItem::Path> path,
    std::vector<FormattedError> errors) {
  return ImportUnsupportedItem(decl, std::move(path), std::move(errors),
                               /*is_hard_error=*/false);
}

IR::Item Importer::ImportUnsupportedItem(
    const clang::Decl& decl, std::optional<UnsupportedItem::Path> path,
    FormattedError error) {
  return ImportUnsupportedItem(decl, std::move(path),
                               std::vector<FormattedError>({std::move(error)}),
                               /*is_hard_error=*/false);
}

IR::Item Importer::ImportUnsupportedItem(
    const clang::Decl& decl, std::optional<UnsupportedItem::Path> path,
    std::vector<FormattedError> errors, bool is_hard_error) {
  auto kind = UnsupportedItem::Kind::kOther;
  if (const clang::TagDecl* named_decl =
          clang::dyn_cast<clang::TagDecl>(&decl)) {
    switch (named_decl->getTagKind()) {
      case clang::TagTypeKind::Struct:
        kind = UnsupportedItem::Kind::kStruct;
        break;
      case clang::TagTypeKind::Class:
        kind = UnsupportedItem::Kind::kClass;
        break;
      case clang::TagTypeKind::Enum:
        kind = UnsupportedItem::Kind::kEnum;
        break;
      case clang::TagTypeKind::Union:
        kind = UnsupportedItem::Kind::kUnion;
        break;
      default:
        break;
    }
  } else if (const clang::FunctionDecl* func_decl =
                 clang::dyn_cast<clang::FunctionDecl>(&decl)) {
    kind = func_decl->getKind() == clang::NamedDecl::Kind::CXXConstructor
               ? UnsupportedItem::Kind::kConstructor
               : UnsupportedItem::Kind::kFunc;
  } else if (clang::isa<clang::NamespaceDecl>(decl)) {
    kind = UnsupportedItem::Kind::kNamespace;
  } else if (clang::isa<clang::VarDecl>(decl)) {
    kind = UnsupportedItem::Kind::kGlobalVar;
  } else if (clang::isa<clang::FunctionTemplateDecl>(decl)) {
    kind = UnsupportedItem::Kind::kFunc;
  } else if (clang::isa<clang::ClassTemplateDecl>(decl)) {
    kind = UnsupportedItem::Kind::kClass;
  } else if (clang::isa<clang::TypeAliasDecl>(decl) ||
             clang::isa<clang::TypedefNameDecl>(decl)) {
    kind = UnsupportedItem::Kind::kTypeAlias;
  }
  std::string name = "unnamed";
  if (const auto* named_decl = clang::dyn_cast<clang::NamedDecl>(&decl)) {
    name = named_decl->getQualifiedNameAsString();
  }
  std::string source_loc = ConvertSourceLocation(decl.getBeginLoc());
  return UnsupportedItem{
      .name = name,
      .kind = kind,
      .path = std::move(path),
      .errors = std::move(errors),
      .source_loc = source_loc,
      .id = GenerateItemId(&decl),
      .must_bind = is_hard_error,
  };
}

static bool ShouldKeepCommentLine(absl::string_view line) {
  // Based on https://clang.llvm.org/extra/clang-tidy/:
  llvm::Regex patterns_to_ignore(
      "^[[:space:]/]*"  // Whitespace, or extra //
      "(NOLINT|NOLINTNEXTLINE|NOLINTBEGIN|NOLINTEND)"
      "(\\([^)[:space:]]*\\)?)?"  // Optional (...)
      "[[:space:]]*$");           // Whitespace
  return !patterns_to_ignore.match(StringRefFromStringView(line));
}

std::optional<std::string> Importer::GetComment(const clang::Decl* decl) const {
  // This does currently not distinguish between different types of comments.
  // In general it is not possible in C++ to reliably only extract doc comments.
  // This is going to be a heuristic that needs to be tuned over time.

  clang::SourceManager& sm = ctx_.getSourceManager();
  clang::RawComment* raw_comment = ctx_.getRawCommentForDeclNoCache(decl);

  if (raw_comment == nullptr) {
    return {};
  }

  std::string raw_comment_text =
      raw_comment->getFormattedText(sm, sm.getDiagnostics());
  std::string cleaned_comment_text = absl::StrJoin(
      absl::StrSplit(raw_comment_text, '\n', ShouldKeepCommentLine), "\n");
  if (cleaned_comment_text.empty()) return {};
  return cleaned_comment_text;
}

std::string Importer::ConvertSourceLocation(clang::SourceLocation loc) const {
  auto& sm = ctx_.getSourceManager();
  // For macros: https://clang.llvm.org/doxygen/SourceManager_8h.html:
  // Spelling location: where the macro is originally defined.
  // Expansion location: where the macro is expanded.
  const clang::SourceLocation& spelling_loc = sm.getSpellingLoc(loc);
  // TODO(b/261185414): The path format should probably come from a command
  // line argument.
  // TODO(b/261185414): Consider linking to the symbol instead of to the line
  // number to avoid wrong links while generated files have not caught up.
  constexpr absl::string_view kGeneratedFrom = "Generated from";
  constexpr absl::string_view kExpandedAt = "Expanded at";
  constexpr auto kSourceLocationFunc =
      [](absl::string_view origin, absl::string_view filename, uint32_t line) {
        return absl::Substitute("$0: $1;l=$2", origin, filename, line);
      };
  constexpr absl::string_view kSourceLocUnknown = "<unknown location>";
  std::string spelling_loc_str;
  if (absl::string_view spelling_filename =
          StringViewFromStringRef(sm.getFilename(spelling_loc));
      spelling_filename.empty()) {
    spelling_loc_str = kSourceLocUnknown;
  } else {
    uint32_t spelling_line = sm.getSpellingLineNumber(loc);
    if (absl::StartsWith(spelling_filename, "./")) {
      spelling_filename = spelling_filename.substr(2);
    }
    spelling_loc_str =
        kSourceLocationFunc(kGeneratedFrom, spelling_filename, spelling_line);
  }
  if (!loc.isMacroID()) {
    return spelling_loc_str;
  }
  const clang::SourceLocation& expansion_loc = sm.getExpansionLoc(loc);
  std::string expansion_loc_str;
  if (absl::string_view expansion_filename =
          StringViewFromStringRef(sm.getFilename(expansion_loc));
      expansion_filename.empty()) {
    expansion_loc_str = kSourceLocUnknown;
  } else {
    uint32_t expansion_line = sm.getExpansionLineNumber(loc);
    if (absl::StartsWith(expansion_filename, "./")) {
      expansion_filename = expansion_filename.substr(2);
    }
    expansion_loc_str =
        kSourceLocationFunc(kExpandedAt, expansion_filename, expansion_line);
  }
  return absl::StrCat(spelling_loc_str, "\n", expansion_loc_str);
}

absl::StatusOr<CcType> Importer::ConvertTemplateSpecializationType(
    const clang::TemplateSpecializationType* type) {
  // Qualifiers are handled separately in TypeMapper::ConvertQualType().
  std::string type_string = clang::QualType(type, 0).getAsString();

  auto* specialization_decl =
      clang::dyn_cast_or_null<clang::ClassTemplateSpecializationDecl>(
          type->getAsCXXRecordDecl());
  if (!specialization_decl) {
    return absl::InvalidArgumentError(absl::Substitute(
        "Template specialization '$0' without an associated record decl "
        "is not supported.",
        type_string));
  }

  if (HasBeenAlreadySuccessfullyImported(specialization_decl))
    return ConvertTypeDecl(specialization_decl);

  // `Sema::isCompleteType` will try to instantiate the class template as a
  // side-effect and we rely on this here. `decl->getDefinition()` can
  // return nullptr before the call to sema and return its definition
  // afterwards.
  // Note: Here we instantiate class template specialization eagerly: its
  // usages in headers may not require the class template specialization to be
  // instantiated (and hence it may not be instantiable), but we attempt
  // instantiation here. So we may attempt non-instantiable template, which
  // would cause the diagnostic stream to contain error, which would case
  // clang::tooling::runToolOnCodeWithArgs to return an error status. To avoid
  // erroring out, we temporarily use our own implementation of
  // DiagnosticConsumer here.
  crubit::RecordingDiagnosticConsumer diagnostic_recorder =
      crubit::RecordDiagnostics(sema_.getDiagnostics(), [&] {
        // Attempt to instantiate.
        (void)sema_.isCompleteType(
            specialization_decl->getLocation(),
            ctx_.getCanonicalTagType(specialization_decl));
      });
  if (diagnostic_recorder.getNumErrors() != 0) {
    return absl::InvalidArgumentError(absl::Substitute(
        "Failed to complete template specialization type $0: Diagnostics "
        "emitted:\n$1",
        type_string, diagnostic_recorder.ConcatenatedDiagnostics()));
  }

  // TODO(lukasza): Limit specialization depth? (e.g. using
  // `isSpecializationDepthGreaterThan` from earlier prototypes).

  absl::Status import_status =
      CheckImportStatus(GetDeclItem(specialization_decl));
  if (!import_status.ok()) {
    return absl::InvalidArgumentError(absl::Substitute(
        "Failed to create bindings for template specialization type $0: $1",
        type_string, import_status.message()));
  }

  return ConvertTypeDecl(specialization_decl);
}

absl::StatusOr<CcType> Importer::ConvertTypeDecl(clang::NamedDecl* decl) {
  if (!EnsureSuccessfullyImported(decl)) {
    return absl::NotFoundError(absl::Substitute(
        "No generated bindings found for '$0'", decl->getNameAsString()));
  }

  return CcType(GenerateItemId(decl));
}

static bool IsSameCanonicalUnqualifiedType(clang::QualType type1,
                                           clang::QualType type2) {
  type1 = type1.getCanonicalType().getUnqualifiedType();
  type2 = type2.getCanonicalType().getUnqualifiedType();

  // `DeducedType::getDeducedType()` can return null, in which case we don't
  // have a more canonical representation. If this happens, optimistically
  // assume the types are equal.
  if (clang::isa<clang::DeducedType>(type1) ||
      clang::isa<clang::DeducedType>(type2))
    return true;

  return type1 == type2;
}

absl::StatusOr<CcType> Importer::ConvertType(
    const clang::Type* type,
    const clang::tidy::lifetimes::ValueLifetimes* lifetimes, bool nullable) {
  absl::StatusOr<CcType> cpp_type =
      ConvertUnattributedType(type, lifetimes, nullable);
  if (cpp_type.ok()) {
    std::optional<std::string> unknown_attr =
        CollectUnknownTypeAttrs(*type, [](clang::attr::Kind kind) {
          using enum clang::attr::Kind;
          switch (kind) {
            // annotate_type is usually meaningless and can be acked as
            // understood. The major exception is lifetimes, which we do
            // already handle separately.
            case AnnotateType:
            // Simply ignore nullability attributes for now.
            // TODO(mboehme): Ultimately, we want to interpret these and
            // change the bindings we produce based on the nullability.
            case TypeNullable:
            case TypeNonNull:
            case TypeNullUnspecified:
            case WarnUnusedResult:
              return true;
            default:
              return false;
          }
        });
    if (unknown_attr.has_value()) {
      cpp_type->unknown_attr = *unknown_attr;
    }
  }
  return cpp_type;
}

absl::StatusOr<CcType> Importer::ConvertUnattributedType(
    const clang::Type* type,
    const clang::tidy::lifetimes::ValueLifetimes* lifetimes, bool nullable) {
  // Qualifiers are handled separately in ConvertQualType().
  std::string type_string = clang::QualType(type, 0).getAsString();

  CRUBIT_ASSIGN_OR_RETURN(
      const clang::AnnotateTypeAttr* crubit_owned_ptr_attr,
      GetTypeAnnotationSingleDecl(type, "crubit_owned_pointer"));

  bool is_owned_ptr = crubit_owned_ptr_attr != nullptr;

  if (is_owned_ptr && !type->isPointerType()) {
    return absl::InvalidArgumentError(
        "CRUBIT_OWNED_POINTER can only be applied to pointer types.");
  }

  assert(!lifetimes || IsSameCanonicalUnqualifiedType(
                           lifetimes->Type(), clang::QualType(type, 0)));

  if (auto primitive = GetPrimitive(*type); primitive.has_value()) {
    return *std::move(primitive);
  } else if (type->isPointerType() || type->isLValueReferenceType() ||
             type->isRValueReferenceType()) {
    clang::QualType pointee_type = type->getPointeeType();
    std::optional<LifetimeId> lifetime;
    const clang::tidy::lifetimes::ValueLifetimes* pointee_lifetimes = nullptr;
    if (lifetimes) {
      lifetime =
          LifetimeId(lifetimes->GetPointeeLifetimes().GetLifetime().Id());
      pointee_lifetimes = &lifetimes->GetPointeeLifetimes().GetValueLifetimes();
    }
    if (const auto* func_type =
            pointee_type->getAs<clang::FunctionProtoType>()) {
      // Assert that the function pointers/references always either 1) have no
      // lifetime or 2) have `'static` lifetime (no other lifetime is allowed).
      CHECK(!lifetime.has_value() ||
            (lifetime->value() ==
             clang::tidy::lifetimes::Lifetime::Static().Id()));

      CRUBIT_ASSIGN_OR_RETURN(
          CallingConv cc_call_conv,
          ConvertCcCallConvToSupportedCallingConv(func_type->getCallConv()));

      std::vector<CcType> param_and_return_types;
      param_and_return_types.reserve(func_type->getNumParams() + 1);
      for (unsigned i = 0; i < func_type->getNumParams(); ++i) {
        const clang::tidy::lifetimes::ValueLifetimes* param_lifetimes = nullptr;
        if (pointee_lifetimes) {
          param_lifetimes =
              &pointee_lifetimes->GetFuncLifetimes().GetParamLifetimes(i);
        }
        CRUBIT_ASSIGN_OR_RETURN(
            CcType cpp_param_type,
            ConvertQualType(func_type->getParamType(i), param_lifetimes));
        param_and_return_types.push_back(std::move(cpp_param_type));
      }

      const clang::tidy::lifetimes::ValueLifetimes* return_lifetimes = nullptr;
      if (pointee_lifetimes) {
        return_lifetimes =
            &pointee_lifetimes->GetFuncLifetimes().GetReturnLifetimes();
      }
      CRUBIT_ASSIGN_OR_RETURN(
          CcType cpp_return_type,
          ConvertQualType(func_type->getReturnType(), return_lifetimes));
      param_and_return_types.push_back(std::move(cpp_return_type));

      CHECK(type->isPointerType() || type->isLValueReferenceType());
      return CcType(CcType::FuncPointer{
          .non_null = type->isLValueReferenceType(),
          .call_conv = cc_call_conv,
          .param_and_return_types = std::move(param_and_return_types),
      });
    }

    CRUBIT_ASSIGN_OR_RETURN(CcType cpp_pointee_type,
                            ConvertQualType(pointee_type, pointee_lifetimes));
    // Note: we don't check for a lifetime here and prefer to defer to the
    // IR consumer to error if a lifetime is required. This allows the IR
    // consumer to infer a lifetime where-appropriate (e.g. constructors).
    if (type->isPointerType()) {
      if (is_owned_ptr) {
        return CcType::OwnedPointerTo(std::move(cpp_pointee_type), lifetime);
      } else {
        return CcType::PointerTo(std::move(cpp_pointee_type), lifetime,
                                 nullable);
      }
    } else if (type->isLValueReferenceType()) {
      return CcType::LValueReferenceTo(std::move(cpp_pointee_type), lifetime);
    } else {
      CHECK(type->isRValueReferenceType());
      return CcType::RValueReferenceTo(std::move(cpp_pointee_type), lifetime);
    }
  } else if (const auto* builtin_type =
                 // Use getAsAdjusted instead of getAs so we don't desugar
                 // typedefs.
             type->getAsAdjusted<clang::BuiltinType>()) {
    switch (builtin_type->getKind()) {
      case clang::BuiltinType::Bool:
        return CcType(CcType::Primitive{"bool"});
      case clang::BuiltinType::Void:
        return CcType(CcType::Primitive{"void"});

      // Floating-point numbers
      //
      // TODO(b/255768062): Generated bindings should explicitly check if
      // `math.h` defines the `__STDC_IEC_559__` macro.
      case clang::BuiltinType::Float:
        return CcType(CcType::Primitive{"float"});
      case clang::BuiltinType::Double:
        return CcType(CcType::Primitive{"double"});

      // `char`
      case clang::BuiltinType::Char_S:  // 'char' in targets where it's signed
      case clang::BuiltinType::Char_U:  // 'char' in targets where it's unsigned
        return CcType(CcType::Primitive{"char"});
      case clang::BuiltinType::SChar:  // 'signed char', explicitly qualified
        return CcType(CcType::Primitive{"signed char"});
      case clang::BuiltinType::UChar:  // 'unsigned char', explicitly qualified
        return CcType(CcType::Primitive{"unsigned char"});

      // Signed integers
      case clang::BuiltinType::Short:
        return CcType(CcType::Primitive{"short"});
      case clang::BuiltinType::Int:
        return CcType(CcType::Primitive{"int"});
      case clang::BuiltinType::Long:
        return CcType(CcType::Primitive{"long"});
      case clang::BuiltinType::LongLong:
        return CcType(CcType::Primitive{"long long"});

      // Unsigned integers
      case clang::BuiltinType::UShort:
        return CcType(CcType::Primitive{"unsigned short"});
      case clang::BuiltinType::UInt:
        return CcType(CcType::Primitive{"unsigned int"});
      case clang::BuiltinType::ULong:
        return CcType(CcType::Primitive{"unsigned long"});
      case clang::BuiltinType::ULongLong:
        return CcType(CcType::Primitive{"unsigned long long"});

      case clang::BuiltinType::Char16:
        return CcType(CcType::Primitive{"char16_t"});
      case clang::BuiltinType::Char32:
        return CcType(CcType::Primitive{"char32_t"});
      default:
        return absl::UnimplementedError("Unsupported builtin type");
    }
  } else if (const auto* tag_type = type->getAsAdjusted<clang::TagType>()) {
    return ConvertTypeDecl(tag_type->getOriginalDecl()->getDefinitionOrSelf());
  } else if (const auto* typedef_type =
                 type->getAsAdjusted<clang::TypedefType>()) {
    return ConvertTypeDecl(typedef_type->getDecl());
  } else if (const auto* using_type = type->getAs<clang::UsingType>()) {
    return ConvertTypeDecl(using_type->getDecl());
  } else if (const auto* tst_type =
                 type->getAs<clang::TemplateSpecializationType>()) {
    return ConvertTemplateSpecializationType(tst_type);
  } else if (const auto* subst_type =
                 type->getAs<clang::SubstTemplateTypeParmType>()) {
    return ConvertQualType(subst_type->getReplacementType(), lifetimes);
  } else if (const auto* deduced_type = type->getAs<clang::DeducedType>()) {
    // Deduction should have taken place earlier (e.g. via DeduceReturnType
    // called from FunctionDeclImporter::Import).
    CHECK(deduced_type->isDeduced());
    return ConvertQualType(deduced_type->getDeducedType(), lifetimes);
  }

  return absl::UnimplementedError(absl::StrCat(
      "Unsupported clang::Type class '", type->getTypeClassName(), "'"));
}

absl::StatusOr<CcType> Importer::ConvertQualType(
    clang::QualType qual_type,
    const clang::tidy::lifetimes::ValueLifetimes* lifetimes, bool nullable) {
  clang::PrintingPolicy printing_policy = ctx_.getPrintingPolicy();
  printing_policy.FullyQualifiedName = true;
  std::string type_string = qual_type.getAsString(printing_policy);

  absl::StatusOr<CcType> type =
      ConvertType(qual_type.getTypePtr(), lifetimes, nullable);
  if (!type.ok()) {
    absl::Status error = absl::UnimplementedError(absl::Substitute(
        "Unsupported type '$0': $1", type_string, type.status().message()));
    error.SetPayload(kTypeStatusPayloadUrl, absl::Cord(type_string));
    return error;
  }

  // Handle cv-qualification.
  type->is_const = qual_type.isConstQualified();
  if (qual_type.isVolatileQualified()) {
    return absl::UnimplementedError(
        absl::StrCat("Unsupported `volatile` qualifier: ", type_string));
  }

  return type;
}

std::string Importer::GetMangledName(const clang::NamedDecl* named_decl) const {
  if (auto record_decl = clang::dyn_cast<clang::RecordDecl>(named_decl)) {
    // Mangled record names are used to 1) provide valid Rust identifiers for
    // C++ template specializations, and 2) help build unique names for virtual
    // upcast thunks.
    llvm::SmallString<128> storage;
    llvm::raw_svector_ostream buffer(storage);
    mangler_->mangleCanonicalTypeName(ctx_.getCanonicalTagType(record_decl),
                                      buffer);

    // The Itanium mangler does not provide a way to get the mangled
    // representation of a type. Instead, we call mangleTypeName() that
    // returns the name of the RTTI typeinfo symbol, and remove the _ZTS
    // prefix.
    constexpr llvm::StringRef kZtsPrefix = "_ZTS";
    CHECK(buffer.str().take_front(4) == kZtsPrefix);
    llvm::StringRef mangled_record_name =
        buffer.str().drop_front(kZtsPrefix.size());

    if (clang::isa<clang::ClassTemplateSpecializationDecl>(named_decl)) {
      // We prepend __CcTemplateInst to reduce chances of conflict
      // with regular C and C++ structs.
      constexpr llvm::StringRef kCcTemplatePrefix = "__CcTemplateInst";
      return llvm::formatv("{0}{1}", kCcTemplatePrefix, mangled_record_name);
    }
    return std::string(mangled_record_name);
  }

  if (!mangler_->shouldMangleDeclName(named_decl)) {
    return named_decl->getIdentifier()->getName().str();
  }

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
  if (auto dtor = clang::dyn_cast<clang::CXXDestructorDecl>(named_decl)) {
    decl = clang::GlobalDecl(dtor, clang::CXXDtorType::Dtor_Complete);
  } else if (auto ctor =
                 clang::dyn_cast<clang::CXXConstructorDecl>(named_decl)) {
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

std::optional<UnsupportedItem::Path>
Importer::GetUnsupportedItemPathForTemplateDecl(
    clang::RedeclarableTemplateDecl* template_decl) {
  auto enclosing_item_id = GetEnclosingItemId(template_decl);
  if (!enclosing_item_id.ok()) {
    return std::nullopt;
  }

  // NOTE: there's no *real* name for an uninstantiated template declaration
  // in Rust, but we're only producing an unsupported item path anyways, so
  // we just use the non-type-qualified name.
  //
  // NOTE: it's tempting to call GetMangledName(template_decl) here, but that
  // segfaults inside ItaniumMangleContextImpl::mangleCXXName. We're not working
  // with a fully-instantiated template declaration, so there is no mangled
  // name to refer to.
  auto names = GetTranslatedName(template_decl);
  if (!names.ok()) {
    return std::nullopt;
  }
  return UnsupportedItem::Path{
      .ident = names->cc_identifier,
      .enclosing_item_id = *enclosing_item_id,
  };
}

std::string Importer::GetNameForSourceOrder(const clang::Decl* decl) const {
  // Implicit class template specializations and their methods all have the
  // same source location. In order to provide deterministic order of the
  // respective items in generated source code, we additionally use the
  // mangled names when sorting the items.
  if (auto* class_template_specialization_decl =
          clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(decl)) {
    return GetMangledName(class_template_specialization_decl);
  } else if (auto* func_decl = clang::dyn_cast<clang::FunctionDecl>(decl)) {
    return GetMangledName(func_decl);
  } else if (auto* friend_decl = clang::dyn_cast<clang::FriendDecl>(decl)) {
    if (auto* named_decl = friend_decl->getFriendDecl()) {
      if (auto function_template_decl =
              clang::dyn_cast<clang::FunctionTemplateDecl>(named_decl)) {
        // Reach through the function template declaration for a function that
        // can be mangled.
        named_decl = function_template_decl->getTemplatedDecl();
      }
      return GetMangledName(named_decl);
    } else {
      // This FriendDecl names a type. We don't import those, so we don't have
      // to assign a name.
      return "";
    }
  } else {
    return "";
  }
}

absl::StatusOr<TranslatedUnqualifiedIdentifier> Importer::GetTranslatedName(
    const clang::NamedDecl* named_decl) const {
  std::optional<UnqualifiedIdentifier> crubit_rust_name =
      CrubitRustName(*named_decl);
  switch (named_decl->getDeclName().getNameKind()) {
    case clang::DeclarationName::Identifier: {
      auto name = std::string(named_decl->getName());
      if (name.empty()) {
        return absl::InvalidArgumentError("Missing identifier");
      }

      // `r#foo` syntax in Rust can't be used to escape `crate`, `self`,
      // `super`, not `Self` identifiers - see
      // https://doc.rust-lang.org/reference/identifiers.html#identifiers
      if ((name == "crate" || name == "self" || name == "super" ||
           name == "Self") &&
          !crubit_rust_name.has_value()) {
        return absl::InvalidArgumentError(
            absl::StrCat("Unescapable identifier: ", name));
      }

      return TranslatedUnqualifiedIdentifier{
          .cc_identifier = Identifier(name),
          .crubit_rust_name = crubit_rust_name,
      };
    }
    case clang::DeclarationName::CXXConstructorName:
      return TranslatedUnqualifiedIdentifier{
          .cc_identifier = SpecialName::kConstructor,
          .crubit_rust_name = crubit_rust_name,
      };
    case clang::DeclarationName::CXXDestructorName:
      return TranslatedUnqualifiedIdentifier{
          .cc_identifier = SpecialName::kDestructor,
          .crubit_rust_name = crubit_rust_name,
      };
    case clang::DeclarationName::CXXOperatorName:
      switch (named_decl->getDeclName().getCXXOverloadedOperator()) {
        case clang::OO_None:
          LOG(FATAL) << "No OO_None expected under CXXOperatorName branch";
        case clang::NUM_OVERLOADED_OPERATORS:
          LOG(FATAL) << "No NUM_OVERLOADED_OPERATORS expected at runtime";
          // clang-format off
        #define OVERLOADED_OPERATOR(name, spelling, ...)  \
        case clang::OO_##name: {                          \
          return TranslatedUnqualifiedIdentifier{ \
            .cc_identifier = Operator(spelling), \
            .crubit_rust_name = crubit_rust_name, \
          }; \
        }
        #include "clang/Basic/OperatorKinds.def"
        #undef OVERLOADED_OPERATOR
          // clang-format on
      }
      LOG(FATAL) << "The `switch` above should handle all cases";
    default:
      // To be implemented later: CXXConversionFunctionName.
      // There are also e.g. literal operators, deduction guides, etc., but
      // we might not need to implement them at all. Full list at:
      // https://clang.llvm.org/doxygen/classclang_1_1DeclarationName.html#a9ab322d434446b43379d39e41af5cbe3
      return absl::UnimplementedError(
          absl::StrCat("Unsupported name: ", named_decl->getNameAsString()));
  }
}

absl::StatusOr<TranslatedIdentifier> Importer::GetTranslatedIdentifier(
    const clang::NamedDecl* named_decl) const {
  CRUBIT_ASSIGN_OR_RETURN(TranslatedUnqualifiedIdentifier unqualified,
                          GetTranslatedName(named_decl));
  Identifier* cc_identifier =
      std::get_if<Identifier>(&unqualified.cc_identifier);
  CHECK(cc_identifier) << "Incorrectly called with a special name";

  TranslatedIdentifier translated_identifiers = {
      .cc_identifier = *cc_identifier,
  };

  if (!unqualified.crubit_rust_name.has_value()) {
    translated_identifiers.crubit_rust_name = *cc_identifier;
    return translated_identifiers;
  }

  if (!std::holds_alternative<Identifier>(*unqualified.crubit_rust_name)) {
    CHECK(unqualified.crubit_rust_name)
        << "Crubit rust name cannot be a special name";
  }

  // TODO(yulanlin): potentially buggy
  translated_identifiers.crubit_rust_name =
      std::move(std::get<Identifier>(*unqualified.crubit_rust_name));

  return translated_identifiers;
}

void Importer::MarkAsSuccessfullyImported(const clang::NamedDecl* decl) {
  known_type_decls_.insert(
      clang::cast<clang::NamedDecl>(CanonicalizeDecl(decl)));
}

bool Importer::HasBeenAlreadySuccessfullyImported(
    const clang::NamedDecl* decl) const {
  return known_type_decls_.contains(
      clang::cast<clang::NamedDecl>(CanonicalizeDecl(decl)));
}

}  // namespace crubit

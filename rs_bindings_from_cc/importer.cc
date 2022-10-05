// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importer.h"

#include <stdint.h>

#include <algorithm>
#include <cstddef>
#include <memory>
#include <optional>
#include <string>
#include <tuple>
#include <utility>
#include <variant>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/check.h"
#include "absl/log/log.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/cord.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_join.h"
#include "absl/strings/string_view.h"
#include "absl/strings/substitute.h"
#include "common/status_macros.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclFriend.h"
#include "clang/AST/Mangle.h"
#include "clang/AST/RawCommentList.h"
#include "clang/AST/Type.h"
#include "clang/Basic/FileManager.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/OperatorKinds.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Sema/Sema.h"
#include "llvm/ADT/Optional.h"
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
    return absl::InvalidArgumentError(unsupported->message);
  }
  return absl::OkStatus();
}
}  // namespace

// A mapping of C++ standard types to their equivalent Rust types.
// To produce more idiomatic results, these types receive special handling
// instead of using the generic type mapping mechanism.
std::optional<absl::string_view> MapKnownCcTypeToRsType(
    absl::string_view cc_type) {
  static const auto* const kWellKnownTypes =
      new absl::flat_hash_map<absl::string_view, absl::string_view>({
          {"ptrdiff_t", "isize"},
          {"intptr_t", "isize"},
          {"size_t", "usize"},
          {"uintptr_t", "usize"},
          {"std::ptrdiff_t", "isize"},
          {"std::intptr_t", "isize"},
          {"std::size_t", "usize"},
          {"std::uintptr_t", "usize"},

          {"int8_t", "i8"},
          {"int16_t", "i16"},
          {"int32_t", "i32"},
          {"int64_t", "i64"},
          {"std::int8_t", "i8"},
          {"std::int16_t", "i16"},
          {"std::int32_t", "i32"},
          {"std::int64_t", "i64"},

          {"uint8_t", "u8"},
          {"uint16_t", "u16"},
          {"uint32_t", "u32"},

          {"uint64_t", "u64"},
          {"std::uint8_t", "u8"},
          {"std::uint16_t", "u16"},
          {"std::uint32_t", "u32"},
          {"std::uint64_t", "u64"},

          {"char16_t", "u16"},
          {"char32_t", "u32"},
          {"wchar_t", "i32"},
      });
  auto it = kWellKnownTypes->find(cc_type);
  if (it == kWellKnownTypes->end()) return std::nullopt;
  return it->second;
}

namespace {

// Converts clang::CallingConv enum [1] into an equivalent Rust Abi [2, 3, 4].
// [1]
// https://github.com/llvm/llvm-project/blob/c6a3225bb03b6afc2b63fbf13db3c100406b32ce/clang/include/clang/Basic/Specifiers.h#L262-L283
// [2] https://doc.rust-lang.org/reference/types/function-pointer.html
// [3]
// https://doc.rust-lang.org/reference/items/functions.html#extern-function-qualifier
// [4]
// https://github.com/rust-lang/rust/blob/b27ccbc7e1e6a04d749e244a3c13f72ca38e80e7/compiler/rustc_target/src/spec/abi.rs#L49
absl::StatusOr<absl::string_view> ConvertCcCallConvIntoRsAbi(
    clang::CallingConv cc_call_conv) {
  switch (cc_call_conv) {
    case clang::CC_C:  // __attribute__((cdecl))
      // https://doc.rust-lang.org/reference/items/external-blocks.html#abi says
      // that:
      // - `extern "C"` [...] whatever the default your C compiler supports.
      // - `extern "cdecl"` -- The default for x86_32 C code.
      //
      // We don't support C++ exceptions and therefore we use "C" (rather than
      // "C-unwind") - we have no need for unwinding across the FFI boundary -
      // e.g. from C++ into Rust frames (or vice versa).
      return "C";
    case clang::CC_X86FastCall:  // __attribute__((fastcall))
      // https://doc.rust-lang.org/reference/items/external-blocks.html#abi says
      // that the fastcall ABI -- corresponds to MSVC's __fastcall and GCC and
      // clang's __attribute__((fastcall)).
      return "fastcall";
    case clang::CC_X86VectorCall:  // __attribute__((vectorcall))
      // https://doc.rust-lang.org/reference/items/external-blocks.html#abi says
      // that the vectorcall ABI -- corresponds to MSVC's __vectorcall and
      // clang's __attribute__((vectorcall)).
      return "vectorcall";
    case clang::CC_X86ThisCall:  // __attribute__((thiscall))
      // We don't support C++ exceptions and therefore we use "thiscall" (rather
      // than "thiscall-unwind") - we have no need for unwinding across the FFI
      // boundary - e.g. from C++ into Rust frames (or vice versa).
      return "thiscall";
    case clang::CC_X86StdCall:  // __attribute__((stdcall))
      // https://doc.rust-lang.org/reference/items/external-blocks.html#abi says
      // extern "stdcall" -- The default for the Win32 API on x86_32.
      //
      // We don't support C++ exceptions and therefore we use "stdcall" (rather
      // than "stdcall-unwind") - we have no need for unwinding across the FFI
      // boundary - e.g. from C++ into Rust frames (or vice versa).
      return "stdcall";
    case clang::CC_Win64:  // __attribute__((ms_abi))
      // https://doc.rust-lang.org/reference/items/external-blocks.html#abi says
      // extern "win64" -- The default for C code on x86_64 Windows.
      return "win64";
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
    case clang::CC_OpenCLKernel:  // inferred for OpenCL kernels
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
                   absl::string_view(
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
  SourceOrderKey(clang::SourceRange source_range, int decl_order = 0,
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
  const bool operator()(const clang::SourceLocation& a,
                        const clang::SourceLocation& b) const {
    return b.isValid() && a.isValid() && sm.isBeforeInTranslationUnit(a, b);
  }
  const bool operator()(const clang::RawComment* a,
                        const clang::SourceLocation& b) const {
    return this->operator()(a->getBeginLoc(), b);
  }
  const bool operator()(const clang::SourceLocation& a,
                        const clang::RawComment* b) const {
    return this->operator()(a, b->getBeginLoc());
  }
  const bool operator()(const clang::RawComment* a,
                        const clang::RawComment* b) const {
    return this->operator()(a->getBeginLoc(), b->getBeginLoc());
  }

  using OrderedItemId = std::pair<SourceOrderKey, ItemId>;
  using OrderedItem = std::pair<SourceOrderKey, IR::Item>;

  template <typename OrderedItemOrId>
  bool operator()(const OrderedItemOrId& a, const OrderedItemOrId& b) const {
    auto a_source_order = a.first;
    auto b_source_order = b.first;
    return a_source_order.isBefore(b_source_order, sm);
  }
  SourceLocationComparator(const clang::SourceManager& sm) : sm(sm) {}

 private:
  const clang::SourceManager& sm;
};

static std::vector<clang::Decl*> GetCanonicalChildren(
    const clang::DeclContext* decl_context) {
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
    if (decl == decl->getCanonicalDecl() ||
        clang::isa<clang::NamespaceDecl>(decl)) {
      result.push_back(decl);
    }
  }

  return result;
}

std::vector<ItemId> Importer::GetItemIdsInSourceOrder(
    clang::Decl* parent_decl) {
  clang::SourceManager& sm = ctx_.getSourceManager();
  std::vector<SourceLocationComparator::OrderedItemId> items;
  auto compare_locations = SourceLocationComparator(sm);

  // We are only interested in comments within this decl context.
  std::vector<const clang::RawComment*> comments_in_range(
      llvm::lower_bound(comments_, parent_decl->getBeginLoc(),
                        compare_locations),
      llvm::upper_bound(comments_, parent_decl->getEndLoc(),
                        compare_locations));

  std::map<clang::SourceLocation, const clang::RawComment*,
           SourceLocationComparator>
      ordered_comments(compare_locations);
  for (auto& comment : comments_in_range) {
    ordered_comments.insert({comment->getBeginLoc(), comment});
  }

  absl::flat_hash_set<ItemId> visited_item_ids;

  auto* decl_context = clang::cast<clang::DeclContext>(parent_decl);
  for (auto decl : GetCanonicalChildren(decl_context)) {
    auto item = GetDeclItem(decl);
    // We generated IR for top level items coming from different targets,
    // however we shouldn't generate bindings for them, so we don't add them
    // to ir.top_level_item_ids.
    if (decl_context->isTranslationUnit() && !IsFromCurrentTarget(decl)) {
      continue;
    }
    // Only add item ids for decls that can be successfully imported.
    if (item.has_value()) {
      auto item_id = GenerateItemId(decl);
      // TODO(rosica): Drop this check when we start importing also other
      // redecls, not just the canonical
      if (visited_item_ids.find(item_id) == visited_item_ids.end()) {
        visited_item_ids.insert(item_id);
        items.push_back({GetSourceOrderKey(decl), item_id});
      }
    }

    // We remove comments attached to a child decl or that are within a child
    // decl.
    if (auto raw_comment = ctx_.getRawCommentForDeclNoCache(decl)) {
      ordered_comments.erase(raw_comment->getBeginLoc());
    }
    ordered_comments.erase(ordered_comments.lower_bound(decl->getBeginLoc()),
                           ordered_comments.upper_bound(decl->getEndLoc()));
  }

  for (auto& [_, comment] : ordered_comments) {
    items.push_back({GetSourceOrderKey(comment), GenerateItemId(comment)});
  }
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
  for (const auto& header : invocation_.entry_headers_) {
    if (auto file = sm.getFileManager().getFile(header.IncludePath())) {
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

  for (auto& comment : comments_) {
    ordered_items.push_back(
        {GetSourceOrderKey(comment),
         Comment{.text = comment->getFormattedText(sm, sm.getDiagnostics()),
                 .id = GenerateItemId(comment)}});
  }

  ImportDeclsFromDeclContext(translation_unit_decl);
  for (const auto& [decl, item] : import_cache_) {
    if (item.has_value()) {
      if (std::holds_alternative<UnsupportedItem>(*item) &&
          !IsFromCurrentTarget(decl)) {
        continue;
      }
      ordered_items.push_back({GetSourceOrderKey(decl), *item});
    }
  }

  llvm::sort(ordered_items, SourceLocationComparator(sm));

  invocation_.ir_.items.reserve(ordered_items.size());
  for (auto& ordered_item : ordered_items) {
    invocation_.ir_.items.push_back(ordered_item.second);
  }
  invocation_.ir_.top_level_item_ids =
      GetItemIdsInSourceOrder(translation_unit_decl);

  // TODO(b/222001243): Consider placing the generated template instantiations
  // into a separate namespace (maybe `crubit::instantiated_templates` ?).
  llvm::copy(GetOrderedItemIdsOfTemplateInstantiations(),
             std::back_inserter(invocation_.ir_.top_level_item_ids));
}

void Importer::ImportDeclsFromDeclContext(
    const clang::DeclContext* decl_context) {
  for (auto decl : GetCanonicalChildren(decl_context)) {
    GetDeclItem(decl);
  }
}

std::optional<IR::Item> Importer::GetDeclItem(clang::Decl* decl) {
  // TODO: Move `decl->getCanonicalDecl()` from callers into here.
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
  if (auto* specialization_decl =
          llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(decl)) {
    // Store `specialization_decl`s so that they will get included in
    // IR::top_level_item_ids.
    class_template_instantiations_.insert(specialization_decl);
  }
  return result;
}

std::optional<IR::Item> Importer::ImportDecl(clang::Decl* decl) {
  std::optional<IR::Item> result;
  for (auto& importer : decl_importers_) {
    if (importer->CanImport(decl)) {
      result = importer->ImportDecl(decl);
    }
  }
  return result;
}

std::optional<IR::Item> Importer::GetImportedItem(const clang::Decl* decl) {
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

  clang::SourceManager& source_manager = ctx_.getSourceManager();
  auto source_location = decl->getLocation();

  // If the header this decl comes from is not associated with a target we
  // consider it a textual header. In that case we go up the include stack
  // until we find a header that has an owning target.

  while (source_location.isValid()) {
    if (source_location.isMacroID()) {
      source_location = source_manager.getExpansionLoc(source_location);
    }
    auto id = source_manager.getFileID(source_location);
    llvm::Optional<llvm::StringRef> filename =
        source_manager.getNonBuiltinFilenameForID(id);
    if (!filename) {
      return BazelLabel("//:_nothing_should_depend_on_private_builtin_hdrs");
    }
    if (filename->startswith("./")) {
      filename = filename->substr(2);
    }

    if (auto target = invocation_.header_target(HeaderName(filename->str()))) {
      return *target;
    }
    source_location = source_manager.getIncludeLoc(id);
  }

  return BazelLabel("//:virtual_clang_resource_dir_target");
}

bool Importer::IsFromCurrentTarget(const clang::Decl* decl) const {
  return invocation_.target_ == GetOwningTarget(decl);
}

IR::Item Importer::ImportUnsupportedItem(const clang::Decl* decl,
                                         std::string error) {
  std::string name = "unnamed";
  if (const auto* named_decl = clang::dyn_cast<clang::NamedDecl>(decl)) {
    name = named_decl->getQualifiedNameAsString();
  }
  SourceLoc source_loc = ConvertSourceLocation(decl->getBeginLoc());
  return UnsupportedItem{.name = name,
                         .message = error,
                         .source_loc = source_loc,
                         .id = GenerateItemId(decl)};
}

IR::Item Importer::ImportUnsupportedItem(const clang::Decl* decl,
                                         std::set<std::string> errors) {
  return ImportUnsupportedItem(decl, absl::StrJoin(errors, "\n\n"));
}

static bool ShouldKeepCommentLine(absl::string_view line) {
  // Based on https://clang.llvm.org/extra/clang-tidy/:
  llvm::Regex patterns_to_ignore(
      "^[[:space:]/]*"  // Whitespace, or extra //
      "(NOLINT|NOLINTNEXTLINE|NOLINTBEGIN|NOLINTEND)"
      "(\\([^)[:space:]]*\\)?)?"  // Optional (...)
      "[[:space:]]*$");           // Whitespace
  return !patterns_to_ignore.match(line);
}

llvm::Optional<std::string> Importer::GetComment(
    const clang::Decl* decl) const {
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

SourceLoc Importer::ConvertSourceLocation(clang::SourceLocation loc) const {
  auto& sm = ctx_.getSourceManager();

  clang::StringRef filename = sm.getFilename(loc);
  if (filename.startswith("./")) {
    filename = filename.substr(2);
  }

  return SourceLoc{.filename = filename.str(),
                   .line = sm.getSpellingLineNumber(loc),
                   .column = sm.getSpellingColumnNumber(loc)};
}

absl::StatusOr<MappedType> Importer::ConvertTemplateSpecializationType(
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
  (void)sema_.isCompleteType(specialization_decl->getLocation(),
                             ctx_.getRecordType(specialization_decl));

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

absl::StatusOr<MappedType> Importer::ConvertTypeDecl(clang::TypeDecl* decl) {
  if (!EnsureSuccessfullyImported(decl)) {
    return absl::NotFoundError(absl::Substitute(
        "No generated bindings found for '$0'", decl->getNameAsString()));
  }

  ItemId decl_id = GenerateItemId(decl);
  return MappedType::WithDeclId(decl_id);
}

absl::StatusOr<MappedType> Importer::ConvertType(
    const clang::Type* type,
    std::optional<clang::tidy::lifetimes::ValueLifetimes>& lifetimes,
    bool nullable) {
  // Qualifiers are handled separately in ConvertQualType().
  std::string type_string = clang::QualType(type, 0).getAsString();

  if (auto maybe_mapped_type = MapKnownCcTypeToRsType(type_string);
      maybe_mapped_type.has_value()) {
    return MappedType::Simple(std::string(*maybe_mapped_type), type_string);
  } else if (type->isPointerType() || type->isLValueReferenceType() ||
             type->isRValueReferenceType()) {
    clang::QualType pointee_type = type->getPointeeType();
    std::optional<LifetimeId> lifetime;
    if (lifetimes.has_value()) {
      lifetime =
          LifetimeId(lifetimes->GetPointeeLifetimes().GetLifetime().Id());
      lifetimes = lifetimes->GetPointeeLifetimes().GetValueLifetimes();
    }
    if (const auto* func_type =
            pointee_type->getAs<clang::FunctionProtoType>()) {
      if (lifetime.has_value() &&
          lifetime->value() !=
              clang::tidy::lifetimes::Lifetime::Static().Id()) {
        return absl::UnimplementedError(
            absl::StrCat("Function pointers with non-'static lifetimes are "
                         "not supported: ",
                         type_string));
      }

      clang::StringRef cc_call_conv =
          clang::FunctionType::getNameForCallConv(func_type->getCallConv());
      CRUBIT_ASSIGN_OR_RETURN(
          absl::string_view rs_abi,
          ConvertCcCallConvIntoRsAbi(func_type->getCallConv()));
      CRUBIT_ASSIGN_OR_RETURN(
          MappedType mapped_return_type,
          ConvertQualType(func_type->getReturnType(), lifetimes));

      std::vector<MappedType> mapped_param_types;
      for (const clang::QualType& param_type : func_type->getParamTypes()) {
        CRUBIT_ASSIGN_OR_RETURN(MappedType mapped_param_type,
                                ConvertQualType(param_type, lifetimes));
        mapped_param_types.push_back(std::move(mapped_param_type));
      }

      if (type->isPointerType()) {
        return MappedType::FuncPtr(cc_call_conv, rs_abi, lifetime,
                                   std::move(mapped_return_type),
                                   std::move(mapped_param_types));
      } else {
        CHECK(type->isLValueReferenceType());
        return MappedType::FuncRef(cc_call_conv, rs_abi, lifetime,
                                   std::move(mapped_return_type),
                                   std::move(mapped_param_types));
      }
    }

    CRUBIT_ASSIGN_OR_RETURN(MappedType mapped_pointee_type,
                            ConvertQualType(pointee_type, lifetimes));
    if (type->isPointerType()) {
      return MappedType::PointerTo(std::move(mapped_pointee_type), lifetime,
                                   nullable);
    } else if (type->isLValueReferenceType()) {
      return MappedType::LValueReferenceTo(std::move(mapped_pointee_type),
                                           lifetime);
    } else {
      CHECK(type->isRValueReferenceType());
      if (!lifetime.has_value()) {
        return absl::UnimplementedError(
            "Unsupported type: && without lifetime");
      }
      return MappedType::RValueReferenceTo(std::move(mapped_pointee_type),
                                           *lifetime);
    }
  } else if (const auto* builtin_type =
                 // Use getAsAdjusted instead of getAs so we don't desugar
                 // typedefs.
             type->getAsAdjusted<clang::BuiltinType>()) {
    switch (builtin_type->getKind()) {
      case clang::BuiltinType::Bool:
        return MappedType::Simple("bool", "bool");
        break;
      case clang::BuiltinType::Float:
        return MappedType::Simple("f32", "float");
        break;
      case clang::BuiltinType::Double:
        return MappedType::Simple("f64", "double");
        break;
      case clang::BuiltinType::Void:
        return MappedType::Void();
        break;
      default:
        if (builtin_type->isIntegerType()) {
          auto size = ctx_.getTypeSize(builtin_type);
          if (size == 8 || size == 16 || size == 32 || size == 64) {
            return MappedType::Simple(
                absl::Substitute(
                    "$0$1", builtin_type->isSignedInteger() ? 'i' : 'u', size),
                type_string);
          }
        }
    }
  } else if (const auto* tag_type = type->getAsAdjusted<clang::TagType>()) {
    return ConvertTypeDecl(tag_type->getDecl());
  } else if (const auto* typedef_type =
                 type->getAsAdjusted<clang::TypedefType>()) {
    return ConvertTypeDecl(typedef_type->getDecl());
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

// Returns a QualType with leading ElaboratedType nodes removed.
//
// This is analogous to getDesugaredType but *only* removes ElaboratedType
// sugar.
static clang::QualType GetUnelaboratedType(clang::QualType qual_type,
                                           clang::ASTContext& ast_context) {
  clang::QualifierCollector qualifiers;
  while (true) {
    const clang::Type* type = qualifiers.strip(qual_type);
    if (const auto* elaborated = llvm::dyn_cast<clang::ElaboratedType>(type)) {
      qual_type = elaborated->getNamedType();
      continue;
    }

    return ast_context.getQualifiedType(type, qualifiers);
  }
}

absl::StatusOr<MappedType> Importer::ConvertQualType(
    clang::QualType qual_type,
    std::optional<clang::tidy::lifetimes::ValueLifetimes>& lifetimes,
    bool nullable) {
  qual_type = GetUnelaboratedType(std::move(qual_type), ctx_);
  std::string type_string = qual_type.getAsString();
  absl::StatusOr<MappedType> type =
      ConvertType(qual_type.getTypePtr(), lifetimes, nullable);
  if (!type.ok()) {
    absl::Status error = absl::UnimplementedError(absl::Substitute(
        "Unsupported type '$0': $1", type_string, type.status().message()));
    error.SetPayload(kTypeStatusPayloadUrl, absl::Cord(type_string));
    return error;
  }

  // Handle cv-qualification.
  type->cc_type.is_const = qual_type.isConstQualified();
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
    mangler_->mangleTypeName(ctx_.getRecordType(record_decl), buffer);

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

std::optional<UnqualifiedIdentifier> Importer::GetTranslatedName(
    const clang::NamedDecl* named_decl) const {
  switch (named_decl->getDeclName().getNameKind()) {
    case clang::DeclarationName::Identifier: {
      auto name = std::string(named_decl->getName());
      if (const clang::ParmVarDecl* param_decl =
              clang::dyn_cast<clang::ParmVarDecl>(named_decl)) {
        int param_pos = param_decl->getFunctionScopeIndex();
        if (name.empty()) {
          return {Identifier(absl::StrCat("__param_", param_pos))};
        }
        if (auto* sttpt = param_decl->getType()
                              ->getAs<clang::SubstTemplateTypeParmType>();
            sttpt && sttpt->getReplacedParameter()->isParameterPack()) {
          // Avoid giving the same name to all parameters expanded from a pack.
          return {Identifier(absl::StrCat("__", name, "_", param_pos))};
        }
      }
      if (name.empty()) {
        if (auto* tag_type = llvm::dyn_cast<clang::TagDecl>(named_decl)) {
          if (auto* typedef_decl = tag_type->getTypedefNameForAnonDecl()) {
            std::optional<Identifier> identifier =
                GetTranslatedIdentifier(typedef_decl);
            CHECK(identifier.has_value());  // This must always hold.
            return {*std::move(identifier)};
          }
        }
        return std::nullopt;
      }
      return {Identifier(std::move(name))};
    }
    case clang::DeclarationName::CXXConstructorName:
      return {SpecialName::kConstructor};
    case clang::DeclarationName::CXXDestructorName:
      return {SpecialName::kDestructor};
    case clang::DeclarationName::CXXOperatorName:
      switch (named_decl->getDeclName().getCXXOverloadedOperator()) {
        case clang::OO_None:
          LOG(FATAL) << "No OO_None expected under CXXOperatorName branch";
          return std::nullopt;
        case clang::NUM_OVERLOADED_OPERATORS:
          LOG(FATAL) << "No NUM_OVERLOADED_OPERATORS expected at runtime";
          return std::nullopt;
          // clang-format off
        #define OVERLOADED_OPERATOR(name, spelling, ...)  \
        case clang::OO_##name: {                          \
          return {Operator(spelling)};                    \
        }
        #include "clang/Basic/OperatorKinds.def"
        #undef OVERLOADED_OPERATOR
          // clang-format on
      }
      LOG(FATAL) << "The `switch` above should handle all cases";
      return std::nullopt;
    default:
      // To be implemented later: CXXConversionFunctionName.
      // There are also e.g. literal operators, deduction guides, etc., but
      // we might not need to implement them at all. Full list at:
      // https://clang.llvm.org/doxygen/classclang_1_1DeclarationName.html#a9ab322d434446b43379d39e41af5cbe3
      return std::nullopt;
  }
}

void Importer::MarkAsSuccessfullyImported(const clang::TypeDecl* decl) {
  known_type_decls_.insert(
      clang::cast<clang::TypeDecl>(decl->getCanonicalDecl()));
}

bool Importer::HasBeenAlreadySuccessfullyImported(
    const clang::TypeDecl* decl) const {
  return known_type_decls_.contains(
      clang::cast<clang::TypeDecl>(decl->getCanonicalDecl()));
}

}  // namespace crubit

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

#include "third_party/absl/container/flat_hash_map.h"
#include "third_party/absl/container/flat_hash_set.h"
#include "third_party/absl/status/status.h"
#include "third_party/absl/status/statusor.h"
#include "third_party/absl/strings/cord.h"
#include "third_party/absl/strings/match.h"
#include "third_party/absl/strings/str_cat.h"
#include "third_party/absl/strings/str_join.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/absl/strings/substitute.h"
#include "common/check.h"
#include "common/status_macros.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "rs_bindings_from_cc/ast_convert.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Attrs.inc"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/CXXInheritance.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Decl.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/DeclCXX.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/DeclTemplate.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Mangle.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/RawCommentList.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/RecordLayout.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Type.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/FileManager.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/OperatorKinds.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/SourceLocation.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/SourceManager.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/Specifiers.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Sema/Sema.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/Optional.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/STLExtras.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/SmallPtrSet.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/Casting.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/ErrorHandling.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/FormatVariadic.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/Regex.h"

namespace crubit {
namespace {

constexpr absl::string_view kTypeStatusPayloadUrl =
    "type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type";

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

ItemId GenerateItemId(const clang::Decl* decl) {
  return ItemId(reinterpret_cast<uintptr_t>(decl->getCanonicalDecl()));
}

ItemId GenerateItemId(const clang::RawComment* comment) {
  return ItemId(reinterpret_cast<uintptr_t>(comment));
}

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
      // These don't seem to have any Rust equivalents.
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

class SourceLocationComparator {
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

  using OrderedItemId = std::tuple<clang::SourceRange, int, ItemId>;
  using OrderedItem = std::tuple<clang::SourceRange, int, IR::Item>;

  template <typename OrderedItemOrId>
  bool operator()(const OrderedItemOrId& a, const OrderedItemOrId& b) const {
    auto a_range = std::get<0>(a);
    auto b_range = std::get<0>(b);
    if (!a_range.isValid() || !b_range.isValid()) {
      if (a_range.isValid() != b_range.isValid())
        return !a_range.isValid() && b_range.isValid();
    } else {
      if (a_range.getBegin() != b_range.getBegin()) {
        return sm.isBeforeInTranslationUnit(a_range.getBegin(),
                                            b_range.getBegin());
      }
      if (a_range.getEnd() != b_range.getEnd()) {
        return sm.isBeforeInTranslationUnit(a_range.getEnd(), b_range.getEnd());
      }
    }
    auto a_decl_order = std::get<1>(a);
    auto b_decl_order = std::get<1>(b);
    return a_decl_order < b_decl_order;
  }

  SourceLocationComparator(clang::SourceManager& sm) : sm(sm) {}

 private:
  clang::SourceManager& sm;
};

std::vector<ItemId> Importer::GetItemIdsInSourceOrder(
    clang::Decl* parent_decl) {
  auto decl_context = clang::cast<clang::DeclContext>(parent_decl);

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
  for (auto child : decl_context->decls()) {
    auto decl = child->getCanonicalDecl();
    if (!IsFromCurrentTarget(decl)) continue;

    // We remove comments attached to a child decl or that are within a child
    // decl.
    if (auto raw_comment = ctx_.getRawCommentForDeclNoCache(decl)) {
      ordered_comments.erase(raw_comment->getBeginLoc());
    }
    ordered_comments.erase(ordered_comments.lower_bound(decl->getBeginLoc()),
                           ordered_comments.upper_bound(decl->getEndLoc()));

    // Only add item ids for decls that can be successfully imported.
    if (GetDeclItem(decl)) {
      auto item_id = GenerateItemId(decl);
      // TODO(rosica): Drop this check when we start importing also other
      // redecls, not just the canonical
      if (visited_item_ids.find(item_id) == visited_item_ids.end()) {
        visited_item_ids.insert(item_id);
        items.push_back({decl->getSourceRange(), GetDeclOrder(decl), item_id});
      }
    }
  }

  for (auto& [_, comment] : ordered_comments) {
    items.push_back({comment->getSourceRange(), 0, GenerateItemId(comment)});
  }
  std::sort(items.begin(), items.end(), compare_locations);

  std::vector<ItemId> ordered_item_ids;
  ordered_item_ids.reserve(items.size());
  for (auto& ordered_item : items) {
    ordered_item_ids.push_back(std::get<2>(ordered_item));
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
  std::sort(comments_.begin(), comments_.end(), SourceLocationComparator(sm));
}

void Importer::Import(clang::TranslationUnitDecl* translation_unit_decl) {
  ImportFreeComments();
  clang::SourceManager& sm = ctx_.getSourceManager();
  std::vector<SourceLocationComparator::OrderedItem> ordered_items;

  for (auto& comment : comments_) {
    ordered_items.push_back(
        {comment->getSourceRange(), 0,
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
      ordered_items.push_back(
          {decl->getSourceRange(), GetDeclOrder(decl), *item});
    }
  }

  std::sort(ordered_items.begin(), ordered_items.end(),
            SourceLocationComparator(sm));

  invocation_.ir_.items.reserve(ordered_items.size());
  for (auto& ordered_item : ordered_items) {
    invocation_.ir_.items.push_back(std::get<2>(ordered_item));
  }
  invocation_.ir_.top_level_item_ids =
      GetItemIdsInSourceOrder(translation_unit_decl);
}

void Importer::ImportDeclsFromDeclContext(
    const clang::DeclContext* decl_context) {
  for (auto decl : decl_context->decls()) {
    // TODO(rosica): We don't always want the canonical decl here (especially
    // not in namespaces).
    GetDeclItem(decl->getCanonicalDecl());
  }
}

std::optional<IR::Item> Importer::GetDeclItem(clang::Decl* decl) {
  // TODO: Move `decl->getCanonicalDecl()` from callers into here.
  auto it = import_cache_.find(decl);
  if (it == import_cache_.end()) {
    it = import_cache_.insert({decl, ImportDecl(decl)}).first;
  }
  return it->second;
}

std::optional<IR::Item> Importer::ImportDecl(clang::Decl* decl) {
  if (auto* namespace_decl = clang::dyn_cast<clang::NamespaceDecl>(decl)) {
    return ImportNamespace(namespace_decl);
  } else if (auto* function_decl = clang::dyn_cast<clang::FunctionDecl>(decl)) {
    return ImportFunction(function_decl);
  } else if (auto* function_template_decl =
                 clang::dyn_cast<clang::FunctionTemplateDecl>(decl)) {
    return ImportFunction(function_template_decl->getTemplatedDecl(),
                          function_template_decl);
  } else if (auto* record_decl = clang::dyn_cast<clang::CXXRecordDecl>(decl)) {
    auto result = ImportRecord(record_decl);
    // TODO(forster): Should we even visit the nested decl if we couldn't
    // import the parent? For now we have tests that check that we generate
    // error messages for those decls, so we're visiting.
    ImportDeclsFromDeclContext(record_decl);
    return result;
  } else if (auto* enum_decl = clang::dyn_cast<clang::EnumDecl>(decl)) {
    return ImportEnum(enum_decl);
  } else if (auto* typedef_name_decl =
                 clang::dyn_cast<clang::TypedefNameDecl>(decl)) {
    return ImportTypedefName(typedef_name_decl);
  } else if (clang::isa<clang::ClassTemplateDecl>(decl)) {
    return ImportUnsupportedItem(decl, "Class templates are not supported yet");
  } else {
    return std::nullopt;
  }
}

std::optional<IR::Item> Importer::ImportNamespace(
    clang::NamespaceDecl* namespace_decl) {
  if (!IsFromCurrentTarget(namespace_decl)) return std::nullopt;

  // TODO(rosica) In order to fully enable namespaces we first need to ensure
  // that each decl Item contains information on its namespace parents.
  if (!absl::StrContains(namespace_decl->getQualifiedNameAsString(),
                         "test_namespace_bindings")) {
    return ImportUnsupportedItem(namespace_decl,
                                 "Namespaces are not supported yet");
  }

  if (namespace_decl->isInline()) {
    return ImportUnsupportedItem(namespace_decl,
                                 "Inline namespaces are not supported yet");
  }
  if (namespace_decl->isAnonymousNamespace()) {
    return ImportUnsupportedItem(namespace_decl,
                                 "Anonymous namespaces are not supported yet");
  }

  ImportDeclsFromDeclContext(namespace_decl);
  auto identifier = GetTranslatedIdentifier(namespace_decl);
  CRUBIT_CHECK(identifier.has_value());
  auto item_ids = GetItemIdsInSourceOrder(namespace_decl);
  return Namespace{
      .name = *identifier,
      .id = GenerateItemId(namespace_decl),
      .owning_target = GetOwningTarget(namespace_decl),
      .child_item_ids = std::move(item_ids),
  };
}

std::optional<IR::Item> Importer::ImportFunction(
    clang::FunctionDecl* function_decl,
    clang::FunctionTemplateDecl* function_template_decl) {
  if (!IsFromCurrentTarget(function_decl)) return std::nullopt;
  if (function_decl->isDeleted()) return std::nullopt;
  if (function_decl->isTemplated()) {
    return ImportUnsupportedItem(function_template_decl,
                                 "Function templates are not supported yet");
  }

  clang::tidy::lifetimes::LifetimeSymbolTable lifetime_symbol_table;
  llvm::Expected<clang::tidy::lifetimes::FunctionLifetimes> lifetimes =
      clang::tidy::lifetimes::GetLifetimeAnnotations(
          function_decl, *invocation_.lifetime_context_,
          &lifetime_symbol_table);

  std::vector<FuncParam> params;
  std::set<std::string> errors;
  auto add_error = [&errors](std::string msg) {
    auto result = errors.insert(std::move(msg));
    CRUBIT_CHECK(result.second && "Duplicated error message");
  };
  if (auto* method_decl =
          clang::dyn_cast<clang::CXXMethodDecl>(function_decl)) {
    if (!type_mapper_.Contains(method_decl->getParent())) {
      return ImportUnsupportedItem(function_decl, "Couldn't import the parent");
    }

    // non-static member functions receive an implicit `this` parameter.
    if (method_decl->isInstance()) {
      std::optional<clang::tidy::lifetimes::ValueLifetimes> this_lifetimes;
      if (lifetimes) {
        this_lifetimes = lifetimes->GetThisLifetimes();
      }
      auto param_type = type_mapper_.ConvertQualType(method_decl->getThisType(),
                                                     this_lifetimes,
                                                     /*nullable=*/false);
      if (!param_type.ok()) {
        add_error(absl::StrCat("`this` parameter is not supported: ",
                               param_type.status().message()));
      } else {
        params.push_back({*std::move(param_type), Identifier("__this")});
      }
    }
  }

  if (lifetimes) {
    CRUBIT_CHECK(lifetimes->IsValidForDecl(function_decl));
  }

  for (unsigned i = 0; i < function_decl->getNumParams(); ++i) {
    const clang::ParmVarDecl* param = function_decl->getParamDecl(i);
    std::optional<clang::tidy::lifetimes::ValueLifetimes> param_lifetimes;
    if (lifetimes) {
      param_lifetimes = lifetimes->GetParamLifetimes(i);
    }
    auto param_type =
        type_mapper_.ConvertQualType(param->getType(), param_lifetimes);
    if (!param_type.ok()) {
      add_error(absl::Substitute("Parameter #$0 is not supported: $1", i,
                                 param_type.status().message()));
      continue;
    }

    if (const clang::RecordType* record_type =
            clang::dyn_cast<clang::RecordType>(param->getType())) {
      if (clang::RecordDecl* record_decl =
              clang::dyn_cast<clang::RecordDecl>(record_type->getDecl())) {
        // TODO(b/200067242): non-trivial_abi structs, when passed by value,
        // have a different representation which needs special support. We
        // currently do not support it.
        if (!record_decl->canPassInRegisters()) {
          add_error(
              absl::Substitute("Non-trivial_abi type '$0' is not "
                               "supported by value as parameter #$1",
                               param->getType().getAsString(), i));
        }
      }
    }

    std::optional<Identifier> param_name = GetTranslatedIdentifier(param);
    CRUBIT_CHECK(param_name.has_value());  // No known failure cases.
    params.push_back({*param_type, *std::move(param_name)});
  }

  if (const clang::RecordType* record_return_type =
          clang::dyn_cast<clang::RecordType>(function_decl->getReturnType())) {
    if (clang::RecordDecl* record_decl =
            clang::dyn_cast<clang::RecordDecl>(record_return_type->getDecl())) {
      // TODO(b/200067242): non-trivial_abi structs, when passed by value,
      // have a different representation which needs special support. We
      // currently do not support it.
      if (!record_decl->canPassInRegisters()) {
        add_error(
            absl::Substitute("Non-trivial_abi type '$0' is not supported "
                             "by value as a return type",
                             function_decl->getReturnType().getAsString()));
      }
    }
  }

  std::optional<clang::tidy::lifetimes::ValueLifetimes> return_lifetimes;
  if (lifetimes) {
    return_lifetimes = lifetimes->GetReturnLifetimes();
  }

  auto return_type = type_mapper_.ConvertQualType(
      function_decl->getReturnType(), return_lifetimes);
  if (!return_type.ok()) {
    add_error(absl::StrCat("Return type is not supported: ",
                           return_type.status().message()));
  }

  llvm::DenseSet<clang::tidy::lifetimes::Lifetime> all_free_lifetimes;
  if (lifetimes) {
    all_free_lifetimes = lifetimes->AllFreeLifetimes();
  }

  std::vector<LifetimeName> lifetime_params;
  for (clang::tidy::lifetimes::Lifetime lifetime : all_free_lifetimes) {
    std::optional<llvm::StringRef> name =
        lifetime_symbol_table.LookupLifetime(lifetime);
    CRUBIT_CHECK(name.has_value());
    lifetime_params.push_back(
        {.name = name->str(), .id = LifetimeId(lifetime.Id())});
  }
  std::sort(lifetime_params.begin(), lifetime_params.end(),
            [](const LifetimeName& l1, const LifetimeName& l2) {
              return l1.name < l2.name;
            });

  llvm::Optional<MemberFuncMetadata> member_func_metadata;
  if (auto* method_decl =
          clang::dyn_cast<clang::CXXMethodDecl>(function_decl)) {
    switch (method_decl->getAccess()) {
      case clang::AS_public:
        break;
      case clang::AS_protected:
      case clang::AS_private:
      case clang::AS_none:
        // No need for IR to include Func representing private methods.
        // TODO(lukasza): Revisit this for protected methods.
        return std::nullopt;
    }
    llvm::Optional<MemberFuncMetadata::InstanceMethodMetadata>
        instance_metadata;
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
          .is_explicit_ctor = false,
      };
      if (auto* ctor_decl =
              clang::dyn_cast<clang::CXXConstructorDecl>(function_decl)) {
        instance_metadata->is_explicit_ctor = ctor_decl->isExplicit();
      }
    }

    member_func_metadata = MemberFuncMetadata{
        .record_id = GenerateItemId(method_decl->getParent()),
        .instance_method_metadata = instance_metadata};
  }

  if (!errors.empty()) {
    return ImportUnsupportedItem(function_decl, errors);
  }

  bool has_c_calling_convention =
      function_decl->getType()->getAs<clang::FunctionType>()->getCallConv() ==
      clang::CC_C;
  std::optional<UnqualifiedIdentifier> translated_name =
      GetTranslatedName(function_decl);

  // Silence ClangTidy, checked above: calling `add_error` if
  // `!return_type.ok()` and returning early if `!errors.empty()`.
  CRUBIT_CHECK(return_type.ok());

  if (translated_name.has_value()) {
    return Func{
        .name = *translated_name,
        .owning_target = GetOwningTarget(function_decl),
        .doc_comment = GetComment(function_decl),
        .mangled_name = GetMangledName(function_decl),
        .return_type = *return_type,
        .params = std::move(params),
        .lifetime_params = std::move(lifetime_params),
        .is_inline = function_decl->isInlined(),
        .member_func_metadata = std::move(member_func_metadata),
        .has_c_calling_convention = has_c_calling_convention,
        .source_loc = ConvertSourceLocation(function_decl->getBeginLoc()),
        .id = function_template_decl == nullptr
                  ? GenerateItemId(function_decl)
                  : GenerateItemId(function_template_decl),
    };
  }
  return std::nullopt;
}

BazelLabel Importer::GetOwningTarget(const clang::Decl* decl) const {
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
      return BazelLabel("//:builtin");
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

std::optional<IR::Item> Importer::ImportRecord(
    clang::CXXRecordDecl* record_decl) {
  const clang::DeclContext* decl_context = record_decl->getDeclContext();
  if (decl_context->isFunctionOrMethod()) {
    return std::nullopt;
  }
  if (record_decl->isInjectedClassName()) {
    return std::nullopt;
  }
  if (decl_context->isRecord()) {
    return ImportUnsupportedItem(record_decl,
                                 "Nested classes are not supported yet");
  }
  // Make sure the record has a definition that we'll be able to call
  // ASTContext::getASTRecordLayout() on.
  record_decl = record_decl->getDefinition();
  if (!record_decl || record_decl->isInvalidDecl() ||
      !record_decl->isCompleteDefinition()) {
    return std::nullopt;
  }

  // To compute the memory layout of the record, it needs to be a concrete type,
  // not a template.
  if (record_decl->getDescribedClassTemplate() ||
      clang::isa<clang::ClassTemplateSpecializationDecl>(record_decl)) {
    return ImportUnsupportedItem(record_decl,
                                 "Class templates are not supported yet");
  }

  sema_.ForceDeclarationOfImplicitMembers(record_decl);

  const clang::ASTRecordLayout& layout = ctx_.getASTRecordLayout(record_decl);

  llvm::Optional<size_t> base_size;
  bool override_alignment = record_decl->hasAttr<clang::AlignedAttr>();
  if (record_decl->getNumBases() != 0) {
    // The size of the base class subobjects is easy to compute, so long as we
    // know that fields start after the base class subobjects. (This is not
    // guaranteed by the standard, but is true on the ABIs we work with.)
    base_size = layout.getFieldCount() == 0
                    ? static_cast<size_t>(layout.getDataSize().getQuantity())
                    : layout.getFieldOffset(0) / 8;
    // Ideally, we'd only include an alignment adjustment if one of the base
    // classes is more-aligned than any of the fields, but it is simpler do it
    // whenever there are any base classes at all.
    override_alignment = true;
  }

  std::optional<Identifier> record_name = GetTranslatedIdentifier(record_decl);
  if (!record_name.has_value()) {
    return std::nullopt;
  }

  absl::StatusOr<std::vector<Field>> fields = ImportFields(record_decl);
  if (!fields.ok()) {
    return ImportUnsupportedItem(record_decl, fields.status().ToString());
  }

  for (const Field& field : *fields) {
    if (field.is_no_unique_address) {
      override_alignment = true;
      break;
    }
  }

  type_mapper_.Insert(record_decl);

  auto item_ids = GetItemIdsInSourceOrder(record_decl);
  return Record{
      .rs_name = std::string(record_name->Ident()),
      .cc_name = std::string(record_name->Ident()),
      .id = GenerateItemId(record_decl),
      .owning_target = GetOwningTarget(record_decl),
      .doc_comment = GetComment(record_decl),
      .unambiguous_public_bases = GetUnambiguousPublicBases(*record_decl),
      .fields = *std::move(fields),
      .size = layout.getSize().getQuantity(),
      .alignment = layout.getAlignment().getQuantity(),
      .base_size = base_size,
      .override_alignment = override_alignment,
      .copy_constructor = GetCopyCtorSpecialMemberFunc(*record_decl),
      .move_constructor = GetMoveCtorSpecialMemberFunc(*record_decl),
      .destructor = GetDestructorSpecialMemberFunc(*record_decl),
      .is_trivial_abi = record_decl->canPassInRegisters(),
      .is_inheritable =
          !record_decl->isEffectivelyFinal() && !record_decl->isUnion(),
      .is_union = record_decl->isUnion(),
      .child_item_ids = std::move(item_ids)};
}

std::optional<IR::Item> Importer::ImportEnum(clang::EnumDecl* enum_decl) {
  std::optional<Identifier> enum_name = GetTranslatedIdentifier(enum_decl);
  if (!enum_name.has_value()) {
    // TODO(b/208945197): This corresponds to an unnamed enum declaration like
    // `enum { kFoo = 1 }`, which only exists to provide constants into the
    // surrounding scope and doesn't actually introduce an enum namespace. It
    // seems like it should probably be handled with other constants.
    return ImportUnsupportedItem(enum_decl,
                                 "Unnamed enums are not supported yet");
  }

  clang::QualType cc_type = enum_decl->getIntegerType();
  if (cc_type.isNull()) {
    // According to https://clang.llvm.org/doxygen/classclang_1_1EnumDecl.html,
    // getIntegerType "returns a null QualType for an enum forward definition
    // with no fixed underlying type." The same page implies that this can't
    // occur in C++ nor in standard C, but clang supports enums like this
    // in C "as an extension".
    return ImportUnsupportedItem(
        enum_decl,
        "Forward declared enums without type specifiers are not supported");
  }
  std::optional<clang::tidy::lifetimes::ValueLifetimes> no_lifetimes;
  absl::StatusOr<MappedType> type =
      type_mapper_.ConvertQualType(cc_type, no_lifetimes);
  if (!type.ok()) {
    return ImportUnsupportedItem(enum_decl, type.status().ToString());
  }

  std::vector<Enumerator> enumerators;
  enumerators.reserve(std::distance(enum_decl->enumerators().begin(),
                                    enum_decl->enumerators().end()));
  for (clang::EnumConstantDecl* enumerator : enum_decl->enumerators()) {
    std::optional<Identifier> enumerator_name =
        GetTranslatedIdentifier(enumerator);
    if (!enumerator_name.has_value()) {
      // It's not clear that this case is possible
      return ImportUnsupportedItem(
          enum_decl, "importing enum failed: missing enumerator name");
    }

    enumerators.push_back(Enumerator{
        .identifier = *enumerator_name,
        .value = IntegerConstant(enumerator->getInitVal()),
    });
  }

  return Enum{
      .identifier = *enum_name,
      .id = GenerateItemId(enum_decl),
      .owning_target = GetOwningTarget(enum_decl),
      .underlying_type = *std::move(type),
      .enumerators = enumerators,
  };
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

std::optional<IR::Item> Importer::ImportTypedefName(
    clang::TypedefNameDecl* typedef_name_decl) {
  const clang::DeclContext* decl_context = typedef_name_decl->getDeclContext();
  if (decl_context) {
    if (decl_context->isFunctionOrMethod()) {
      return std::nullopt;
    }
    if (decl_context->isRecord()) {
      return ImportUnsupportedItem(
          typedef_name_decl,
          "Typedefs nested in classes are not supported yet");
    }
  }

  clang::QualType type =
      typedef_name_decl->getASTContext().getTypedefType(typedef_name_decl);
  if (MapKnownCcTypeToRsType(type.getAsString()).has_value()) {
    return std::nullopt;
  }

  std::optional<Identifier> identifier =
      GetTranslatedIdentifier(typedef_name_decl);
  CRUBIT_CHECK(identifier.has_value());  // This should never happen.
  std::optional<clang::tidy::lifetimes::ValueLifetimes> no_lifetimes;
  absl::StatusOr<MappedType> underlying_type = type_mapper_.ConvertQualType(
      typedef_name_decl->getUnderlyingType(), no_lifetimes);
  if (underlying_type.ok()) {
    type_mapper_.Insert(typedef_name_decl);
    return TypeAlias{.identifier = *identifier,
                     .id = GenerateItemId(typedef_name_decl),
                     .owning_target = GetOwningTarget(typedef_name_decl),
                     .doc_comment = GetComment(typedef_name_decl),
                     .underlying_type = *underlying_type};
  } else {
    return ImportUnsupportedItem(
        typedef_name_decl, std::string(underlying_type.status().message()));
  }
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

absl::StatusOr<MappedType> Importer::TypeMapper::ConvertTypeDecl(
    const clang::TypeDecl* decl) const {
  if (!known_type_decls_.contains(decl)) {
    return absl::NotFoundError(absl::Substitute(
        "No generated bindings found for '$0'", decl->getNameAsString()));
  }

  ItemId decl_id = GenerateItemId(decl);
  return MappedType::WithDeclId(decl_id);
}

absl::StatusOr<MappedType> Importer::TypeMapper::ConvertType(
    const clang::Type* type,
    std::optional<clang::tidy::lifetimes::ValueLifetimes>& lifetimes,
    bool nullable) const {
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
        CRUBIT_CHECK(type->isLValueReferenceType());
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
      CRUBIT_CHECK(type->isRValueReferenceType());
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
          auto size = ctx_->getTypeSize(builtin_type);
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
  }

  return absl::UnimplementedError(absl::StrCat(
      "Unsupported clang::Type class '", type->getTypeClassName(), "'"));
}

absl::StatusOr<MappedType> Importer::TypeMapper::ConvertQualType(
    clang::QualType qual_type,
    std::optional<clang::tidy::lifetimes::ValueLifetimes>& lifetimes,
    bool nullable) const {
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

absl::StatusOr<std::vector<Field>> Importer::ImportFields(
    clang::CXXRecordDecl* record_decl) {
  // Provisionally assume that we know this RecordDecl so that we'll be able
  // to import fields whose type contains the record itself.
  TypeMapper temp_import_mapper(type_mapper_);
  temp_import_mapper.Insert(record_decl);

  clang::AccessSpecifier default_access =
      record_decl->isClass() ? clang::AS_private : clang::AS_public;
  std::vector<Field> fields;
  const clang::ASTRecordLayout& layout = ctx_.getASTRecordLayout(record_decl);
  for (const clang::FieldDecl* field_decl : record_decl->fields()) {
    std::optional<clang::tidy::lifetimes::ValueLifetimes> no_lifetimes;
    auto type =
        temp_import_mapper.ConvertQualType(field_decl->getType(), no_lifetimes);
    if (!type.ok()) {
      return absl::UnimplementedError(absl::Substitute(
          "Type of field '$0' is not supported: $1",
          field_decl->getNameAsString(), type.status().message()));
    }
    clang::AccessSpecifier access = field_decl->getAccess();
    if (access == clang::AS_none) {
      access = default_access;
    }

    std::optional<Identifier> field_name = GetTranslatedIdentifier(field_decl);
    if (!field_name.has_value()) {
      return absl::UnimplementedError(
          absl::Substitute("Cannot translate name for field '$0'",
                           field_decl->getNameAsString()));
    }
    fields.push_back(
        {.identifier = *std::move(field_name),
         .doc_comment = GetComment(field_decl),
         .type = *type,
         .access = TranslateAccessSpecifier(access),
         .offset = layout.getFieldOffset(field_decl->getFieldIndex()),
         .is_no_unique_address =
             field_decl->hasAttr<clang::NoUniqueAddressAttr>()});
  }
  return fields;
}

std::string Importer::GetMangledName(const clang::NamedDecl* named_decl) const {
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

std::optional<UnqualifiedIdentifier> Importer::GetTranslatedName(
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
    case clang::DeclarationName::CXXOperatorName:
      switch (named_decl->getDeclName().getCXXOverloadedOperator()) {
        case clang::OO_None:
          CRUBIT_CHECK(false &&
                       "No OO_None expected under CXXOperatorName branch");
          return std::nullopt;
        case clang::NUM_OVERLOADED_OPERATORS:
          CRUBIT_CHECK(false &&
                       "No NUM_OVERLOADED_OPERATORS expected at runtime");
          return std::nullopt;
          // clang-format off
        #define OVERLOADED_OPERATOR(name, spelling, ...)  \
        case clang::OO_##name: {                          \
          return {Operator(spelling)};                    \
        }
        #include "third_party/llvm/llvm-project/clang/include/clang/Basic/OperatorKinds.def"
        #undef OVERLOADED_OPERATOR
          // clang-format on
      }
      CRUBIT_CHECK(false && "The `switch` above should handle all cases");
      return std::nullopt;
    default:
      // To be implemented later: CXXConversionFunctionName.
      // There are also e.g. literal operators, deduction guides, etc., but
      // we might not need to implement them at all. Full list at:
      // https://clang.llvm.org/doxygen/classclang_1_1DeclarationName.html#a9ab322d434446b43379d39e41af5cbe3
      return std::nullopt;
  }
}

std::vector<BaseClass> Importer::GetUnambiguousPublicBases(
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
      if (paths.isAmbiguous(ctx_.getCanonicalType(base))) {
        continue;
      }

      clang::CXXRecordDecl* base_record_decl =
          CRUBIT_DIE_IF_NULL(base_specifier.getType()->getAsCXXRecordDecl());
      if (!type_mapper_.ConvertTypeDecl(base_record_decl).status().ok()) {
        continue;
      }

      llvm::Optional<int64_t> offset = {0};
      for (const clang::CXXBasePathElement& base_path_element : path) {
        if (base_path_element.Base->isVirtual()) {
          offset.reset();
          break;
        }
        *offset +=
            {ctx_.getASTRecordLayout(base_path_element.Class)
                 .getBaseClassOffset(CRUBIT_DIE_IF_NULL(
                     base_path_element.Base->getType()->getAsCXXRecordDecl()))
                 .getQuantity()};
      }
      CRUBIT_CHECK((!offset.hasValue() || *offset >= 0) &&
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

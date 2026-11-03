// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importer.h"

#include <stdint.h>

#include <cassert>
#include <cstddef>
#include <functional>
#include <iterator>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <tuple>
#include <utility>
#include <variant>
#include <vector>

#include "absl/algorithm/container.h"
#include "absl/base/no_destructor.h"
#include "absl/base/nullability.h"
#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/log.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_join.h"
#include "absl/strings/str_split.h"
#include "absl/strings/string_view.h"
#include "absl/strings/strip.h"
#include "common/annotation_reader.h"
#include "common/check.h"
#include "common/status_macros.h"
#include "common/string_view_conversion.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "nullability/pragma.h"
#include "rs_bindings_from_cc/annotations_consumer.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/clang_compat_macros.h"
#include "rs_bindings_from_cc/cmdline_flags.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/importers/class_template.h"
#include "rs_bindings_from_cc/importers/cxx_record.h"
#include "rs_bindings_from_cc/importers/enum.h"
#include "rs_bindings_from_cc/importers/enum_constant.h"
#include "rs_bindings_from_cc/importers/existing_rust_type.h"
#include "rs_bindings_from_cc/importers/friend.h"
#include "rs_bindings_from_cc/importers/function.h"
#include "rs_bindings_from_cc/importers/function_template.h"
#include "rs_bindings_from_cc/importers/namespace.h"
#include "rs_bindings_from_cc/importers/type_alias.h"
#include "rs_bindings_from_cc/importers/var.h"
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
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeBase.h"
#include "clang/Basic/ABI.h"
#include "clang/Basic/AttrKinds.h"
#include "clang/Basic/CommentOptions.h"
#include "clang/Basic/Diagnostic.h"
#include "clang/Basic/FileEntry.h"
#include "clang/Basic/FileManager.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/OperatorKinds.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Basic/TokenKinds.h"
#include "clang/Lex/HeaderSearch.h"
#include "clang/Lex/HeaderSearchOptions.h"
#include "clang/Lex/Lexer.h"
#include "clang/Lex/Preprocessor.h"
#include "clang/Sema/Sema.h"
#include "clang/UnifiedSymbolResolution/USRGeneration.h"
#include "llvm/ADT/DenseSet.h"
#include "llvm/ADT/STLExtras.h"
#include "llvm/Support/Casting.h"
#include "llvm/Support/FormatVariadic.h"
#include "llvm/Support/Path.h"
#include "llvm/Support/Regex.h"
#include "llvm/Support/SaveAndRestore.h"
#include "llvm/Support/raw_ostream.h"

namespace crubit {

namespace ir_proto = ::crubit::ffi;

namespace {

constexpr absl::string_view kTypeStatusPayloadUrl =
    "type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type";

// Prefix of the synthetic label that `Importer::GetOwningTarget` returns when
// no Bazel target visible to Crubit claims ownership of a header. The rest of
// the label is the header path, which is useful for error messages.
constexpr absl::string_view kUnknownTargetPrefix = "//_unknown_target:";

// Returns the header path if `label` is a synthetic unknown-target label (see
// `kUnknownTargetPrefix`), or `std::nullopt` if it is a real Bazel label.
std::optional<absl::string_view> HeaderOfUnknownTarget(
    const BazelLabel& label) {
  absl::string_view value = label.value();
  if (!absl::ConsumePrefix(&value, kUnknownTargetPrefix)) {
    return std::nullopt;
  }
  return value;
}

// Checks if the return value from `GetDeclItem` indicates that the import was
// successful.
absl::Status CheckImportStatus(const ir_proto::Item* item) {
  if (item == nullptr) {
    return absl::InvalidArgumentError("The import has been skipped");
  }
  if (item->has_unsupported_item()) {
    const auto& unsupported = item->unsupported_item();
    std::vector<absl::string_view> messages;
    messages.reserve(unsupported.errors().size());
    for (const auto& error : unsupported.errors()) {
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

// Returns true if `type` is a supported element type for fixed-size C arrays
// (currently non-void primitives and enums, including nested arrays).
bool IsSupportedArrayElementType(const clang::ASTContext& ctx,
                                 clang::QualType type) {
  while (const auto* inner_array = ctx.getAsArrayType(type)) {
    type = inner_array->getElementType();
  }
  return (type->isBuiltinType() && !type->isVoidType()) ||
         type->isEnumeralType();
}

bool IsTypeValueOrRefToConst(clang::QualType candidate,
                             clang::CanQualType target) {
  return candidate->getCanonicalTypeUnqualified() == target ||
         (candidate->isLValueReferenceType() &&
          candidate.getNonReferenceType().getCanonicalType() ==
              target.withConst());
}

bool IsLvalueRefToUnqualified(clang::QualType type) {
  return type->isLValueReferenceType() &&
         !type.getNonReferenceType().getCanonicalType().hasQualifiers();
}

bool IsStdTemplate(const clang::TemplateDecl& decl, clang::StringRef name) {
  const auto* class_template =
      clang::dyn_cast<const clang::ClassTemplateDecl>(&decl);
  return class_template != nullptr && class_template->getName() == name &&
         class_template->isInStdNamespace();
}

bool IsCharTraitsChar(clang::QualType type) {
  const clang::TemplateSpecializationType* specialization =
      type->getAsNonAliasTemplateSpecializationType();
  return specialization != nullptr &&
         IsStdTemplate(*specialization->getTemplateName().getAsTemplateDecl(),
                       "char_traits") &&
         specialization->template_arguments().size() == 1 &&
         specialization->template_arguments()[0].getAsType()->isCharType();
}

bool IsOstreamRef(clang::QualType type) {
  if (!IsLvalueRefToUnqualified(type)) {
    return false;
  }
  const clang::TemplateSpecializationType* specialization =
      type.getNonReferenceType()->getAsNonAliasTemplateSpecializationType();
  return specialization != nullptr &&
         IsStdTemplate(*specialization->getTemplateName().getAsTemplateDecl(),
                       "basic_ostream") &&
         !specialization->template_arguments().empty() &&
         specialization->template_arguments()[0].getAsType()->isCharType() &&
         (specialization->template_arguments().size() == 1 ||
          IsCharTraitsChar(
              specialization->template_arguments()[1].getAsType()));
}

bool DetectFormatterFunction(const clang::Decl& decl,
                             clang::CanQualType formatted_type) {
  if (const auto* function_template_decl =
          clang::dyn_cast<const clang::FunctionTemplateDecl>(&decl);
      function_template_decl != nullptr) {
    return function_template_decl->getDeclName().isIdentifier() &&
           function_template_decl->getName() == "AbslStringify" &&
           function_template_decl->getAsFunction()->getNumParams() == 2 &&
           IsTypeValueOrRefToConst(
               /*candidate=*/function_template_decl->getAsFunction()
                   ->getParamDecl(1)
                   ->getType(),
               /*target=*/formatted_type);
  }
  if (const auto* function_decl =
          clang::dyn_cast<const clang::FunctionDecl>(&decl);
      function_decl != nullptr) {
    return function_decl->getOverloadedOperator() == clang::OO_LessLess &&
           function_decl->getNumParams() == 2 &&
           IsOstreamRef(function_decl->getParamDecl(0)->getType()) &&
           IsTypeValueOrRefToConst(
               /*candidate=*/function_decl->getParamDecl(1)->getType(),
               /*target=*/formatted_type) &&
           IsOstreamRef(function_decl->getReturnType());
  }
  return false;
}

}  // namespace

namespace {
template <typename T>
const T* absl_nullable FindDecl(const clang::DeclContext* absl_nonnull context,
                                absl::string_view name) {
  if (const auto* absl_nullable ns =
          llvm::dyn_cast<clang::NamespaceDecl>(context);
      ns != nullptr) {
    for (const auto* absl_nonnull redecl : ns->redecls()) {
      for (const clang::Decl* absl_nonnull decl : redecl->decls()) {
        if (const auto* absl_nullable type = llvm::dyn_cast<T>(decl);
            type != nullptr) {
          if (StringViewFromStringRef(type->getName()) == name) {
            return type;
          }
        }
      }
    }
  } else {
    for (const clang::Decl* absl_nonnull decl : context->decls()) {
      if (const auto* absl_nullable type = llvm::dyn_cast<T>(decl);
          type != nullptr) {
        if (StringViewFromStringRef(type->getName()) == name) {
          return type;
        }
      }
    }
  }
  return nullptr;
}

// Returns the type of `rs::core::fmt::Debug`, or a null `QualType` if it is not
// declared in the translation unit.  Most translation units don't declare it
// (only headers that include `support/rs_std/traits.h` do), so its absence is
// not an error.
clang::QualType LookupRsCoreFmtDebug(
    const clang::ASTContext* absl_nonnull context) {
  const clang::TranslationUnitDecl* translation_unit =
      context->getTranslationUnitDecl();
  const clang::NamespaceDecl* rs =
      FindDecl<clang::NamespaceDecl>(translation_unit, "rs");
  if (rs == nullptr) {
    return clang::QualType();
  }
  const clang::NamespaceDecl* core = FindDecl<clang::NamespaceDecl>(rs, "core");
  if (core == nullptr) {
    return clang::QualType();
  }
  const clang::NamespaceDecl* fmt = FindDecl<clang::NamespaceDecl>(core, "fmt");
  if (fmt == nullptr) {
    return clang::QualType();
  }
  const clang::TypeDecl* debug = FindDecl<clang::TypeDecl>(fmt, "Debug");
  if (debug == nullptr) {
    return clang::QualType();
  }
  return context->getTypeDeclType(debug);
}

// Returns the canonical declaration of `<ns_name>::impl`, or null if it is not
// declared in the translation unit.  Most translation units don't declare it
// (only headers that include `support/rs_std/traits.h` do), so its absence is
// not an error.
const clang::ClassTemplateDecl* absl_nullable LookupCanonicalImplInNamespace(
    const clang::ASTContext* absl_nonnull context, absl::string_view ns_name) {
  const clang::TranslationUnitDecl* translation_unit =
      context->getTranslationUnitDecl();
  const clang::NamespaceDecl* ns =
      FindDecl<clang::NamespaceDecl>(translation_unit, ns_name);
  if (ns == nullptr) {
    return nullptr;
  }
  const clang::ClassTemplateDecl* impl =
      FindDecl<clang::ClassTemplateDecl>(ns, "impl");
  if (impl == nullptr) {
    return nullptr;
  }
  return impl->getCanonicalDecl();
}

}  // namespace

Importer::Importer(Invocation& invocation, clang::ASTContext& ctx,
                   clang::Sema& sema)
    : ImportContext(invocation, ctx, sema),
      mangler_(CRUBIT_DIE_IF_NULL(ctx_.createMangleContext())),
      itanium_tag_name_mangler_(CRUBIT_DIE_IF_NULL(
          clang::ItaniumMangleContext::create(ctx_, ctx_.getDiagnostics()))),
      module_comments_(ctx.getSourceManager()),
      rs_core_fmt_debug_(LookupRsCoreFmtDebug(&ctx)),
      rs_impl_(LookupCanonicalImplInNamespace(&ctx, "rs")),
      rs_std_impl_(LookupCanonicalImplInNamespace(&ctx, "rs_std")) {
  decl_importers_.push_back(std::make_unique<ExistingRustTypeImporter>(*this));
  decl_importers_.push_back(std::make_unique<ClassTemplateDeclImporter>(*this));
  decl_importers_.push_back(std::make_unique<CXXRecordDeclImporter>(*this));
  decl_importers_.push_back(std::make_unique<EnumDeclImporter>(*this));
  decl_importers_.push_back(std::make_unique<EnumConstantDeclImporter>(*this));
  decl_importers_.push_back(std::make_unique<VarDeclImporter>(*this));
  decl_importers_.push_back(std::make_unique<FriendDeclImporter>(*this));
  decl_importers_.push_back(std::make_unique<FunctionDeclImporter>(*this));
  decl_importers_.push_back(
      std::make_unique<FunctionTemplateDeclImporter>(*this));
  decl_importers_.push_back(std::make_unique<NamespaceDeclImporter>(*this));
  decl_importers_.push_back(std::make_unique<TypeAliasImporter>(*this));
}

// Multiple IR items can be associated with the same source location (e.g. the
// implicitly defined constructors and assignment operators). To produce
// deterministic output, we order such items based on GetDeclOrder.  The order
// is somewhat arbitrary, but we still try to make it aesthetically pleasing
// (e.g. constructors go before assignment operators;  default constructor goes
// first, etc.).
static int GetDeclOrder(const clang::Decl& decl) {
  if (clang::isa<clang::RecordDecl>(&decl)) {
    return decl.getDeclContext()->isRecord() ? 101 : 100;
  }

  if (auto* ctor = clang::dyn_cast<clang::CXXConstructorDecl>(&decl)) {
    return ctor->isDefaultConstructor() ? 202
           : ctor->isCopyConstructor()  ? 203
           : ctor->isMoveConstructor()  ? 204
                                        : 299;
  }

  if (clang::isa<clang::CXXDestructorDecl>(&decl)) {
    return 306;
  }

  if (auto* method = clang::dyn_cast<clang::CXXMethodDecl>(&decl)) {
    return method->isCopyAssignmentOperator()   ? 401
           : method->isMoveAssignmentOperator() ? 402
                                                : 499;
  }

  return 999;
}

// Returns the name of the file and the offset in it of `location`, which must
// be a valid file location.
static std::pair<llvm::StringRef, unsigned> GetFileNameAndOffset(
    const clang::SourceManager& sm, clang::SourceLocation location) {
  auto [file_id, offset] = sm.getDecomposedLoc(location);
  // Not `getBufferName`, which loads the contents of the file. Buffers that
  // aren't files (e.g. `<built-in>`) have no name here, and come first.
  llvm::StringRef name;
  if (clang::OptionalFileEntryRef file = sm.getFileEntryRefForID(file_id)) {
    name = file->getName();
    name.consume_front("./");
  }
  return {name, offset};
}

// Returns a key for the position of the loaded `location`, which doesn't depend
// on the order in which precompiled header modules were loaded: the file names
// and offsets of the include stack of the expansion location within its module,
// from the outermost file (the root file of the module) to the innermost one,
// followed by the file name and offset of the spelling location (as a tie
// breaker, e.g. for several declarations generated by the same macro
// expansion). Buffers that aren't files are skipped.
//
// The include stack stops at the root file of the module: the "include
// location" of the root file of a module is where the module was imported,
// i.e. in whichever module happened to import it first, which depends on the
// order of the `-fmodule-file` flags.
//
// Comparing these keys lexicographically orders locations in the same file by
// offset, and a location in an included file like the `#include` directive, so
// it is the same as the order in the translation unit for locations in the same
// tree of includes in the same module.
static std::vector<std::pair<llvm::StringRef, unsigned>> GetLoadedPositionKey(
    const clang::SourceManager& sm, clang::SourceLocation location) {
  std::vector<std::pair<llvm::StringRef, unsigned>> key;
  clang::SourceLocation loc = sm.getExpansionLoc(location);
  llvm::StringRef module_name = sm.getModuleImportLoc(loc).second;
  while (loc.isValid()) {
    std::pair<llvm::StringRef, unsigned> name_and_offset =
        GetFileNameAndOffset(sm, loc);
    if (!name_and_offset.first.empty()) key.push_back(name_and_offset);
    loc = sm.getExpansionLoc(sm.getIncludeLoc(sm.getFileID(loc)));
    if (loc.isValid() && (!sm.isLoadedSourceLocation(loc) ||
                          sm.getModuleImportLoc(loc).second != module_name)) {
      break;
    }
  }
  absl::c_reverse(key);
  key.push_back(GetFileNameAndOffset(sm, sm.getSpellingLoc(location)));
  return key;
}

// Returns whether `a` comes before `b` in the order of the generated bindings.
// Invalid locations come first.
//
// This is `isBeforeInTranslationUnit`, except for locations in precompiled
// header modules: for locations in two different modules,
// `isBeforeInTranslationUnit` compares the order in which the modules were
// loaded, which depends on the order of the `-fmodule-file` flags, and on which
// other modules happen to be loaded. Instead, locations in modules are compared
// by `GetLoadedPositionKey`. All locations in modules come before all locations
// that were parsed textually. This is a strict weak order. Without modules, it
// is the same as `isBeforeInTranslationUnit`.
static bool IsBeforeInSourceOrder(const clang::SourceManager& sm,
                                  clang::SourceLocation a,
                                  clang::SourceLocation b) {
  if (!a.isValid() || !b.isValid()) return !a.isValid() && b.isValid();
  bool a_is_loaded = sm.isLoadedSourceLocation(a);
  bool b_is_loaded = sm.isLoadedSourceLocation(b);
  if (a_is_loaded != b_is_loaded) return a_is_loaded;
  if (!a_is_loaded) return sm.isBeforeInTranslationUnit(a, b);
  if (a == b) return false;
  return GetLoadedPositionKey(sm, a) < GetLoadedPositionKey(sm, b);
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
      // Not `!=`: two different locations in different copies of the same
      // header (from different modules) are equivalent.
      if (IsBeforeInSourceOrder(sm, source_range_.getBegin(),
                                other.source_range_.getBegin())) {
        return true;
      }
      if (IsBeforeInSourceOrder(sm, other.source_range_.getBegin(),
                                source_range_.getBegin())) {
        return false;
      }
      if (IsBeforeInSourceOrder(sm, source_range_.getEnd(),
                                other.source_range_.getEnd())) {
        return true;
      }
      if (IsBeforeInSourceOrder(sm, other.source_range_.getEnd(),
                                source_range_.getEnd())) {
        return false;
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

using RepresentativeCopyCache = Importer::RepresentativeCopyCache;

// Returns the redeclaration of `decl` that is at the same position (file and
// offset) as `decl` and that was parsed textually, if there is one, or else the
// one that comes first in the translation unit.
//
// When a header is compiled into several precompiled header modules (e.g. a
// standard library header, which is a textual header of every module that
// includes it), or is both compiled into a module and parsed textually (e.g. a
// header of the current target that is also a header of a dependency), each
// copy of the header contains its own copy of the declarations in the header,
// at different `SourceLocation`s. Clang merges the copies into one
// redeclaration chain. This function maps all copies of a declaration to the
// same representative copy: preferably the textual one, which is where the
// comments are (precompiled modules don't contain ordinary comments), or else
// the one from the first module in the translation unit, which is where the
// header would have been first included without modules.
//
// The results are memoized in `cache`, for all the redeclarations of `decl` at
// once: a namespace can have thousands of redeclarations (one per copy of each
// header that reopens it), and this is called for each of them, so computing
// each result separately would take quadratic time.
static const clang::Decl* absl_nonnull GetRepresentativeCopy(
    const clang::Decl* absl_nonnull decl, const clang::SourceManager& sm,
    RepresentativeCopyCache& cache) {
  if (auto it = cache.find(decl); it != cache.end()) return it->second;

  using Position = std::pair<const clang::FileEntry*, unsigned>;
  auto get_position = [&sm](const clang::Decl* d) -> std::optional<Position> {
    auto [file_id, offset] = sm.getDecomposedExpansionLoc(d->getLocation());
    clang::OptionalFileEntryRef file = sm.getFileEntryRefForID(file_id);
    if (!file.has_value()) return std::nullopt;
    return std::make_pair(&file->getFileEntry(), offset);
  };
  // Returns whether `a` is a better representative than `b`: textual copies
  // come first, then copies that come first in the translation unit.
  auto is_better = [&sm](const clang::Decl* a, const clang::Decl* b) {
    bool a_is_textual = !a->isFromASTFile();
    bool b_is_textual = !b->isFromASTFile();
    if (a_is_textual != b_is_textual) return a_is_textual;
    return IsBeforeInSourceOrder(sm, a->getLocation(), b->getLocation());
  };

  // Group the redeclarations by position, and find the best one of each group.
  // Redeclarations without a position are their own representative.
  absl::flat_hash_map<Position, const clang::Decl*> best_at_position;
  std::vector<std::pair<const clang::Decl*, std::optional<Position>>> positions;
  for (const clang::Decl* redecl : decl->redecls()) {
    std::optional<Position> position = get_position(redecl);
    positions.push_back({redecl, position});
    if (!position.has_value()) continue;
    auto [it, inserted] = best_at_position.try_emplace(*position, redecl);
    if (!inserted && is_better(redecl, it->second)) it->second = redecl;
  }
  for (const auto& [redecl, position] : positions) {
    cache[redecl] =
        position.has_value() ? best_at_position.at(*position) : redecl;
  }
  // `decl` is always one of its own redeclarations, but don't rely on it.
  auto [it, _] = cache.try_emplace(decl, decl);
  return it->second;
}

// Returns the copy of `decl` whose source locations should be used for finding
// comments and for ordering. See `GetRepresentativeCopy`.
static const clang::Decl& GetDeclForSourceLocations(
    const clang::Decl& decl, const clang::ASTContext& ast_context,
    RepresentativeCopyCache& cache) {
  // Without precompiled header modules there is only one copy of each
  // declaration.
  if (ast_context.getExternalSource() == nullptr) return decl;
  return *GetRepresentativeCopy(&decl, ast_context.getSourceManager(), cache);
}

// Returns the source range used for ordering `decl`.
//
// With header modules, `decl` may be a copy of a declaration that is not where
// the comments are, so we use the range of the representative copy instead (see
// `GetRepresentativeCopy`).
//
// For an implicit instantiation, `getSourceRange()` is the range of the
// template (or partial specialization) that it was instantiated from. With
// header modules, instantiations of the same template may refer to different
// copies of it, so we use the range of the representative copy of the template
// instead. Otherwise the order of the instantiations would depend on which
// module each of them was loaded from.
static clang::SourceRange GetSourceRangeForOrdering(
    const clang::Decl& decl, const clang::ASTContext& ast_context,
    RepresentativeCopyCache& cache) {
  // Without precompiled header modules there is only one copy of each
  // declaration.
  if (ast_context.getExternalSource() == nullptr) return decl.getSourceRange();
  const auto* spec =
      clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(&decl);
  if (spec == nullptr ||
      clang::isa<clang::ClassTemplatePartialSpecializationDecl>(spec) ||
      (spec->getSpecializationKind() != clang::TSK_ImplicitInstantiation &&
       spec->getSpecializationKind() != clang::TSK_Undeclared)) {
    return GetDeclForSourceLocations(decl, ast_context, cache).getSourceRange();
  }
  auto pattern = spec->getSpecializedTemplateOrPartial();
  const clang::Decl* pattern_decl = nullptr;
  if (auto* partial =
          pattern.dyn_cast<clang::ClassTemplatePartialSpecializationDecl*>()) {
    pattern_decl = partial;
  } else {
    pattern_decl = pattern.dyn_cast<clang::ClassTemplateDecl*>();
  }
  if (pattern_decl == nullptr) return decl.getSourceRange();
  return GetRepresentativeCopy(pattern_decl, ast_context.getSourceManager(),
                               cache)
      ->getSourceRange();
}

// Returns the locations where Clang looks for the comment attached to `decl`.
//
// This is a copy of `getLocsForCommentSearch` in clang/lib/AST/ASTContext.cpp
// (minus Objective-C), which is not exposed by Clang.
static llvm::SmallVector<clang::SourceLocation, 2> GetLocsForCommentSearch(
    const clang::Decl& decl, const clang::SourceManager& sm) {
  // User can not attach documentation to implicit declarations.
  if (decl.isImplicit()) return {};

  // User can not attach documentation to implicit instantiations.
  if (const auto* function = clang::dyn_cast<clang::FunctionDecl>(&decl)) {
    if (function->getTemplateSpecializationKind() ==
        clang::TSK_ImplicitInstantiation) {
      return {};
    }
  }
  if (const auto* var = clang::dyn_cast<clang::VarDecl>(&decl)) {
    if (var->isStaticDataMember() && var->getTemplateSpecializationKind() ==
                                         clang::TSK_ImplicitInstantiation) {
      return {};
    }
  }
  if (const auto* record = clang::dyn_cast<clang::CXXRecordDecl>(&decl)) {
    if (record->getTemplateSpecializationKind() ==
        clang::TSK_ImplicitInstantiation) {
      return {};
    }
  }
  if (const auto* spec =
          clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(&decl)) {
    clang::TemplateSpecializationKind kind = spec->getSpecializationKind();
    if (kind == clang::TSK_ImplicitInstantiation ||
        kind == clang::TSK_Undeclared) {
      return {};
    }
  }
  if (const auto* enum_decl = clang::dyn_cast<clang::EnumDecl>(&decl)) {
    if (enum_decl->getTemplateSpecializationKind() ==
        clang::TSK_ImplicitInstantiation) {
      return {};
    }
  }
  if (const auto* tag = clang::dyn_cast<clang::TagDecl>(&decl)) {
    // When tag declaration (but not definition!) is part of the
    // decl-specifier-seq of some other declaration, it doesn't get comment
    if (tag->isEmbeddedInDeclarator() && !tag->isCompleteDefinition()) {
      return {};
    }
  }
  if (clang::isa<clang::ParmVarDecl, clang::TemplateTypeParmDecl,
                 clang::NonTypeTemplateParmDecl,
                 clang::TemplateTemplateParmDecl>(&decl)) {
    return {};
  }

  // Declarations with multiple declarators are common, so we use the location
  // of the identifier as the location of the declaration, except for these
  // kinds of declarations.
  clang::SourceLocation base_location =
      clang::isa<clang::RedeclarableTemplateDecl,
                 clang::ClassTemplateSpecializationDecl, clang::TypedefDecl>(
          &decl)
          ? decl.getBeginLoc()
          : decl.getLocation();

  llvm::SmallVector<clang::SourceLocation, 2> locations;
  if (!decl.getLocation().isMacroID()) {
    locations.push_back(base_location);
  } else {
    // For declarations generated by a macro (that are not contained by another
    // declaration in the macro), look first at the expansion site, and then
    // inside the macro.
    const clang::DeclContext* decl_context = decl.getDeclContext();
    if (!(decl_context && clang::Decl::castFromDeclContext(decl_context)
                              ->getLocation()
                              .isMacroID())) {
      locations.push_back(sm.getExpansionLoc(base_location));
    }
    locations.push_back(sm.getSpellingLoc(decl.getBeginLoc()));
  }
  return locations;
}

Importer::SourceOrderKey Importer::GetSourceOrderKey(
    const clang::Decl& decl) const {
  return SourceOrderKey(
      GetSourceRangeForOrdering(decl, ctx_, representative_copy_cache_),
      GetDeclOrder(decl), GetNameForSourceOrder(decl));
}

Importer::SourceOrderKey Importer::GetSourceOrderKey(
    const clang::RawComment& comment) const {
  return SourceOrderKey(comment.getSourceRange());
}

class Importer::SourceLocationComparator {
 public:
  bool operator()(const clang::SourceLocation& a,
                  const clang::SourceLocation& b) const {
    return b.isValid() && a.isValid() && IsBeforeInSourceOrder(sm_, a, b);
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
  using OrderedItem = std::pair<SourceOrderKey, const ir_proto::Item*>;

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

// Returns the definitions of `record` that Clang demoted to declarations when
// it merged them into the (kept) definition `record` (see
// `PreferMemberOfKeptDefinition`), in redeclaration order. Empty without header
// modules.
static std::vector<const clang::CXXRecordDecl*> GetDemotedDefinitions(
    const clang::CXXRecordDecl& record) {
  std::vector<const clang::CXXRecordDecl*> result;
  if (record.getASTContext().getExternalSource() == nullptr ||
      !record.isThisDeclarationADefinition()) {
    return result;
  }
  for (const clang::TagDecl* redecl : record.redecls()) {
    if (redecl != &record && redecl->isThisDeclarationADemotedDefinition()) {
      result.push_back(clang::cast<clang::CXXRecordDecl>(redecl));
    }
  }
  return result;
}

std::vector<clang::Decl*> Importer::GetCanonicalChildren(
    const clang::DeclContext& decl_context) const {
  std::vector<clang::Decl*> result;
  const auto* spec_decl =
      llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(&decl_context);
  bool is_always_instantiate =
      spec_decl == nullptr || IsAlwaysInstantiate(*spec_decl);
  // With header modules, the implicit special members of a merged class
  // definition can be lexically declared only in a definition that Clang
  // demoted to a declaration: e.g. a template instantiation whose implicit
  // default constructor was declared in only one of the modules. (Clang merges
  // the flags that say which special members were declared, so it doesn't
  // declare them again in the kept definition.) They are children of the kept
  // definition, too.
  //
  // Only implicit members are taken from demoted definitions: all other members
  // are the same in all copies of the definition (or else there is an ODR
  // violation), so the kept definition has them too. Implicit members that are
  // declared in several definitions are merged into one redeclaration chain,
  // and `CanonicalizeDecl` (below) keeps only one of them.
  //
  // The children are sorted by source order afterwards, so the order in which
  // they are collected here only matters for ties, and it is deterministic.
  std::vector<clang::Decl*> children(decl_context.decls_begin(),
                                     decl_context.decls_end());
  if (const auto* record = llvm::dyn_cast<clang::CXXRecordDecl>(&decl_context);
      record != nullptr) {
    for (const clang::CXXRecordDecl* demoted : GetDemotedDefinitions(*record)) {
      for (clang::Decl* member : demoted->decls()) {
        if (member->isImplicit() && clang::isa<clang::CXXMethodDecl>(member)) {
          children.push_back(member);
        }
      }
    }
    // Implicit special members that were declared in another module's copy of
    // the definition can also be merged into the kept definition's lookup
    // table without being in its `decls()`. Which ones are depends on which
    // copy Clang kept, i.e. on the order in which the modules were loaded, so
    // find them all by name lookup.
    if (record->getASTContext().getExternalSource() != nullptr &&
        record->isThisDeclarationADefinition()) {
      clang::CanQualType type = ctx_.getCanonicalTagType(record);
      const clang::DeclarationName names[] = {
          ctx_.DeclarationNames.getCXXConstructorName(type),
          ctx_.DeclarationNames.getCXXDestructorName(type),
          ctx_.DeclarationNames.getCXXOperatorName(clang::OO_Equal),
      };
      for (const clang::DeclarationName& name : names) {
        for (clang::NamedDecl* member : record->lookup(name)) {
          if (!member->isImplicit() ||
              !clang::isa<clang::CXXMethodDecl>(member))
            continue;
          clang::Decl* canonical = CanonicalizeDecl(member);
          if (canonical != nullptr &&
              !absl::c_linear_search(children, canonical))
            children.push_back(canonical);
        }
      }
    }
  }
  for (clang::Decl* decl : children) {
    if (!is_always_instantiate) {
      if (auto* method_decl = llvm::dyn_cast<clang::CXXMethodDecl>(decl)) {
        if (!llvm::isa<clang::CXXConstructorDecl>(decl) &&
            !llvm::isa<clang::CXXDestructorDecl>(decl) &&
            method_decl->getOverloadedOperator() != clang::OO_Equal) {
          continue;
        }
      }
    }
    // Bubble anonymous enum constants up so that they are imported as if they
    // were top-level constants.
    if (const auto* enum_decl = llvm::dyn_cast<clang::EnumDecl>(decl)) {
      if (enum_decl->getName().empty()) {
        for (clang::EnumConstantDecl* enumerator : enum_decl->enumerators()) {
          // Skip enumerators that are unmerged duplicates of others.
          if (CanonicalizeDecl(enumerator) == enumerator) {
            result.push_back(enumerator);
          }
        }
        continue;
      }
    }

    if (const auto* linkage_spec_decl =
            llvm::dyn_cast<clang::LinkageSpecDecl>(decl)) {
      llvm::move(GetCanonicalChildren(*linkage_spec_decl),
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

    // For the special case of aliases, add their referent as a child if
    // this alias is the preferred name of the template specialization they
    // refer to.
    if (clang::TypedefNameDecl* alias_decl =
            clang::dyn_cast<clang::TypedefNameDecl>(decl)) {
      if (clang::CXXRecordDecl* record_decl =
              alias_decl->getUnderlyingType()->getAsCXXRecordDecl()) {
        if (alias_decl == GetTemplateSpecializationAlias(record_decl)) {
          result.push_back(record_decl);
        }
      }
    }
  }
  return result;
}

// If `decl` is lexically declared in a class definition that Clang demoted to
// a declaration when merging it with an equivalent definition from another
// (header) module, returns the redeclaration of `decl` that is lexically
// declared in the kept definition. Otherwise returns `decl`.
//
// When the same class (e.g. an implicit instantiation like
// `std::allocator<std::string>`) is defined in several modules, Clang keeps one
// definition and merges each member of the other definitions into the
// redeclaration chain of the corresponding member of the kept definition. The
// canonical declaration of a member may thus be lexically inside a demoted
// definition, which we never visit (we only visit the children of the kept
// definition).
static const clang::Decl* absl_nonnull PreferMemberOfKeptDefinition(
    const clang::Decl* absl_nonnull decl) {
  const auto* lexical_record =
      clang::dyn_cast<clang::TagDecl>(decl->getLexicalDeclContext());
  if (lexical_record == nullptr ||
      !lexical_record->isThisDeclarationADemotedDefinition()) {
    return decl;
  }
  const clang::TagDecl* definition = lexical_record->getDefinition();
  if (definition == nullptr) return decl;
  for (const clang::Decl* redecl : decl->redecls()) {
    if (redecl->getLexicalDeclContext() == definition) return redecl;
  }
  return decl;
}

const clang::Decl* absl_nullable Importer::CanonicalizeDecl(
    const clang::Decl& decl) const {
  const clang::Decl* canonical = CanonicalizeDeclInRedeclChain(decl);
  if (canonical == nullptr) return nullptr;
  return CanonicalizeUnmergedDuplicates(canonical);
}

const clang::Decl* absl_nonnull Importer::CanonicalizeUnmergedDuplicates(
    const clang::Decl* absl_nonnull decl) const {
  // Unmerged duplicates can only come from precompiled header modules.
  if (ctx_.getExternalSource() == nullptr) return decl;

  // Clang merges namespaces into one redeclaration chain, and also tags,
  // functions and variables with external linkage. (Tags have no linkage only
  // in anonymous namespaces, which are not supported.)
  //
  // Functions with internal linkage are not merged, but they are not
  // deduplicated here either: both copies are visible in the generated C++
  // code, where taking the address of the function would be ambiguous. Instead,
  // they are treated like overloads, which are not bound.
  const auto* named_decl = clang::dyn_cast<clang::NamedDecl>(decl);
  if (named_decl == nullptr || !named_decl->getDeclName() ||
      clang::isa<clang::NamespaceDecl, clang::TagDecl, clang::FunctionDecl,
                 clang::FunctionTemplateDecl>(decl)) {
    return decl;
  }
  // Only declarations at namespace scope can be duplicated this way (members of
  // merged classes and enums come from the one definition of the class or
  // enum). The exception are enumerators of anonymous enums, which are not
  // merged, and which are imported as if they were declared in the enclosing
  // namespace (see `GetCanonicalChildren`).
  if (const auto* enum_decl =
          clang::dyn_cast<clang::EnumDecl>(named_decl->getDeclContext());
      enum_decl != nullptr && !enum_decl->getName().empty()) {
    return decl;
  }
  const clang::DeclContext* lookup_context =
      named_decl->getDeclContext()->getRedeclContext();
  if (!lookup_context->isFileContext()) return decl;

  if (auto it = unmerged_duplicates_cache_.find(decl);
      it != unmerged_duplicates_cache_.end()) {
    return it->second;
  }

  // Pick a representative among all the declarations of the same entity, in a
  // way that doesn't depend on which of them we started from, nor on the order
  // of the lookup results (which depends on the order in which the modules were
  // loaded): one that was parsed textually if there is one (because
  // declarations loaded from a module may lack comments), and among those, the
  // first one in source order (see `IsBeforeInSourceOrder`). Remaining ties are
  // between copies at the same position in different copies of the same
  // header, which are interchangeable, so they are broken by lookup order.
  const clang::SourceManager& sm = ctx_.getSourceManager();
  auto is_better = [&sm](const clang::Decl* a, const clang::Decl* b) {
    bool a_is_textual = !a->isFromASTFile();
    bool b_is_textual = !b->isFromASTFile();
    if (a_is_textual != b_is_textual) return a_is_textual;
    return IsBeforeInSourceOrder(sm, a->getLocation(), b->getLocation());
  };
  const clang::Decl* best = nullptr;
  bool found_self = false;
  for (const clang::NamedDecl* candidate :
       lookup_context->lookup(named_decl->getDeclName())) {
    if (candidate->isInvalidDecl() ||
        !ctx_.isSameEntity(candidate, named_decl)) {
      continue;
    }
    const clang::Decl* canonical_candidate =
        CanonicalizeDeclInRedeclChain(*candidate);
    if (canonical_candidate == nullptr) continue;
    if (canonical_candidate == decl) found_self = true;
    if (best == nullptr || is_better(canonical_candidate, best)) {
      best = canonical_candidate;
    }
  }
  const clang::Decl* representative = decl;
  // If `decl` itself isn't found by lookup, we can't tell how it relates to the
  // declarations that are, so leave it alone.
  if (found_self) {
    // `best` isn't null, since lookup found `decl` itself.
    representative = CRUBIT_DIE_IF_NULL(best);
  }
  unmerged_duplicates_cache_[decl] = representative;
  return representative;
}

const clang::Decl* absl_nullable Importer::CanonicalizeDeclInRedeclChain(
    const clang::Decl& decl_ref) const {
  const clang::Decl* decl = &decl_ref;
  if (auto* namespace_decl = llvm::dyn_cast<clang::NamespaceDecl>(decl)) {
    return namespace_decl;
  }
  auto is_injected_class_name = [](const clang::Decl* decl) {
    if (auto* cxx_record_decl = llvm::dyn_cast<clang::CXXRecordDecl>(decl)) {
      return cxx_record_decl->isInjectedClassName();
    }
    return false;
  };

  if (auto* alias_decl = clang::dyn_cast<clang::TypedefNameDecl>(decl)) {
    if (clang::CXXRecordDecl* record_decl =
            alias_decl->getUnderlyingType()->getAsCXXRecordDecl()) {
      if (alias_decl == GetTemplateSpecializationAlias(record_decl)) {
        decl = record_decl;
      }
    }
    if (const clang::TagDecl* tag_decl =
            StripCStyleNameIntroducingTypedef(alias_decl)) {
      if (!llvm::isa<clang::ClassTemplateSpecializationDecl>(tag_decl)) {
        decl = const_cast<clang::TagDecl*>(tag_decl);
      }
    }
  }

  if (llvm::isa<clang::TagDecl>(decl)) {
    if (is_injected_class_name(decl)) {
      return nullptr;
    }
    auto owning_target = GetOwningTarget(*decl);
    const auto& source_manager = sema_.getSourceManager();
    clang::Decl* canonical = nullptr;
    for (auto iter = decl->redecls_begin(); iter != decl->redecls_end();
         ++iter) {
      auto* redecl = llvm::dyn_cast<clang::TagDecl>(*iter);
      CRUBIT_CHECK(redecl);
      if (is_injected_class_name(redecl)) {
        continue;
      }
      if (GetOwningTarget(*redecl) != owning_target) {
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
      // Not `isBeforeInTranslationUnit`, which asserts that the locations are
      // valid (they aren't for implicit declarations, e.g. of
      // `std::align_val_t`, which Clang declares in each header module that
      // needs it), and which compares locations in different header modules
      // by the order in which the modules were loaded.
      if (IsBeforeInSourceOrder(source_manager,
                                redecl->getSourceRange().getBegin(),
                                canonical->getSourceRange().getBegin())) {
        canonical = redecl;
      }
    }
    CRUBIT_CHECK(canonical != nullptr);
    return canonical;
  }
  if (const auto* function_decl = llvm::dyn_cast<clang::FunctionDecl>(decl)) {
    if (llvm::isa<clang::CXXMethodDecl>(function_decl)) {
      return PreferMemberOfKeptDefinition(function_decl->getCanonicalDecl());
    }
    const auto* canonical = function_decl->getCanonicalDecl();
    if (canonical->getLexicalDeclContext()->isRecord()) {
      for (const auto* redecl : function_decl->redecls()) {
        if (!redecl->getLexicalDeclContext()->isRecord()) {
          return redecl;
        }
      }
    }
    return canonical;
  }
  return PreferMemberOfKeptDefinition(decl->getCanonicalDecl());
}

clang::Decl* absl_nullable Importer::CanonicalizeDecl(
    clang::Decl* absl_nonnull decl) const {
  const clang::Decl* ret = CanonicalizeDecl(*decl);
  return const_cast<clang::Decl*>(ret);
}

ItemId Importer::GenerateItemId(const clang::Decl& decl) const {
  const clang::Decl* canonicalized = CanonicalizeDecl(decl);
  return ItemId(reinterpret_cast<uintptr_t>(canonicalized));
}

bool Importer::IsUnsupportedAndAlien(ItemId item_id) const {
  auto it = import_cache_.find(reinterpret_cast<clang::Decl*>(item_id.value()));
  return it != import_cache_.end() && it->second.proto_item != nullptr &&
         it->second.proto_item->has_unsupported_item() &&
         !IsFromCurrentTarget(*it->first);
}

ItemId Importer::GenerateItemId(const clang::RawComment& comment) const {
  return ItemId(reinterpret_cast<uintptr_t>(&comment));
}

absl::StatusOr<std::optional<ItemId>> Importer::GetEnclosingItemId(
    clang::Decl* absl_nonnull decl) {
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
      return GenerateItemId(*record_decl);
    }
    if (auto* namespace_decl =
            clang::dyn_cast<clang::NamespaceDecl>(decl_context)) {
      return GenerateItemId(*namespace_decl);
    }
  }
}

// Returns true if `decl_context` is the synthetic namespace that `IrFromCc`
// wraps `cc_template!` instantiation aliases in.
static bool IsCcTemplateInstantiationsNamespace(
    const clang::DeclContext* absl_nullable decl_context) {
  const auto* ns_decl =
      clang::dyn_cast_or_null<clang::NamespaceDecl>(decl_context);
  return ns_decl != nullptr && ns_decl->getParent()->isTranslationUnit() &&
         StringViewFromStringRef(ns_decl->getName()) ==
             kInstantiationsNamespaceName;
}

// Returns true if `decl` (a child of a record if `is_record`) should be
// imported during the initial AST traversal. If lazy import is enabled, other
// (alien) decls are only imported once something in the current target refers
// to them.
//
// The `cc_template!` instantiations live in the virtual input file, which is
// not owned by the current target (and must not be, or the synthetic namespace
// would be emitted into the bindings), but they need to be imported eagerly.
static bool ShouldImportEagerly(const clang::Decl& decl, bool is_record,
                                const ImportContext& ictx) {
  if (!ictx.invocation_.lazy_import_alien_decls()) return true;
  if (is_record || ictx.IsFromCurrentTarget(decl)) return true;
  return IsCcTemplateInstantiationsNamespace(
             clang::dyn_cast<clang::DeclContext>(&decl)) ||
         IsCcTemplateInstantiationsNamespace(decl.getDeclContext());
}

Importer::DeclItems Importer::GetDeclItems(const clang::Decl& decl) {
  DeclItems decl_items;
  clang::SourceManager& sm = ctx_.getSourceManager();
  auto compare_locations = SourceLocationComparator(sm);

  // We are only interested in comments within this decl context.
  //
  // With header modules, there can be several copies of the same namespace
  // block (one per module that contains the header), which all have the same
  // representative copy. Only the representative copy gets the comments, so
  // that they are not repeated.
  //
  // The locations are translated to the copy of the header that `comments_`
  // are in (see `TranslateToCommentFile`).
  const clang::Decl& decl_for_locations =
      GetDeclForSourceLocations(decl, ctx_, representative_copy_cache_);
  std::vector<const clang::RawComment*> comments_in_range;
  if (&decl_for_locations == &decl ||
      !clang::isa<clang::NamespaceDecl>(&decl)) {
    comments_in_range.assign(
        llvm::lower_bound(comments_, TranslateToCommentFile(decl.getBeginLoc()),
                          compare_locations),
        llvm::upper_bound(comments_, TranslateToCommentFile(decl.getEndLoc()),
                          compare_locations));
  }

  std::map<clang::SourceLocation, const clang::RawComment*,
           SourceLocationComparator>
      ordered_comments(compare_locations);
  for (auto& comment : comments_in_range) {
    if (IsFilteredComment(sm, *comment)) continue;
    ordered_comments.insert({comment->getBeginLoc(), comment});
  }

  // We remove comments attached to a child decl or that are within a child
  // decl.
  auto erase_comments_of_child = [&](const clang::Decl& child) {
    if (auto raw_comment = GetRawComment(child)) {
      ordered_comments.erase(
          TranslateToCommentFile(raw_comment->getBeginLoc()));
    }
    // Implicit decls can have an inverted source range: e.g. the implicit copy
    // deduction guide of a class template that is declared more than once can
    // begin at the `template` keyword of its most recent declaration, but end
    // at the name of an earlier one. Erasing an inverted range is UB.
    clang::SourceLocation begin = TranslateToCommentFile(child.getBeginLoc());
    clang::SourceLocation end = TranslateToCommentFile(child.getEndLoc());
    if (child.getLocation().isValid() && !compare_locations(end, begin)) {
      ordered_comments.erase(ordered_comments.lower_bound(begin),
                             ordered_comments.upper_bound(end));
    }
  };

  absl::flat_hash_set<ItemId> visited_item_ids;

  const auto& decl_context = clang::cast<clang::DeclContext>(decl);
  const bool is_record = clang::isa<clang::RecordDecl>(&decl_context);
  for (auto decl : GetCanonicalChildren(decl_context)) {
    const ir_proto::Item* item = nullptr;
    if (ShouldImportEagerly(*decl, is_record, *this)) {
      item = GetDeclItem(decl);
    } else if (auto it = import_cache_.find(decl); it != import_cache_.end()) {
      item = it->second.proto_item.get();
    }
    // Only add item ids for decls that can be successfully imported.
    if (item != nullptr) {
      auto item_id = GenerateItemId(*decl);
      // TODO(rosica): Drop this check when we start importing also other
      // redecls, not just the canonical
      if (visited_item_ids.find(item_id) == visited_item_ids.end()) {
        visited_item_ids.insert(item_id);
        decl_items.canonical_children.push_back({decl, item_id});
      }
    }

    erase_comments_of_child(*decl);
  }

  // With header modules, the children of the other copies of this namespace
  // block (which are not in `decl.decls()` if `CanonicalizeDecl` picked a
  // declaration from another copy) also have their comments in
  // `ordered_comments`.
  if (!ordered_comments.empty() && ctx_.getExternalSource() != nullptr &&
      clang::isa<clang::NamespaceDecl>(&decl)) {
    for (const clang::Decl* redecl : decl.redecls()) {
      if (redecl == &decl || &GetDeclForSourceLocations(
                                 *redecl, ctx_, representative_copy_cache_) !=
                                 &decl_for_locations) {
        continue;
      }
      for (const clang::Decl* child :
           GetCanonicalChildren(*clang::cast<clang::DeclContext>(redecl))) {
        erase_comments_of_child(*child);
      }
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
    const clang::TranslationUnitDecl& translation_unit_decl) {
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
        {GetSourceOrderKey(*comment), GenerateItemId(*comment)});
  }

  // Push all the other items
  for (auto& [decl, item_id] : decl_items.canonical_children) {
    // Don't include nested records in the total set of top-level items,
    // as they will be generated as part of their parents.
    if (llvm::isa<clang::CXXRecordDecl>(decl) &&
        llvm::isa<clang::CXXRecordDecl>(decl->getDeclContext())) {
      continue;
    }
    items[GetOwningTarget(*decl)].push_back(
        {GetSourceOrderKey(*decl), item_id});
  }

  clang::SourceManager& sm = ctx_.getSourceManager();
  auto compare_locations = SourceLocationComparator(sm);

  absl::flat_hash_map<BazelLabel, std::vector<ItemId>> ordered_items;
  for (auto& [target, item_ids_in_target] : items) {
    llvm::stable_sort(item_ids_in_target, compare_locations);

    std::vector<ItemId>& ordered_item_ids = ordered_items[target];
    ordered_item_ids.reserve(item_ids_in_target.size());
    for (auto& ordered_item : item_ids_in_target) {
      ordered_item_ids.push_back(ordered_item.second);
    }
  }
  return ordered_items;
}

std::vector<ItemId> Importer::GetItemIdsInSourceOrder(
    clang::Decl* absl_nonnull parent_decl) {
  Importer::DeclItems decl_items = GetDeclItems(*parent_decl);

  std::vector<SourceLocationComparator::OrderedItemId> items;

  items.reserve(decl_items.comments.size() +
                decl_items.canonical_children.size());
  for (auto& comment : decl_items.comments) {
    items.push_back({GetSourceOrderKey(*comment), GenerateItemId(*comment)});
  }
  for (auto& [decl, item_id] : decl_items.canonical_children) {
    if (IsUnsupportedAndAlien(item_id)) {
      continue;
    }
    // Skip out-of-line defined children of records when listing children of
    // namespaces!
    if (decl->getDeclContext() !=
            clang::cast<clang::DeclContext>(parent_decl) &&
        llvm::isa<clang::CXXRecordDecl>(decl->getDeclContext())) {
      continue;
    }
    items.push_back({GetSourceOrderKey(*decl), item_id});
  }

  clang::SourceManager& sm = ctx_.getSourceManager();
  auto compare_locations = SourceLocationComparator(sm);

  llvm::stable_sort(items, compare_locations);
  std::vector<ItemId> ordered_item_ids;

  ordered_item_ids.reserve(items.size());
  for (auto& ordered_item : items) {
    ordered_item_ids.push_back(ordered_item.second);
  }
  return ordered_item_ids;
}

// Returns true if `function_decl` is a hidden friend, i.e. a non-member
// function that is only ever declared by friend declarations within classes,
// and can thus only be found by argument-dependent lookup.
static bool IsHiddenFriend(const clang::FunctionDecl& function_decl) {
  if (llvm::isa<clang::CXXMethodDecl>(function_decl)) return false;
  return llvm::all_of(function_decl.redecls(),
                      [](const clang::FunctionDecl* redecl) {
                        return redecl->getLexicalDeclContext()->isRecord();
                      });
}

// Appends to `operators` the hidden friend operators that are declared in the
// (direct or indirect) bases of `record_decl`, and whose first parameter is of
// type `derived_type` (or a reference to it).
//
// Bases that are already in `visited_bases` are skipped. Newly visited bases
// are added to it.
static void CollectInheritedFriendOperators(
    const clang::CXXRecordDecl& record_decl, clang::QualType derived_type,
    const clang::ASTContext& ctx,
    absl::flat_hash_set<const clang::CXXRecordDecl*>& visited_bases,
    std::vector<clang::FunctionDecl*>& operators) {
  for (const clang::CXXBaseSpecifier& base : record_decl.bases()) {
    const clang::CXXRecordDecl* base_decl =
        base.getType()->getAsCXXRecordDecl();
    if (base_decl == nullptr) continue;
    base_decl = base_decl->getDefinition();
    if (base_decl == nullptr || !visited_bases.insert(base_decl).second) {
      continue;
    }
    for (clang::Decl* decl : base_decl->decls()) {
      const auto* friend_decl = llvm::dyn_cast<clang::FriendDecl>(decl);
      if (friend_decl == nullptr) continue;
      auto* function_decl = llvm::dyn_cast_or_null<clang::FunctionDecl>(
          friend_decl->getFriendDecl());
      // Like `FriendDeclImporter`, skip non-canonical decls, and functions that
      // are redeclared outside of a class (which are imported on their own).
      if (function_decl == nullptr ||
          function_decl->getOverloadedOperator() == clang::OO_None ||
          function_decl->getNumParams() == 0 ||
          function_decl != function_decl->getCanonicalDecl() ||
          !IsHiddenFriend(*function_decl)) {
        continue;
      }
      if (ctx.hasSameUnqualifiedType(
              function_decl->getParamDecl(0)->getType().getNonReferenceType(),
              derived_type)) {
        operators.push_back(function_decl);
      }
    }
    CollectInheritedFriendOperators(*base_decl, derived_type, ctx,
                                    visited_bases, operators);
  }
}

std::vector<ItemId> Importer::ImportInheritedFriendOperators(
    clang::CXXRecordDecl* absl_nonnull record_decl) {
  // Generating new trait impls can conflict with manual impls in user code, so
  // this is opt-in.
  //
  // TODO(b/530193579): After expanding features earlier, just check the
  // specific feature without manually checking for "all".
  if (!IsFeatureEnabledForTarget(invocation_.target_,
                                 "inherited_friend_operators") &&
      !IsFeatureEnabledForTarget(invocation_.target_, "all")) {
    return {};
  }

  // The members of class template specializations are only imported
  // selectively (see `GetCanonicalChildren`). For simplicity, inherited
  // operators are not imported for them, nor for records nested in them.
  if (IsFullClassTemplateSpecializationOrChild(record_decl)) return {};

  std::vector<clang::FunctionDecl*> operators;
  absl::flat_hash_set<const clang::CXXRecordDecl*> visited_bases;
  CollectInheritedFriendOperators(*record_decl,
                                  ctx_.getCanonicalTagType(record_decl), ctx_,
                                  visited_bases, operators);
  if (operators.empty()) return {};

  // A hidden friend is a member of the innermost namespace that encloses the
  // class that declares it. Place inherited operators in the innermost
  // namespace that encloses `record_decl` instead, as if they had been declared
  // by `record_decl` itself.
  std::optional<ItemId> enclosing_namespace_id;
  for (const clang::DeclContext* dc = record_decl->getDeclContext();
       !dc->isTranslationUnit() && !dc->isFunctionOrMethod();
       dc = dc->getParent()) {
    if (const auto* namespace_decl = llvm::dyn_cast<clang::NamespaceDecl>(dc)) {
      enclosing_namespace_id = GenerateItemId(*namespace_decl);
      break;
    }
  }
  const ItemId record_id = GenerateItemId(*record_decl);

  std::vector<ItemId> item_ids;
  for (clang::FunctionDecl* function_decl : operators) {
    // The first parameter of each operator refers to `record_decl`, so no other
    // record can have imported it already.
    if (import_cache_.contains(function_decl)) continue;
    {
      llvm::SaveAndRestore<const clang::FunctionDecl*>
          inherited_friend_operator(inherited_friend_operator_, function_decl);
      GetDeclItem(function_decl);
    }
    auto it = import_cache_.find(function_decl);
    if (it == import_cache_.end()) continue;
    ir_proto::Item* item = it->second.proto_item.get();
    // Like `FriendDeclImporter`, silently drop operators that can't be
    // imported.
    if (item == nullptr || !item->has_func()) continue;

    ir_proto::Func* func = item->mutable_func();
    if (enclosing_namespace_id.has_value()) {
      func->set_enclosing_item_id(enclosing_namespace_id->value());
    } else {
      func->clear_enclosing_item_id();
    }
    // The operator is only visible through ADL on arguments of type
    // `record_decl`, rather than through the base class that declares it.
    func->set_adl_enclosing_record(record_id.value());
    item_ids.push_back(GenerateItemId(*function_decl));
  }
  return item_ids;
}

std::vector<ItemId> Importer::GetOrderedItemIdsOfTemplateInstantiations()
    const {
  std::vector<SourceLocationComparator::OrderedItemId> items;
  items.reserve(class_template_instantiations_.size());
  for (const auto* decl : class_template_instantiations_) {
    items.push_back({GetSourceOrderKey(*decl), GenerateItemId(*decl)});
  }

  clang::SourceManager& sm = ctx_.getSourceManager();
  auto compare_locations = SourceLocationComparator(sm);
  llvm::stable_sort(items, compare_locations);

  std::vector<ItemId> ordered_item_ids;
  ordered_item_ids.reserve(items.size());
  for (const auto& ordered_item : items) {
    ordered_item_ids.push_back(ordered_item.second);
  }
  return ordered_item_ids;
}

bool Importer::IsInSystemHeader(clang::SourceLocation location) const {
  const clang::SourceManager& sm = ctx_.getSourceManager();
  if (location.isInvalid()) return false;
  location = sm.getExpansionLoc(location);
  bool is_in_system_header = sm.isInSystemHeader(location);
  if (!sm.isLoadedSourceLocation(location)) return is_in_system_header;
  // The file was loaded from a precompiled header module. Apply the
  // `--system-header-prefix` and `--no-system-header-prefix` options like
  // `HeaderSearch::LookupFile` does when the file is included textually: the
  // last matching option wins.
  std::optional<llvm::StringRef> filename =
      sm.getNonBuiltinFilenameForID(sm.getFileID(location));
  if (!filename.has_value()) return is_in_system_header;
  filename->consume_front("./");
  const auto& prefixes = sema_.getPreprocessor()
                             .getHeaderSearchInfo()
                             .getHeaderSearchOpts()
                             .SystemHeaderPrefixes;
  for (auto it = prefixes.rbegin(); it != prefixes.rend(); ++it) {
    if (filename->starts_with(it->Prefix)) return it->IsSystemHeader;
  }
  return is_in_system_header;
}

clang::FileID Importer::GetFirstLoadedFileID(clang::FileEntryRef file) const {
  const clang::SourceManager& sm = ctx_.getSourceManager();
  clang::SourceLocation first;
  for (unsigned i = 0, n = sm.loaded_sloc_entry_size(); i != n; ++i) {
    const clang::SrcMgr::SLocEntry& entry = sm.getLoadedSLocEntry(i);
    if (!entry.isFile()) continue;
    clang::OptionalFileEntryRef entry_file =
        entry.getFile().getContentCache().OrigEntry;
    if (!entry_file.has_value() ||
        &entry_file->getFileEntry() != &file.getFileEntry()) {
      continue;
    }
    clang::SourceLocation start =
        clang::SourceLocation::getFromRawEncoding(entry.getOffset());
    if (first.isInvalid() || IsBeforeInSourceOrder(sm, start, first)) {
      first = start;
    }
  }
  if (first.isInvalid()) return clang::FileID();
  return sm.getFileID(first);
}

const std::map<unsigned, clang::RawComment*>* absl_nullable
Importer::GetModuleCommentsInFile(clang::FileID file) const {
  clang::SourceManager& sm = ctx_.getSourceManager();
  if (module_comment_files_.insert(file).second) {
    // This mirrors what the preprocessor and `Sema::ActOnComment` do when the
    // file is parsed textually, except that comments in blocks skipped by
    // conditional compilation (e.g. `#if 0`) are not skipped.
    const clang::CommentOptions& comment_options =
        ctx_.getLangOpts().CommentOpts;
    std::optional<llvm::MemoryBufferRef> buffer = sm.getBufferOrNone(file);
    if (buffer.has_value() &&
        (comment_options.RetainCommentsFromSystemHeaders ||
         !IsInSystemHeader(sm.getLocForStartOfFile(file)))) {
      clang::Lexer lexer(file, *buffer, sm, ctx_.getLangOpts());
      lexer.SetCommentRetentionState(true);
      clang::Token token;
      while (true) {
        bool at_end = lexer.LexFromRawLexer(token);
        if (token.is(clang::tok::comment)) {
          module_comments_.addComment(
              clang::RawComment(
                  sm,
                  clang::SourceRange(token.getLocation(), token.getEndLoc()),
                  comment_options, /*Merged=*/false),
              comment_options, module_comments_allocator_);
        }
        if (at_end || token.is(clang::tok::eof)) break;
      }
    }
  }
  return module_comments_.getCommentsInFile(file);
}

const clang::RawComment* absl_nullable Importer::GetRawComment(
    const clang::Decl& decl) const {
  const clang::Decl& decl_for_locations =
      GetDeclForSourceLocations(decl, ctx_, representative_copy_cache_);
  // Without precompiled header modules, all comments are in `ctx_.Comments`.
  if (ctx_.getExternalSource() == nullptr) {
#if LLVM_DEV_DATE_GE(20260527)
    return ctx_.getRawCommentNoCache(&decl_for_locations);
#else
    return ctx_.getRawCommentForDeclNoCache(&decl_for_locations);
#endif
  }
  // Like `ASTContext::getRawCommentNoCache`, except that the comments of files
  // loaded from a module are in `module_comments_`.
  const clang::SourceManager& sm = ctx_.getSourceManager();
  for (clang::SourceLocation location :
       GetLocsForCommentSearch(decl_for_locations, sm)) {
    if (location.isInvalid() || !location.isFileID()) continue;
    clang::FileID file = sm.getFileID(location);
    if (file.isInvalid()) continue;
    const std::map<unsigned, clang::RawComment*>* comments_in_file =
        sm.isLoadedFileID(file) ? GetModuleCommentsInFile(file)
                                : ctx_.Comments.getCommentsInFile(file);
    if (comments_in_file == nullptr) continue;
    // This works for the comments in `module_comments_` too:
    // `getRawCommentNoCacheImpl` only uses `ctx_.Comments` to cache their
    // positions.
#if LLVM_DEV_DATE_GE(20260527)
    const clang::RawComment* comment = ctx_.getRawCommentNoCacheImpl(
        &decl_for_locations, location, *comments_in_file);
#else
    const clang::RawComment* comment = ctx_.getRawCommentForDeclNoCacheImpl(
        &decl_for_locations, location, *comments_in_file);
#endif
    if (comment != nullptr) return comment;
  }
  return nullptr;
}

clang::SourceLocation Importer::TranslateToCommentFile(
    clang::SourceLocation location) const {
  if (comment_file_ids_.empty() || location.isInvalid()) return location;
  const clang::SourceManager& sm = ctx_.getSourceManager();
  auto [file_id, offset] = sm.getDecomposedExpansionLoc(location);
  clang::OptionalFileEntryRef file = sm.getFileEntryRefForID(file_id);
  if (!file.has_value()) return location;
  auto it = comment_file_ids_.find(&file->getFileEntry());
  if (it == comment_file_ids_.end() || it->second == file_id) return location;
  return sm.getComposedLoc(it->second, offset);
}

void Importer::ImportFreeComments() {
  clang::SourceManager& sm = ctx_.getSourceManager();
  // With header modules, the files that contain top-level declarations that
  // were parsed textually (i.e. that are not from a module). Implicit
  // `ImportDecl`s (for `#include`s translated to module imports) don't count:
  // a textual copy that was skipped by its include guard can still contain
  // them.
  llvm::DenseSet<clang::FileID> files_with_textual_decls;
  if (ctx_.getExternalSource() != nullptr) {
    for (const clang::Decl* decl : ctx_.getTranslationUnitDecl()->decls()) {
      if (decl->isFromASTFile() || decl->getLocation().isInvalid() ||
          clang::isa<clang::ImportDecl>(decl)) {
        continue;
      }
      files_with_textual_decls.insert(
          sm.getFileID(sm.getExpansionLoc(decl->getLocation())));
    }
  }
  for (const auto& header : invocation_.public_headers_) {
    if (auto file = sm.getFileManager().getFileRef(
            StringRefFromStringView(header.IncludePath()))) {
      // If the header was only loaded from precompiled header modules, use the
      // same copy of it as `GetRepresentativeCopy`.
      if (ctx_.getExternalSource() != nullptr) {
        // The textual copy of the header can be (almost) empty, if a module
        // that was imported earlier defined its include guard. Then the
        // representative copies of its declarations (and its comments) are in
        // a module.
        clang::FileID file_id = sm.translateFile(*file);
        if (file_id.isValid() && !sm.isLoadedFileID(file_id) &&
            files_with_textual_decls.contains(file_id)) {
          comment_file_ids_[&file->getFileEntry()] = file_id;
        } else {
          clang::FileID loaded_file_id = GetFirstLoadedFileID(*file);
          if (loaded_file_id.isValid()) {
            comment_file_ids_[&file->getFileEntry()] = loaded_file_id;
            if (auto comments_in_file =
                    GetModuleCommentsInFile(loaded_file_id)) {
              for (const auto& [_, comment] : *comments_in_file) {
                comments_.push_back(comment);
              }
            }
            continue;
          }
        }
      }
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

void Importer::FindAlwaysInstantiateSpecs(
    const clang::DeclContext& decl_context) {
  for (const clang::Decl* decl : decl_context.decls()) {
    if (const auto* alias_decl =
            clang::dyn_cast<clang::TypedefNameDecl>(decl)) {
      if (auto status_or_bool = HasAnnotationWithoutArgs(
              *alias_decl, "crubit_always_instantiate");
          status_or_bool.ok() && *status_or_bool) {
        if (const auto* record_decl =
                alias_decl->getUnderlyingType()->getAsCXXRecordDecl()) {
          if (const auto* spec =
                  clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(
                      record_decl)) {
            if (const auto* canonical_spec =
                    clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(
                        spec->getCanonicalDecl())) {
              always_instantiate_specs_.insert(canonical_spec);
            }
          }
        }
      }
    } else if (const auto* namespace_decl =
                   clang::dyn_cast<clang::NamespaceDecl>(decl)) {
      FindAlwaysInstantiateSpecs(*namespace_decl);
    } else if (const auto* linkage_spec =
                   clang::dyn_cast<clang::LinkageSpecDecl>(decl)) {
      FindAlwaysInstantiateSpecs(*linkage_spec);
    }
  }
}

// Returns true if `decl` is in a namespace whose templates get special support
// (e.g. `std::optional`, `absl::Span`).
static bool IsInSpecialCasedNamespace(const clang::Decl& decl) {
  std::optional<llvm::StringRef> top_level_ns =
      AsTopLevelNamespace(decl.getDeclContext());
  return top_level_ns == "std" || top_level_ns == "absl" ||
         top_level_ns == "c9" || top_level_ns == "rs" ||
         top_level_ns == "rs_std";
}

bool Importer::IsAlwaysInstantiate(
    const clang::ClassTemplateSpecializationDecl& spec_decl) const {
  if (always_instantiate_specs_.contains(
          clang::cast<clang::ClassTemplateSpecializationDecl>(
              spec_decl.getCanonicalDecl()))) {
    return true;
  }
  if (auto* template_decl = spec_decl.getSpecializedTemplate()) {
    if (auto* templated_decl = template_decl->getTemplatedDecl()) {
      if (IsInSpecialCasedNamespace(*templated_decl)) {
        return true;
      }
      if (auto status_or_bool = HasAnnotationWithoutArgs(
              *templated_decl, "crubit_always_instantiate");
          status_or_bool.ok() && *status_or_bool) {
        return true;
      }
    }
  }
  return false;
}

std::optional<size_t> Importer::CountLifetimes(clang::QualType type) const {
  type = type.getCanonicalType();
  if (type->isPointerType() || type->isReferenceType()) {
    clang::QualType pointee_type = type->getPointeeType();
    // Function pointers and references only ever have a `'static` lifetime.
    if (pointee_type->isFunctionType()) return 0;
    std::optional<size_t> pointee_count = CountLifetimes(pointee_type);
    if (!pointee_count.has_value()) return std::nullopt;
    return 1 + *pointee_count;
  }
  const clang::CXXRecordDecl* record_decl = type->getAsCXXRecordDecl();
  if (record_decl == nullptr) return 0;
  record_decl = record_decl->getDefinitionOrSelf();
  if (!AreAssumedLifetimesEnabledForTarget(GetOwningTarget(*record_decl))) {
    return 0;
  }
  if (const auto* specialization_decl =
          clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(record_decl);
      specialization_decl != nullptr &&
      GetTemplateArgumentLifetimeParam(*specialization_decl).has_value()) {
    return 1;
  }
  absl::StatusOr<std::vector<absl::string_view>> lifetime_inputs =
      CollectLifetimeInputs(ctx_, record_decl);
  if (!lifetime_inputs.ok()) return std::nullopt;
  return lifetime_inputs->size();
}

std::optional<std::string> Importer::GetTemplateArgumentLifetimeParam(
    const clang::ClassTemplateSpecializationDecl& specialization_decl) const {
  if (!AreAssumedLifetimesEnabledForTarget(
          GetOwningTarget(specialization_decl))) {
    return std::nullopt;
  }
  // Only specializations instantiated from the primary template: their members
  // are instantiated from it, so a member whose type is the template parameter
  // is still marked as such (by a `SubstTemplateTypeParmType`).
  if (specialization_decl.getSpecializationKind() ==
      clang::TSK_ExplicitSpecialization) {
    return std::nullopt;
  }
  auto* template_decl = specialization_decl.getSpecializedTemplateOrPartial()
                            .dyn_cast<clang::ClassTemplateDecl*>();
  if (template_decl == nullptr) return std::nullopt;
  // The special support for these templates (e.g. `absl::Span`) already decides
  // what their arguments' lifetimes mean.
  if (IsInSpecialCasedNamespace(*template_decl->getTemplatedDecl())) {
    return std::nullopt;
  }
  // Bridged types are not bound as records.
  for (const auto* attr :
       specialization_decl.specific_attrs<clang::AnnotateAttr>()) {
    if (attr->getAnnotation() == "crubit_bridge_rust_name") {
      return std::nullopt;
    }
  }
  // A template that declares its own lifetime parameters (`LIFETIME_PARAMS`)
  // names them itself.
  absl::StatusOr<std::vector<absl::string_view>> lifetime_inputs =
      CollectLifetimeInputs(ctx_, &specialization_decl);
  if (!lifetime_inputs.ok() || !lifetime_inputs->empty()) return std::nullopt;

  // TODO(zarko): Support more than one template argument.
  const clang::TemplateArgumentList& args =
      specialization_decl.getTemplateArgs();
  if (args.size() != 1 || args[0].getKind() != clang::TemplateArgument::Type ||
      CountLifetimes(args[0].getAsType()) != 1) {
    return std::nullopt;
  }

  // Name the lifetime after the template parameter, so that the generated
  // struct reads as "the lifetime of `T`".
  const clang::NamedDecl* param =
      template_decl->getTemplateParameters()->getParam(0);
  if (param->getIdentifier() == nullptr || param->getName() == "unknown") {
    return "__arg";
  }
  return param->getName().str();
}

void SetMustBindItem(ir_proto::Item& item) {
  switch (item.kind()) {
    case ir_proto::Item::kFunc:
      item.mutable_func()->set_must_bind(true);
      break;
    case ir_proto::Item::kRecord:
      item.mutable_record()->set_must_bind(true);
      break;
    case ir_proto::Item::kIncompleteRecord:
      item.mutable_incomplete_record()->set_must_bind(true);
      break;
    case ir_proto::Item::kEnum:
      item.mutable_enum_decl()->set_must_bind(true);
      break;
    case ir_proto::Item::kConstant:
      item.mutable_constant()->set_must_bind(true);
      break;
    case ir_proto::Item::kTypeAlias:
      item.mutable_type_alias()->set_must_bind(true);
      break;
    case ir_proto::Item::kGlobalVar:
      item.mutable_global_var()->set_must_bind(true);
      break;
    case ir_proto::Item::kUnsupportedItem:
      item.mutable_unsupported_item()->set_must_bind(true);
      break;
    case ir_proto::Item::kComment:
      item.mutable_comment()->set_must_bind(true);
      break;
    case ir_proto::Item::kNamespace:
      item.mutable_namespace_decl()->set_must_bind(true);
      break;
    case ir_proto::Item::kUseMod:
      item.mutable_use_mod()->set_must_bind(true);
      break;
    case ir_proto::Item::kExistingRustType:
      item.mutable_existing_rust_type()->set_must_bind(true);
      break;
  }
}

// Returns the DeclContext that determines `decl`'s enclosing item in the IR, or
// nullptr if `decl` is always placed at the top level.
//
// Class template specializations are emitted at the top level unless they have
// a `preferred_name` alias, in which case they are placed inside the alias's
// enclosing context (see `CXXRecordDeclImporter::Import` and
// `Importer::GetCanonicalChildren`).
static clang::DeclContext* absl_nullable GetEnclosingDeclContext(
    clang::Decl* absl_nonnull decl, const ImportContext& ictx) {
  if (clang::isa<clang::ClassTemplateSpecializationDecl>(decl)) {
    clang::TypedefNameDecl* alias_decl =
        ictx.GetTemplateSpecializationAlias(decl);
    return alias_decl != nullptr ? alias_decl->getDeclContext() : nullptr;
  }
  return decl->getDeclContext();
}

void Importer::CheckLazyImportInvariants() const {
  for (const auto& [decl, entry] : import_cache_) {
    if (entry.proto_item == nullptr) continue;
    for (const clang::DeclContext* dc =
             GetEnclosingDeclContext(const_cast<clang::Decl*>(decl), *this);
         dc != nullptr && !dc->isTranslationUnit() && !dc->isFunctionOrMethod();
         dc = dc->getParent()) {
      if (const auto* ns_decl = llvm::dyn_cast<clang::NamespaceDecl>(dc)) {
        CRUBIT_CHECK(import_cache_.contains(ns_decl))
            << "Enclosing namespace of an imported decl was not imported";
      }
    }
    if (entry.proto_item->has_namespace_decl()) {
      const auto* ns_decl = llvm::cast<clang::NamespaceDecl>(decl);
      CRUBIT_CHECK(import_cache_.contains(ns_decl->getCanonicalDecl()))
          << "Canonical namespace of an imported namespace was not imported";
    }
  }
}

void Importer::Import(
    clang::TranslationUnitDecl* absl_nonnull translation_unit_decl) {
  FindAlwaysInstantiateSpecs(*translation_unit_decl);
  ImportFreeComments();
  clang::SourceManager& sm = ctx_.getSourceManager();
  std::vector<SourceLocationComparator::OrderedItem> ordered_items;

  // Store free comments in comment_items to preserve their lifetime during
  // Import.
  std::vector<std::unique_ptr<ir_proto::Item>> comment_items;
  comment_items.reserve(comments_.size());
  ordered_items.reserve(comments_.size());
  for (auto& comment : comments_) {
    auto item = std::make_unique<ir_proto::Item>();
    auto* c = item->mutable_comment();
    c->set_text(comment->getFormattedText(sm, sm.getDiagnostics()));
    c->set_id(GenerateItemId(*comment).value());
    ordered_items.push_back({GetSourceOrderKey(*comment), item.get()});
    comment_items.push_back(std::move(item));
  }

  ImportDeclsFromDeclContext(*translation_unit_decl);

  // Augment child_item_ids for records with out-of-line defined children, e.g.
  // class A { class B; };   // declares A::B
  // class A::B { ... };  // defines A::B
  std::vector<std::pair<SourceOrderKey, const clang::Decl*>> ordered_children;
  for (const clang::Decl* decl : import_cache_insertion_order_) {
    const ItemCacheEntry& entry = import_cache_.at(decl);
    if (entry.proto_item == nullptr) continue;
    if (auto* parent_record_decl =
            llvm::dyn_cast<clang::CXXRecordDecl>(decl->getDeclContext())) {
      auto parent_it = import_cache_.find(parent_record_decl);
      if (parent_it != import_cache_.end() &&
          parent_it->second.proto_item != nullptr &&
          parent_it->second.proto_item->has_record()) {
        ordered_children.push_back({GetSourceOrderKey(*decl), decl});
      }
    }
  }

  // The sort is stable and starts from insertion order, so that ties (which
  // are common with header modules, where all copies of a declaration share
  // the source range of the representative copy) don't depend on the hash
  // seed.
  SourceLocationComparator compare_locations(sm);
  llvm::stable_sort(ordered_children, compare_locations);

  for (const auto& [key, decl] : ordered_children) {
    auto* parent_record_decl =
        llvm::dyn_cast<clang::CXXRecordDecl>(decl->getDeclContext());
    auto parent_it = import_cache_.find(parent_record_decl);
    auto parent_id = parent_it->second.proto_item->record().id();

    auto child_id = GenerateItemId(*decl);
    auto& parent_child_ids = invocation_.child_item_ids_[ItemId(parent_id)];
    if (!IsUnsupportedAndAlien(child_id) &&
        !absl::c_linear_search(parent_child_ids, child_id)) {
      parent_child_ids.push_back(child_id);
    }
  }

  // Because alien declarations are imported lazily (i.e. only once something
  // in the current target refers to them) when `lazy_import_alien_decls` is
  // enabled, an import can appear *after* its enclosing namespace has already
  // computed its `child_item_ids_`.
  // `GetDeclItem` guarantees that the enclosing namespaces of every imported
  // decl are imported too (and `NamespaceDeclImporter` imports the canonical
  // namespace decl), but their child lists may be stale. Refresh them here,
  // repeating until a fixpoint in case the refresh itself triggers further
  // imports.
  //
  // This needs to look at every entry of `import_cache_`, which is a hash map.
  // Importing while iterating it directly is not safe here, for two separate
  // reasons:
  //
  //  1. Importing inserts into `import_cache_`, and a rehash invalidates any
  //     live iterator.
  //  2. Its iteration order is unspecified (and deliberately randomized
  //     between processes), so importing in that order would make the
  //     generated bindings depend on the hash seed.
  //
  // We therefore snapshot the decls of interest into a vector and sort them by
  // source order before doing any work that can import. The snapshot is taken
  // in insertion order and the sort is stable, so that ties in source order
  // (e.g. the copies of a namespace block from several header modules, which
  // all have the source range of the representative copy) are broken
  // deterministically too. This terminates because `import_cache_` only ever
  // grows, and is bounded by the number of decls in the translation unit.
  while (invocation_.lazy_import_alien_decls()) {
    const size_t cache_size_before_refresh = import_cache_.size();
    std::vector<std::pair<SourceOrderKey, clang::NamespaceDecl*>>
        namespaces_to_refresh;
    for (const clang::Decl* decl : import_cache_insertion_order_) {
      const ItemCacheEntry& entry = import_cache_.at(decl);
      if (entry.proto_item == nullptr) continue;
      if (const auto* ns_decl = llvm::dyn_cast<clang::NamespaceDecl>(decl)) {
        namespaces_to_refresh.push_back(
            {GetSourceOrderKey(*ns_decl),
             const_cast<clang::NamespaceDecl*>(ns_decl)});
      }
    }
    llvm::stable_sort(namespaces_to_refresh, compare_locations);
    for (const auto& [key, ns_decl] : namespaces_to_refresh) {
      invocation_.child_item_ids_[GenerateItemId(*ns_decl)] =
          GetItemIdsInSourceOrder(ns_decl);
    }

    // The refresh imported nothing new, so the `child_item_ids_` assigned
    // above were computed against the final set of items and are complete.
    if (import_cache_.size() == cache_size_before_refresh) break;
  }

  if (invocation_.lazy_import_alien_decls() &&
      invocation_.check_importer_invariants()) {
    CheckLazyImportInvariants();
  }

  invocation_.top_level_item_ids_ =
      GetTopLevelItemIdsInSourceOrder(*translation_unit_decl);

  // TODO(b/257302656): Consider placing the generated template instantiations
  // into a separate namespace (maybe `crubit::instantiated_templates` ?).
  llvm::copy(
      GetOrderedItemIdsOfTemplateInstantiations(),
      std::back_inserter(invocation_.top_level_item_ids_[invocation_.target_]));

  // TODO(b/530340081): This logic to build a representative tree IR
  // replaces BuildTree() in ir.cc. The latter should be deleted.
  //
  // Map each ItemId to its corresponding ir_proto::Item pointer.
  absl::flat_hash_map<ItemId, ir_proto::Item* absl_nonnull> id_to_item;
  for (const auto& comment_item : comment_items) {
    id_to_item[ItemId(comment_item->comment().id())] = comment_item.get();
  }
  // In insertion order, so that if several decls map to the same `ItemId`, the
  // one that wins doesn't depend on the hash seed.
  for (const clang::Decl* decl : import_cache_insertion_order_) {
    const ItemCacheEntry& entry = import_cache_.at(decl);
    if (entry.proto_item != nullptr) {
      id_to_item[GenerateItemId(*decl)] = entry.proto_item.get();
    }
  }

  // IR previously maintains a flat array of opaque items. To improve
  // readability, we enforce a nested IR structure by populating nested child
  // items for Record and NamespaceDecl containers. See crubit.rs-ir-proto.
  //
  // Protobufs do not allow pointer aliasing. We have to explicitly populate
  // items in a post-order traversal to ensure that children items are complete
  // before copying them into parent items.
  std::vector<ItemId> ordered_item_ids;
  {
    absl::flat_hash_set<ItemId> visited;
    auto collect_in_post_order = [&](auto& self, ItemId item_id) -> void {
      if (!visited.insert(item_id).second) return;
      if (auto child_ids_it = invocation_.child_item_ids_.find(item_id);
          child_ids_it != invocation_.child_item_ids_.end()) {
        for (ItemId child_id : child_ids_it->second) {
          self(self, child_id);
        }
      }
      ordered_item_ids.push_back(item_id);
    };
    for (const auto& [parent_id, _] : invocation_.child_item_ids_) {
      collect_in_post_order(collect_in_post_order, parent_id);
    }
  }

  for (ItemId item_id : ordered_item_ids) {
    auto child_ids_it = invocation_.child_item_ids_.find(item_id);
    if (child_ids_it == invocation_.child_item_ids_.end()) continue;

    auto parent_it = id_to_item.find(item_id);
    if (parent_it == id_to_item.end()) continue;

    if (parent_it->second->has_record()) {
      auto* record = parent_it->second->mutable_record();
      record->clear_children();
      for (ItemId child_id : child_ids_it->second) {
        auto child_it = id_to_item.find(child_id);
        if (child_it != id_to_item.end()) {
          *record->add_children() = *child_it->second;
        }
      }
    } else if (parent_it->second->has_namespace_decl()) {
      auto* ns = parent_it->second->mutable_namespace_decl();
      ns->clear_children();
      for (ItemId child_id : child_ids_it->second) {
        auto child_it = id_to_item.find(child_id);
        if (child_it != id_to_item.end()) {
          *ns->add_children() = *child_it->second;
        }
      }
    }
  }

  // Populate top-level items for each Bazel label in the IR protobuf. This
  // preserves the source order recorded in top_level_item_ids_.
  for (const auto& [target, item_ids] : invocation_.top_level_item_ids_) {
    auto& target_items =
        (*invocation_.ir_.mutable_top_level_items())[target.value()];
    for (ItemId id : item_ids) {
      auto it = id_to_item.find(id);
      // Remove any unsupported alien (to the current target) items.
      // IsUnsupportedAndAlien only returns true for items outside the current
      // target that are unsupported. You have to run it only if label is not
      // equal to the current target; you can skip the current target.
      if (it != id_to_item.end() && !IsUnsupportedAndAlien(id)) {
        *target_items.Add() = *it->second;
      }
    }
  }
}

static bool HasDeclsFromCurrentTarget(const clang::DeclContext& dc,
                                      const ImportContext& ictx) {
  for (const clang::Decl* decl : dc.decls()) {
    if (ictx.IsFromCurrentTarget(*decl)) {
      return true;
    }
    if (const auto* child_dc = clang::dyn_cast<clang::DeclContext>(decl)) {
      if ((clang::isa<clang::NamespaceDecl>(decl) ||
           clang::isa<clang::LinkageSpecDecl>(decl)) &&
          HasDeclsFromCurrentTarget(*child_dc, ictx)) {
        return true;
      }
    }
  }
  return false;
}

void Importer::ImportDeclsFromDeclContext(
    const clang::DeclContext& decl_context) {
  const bool is_record = clang::isa<clang::RecordDecl>(&decl_context);
  for (auto decl : GetCanonicalChildren(decl_context)) {
    if (ShouldImportEagerly(*decl, is_record, *this)) {
      GetDeclItem(decl);
    } else if (const auto* ns_decl =
                   clang::dyn_cast<clang::NamespaceDecl>(decl)) {
      if (HasDeclsFromCurrentTarget(*ns_decl, *this)) {
        GetDeclItem(decl);
      }
    }
  }
}

const ir_proto::Item* absl_nullable Importer::GetDeclItem(
    clang::Decl* absl_nonnull decl) {
  if (auto it = import_cache_.find(decl); it != import_cache_.end()) {
    if (it->second.status == ItemCacheEntry::Status::kInProgress) {
      // TODO(jeanpierreda): Fix and promote to CHECK.
      // At least one cycle occurs with Typedef, where a typedef will import
      // itself during its own import. This isn't an infinite loop, because the
      // recursive cycle gets broken between the two by CXXRecordDecl, but the
      // result is that we get this typedef inserted while we were attempting to
      // insert it.
      //
      // Alternatively, maybe it's sufficient to check that they're _equal_.
      // It's not a bug at all to import it twice if it has no effect.
      LOG(INFO) << "Re-entrant import discovered. Trying to import a "
                << decl->getDeclKindName()
                << " while its import is still in progress. Returning nullptr "
                   "to break the cycle.";
      return nullptr;
    }
    return it->second.proto_item.get();
  }

  // In lazy mode, if `decl` is nested inside an unimported record (e.g. an
  // alien `Outer::Inner` referenced directly from the current target), import
  // the enclosing record first so that `Outer` and all of its nested
  // declarations are imported top-down in source order. Otherwise `Inner`'s
  // `GetEnclosingItemId` would trigger importing `Outer` while `Inner` is still
  // `kInProgress`.
  if (invocation_.lazy_import_alien_decls()) {
    for (clang::DeclContext* dc = GetEnclosingDeclContext(decl, *this);
         dc != nullptr && !dc->isTranslationUnit() && !dc->isFunctionOrMethod();
         dc = dc->getParent()) {
      if (auto* parent_record = clang::dyn_cast<clang::RecordDecl>(dc)) {
        if (clang::Decl* canonical_parent = CanonicalizeDecl(parent_record);
            canonical_parent != nullptr &&
            !import_cache_.contains(canonical_parent)) {
          GetDeclItem(canonical_parent);
        }
        break;
      }
    }
    if (auto it = import_cache_.find(decl); it != import_cache_.end()) {
      return it->second.proto_item.get();
    }
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
  // a CRUBIT_CHECK(inserted).

  // Note: insert_or_assign, not insert, in case a record, so as to overwrite
  // any null entries introduced by cycles.

  ItemId id = GenerateItemId(*decl);
  if (import_cache_
          .try_emplace(decl,
                       ItemCacheEntry{
                           .status = ItemCacheEntry::Status::kInProgress,
                           .id = id,
                           .proto_item = nullptr,
                       })
          .second) {
    import_cache_insertion_order_.push_back(decl);
  }

  std::unique_ptr<ir_proto::Item> result = ImportDecl(decl);
  const ir_proto::Item* item_ptr = result.get();

  auto& entry = import_cache_.find(decl)->second;
  entry.status = result != nullptr ? ItemCacheEntry::Status::kCompleted
                                   : ItemCacheEntry::Status::kFailed;
  entry.proto_item = std::move(result);

  if (auto* record_decl = clang::dyn_cast<clang::CXXRecordDecl>(decl)) {
    // TODO(forster): Should we even visit the nested decl if we couldn't
    // import the parent? For now we have tests that check that we generate
    // error messages for those decls, so we're visiting.
    ImportDeclsFromDeclContext(*record_decl);
  }

  // Logic for `ClassTemplateSpecializationDecl`: insert them into
  // `class_template_instantiations_`, so that they will get included in
  // the top-level items.
  // Note: The 'gating' logic here needs to be consistent with the 'gating'
  // logic in `GetItemIdsInSourceOrder`, or else the class template
  // instantiation decl ID is inserted into the top-level items but does
  // not have its corresponding IR item, resulting in lookup failures (crashes)
  // when generating bindings.
  if (item_ptr != nullptr) {
    // In lazy mode, alien decls are imported on demand, so make sure their
    // enclosing namespaces are imported too. Skip namespaces that are already
    // in the cache: in the common case the enclosing namespace is the one
    // currently being imported (kInProgress), and calling `GetDeclItem` on it
    // would only hit the re-entrancy path above.
    if (invocation_.lazy_import_alien_decls()) {
      for (clang::DeclContext* dc = GetEnclosingDeclContext(decl, *this);
           dc != nullptr && !dc->isTranslationUnit() &&
           !dc->isFunctionOrMethod();
           dc = dc->getParent()) {
        if (auto* ns_decl = clang::dyn_cast<clang::NamespaceDecl>(dc)) {
          if (!import_cache_.contains(ns_decl)) {
            GetDeclItem(ns_decl);
          }
        }
      }
    }
    if (auto* specialization_decl =
            llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(decl);
        specialization_decl && IsFromCurrentTarget(*specialization_decl)) {
      if (GetTemplateSpecializationAlias(specialization_decl) == nullptr) {
        class_template_instantiations_.insert(specialization_decl);
      }
    }
  }
  return item_ptr;
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

absl_nullable std::unique_ptr<ir_proto::Item> Importer::ImportDecl(
    clang::Decl* absl_nonnull decl) {
  const absl::StatusOr<bool> must_bind =
      HasAnnotationWithoutArgs(*decl, "crubit_must_bind");
  if (!must_bind.ok()) {
    return HardError(*decl, FormattedError::FromStatus(must_bind.status()));
  }

  if (IsTransitivelyInPrivate(decl)) {
    if (*must_bind) {
      return HardError(*decl,
                       FormattedError::Static(
                           "Private declarations cannot receive bindings"));
    }
    return nullptr;
  }

  const absl::StatusOr<bool> pub_crate =
      HasAnnotationWithoutArgs(*decl, "crubit_pub_crate");
  if (!pub_crate.ok()) {
    return HardError(*decl, FormattedError::FromStatus(pub_crate.status()));
  }

  const absl::StatusOr<bool> do_not_bind =
      HasAnnotationWithoutArgs(*decl, "crubit_do_not_bind");
  if (!do_not_bind.ok()) {
    return HardError(*decl, FormattedError::FromStatus(do_not_bind.status()));
  }
  if (*do_not_bind) {
    if (*must_bind) {
      return HardError(
          *decl, FormattedError::Static("Conflicting CRUBIT_MUST_BIND and "
                                        "CRUBIT_DO_NOT_BIND annotations"));
    }
    if (*pub_crate) {
      return HardError(
          *decl, FormattedError::Static("Conflicting CRUBIT_PUB_CRATE and "
                                        "CRUBIT_DO_NOT_BIND annotations"));
    }
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
    return nullptr;
  }

  if (*pub_crate) {
    if (clang::isa<clang::TypeDecl>(decl) ||
        clang::isa<clang::ClassTemplateDecl>(decl) ||
        clang::isa<clang::TypeAliasTemplateDecl>(decl)) {
      return HardError(
          *decl, FormattedError::Static(
                     "`CRUBIT_PUB_CRATE` cannot be used on types; it can only "
                     "be used on functions and methods"));
    }
    if (!clang::isa<clang::FunctionDecl>(decl)) {
      return HardError(
          *decl,
          FormattedError::Static(
              "`CRUBIT_PUB_CRATE` can only be used on functions and methods"));
    }
  }

  std::string unavailable_error;
  if (decl->isUnavailable(&unavailable_error)) {
    return ImportUnsupportedItem(
        *decl, std::nullopt,
        {FormattedError::PrefixedStrCat("Decl is unavailable: ",
                                        std::move(unavailable_error))},
        /*is_hard_error=*/*must_bind);
  }

  // Types spelled in `decl` are governed by the file containing `decl`, for
  // the purpose of `#pragma nullability file_default`. Nested imports (e.g. of
  // members) set their own governing file and restore ours afterwards.
  llvm::SaveAndRestore governing_file(
      governing_file_, ctx_.getSourceManager()
                           .getDecomposedExpansionLoc(decl->getLocation())
                           .first);
  for (auto& importer : decl_importers_) {
    std::unique_ptr<ir_proto::Item> result =
        importer->ImportDecl(decl, *must_bind);
    if (result != nullptr) {
      if (*must_bind) {
        SetMustBindItem(*result);
      }
      if (*pub_crate) {
        if (result->has_func()) {
          result->mutable_func()->set_is_pub_crate(true);
        }
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
  return nullptr;
}

const ir_proto::Item* absl_nullable Importer::GetImportedItem(
    const clang::Decl& decl) const {
  auto it = import_cache_.find(&decl);
  if (it != import_cache_.end()) {
    return it->second.proto_item.get();
  }
  return nullptr;
}

BazelLabel Importer::GetOwningTarget(const clang::Decl& decl) const {
  // Inherited friend operators are imported as if they were hidden friends of
  // the derived record, which belongs to the current target. See
  // `ImportInheritedFriendOperators`.
  if (&decl == inherited_friend_operator_) {
    return invocation_.target_;
  }

  // Template instantiations need to be generated in the target that triggered
  // the instantiation (not in the target where the template is defined).
  if (IsFullClassTemplateSpecializationOrChild(&decl)) {
    return invocation_.target_;
  }

  // Built-in functions are defined by the compiler and are not associated with
  // any target. Without this check, the decl item ID will show up in the IR
  // as the ID of a top-level item, but the function itself will be missing from
  // the IR's list of items, resulting in a crash when generating bindings.
  if (IsBuiltinFunction(&decl)) {
    return BazelLabel("//:virtual_clang_resource_dir_target");
  }

  return GetOwningTargetOfLocation(decl.getLocation());
}

BazelLabel Importer::GetOwningTargetOfLocation(
    clang::SourceLocation source_location) const {
  clang::SourceManager& source_manager = ctx_.getSourceManager();

  // If the header this location comes from is not associated with a target,
  // and appears to be a textual header, then we go up the include stack
  // until we find a header that has an owning target.

  std::optional<std::string> filename;

  while (source_location.isValid()) {
    if (source_location.isMacroID()) {
      source_location = source_manager.getExpansionLoc(source_location);
    }
    auto id = source_manager.getFileID(source_location);
    std::optional<llvm::StringRef> raw_filename =
        source_manager.getNonBuiltinFilenameForID(id);
    if (!raw_filename) {
      return BazelLabel("//:_nothing_should_depend_on_private_builtin_hdrs");
    }
    // On Windows, Clang may use `\` as a path separator (e.g. `.\../../a.h` or
    // `../..\a.h`), but header names from the command-line flags use `/`.
    filename = llvm::sys::path::convert_to_slash(*raw_filename);
    if (filename->starts_with("./")) {
      filename->erase(0, 2);
    }

    if (auto target = invocation_.header_target(HeaderName(*filename))) {
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
      absl::StrCat(kUnknownTargetPrefix, absl::string_view(*filename)));
}

bool Importer::IsFromCurrentTarget(const clang::Decl& decl) const {
  return invocation_.target_ == GetOwningTarget(decl);
}

bool Importer::IsFromCurrentTargetAndNotUnderSpecialization(
    const clang::Decl& decl) const {
  if (IsFullClassTemplateSpecializationOrChild(&decl)) return false;
  return IsFromCurrentTarget(decl);
}

bool Importer::RefersToOwnedDefinition(const clang::CXXRecordDecl& decl) const {
  absl::flat_hash_set<const clang::CXXRecordDecl*> visited;
  return RefersToOwnedDefinitionImpl(decl, visited);
}

bool Importer::RefersToOwnedDefinitionImpl(
    const clang::CXXRecordDecl& decl,
    absl::flat_hash_set<const clang::CXXRecordDecl*>& visited) const {
  if (!visited.insert(&decl).second) return false;

  const clang::CXXRecordDecl* def = decl.getDefinition();
  if (def != nullptr &&
      !clang::isa<clang::ClassTemplateSpecializationDecl>(decl)) {
    if (IsFromCurrentTargetAndNotUnderSpecialization(*def)) return true;
  }

  if (auto* spec =
          clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(&decl)) {
    auto primary_or_partial = spec->getSpecializedTemplateOrPartial();
    if (auto* primary =
            primary_or_partial.dyn_cast<clang::ClassTemplateDecl*>()) {
      if (auto* primary_def = primary->getTemplatedDecl()->getDefinition()) {
        if (IsFromCurrentTargetAndNotUnderSpecialization(*primary_def) ||
            (primary_def != &decl &&
             RefersToOwnedDefinitionImpl(*primary_def, visited))) {
          return true;
        }
      }
    } else if (auto* partial =
                   primary_or_partial.dyn_cast<
                       clang::ClassTemplatePartialSpecializationDecl*>()) {
      if (auto* partial_def = partial->getDefinition()) {
        if (IsFromCurrentTargetAndNotUnderSpecialization(*partial_def) ||
            (partial_def != &decl &&
             RefersToOwnedDefinitionImpl(*partial_def, visited))) {
          return true;
        }
      }
    }

    for (const clang::TemplateArgument& arg :
         spec->getTemplateArgs().asArray()) {
      if (arg.getKind() == clang::TemplateArgument::Type) {
        if (clang::CXXRecordDecl* arg_record =
                arg.getAsType()->getAsCXXRecordDecl()) {
          if (RefersToOwnedDefinitionImpl(*arg_record, visited)) {
            return true;
          }
        } else if (auto* tst =
                       arg.getAsType()
                           ->getAs<clang::TemplateSpecializationType>()) {
          if (auto* sub_record = tst->getAsCXXRecordDecl()) {
            if (RefersToOwnedDefinitionImpl(*sub_record, visited)) {
              return true;
            }
          }
        }
      }
    }
  }

  if (def != nullptr) {
    for (const clang::FieldDecl* field : def->fields()) {
      if (clang::CXXRecordDecl* field_record =
              field->getType()->getAsCXXRecordDecl()) {
        if (RefersToOwnedDefinitionImpl(*field_record, visited)) return true;
      } else if (auto* tst = field->getType()
                                 ->getAs<clang::TemplateSpecializationType>()) {
        if (auto* sub_record = tst->getAsCXXRecordDecl()) {
          if (RefersToOwnedDefinitionImpl(*sub_record, visited)) return true;
        }
      }
    }
    for (const clang::CXXBaseSpecifier& base : def->bases()) {
      if (clang::CXXRecordDecl* base_record =
              base.getType()->getAsCXXRecordDecl()) {
        if (RefersToOwnedDefinitionImpl(*base_record, visited)) return true;
      } else if (auto* tst = base.getType()
                                 ->getAs<clang::TemplateSpecializationType>()) {
        if (auto* sub_record = tst->getAsCXXRecordDecl()) {
          if (RefersToOwnedDefinitionImpl(*sub_record, visited)) return true;
        }
      }
    }
  }

  return false;
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

bool Importer::IsFeatureEnabledForTarget(const BazelLabel& label,
                                         absl::string_view feature) const {
  if (auto i = invocation_.ir_.crubit_features().find(label.value());
      i != invocation_.ir_.crubit_features().end()) {
    for (const auto& f : i->second) {
      if (f == feature) return true;
    }
  }
  return false;
}

// LINT.IfChange
bool Importer::AreAssumedLifetimesEnabledForTarget(
    const BazelLabel& label) const {
  // TODO(b/530193579): After expanding features earlier, just check the
  // specific feature without manually checking for "all" or
  // "no_assume_lifetimes".
  return (IsFeatureEnabledForTarget(label, "assume_lifetimes") ||
          IsFeatureEnabledForTarget(label, "all")) &&
         !IsFeatureEnabledForTarget(label, "no_assume_lifetimes");
}
// LINT.ThenChange(//depot/common/crubit_feature.rs,
// //depot/features/BUILD)

bool Importer::IsUnsafeViewEnabledForTarget(const BazelLabel& label) const {
  return IsFeatureEnabledForTarget(label, "unsafe_view");
}

bool Importer::IsRecordImplDebugEnabledForTarget(
    const BazelLabel& label) const {
  // TODO(b/530193579): After expanding features earlier, just check the
  // specific feature without manually checking for "all".
  return IsFeatureEnabledForTarget(label, "record_impl_debug") ||
         IsFeatureEnabledForTarget(label, "all");
}

absl::StatusOr<bool> Importer::DetectFormatter(
    const clang::TypeDecl& decl) const {
  CRUBIT_ASSIGN_OR_RETURN(std::optional<bool> override_display,
                          GetCrubitOverrideDisplayAnnotation(decl));
  if (override_display.has_value()) {
    return *override_display;
  }
  clang::CanQualType type = ctx_.getCanonicalTypeDeclType(&decl);
  CRUBIT_ASSIGN_OR_RETURN(
      std::optional<bool> found,
      DetectFormatterForType(/*lookup=*/type, /*target=*/type));
  return found.value_or(false);
}

bool Importer::ImplementsCoreFmtDebug(const clang::TypeDecl& type) const {
  if (rs_core_fmt_debug_.isNull()) {
    return false;
  }

  for (const clang::ClassTemplateDecl* impl_decl : {rs_impl_, rs_std_impl_}) {
    if (impl_decl == nullptr) {
      continue;
    }
    for (const clang::ClassTemplateSpecializationDecl* absl_nonnull spec :
         impl_decl->specializations()) {
      const clang::TemplateArgumentList& template_args =
          spec->getTemplateArgs();
      if (template_args.size() != 2) {
        continue;
      }
      if (!clang::ASTContext::hasSameType(template_args[0].getAsType(),
                                          ctx_.getTypeDeclType(&type))) {
        continue;
      }
      if (!clang::ASTContext::hasSameType(template_args[1].getAsType(),
                                          rs_core_fmt_debug_)) {
        continue;
      }

      const clang::CXXRecordDecl* absl_nullable def = spec->getDefinition();
      if (def == nullptr) {
        return false;
      }

      for (const clang::Decl* absl_nonnull decl : def->decls()) {
        const auto* absl_nullable var = llvm::dyn_cast<clang::VarDecl>(decl);
        if (var == nullptr || var->getName() != "kIsImplemented") {
          continue;
        }
        const clang::Expr* absl_nullable initializer = var->getAnyInitializer();
        if (initializer == nullptr) {
          break;
        }

        clang::Expr::EvalResult result;
        if (!initializer->EvaluateAsConstantExpr(result, ctx_) ||
            !result.Val.isInt()) {
          break;
        }
        return result.Val.getInt().getBoolValue();
      }
      return false;
    }
  }

  return false;
}

absl::StatusOr<std::optional<bool>> Importer::GetCrubitOverrideDebugAnnotation(
    const clang::TypeDecl& type) const {
  CRUBIT_ASSIGN_OR_RETURN(std::optional<AnnotateArgs> args,
                          GetAnnotateAttrArgs(type, "crubit_override_debug"));
  if (!args.has_value()) {
    return std::nullopt;
  }
  if (args->size() != 1) {
    return absl::InvalidArgumentError(
        "crubit_override_debug annotation must have exactly one argument");
  }
  CRUBIT_ASSIGN_OR_RETURN(bool result, GetExprAsBool(*args->front(), ctx_));
  return result;
}

absl::StatusOr<std::optional<bool>>
Importer::GetCrubitOverrideDisplayAnnotation(
    const clang::TypeDecl& decl) const {
  CRUBIT_ASSIGN_OR_RETURN(std::optional<AnnotateArgs> args,
                          GetAnnotateAttrArgs(decl, "crubit_override_display"));
  if (!args.has_value()) {
    return std::nullopt;
  }
  if (args->size() != 1) {
    return absl::InvalidArgumentError(
        "crubit_override_display annotation must have exactly one argument");
  }
  CRUBIT_ASSIGN_OR_RETURN(bool result, GetExprAsBool(*args->front(), ctx_));
  return result;
}

absl::StatusOr<std::optional<bool>> Importer::DetectFormatterForType(
    clang::CanQualType lookup, clang::CanQualType target) const {
  const clang::Type* lookup_type = lookup.getTypePtrOrNull();
  const clang::Type* target_type = target.getTypePtrOrNull();
  if (lookup_type == nullptr) {
    std::string error_message;
    llvm::raw_string_ostream os(error_message);
    os << "unexpectedly null lookup type, while finding formatter for ";
    if (target_type == nullptr) {
      os << "(also unexpected) null type";
    } else {
      target_type->dump(os, ctx_);
    }
    return absl::InternalError(error_message);
  }
  if (target_type == nullptr) {
    std::string error_message;
    llvm::raw_string_ostream os(error_message);
    os << "unexpectedly null target type, while finding its formatter in ";
    lookup_type->dump(os, ctx_);
    return absl::InternalError(error_message);
  }

  if (const clang::CXXRecordDecl* record = lookup->getAsCXXRecordDecl();
      record != nullptr) {
    for (const clang::FriendDecl* absl_nonnull friend_decl :
         record->friends()) {
      const clang::NamedDecl* inner_friend = friend_decl->getFriendDecl();
      if (inner_friend != nullptr &&
          DetectFormatterFunction(*inner_friend, target)) {
        return true;
      }
    }
    for (const clang::CXXBaseSpecifier& base_specifier : record->bases()) {
      clang::CanQualType base_type =
          ctx_.getCanonicalType(base_specifier.getType());
      CRUBIT_ASSIGN_OR_RETURN(
          std::optional<bool> detected_for_target_in_base,
          DetectFormatterForType(/*lookup=*/base_type, /*target=*/target));
      if (detected_for_target_in_base.has_value()) {
        return *detected_for_target_in_base;
      }

      auto* base_decl = base_specifier.getType()->getAsTagDecl();
      if (base_decl == nullptr) {
        std::string error_message;
        llvm::raw_string_ostream os(error_message);
        os << "base ";
        base_specifier.getType()->dump(os, ctx_);
        os << " of ";
        record->dump(os);
        os << " unexpectedly not a TagDecl, while finding formatter for ";
        target_type->dump(os, ctx_);
        return absl::InternalError(error_message);
      }
      CRUBIT_ASSIGN_OR_RETURN(bool detected_for_base,
                              DetectFormatter(*base_decl));
      if (detected_for_base) {
        return detected_for_base;
      }
    }
  }
  const clang::TagDecl* lookup_decl = lookup_type->getAsTagDecl();
  if (lookup_decl == nullptr) {
    return std::nullopt;
  }
  const clang::DeclContext* absl_nonnull context =
      lookup_decl->getDeclContext()->getEnclosingNamespaceContext();
  for (const clang::Decl* absl_nonnull decl : context->decls()) {
    if (DetectFormatterFunction(*decl, target)) {
      return true;
    }
  }
  return std::nullopt;
}

bool Importer::IsCrubitEnabledForTarget(const BazelLabel& label) const {
  if (auto i = invocation_.ir_.crubit_features().find(label.value());
      i != invocation_.ir_.crubit_features().end()) {
    return !i->second.empty();
  }
  return false;
}

std::unique_ptr<ir_proto::Item> Importer::HardError(const clang::Decl& decl,
                                                    FormattedError error) {
  return ImportUnsupportedItem(decl, std::nullopt, {std::move(error)},
                               /*is_hard_error=*/true);
}

std::unique_ptr<ir_proto::Item> Importer::ImportUnsupportedItem(
    const clang::Decl& original_decl,
    std::optional<ir_proto::UnsupportedItem::Path> path,
    std::vector<FormattedError> errors, bool is_hard_error) {
  ir_proto::UnsupportedItem::Kind kind = ir_proto::UnsupportedItem::OTHER;
  const clang::Decl* decl = &original_decl;
  while (auto* using_decl = clang::dyn_cast<clang::UsingShadowDecl>(decl)) {
    decl = using_decl->getTargetDecl();
  }
  if (const clang::TagDecl* named_decl =
          clang::dyn_cast<clang::TagDecl>(decl)) {
    switch (named_decl->getTagKind()) {
      case clang::TagTypeKind::Struct:
        kind = ir_proto::UnsupportedItem::STRUCT;
        break;
      case clang::TagTypeKind::Class:
        kind = ir_proto::UnsupportedItem::CLASS;
        break;
      case clang::TagTypeKind::Enum:
        kind = ir_proto::UnsupportedItem::ENUM;
        break;
      case clang::TagTypeKind::Union:
        kind = ir_proto::UnsupportedItem::UNION;
        break;
      default:
        break;
    }
  } else if (const clang::FunctionDecl* func_decl =
                 clang::dyn_cast<clang::FunctionDecl>(decl)) {
    kind = func_decl->getKind() == clang::NamedDecl::Kind::CXXConstructor
               ? ir_proto::UnsupportedItem::CONSTRUCTOR
               : ir_proto::UnsupportedItem::FUNC;
  } else if (clang::isa<clang::NamespaceDecl>(decl)) {
    kind = ir_proto::UnsupportedItem::NAMESPACE;
  } else if (clang::isa<clang::VarDecl>(decl)) {
    kind = ir_proto::UnsupportedItem::GLOBAL_VAR;
  } else if (clang::isa<clang::FunctionTemplateDecl>(decl)) {
    kind = ir_proto::UnsupportedItem::FUNC;
  } else if (clang::isa<clang::ClassTemplateDecl>(decl)) {
    kind = ir_proto::UnsupportedItem::CLASS;
  } else if (clang::isa<clang::TypeAliasDecl>(decl) ||
             clang::isa<clang::TypedefNameDecl>(decl)) {
    kind = ir_proto::UnsupportedItem::TYPE_ALIAS;
  }
  std::string name = "unnamed";
  if (const auto* named_decl =
          clang::dyn_cast<clang::NamedDecl>(&original_decl)) {
    name = named_decl->getQualifiedNameAsString();
  }
  std::optional<std::string> inline_cpp_source_text;
  if (invocation_.allow_incomplete_migration()) {
    clang::SourceManager& sm = ctx_.getSourceManager();
    bool invalid = false;
    llvm::StringRef text = clang::Lexer::getSourceText(
        clang::CharSourceRange::getTokenRange(original_decl.getBeginLoc(),
                                              original_decl.getEndLoc()),
        sm, ctx_.getLangOpts(), &invalid);
    if (!invalid) {
      inline_cpp_source_text = text.str();
    }
  }

  auto item = std::make_unique<ir_proto::Item>();
  auto* unsupported = item->mutable_unsupported_item();
  unsupported->set_name(std::move(name));
  unsupported->set_unique_name(GetUniqueName(original_decl));
  unsupported->set_kind(kind);
  if (path.has_value()) {
    *unsupported->mutable_path() = std::move(*path);
  }
  for (const auto& err : errors) {
    *unsupported->add_errors() = err.AsFfi();
  }
  *unsupported->mutable_decl_loc() =
      ConvertDeclLocation(original_decl.getBeginLoc(), nullptr);
  unsupported->set_id(GenerateItemId(original_decl).value());
  unsupported->set_defining_target(GetOwningTarget(original_decl).value());
  unsupported->set_must_bind(is_hard_error || invocation_.carcinize_mode() ==
                                                  CarcinizeMode::kStrict);
  if (inline_cpp_source_text.has_value()) {
    unsupported->set_inline_cpp_source_text(*inline_cpp_source_text);
  }
  return item;
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

std::optional<std::string> Importer::GetComment(const clang::Decl& decl) const {
  // This does currently not distinguish between different types of comments.
  // In general it is not possible in C++ to reliably only extract doc comments.
  // This is going to be a heuristic that needs to be tuned over time.

  clang::SourceManager& sm = ctx_.getSourceManager();
  const clang::RawComment* raw_comment = GetRawComment(decl);

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

ir_proto::DeclLocation Importer::ConvertDeclLocation(
    clang::SourceLocation loc,
    clang::DeclarationNameInfo* absl_nullable name_info) const {
  const clang::SourceManager& sm = ctx_.getSourceManager();
  // Returns the path of the file containing `file_loc`, or an empty string if
  // it's unknown.
  auto get_path = [&sm](clang::SourceLocation file_loc) {
    absl::string_view path = StringViewFromStringRef(sm.getFilename(file_loc));
    absl::ConsumePrefix(&path, "./");
    return path;
  };

  ir_proto::DeclLocation result;
  // For macros (https://clang.llvm.org/doxygen/SourceManager_8h.html), the
  // spelling location is where the macro is defined, and the expansion
  // location is where it is expanded.
  bool spelling_known = false;
  if (absl::string_view path = get_path(sm.getSpellingLoc(loc));
      !path.empty()) {
    spelling_known = true;
    ir_proto::SourceLocation& spelling = *result.mutable_spelling();
    spelling.set_path(path);
    spelling.set_line(sm.getSpellingLineNumber(loc));
  }
  if (invocation_.kythe_annotations()) {
    ir_proto::DeclLocation::ByteRange& name_range =
        *result.mutable_name_range();
    name_range.set_begin(0);
    name_range.set_end(0);
    if (spelling_known && name_info != nullptr) {
      // This gets us most of the way there, but getting it right in generality
      // is tricky (see ClangRangeFinder::RangeForNameInfo for details).
      auto tr =
          clang::CharSourceRange::getTokenRange(name_info->getSourceRange());
      auto sr =
          clang::Lexer::getAsCharRange(tr, sm, ctx_.getLangOpts()).getAsRange();
      name_range.set_begin(sm.getFileOffset(sr.getBegin()));
      name_range.set_end(sm.getFileOffset(sr.getEnd()));
    }
  }
  if (loc.isMacroID()) {
    if (absl::string_view path = get_path(sm.getExpansionLoc(loc));
        !path.empty()) {
      ir_proto::SourceLocation& expansion = *result.mutable_expansion();
      expansion.set_path(path);
      expansion.set_line(sm.getExpansionLineNumber(loc));
    }
  }
  return result;
}

std::optional<std::string> Importer::CheckTemplateArgInstantiable(
    const clang::TemplateArgument& arg) {
  switch (arg.getKind()) {
    case clang::TemplateArgument::ArgKind::Pack:
      for (const clang::TemplateArgument& pack_arg : arg.pack_elements()) {
        if (std::optional<std::string> errors =
                CheckTemplateArgInstantiable(pack_arg);
            errors.has_value()) {
          return errors;
        }
      }
      return std::nullopt;
    case clang::TemplateArgument::ArgKind::Type:
      break;
    case clang::TemplateArgument::ArgKind::Declaration:
    case clang::TemplateArgument::ArgKind::Template:
    case clang::TemplateArgument::ArgKind::Expression:
    case clang::TemplateArgument::ArgKind::Integral:
    case clang::TemplateArgument::ArgKind::NullPtr:
    case clang::TemplateArgument::ArgKind::TemplateExpansion:
    case clang::TemplateArgument::ArgKind::StructuralValue:
    case clang::TemplateArgument::ArgKind::Null:
      return std::nullopt;
  }

  // Look through pointers, references and arrays: `Foo<Bar*>` is instantiated
  // by the generated code just like `Foo<Bar>` is, and instantiating it may
  // well require `Bar` to be instantiated too (e.g. because `Foo` constrains
  // its parameter with a concept).
  clang::QualType type =
      StripPointersReferencesAndArrays(ctx_, arg.getAsType());

  clang::CXXRecordDecl* arg_record_decl = type->getAsCXXRecordDecl();
  if (arg_record_decl == nullptr) {
    return std::nullopt;
  }
  // Some failed instantiations (e.g. with a field of incomplete type) *are*
  // marked invalid by Clang. Attempting them again reports nothing new, so
  // this has to be checked explicitly.
  if (arg_record_decl->isInvalidDecl()) {
    return absl::StrCat("Template argument '", type.getAsString(),
                        "' is an invalid declaration.");
  }
  auto* arg_specialization_decl =
      clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(arg_record_decl);
  if (arg_specialization_decl == nullptr) {
    return std::nullopt;
  }
  return CheckSpecializationInstantiable(arg_specialization_decl);
}

std::optional<std::string> Importer::CheckSpecializationInstantiable(
    clang::ClassTemplateSpecializationDecl* absl_nonnull specialization_decl) {
  const clang::CXXRecordDecl* absl_nonnull key =
      specialization_decl->getCanonicalDecl();
  if (auto it = instantiation_attempts_.find(key);
      it != instantiation_attempts_.end()) {
    return it->second;
  }

  // The template arguments are checked first, and their failures are inherited.
  //
  // This is what makes the result independent of the order in which the
  // translation unit is traversed. Clang reports the errors of a failed
  // instantiation only once, and leaves behind a complete but ill-formed
  // definition. So once `std::variant<Incomplete>` has been (unsuccessfully)
  // instantiated, instantiating `std::reverse_iterator<const
  // std::variant<Incomplete>*>` silently succeeds -- even though the generated
  // bindings for it would not compile, because in the generated
  // `..._rs_api_impl.cc` the `CRUBIT_SIZEOF` assertion is what triggers the
  // instantiation of the variant for the first time.
  //
  // TODO(zarko): This only looks at template arguments. A specialization whose
  // instantiation already failed is still accepted when it is reached only
  // through a field, a base, or a function body, e.g.
  //
  //   template <typename T>
  //   struct UsesViaMember { NeedsComplete<typename T::type> f; };
  //
  // once `NeedsComplete<Incomplete>` has failed: completing
  // `UsesViaMember<HasIncompleteType>` then emits no diagnostics. Follow-up
  // plan, avoiding mutation of Clang's AST: factor out a
  // `CheckTypeInstantiable` (looking through pointers, references and arrays),
  // apply it to the fields and bases of successfully completed specializations
  // (with a cycle guard for self-referential types), expose it via
  // `ImportContext`, and apply it to the types referenced in the bodies
  // walked by `GetInvalidCallTarget`. Even then, implicit destructor calls and
  // virtual members instantiated along with the vtable are not visible to that
  // walk.
  std::optional<std::string> errors;
  if (specialization_decl->isInvalidDecl()) {
    // Already failed (and was marked invalid by Clang) before we got here.
    // Attempting it again would report nothing.
    errors = "The template specialization is an invalid declaration.";
  }
  for (const clang::TemplateArgument& arg :
       specialization_decl->getTemplateArgs().asArray()) {
    if (errors.has_value()) break;
    errors = CheckTemplateArgInstantiable(arg);
  }

  if (!errors.has_value()) {
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
        crubit::RecordDiagnosticsAndMarkFailedInstantiations(*this, [&] {
          // Attempt to instantiate.
          (void)sema_.isCompleteType(
              specialization_decl->getLocation(),
              ctx_.getCanonicalTagType(specialization_decl));
        });
    if (diagnostic_recorder.getNumErrors() != 0) {
      errors = diagnostic_recorder.ConcatenatedDiagnostics();
    }
  }

  // Note: no iterator from the lookup above can be reused here, because the
  // recursive calls and the instantiation attempt may have inserted into
  // `instantiation_attempts_`.
  instantiation_attempts_.insert_or_assign(key, errors);
  return errors;
}

CcType Importer::ConvertTemplateSpecializationType(
    const clang::TemplateSpecializationType& type, bool assume_lifetimes) {
  // Qualifiers are handled separately in TypeMapper::ConvertQualType().
  std::string type_string = clang::QualType(&type, 0).getAsString();

  auto* specialization_decl =
      clang::dyn_cast_or_null<clang::ClassTemplateSpecializationDecl>(
          type.getAsCXXRecordDecl());
  if (!specialization_decl) {
    return CcType(FormattedError::Substitute(
        "Template specialization '$0' without an associated record decl "
        "is not supported.",
        type_string));
  }

  if (auto* template_decl = type.getTemplateName().getAsTemplateDecl()) {
    // Resolve template_decl to its definition if possible.
    if (auto* class_template_decl =
            clang::dyn_cast<clang::ClassTemplateDecl>(template_decl)) {
      if (clang::CXXRecordDecl* definition =
              class_template_decl->getTemplatedDecl()->getDefinition()) {
        if (auto* definition_template_decl =
                definition->getDescribedClassTemplate()) {
          template_decl = definition_template_decl;
        }
      }
    }
    auto target = GetOwningTarget(*template_decl);
    if (!IsCrubitEnabledForTarget(target)) {
      if (std::optional<absl::string_view> header =
              HeaderOfUnknownTarget(target);
          header.has_value()) {
        return CcType(FormattedError::Substitute(
            "crubit.rs/errors/unknown_target: Failed to complete template "
            "specialization type $0:\n"
            "the template is defined in\n"
            "  $1\n"
            "which is not a public header of any Crubit-enabled target that\n"
            "  $2\n"
            "depends on directly. Either enable Crubit on the library that "
            "provides the header,\n"
            "add that library as a direct dependency (and `#include` the "
            "header directly),\n"
            "or move the template to a public header.",
            type_string, *header, invocation_.target_.value()));
      }
      return CcType(FormattedError::Substitute(
          "Failed to complete template specialization type $0: template "
          "belongs to target $1, which does not support Crubit.",
          type_string, target.value()));
    }
  }

  if (HasBeenAlreadySuccessfullyImported(*specialization_decl))
    return WithAsWrittenTemplateArgs(ConvertTypeDecl(specialization_decl), type,
                                     assume_lifetimes);

  if (std::optional<std::string> instantiation_errors =
          CheckSpecializationInstantiable(specialization_decl);
      instantiation_errors.has_value()) {
    return CcType(FormattedError::Substitute(
        "Failed to complete template specialization type $0: Diagnostics "
        "emitted:\n$1",
        type_string, *instantiation_errors));
  }

  // TODO(lukasza): Limit specialization depth? (e.g. using
  // `isSpecializationDepthGreaterThan` from earlier prototypes).

  // Import record type arguments before the specialization itself. Importing a
  // specialization like `std::unique_ptr<T>` converts `T`, and if `T` has not
  // been imported yet, that imports `T` together with all of its members. If
  // one of those members mentions `std::unique_ptr<T>` (e.g. a
  // `static std::unique_ptr<T> Create()` factory), it would find the
  // specialization still in progress, and fail to be imported. Importing `T`
  // first means that `T`'s members are imported while the specialization is
  // not in progress yet, and that the specialization later finds `T` already
  // imported.
  //
  // Arguments that are themselves specializations are skipped, so that this
  // doesn't import specializations (e.g. `std::allocator<T>`) that the
  // specialization's importer would not.
  for (const clang::TemplateArgument& arg :
       specialization_decl->getTemplateArgs().asArray()) {
    if (arg.getKind() != clang::TemplateArgument::Type) continue;
    auto* arg_record = arg.getAsType()->getAsCXXRecordDecl();
    if (arg_record == nullptr ||
        clang::isa<clang::ClassTemplateSpecializationDecl>(arg_record)) {
      continue;
    }
    // The result is discarded: if `T` can't be imported, the specialization's
    // importer reports that.
    (void)EnsureSuccessfullyImported(arg_record);
  }

  absl::Status import_status =
      CheckImportStatus(GetDeclItem(specialization_decl));
  if (!import_status.ok()) {
    return CcType(FormattedError::Substitute(
        "Failed to create bindings for template specialization type $0: $1",
        type_string, import_status.message()));
  }

  return WithAsWrittenTemplateArgs(ConvertTypeDecl(specialization_decl), type,
                                   assume_lifetimes);
}

CcType Importer::WithAsWrittenTemplateArgs(
    CcType converted, const clang::TemplateSpecializationType& type,
    bool assume_lifetimes) {
  // The specialization decl is shared by every use with the same canonical
  // arguments, so it cannot carry anything that differs between uses: e.g.
  // the nullability in `std::vector<absl_nonnull std::unique_ptr<T>>`, or the
  // lifetime in `absl::Span<View $a>`. Record the arguments as written here,
  // and let consumers prefer them over the decl's.
  //
  // Only arguments written as types are recorded, and only when all of them
  // are. Defaulted arguments (like `std::vector`'s allocator) are not written,
  // and so are not counted. Consumers that only look at one argument (see
  // `choose_one_type` in `rs_snippet.rs`) use the first, and binding a
  // specialization's own lifetime parameter (see `AddTemplateArgumentLifetime`)
  // is still limited to a single argument.
  if (!converted.variant().has_decl()) return converted;

  // An alias template specialization carries the arguments of the alias, not
  // of the class template it names: e.g. `c9::CoStatusOr<T>` is written with
  // `T`, but names `Co<absl::StatusOr<T>>`. Look through aliases to find the
  // arguments of the class template specialization itself.
  const clang::TemplateSpecializationType* specialization_type = &type;
  while (specialization_type->isTypeAlias()) {
    specialization_type = specialization_type->getAliasedType()
                              ->getAs<clang::TemplateSpecializationType>();
    if (specialization_type == nullptr) return converted;
  }

  llvm::ArrayRef<clang::TemplateArgument> args =
      specialization_type->template_arguments();
  if (args.empty()) return converted;
  for (const clang::TemplateArgument& arg : args) {
    if (arg.getKind() != clang::TemplateArgument::Type) return converted;
  }

  // Only convert the arguments if one of them can say something the decl's
  // arguments do not. Converting them is not free: it imports whatever the
  // arguments name into the current target, which can change that target's
  // bindings (e.g. which support headers its generated code includes).
  const bool has_explicit_lifetimes =
      assume_lifetimes &&
      absl::c_any_of(args, [this](const clang::TemplateArgument& arg) {
        return ContainsExplicitLifetimes(arg.getAsType());
      });
  const bool has_known_nullability =
      absl::c_any_of(args, [this](const clang::TemplateArgument& arg) {
        return ContainsKnownNullability(arg.getAsType());
      });
  if (!has_explicit_lifetimes && !has_known_nullability) return converted;

  std::vector<CcType> converted_args;
  converted_args.reserve(args.size());
  for (const clang::TemplateArgument& arg : args) {
    CcType converted_arg =
        ConvertQualType(arg.getAsType(), /*lifetimes=*/nullptr,
                        /*nullable=*/true, assume_lifetimes);
    if (converted_arg.variant().has_error()) {
      // A malformed lifetime annotation surfaces here as an error type, rather
      // than being silently dropped: the lifetime was written down in the
      // source, so dropping it would produce bindings that disagree with the
      // header.
      if (has_explicit_lifetimes) {
        return CcType(FormattedError::Substitute(
            "Failed to convert a template argument of '$0', which carries "
            "an explicit lifetime: $1",
            clang::QualType(&type, 0).getAsString(),
            converted_arg.variant().error().message()));
      }
      // Otherwise, fall back to the decl's arguments. They were converted
      // successfully when the specialization was imported, so this only loses
      // information that is specific to this use.
      return converted;
    }
    converted_args.push_back(std::move(converted_arg));
  }
  for (CcType& converted_arg : converted_args) {
    *converted.add_template_args() = std::move(converted_arg);
  }
  return converted;
}

CcType Importer::ConvertTypeDecl(clang::NamedDecl* absl_nonnull decl) {
  if (!EnsureSuccessfullyImported(decl)) {
    return CcType(FormattedError::Substitute(
        "No generated bindings found for '$0'", decl->getNameAsString()));
  }

  return CcTypeDecl(GenerateItemId(*decl));
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

std::optional<clang::NullabilityKind> Importer::GetDefaultNullability(
    const clang::Type& type) const {
  // By default, the type was spelled directly in the decl being imported.
  clang::FileID file = governing_file_;
  // But if it was spelled via a typedef, the type was written in the typedef's
  // declaration, so that file governs. With nested typedefs, the innermost one
  // is where the type was written.
  //
  // TODO(okabayashi): Handle substituted template arguments, e.g.
  // `std::vector<std::unique_ptr<T>>`, which should be governed by the file
  // in which the template argument was written.
  const clang::Type* current = &type;
  while (true) {
    if (const auto* typedef_type =
            clang::dyn_cast<clang::TypedefType>(current)) {
      file =
          ctx_.getSourceManager()
              .getDecomposedExpansionLoc(typedef_type->getDecl()->getLocation())
              .first;
    }
    const clang::Type* next =
        current->getLocallyUnqualifiedSingleStepDesugaredType().getTypePtr();
    if (next == current) break;
    current = next;
  }

  const clang::tidy::nullability::NullabilityPragmas& pragmas =
      invocation_.nullability_pragmas_;
  auto it = pragmas.find(file);
  if (it == pragmas.end()) return std::nullopt;
  return it->second;
}

bool Importer::ContainsKnownNullability(clang::QualType type) const {
  // `RecursiveASTVisitor` walks the type as written: it descends into
  // pointees, template arguments, and function parameter and return types,
  // but not into the underlying type of a typedef. That is fine, because both
  // `getNullability` and `GetDefaultNullability` see through typedefs.
  struct NullabilityFinder
      : public clang::RecursiveASTVisitor<NullabilityFinder> {
    const Importer& importer;
    bool found = false;
    explicit NullabilityFinder(const Importer& importer) : importer(importer) {}
    bool VisitType(clang::Type* type) {
      if (type->getNullability().has_value() ||
          (type->canHaveNullability(/*ResultIfUnknown=*/false) &&
           importer.GetDefaultNullability(*type).has_value())) {
        found = true;
        // Returning false stops the traversal.
        return false;
      }
      return true;
    }
  };
  NullabilityFinder finder(*this);
  finder.TraverseType(type);
  return finder.found;
}

absl::StatusOr<CcType> Importer::ConvertType(
    const clang::Type& type,
    const clang::tidy::lifetimes::ValueLifetimes* absl_nullable lifetimes,
    bool nullable, bool assume_lifetimes) {
  absl::StatusOr<CcType> cpp_type =
      ConvertUnattributedType(type, lifetimes, nullable, assume_lifetimes);
  if (cpp_type.ok()) {
    // Nullability is sugar: it lives on an `AttributedType` wrapping the type,
    // and `ConvertUnattributedType` looks straight through that. `type` is
    // still sugared here, so this is the last point at which we can see it.
    auto nullability = type.getNullability();
    if (!nullability.has_value() &&
        type.canHaveNullability(/*ResultIfUnknown=*/false)) {
      if (std::optional<clang::NullabilityKind> default_nullability =
              GetDefaultNullability(type);
          default_nullability.has_value()) {
        nullability = *default_nullability;
      }
    }
    cpp_type->set_is_nonnull(nullability.has_value() &&
                             *nullability == clang::NullabilityKind::NonNull);
    // Raw pointers record non-nullness in their kind. Both kinds still map to
    // a Rust raw pointer; this only makes the information available to codegen.
    if (cpp_type->variant().has_pointer() &&
        cpp_type->variant().pointer().kind() == PointerTypeKind::NULLABLE &&
        cpp_type->is_nonnull()) {
      cpp_type->mutable_variant()->mutable_pointer()->set_kind(
          PointerTypeKind::NON_NULL);
    }

    std::optional<std::string> unknown_attr =
        CollectUnknownTypeAttrs(type, [](clang::attr::Kind kind) {
          using enum clang::attr::Kind;
          switch (kind) {
            // annotate_type is usually meaningless and can be acked as
            // understood. The major exception is lifetimes, which we do
            // already handle separately.
            case AnnotateType:
            // Nullability is read above, via `Type::getNullability()`. These
            // cases only mark the attributes as recognized, so that they are
            // not reported as unknown; removing them would cost every
            // annotated type its bindings.
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
      cpp_type->set_unknown_attr(*unknown_attr);
    }
    if (assume_lifetimes) {
      CRUBIT_ASSIGN_OR_RETURN(
          std::vector<absl::string_view> explicit_lifetime_views,
          CollectExplicitLifetimes(ctx_, type));
      for (absl::string_view lifetime_view : explicit_lifetime_views) {
        cpp_type->add_explicit_lifetimes(lifetime_view);
      }
      CRUBIT_RETURN_IF_ERROR(AddTemplateArgumentLifetime(
          type, /*has_written_lifetimes=*/!explicit_lifetime_views.empty(),
          *cpp_type));
    }
  }
  return cpp_type;
}

// Returns the type that `type` spells, looking only through attributes (such
// as a lifetime) and parentheses. Unlike `getAs<T>()`, this does not look
// through typedefs: their underlying type is converted, and so handled,
// separately.
static const clang::Type& StripAttributes(const clang::Type& type) {
  const clang::Type* current = &type;
  while (true) {
    if (const auto* attributed =
            clang::dyn_cast<clang::AttributedType>(current)) {
      current = attributed->getModifiedType().getTypePtr();
    } else if (const auto* macro_qualified =
                   clang::dyn_cast<clang::MacroQualifiedType>(current)) {
      current = macro_qualified->getUnderlyingType().getTypePtr();
    } else if (const auto* paren = clang::dyn_cast<clang::ParenType>(current)) {
      current = paren->getInnerType().getTypePtr();
    } else {
      return *current;
    }
  }
}

absl::Status Importer::AddTemplateArgumentLifetime(const clang::Type& type,
                                                   bool has_written_lifetimes,
                                                   CcType& cpp_type) {
  const clang::Type& spelled_type = StripAttributes(type);

  // A use of a specialization that takes a lifetime parameter for its template
  // argument, e.g. `View<int& $a>`: the specialization is shared with
  // `View<int& $b>`, so bind its parameter to the lifetime written on this
  // use's argument, which `WithAsWrittenTemplateArgs` recorded.
  if (const auto* specialization_type =
          clang::dyn_cast<clang::TemplateSpecializationType>(&spelled_type)) {
    const auto* specialization_decl =
        clang::dyn_cast_or_null<clang::ClassTemplateSpecializationDecl>(
            specialization_type->getAsCXXRecordDecl());
    if (specialization_decl == nullptr || !cpp_type.variant().has_decl() ||
        cpp_type.template_args().size() != 1 ||
        !GetTemplateArgumentLifetimeParam(*specialization_decl).has_value()) {
      return absl::OkStatus();
    }
    const auto& arg_lifetimes =
        cpp_type.template_args()[0].explicit_lifetimes();
    if (arg_lifetimes.empty()) return absl::OkStatus();
    std::string type_string = clang::QualType(&type, 0).getAsString();
    if (has_written_lifetimes) {
      // E.g. `View<int& $a> $b`. Both stand for the same lifetime parameter.
      return absl::InvalidArgumentError(absl::StrCat(
          "lifetimes are written both on the template argument of '",
          type_string,
          "' and on the type itself; write them only on the "
          "template argument"));
    }
    if (arg_lifetimes.size() != 1) {
      return absl::InvalidArgumentError(absl::StrCat(
          "the template argument of '", type_string,
          "' must have exactly one lifetime, but has ", arg_lifetimes.size()));
    }
    *cpp_type.mutable_explicit_lifetimes() = arg_lifetimes;
    return absl::OkStatus();
  }

  // Inside such a specialization, a type spelled as its template parameter,
  // e.g. `T value` in `View<int&>`: it has the specialization's lifetime
  // parameter.
  //
  // Only the template parameter spelled directly counts. A member typedef of
  // the specialization (`using type = T;`) also desugars to the parameter, but
  // it can be named from outside, as `Holder<int* $a>::type`, where the
  // specialization's lifetime parameter is not in scope.
  //
  // TODO(zarko): Also handle member typedefs used inside the specialization,
  // and bind `Holder<int* $a>::type` to `$a`.
  //
  // A reference to the template parameter counts too when the template
  // argument is itself a reference, e.g. `const T&` in `View<int&>`: the
  // references collapse into the argument's reference, so the type refers to
  // the same object as the argument.
  if (!cpp_type.explicit_lifetimes().empty()) return absl::OkStatus();
  const clang::Type* parameter_type = &spelled_type;
  if (const auto* reference_type =
          clang::dyn_cast<clang::ReferenceType>(&spelled_type);
      reference_type != nullptr && reference_type->isInnerRef()) {
    parameter_type =
        &StripAttributes(*reference_type->getPointeeTypeAsWritten());
  }
  const auto* subst_type =
      clang::dyn_cast<clang::SubstTemplateTypeParmType>(parameter_type);
  if (subst_type == nullptr || subst_type->getIndex() != 0) {
    return absl::OkStatus();
  }
  const auto* specialization_decl =
      clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(
          subst_type->getAssociatedDecl());
  if (specialization_decl == nullptr) return absl::OkStatus();
  if (std::optional<std::string> lifetime =
          GetTemplateArgumentLifetimeParam(*specialization_decl)) {
    cpp_type.clear_explicit_lifetimes();
    cpp_type.add_explicit_lifetimes(*lifetime);
  }
  return absl::OkStatus();
}

absl::StatusOr<CcType> Importer::ConvertUnattributedType(
    const clang::Type& type,
    const clang::tidy::lifetimes::ValueLifetimes* absl_nullable lifetimes,
    bool nullable, bool assume_lifetimes) {
  // Qualifiers are handled separately in ConvertQualType().
  std::string type_string = clang::QualType(&type, 0).getAsString();

  CRUBIT_ASSIGN_OR_RETURN(
      const clang::AnnotateTypeAttr* crubit_owned_ptr_attr,
      GetTypeAnnotationSingleDecl(&type, "crubit_owned_pointer"));

  bool is_owned_ptr = crubit_owned_ptr_attr != nullptr;

  if (is_owned_ptr && !type.isPointerType()) {
    return absl::InvalidArgumentError(
        "CRUBIT_OWNED_POINTER can only be applied to pointer types.");
  }

  assert(!lifetimes || IsSameCanonicalUnqualifiedType(
                           lifetimes->Type(), clang::QualType(&type, 0)));

  if (auto primitive = GetPrimitive(type); primitive.has_value()) {
    return *std::move(primitive);
  } else if (type.isPointerType() || type.isLValueReferenceType() ||
             type.isRValueReferenceType()) {
    clang::QualType pointee_type = type.getPointeeType();
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
      CRUBIT_CHECK(!lifetime.has_value() ||
                   (lifetime->value() ==
                    clang::tidy::lifetimes::Lifetime::Static().Id()));

      CRUBIT_ASSIGN_OR_RETURN(
          CallingConv cc_call_conv,
          ConvertCcCallConvToSupportedCallingConv(func_type->getCallConv()));

      CcType func_ptr_type;
      auto* fp = func_ptr_type.mutable_variant()->mutable_func_pointer();
      fp->set_non_null(type.isLValueReferenceType());
      fp->set_call_conv(cc_call_conv);
      for (unsigned i = 0; i < func_type->getNumParams(); ++i) {
        const clang::tidy::lifetimes::ValueLifetimes* param_lifetimes = nullptr;
        if (pointee_lifetimes) {
          param_lifetimes =
              &pointee_lifetimes->GetFuncLifetimes().GetParamLifetimes(i);
        }
        CcType cpp_param_type =
            ConvertQualType(func_type->getParamType(i), param_lifetimes,
                            /*nullable=*/true, assume_lifetimes);
        *fp->add_param_and_return_types() = std::move(cpp_param_type);
      }

      const clang::tidy::lifetimes::ValueLifetimes* return_lifetimes = nullptr;
      if (pointee_lifetimes) {
        return_lifetimes =
            &pointee_lifetimes->GetFuncLifetimes().GetReturnLifetimes();
      }
      CcType cpp_return_type =
          ConvertQualType(func_type->getReturnType(), return_lifetimes,
                          /*nullable=*/true, assume_lifetimes);
      *fp->add_param_and_return_types() = std::move(cpp_return_type);

      CRUBIT_CHECK(type.isPointerType() || type.isLValueReferenceType());
      return func_ptr_type;
    }
    CcType cpp_pointee_type = ConvertQualType(
        pointee_type, pointee_lifetimes, /*nullable=*/true, assume_lifetimes);
    // Note: we don't check for a lifetime here and prefer to defer to the
    // IR consumer to error if a lifetime is required. This allows the IR
    // consumer to infer a lifetime where-appropriate (e.g. constructors).
    if (type.isPointerType()) {
      if (is_owned_ptr) {
        return OwnedPointerTo(std::move(cpp_pointee_type), lifetime);
      } else {
        return PointerTo(std::move(cpp_pointee_type), lifetime, nullable);
      }
    } else if (type.isLValueReferenceType()) {
      return LValueReferenceTo(std::move(cpp_pointee_type), lifetime);
    } else {
      CRUBIT_CHECK(type.isRValueReferenceType());
      return RValueReferenceTo(std::move(cpp_pointee_type), lifetime);
    }
  } else if (const auto* builtin_type =
                 // Use getAsAdjusted instead of getAs so we don't desugar
                 // typedefs.
             type.getAsAdjusted<clang::BuiltinType>()) {
    switch (builtin_type->getKind()) {
      case clang::BuiltinType::Bool:
        return CcType(Primitive::BOOL);
      case clang::BuiltinType::Void:
        return CcType(Primitive::VOID);

      // Floating-point numbers
      //
      // TODO(b/255768062): Generated bindings should explicitly check if
      // `math.h` defines the `__STDC_IEC_559__` macro.
      case clang::BuiltinType::Float:
        return CcType(Primitive::FLOAT);
      case clang::BuiltinType::Double:
        return CcType(Primitive::DOUBLE);

      // `char`
      case clang::BuiltinType::Char_S:  // 'char' in targets where it's signed
      case clang::BuiltinType::Char_U:  // 'char' in targets where it's unsigned
        return CcType(Primitive::CHAR);
      case clang::BuiltinType::SChar:  // 'signed char', explicitly qualified
        return CcType(Primitive::SIGNED_CHAR);
      case clang::BuiltinType::UChar:  // 'unsigned char', explicitly qualified
        return CcType(Primitive::UNSIGNED_CHAR);

      // Signed integers
      case clang::BuiltinType::Short:
        return CcType(Primitive::SHORT);
      case clang::BuiltinType::Int:
        return CcType(Primitive::INT);
      case clang::BuiltinType::Long:
        return CcType(Primitive::LONG);
      case clang::BuiltinType::LongLong:
        return CcType(Primitive::LONG_LONG);
      case clang::BuiltinType::Int128:
        return CcType(Primitive::INT128);

      // Unsigned integers
      case clang::BuiltinType::UShort:
        return CcType(Primitive::UNSIGNED_SHORT);
      case clang::BuiltinType::UInt:
        return CcType(Primitive::UNSIGNED_INT);
      case clang::BuiltinType::ULong:
        return CcType(Primitive::UNSIGNED_LONG);
      case clang::BuiltinType::ULongLong:
        return CcType(Primitive::UNSIGNED_LONG_LONG);
      case clang::BuiltinType::UInt128:
        return CcType(Primitive::UNSIGNED_INT128);

      case clang::BuiltinType::Char16:
        return CcType(Primitive::CHAR16_T);
      case clang::BuiltinType::Char32:
        return CcType(Primitive::CHAR32_T);
      default:
        return absl::UnimplementedError("Unsupported builtin type");
    }
  } else if (const auto* tag_type = type.getAsAdjusted<clang::TagType>()) {
    // With precompiled header modules, the definition may come from a module
    // that was loaded, but not imported by the headers of the current target.
    // Such a definition can't be used in the generated C++ code, so the type is
    // treated as incomplete instead (see also `CXXRecordDeclImporter::Import`).
    clang::TagDecl* tag_decl = tag_type->getDecl();
    if (clang::TagDecl* definition = tag_decl->getDefinition();
        definition != nullptr && sema_.hasReachableDefinition(definition)) {
      tag_decl = definition;
    }
    return ConvertTypeDecl(tag_decl);
  } else if (const auto* typedef_type =
                 type.getAsAdjusted<clang::TypedefType>()) {
    if (!IsInSystemHeader(typedef_type->getDecl()->getLocation()) &&
        EnsureSuccessfullyImported(typedef_type->getDecl())) {
      return ConvertTypeDecl(typedef_type->getDecl());
    }
    // The underlying type was written in the typedef's file, so that file's
    // nullability default governs the types within it, such as its template
    // arguments.
    llvm::SaveAndRestore governing_file(
        governing_file_,
        ctx_.getSourceManager()
            .getDecomposedExpansionLoc(typedef_type->getDecl()->getLocation())
            .first);
    return ConvertQualType(typedef_type->getDecl()->getUnderlyingType(),
                           lifetimes, /*nullable=*/true, assume_lifetimes);
  } else if (const auto* using_type = type.getAs<clang::UsingType>()) {
    return ConvertTypeDecl(using_type->getDecl());
  } else if (const auto* tst_type =
                 type.getAs<clang::TemplateSpecializationType>()) {
    // Desugar alias templates to arrays (e.g., `template <class T, int N> using
    // Arr = T[N]`).
    if (tst_type->isTypeAlias() && type.isArrayType()) {
      return ConvertQualType(tst_type->getAliasedType(), lifetimes,
                             /*nullable=*/true, assume_lifetimes);
    }
    return ConvertTemplateSpecializationType(*tst_type, assume_lifetimes);
  } else if (const auto* subst_type =
                 type.getAs<clang::SubstTemplateTypeParmType>()) {
    return ConvertQualType(subst_type->getReplacementType(), lifetimes,
                           /*nullable=*/true, assume_lifetimes);
  } else if (const auto* deduced_type = type.getAs<clang::DeducedType>()) {
    // Deduction should have taken place earlier (e.g. via DeduceReturnType
    // called from FunctionDeclImporter::Import).
    CRUBIT_CHECK(deduced_type->isDeduced());
    return ConvertQualType(deduced_type->getDeducedType(), lifetimes,
                           /*nullable=*/true, assume_lifetimes);
  } else if (const auto* decltype_type = type.getAs<clang::DecltypeType>()) {
    return ConvertQualType(decltype_type->getUnderlyingType(), lifetimes,
                           /*nullable=*/true, assume_lifetimes);
  } else if (const auto* array_type =
                 ctx_.getAsConstantArrayType(clang::QualType(&type, 0))) {
    std::optional<int64_t> num_elements = array_type->getSize().trySExtValue();
    if (!num_elements.has_value()) {
      // C-style arrays could technically be size_t, but practically shouldn't
      // be that big.
      return absl::UnimplementedError("Array size exceeds int64_t");
    }
    if (*num_elements == 0) {
      // Don't support the GNU extension for zero-length arrays (which may
      // actually be flexible arrays / not a constant size).
      return absl::UnimplementedError("Zero-length C arrays are not supported");
    }
    if (!IsSupportedArrayElementType(ctx_, array_type->getElementType())) {
      return absl::UnimplementedError(
          "C-style arrays of non-primitive, non-enum types are not yet "
          "supported (b/575922365)");
    }
    CcType element_type =
        ConvertQualType(array_type->getElementType(), /*lifetimes=*/nullptr,
                        /*nullable=*/true, assume_lifetimes);
    CcType result;
    auto* arr = result.mutable_variant()->mutable_constant_array();
    arr->set_num_elements(static_cast<size_t>(*num_elements));
    *arr->mutable_element_type() = std::move(element_type);
    return result;
  }

  return absl::UnimplementedError(absl::StrCat(
      "Unsupported clang::Type class '", type.getTypeClassName(), "'"));
}

CcType Importer::ConvertQualType(
    clang::QualType qual_type,
    const clang::tidy::lifetimes::ValueLifetimes* absl_nullable lifetimes,
    bool nullable, bool assume_lifetimes) {
  clang::PrintingPolicy printing_policy = ctx_.getPrintingPolicy();
  printing_policy.FullyQualifiedName = true;
  std::string type_string = qual_type.getAsString(printing_policy);

  absl::StatusOr<CcType> type = ConvertType(*qual_type.getTypePtr(), lifetimes,
                                            nullable, assume_lifetimes);

  // TODO(b/468093766): make _all_ the functions return a CcType.
  // Then we can remove this duplicated logic: "either it was bad and returned
  // as a bad status, or it was bad and returned as a CcType with a
  // FormattedError variant". That's a bit ridiculous! But the remaining uses
  // of Status are a bit tricky and don't fit in a first draft.
  if (!type.ok()) {
    return CcType(FormattedError::Substitute(
        "Unsupported type '$0': $1", type_string, type.status().message()));
  }
  if (type->variant().has_error()) {
    // TODO(jeanpierreda): The most-specific type should go at the start, and
    // be something we can aggregate by somehow.
    // TODO(jeanpierreda): This produces pretty verbose errors.
    return CcType(
        FormattedError::Substitute("Unsupported type '$0': $1", type_string,
                                   type->variant().error().message()));
  }

  // Handle cv-qualification.
  type->set_is_const(qual_type.isConstQualified());
  if (qual_type.isVolatileQualified()) {
    return CcType(FormattedError::PrefixedStrCat(
        "Unsupported `volatile` qualifier", type_string));
  }

  return *std::move(type);
}

std::string Importer::GetUniqueName(const clang::Decl& decl) const {
  llvm::SmallString<128> usr;
  if (!clang::index::generateUSRForDecl(&decl, usr)) {
    return std::string(usr.str());
  }

  // Note: we shouldn't get this far for anything with a real name, and we
  // can't really fall back to GetMangledName unconditionally because
  // mangleName crashes at least for types like the implicit `__uint128_t`
  // typedef.
  return "";
}

std::string Importer::GetMangledName(const clang::NamedDecl& named_decl) const {
  if (auto tag_decl = clang::dyn_cast<clang::TagDecl>(&named_decl)) {
    // Mangled tag names are used to 1) provide valid Rust identifiers for
    // C++ template specializations, 2) help build unique names for virtual
    // upcast thunks, and 3) provide cfi_encoding for structs and enums.
    //
    // All three consumers want an Itanium-mangled name, regardless of the C++
    // ABI of the target platform:
    // * 1) and 2) need a name that is a valid Rust and C++ identifier.  The
    //   Microsoft mangling scheme uses characters such as `?`, `@` and `$`,
    //   which are not valid in identifiers.
    // * 3) `#[cfi_encoding]` is defined by `rustc` in terms of the Itanium C++
    //   ABI mangling, on all target platforms.
    //   TODO(b/561578105): Verify the CFI story on MSVC-ABI targets.  `rustc`
    //   does not support CFI on `*-windows-msvc` and `clang-cl` derives CFI
    //   type ids from Microsoft-mangled names, so the Itanium name emitted
    //   here is expected to be unused (rather than wrong) there.
    //
    // Therefore we deliberately use `itanium_tag_name_mangler_` here, rather
    // than the target platform's `mangler_`.
    llvm::SmallString<128> storage;
    llvm::raw_svector_ostream buffer(storage);
    itanium_tag_name_mangler_->mangleCanonicalTypeName(
        ctx_.getCanonicalTagType(tag_decl), buffer);

    // The Itanium mangler does not provide a way to get the mangled
    // representation of a type. Instead, we call mangleTypeName() that
    // returns the name of the RTTI typeinfo symbol, and remove the _ZTS
    // prefix.
    constexpr llvm::StringRef kZtsPrefix = "_ZTS";
    CRUBIT_CHECK(buffer.str().take_front(4) == kZtsPrefix);
    llvm::StringRef mangled_tag_name =
        buffer.str().drop_front(kZtsPrefix.size());

    if (clang::isa<clang::ClassTemplateSpecializationDecl>(named_decl)) {
      // We prepend __CcTemplateInst to reduce chances of conflict
      // with regular C and C++ structs.
      // TODO(b/526962187): this will break if we use these for CFI.
      constexpr llvm::StringRef kCcTemplatePrefix = "__CcTemplateInst";
      return llvm::formatv("{0}{1}", kCcTemplatePrefix, mangled_tag_name);
    }
    return std::string(mangled_tag_name);
  }

  if (!mangler_->shouldMangleDeclName(&named_decl)) {
    return named_decl.getIdentifier()->getName().str();
  }

  clang::GlobalDecl decl;

  // There are only three named decl types that don't work with the GlobalDecl
  // unary constructor: GPU kernels (which do not exist in standard C++, so we
  // ignore), constructors, and destructors. GlobalDecl does not support
  // constructors and destructors from the unary constructor because there is
  // more than one global declaration for a given constructor or destructor!
  //
  //   * (Ctor|Dtor)_Complete is a function which constructs / destroys the
  //     entire object. This is what we want. :)  (Except for destructors in
  //     the Microsoft C++ ABI - see below.)
  //   * Dtor_Deleting is a function which additionally calls operator delete.
  //   * (Ctor|Dtor)_Base is a function which constructs/destroys the object but
  //     NOT including virtual base class subobjects.
  //   * (Ctor|Dtor)_Comdat: I *believe* this is the identifier used to
  //     deduplicate inline functions, and is not callable.
  //   * Ctor_(Copying|Default)Closure: These only exist in the MSVC++ ABI.
  //     They are compiler-generated wrappers with a simpler signature, for
  //     constructors with default arguments or a non-default calling
  //     convention (used for `dllexport` and for copying exception objects).
  //     Crubit does not call them.
  //
  // It was hard to piece this together, so writing it down here to explain why
  // we magically picked the *_Complete variants.
  if (auto dtor = clang::dyn_cast<clang::CXXDestructorDecl>(&named_decl)) {
    clang::CXXDtorType dtor_type = clang::CXXDtorType::Dtor_Complete;
    // In the Microsoft C++ ABI, the translation unit that defines a destructor
    // normally only emits the base destructor (`??1`).  The complete destructor
    // (`??_D`, "vbase dtor") is emitted on demand by its callers.  For classes
    // without virtual bases the two variants are equivalent, and Clang also
    // calls the base destructor in this case (see
    // `MicrosoftCXXABI::EmitDestructorCall`).
    if (ctx_.getTargetInfo().getCXXABI().isMicrosoft() &&
        dtor->getParent()->getNumVBases() == 0) {
      dtor_type = clang::CXXDtorType::Dtor_Base;
    }
    decl = clang::GlobalDecl(dtor, dtor_type);
  } else if (auto ctor =
                 clang::dyn_cast<clang::CXXConstructorDecl>(&named_decl)) {
    decl = clang::GlobalDecl(ctor, clang::CXXCtorType::Ctor_Complete);
  } else {
    decl = clang::GlobalDecl(&named_decl);
  }

  std::string name;
  llvm::raw_string_ostream stream(name);
  mangler_->mangleName(decl, stream);
  stream.flush();
  return name;
}

std::optional<ir_proto::UnsupportedItem::Path>
Importer::GetUnsupportedItemPathForTemplateDecl(
    clang::RedeclarableTemplateDecl* absl_nonnull template_decl) {
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
  auto names = GetTranslatedName(*template_decl);
  if (!names.ok()) {
    return std::nullopt;
  }
  ir_proto::UnsupportedItem::Path path;
  *path.mutable_ident() = std::move(names->cc_identifier);
  if (enclosing_item_id->has_value()) {
    path.set_enclosing_item_id((*enclosing_item_id)->value());
  }
  return path;
}

std::string Importer::GetNameForSourceOrder(const clang::Decl& decl) const {
  // Implicit class template specializations and their methods all have the
  // same source location. In order to provide deterministic order of the
  // respective items in generated source code, we additionally use the
  // mangled names when sorting the items.
  if (auto* class_template_specialization_decl =
          clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(&decl)) {
    return GetMangledName(*class_template_specialization_decl);
  } else if (auto* func_decl = clang::dyn_cast<clang::FunctionDecl>(&decl)) {
    return GetMangledName(*func_decl);
  } else if (auto* friend_decl = clang::dyn_cast<clang::FriendDecl>(&decl)) {
    if (auto* named_decl = friend_decl->getFriendDecl()) {
      if (auto function_template_decl =
              clang::dyn_cast<clang::FunctionTemplateDecl>(named_decl)) {
        // Reach through the function template declaration for a function that
        // can be mangled.
        named_decl = function_template_decl->getTemplatedDecl();
      }
      return GetMangledName(*named_decl);
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
    const clang::NamedDecl& named_decl) const {
  std::optional<UnqualifiedIdentifier> crubit_rust_name =
      CrubitRustName(named_decl);
  switch (named_decl.getDeclName().getNameKind()) {
    case clang::DeclarationName::Identifier: {
      auto name = std::string(named_decl.getName());
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
      // The string_view -> raw_string_view renaming happens centrally, so that
      // it applies to both records (if we use preferred_name renaming) and
      // type aliases (if preferred_name does not exist).
      if (name == "string_view" && named_decl.isInStdNamespace()) {
        if (!crubit_rust_name.has_value()) {
          crubit_rust_name =
              UnqualifiedIdentifier(Identifier("raw_string_view"));
        }
      }

      return TranslatedUnqualifiedIdentifier{
          .cc_identifier = UnqualifiedIdentifier(Identifier(name)),
          .crubit_rust_name = std::move(crubit_rust_name),
      };
    }
    case clang::DeclarationName::CXXConstructorName:
      return TranslatedUnqualifiedIdentifier{
          .cc_identifier = UnqualifiedIdentifier::Constructor(),
          .crubit_rust_name = std::move(crubit_rust_name),
      };
    case clang::DeclarationName::CXXDestructorName:
      return TranslatedUnqualifiedIdentifier{
          .cc_identifier = UnqualifiedIdentifier::Destructor(),
          .crubit_rust_name = std::move(crubit_rust_name),
      };
    case clang::DeclarationName::CXXOperatorName:
      switch (named_decl.getDeclName().getCXXOverloadedOperator()) {
        case clang::OO_None:
          LOG(FATAL) << "No OO_None expected under CXXOperatorName branch";
        case clang::NUM_OVERLOADED_OPERATORS:
          LOG(FATAL) << "No NUM_OVERLOADED_OPERATORS expected at runtime";
          // clang-format off
        #define OVERLOADED_OPERATOR(name, spelling, ...)  \
        case clang::OO_##name: {                          \
          return TranslatedUnqualifiedIdentifier{ \
            .cc_identifier = UnqualifiedIdentifier(Operator(spelling)), \
            .crubit_rust_name = std::move(crubit_rust_name), \
          }; \
        }
        #include "clang/Basic/OperatorKinds.def"
        #undef OVERLOADED_OPERATOR
          // clang-format on
      }
      LOG(FATAL) << "The `switch` above should handle all cases";
    case clang::DeclarationName::CXXConversionFunctionName:
      return TranslatedUnqualifiedIdentifier{
          .cc_identifier = UnqualifiedIdentifier::ConversionOperator(),
          .crubit_rust_name = std::move(crubit_rust_name),
      };
    default:
      // There are also e.g. literal operators, deduction guides, etc., but
      // we might not need to implement them at all. Full list at:
      // https://clang.llvm.org/doxygen/classclang_1_1DeclarationName.html#a9ab322d434446b43379d39e41af5cbe3
      return absl::UnimplementedError(
          absl::StrCat("Unsupported name: ", named_decl.getNameAsString()));
  }
}

absl::StatusOr<TranslatedIdentifier> Importer::GetTranslatedIdentifier(
    const clang::NamedDecl& named_decl) const {
  CRUBIT_ASSIGN_OR_RETURN(TranslatedUnqualifiedIdentifier unqualified,
                          GetTranslatedName(named_decl));
  CRUBIT_CHECK(unqualified.cc_identifier.has_ident())
      << "Incorrectly called with a special name";
  Identifier cc_identifier(unqualified.cc_identifier.ident().identifier());

  TranslatedIdentifier translated_identifiers = {
      .cc_identifier = cc_identifier,
  };

  if (!unqualified.crubit_rust_name.has_value()) {
    translated_identifiers.crubit_rust_name = std::move(cc_identifier);
    return translated_identifiers;
  }

  CRUBIT_CHECK(unqualified.crubit_rust_name->has_ident())
      << "Crubit rust name cannot be a special name";

  translated_identifiers.crubit_rust_name =
      Identifier(unqualified.crubit_rust_name->ident().identifier());

  return translated_identifiers;
}

void Importer::MarkAsSuccessfullyImported(const clang::NamedDecl& decl) {
  known_type_decls_.insert(
      clang::cast<clang::NamedDecl>(CanonicalizeDecl(decl)));
}

bool Importer::HasBeenAlreadySuccessfullyImported(
    const clang::NamedDecl& decl) const {
  return known_type_decls_.contains(
      clang::cast<clang::NamedDecl>(CanonicalizeDecl(decl)));
}

void Importer::MarkAsInvalidTemplateSpecialization(
    const clang::ClassTemplateSpecializationDecl& decl, std::string reason) {
  // Overwrites an earlier recorded success: that just means the failure was
  // observed by someone other than `CheckSpecializationInstantiable`.
  instantiation_attempts_.insert_or_assign(decl.getCanonicalDecl(),
                                           std::move(reason));
}

bool Importer::IsInvalidTemplateSpecialization(
    const clang::CXXRecordDecl& decl) const {
  if (decl.isInvalidDecl()) return true;
  auto it = instantiation_attempts_.find(decl.getCanonicalDecl());
  return it != instantiation_attempts_.end() && it->second.has_value();
}

clang::TypedefNameDecl* absl_nullable Importer::GetTemplateSpecializationAlias(
    clang::Decl* absl_nonnull decl) const {
  clang::ClassTemplateSpecializationDecl* spec_decl =
      clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(decl);
  if (!spec_decl) {
    return nullptr;
  }
  for (const auto* preferred_name :
       spec_decl->getMostRecentDecl()
           ->specific_attrs<clang::PreferredNameAttr>()) {
    clang::QualType preferred_type = preferred_name->getTypedefType();
    if (!clang::declaresSameEntity(preferred_type->getAsCXXRecordDecl(),
                                   spec_decl)) {
      continue;
    }
    while (true) {
      if (auto* typedef_decl =
              clang::dyn_cast<clang::TypedefType>(preferred_type)) {
        // TODO(b/485041750): For Some Reason this doesn't work for std::pmr...
        // un-allowlist this once we figure out why.
        // TODO(b/485041750): This doesn't work for std::string because the
        // wrapper type expects to use the original C++ type name, not the
        // version produced here.
        std::string qualified_name =
            typedef_decl->getDecl()->getQualifiedNameAsString();
        if (qualified_name != "std::string_view" &&
            qualified_name != "std::wstring_view") {
          return nullptr;
        }
        return typedef_decl->getDecl();
      }
      clang::QualType next_preferred_type =
          preferred_type.getSingleStepDesugaredType(ctx_);
      if (next_preferred_type == preferred_type) {
        break;
      }
      preferred_type = std::move(next_preferred_type);
    }
    return nullptr;
  }
  return nullptr;
}

}  // namespace crubit

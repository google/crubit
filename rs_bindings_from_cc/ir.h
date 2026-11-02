// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This file defines an intermediate representation (IR) used between Clang AST
// and code generators that generate Rust bindings and C++ bindings
// implementation.
//
// All types in this file own their data. This IR is expected to outlive the
// Clang's AST context, therefore it cannot reference data owned by it.
#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IR_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IR_H_

#include <stdint.h>

#include <algorithm>
#include <cstddef>
#include <iomanip>
#include <memory>
#include <optional>
#include <ostream>
#include <string>
#include <tuple>
#include <utility>
#include <vector>

#include "common/check.h"
#include "common/status.h"
#include "common/strong_int.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir_generated.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/RawCommentList.h"
#include "llvm/ADT/APSInt.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/DenseMapInfo.h"
#include "llvm/ADT/Hashing.h"
#include "llvm/ADT/SmallString.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/FormatVariadic.h"
#include "llvm/Support/raw_ostream.h"

namespace crubit {

namespace ir_proto = ::crubit::ffi;
namespace internal {
inline std::string Substitute(llvm::StringRef format,
                              llvm::ArrayRef<std::string> args) {
  std::string result;
  for (size_t i = 0; i < format.size(); ++i) {
    if (format[i] == '$' && i + 1 < format.size()) {
      if (format[i + 1] >= '0' && format[i + 1] <= '9') {
        size_t idx = format[i + 1] - '0';
        if (idx < args.size()) {
          result += args[idx];
        }
        ++i;
        continue;
      } else if (format[i + 1] == '$') {
        result += '$';
        ++i;
        continue;
      }
    }
    result += format[i];
  }
  return result;
}

template <typename T>
std::string ToStringForSubstitute(const T& t) {
  return (llvm::Twine(t)).str();
}
inline std::string ToStringForSubstitute(llvm::StringRef t) { return t.str(); }
inline std::string ToStringForSubstitute(const char* t) {
  return std::string(t);
}
inline std::string ToStringForSubstitute(const std::string& t) { return t; }

template <typename T>
inline void AppendToString(std::string& s, const T& t) {
  s += (llvm::Twine(t)).str();
}

inline void AppendToString(std::string& s, llvm::StringRef ref) {
  s += ref.str();
}
inline void AppendToString(std::string& s, const std::string& ref) { s += ref; }
inline void AppendToString(std::string& s, const char* ref) { s += ref; }
}  // namespace internal

struct Item;

namespace internal {
inline constexpr int kJsonIndent = 2;
}  // namespace internal

// Name of the top-level namespace in which `IrFromCc` generates code that
// triggers `cc_template!` class template instantiations.
inline constexpr llvm::StringRef kInstantiationsNamespaceName =
    "__cc_template_instantiations";

// A name of a public header of the C++ library.
class HeaderName {
 public:
  explicit HeaderName(llvm::StringRef name) { inner_.set_name(name); }

  llvm::StringRef IncludePath() const { return inner_.name(); }

  const ffi::HeaderName& AsFfi() const { return inner_; }
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator const ffi::HeaderName&() const { return inner_; }

  template <typename H>
  friend H AbslHashValue(H h, const HeaderName& header_name) {
    return H::combine(std::move(h), header_name.IncludePath());
  }

 private:
  // Header pathname in the format suitable for a quote include.
  ffi::HeaderName inner_;
};

inline bool operator==(const HeaderName& lhs, const HeaderName& rhs) {
  return lhs.IncludePath() == rhs.IncludePath();
}

inline std::ostream& operator<<(std::ostream& o, const HeaderName& h) {
  return o << h.IncludePath().str();
}

// An integer uniquely identifying an Item across the FFI IR so cross-item
// references (for example between types and records, or enclosing items for
// comments and unsupported declarations) do not depend on tree containment.
CRUBIT_DEFINE_STRONG_INT_TYPE(ItemId, uintptr_t);

inline std::string DebugStringFromDecl(const clang::Decl& decl) {
  auto canonical_decl_id = reinterpret_cast<uintptr_t>(decl.getCanonicalDecl());
  auto decl_id = reinterpret_cast<uintptr_t>(&decl);
  std::string decl_name;
  auto ostream = llvm::raw_string_ostream(decl_name);
  decl.print(ostream);
  ostream.flush();
  return llvm::formatv("Canonical DeclID: {0}; DeclID: {1}; decl: {2}",
                       canonical_decl_id, decl_id, decl_name)
      .str();
}

// A numerical ID that uniquely identifies a lifetime.
CRUBIT_DEFINE_STRONG_INT_TYPE(LifetimeId, int);

// An error that stores its format string as well as the formatted message.
class FormattedError final {
 public:
  auto operator<=>(const FormattedError& other) const {
    return std::make_tuple(fmt(), message()) <=>
           std::make_tuple(other.fmt(), other.message());
  }
  bool operator==(const FormattedError& other) const {
    return fmt() == other.fmt() && message() == other.message();
  }

  template <typename H>
  friend H AbslHashValue(H h, const FormattedError& e) {
    return H::combine(std::move(h), e.fmt(), e.message());
  }

  // Returns a FormattedError for a static string. The string is used as both
  // the format string and the formatted message. Intended to be used only with
  // string literals.
  template <size_t N>
  static FormattedError Static(const char (&array)[N]) {
    return FormattedError(array, array);
  }

  // Returns a FormattedError built with `absl::StrCat()`. The first argument is
  // taken as the format string. All arguments are concatenated to form the
  // formatted message, with an extra `": "` inserted after the first argument.
  template <size_t N, typename... Ts>
  static FormattedError PrefixedStrCat(const char (&prefix)[N],
                                       Ts&&... moreArgs) {
    std::string message;
    llvm::raw_string_ostream os(message);
    os << prefix << ": ";
    (os << ... << std::forward<Ts>(moreArgs));
    return FormattedError(prefix, message);
  }

  // Returns a FormattedError built with `internal::Substitute()`.
  template <size_t N, typename... Ts>
  static FormattedError Substitute(const char (&format)[N], Ts&&... args) {
    if constexpr (sizeof...(args) == 0) {
      return FormattedError(format, crubit::internal::Substitute(format, {}));
    } else {
      std::string str_args[] = {
          crubit::internal::ToStringForSubstitute(std::forward<Ts>(args))...};
      return FormattedError(
          format,
          crubit::internal::Substitute(
              format, llvm::ArrayRef<std::string>(str_args, sizeof...(args))));
    }
  }

  // Extracts a format string from a status payload, if present.
  static FormattedError FromStatus(llvm::Error error);

  llvm::StringRef fmt() const { return inner_.fmt(); }
  llvm::StringRef message() const { return inner_.message(); }

  const ffi::FormattedError& AsFfi() const { return inner_; }
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator const ffi::FormattedError&() const { return inner_; }
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator ffi::cc_type::Variant() const {
    return ffi::cc_type::Variant(inner_);
  }

  // Type URL for use as an `absl::Status` payload.
  static constexpr llvm::StringRef kFmtPayloadTypeUrl =
      "type.googleapis.com/crubit.FormattedError.fmt";

 private:
  FormattedError(llvm::StringRef fmt, llvm::StringRef message) {
    inner_.set_fmt(fmt);
    inner_.set_message(message);
  }

  ffi::FormattedError inner_;
};

using ir_proto::CallingConv;
using ir_proto::CcType;
using ir_proto::PointerTypeKind;
using ir_proto::Primitive;
using ir_proto::SafetyAnnotation;

llvm::StringRef PrimitiveSpelling(Primitive primitive);
CcType CcTypeDecl(ItemId id);
CcType PointerTo(CcType pointee_type, std::optional<LifetimeId> lifetime,
                 bool nullable);
CcType OwnedPointerTo(CcType pointee_type, std::optional<LifetimeId> lifetime);
CcType LValueReferenceTo(CcType pointee_type,
                         std::optional<LifetimeId> lifetime);
CcType RValueReferenceTo(CcType pointee_type,
                         std::optional<LifetimeId> lifetime);

// An identifier involved in bindings.
//
// For example, the identifier for the C++ function `int Add(int a, int b);`
// is `Identifier("Add")`.
//
// This also includes operator names, such as "operator==". Non-symbol tokens in
// the operator name are separated by a single space. For example:
//
//  * `Identifier("operator==")`
//  * `Identifier("operator new[]")`
//  * `Identifier("operator co_await")`
//
// Invariants:
//     `identifier` cannot be empty.
class Identifier {
 public:
  explicit Identifier(llvm::StringRef identifier) {
    CRUBIT_CHECK(!identifier.empty());
    inner_.set_identifier(identifier);
  }

  llvm::StringRef Ident() const { return inner_.identifier(); }
  const ffi::Identifier& AsFfi() const { return inner_; }
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator const ffi::Identifier&() const { return inner_; }

  template <typename H>
  friend H AbslHashValue(H h, const Identifier& i) {
    return H::combine(std::move(h), i.Ident());
  }

  bool operator==(const Identifier& other) const {
    return Ident() == other.Ident();
  }

 private:
  ffi::Identifier inner_;
};

inline std::ostream& operator<<(std::ostream& o, const Identifier& id) {
  return o << std::setw(internal::kJsonIndent) << id.Ident().str();
}

using IntegerConstant = ir_proto::IntegerConstant;

llvm::Expected<IntegerConstant> IntegerConstantFromAPValue(
    const llvm::APSInt& value);

using ir_proto::Operator;
using UnqualifiedIdentifier = ir_proto::UnqualifiedIdentifier;

struct TranslatedUnqualifiedIdentifier {
  UnqualifiedIdentifier cc_identifier;
  std::optional<UnqualifiedIdentifier> crubit_rust_name;

  UnqualifiedIdentifier& rs_identifier();
};

struct TranslatedIdentifier {
  Identifier cc_identifier;
  std::optional<Identifier> crubit_rust_name;
  Identifier& rs_identifier();
};

using ir_proto::BridgeType;
using ir_proto::OwnedPtrConfig;
using ir_proto::SpecialMemberFunc;
using ir_proto::TemplateArg;
using ir_proto::TemplateSpecialization;
using ir_proto::TraitDerives;
using ir_proto::TraitImplPolarity;

// A complete intermediate representation of bindings for publicly accessible
// declarations of a single C++ library.
template <typename T>
const T* _Nullable get_item_if(const ir_proto::Item& item) {
  if constexpr (std::is_same_v<T, ir_proto::Record>) {
    return item.has_record() ? &item.record() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::Func>) {
    return item.has_func() ? &item.func() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::Enum>) {
    return item.has_enum_decl() ? &item.enum_decl() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::UnsupportedItem>) {
    return item.has_unsupported_item() ? &item.unsupported_item() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::IncompleteRecord>) {
    return item.has_incomplete_record() ? &item.incomplete_record() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::Namespace>) {
    return item.has_namespace_decl() ? &item.namespace_decl() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::TypeAlias>) {
    return item.has_type_alias() ? &item.type_alias() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::Constant>) {
    return item.has_constant() ? &item.constant() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::GlobalVar>) {
    return item.has_global_var() ? &item.global_var() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::Comment>) {
    return item.has_comment() ? &item.comment() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::ExistingRustType>) {
    return item.has_existing_rust_type() ? &item.existing_rust_type() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::UseMod>) {
    return item.has_use_mod() ? &item.use_mod() : nullptr;
  }
}

template <typename T>
T* _Nullable get_item_if(ir_proto::Item& item) {
  if constexpr (std::is_same_v<T, ir_proto::Record>) {
    return item.has_record() ? item.mutable_record() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::Func>) {
    return item.has_func() ? item.mutable_func() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::Enum>) {
    return item.has_enum_decl() ? item.mutable_enum_decl() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::UnsupportedItem>) {
    return item.has_unsupported_item() ? item.mutable_unsupported_item()
                                       : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::IncompleteRecord>) {
    return item.has_incomplete_record() ? item.mutable_incomplete_record()
                                        : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::Namespace>) {
    return item.has_namespace_decl() ? item.mutable_namespace_decl() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::TypeAlias>) {
    return item.has_type_alias() ? item.mutable_type_alias() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::Constant>) {
    return item.has_constant() ? item.mutable_constant() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::GlobalVar>) {
    return item.has_global_var() ? item.mutable_global_var() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::Comment>) {
    return item.has_comment() ? item.mutable_comment() : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::ExistingRustType>) {
    return item.has_existing_rust_type() ? item.mutable_existing_rust_type()
                                         : nullptr;
  } else if constexpr (std::is_same_v<T, ir_proto::UseMod>) {
    return item.has_use_mod() ? item.mutable_use_mod() : nullptr;
  }
}

template <typename T>
std::vector<const T* _Nonnull> get_items_if(const ir_proto::IRProto& ir_proto) {
  std::vector<const T* _Nonnull> filtered_items;
  auto process_item = [&](auto& self, const ir_proto::Item& item) -> void {
    if (const T* val = get_item_if<T>(item)) {
      filtered_items.push_back(val);
    }
    if (item.has_record()) {
      for (const auto& child : item.record().children()) {
        self(self, child);
      }
    } else if (item.has_namespace_decl()) {
      for (const auto& child : item.namespace_decl().children()) {
        self(self, child);
      }
    }
  };

  // Sort target keys to iterate `top_level_items()` deterministically.
  std::vector<std::string> target_keys;
  target_keys.reserve(ir_proto.top_level_items().size());
  for (const auto& [target, _] : ir_proto.top_level_items()) {
    target_keys.push_back(std::string(target));
  }
  std::sort(target_keys.begin(), target_keys.end());

  for (const auto& target : target_keys) {
    const auto& item_list = ir_proto.top_level_items().at(target);
    for (const auto& item : item_list) {
      process_item(process_item, item);
    }
  }
  return filtered_items;
}

template <typename T>
std::vector<T* _Nonnull> get_items_if(ir_proto::IRProto& ir_proto) {
  std::vector<T* _Nonnull> filtered_items;
  auto process_item = [&](auto& self, ir_proto::Item& item) -> void {
    if (T* val = get_item_if<T>(item)) {
      filtered_items.push_back(val);
    }
    if (item.has_record()) {
      for (auto& child : *item.mutable_record()->mutable_children()) {
        self(self, child);
      }
    } else if (item.has_namespace_decl()) {
      for (auto& child : *item.mutable_namespace_decl()->mutable_children()) {
        self(self, child);
      }
    }
  };

  std::vector<std::string> target_keys;
  target_keys.reserve(ir_proto.top_level_items().size());
  for (const auto& [target, _] : ir_proto.top_level_items()) {
    target_keys.push_back(std::string(target));
  }
  std::sort(target_keys.begin(), target_keys.end());

  for (const auto& target : target_keys) {
    auto& item_list = (*ir_proto.mutable_top_level_items())[target];
    for (auto& item : item_list) {
      process_item(process_item, item);
    }
  }
  return filtered_items;
}

using IR = ir_proto::IRProto;

}  // namespace crubit

namespace llvm {

template <>
struct DenseMapInfo<crubit::HeaderName> {
  static inline crubit::HeaderName getEmptyKey() {
    return crubit::HeaderName("\x01<empty>");
  }
  static inline crubit::HeaderName getTombstoneKey() {
    return crubit::HeaderName("\x01<tombstone>");
  }
  static unsigned getHashValue(const crubit::HeaderName& Val) {
    return llvm::hash_value(Val.IncludePath());
  }
  static bool isEqual(const crubit::HeaderName& LHS,
                      const crubit::HeaderName& RHS) {
    return LHS == RHS;
  }
};

template <>
struct DenseMapInfo<crubit::Identifier> {
  static inline crubit::Identifier getEmptyKey() {
    return crubit::Identifier("\x01<empty>");
  }
  static inline crubit::Identifier getTombstoneKey() {
    return crubit::Identifier("\x01<tombstone>");
  }
  static unsigned getHashValue(const crubit::Identifier& Val) {
    return llvm::hash_value(Val.Ident());
  }
  static bool isEqual(const crubit::Identifier& LHS,
                      const crubit::Identifier& RHS) {
    return LHS == RHS;
  }
};

template <>
struct DenseMapInfo<crubit::FormattedError> {
  static inline crubit::FormattedError getEmptyKey() {
    return crubit::FormattedError::Static("\x01<empty>");
  }
  static inline crubit::FormattedError getTombstoneKey() {
    return crubit::FormattedError::Static("\x01<tombstone>");
  }
  static unsigned getHashValue(const crubit::FormattedError& Val) {
    return llvm::hash_combine(Val.fmt(), Val.message());
  }
  static bool isEqual(const crubit::FormattedError& LHS,
                      const crubit::FormattedError& RHS) {
    return LHS == RHS;
  }
};

}  // namespace llvm
#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_H_

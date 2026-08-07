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
#include <utility>
#include <variant>
#include <vector>

#include "absl/base/nullability.h"
#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "absl/strings/substitute.h"
#include "common/strong_int.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.pb.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/RawCommentList.h"
#include "llvm/ADT/APSInt.h"
#include "llvm/ADT/SmallString.h"
#include "llvm/Support/raw_ostream.h"

namespace crubit {

namespace ir_proto = ::crubit::rs_bindings_from_cc::ir_proto::flat;

struct Item;

namespace internal {
inline constexpr int kJsonIndent = 2;
}  // namespace internal

// A name of a public header of the C++ library.
class HeaderName {
 public:
  explicit HeaderName(std::string name) : name_(std::move(name)) {}

  absl::string_view IncludePath() const { return name_; }

  void WriteToProto(ir_proto::HeaderName& proto) const;

  template <typename H>
  friend H AbslHashValue(H h, const HeaderName& header_name) {
    return H::combine(std::move(h), header_name.name_);
  }

 private:
  // Header pathname in the format suitable for a quote include.
  std::string name_;
};

inline bool operator==(const HeaderName& lhs, const HeaderName& rhs) {
  return lhs.IncludePath() == rhs.IncludePath();
}

inline std::ostream& operator<<(std::ostream& o, const HeaderName& h) {
  return o << h.IncludePath();
}

// An int uniquely representing an Item. Since our IR AST is serialized to
// Protobuf, we need a way to restore graph edges that don't follow the tree
// structure (for example between types and records), as well as location of
// comments and items we don't yet support.
// We use ItemIds for this.
CRUBIT_DEFINE_STRONG_INT_TYPE(ItemId, uintptr_t);

inline std::string DebugStringFromDecl(const clang::Decl* decl) {
  auto canonical_decl_id =
      reinterpret_cast<uintptr_t>(decl->getCanonicalDecl());
  auto decl_id = reinterpret_cast<uintptr_t>(decl);
  std::string decl_name;
  auto ostream = llvm::raw_string_ostream(decl_name);
  decl->print(ostream);
  ostream.flush();
  return absl::StrFormat("Canonical DeclID: %d; DeclID: %d; decl: %s",
                         canonical_decl_id, decl_id, decl_name);
}

// A numerical ID that uniquely identifies a lifetime.
CRUBIT_DEFINE_STRONG_INT_TYPE(LifetimeId, int);

// An error that stores its format string as well as the formatted message.
class FormattedError final {
 public:
  auto operator<=>(const FormattedError&) const = default;

  template <typename H>
  friend H AbslHashValue(H h, const FormattedError& e) {
    return H::combine(std::move(h), e.fmt_, e.message_);
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
    return FormattedError(
        prefix, absl::StrCat(prefix, ": ", std::forward<Ts>(moreArgs)...));
  }

  // Returns a FormattedError built with `absl::Substitute()`.
  template <size_t N, typename... Ts>
  static FormattedError Substitute(const char (&format)[N], Ts&&... args) {
    return FormattedError(format,
                          absl::Substitute(format, std::forward<Ts>(args)...));
  }

  // Extracts a format string from a status payload, if present.
  static FormattedError FromStatus(absl::Status status);

  absl::string_view fmt() const { return fmt_; }
  absl::string_view message() const { return message_; }

  void WriteToProto(ir_proto::FormattedError& proto) const;

  // Type URL for use as an `absl::Status` payload.
  static constexpr absl::string_view kFmtPayloadTypeUrl =
      "type.googleapis.com/crubit.FormattedError.fmt";

 private:
  FormattedError(std::string fmt, std::string message)
      : fmt_(fmt), message_(message) {}

  // The format string that produced the error message, if available. This is
  // used as an aggregation key for error reports.
  std::string fmt_;
  // Explanation of why we couldn't generate bindings.
  std::string message_;
};

// Whether a function is annotated with `CRUBIT_UNSAFE` or
// `CRUBIT_DISABLE_UNSAFE`. `[[clang::unsafe_buffer_usage]]` is also considered
// unsafe.
enum class SafetyAnnotation : char { kDisableUnsafe, kUnsafe, kUnannotated };

rs_bindings_from_cc::ir_proto::flat::SafetyAnnotation ToFlatProto(
    SafetyAnnotation safety_annotation);

enum class PointerTypeKind {
  kRValueRef,
  kLValueRef,
  kNullable,
  kNonNull,
  kOwned
};

rs_bindings_from_cc::ir_proto::flat::PointerTypeKind ToFlatProto(
    PointerTypeKind pointer_type_kind);

// Calling conventions for functions that are supported by Crubit.
//
// This is a subset of the calling conventions supported by Clang.
enum class CallingConv {
  kC,              // __attribute__((cdecl))
  kX86VectorCall,  // __attribute__((vectorcall))
  kX86FastCall,    // __attribute__((fastcall))
  kX864ThisCall,   // __attribute__((thiscall))
  kX86StdCall,     // __attribute__((stdcall))
  kWin64,          // __attribute__((ms_abi))
};

rs_bindings_from_cc::ir_proto::flat::CallingConv ToFlatProto(
    CallingConv calling_conv);

struct CcType {
  rs_bindings_from_cc::ir_proto::flat::CcType ToFlatProto() const;

  struct FuncPointer {
    // When true, this is a C++ function reference that maps to a Rust function
    // pointer. When false, this is a C++ function pointer that maps to a Rust
    // function pointer wrapped in an `Option`.
    bool non_null;
    CallingConv call_conv;
    // param_and_return_types assumes the last type is the return type.
    std::vector<CcType> param_and_return_types;
    std::vector<std::string> lifetime_inputs;
  };

  struct PointerType {
    PointerTypeKind kind;

    // `LifetimeId` is present if the C++ code contained an explicit
    // lifetime annotation or if C++ lifetime annotation elision was enabled.
    //
    // Today, this is rare: the Clang lifetime annotation pass isn't stable or
    // functional.
    std::optional<LifetimeId> lifetime;

    std::shared_ptr<CcType> pointee_type;
  };

  struct Primitive {
    std::string spelling;
  };

  static CcType PointerTo(CcType pointee_type,
                          std::optional<LifetimeId> lifetime, bool nullable);

  static CcType OwnedPointerTo(CcType pointee_type,
                               std::optional<LifetimeId> lifetime);

  static CcType LValueReferenceTo(CcType pointee_type,
                                  std::optional<LifetimeId> lifetime);

  static CcType RValueReferenceTo(CcType pointee_type,
                                  std::optional<LifetimeId> lifetime);

  bool IsVoid() const {
    const auto* primitive = std::get_if<CcType::Primitive>(&variant);
    return primitive != nullptr && primitive->spelling == "void";
  }

  using Variant =
      std::variant<Primitive, PointerType, FuncPointer, ItemId, FormattedError>;

  explicit CcType(Variant variant) : variant(std::move(variant)) {}

  Variant variant;
  bool is_const = false;
  std::string unknown_attr = "";
  // An ordered list of lifetime variable names applied to this type. It is
  // valid for the same name to appear multiple times.
  std::vector<std::string> explicit_lifetimes;
};

inline std::ostream& operator<<(std::ostream& o, const CcType& type) {
  return o << type.ToFlatProto().ShortDebugString();
}

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
  explicit Identifier(std::string identifier)
      : identifier_(std::move(identifier)) {
    CHECK(!identifier_.empty());
  }

  absl::string_view Ident() const { return identifier_; }
  void WriteToProto(ir_proto::Identifier& proto) const;

  template <typename H>
  friend H AbslHashValue(H h, const Identifier& i) {
    return H::combine(std::move(h), i.identifier_);
  }

  bool operator==(const Identifier& other) const {
    return identifier_ == other.identifier_;
  }

 private:
  std::string identifier_;
};

inline std::ostream& operator<<(std::ostream& o, const Identifier& id) {
  return o << std::setw(internal::kJsonIndent) << id.Ident();
}

// An integer value in the range [-2**63, 2**64). This is intended to be used
// to produce integer literals in Rust code while specifying the type
// out-of-band.
class IntegerConstant {
 public:
  static absl::StatusOr<IntegerConstant> FromAPValue(
      const llvm::APSInt& value) {
    if (value.getSignificantBits() > 64) {
      llvm::SmallString<128> value_str;
      value.toString(value_str);
      return absl::InvalidArgumentError(absl::StrCat(
          "value is too large to fit in 64 bits: ", value_str.c_str()));
    }
    return IntegerConstant(value);
  }

  IntegerConstant(const IntegerConstant& other) = default;
  IntegerConstant& operator=(const IntegerConstant& other) = default;
  void WriteToProto(ir_proto::IntegerConstant& proto) const;

 private:
  explicit IntegerConstant(const llvm::APSInt& value) {
    is_negative_ = value < 0;
    // TODO: double-check that the following is correct to adapt for
    // https://github.com/llvm/llvm-project/commit/0a89825a289d149195be390003424adad026067f
    // Before:
    // wrapped_value_ = static_cast<uint64_t>(value.getExtValue());
    wrapped_value_ = static_cast<uint64_t>(
        value.isSigned() ? value.getSExtValue() : value.getZExtValue());
  }

  // value < 0
  bool is_negative_;

  // value (mod 2**64)
  uint64_t wrapped_value_;
};

class Operator {
 public:
  explicit Operator(std::string name) : name_(std::move(name)) {
    CHECK(!name_.empty());
  }

  absl::string_view Name() const { return name_; }

  void WriteToProto(ir_proto::Operator& proto) const;

 private:
  std::string name_;
};

inline std::ostream& operator<<(std::ostream& stream, const Operator& op) {
  char first_char = op.Name()[0];
  const char* separator = ('a' <= first_char) && (first_char <= 'z') ? " " : "";
  return stream << std::setw(internal::kJsonIndent) << "`operator" << separator
                << op.Name() << "`";
}

enum SpecialName {
  kDestructor,
  kConstructor,
};

// The target type of the conversion operator is not stored in the struct but
// rather is resolved using the enclosing Func's return type.
struct ConversionOperator {};

rs_bindings_from_cc::ir_proto::flat::SpecialName ToFlatProto(
    SpecialName special_name);

std::ostream& operator<<(std::ostream& o, const SpecialName& special_name);

// A generalized notion of identifier, or an "Unqualified Identifier" in C++
// jargon: https://en.cppreference.com/w/cpp/language/identifiers
//
// Note that constructors are given a separate variant, so that we can treat
// them differently. After all, they are not invoked or defined like normal
// functions.
using UnqualifiedIdentifier =
    std::variant<Identifier, Operator, SpecialName, ConversionOperator>;
void WriteToProto(const UnqualifiedIdentifier& unqualified_identifier,
                  ir_proto::UnqualifiedIdentifier& proto);

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

// TODO(lukasza): Consider extracting a separate ConstructorMetadata struct to
// account for the fact that `is_const` and `is_virtual` never applies to
// constructors.

// Information about special member functions.
//
// Nontrivial definitions are divided into two: there are nontrivial definitions
// which are nontrivial only due to a member variable which defines the special
// member function, and those which are nontrivial because the operation was
// user defined for the object itself, or for any base class.
//
// This allows us to sidestep calling C++ implementations of special member
// functions in narrow cases: even for a nontrivial special member function, if
// it is kNontrivialMembers, we can directly implement it in Rust in terms of
// the member variables.
enum class SpecialMemberFunc : char {
  kTrivial,
  // Nontrivial, but only because of a member variable with a nontrivial special
  // member function.
  kNontrivialMembers,
  // Nontrivial because of a user-defined special member function in this or a
  // base class.
  kNontrivialUserDefined,
  // Deleted or non-public.
  kUnavailable,
};

rs_bindings_from_cc::ir_proto::flat::SpecialMemberFunc ToFlatProto(
    SpecialMemberFunc f);

inline std::ostream& operator<<(std::ostream& o, const SpecialMemberFunc& f) {
  switch (f) {
    case SpecialMemberFunc::kTrivial:
      return o << "Trivial";
    case SpecialMemberFunc::kNontrivialMembers:
      return o << "NontrivialMembers";
    case SpecialMemberFunc::kNontrivialUserDefined:
      return o << "NontrivialUserDefined";
    case SpecialMemberFunc::kUnavailable:
      return o << "Unavailable";
  }
}

struct TemplateArg {
  rs_bindings_from_cc::ir_proto::flat::TemplateArg ToFlatProto() const;

  using Variant = std::variant<CcType, bool, int64_t>;

  Variant variant;
};

// Present on records that are bridge types.
struct BridgeType {
  rs_bindings_from_cc::ir_proto::flat::BridgeType ToFlatProto() const;

  // From CRUBIT_BRIDGE.
  struct Bridge {
    std::string rust_name;
    std::string abi_rust;
    std::string abi_cpp;
    std::vector<CcType> template_args;
    std::optional<std::string> label_hint;
  };

  struct StdOptional {
    std::shared_ptr<CcType> inner_type;
  };

  struct StdPair {
    std::shared_ptr<CcType> first_type;
    std::shared_ptr<CcType> second_type;
  };

  struct ProtoMessageBridge {
    std::string rust_name;
  };

  struct StdString {};

  struct Callable {
    enum BackingType {
      kDynCallable,
      kAnyInvocable,
    } backing_type;
    enum FnTrait {
      kFn,
      kFnMut,
      kFnOnce,
    } fn_trait;
    std::shared_ptr<CcType> return_type;
    std::vector<CcType> param_types;
  };

  std::variant<Bridge, StdOptional, StdPair, StdString, ProtoMessageBridge,
               Callable>
      variant;
};

// A template specialization for a template record, containing information
// including the template name (like `ns::vector` for `ns::vector<int>`) and the
// template arguments (like [`int`, `float`] for `ns::map<int, float>`).
struct TemplateSpecialization {
  rs_bindings_from_cc::ir_proto::flat::TemplateSpecialization ToFlatProto()
      const;

  struct StdStringView {};
  struct StdWStringView {};
  struct StdVector {
    CcType element_type;
  };
  struct StdSharedPtr {
    CcType element_type;
  };
  struct StdUniquePtr {
    CcType element_type;
  };
  struct AbslSpan {
    CcType element_type;
  };
  struct AbslFlatHashMap {
    CcType key_type;
    CcType value_type;
  };
  struct AbslFlatHashSet {
    CcType element_type;
  };
  struct C9Co {
    CcType element_type;
  };
  struct NonSpecial {};

  using Kind = std::variant<StdStringView, StdWStringView, StdVector,
                            StdSharedPtr, StdUniquePtr, AbslSpan,
                            AbslFlatHashMap, AbslFlatHashSet, C9Co, NonSpecial>;

  BazelLabel defining_target;
  Kind kind = NonSpecial{};
};

enum class TraitImplPolarity : int8_t { kNegative, kNone, kPositive };

rs_bindings_from_cc::ir_proto::flat::TraitImplPolarity ToFlatProto(
    TraitImplPolarity trait_impl_polarity);

// The set of traits to derive on the Rust type.
struct TraitDerives {
  rs_bindings_from_cc::ir_proto::flat::TraitDerives ToFlatProto() const;

  TraitImplPolarity* absl_nullable Polarity(absl::string_view trait);

  // <internal link> start
  TraitImplPolarity clone = TraitImplPolarity::kNone;
  TraitImplPolarity copy = TraitImplPolarity::kNone;
  TraitImplPolarity debug = TraitImplPolarity::kNone;
  // <internal link> end
  bool send = false;
  bool sync = false;
  std::vector<std::string> custom;
};

struct OwnedPtrConfig {
  rs_bindings_from_cc::ir_proto::flat::OwnedPtrConfig ToFlatProto() const;

  std::string owned_ptr_type;
  std::string drop_impl;
};

// A complete intermediate representation of bindings for publicly accessible
// declarations of a single C++ library.
template <typename T>
const T* absl_nullable get_item_if(const ir_proto::Item& item) {
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
T* absl_nullable get_item_if(ir_proto::Item& item) {
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
std::vector<const T* absl_nonnull> get_items_if(
    const ir_proto::IRProto& ir_proto) {
  std::vector<const T* absl_nonnull> filtered_items;
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

  // Sort target keys to iterate `top_level_items()` deterministically, this
  // prevents non-deterministic hash bucket ordering from protobuf::Map.
  std::vector<std::string> target_keys;
  target_keys.reserve(ir_proto.top_level_items().size());
  for (const auto& [target, _] : ir_proto.top_level_items()) {
    target_keys.push_back(target);
  }
  std::sort(target_keys.begin(), target_keys.end());

  for (const auto& target : target_keys) {
    const auto& item_list = ir_proto.top_level_items().at(target);
    for (const auto& item : item_list.items()) {
      process_item(process_item, item);
    }
  }
  return filtered_items;
}

template <typename T>
std::vector<T* absl_nonnull> get_items_if(ir_proto::IRProto& ir_proto) {
  std::vector<T* absl_nonnull> filtered_items;
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
    target_keys.push_back(target);
  }
  std::sort(target_keys.begin(), target_keys.end());

  for (const auto& target : target_keys) {
    auto& item_list = (*ir_proto.mutable_top_level_items())[target];
    for (auto& item : *item_list.mutable_items()) {
      process_item(process_item, item);
    }
  }
  return filtered_items;
}

using IR = ir_proto::IRProto;

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_H_

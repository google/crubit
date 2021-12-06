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

#include <iomanip>
#include <optional>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "base/integral_types.h"
#include "base/logging.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/json/src/json.hpp"
#include "util/intops/strong_int.h"

namespace rs_bindings_from_cc {

namespace internal {
inline constexpr absl::string_view kRustPtrMut = "*mut";
inline constexpr absl::string_view kRustPtrConst = "*const";
inline constexpr absl::string_view kRustRefMut = "&mut";
inline constexpr absl::string_view kRustRefConst = "&";
inline constexpr absl::string_view kCcPtr = "*";
inline constexpr absl::string_view kCcLValueRef = "&";
inline constexpr int kJsonIndent = 2;
}  // namespace internal

// A name of a public header of the C++ library.
class HeaderName {
 public:
  explicit HeaderName(std::string name) : name_(std::move(name)) {}

  absl::string_view IncludePath() const { return name_; }

  nlohmann::json ToJson() const;

  template <typename H>
  friend H AbslHashValue(H h, const HeaderName& header_name) {
    return H::combine(std::move(h), header_name.name_);
  }

 private:
  // Header pathname in the format suitable for a google3-relative quote
  // include.
  std::string name_;
};

inline bool operator==(const HeaderName& lhs, const HeaderName& rhs) {
  return lhs.IncludePath() == rhs.IncludePath();
}

inline std::ostream& operator<<(std::ostream& o, const HeaderName& h) {
  return o << std::setw(internal::kJsonIndent) << h.ToJson();
}

// An int uniquely representing a Decl. Since our IR goes through the JSON
// serialization/deserialization at the moment, we need a way to restore graph
// edges that don't follow the JSON tree structure (for example between types
// and records). We use DeclIds for this.
DEFINE_STRONG_INT_TYPE(DeclId, uintptr_t);

// A numerical ID that uniquely identifies a lifetime.
DEFINE_STRONG_INT_TYPE(LifetimeId, int);

// A lifetime.
struct Lifetime {
  nlohmann::json ToJson() const;

  // Lifetime name. Unlike syn::Lifetime, this does not include the apostrophe.
  //
  // Note that this is not an identifier; the rules for what is a valid lifetime
  // name are slightly different than for identifiers, so we simply use a
  // std::string instead of an Identifier here.
  std::string name;

  LifetimeId id;
};

inline std::ostream& operator<<(std::ostream& o, const Lifetime& l) {
  return o << std::setw(internal::kJsonIndent) << l.ToJson();
}

// A C++ type involved in the bindings. It has the knowledge of how the type
// is spelled in C++.
struct CcType {
  nlohmann::json ToJson() const;
  // The name of the type. For example, int or void.
  std::string name;

  // Id of a decl that this type corresponds to. `nullopt` for primitive types.
  std::optional<DeclId> decl_id = std::nullopt;

  // The C++ const-qualification for the type.
  //
  // Note: there are two types for which cv-qualification does not do anything:
  // references and functions. if `T` is either a function type like `void()`,
  // or a reference type like `int&`, then `T`, `const T`, and `volatile T` are
  // all the same type in C++.
  bool is_const = false;

  // Type arguments for a generic type. Examples:
  //   int has no type arguments.
  //   int* has a single type argument, int.
  //   tuple<int, float> has two type arguments, int and float.
  std::vector<CcType> type_args = {};
};

// A Rust type involved in the bindings. It has the knowledge of how the type
// is spelled in Rust.
struct RsType {
  nlohmann::json ToJson() const;

  // The name of the type. For example, i32 or ().
  std::string name;

  // Id of a decl that this type corresponds to. `nullopt` for primitive types.
  std::optional<DeclId> decl_id = std::nullopt;

  // Lifetime arguments for a generic type. Examples:
  //   *mut i32 has no lifetime arguments
  //   &'a 32 has a single lifetime argument, 'a.
  //   SomeType<'a, 'b> has two lifetime arguments, 'a and 'b.
  // Lifetimes are identified by their unique ID. The corresponding Lifetime
  // will be found within the lifetime_params of a Func or Record that uses
  // this type.
  std::vector<LifetimeId> lifetime_args = {};

  // Type arguments for a generic type. Examples:
  //   i32 has no type arguments.
  //   *mut i32 has a single type argument, i32.
  //   (i32, f32) has two type arguments, i32 and f32.
  std::vector<RsType> type_args = {};
};

inline std::ostream& operator<<(std::ostream& o, const RsType& type) {
  return o << std::setw(internal::kJsonIndent) << type.ToJson();
}

// A type involved in the bindings. The rs_type and cc_type will be treated
// as interchangeable during bindings, and so should share the same layout.
//
// For example: a C++ pointer may be a usize in Rust, rather than a pointer, but
// should almost certainly not be a u8, because u8 and pointers are sized and
// aligned differently.
struct MappedType {
  static MappedType Void() { return Simple("()", "void"); }

  /// Returns the MappedType for a non-templated/generic, non-cv-qualified type.
  /// For example, Void() is Simple("()", "void").
  static MappedType Simple(std::string rs_name, std::string cc_name) {
    return MappedType{RsType{rs_name}, CcType{cc_name}};
  }

  static MappedType WithDeclIds(std::string rs_name, DeclId rs_decl_id,
                                std::string cc_name, DeclId cc_decl_id) {
    return MappedType{RsType{std::move(rs_name), rs_decl_id},
                      CcType{std::move(cc_name), cc_decl_id}};
  }

  static MappedType PointerTo(MappedType pointee_type,
                              std::optional<LifetimeId> lifetime,
                              bool nullable = true) {
    absl::string_view rs_name;
    // TODO(mboehme): Map nullable pointers with lifetimes to
    // Option<&> / Option<&mut>
    if (lifetime.has_value() && !nullable) {
      rs_name = pointee_type.cc_type.is_const ? internal::kRustRefConst
                                              : internal::kRustRefMut;
    } else {
      rs_name = pointee_type.cc_type.is_const ? internal::kRustPtrConst
                                              : internal::kRustPtrMut;
    }
    auto pointer_type =
        Simple(std::string(rs_name), std::string(internal::kCcPtr));
    if (lifetime.has_value()) {
      pointer_type.rs_type.lifetime_args.push_back(*std::move(lifetime));
    }
    pointer_type.rs_type.type_args.push_back(std::move(pointee_type.rs_type));
    pointer_type.cc_type.type_args.push_back(std::move(pointee_type.cc_type));
    return pointer_type;
  }

  static MappedType LValueReferenceTo(MappedType pointee_type,
                                      std::optional<LifetimeId> lifetime) {
    absl::string_view rs_name;
    if (lifetime.has_value()) {
      rs_name = pointee_type.cc_type.is_const ? internal::kRustRefConst
                                              : internal::kRustRefMut;
    } else {
      rs_name = pointee_type.cc_type.is_const ? internal::kRustPtrConst
                                              : internal::kRustPtrMut;
    }
    auto reference_type =
        Simple(std::string(rs_name), std::string(internal::kCcLValueRef));
    if (lifetime.has_value()) {
      reference_type.rs_type.lifetime_args.push_back(*std::move(lifetime));
    }
    reference_type.rs_type.type_args.push_back(std::move(pointee_type.rs_type));
    reference_type.cc_type.type_args.push_back(std::move(pointee_type.cc_type));
    return reference_type;
  }

  bool IsVoid() const { return rs_type.name == "()"; }

  nlohmann::json ToJson() const;

  RsType rs_type;
  CcType cc_type;
};

inline std::ostream& operator<<(std::ostream& o, const MappedType& type) {
  return o << std::setw(internal::kJsonIndent) << type.ToJson();
}

// An identifier involved in bindings.
//
// Examples:
//     Identifier of C++'s `int32_t Add(int32_t a, int32_t b)` will be
//     `Identifier("add")`.
//
// Invariants:
//     `identifier` cannot be empty.
class Identifier {
 public:
  explicit Identifier(std::string identifier)
      : identifier_(std::move(identifier)) {
    CHECK(!identifier_.empty()) << "Identifier name cannot be empty.";
  }

  absl::string_view Ident() const { return identifier_; }

  nlohmann::json ToJson() const;

 private:
  std::string identifier_;
};

inline std::ostream& operator<<(std::ostream& o, const Identifier& id) {
  return o << std::setw(internal::kJsonIndent) << id.Ident();
}

// A function parameter.
//
// Examples:
//    FuncParam of a C++ function `void Foo(int32_t a);` will be
//    `FuncParam{.type=Type{"i32", "int32_t"}, .identifier=Identifier("foo"))`.
struct FuncParam {
  nlohmann::json ToJson() const;

  MappedType type;
  Identifier identifier;
};

inline std::ostream& operator<<(std::ostream& o, const FuncParam& param) {
  return o << std::setw(internal::kJsonIndent) << param.ToJson();
}

enum SpecialName {
  kDestructor,
  kConstructor,
};

std::ostream& operator<<(std::ostream& o, const SpecialName& special_name);

// A generalized notion of identifier, or an "Unqualified Identifier" in C++
// jargon: https://en.cppreference.com/w/cpp/language/identifiers
//
// Note that constructors are given a separate variant, so that we can treat
// them differently. After all, they are not invoked or defined like normal
// functions.
using UnqualifiedIdentifier = std::variant<Identifier, SpecialName>;

struct MemberFuncMetadata {
  enum ReferenceQualification : char {
    kLValue,       // void Foo() &;
    kRValue,       // void Foo() &&;
    kUnqualified,  // void Foo();
  };

  struct InstanceMethodMetadata {
    ReferenceQualification reference = kUnqualified;
    bool is_const = false;
    bool is_virtual = false;
  };

  nlohmann::json ToJson() const;

  // The type that this is a member function for.
  DeclId record_id;

  // Qualifiers for the instance method.
  //
  // If null, this is a static method.
  std::optional<InstanceMethodMetadata> instance_method_metadata;
};

// A function involved in the bindings.
struct Func {
  nlohmann::json ToJson() const;

  UnqualifiedIdentifier name;
  Label owning_target;
  std::optional<std::string> doc_comment;
  std::string mangled_name;
  MappedType return_type;
  std::vector<FuncParam> params;
  std::vector<Lifetime> lifetime_params;
  bool is_inline;
  // If null, this is not a member function.
  std::optional<MemberFuncMetadata> member_func_metadata;
};

inline std::ostream& operator<<(std::ostream& o, const Func& f) {
  return o << std::setw(internal::kJsonIndent) << f.ToJson();
}

// Access specifier for a member or base class.
enum AccessSpecifier {
  kPublic,
  kProtected,
  kPrivate,
};

std::ostream& operator<<(std::ostream& o, const AccessSpecifier& access);

// A field (non-static member variable) of a record.
struct Field {
  nlohmann::json ToJson() const;

  Identifier identifier;
  std::optional<std::string> doc_comment;
  MappedType type;
  AccessSpecifier access;
  // Field offset in bits.
  uint64_t offset;
};

inline std::ostream& operator<<(std::ostream& o, const Field& f) {
  return o << std::setw(internal::kJsonIndent) << f.ToJson();
}

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
struct SpecialMemberFunc {
  enum class Definition : char {
    kTrivial,
    // Nontrivial, but only because of a member variable with a nontrivial
    // special member function.
    kNontrivialMembers,
    // Nontrivial because of a user-defined special member function in this or a
    // base class. (May *also* be nontrivial due to member variables.)
    kNontrivialSelf,
    kDeleted,
  };

  nlohmann::json ToJson() const;

  Definition definition = Definition::kTrivial;
  AccessSpecifier access = AccessSpecifier::kPublic;
};

std::ostream& operator<<(std::ostream& o,
                         const SpecialMemberFunc::Definition& definition);

inline std::ostream& operator<<(std::ostream& o, const SpecialMemberFunc& f) {
  return o << std::setw(internal::kJsonIndent) << f.ToJson();
}

// A record (struct, class, union).
struct Record {
  nlohmann::json ToJson() const;

  Identifier identifier;
  DeclId id;
  Label owning_target;
  std::optional<std::string> doc_comment;
  std::vector<Field> fields;
  std::vector<Lifetime> lifetime_params;
  // Size and alignment in bytes.
  int64_t size;
  int64_t alignment;

  // Special member functions.
  SpecialMemberFunc copy_constructor = {};
  SpecialMemberFunc move_constructor = {};
  SpecialMemberFunc destructor = {};

  // Whether this type is passed by value as if it were a trivial type (the same
  // as it would be if it were a struct in C).
  //
  // This can be either due to language rules (it *is* a trivial type), or due
  // to the usage of a Clang attribute that forces trivial for calls:
  //
  //  * https://eel.is/c++draft/class.temporary#3
  //  * https://clang.llvm.org/docs/AttributeReference.html#trivial-abi
  bool is_trivial_abi = false;
};

inline std::ostream& operator<<(std::ostream& o, const Record& r) {
  return o << std::setw(internal::kJsonIndent) << r.ToJson();
}

// Source code location
struct SourceLoc {
  nlohmann::json ToJson() const;

  std::string filename;
  uint64 line;
  uint64 column;
};

inline std::ostream& operator<<(std::ostream& o, const SourceLoc& r) {
  return o << std::setw(internal::kJsonIndent) << r.ToJson();
}

// A placeholder for an item that we can't generate bindings for (yet)
struct UnsupportedItem {
  nlohmann::json ToJson() const;

  // TODO(forster): We could show the original declaration in the generated
  // message (potentially also for successfully imported items).

  // Qualified name of the item for which we couldn't generate bindings
  std::string name;

  // Explanation of why we couldn't generate bindings
  // TODO(forster): We should support multiple reasons per unsupported item.
  std::string message;
  SourceLoc source_loc;
};

inline std::ostream& operator<<(std::ostream& o, const UnsupportedItem& r) {
  return o << std::setw(internal::kJsonIndent) << r.ToJson();
}

struct Comment {
  nlohmann::json ToJson() const;

  std::string text;
};

inline std::ostream& operator<<(std::ostream& o, const Comment& r) {
  return o << std::setw(internal::kJsonIndent) << r.ToJson();
}

// A complete intermediate representation of bindings for publicly accessible
// declarations of a single C++ library.
struct IR {
  nlohmann::json ToJson() const;

  template <typename T>
  std::vector<const T*> get_items_if() const {
    std::vector<const T*> filtered_items;
    for (const auto& item : items) {
      if (auto* filtered_item = std::get_if<T>(&item)) {
        filtered_items.push_back(filtered_item);
      }
    }
    return filtered_items;
  }

  // Collection of public headers that were used to construct the AST this `IR`
  // is generated from.
  std::vector<HeaderName> used_headers;
  Label current_target;
  std::vector<std::variant<Func, Record, UnsupportedItem, Comment>> items;
};

inline std::ostream& operator<<(std::ostream& o, const IR& ir) {
  return o << std::setw(internal::kJsonIndent) << ir.ToJson();
}
}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_H_

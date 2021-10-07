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

#include <string>
#include <string_view>
#include <utility>
#include <vector>

#include "base/logging.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/json/src/json.hpp"

namespace rs_bindings_from_cc {

namespace internal {
inline constexpr absl::string_view kRustPtrMut = "*mut";
inline constexpr absl::string_view kRustPtrConst = "*const";
inline constexpr absl::string_view kCcPtr = "*";
}  // namespace internal

// A name of a public header of the C++ library.
class HeaderName {
 public:
  explicit HeaderName(std::string name) : name_(std::move(name)) {}

  absl::string_view IncludePath() const { return name_; }

  nlohmann::json ToJson() const;

 private:
  // Header pathname in the format suitable for a google3-relative quote
  // include.
  std::string name_;
};

inline std::ostream& operator<<(std::ostream& o, const HeaderName& h) {
  return o << h.ToJson();
}

// A C++ type involved in the bindings. It has the knowledge of how the type
// is spelled in C++.
struct CcType {
  nlohmann::json ToJson() const;
  // The name of the type. For example, int or void.
  std::string name;

  // The C++ const-qualification for the type.
  //
  // Note: there are two types for which cv-qualification does not do anything:
  // references and functions. if `T` is either a function type like `void()`,
  // or a reference type like `int&`, then `T`, `const T`, and `volatile T` are
  // all the same type in C++.
  bool is_const = false;

  // Type parameters for a generic type. Examples:
  //   int has no type parameters.
  //   int* has a single type parameter, int.
  //   tuple<int, float> has two type parameters, int and float.
  std::vector<CcType> type_params = {};
};

// A Rust type involved in the bindings. It has the knowledge of how the type
// is spelled in Rust.
struct RsType {
  nlohmann::json ToJson() const;

  // The name of the type. For example, i32 or ().
  std::string name;

  // Type parameters for a generic type. Examples:
  //   i32 has no type parameters.
  //   *mut i32 has a single type parameter, i32.
  //   (i32, f32) has two type parameters, i32 and f32.
  std::vector<RsType> type_params = {};
};

inline std::ostream& operator<<(std::ostream& o, const RsType& type) {
  return o << type.ToJson();
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

  static MappedType PointerTo(MappedType pointee_type) {
    absl::string_view rs_name = pointee_type.cc_type.is_const
                                    ? internal::kRustPtrConst
                                    : internal::kRustPtrMut;
    auto pointer_type =
        Simple(std::string(rs_name), std::string(internal::kCcPtr));
    pointer_type.rs_type.type_params.push_back(std::move(pointee_type.rs_type));
    pointer_type.cc_type.type_params.push_back(std::move(pointee_type.cc_type));
    return pointer_type;
  }

  bool IsVoid() const { return rs_type.name == "()"; }

  nlohmann::json ToJson() const;

  RsType rs_type;
  CcType cc_type;
};

inline std::ostream& operator<<(std::ostream& o, const MappedType& type) {
  return o << type.ToJson();
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
  return o << id.Ident();
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
  return o << param.ToJson();
}

// A function involved in the bindings.
struct Func {
  nlohmann::json ToJson() const;

  Identifier identifier;
  std::string mangled_name;
  MappedType return_type;
  std::vector<FuncParam> params;
  bool is_inline;
};

inline std::ostream& operator<<(std::ostream& o, const Func& f) {
  return o << f.ToJson();
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
  return o << f.ToJson();
}

/// Information about special member functions.
struct SpecialMemberFunc {
  enum class Definition : char {
    kTrivial,
    kNontrivial,
    kDeleted,
  };

  nlohmann::json ToJson() const;

  Definition definition = Definition::kTrivial;
  AccessSpecifier access = AccessSpecifier::kPublic;
};

std::ostream& operator<<(std::ostream& o,
                         const SpecialMemberFunc::Definition& definition);

inline std::ostream& operator<<(std::ostream& o, const SpecialMemberFunc& f) {
  return o << f.ToJson();
}

// A record (struct, class, union).
struct Record {
  nlohmann::json ToJson() const;

  Identifier identifier;
  std::optional<std::string> doc_comment;
  std::vector<Field> fields;
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
  return o << r.ToJson();
}

// A complete intermediate representation of bindings for publicly accessible
// declarations of a single C++ library.
struct IR {
  nlohmann::json ToJson() const;

  // Collection of public headers that were used to construct the AST this `IR`
  // is generated from.
  std::vector<HeaderName> used_headers;
  std::vector<std::variant<Func, Record>> items;
};

inline std::ostream& operator<<(std::ostream& o, const IR& ir) {
  return o << ir.ToJson();
}

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_H_

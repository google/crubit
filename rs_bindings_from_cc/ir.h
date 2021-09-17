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

// A type involved in the bindings. It has the knowledge about how the type is
// spelled in Rust and in C++ code.
//
// Examples:
//     C++'s `int32_t` will be `Type{"i32", "int"}`.
//     C++'s `struct Foo` will be `Type{"Foo", "Foo"}`.
//     C++'s `int*` will be `Type{"*mut", "*", {Type{"i32", "int"}}}
struct Type {
  static Type Void() { return Type{"()", "void"}; }
  bool IsVoid() const { return rs_name == "()"; }

  static Type PointerTo(Type pointee_type) {
    absl::string_view rs_name =
        pointee_type.cc_const ? internal::kRustPtrConst : internal::kRustPtrMut;
    auto pointer_type = Type{.rs_name = std::string(rs_name),
                             .cc_name = std::string(internal::kCcPtr)};
    pointer_type.type_params.push_back(std::move(pointee_type));
    return pointer_type;
  }

  nlohmann::json ToJson() const;

  // The rust name of the type. For example, i32 or ().
  std::string rs_name;

  // The C++ name for the type. For example, int or void.
  std::string cc_name;

  // The C++ const-qualification for the type.
  //
  // Note: there are two types for which cv-qualification does not apply:
  // references and functions. So strictly speaking, much as the current type
  // structure allows for you to make a nonsensical `*<T, U>` or `*<>`, it also
  // allows for a nonsensical cv-qualified reference type of function type
  // (when we add those).
  bool cc_const = false;

  // Type parameters for a generic type. Examples:
  //   int* has a single type parameter, int.
  //   tuple<int, float> has two type parameters, int and float.
  std::vector<Type> type_params = {};
};

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

// A function parameter.
//
// Examples:
//    FuncParam of a C++ function `void Foo(int32_t a);` will be
//    `FuncParam{.type=Type{"i32", "int32_t"}, .identifier=Identifier("foo"))`.
struct FuncParam {
  nlohmann::json ToJson() const;

  Type type;
  Identifier identifier;
};

// A function involved in the bindings.
struct Func {
  nlohmann::json ToJson() const;

  Identifier identifier;
  std::string mangled_name;
  Type return_type;
  std::vector<FuncParam> params;
  bool is_inline;
};

// Access specifier for a member or base class.
enum AccessSpecifier {
  kPublic,
  kProtected,
  kPrivate,
};

// A field (non-static member variable) of a record.
struct Field {
  nlohmann::json ToJson() const;

  Identifier identifier;
  Type type;
  AccessSpecifier access;
};

// A record (struct, class, union).
struct Record {
  nlohmann::json ToJson() const;

  Identifier identifier;
  std::vector<Field> fields;
};

// A complete intermediate representation of bindings for publicly accessible
// declarations of a single C++ library.
struct IR {
  nlohmann::json ToJson() const;

  // Collection of public headers that were used to construct the AST this `IR`
  // is generated from.
  std::vector<HeaderName> used_headers;
  std::vector<Func> functions;
  std::vector<Record> records;
};

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_H_

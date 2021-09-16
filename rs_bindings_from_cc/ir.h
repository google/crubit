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
#include <utility>
#include <vector>

#include "base/logging.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/json/src/json.hpp"

namespace rs_bindings_from_cc {

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
//     Type of C++'s `int32_t` will be `Type{"i32", "int"}`.
//     Type of C++'s `struct Foo` will be `Type{"Foo", "Foo"}`.
struct Type {
  static Type Void() { return Type{std::string("()"), std::string("void")}; }
  bool IsVoid() const { return rs_name == "()"; }

  nlohmann::json ToJson() const;

  // The rust name of the type. For example, i32 or ().
  std::string rs_name;
  // The C++ name for the type. For example, int or void.
  std::string cc_name;
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

// A field (non-static member variable) of a record.
struct Field {
  nlohmann::json ToJson() const;

  Identifier identifier;
  Type type;
};

// A record (struct, class, union).
class Record {
 public:
  Record(Identifier identifier, std::vector<Field> fields)
      : identifier_(std::move(identifier)), fields_(std::move(fields)) {}

  const Identifier& Ident() const { return identifier_; }
  const std::vector<Field>& Fields() const { return fields_; }
  nlohmann::json ToJson() const;

 private:
  Identifier identifier_;
  std::vector<Field> fields_;
};

// A complete intermediate representation of bindings for publicly accessible
// declarations of a single C++ library.
class IR {
 public:
  IR() {}
  IR(std::vector<HeaderName> used_headers, std::vector<Func> functions,
     std::vector<Record> records)
      : used_headers_(std::move(used_headers)),
        functions_(std::move(functions)),
        records_(std::move(records)) {}

  nlohmann::json ToJson() const;

  const std::vector<HeaderName>& UsedHeaders() const { return used_headers_; }
  std::vector<HeaderName>& UsedHeaders() { return used_headers_; }

  const std::vector<Func>& Functions() const { return functions_; }
  std::vector<Func>& Functions() { return functions_; }

  const std::vector<Record>& Records() const { return records_; }
  std::vector<Record>& Records() { return records_; }

 private:
  // Collection of public headers that were used to construct the AST this `IR`
  // is generated from.
  std::vector<HeaderName> used_headers_;
  std::vector<Func> functions_;
  std::vector<Record> records_;
};

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_H_

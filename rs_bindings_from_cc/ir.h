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

#include <utility>
#include <vector>

#include "base/logging.h"
#include "third_party/absl/strings/cord.h"
#include "third_party/json/src/json.hpp"

namespace rs_bindings_from_cc {

// A name of a public header of the C++ library.
class HeaderName {
 public:
  explicit HeaderName(absl::Cord name) : name_(std::move(name)) {}

  const absl::Cord &IncludePath() const { return name_; }

  nlohmann::json ToJson() const;

 private:
  // Header pathname in the format suitable for a google3-relative quote
  // include.
  absl::Cord name_;
};

// A type involved in the bindings. It has the knowledge about how the type is
// spelled in Rust and in C++ code.
//
// Examples:
//     Type of C++'s `int32_t` will be `Type("i32", "int")`.
//     Type of C++'s `struct foo` will be `Type("Foo", "Foo")`.
//
// Conventions:
//     `rs_name` cannot be empty.
//     `cc_name` cannot be empty.
class Type {
 public:
  explicit Type(absl::Cord rs_name, absl::Cord cc_name)
      : rs_name_(std::move(rs_name)), cc_name_(std::move(cc_name)) {}

  static Type Void() { return Type(absl::Cord("()"), absl::Cord("void")); }
  bool IsVoid() const { return rs_name_ == "()"; }
  const absl::Cord &RsName() const { return rs_name_; }
  const absl::Cord &CcName() const { return cc_name_; }

  nlohmann::json ToJson() const;

 private:
  absl::Cord rs_name_;
  absl::Cord cc_name_;
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
  explicit Identifier(absl::Cord identifier)
      : identifier_(std::move(identifier)) {
    CHECK(!identifier_.empty()) << "Identifier name cannot be empty.";
  }

  const absl::Cord &Ident() const { return identifier_; }

  nlohmann::json ToJson() const;

 private:
  absl::Cord identifier_;
};

// A function parameter.
//
// Examples:
//    FuncParam of a C++ function `void Foo(int32_t a);` will be
//    `FuncParam(Type("i32"), Identifier("foo"))`.
class FuncParam {
 public:
  explicit FuncParam(Type type, Identifier identifier)
      : type_(std::move(type)), identifier_(std::move(identifier)) {}

  const Type &ParamType() const { return type_; }
  const Identifier &Ident() const { return identifier_; }

  nlohmann::json ToJson() const;

 private:
  Type type_;
  Identifier identifier_;
};

// A function involved in the bindings.
class Func {
 public:
  explicit Func(Identifier identifier, absl::Cord mangled_name,
                Type return_type, std::vector<FuncParam> params, bool is_inline)
      : identifier_(std::move(identifier)),
        mangled_name_(std::move(mangled_name)),
        return_type_(std::move(return_type)),
        params_(std::move(params)),
        is_inline_(is_inline) {}

  const absl::Cord &MangledName() const { return mangled_name_; }
  const Type &ReturnType() const { return return_type_; }
  const Identifier &Ident() const { return identifier_; }

  const std::vector<FuncParam> &Params() const { return params_; }
  bool IsInline() const { return is_inline_; }

  nlohmann::json ToJson() const;

 private:
  Identifier identifier_;
  absl::Cord mangled_name_;
  Type return_type_;
  std::vector<FuncParam> params_;
  bool is_inline_;
};

// A complete intermediate representation of bindings for publicly accessible
// declarations of a single C++ library.
class IR {
 public:
  explicit IR() {}
  explicit IR(std::vector<HeaderName> used_headers, std::vector<Func> functions)
      : used_headers_(std::move(used_headers)),
        functions_(std::move(functions)) {}

  nlohmann::json ToJson() const;

  const std::vector<HeaderName> &UsedHeaders() const { return used_headers_; }
  std::vector<HeaderName> &UsedHeaders() { return used_headers_; }

  const std::vector<Func> &Functions() const { return functions_; }
  std::vector<Func> &Functions() { return functions_; }

 private:
  // Collection of public headers that were used to construct the AST this `IR`
  // is generated from.
  std::vector<HeaderName> used_headers_;
  std::vector<Func> functions_;
};

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_H_

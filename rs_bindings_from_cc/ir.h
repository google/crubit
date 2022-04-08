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

#include <cstddef>
#include <iomanip>
#include <optional>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "third_party/absl/strings/string_view.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/util/check.h"
#include "rs_bindings_from_cc/util/strong_int.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/APSInt.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/Optional.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/FormatVariadic.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/JSON.h"

namespace rs_bindings_from_cc {

namespace internal {
// Pointers and LValue references.
inline constexpr absl::string_view kRustPtrMut = "*mut";
inline constexpr absl::string_view kRustPtrConst = "*const";
inline constexpr absl::string_view kRustRefMut = "&mut";
inline constexpr absl::string_view kRustRefConst = "&";

// RValue References
inline constexpr absl::string_view kRustRvalueRefMut = "#RvalueReference mut";
inline constexpr absl::string_view kRustRvalueRefConst =
    "#RvalueReference const";

// Function pointers.
inline constexpr absl::string_view kRustFuncPtr = "#funcPtr";

// C++ types therein.
inline constexpr absl::string_view kCcPtr = "*";
inline constexpr absl::string_view kCcLValueRef = "&";
inline constexpr absl::string_view kCcRValueRef = "&&";
inline constexpr absl::string_view kCcFuncValue = "#funcValue";

inline constexpr int kJsonIndent = 2;
}  // namespace internal

// A name of a public header of the C++ library.
class HeaderName {
 public:
  explicit HeaderName(std::string name) : name_(std::move(name)) {}

  absl::string_view IncludePath() const { return name_; }

  llvm::json::Value ToJson() const;

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
  return o << std::string(llvm::formatv("{0:2}", h.ToJson()));
}

// An int uniquely representing an Item. Since our IR goes through the JSON
// serialization/deserialization at the moment, we need a way to restore graph
// edges that don't follow the JSON tree structure (for example between types
// and records), as well as location of comments and items we don't yet support.
//  We use ItemIds for this.
CRUBIT_DEFINE_STRONG_INT_TYPE(ItemId, uintptr_t);

// A numerical ID that uniquely identifies a lifetime.
CRUBIT_DEFINE_STRONG_INT_TYPE(LifetimeId, int);

// A lifetime.
struct LifetimeName {
  llvm::json::Value ToJson() const;

  // Lifetime name. Unlike syn::Lifetime, this does not include the apostrophe.
  //
  // Note that this is not an identifier; the rules for what is a valid lifetime
  // name are slightly different than for identifiers, so we simply use a
  // std::string instead of an Identifier here.
  std::string name;

  LifetimeId id;
};

inline std::ostream& operator<<(std::ostream& o, const LifetimeName& l) {
  return o << std::string(llvm::formatv("{0:2}", l.ToJson()));
}

// A C++ type involved in the bindings. It has the knowledge of how the type
// is spelled in C++.
struct CcType {
  llvm::json::Value ToJson() const;

  // The name of the type. Examples:
  // - "int32_t", "std::ptrdiff_t", "long long", "bool"
  // - "void"
  // - "&" or "*" (pointee stored in `type_args[0]`)
  // - "#funcValue <callConv>" (compare with "#funcPtr <abi>" in RsType::name
  //   and note that Rust only supports function pointers; note that <callConv>
  //   in CcType doesn't map 1:1 to <abi> in RsType).
  // - An empty string when `decl_id` is non-empty.
  std::string name;

  // Id of a decl that this type corresponds to. `nullopt` when `name` is
  // non-empty. `llvm::Optional` is used because it integrates better with
  // `llvm::json` library than `std::optional`.
  llvm::Optional<ItemId> decl_id;

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
  llvm::json::Value ToJson() const;

  // The name of the type. Examples:
  // - "i32" or "bool"
  // - "()" (the unit type, equivalent of "void" in CcType)
  // - "&", "&mut", "*const", "*mut" (pointee stored in `type_args[0]`)
  // - "Option" (e.g. representing nullable, lifetime-annotated C++ pointer as
  //   `Option<&'a SomeOtherType>` - in this case `type_args[0]` is the generic
  //    argument representing the Rust reference type).
  // - "#funcPtr <abi>" (function pointer; return type is the last elem in
  //   `type_args`; param types are stored in other `type_args`; <abi> would be
  //   replaced with "cdecl", "stdcall" or other Abi - see
  //   https://doc.rust-lang.org/reference/types/function-pointer.html);
  // - An empty string when `decl_id` is non-empty.
  std::string name;

  // Id of a decl that this type corresponds to. `nullopt` when `name` is
  // non-empty. `llvm::Optional` is used because it integrates better with
  // `llvm::json` library than `std::optional`.
  llvm::Optional<ItemId> decl_id;

  // Lifetime arguments for a generic type. Examples:
  //   *mut i32 has no lifetime arguments
  //   &'a 32 has a single lifetime argument, 'a.
  //   SomeType<'a, 'b> has two lifetime arguments, 'a and 'b.
  // Lifetimes are identified by their unique ID. The corresponding LifetimeName
  // will be found within the lifetime_params of a Func or Record or TypeAlias
  // that uses this type underneath (as a parameter type, field type, or aliased
  // type).
  std::vector<LifetimeId> lifetime_args = {};

  // Type arguments for a generic type. Examples:
  //   i32 has no type arguments.
  //   *mut i32 has a single type argument, i32.
  //   (i32, f32) has two type arguments, i32 and f32.
  std::vector<RsType> type_args = {};
};

inline std::ostream& operator<<(std::ostream& o, const RsType& type) {
  return o << std::string(llvm::formatv("{0:2}", type.ToJson()));
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

  static MappedType WithDeclId(ItemId decl_id) {
    return MappedType{RsType{.decl_id = decl_id}, CcType{.decl_id = decl_id}};
  }

  static MappedType PointerTo(MappedType pointee_type,
                              std::optional<LifetimeId> lifetime,
                              bool nullable = true);

  static MappedType LValueReferenceTo(MappedType pointee_type,
                                      std::optional<LifetimeId> lifetime);

  // Creates an Rvalue Reference mapped type.
  //
  // Note: we don't currently support rvalue references that do not have a
  // lifetime. (Such a thing would require an "Rvalue Pointer" type -- probably
  // spelled `Move<*mut T>` in Rust, although that doesn't work today due to
  // the `P: DerefMut` bound in `Move<P>`.)
  static MappedType RValueReferenceTo(MappedType pointee_type,
                                      LifetimeId lifetime);

  static MappedType FuncPtr(absl::string_view cc_call_conv,
                            absl::string_view rs_abi,
                            std::optional<LifetimeId> lifetime,
                            MappedType return_type,
                            std::vector<MappedType> param_types);
  static MappedType FuncRef(absl::string_view cc_call_conv,
                            absl::string_view rs_abi,
                            std::optional<LifetimeId> lifetime,
                            MappedType return_type,
                            std::vector<MappedType> param_types);

  bool IsVoid() const { return rs_type.name == "()"; }

  llvm::json::Value ToJson() const;

  RsType rs_type;
  CcType cc_type;
};

inline std::ostream& operator<<(std::ostream& o, const MappedType& type) {
  return o << std::string(llvm::formatv("{0:2}", type.ToJson()));
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
    CRUBIT_CHECK(!identifier_.empty());
  }

  absl::string_view Ident() const { return identifier_; }

  llvm::json::Value ToJson() const;

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
  explicit IntegerConstant(const llvm::APSInt& value) {
    CRUBIT_CHECK(value.getSignificantBits() <= 64);
    is_negative_ = value < 0;
    wrapped_value_ = static_cast<uint64_t>(value.getExtValue());
  }
  IntegerConstant(const IntegerConstant& other) = default;
  IntegerConstant& operator=(const IntegerConstant& other) = default;

  llvm::json::Value ToJson() const;

 private:
  // value < 0
  bool is_negative_;

  // value (mod 2**64)
  uint64_t wrapped_value_;
};

class Operator {
 public:
  explicit Operator(std::string name) : name_(std::move(name)) {
    CRUBIT_CHECK(!name_.empty());
  }

  absl::string_view Name() const { return name_; }

  llvm::json::Value ToJson() const;

 private:
  std::string name_;
};

inline std::ostream& operator<<(std::ostream& stream, const Operator& op) {
  char first_char = op.Name()[0];
  const char* separator = ('a' <= first_char) && (first_char <= 'z') ? " " : "";
  return stream << std::setw(internal::kJsonIndent) << "`operator" << separator
                << op.Name() << "`";
}

// A function parameter.
//
// Examples:
//    FuncParam of a C++ function `void Foo(int32_t a);` will be
//    `FuncParam{.type=Type{"i32", "int32_t"}, .identifier=Identifier("foo"))`.
struct FuncParam {
  llvm::json::Value ToJson() const;

  MappedType type;
  Identifier identifier;
};

inline std::ostream& operator<<(std::ostream& o, const FuncParam& param) {
  return o << std::string(llvm::formatv("{0:2}", param.ToJson()));
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
using UnqualifiedIdentifier = std::variant<Identifier, Operator, SpecialName>;
llvm::json::Value toJSON(const UnqualifiedIdentifier& unqualified_identifier);

struct MemberFuncMetadata {
  enum ReferenceQualification : char {
    kLValue,       // void Foo() &;
    kRValue,       // void Foo() &&;
    kUnqualified,  // void Foo();
  };

  // TODO(lukasza): Consider extracting a separate ConstructorMetadata struct to
  // account for the fact that 1) `is_explicit_ctor` applies only to
  // constructors and 2) `is_const` and `is_virtual` never applies to
  // constructors.
  struct InstanceMethodMetadata {
    llvm::json::Value ToJson() const;

    ReferenceQualification reference = kUnqualified;
    bool is_const = false;
    bool is_virtual = false;

    // If the member function was a constructor with an `explicit` specifier.
    bool is_explicit_ctor = false;
  };

  llvm::json::Value ToJson() const;

  // The type that this is a member function for.
  ItemId record_id;

  // Qualifiers for the instance method.
  //
  // If null, this is a static method.
  //
  // `llvm::Optional` is used because it integrates better with `llvm::json`
  // library than `std::optional`.
  llvm::Optional<InstanceMethodMetadata> instance_method_metadata;
};

// Source code location
struct SourceLoc {
  llvm::json::Value ToJson() const;

  std::string filename;
  uint64 line;
  uint64 column;
};

inline std::ostream& operator<<(std::ostream& o, const SourceLoc& r) {
  return o << std::string(llvm::formatv("{0:2}", r.ToJson()));
}

// A function involved in the bindings.
struct Func {
  llvm::json::Value ToJson() const;

  UnqualifiedIdentifier name;
  BazelLabel owning_target;
  llvm::Optional<std::string> doc_comment;
  std::string mangled_name;
  MappedType return_type;
  std::vector<FuncParam> params;
  std::vector<LifetimeName> lifetime_params;
  bool is_inline;
  // If null, this is not a member function.
  llvm::Optional<MemberFuncMetadata> member_func_metadata;
  bool has_c_calling_convention = true;
  SourceLoc source_loc;
  ItemId id;
};

inline std::ostream& operator<<(std::ostream& o, const Func& f) {
  return o << std::string(llvm::formatv("{0:2}", f.ToJson()));
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
  llvm::json::Value ToJson() const;

  Identifier identifier;
  llvm::Optional<std::string> doc_comment;
  MappedType type;
  AccessSpecifier access;
  // Field offset in bits.
  uint64_t offset;
  // True if the field is [[no_unique_address]].
  bool is_no_unique_address;
};

inline std::ostream& operator<<(std::ostream& o, const Field& f) {
  return o << std::string(llvm::formatv("{0:2}", f.ToJson()));
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
    kNontrivialUserDefined,
    kDeleted,
  };

  llvm::json::Value ToJson() const;

  Definition definition = Definition::kTrivial;
  AccessSpecifier access = AccessSpecifier::kPublic;
};

std::ostream& operator<<(std::ostream& o,
                         const SpecialMemberFunc::Definition& definition);

inline std::ostream& operator<<(std::ostream& o, const SpecialMemberFunc& f) {
  return o << std::string(llvm::formatv("{0:2}", f.ToJson()));
}

// A base class subobject of a struct or class.
struct BaseClass {
  llvm::json::Value ToJson() const;
  ItemId base_record_id;

  // The offset the base class subobject is located at. This is always nonempty
  // for nonvirtual inheritance, and always empty if a virtual base class is
  // anywhere in the inheritance chain.
  //
  // `llvm::Optional` is used because it integrates better with `llvm::json`
  // library than `std::optional`.
  llvm::Optional<int64_t> offset;
};

// A record (struct, class, union).
struct Record {
  llvm::json::Value ToJson() const;

  // `rs_name` and `cc_name` are typically equal, but they may be different for
  // template instantiations (when `cc_name` is similar to `MyStruct<int>` and
  // `rs_name` is similar to "__CcTemplateInst8MyStructIiE").
  std::string rs_name;
  std::string cc_name;

  ItemId id;
  BazelLabel owning_target;
  llvm::Optional<std::string> doc_comment;
  std::vector<BaseClass> unambiguous_public_bases;
  std::vector<Field> fields;
  std::vector<LifetimeName> lifetime_params;
  // Size and alignment in bytes.
  int64_t size;
  int64_t alignment;

  // The size of the base class subobjects, or null if there are none.
  //
  // More information: docs/struct_layout
  //
  // `llvm::Optional` is used because it integrates better with `llvm::json`
  // library than `std::optional`.
  llvm::Optional<size_t> base_size;

  // True if the alignment may differ from what the fields would imply.
  //
  // For example, a base class or [[no_unique_address]] of alignment 8 should
  // cause the record to have alignment at least 8. Since the field cannot be
  // aligned due to layout issues, the parent struct must instead receive an
  // alignment adjustment as necessary, via .override_alignment=true.
  //
  // More information: docs/struct_layout
  bool override_alignment = false;

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

  // Whether this type can be inherited from.
  //
  // A type might not be inheritable if:
  // * The type was explicitly marked final
  // * A core function like the destructor was marked final
  // * The type is a C++ union, which does not support inheritance
  bool is_inheritable = false;

  // Whether this type is a C++ union (rather than a struct)
  bool is_union = false;
};

struct Enumerator {
  llvm::json::Value ToJson() const;

  Identifier identifier;
  IntegerConstant value;
};

struct Enum {
  llvm::json::Value ToJson() const;

  Identifier identifier;
  ItemId id;
  BazelLabel owning_target;
  MappedType underlying_type;
  std::vector<Enumerator> enumerators;
};

inline std::ostream& operator<<(std::ostream& o, const Record& r) {
  return o << std::string(llvm::formatv("{0:2}", r.ToJson()));
}

// A type alias (defined either using `typedef` or `using`).
struct TypeAlias {
  llvm::json::Value ToJson() const;

  Identifier identifier;
  ItemId id;
  BazelLabel owning_target;
  llvm::Optional<std::string> doc_comment;
  MappedType underlying_type;
};

inline std::ostream& operator<<(std::ostream& o, const TypeAlias& t) {
  return o << std::string(llvm::formatv("{0:2}", t.ToJson()));
}

// A placeholder for an item that we can't generate bindings for (yet)
struct UnsupportedItem {
  llvm::json::Value ToJson() const;

  // TODO(forster): We could show the original declaration in the generated
  // message (potentially also for successfully imported items).

  // Qualified name of the item for which we couldn't generate bindings
  std::string name;

  // Explanation of why we couldn't generate bindings
  // TODO(forster): We should support multiple reasons per unsupported item.
  std::string message;
  SourceLoc source_loc;
  ItemId id;
};

inline std::ostream& operator<<(std::ostream& o, const UnsupportedItem& r) {
  return o << std::string(llvm::formatv("{0:2}", r.ToJson()));
}

struct Comment {
  llvm::json::Value ToJson() const;

  std::string text;
  ItemId id;
};

inline std::ostream& operator<<(std::ostream& o, const Comment& r) {
  return o << std::string(llvm::formatv("{0:2}", r.ToJson()));
}

// A complete intermediate representation of bindings for publicly accessible
// declarations of a single C++ library.
struct IR {
  llvm::json::Value ToJson() const;

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
  BazelLabel current_target;

  using Item =
      std::variant<Func, Record, Enum, TypeAlias, UnsupportedItem, Comment>;
  std::vector<Item> items;
};

inline std::ostream& operator<<(std::ostream& o, const IR& ir) {
  return o << std::string(llvm::formatv("{0:2}", ir.ToJson()));
}
}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_H_

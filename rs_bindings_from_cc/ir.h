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
#include <ostream>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/check.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "common/strong_int.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/RawCommentList.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LLVM.h"
#include "llvm/ADT/APSInt.h"
#include "llvm/Support/FormatVariadic.h"
#include "llvm/Support/JSON.h"
#include "llvm/Support/raw_ostream.h"

namespace crubit {

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
  // non-empty.
  std::optional<ItemId> decl_id;

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
  // - "i32" or "bool" or "::core::ffi::c_int"
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
  // non-empty.
  std::optional<ItemId> decl_id;

  // Lifetime arguments for a generic type. Examples:
  //   *mut i32 has no lifetime arguments
  //   &'a 32 has a single lifetime argument, 'a.
  //   SomeType<'a, 'b> has two lifetime arguments, 'a and 'b.
  // Lifetimes are identified by their unique ID. The corresponding LifetimeName
  // will be found within the lifetime_params of a Func or Record or TypeAlias
  // that uses this type underneath (as a parameter type, field type, or aliased
  // type).
  std::vector<LifetimeId> lifetime_args = {};

  // A human-readable list of unknown attributes that should have applied to
  // this RsType, or None if all attributes were understood.
  std::optional<std::string> unknown_attr;

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

  static MappedType PointerTo(
      MappedType pointee_type, std::optional<LifetimeId> lifetime,
      std::optional<clang::RefQualifierKind> ref_qualifier_kind,
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
    CHECK(!identifier_.empty());
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
    CHECK_LE(value.getSignificantBits(), 64);
    is_negative_ = value < 0;
    // TODO: double-check that the following is correct to adapt for
    // https://github.com/llvm/llvm-project/commit/0a89825a289d149195be390003424adad026067f
    // Before:
    // wrapped_value_ = static_cast<uint64_t>(value.getExtValue());
    wrapped_value_ = static_cast<uint64_t>(
        value.isSigned() ? value.getSExtValue() : value.getZExtValue());
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
    CHECK(!name_.empty());
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
  std::optional<std::string> unknown_attr;
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
  // account for the fact that `is_const` and `is_virtual` never applies to
  // constructors.
  struct InstanceMethodMetadata {
    llvm::json::Value ToJson() const;

    ReferenceQualification reference = kUnqualified;
    bool is_const = false;
    bool is_virtual = false;
  };

  llvm::json::Value ToJson() const;

  // The type that this is a member function for.
  ItemId record_id;

  // Qualifiers for the instance method.
  //
  // If null, this is a static method.
  std::optional<InstanceMethodMetadata> instance_method_metadata;
};

// A function involved in the bindings.
struct Func {
  llvm::json::Value ToJson() const;

  UnqualifiedIdentifier name;
  BazelLabel owning_target;
  std::optional<std::string> doc_comment;
  std::string mangled_name;
  MappedType return_type;
  std::vector<FuncParam> params;
  std::vector<LifetimeName> lifetime_params;
  bool is_inline;
  // If null, this is not a member function.
  std::optional<MemberFuncMetadata> member_func_metadata;
  bool is_extern_c = false;
  bool is_noreturn = false;
  std::optional<std::string> nodiscard;
  std::optional<std::string> deprecated;
  std::optional<std::string> unknown_attr;
  bool has_c_calling_convention = true;
  bool is_member_or_descendant_of_class_template = false;
  std::string source_loc;
  ItemId id;
  std::optional<ItemId> enclosing_item_id;
  // If present, this function should only generate top-level bindings if its
  // arguments refer to this enclosing record according to the ADL rules.
  //
  // This could in principle be resolved while generating the IR, but the richer
  // Rust type modeling in src_code_gen makes it much easier to do on the
  // consuming end.
  std::optional<ItemId> adl_enclosing_record;
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

  // Name of the field.  This may be missing for "unnamed members" - see:
  // - https://en.cppreference.com/w/c/language/struct
  // - https://rust-lang.github.io/rfcs/2102-unnamed-fields.html
  std::optional<Identifier> identifier;

  std::optional<std::string> doc_comment;
  absl::StatusOr<MappedType> type;
  AccessSpecifier access;
  uint64_t offset;            // Field offset in bits.
  uint64_t size;              // Field size in bits.
  std::optional<std::string> unknown_attr;
  bool is_no_unique_address;  // True if the field is [[no_unique_address]].
  bool is_bitfield;           // True if the field is a bitfield.
  bool is_inheritable;        // True if the field is inheritable.
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
enum class SpecialMemberFunc : char {
  kTrivial,
  // Nontrivial, but only because of a member variable with a nontrivial
  // special member function.
  kNontrivialMembers,
  // Nontrivial because of a user-defined special member function in this or a
  // base class. (May *also* be nontrivial due to member variables.)
  kNontrivialUserDefined,
  // Deleted or non-public.
  kUnavailable,
};

llvm::json::Value toJSON(const SpecialMemberFunc& f);

inline std::ostream& operator<<(std::ostream& o, const SpecialMemberFunc& f) {
  return o << std::string(llvm::formatv("{0:2}", toJSON(f)));
}

// A base class subobject of a struct or class.
struct BaseClass {
  llvm::json::Value ToJson() const;
  ItemId base_record_id;

  // The offset the base class subobject is located at. This is always nonempty
  // for nonvirtual inheritance, and always empty if a virtual base class is
  // anywhere in the inheritance chain.
  std::optional<int64_t> offset;
};

enum RecordType {
  // `struct` in Rust and C++
  kStruct,

  // `union` in Rust and C++
  kUnion,

  // `class` in C++.  This is distinct from `kStruct` to avoid generating
  // `struct SomeClass` in `..._rs_api_impl.cc` and getting `-Wmismatched-tags`
  // warnings (see also b/238212337).
  kClass,
};

std::ostream& operator<<(std::ostream& o, const RecordType& record_type);

struct SizeAlign {
  llvm::json::Value ToJson() const;

  int64_t size;
  int64_t alignment;
};

// A record (struct, class, union).
struct Record {
  llvm::json::Value ToJson() const;

  // `rs_name` and `cc_name` are typically equal, but they may be different for
  // template instantiations (when `cc_name` is similar to `MyStruct<int>` and
  // `rs_name` is similar to "__CcTemplateInst8MyStructIiE").
  std::string rs_name;
  std::string cc_name;
  std::string mangled_cc_name;

  ItemId id;
  BazelLabel owning_target;
  std::optional<BazelLabel> defining_target;
  std::optional<std::string> unknown_attr;
  std::optional<std::string> doc_comment;
  std::string source_loc;
  std::vector<BaseClass> unambiguous_public_bases;
  std::vector<Field> fields;
  std::vector<LifetimeName> lifetime_params;
  SizeAlign size_align;

  // True if any base classes exist.
  bool is_derived_class;

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
  SpecialMemberFunc copy_constructor = SpecialMemberFunc::kUnavailable;
  SpecialMemberFunc move_constructor = SpecialMemberFunc::kUnavailable;
  SpecialMemberFunc destructor = SpecialMemberFunc::kUnavailable;

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

  // Whether this type is abstract.
  bool is_abstract = false;

  // Whether this `Record` corresponds to a C++ `union`, `struct`, or `class`.
  RecordType record_type;

  // Whether this type can be initialized using aggregate initialization syntax.
  //
  // For more context, see:
  // * https://en.cppreference.com/w/cpp/types/is_aggregate
  // * https://en.cppreference.com/w/cpp/language/aggregate_initialization
  bool is_aggregate = false;

  // It is an anoymous record with a typedef name.
  bool is_anon_record_with_typedef = false;

  // True when this record is created from an explicit class template
  // instantiation definition (which is also what cc_template!{} macro results
  // in).
  bool is_explicit_class_template_instantiation_definition = false;

  std::vector<ItemId> child_item_ids;
  std::optional<ItemId> enclosing_item_id;
};

// A forward-declared record (e.g. `struct Foo;`)
struct IncompleteRecord {
  llvm::json::Value ToJson() const;
  std::string cc_name;
  std::string rs_name;
  ItemId id;
  BazelLabel owning_target;
  std::optional<std::string> unknown_attr;
  RecordType record_type;
  std::optional<ItemId> enclosing_item_id;
};

struct Enumerator {
  llvm::json::Value ToJson() const;

  Identifier identifier;
  IntegerConstant value;
  std::optional<std::string> unknown_attr;
};

struct Enum {
  llvm::json::Value ToJson() const;

  Identifier identifier;
  ItemId id;
  BazelLabel owning_target;
  std::string source_loc;
  MappedType underlying_type;
  std::optional<std::vector<Enumerator>> enumerators;
  std::optional<std::string> unknown_attr;
  std::optional<ItemId> enclosing_item_id;
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
  std::optional<std::string> doc_comment;
  std::optional<std::string> unknown_attr;
  MappedType underlying_type;
  std::string source_loc;
  std::optional<ItemId> enclosing_item_id;
};

inline std::ostream& operator<<(std::ostream& o, const TypeAlias& t) {
  return o << std::string(llvm::formatv("{0:2}", t.ToJson()));
}

// An error that stores its format string as well as the formatted message.
struct FormattedError {
  llvm::json::Value ToJson() const;

  // The format string that produced the error message, if available. This is
  // used as an aggregation key for error reports.
  std::string fmt;
  // Explanation of why we couldn't generate bindings.
  std::string message;
};

// A placeholder for an item that we can't generate bindings for (yet)
struct UnsupportedItem {
  llvm::json::Value ToJson() const;

  // TODO(forster): We could show the original declaration in the generated
  // message (potentially also for successfully imported items).

  // Qualified name of the item for which we couldn't generate bindings
  std::string name;

  std::vector<FormattedError> errors;
  std::string source_loc;
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

struct Namespace {
  llvm::json::Value ToJson() const;

  Identifier name;
  ItemId id;
  ItemId canonical_namespace_id;
  std::optional<std::string> unknown_attr;
  BazelLabel owning_target;
  std::vector<ItemId> child_item_ids;
  std::optional<ItemId> enclosing_item_id;
  bool is_inline = false;
};

inline std::ostream& operator<<(std::ostream& o, const Namespace& n) {
  return o << std::string(llvm::formatv("{0:2}", n.ToJson()));
}

// Declare a module and use its contents.
//
// This is used to support extra Rust source files.
struct UseMod {
  llvm::json::Value ToJson() const;

  std::string path;
  Identifier mod_name;
  ItemId id;
};

inline std::ostream& operator<<(std::ostream& o, const UseMod& use_mod) {
  return o << std::string(llvm::formatv("{0:2}", use_mod.ToJson()));
}

// A type which has no bindings generated, and instead uses an already-existing
// rust type.
struct TypeMapOverride {
  llvm::json::Value ToJson() const;

  std::string rs_name;
  std::string cc_name;

  BazelLabel owning_target;
  // Size and alignment, if known.
  // (These will not be known for a forward declaration, for example.)
  std::optional<SizeAlign> size_align;

  bool is_same_abi;
  ItemId id;
};

inline std::ostream& operator<<(std::ostream& o,
                                const TypeMapOverride& type_mapped) {
  return o << std::string(llvm::formatv("{0:2}", type_mapped.ToJson()));
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

  // Collection of public headers that were used to construct the AST this `IR`.
  //
  // In production, these come from the `--public_headers` cmdline flag.
  // Note that the order of the headers might be significant and needs to be
  // preserved.
  std::vector<HeaderName> public_headers;

  BazelLabel current_target;

  using Item = std::variant<Func, Record, IncompleteRecord, Enum, TypeAlias,
                            UnsupportedItem, Comment, Namespace, UseMod,
                            TypeMapOverride>;
  std::vector<Item> items;
  std::vector<ItemId> top_level_item_ids;
  // Empty string signals that the bindings should be generated in the crate
  // root. This is the default state.
  //
  // Non-empty value represents the name of the first-level submodule inside of
  // which bindings should be generated. This is how we generate bindings for
  // class template instantiations - we put all generated bindings into a hidden
  // module of the user crate so everything can be compiled in one rustc
  // invocation (this enables us to access types introduced in the user crate
  // for template instantiations in the future).
  //
  // TODO(hlopko): Replace empty strings with std::optional<std::string>
  // throughout the codebase
  std::string crate_root_path;

  absl::flat_hash_map<BazelLabel, absl::flat_hash_set<std::string>>
      crubit_features;
};

inline std::string IrToJson(const IR& ir) {
  return std::string(llvm::formatv("{0:2}", ir.ToJson()));
}

inline std::ostream& operator<<(std::ostream& o, const IR& ir) {
  return o << IrToJson(ir);
}

// Utility function to convert items to string.
std::string ItemToString(const IR::Item& item);
inline std::string ItemToString(const std::optional<IR::Item>& item) {
  if (item.has_value()) return ItemToString(*item);
  return "null";
}

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_H_

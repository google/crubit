// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_LIFETIME_ANNOTATIONS_TYPE_LIFETIMES_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_TYPE_LIFETIMES_H_

#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/lifetime_substitutions.h"
#include "lifetime_annotations/lifetime_symbol_table.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeLoc.h"
#include "clang/AST/TypeOrdering.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/DenseMapInfo.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"

namespace clang {
namespace tidy {
namespace lifetimes {

// Returns a lifetime in some human-readable format.
using LifetimeFormatter = std::function<std::string(Lifetime)>;

// Variance of a reference-like type with respect to the type it refers to.
enum Variance {
  kCovariant,
  kInvariant,
};

// Extracts the lifetime parameters of the given type.
llvm::SmallVector<std::string> GetLifetimeParameters(clang::QualType type);

// The parameter, if non-null, is the name that the lifetime was annotated with.
// This is provided as an `Expr` that is expected to evaluate to a string
// literal, rather than an actual string, as callers of the `LifetimeFactory`
// may not have access to an `ASTContext`, which is needed to evaluate the
// expression.
using LifetimeFactory =
    std::function<llvm::Expected<Lifetime>(const clang::Expr*)>;

class ObjectLifetimes;
class FunctionLifetimes;

// Represents the lifetimes of a value; these may be 0 for non-reference-like
// types, 1 for pointers/references, and an arbitrary number for structs with
// template arguments/lifetime parameters.
class ValueLifetimes {
 public:
  // Creates an invalid ValueLifetimes, which should not be used. This is
  // provided only for usage with functions with output parameters.
  ValueLifetimes() : ValueLifetimes(clang::QualType()) {}

  ValueLifetimes(const ValueLifetimes& other);

  ValueLifetimes& operator=(const ValueLifetimes& other);

  ~ValueLifetimes();

  // Creates a ValueLifetimes for a *value* of a given type.
  // Only fails if lifetime_factory fails.
  // Lifetimes will be created in post-order in the tree of lifetimes.
  static llvm::Expected<ValueLifetimes> Create(
      clang::QualType type, LifetimeFactory lifetime_factory);

  // Returns a ValueLifetimes for a lifetime-less type.
  // `type` must not be a pointer-like type or a record type.
  static ValueLifetimes ForLifetimeLessType(clang::QualType type);

  // Returns a ValueLifetimes for a pointer-like type that points to an object
  // having lifetimes `object_lifetimes`.
  static ValueLifetimes ForPointerLikeType(
      clang::QualType type, const ObjectLifetimes& object_lifetimes);

  // Returns a ValueLifetimes for a record type. If the record type has template
  // parameters, pass the corresponding template argument lifetimes in
  // `template_argument_lifetimes`; otherwise, pass an empty vector.
  // Similarly, pass any lifetime parameters in `lifetime_parameters`.
  // The structure of `template_argument_lifetimes` is analogous to that of the
  // return value of `GetTemplateArgs()`.
  static ValueLifetimes ForRecord(
      clang::QualType type,
      std::vector<std::vector<std::optional<ValueLifetimes>>>
          template_argument_lifetimes,
      LifetimeSymbolTable lifetime_parameters);

  std::string DebugString(const LifetimeFormatter& formatter = [](Lifetime l) {
    return l.DebugString();
  }) const;

  // Returns the type of the value.
  clang::QualType Type() const { return type_; }

  // Returns the ObjectLifetimes of the pointed-to object. Type() must be a
  // pointer or reference type.
  const ObjectLifetimes& GetPointeeLifetimes() const;

  // Returns the lifetimes of the i-th template argument at the given nesting
  // `depth` within a chain of nested templates and with the given `index`.
  // For example, for a type `Outer<int*, double*>::Inner<long*>`, the
  // `double*` template argument has depth 0 and index 1.
  const std::optional<ValueLifetimes>& GetTemplateArgumentLifetimes(
      size_t depth, size_t index) const {
    assert(type_->isRecordType());
    return template_argument_lifetimes_.at(depth).at(index);
  }

  // Returns the number of template nesting levels.
  size_t GetNumTemplateNestingLevels() const {
    assert(type_->isRecordType());
    return template_argument_lifetimes_.size();
  }

  // Returns the number of template arguments at a given nesting `depth` (see
  // `GetTemplateArgumentLifetimes` for details).
  size_t GetNumTemplateArgumentsAtDepth(size_t depth) const {
    assert(type_->isRecordType());
    return template_argument_lifetimes_.at(depth).size();
  }

  // Returns the lifetime associated with the given named lifetime parameter.
  Lifetime GetLifetimeParameter(llvm::StringRef param) const {
    std::optional<Lifetime> ret =
        lifetime_parameters_by_name_.LookupName(param);
    assert(ret.has_value());
    return ret.value();
  }

  bool HasLifetimes() const {
    return pointee_lifetimes_ != nullptr ||
           !lifetime_parameters_by_name_.GetMapping().empty() ||
           !template_argument_lifetimes_.empty();
  }

  // Returns true if `predicate` returns true for any lifetime that appears in
  // the `ValueLifetimes`.
  bool HasAny(const std::function<bool(Lifetime)>& predicate) const;

  // Applies `subst` to all lifetimes in this ValueLifetimes.
  // See FunctionLifetimes::SubstituteLifetimes() for details.
  void SubstituteLifetimes(const LifetimeSubstitutions& subst);

  // Traverses all the lifetimes in the object, recursively. The
  // visit is done in post-order on the lifetime tree of this type.
  // The callback may mutate the lifetime in an arbitrary way; `variance` will
  // be set to signal the variance of the lifetime in the position it appears
  // in; this is decided with the `variance` parameter to Traverse, which
  // signals the variance of the current Value. Note that the visitor may be
  // called multiple times with the same lifetime (but in different positions).
  // TODO(veluca): verify that the handling of variance is correct in all cases.
  void Traverse(std::function<void(Lifetime&, Variance)> visitor,
                Variance variance = kCovariant);
  void Traverse(std::function<void(const Lifetime&, Variance)> visitor,
                Variance variance = kCovariant) const;

 private:
  explicit ValueLifetimes(clang::QualType type);

  // Note: only one of `pointee_lifetimes_`, `function_lifetimes_` or
  // `template_argument_lifetimes_` is non-empty.
  std::unique_ptr<ObjectLifetimes> pointee_lifetimes_;
  std::unique_ptr<FunctionLifetimes> function_lifetimes_;
  std::vector<std::vector<std::optional<ValueLifetimes>>>
      template_argument_lifetimes_;
  clang::QualType type_;

  // Tracks the mapping from the names of the lifetimes on the struct/class
  // definition to the associated `Lifetime`s. For example, in the following
  // code
  //
  // class string_view LIFETIME_PARAM(d) { ... };
  //
  // string_view $a drop_last(string_view $a in) {
  //   string_view result;
  //   ...
  //   return result;
  // }
  //
  // the value stored in `result`/`in` has 1 lifetime argument. This lifetime
  // has a local name "a" (it is not possible to retrieve this mapping from this
  // ValueLifetimes object). This lifetime substitutes lifetime "d" from
  // string_view (this mapping is tracked by lifetime_parameters_by_name_).
  LifetimeSymbolTable lifetime_parameters_by_name_;

  friend class llvm::DenseMapInfo<clang::tidy::lifetimes::ValueLifetimes>;
};

// Represents all the lifetimes of an object. The object itself always has
// a lifetime; in addition, there may be lifetimes associated with the value
// of the object.
class ObjectLifetimes {
 public:
  // Creates an invalid ObjectLifetimes, which should not be used. This is
  // provided only for usage with functions with output parameters.
  ObjectLifetimes() {}

  ObjectLifetimes(Lifetime lifetime, ValueLifetimes value_lifetimes)
      : lifetime_(lifetime), value_lifetimes_(value_lifetimes) {}

  // Creates lifetimes for an *object* of a given type.
  // Only fails if lifetime_factory fails.
  // Lifetimes will be created in post-order in the tree of lifetimes.
  static llvm::Expected<ObjectLifetimes> Create(
      clang::QualType type, LifetimeFactory lifetime_factory);

  // Returns the lifetime of the object itself.
  Lifetime GetLifetime() const { return lifetime_; }

  // Returns the lifetime of the contained value.
  const ValueLifetimes& GetValueLifetimes() const { return value_lifetimes_; }

  clang::QualType Type() const { return value_lifetimes_.Type(); }

  std::string DebugString(const LifetimeFormatter& formatter = [](Lifetime l) {
    return l.DebugString();
  }) const;

  // Returns the ObjectLifetimes for a base class or a field of the given type.
  ObjectLifetimes GetFieldOrBaseLifetimes(
      clang::QualType type,
      llvm::SmallVector<std::string> type_lifetime_args) const;

  // Returns true if `predicate` returns true for any lifetime that appears in
  // the `ObjectLifetimes`.
  bool HasAny(const std::function<bool(Lifetime)>& predicate) const;

  // Applies `subst` to all lifetimes in this ObjectLifetimes.
  // See FunctionLifetimes::SubstituteLifetimes() for details.
  void SubstituteLifetimes(const LifetimeSubstitutions& subst);

  // Traverses all the lifetimes in the object, recursively. The
  // visit is done in post-order on the lifetime tree of this type.
  // The callback may mutate the lifetime in an arbitrary way; `variance` will
  // be set to signal the variance of the lifetime in the position it appears
  // in; this is decided with the `variance` parameter to Traverse, which
  // signals the variance of the current Object. Note that the visitor may be
  // called multiple times with the same lifetime (but in different positions).
  // `indirection_type` defines the type of the pointer (or reference) to this
  // object; this is used to determine variance of its pointees.
  // TODO(veluca): verify that the handling of variance is correct in all cases.
  void Traverse(std::function<void(Lifetime&, Variance)> visitor,
                Variance variance = kCovariant,
                clang::QualType indirection_type = clang::QualType());
  void Traverse(std::function<void(const Lifetime&, Variance)> visitor,
                Variance variance = kCovariant,
                clang::QualType indirection_type = clang::QualType()) const;

 private:
  // Returns the ObjectLifetimes for an object of a given type, whose lifetimes
  // are scoped within or derived from the object that `this`
  // represents, i.e. it is a field or a base class of the object.
  // `value_lifetimes_.Type()` must be a record type (class, struct or union).
  // TODO(veluca): ideally, `type_lifetime_args` should not be needed, but
  // rather extracted from attributes on the `type`.
  ObjectLifetimes GetObjectLifetimesForTypeInContext(
      clang::QualType type, llvm::SmallVector<std::string> type_lifetime_args,
      llvm::StringRef object_lifetime_parameter) const;

  friend class llvm::DenseMapInfo<clang::tidy::lifetimes::ObjectLifetimes>;
  Lifetime lifetime_;
  ValueLifetimes value_lifetimes_;
};

// TODO(lukasza): Try deduplicating GetTemplateArgs(QualType) vs
// GetTemplateArgs(TypeLoc).
// Return type: The outer vector is indexed by the "depth" of the template
// argument within a chain of nested templates; the inner vector contains the
// template arguments at a given depth.
// For example, for a type `Outer<int*, double*>::Inner<long*>`, this returns
// (in pseudo-code) { { int*, double* }, { long* } };
const llvm::SmallVector<llvm::ArrayRef<clang::TemplateArgument>>
GetTemplateArgs(clang::QualType type);

// Returns any template arguments present on `type_loc`. If `type_loc` does not
// have template arguments, returns an empty vector.
// Return type: The outer vector is indexed by the "depth" of the template
// argument within a chain of nested templates; the inner vector contains the
// template arguments at a given depth.
// For example, for a type `Outer<int *, double *>::Inner<long *>`, this returns
// (in pseudo-code) { { int *, double * }, { long * } };
// This helper function is placed here to be able to share it with clang-tidy.
llvm::SmallVector<llvm::SmallVector<clang::TypeLoc>> GetTemplateArgs(
    clang::TypeLoc type_loc);

// Evaluate the given expression as a string literal. Returns an error if the
// expression is not a string literal.
// This is exposed here so that it can be used in other places that need to
// evaluate string literal arguments of `annotate` attributes.
llvm::Expected<llvm::StringRef> EvaluateAsStringLiteral(
    const clang::Expr* expr, const clang::ASTContext& ast_context);

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

namespace llvm {

template <>
struct DenseMapInfo<clang::tidy::lifetimes::ValueLifetimes> {
  static clang::tidy::lifetimes::ValueLifetimes getEmptyKey() {
    return clang::tidy::lifetimes::ValueLifetimes(
        DenseMapInfo<clang::QualType>().getEmptyKey());
  }

  static clang::tidy::lifetimes::ValueLifetimes getTombstoneKey() {
    return clang::tidy::lifetimes::ValueLifetimes(
        DenseMapInfo<clang::QualType>().getTombstoneKey());
  }

  static bool isEqual(const clang::tidy::lifetimes::ValueLifetimes& lhs,
                      const clang::tidy::lifetimes::ValueLifetimes& rhs);

  static unsigned getHashValue(
      const clang::tidy::lifetimes::ValueLifetimes& value_lifetimes);
};

template <>
struct DenseMapInfo<clang::tidy::lifetimes::ObjectLifetimes> {
  static clang::tidy::lifetimes::ObjectLifetimes getEmptyKey() {
    return clang::tidy::lifetimes::ObjectLifetimes(
        DenseMapInfo<clang::tidy::lifetimes::Lifetime>().getEmptyKey(),
        DenseMapInfo<clang::tidy::lifetimes::ValueLifetimes>().getEmptyKey());
  }

  static clang::tidy::lifetimes::ObjectLifetimes getTombstoneKey() {
    return clang::tidy::lifetimes::ObjectLifetimes(
        DenseMapInfo<clang::tidy::lifetimes::Lifetime>().getTombstoneKey(),
        DenseMapInfo<clang::tidy::lifetimes::ValueLifetimes>()
            .getTombstoneKey());
  }

  static bool isEqual(const clang::tidy::lifetimes::ObjectLifetimes& lhs,
                      const clang::tidy::lifetimes::ObjectLifetimes& rhs) {
    return DenseMapInfo<clang::tidy::lifetimes::Lifetime>::isEqual(
               lhs.lifetime_, rhs.lifetime_) &&
           DenseMapInfo<clang::tidy::lifetimes::ValueLifetimes>::isEqual(
               lhs.value_lifetimes_, rhs.value_lifetimes_);
  }

  static unsigned getHashValue(
      const clang::tidy::lifetimes::ObjectLifetimes& object_lifetimes) {
    unsigned hash =
        DenseMapInfo<clang::tidy::lifetimes::Lifetime>::getHashValue(
            object_lifetimes.lifetime_);
    return hash_combine(
        hash,
        DenseMapInfo<clang::tidy::lifetimes::ValueLifetimes>::getHashValue(
            object_lifetimes.value_lifetimes_));
  }
};

}  // namespace llvm

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_TYPE_LIFETIMES_H_

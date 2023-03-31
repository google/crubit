// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/type_lifetimes.h"

#include <algorithm>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "absl/strings/str_cat.h"
#include "absl/strings/str_format.h"
#include "absl/strings/str_join.h"
#include "lifetime_annotations/function_lifetimes.h"
#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/lifetime_symbol_table.h"
#include "lifetime_annotations/pointee_type.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/TemplateBase.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/SourceLocation.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/DenseMapInfo.h"
#include "llvm/ADT/Hashing.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/ErrorHandling.h"

namespace clang {
namespace tidy {
namespace lifetimes {

clang::QualType StripAttributes(clang::QualType type) {
  while (true) {
    if (auto macro_qualified_type = type->getAs<clang::MacroQualifiedType>()) {
      type = macro_qualified_type->getUnderlyingType();
    } else if (auto attr_type = type->getAs<clang::AttributedType>()) {
      type = attr_type->getModifiedType();
    } else {
      return type;
    }
  }
}

clang::TypeLoc StripAttributes(clang::TypeLoc type_loc,
                               llvm::SmallVector<const clang::Attr*>& attrs) {
  while (true) {
    if (auto macro_qualified_type_loc =
            type_loc.getAs<clang::MacroQualifiedTypeLoc>()) {
      type_loc = macro_qualified_type_loc.getInnerLoc();
    } else if (auto attr_type_loc =
                   type_loc.getAs<clang::AttributedTypeLoc>()) {
      attrs.push_back(attr_type_loc.getAttr());
      type_loc = attr_type_loc.getModifiedLoc();
    } else {
      // The last attribute in a chain of attributes will appear as the
      // outermost AttributedType, e.g. `int $a $b` will be represented as
      // follows (in pseudocode):
      //
      // AttributedType(annotate_type("lifetime", "b"),
      //   AttributedType(annotate_type("lifetime", "a"),
      //     BuiltinType("int")
      //   )
      // )
      //
      // We reverse the attributes so that we obtain the more intuitive ordering
      // "a, b".
      std::reverse(attrs.begin(), attrs.end());
      return type_loc;
    }
  }
}

llvm::Expected<llvm::SmallVector<const clang::Expr*>> GetAttributeLifetimes(
    llvm::ArrayRef<const clang::Attr*> attrs) {
  llvm::SmallVector<const clang::Expr*> result;

  bool saw_annotate_type = false;

  for (const clang::Attr* attr : attrs) {
    auto annotate_type_attr = clang::dyn_cast<clang::AnnotateTypeAttr>(attr);
    if (!annotate_type_attr ||
        annotate_type_attr->getAnnotation() != "lifetime")
      continue;

    if (saw_annotate_type) {
      return llvm::createStringError(
          llvm::inconvertibleErrorCode(),
          "Only one `[[annotate_type(\"lifetime\", ...)]]` attribute may be "
          "placed on a type");
    }
    saw_annotate_type = true;

    for (const clang::Expr* arg : annotate_type_attr->args()) {
      result.push_back(arg);
    }
  }

  return result;
}

llvm::SmallVector<std::string> GetLifetimeParameters(clang::QualType type) {
  // TODO(mboehme):
  // - Add support for type aliases with lifetime parameters
  // - Report errors as Clang diagnostics, not through
  //   llvm::report_fatal_error().

  auto record = type->getAs<clang::RecordType>();
  if (!record) {
    return {};
  }

  auto cxx_record = record->getAsCXXRecordDecl();
  if (!cxx_record) {
    return {};
  }

  const clang::AnnotateAttr* lifetime_params_attr = nullptr;
  for (auto annotate : cxx_record->specific_attrs<clang::AnnotateAttr>()) {
    if (annotate->getAnnotation() == "lifetime_params") {
      if (lifetime_params_attr) {
        llvm::report_fatal_error("repeated lifetime annotation");
      }
      lifetime_params_attr = annotate;
    }
  }

  llvm::SmallVector<std::string> ret;

  // TODO(mboehme): Require derived class to explicitly declare all lifetime
  // parameters inherited from the base class.
  if (cxx_record->hasDefinition()) {
    for (const clang::CXXBaseSpecifier& base : cxx_record->bases()) {
      if (lifetime_params_attr) {
        llvm::report_fatal_error(
            "derived classes may not add lifetime parameters");
      }
      if (!ret.empty()) {
        llvm::report_fatal_error(
            "only one base class may have lifetime parameters");
      }
      ret = GetLifetimeParameters(base.getType());
    }
  }

  if (!lifetime_params_attr) {
    return ret;
  }

  for (const auto& arg : lifetime_params_attr->args()) {
    llvm::StringRef lifetime;
    if (llvm::Error err =
            EvaluateAsStringLiteral(arg, cxx_record->getASTContext())
                .moveInto(lifetime)) {
      llvm::report_fatal_error(llvm::StringRef(toString(std::move(err))));
    }
    ret.push_back(lifetime.str());
  }

  return ret;
}

ValueLifetimes::ValueLifetimes(const ValueLifetimes& other) { *this = other; }

ValueLifetimes& ValueLifetimes::operator=(const ValueLifetimes& other) {
  type_ = other.type_;
  template_argument_lifetimes_ = other.template_argument_lifetimes_;
  lifetime_parameters_by_name_ = other.lifetime_parameters_by_name_;
  auto pointee_lifetimes =
      other.pointee_lifetimes_
          ? std::make_unique<ObjectLifetimes>(*other.pointee_lifetimes_)
          : nullptr;
  auto function_lifetimes =
      other.function_lifetimes_
          ? std::make_unique<FunctionLifetimes>(*other.function_lifetimes_)
          : nullptr;
  // Note: because ValueLifetimes is a recursive type (pointee_lifetimes_
  // contains a ValueLifetimes), the following line can destroy `other`.
  // (Thus the temporary local variables before we perform the assignment.)
  pointee_lifetimes_ = std::move(pointee_lifetimes);
  function_lifetimes_ = std::move(function_lifetimes);
  return *this;
}

// Defined here because FunctionLifetimes is an incomplete type in the header.
ValueLifetimes::~ValueLifetimes() = default;

namespace {

llvm::Error ForEachTemplateArgument(
    clang::QualType type, clang::TypeLoc type_loc,
    const std::function<llvm::Error(int, clang::QualType, clang::TypeLoc)>&
        callback) {
  llvm::SmallVector<llvm::ArrayRef<clang::TemplateArgument>> template_args =
      GetTemplateArgs(type);
  std::optional<llvm::SmallVector<llvm::SmallVector<clang::TypeLoc>>>
      maybe_template_arg_locs;
  if (type_loc) {
    maybe_template_arg_locs = GetTemplateArgs(type_loc);
  }
  llvm::SmallVector<llvm::SmallVector<clang::TypeLoc>> template_arg_locs;
  if (maybe_template_arg_locs) {
    template_arg_locs = *std::move(maybe_template_arg_locs);
  } else {
    // Fill all of `template_arg_locs` with empty `TypeLoc`s so we don't need to
    // make any case distinctions below.
    template_arg_locs.resize(template_args.size());
    for (size_t depth = 0; depth < template_args.size(); depth++) {
      template_arg_locs[depth].resize(template_args[depth].size());
    }
  }
  assert(template_arg_locs.size() == template_args.size());
  for (int depth = 0; depth < template_args.size(); depth++) {
    const auto& args_at_depth = template_args[depth];
    const auto& arg_locs_at_depth = template_arg_locs[depth];
    assert(args_at_depth.size() == arg_locs_at_depth.size());
    for (size_t i = 0; i < args_at_depth.size(); ++i) {
      const clang::TemplateArgument& arg = args_at_depth[i];
      if (arg.getKind() == clang::TemplateArgument::Type) {
        if (llvm::Error err =
                callback(depth, arg.getAsType(), arg_locs_at_depth[i])) {
          return err;
        }
      } else if (arg.getKind() == clang::TemplateArgument::Pack) {
        for (const clang::TemplateArgument& inner_arg : arg.getPackAsArray()) {
          if (inner_arg.getKind() == clang::TemplateArgument::Type) {
            // We pass `TypeLoc()` here because if the argument is a parameter
            // pack, the only thing spelled out in the source code is that
            // parameter pack -- there is no source location for the individual
            // arguments contained in the pack.
            if (llvm::Error err =
                    callback(depth, inner_arg.getAsType(), clang::TypeLoc())) {
              return err;
            }
          }
        }
      } else {
        if (llvm::Error err =
                callback(depth, clang::QualType(), clang::TypeLoc())) {
          return err;
        }
      }
    }
  }
  return llvm::Error::success();
}

QualType Undecay(clang::QualType type) {
  if (auto decayed = type->getAs<clang::DecayedType>()) {
    return decayed->getOriginalType();
  }
  return type;
}

bool SameType(clang::QualType type1, clang::QualType type2) {
  // If both types are an AutoType, ignore the actual type and assume they're
  // the same.
  // An `AutoType` that came from a TypeLoc will have type `auto` (i.e. as
  // written), whereas an `AutoType` that didn't come from a `TypeLoc` will be
  // the actual deduced type. We still want these to compare equal though.
  if (type1->getAs<clang::AutoType>() && type2->getAs<clang::AutoType>()) {
    return true;
  }
  return Undecay(type1) == Undecay(type2);
}

}  // namespace

ValueLifetimes ValueLifetimes::PointerTo(const clang::QualType pointer_type,
                                         const ObjectLifetimes& obj) {
  ValueLifetimes ret;
  ret.type_ = pointer_type;
  assert(pointer_type->getPointeeType().getCanonicalType() ==
         obj.Type().getCanonicalType());
  ret.pointee_lifetimes_ = std::make_unique<ObjectLifetimes>(obj);
  return ret;
}

llvm::Expected<ValueLifetimes> ValueLifetimes::Create(
    clang::QualType type, clang::TypeLoc type_loc,
    LifetimeFactory lifetime_factory) {
  assert(!type.isNull());
  if (type_loc) {
    assert(SameType(type_loc.getType(), type));
  }

  type = type.IgnoreParens();

  type = StripAttributes(type);
  llvm::SmallVector<const clang::Attr*> attrs;
  if (!type_loc.isNull()) {
    type_loc = StripAttributes(type_loc, attrs);
  }
  llvm::SmallVector<const clang::Expr*> lifetime_names;
  if (llvm::Error err = GetAttributeLifetimes(attrs).moveInto(lifetime_names)) {
    return std::move(err);
  }

  ValueLifetimes ret(type);

  if (const auto* fn = clang::dyn_cast<clang::FunctionProtoType>(type)) {
    // TODO(veluca): this will not correctly handle the distinction between
    // parameter and return lifetimes.
    FunctionLifetimeFactorySingleCallback factory(lifetime_factory);
    FunctionLifetimes fn_lftm;
    if (llvm::Error err =
            FunctionLifetimes::CreateForFunctionType(fn, type_loc, factory)
                .moveInto(fn_lftm)) {
      return std::move(err);
    }
    ret.function_lifetimes_ =
        std::make_unique<FunctionLifetimes>(std::move(fn_lftm));
    return ret;
  }

  llvm::SmallVector<std::string> lifetime_params = GetLifetimeParameters(type);
  if (!lifetime_params.empty() && !lifetime_names.empty() &&
      lifetime_names.size() != lifetime_params.size()) {
    return llvm::createStringError(
        llvm::inconvertibleErrorCode(),
        absl::StrCat("Type has ", lifetime_params.size(),
                     " lifetime parameters but ", lifetime_names.size(),
                     " lifetime arguments were given"));
  }
  for (size_t i = 0; i < lifetime_params.size(); ++i) {
    Lifetime l;
    const clang::Expr* lifetime_name = nullptr;
    if (i < lifetime_names.size()) {
      lifetime_name = lifetime_names[i];
    }
    if (llvm::Error err = lifetime_factory(lifetime_name).moveInto(l)) {
      return std::move(err);
    }
    ret.lifetime_parameters_by_name_.Add(lifetime_params[i], l);
  }

  // Add implicit lifetime parameters for type template parameters.
  if (llvm::Error err = ForEachTemplateArgument(
          type, type_loc,
          [&ret, &lifetime_factory](
              int depth, clang::QualType arg_type,
              clang::TypeLoc arg_type_loc) -> llvm::Error {
            std::optional<ValueLifetimes> maybe_template_arg_lifetime;
            if (!arg_type.isNull()) {
              maybe_template_arg_lifetime.emplace();
              if (llvm::Error err =
                      ValueLifetimes::Create(arg_type, arg_type_loc,
                                             lifetime_factory)
                          .moveInto(*maybe_template_arg_lifetime)) {
                return err;
              }
            }
            if (ret.template_argument_lifetimes_.size() <= depth) {
              ret.template_argument_lifetimes_.resize(depth + 1);
            }
            ret.template_argument_lifetimes_[depth].push_back(
                maybe_template_arg_lifetime);
            return llvm::Error::success();
          })) {
    return std::move(err);
  }

  clang::QualType pointee = PointeeType(type);
  if (pointee.isNull() || type->isFunctionPointerType() ||
      type->isFunctionReferenceType()) {
    if (lifetime_params.empty() && !lifetime_names.empty())
      return llvm::createStringError(
          llvm::inconvertibleErrorCode(),
          absl::StrCat("Type may not be annotated with lifetimes"));
  }
  if (pointee.isNull()) {
    return ret;
  }

  clang::TypeLoc pointee_type_loc;
  if (type_loc) {
    pointee_type_loc = PointeeTypeLoc(type_loc);
    // Note: We can't assert that `pointee_type_loc` is non-null here. If
    // `type_loc` is a `TypedefTypeLoc`, then there will be no `TypeLoc` for
    // the pointee type because the pointee type never got spelled out at the
    // location of the original `TypeLoc`.
  }

  ValueLifetimes value_lifetimes;
  if (llvm::Error err =
          ValueLifetimes::Create(pointee, pointee_type_loc, lifetime_factory)
              .moveInto(value_lifetimes)) {
    return std::move(err);
  }

  Lifetime object_lifetime;
  if (type->isFunctionPointerType() || type->isFunctionReferenceType()) {
    // Function pointers and function references may not be annotated with a
    // lifetime. Instead, it is always implied that they have static lifetime.
    object_lifetime = Lifetime::Static();
  } else {
    const clang::Expr* lifetime_name = nullptr;
    if (!lifetime_names.empty()) {
      if (lifetime_names.size() != 1) {
        return llvm::createStringError(
            llvm::inconvertibleErrorCode(),
            absl::StrCat("Expected a single lifetime but ",
                         lifetime_names.size(), " were given"));
      }
      lifetime_name = lifetime_names.front();
    }
    if (llvm::Error err =
            lifetime_factory(lifetime_name).moveInto(object_lifetime)) {
      return std::move(err);
    }
  }
  ret.pointee_lifetimes_ =
      std::make_unique<ObjectLifetimes>(object_lifetime, value_lifetimes);
  return ret;
}

ValueLifetimes ValueLifetimes::ForLifetimeLessType(clang::QualType type) {
  assert(PointeeType(type).isNull() && !type->isRecordType());
  return ValueLifetimes(type);
}

ValueLifetimes ValueLifetimes::ForPointerLikeType(
    clang::QualType type, const ObjectLifetimes& object_lifetimes) {
  assert(!PointeeType(type).isNull());
  ValueLifetimes result(type);
  result.pointee_lifetimes_ =
      std::make_unique<ObjectLifetimes>(object_lifetimes);
  return result;
}

ValueLifetimes ValueLifetimes::ForRecord(
    clang::QualType type,
    std::vector<std::vector<std::optional<ValueLifetimes>>>
        template_argument_lifetimes,
    LifetimeSymbolTable lifetime_parameters) {
  assert(type->isRecordType());
  assert(GetTemplateArgs(type).size() == template_argument_lifetimes.size());
  for (size_t depth = 0; depth < template_argument_lifetimes.size(); ++depth) {
    assert(GetTemplateArgs(type)[depth].size() ==
           template_argument_lifetimes[depth].size());
  }
  ValueLifetimes result(type);
  result.template_argument_lifetimes_ = std::move(template_argument_lifetimes);
  result.lifetime_parameters_by_name_ = lifetime_parameters;
  return result;
}

std::string ValueLifetimes::DebugString(
    const LifetimeFormatter& formatter) const {
  if (Type()->isFunctionPointerType() || Type()->isFunctionReferenceType()) {
    // Function pointers and function references have an implied static
    // lifetime. This is never annotated, so don't print it either.
    // Note: If at some point we decide we want to distinguish between regular
    // pointers (to objects) and function pointers in more places, we could
    // consider adding a `function_pointee_lifetimes_` in addition to
    // `pointee_lifetimes_`.
    assert(pointee_lifetimes_->GetLifetime() == Lifetime::Static());
    return pointee_lifetimes_->GetValueLifetimes().DebugString(formatter);
  }
  if (!PointeeType(Type()).isNull()) {
    assert(pointee_lifetimes_);
    return pointee_lifetimes_->DebugString(formatter);
  }
  if (clang::isa<clang::FunctionProtoType>(Type())) {
    assert(function_lifetimes_);
    std::string fn_lifetimes = function_lifetimes_->DebugString(formatter);
    if (fn_lifetimes.empty()) return "";
    return absl::StrCat("(", fn_lifetimes, ")");
  }

  std::vector<std::vector<std::string>> tmpl_lifetimes;
  for (auto& tmpl_arg_at_depth : template_argument_lifetimes_) {
    tmpl_lifetimes.emplace_back();
    for (const std::optional<ValueLifetimes>& tmpl_arg : tmpl_arg_at_depth) {
      if (tmpl_arg) {
        std::string inner = tmpl_arg->DebugString(formatter);
        if (!inner.empty()) {
          tmpl_lifetimes.back().push_back(std::move(inner));
        }
      }
    }
  }
  std::vector<std::string> lifetime_parameters;
  for (const auto& lftm_arg : GetLifetimeParameters(type_)) {
    std::optional<Lifetime> lifetime =
        lifetime_parameters_by_name_.LookupName(lftm_arg);
    assert(lifetime.has_value());
    lifetime_parameters.push_back(formatter(*lifetime));
  }
  bool empty_tmpl_lifetimes = true;
  for (const auto& tmpl_arg_at_depth : tmpl_lifetimes) {
    for (const auto& tmpl : tmpl_arg_at_depth) {
      empty_tmpl_lifetimes &= tmpl.empty();
    }
  }
  if (empty_tmpl_lifetimes && lifetime_parameters.empty()) {
    return "";
  } else if (empty_tmpl_lifetimes) {
    if (lifetime_parameters.size() == 1) {
      return lifetime_parameters[0];
    }
    return absl::StrCat("[", absl::StrJoin(lifetime_parameters, ", "), "]");
  } else if (lifetime_parameters.empty() && tmpl_lifetimes.size() == 1 &&
             tmpl_lifetimes[0].size() == 1) {
    return tmpl_lifetimes[0][0];
  }

  std::vector<std::string> tmpl_lifetimes_per_depth;
  tmpl_lifetimes_per_depth.reserve(tmpl_lifetimes.size());
  for (const auto& tmpl_depth : tmpl_lifetimes) {
    tmpl_lifetimes_per_depth.push_back(
        absl::StrCat("<", absl::StrJoin(tmpl_depth, ", "), ">"));
  }
  std::string lifetime_parameter_string;
  if (!lifetime_parameters.empty()) {
    lifetime_parameter_string =
        absl::StrCat(" [", absl::StrJoin(lifetime_parameters, ", "), "]");
  }

  return absl::StrCat(absl::StrJoin(tmpl_lifetimes_per_depth, "::"),
                      lifetime_parameter_string);
}

const ObjectLifetimes& ValueLifetimes::GetPointeeLifetimes() const {
  assert(!PointeeType(type_).isNull());
  return *pointee_lifetimes_;
}

const FunctionLifetimes& ValueLifetimes::GetFuncLifetimes() const {
  assert(clang::isa<clang::FunctionProtoType>(type_));
  return *function_lifetimes_;
}

bool ValueLifetimes::HasAny(
    const std::function<bool(Lifetime)>& predicate) const {
  for (const auto& tmpl_arg_at_depth : template_argument_lifetimes_) {
    for (const std::optional<ValueLifetimes>& tmpl_arg : tmpl_arg_at_depth) {
      if (tmpl_arg && tmpl_arg->HasAny(predicate)) {
        return true;
      }
    }
  }
  if (pointee_lifetimes_ && pointee_lifetimes_->HasAny(predicate)) {
    return true;
  }
  for (const auto& lftm_arg : GetLifetimeParameters(type_)) {
    std::optional<Lifetime> lifetime =
        lifetime_parameters_by_name_.LookupName(lftm_arg);
    assert(lifetime.has_value());
    if (predicate(lifetime.value())) {
      return true;
    }
  }
  if (function_lifetimes_ && function_lifetimes_->HasAny(predicate)) {
    return true;
  }
  return false;
}

void ValueLifetimes::SubstituteLifetimes(const LifetimeSubstitutions& subst) {
  for (auto& tmpl_arg_at_depth : template_argument_lifetimes_) {
    for (std::optional<ValueLifetimes>& tmpl_arg : tmpl_arg_at_depth) {
      if (tmpl_arg) {
        tmpl_arg->SubstituteLifetimes(subst);
      }
    }
  }
  if (pointee_lifetimes_) {
    pointee_lifetimes_->SubstituteLifetimes(subst);
  }
  for (const auto& lftm_arg : GetLifetimeParameters(type_)) {
    std::optional<Lifetime> lifetime =
        lifetime_parameters_by_name_.LookupName(lftm_arg);
    assert(lifetime.has_value());
    lifetime_parameters_by_name_.Rebind(lftm_arg, subst.Substitute(*lifetime));
  }
  if (function_lifetimes_) {
    function_lifetimes_->SubstituteLifetimes(subst);
  }
}

void ValueLifetimes::Traverse(std::function<void(Lifetime&, Variance)> visitor,
                              Variance variance) {
  for (auto& tmpl_arg_at_depth : template_argument_lifetimes_) {
    for (std::optional<ValueLifetimes>& tmpl_arg : tmpl_arg_at_depth) {
      if (tmpl_arg) {
        tmpl_arg->Traverse(visitor, kInvariant);
      }
    }
  }
  if (pointee_lifetimes_) {
    pointee_lifetimes_->Traverse(visitor, variance, Type());
  }
  for (const auto& lftm_arg : GetLifetimeParameters(type_)) {
    std::optional<Lifetime> lifetime =
        lifetime_parameters_by_name_.LookupName(lftm_arg);
    assert(lifetime.has_value());
    Lifetime new_lifetime = *lifetime;
    visitor(new_lifetime, variance);

    // Note that this check is not an optimization, but a guard against UB:
    // if one calls the const version of Traverse, with a callback that does not
    // mutate the lifetime, on a const instance of the Value/ObjectLifetimes,
    // then calling Rebind would be UB, even if Rebind would do nothing in
    // practice.
    if (new_lifetime != lifetime) {
      lifetime_parameters_by_name_.Rebind(lftm_arg, new_lifetime);
    }
  }
  if (function_lifetimes_) {
    function_lifetimes_->Traverse(visitor);
  }
}

void ValueLifetimes::Traverse(
    std::function<void(const Lifetime&, Variance)> visitor,
    Variance variance) const {
  const_cast<ValueLifetimes*>(this)->Traverse(
      [&visitor](Lifetime& l, Variance v) { visitor(l, v); }, variance);
}

ValueLifetimes::ValueLifetimes(clang::QualType type) : type_(type) {}

const llvm::SmallVector<llvm::ArrayRef<clang::TemplateArgument>>
GetTemplateArgs(clang::QualType type) {
  llvm::SmallVector<llvm::ArrayRef<clang::TemplateArgument>> result;

  // Desugar any typedefs on `type`. We need to do this so that the "depth" of
  // the template arguments is compatible with
  // TemplateTypeParmType::getDepth(), which likewise assumes that typedefs are
  // desugared; see the call to getCanonicalTypeInternal() in
  // TemplateTypeParmType::getCanTTPTInfo(). Unlike that function, we can't
  // simply canonicalize the type, as that would also remove type annotations,
  // and we need those.
  while (const auto* typedef_type = type->getAs<clang::TypedefType>()) {
    type = typedef_type->desugar();
  }

  if (auto elaborated = type->getAs<clang::ElaboratedType>()) {
    if (clang::NestedNameSpecifier* qualifier = elaborated->getQualifier()) {
      if (const clang::Type* qualifier_type = qualifier->getAsType()) {
        result = GetTemplateArgs(clang::QualType(qualifier_type, 0));
      }
    }
  }

  if (auto specialization = type->getAs<clang::TemplateSpecializationType>()) {
    result.push_back(specialization->template_arguments());
  } else if (auto record = type->getAs<clang::RecordType>()) {
    if (auto specialization_decl =
            clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(
                record->getDecl())) {
      result.push_back(specialization_decl->getTemplateArgs().asArray());
    }
  }

  return result;
}

std::optional<llvm::SmallVector<llvm::SmallVector<clang::TypeLoc>>>
GetTemplateArgs(clang::TypeLoc type_loc) {
  assert(type_loc);

  type_loc = type_loc.getUnqualifiedLoc();

  // We can't get template arguments from a `SubstTemplateTypeParmTypeLoc`
  // because there is no `TypeLoc` for the type that replaces the template
  // parameter.
  if (type_loc.getAs<clang::SubstTemplateTypeParmTypeLoc>()) {
    assert(!type_loc.getNextTypeLoc());
    return std::nullopt;
  }

  // Similarly, there is no `TypeLoc` for the type that a `TypedefType` resolves
  // to. Note that we intentionally use Type::getAs<TypedefType>() instead of
  // TypeLoc::getAs<TypedefTypeLoc>() here because the former will strip off any
  // potential other layers of sugar to get to the `TypedefType`, and we want to
  // be consistent with `GetTemplateArgs<QualType>`, which obviously uses
  // `Type::getAs<TypedefType>()`, too.
  if (type_loc.getType()->getAs<clang::TypedefType>() != nullptr) {
    return std::nullopt;
  }

  llvm::SmallVector<llvm::SmallVector<clang::TypeLoc>> args;

  if (auto elaborated_type_loc = type_loc.getAs<clang::ElaboratedTypeLoc>()) {
    if (clang::NestedNameSpecifierLoc qualifier =
            elaborated_type_loc.getQualifierLoc()) {
      if (qualifier.getTypeLoc()) {
        if (auto qualifier_args = GetTemplateArgs(qualifier.getTypeLoc())) {
          args = *qualifier_args;
        } else {
          return std::nullopt;
        }
      }
    }
    if (auto elaborated_args =
            GetTemplateArgs(elaborated_type_loc.getNamedTypeLoc())) {
      args.append(*elaborated_args);
    } else {
      return std::nullopt;
    }
  } else if (auto template_specialization_type_loc =
                 type_loc.getAs<clang::TemplateSpecializationTypeLoc>()) {
    args.push_back({});
    for (unsigned i = 0; i < template_specialization_type_loc.getNumArgs();
         ++i) {
      if (auto* source_info = template_specialization_type_loc.getArgLoc(i)
                                  .getTypeSourceInfo()) {
        args.back().push_back(source_info->getTypeLoc());
      } else {
        args.back().push_back(clang::TypeLoc());
      }
    }
  } else if (auto record_type_loc = type_loc.getAs<clang::RecordTypeLoc>()) {
    if (auto specialization_decl =
            clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(
                record_type_loc.getDecl())) {
      if (specialization_decl->getTypeAsWritten()) {
        return GetTemplateArgs(
            specialization_decl->getTypeAsWritten()->getTypeLoc());
      }
      return std::nullopt;
    }
  } else if (auto dependent_template_specialization_type_loc =
                 type_loc
                     .getAs<clang::DependentTemplateSpecializationTypeLoc>()) {
    args.push_back({});
    for (unsigned i = 0;
         i < dependent_template_specialization_type_loc.getNumArgs(); ++i) {
      args.back().push_back(
          dependent_template_specialization_type_loc.getArgLoc(i)
              .getTypeSourceInfo()
              ->getTypeLoc());
    }
    // TODO(mboehme): Where does this occur exactly? Do we need to be handling
    // it?
    // AFAICT, this happens if we're looking at a dependent template name
    // (https://en.cppreference.com/w/cpp/language/dependent_name), which
    // probably means that this can only happen in template definitions (as
    // opposed to template instantiations), and we aren't analyzing those for
    // now. At the least, I haven't been able to trigger this case from a test.
    // Triggering a fatal error here so that we notice this case if it does come
    // up. We only trigger the fatal error _after_ we've done the processing
    // above so that we don't get a warning about dead code.
    llvm::report_fatal_error(
        "Unexpectedly got a DependentSpecializationTypeLoc");
  }

  return args;
}

ObjectLifetimes ObjectLifetimes::GetObjectLifetimesForTypeInContext(
    clang::QualType type, llvm::SmallVector<std::string> type_lifetime_args,
    llvm::StringRef object_lifetime_parameter) const {
  assert(value_lifetimes_.Type()->isRecordType());

  // The object of the `type` (i.e. field or a base class) basically has the
  // same lifetime as the struct.
  Lifetime ret_lifetime = lifetime_;
  // ... unless the field has lifetime parameters.
  if (!object_lifetime_parameter.empty()) {
    ret_lifetime =
        value_lifetimes_.GetLifetimeParameter(object_lifetime_parameter);
  }

  // `type` is one of a template argument, a struct, a pointer, or a type
  // with no lifetimes (other than its own).

  // First case: template argument. We just attach the
  // template argument's lifetimes to the leaf ObjectLifetimes.
  if (auto targ = type->getAs<clang::SubstTemplateTypeParmType>()) {
    const clang::TemplateTypeParmDecl* type_parm = targ->getReplacedParameter();
    const std::optional<ValueLifetimes>& arg_lifetimes =
        value_lifetimes_.GetTemplateArgumentLifetimes(type_parm->getDepth(),
                                                      type_parm->getIndex());
    if (!arg_lifetimes) {
      assert(false);
      return llvm::DenseMapInfo<ObjectLifetimes>::getEmptyKey();
    }
    return ObjectLifetimes(ret_lifetime, *arg_lifetimes);
  } else if (type->isStructureOrClassType()) {
    // Second case: struct.
    // Resolve lifetime parameters for the struct, if it has any.
    LifetimeSymbolTable lifetime_params;
    llvm::SmallVector<std::string> params = GetLifetimeParameters(type);
    for (size_t i = params.size(); i-- > 0;) {
      assert(!type_lifetime_args.empty());
      auto lftm_arg = type_lifetime_args.back();
      type_lifetime_args.pop_back();
      lifetime_params.Add(params[i],
                          value_lifetimes_.GetLifetimeParameter(lftm_arg));
    }

    // We need to construct potentally reshuffled
    // template arguments, if the struct is a template.

    // TODO(veluca): mixing lifetimes and template parameters is not supported
    // yet.
    std::vector<std::vector<std::optional<ValueLifetimes>>>
        template_argument_lifetimes;
    for (const auto& args_at_depth : GetTemplateArgs(type)) {
      size_t parameter_pack_expansion_index = 0;
      template_argument_lifetimes.push_back({});
      for (const clang::TemplateArgument& arg : args_at_depth) {
        if (arg.getKind() == clang::TemplateArgument::Type) {
          if (auto templ_arg =
                  clang::dyn_cast<clang::SubstTemplateTypeParmType>(
                      arg.getAsType())) {
            const clang::TemplateTypeParmDecl* type_parm =
                templ_arg->getReplacedParameter();
            // Template parameter packs get the index of the *pack*, not the
            // index of the type inside the pack itself. As they must appear
            // last, we can just increase the counter at every occurrence and
            // wraparound when we run out of template arguments.
            size_t index = type_parm->getIndex();
            if (type_parm->isParameterPack()) {
              index += parameter_pack_expansion_index++;
              if (index + 1 >= value_lifetimes_.GetNumTemplateArgumentsAtDepth(
                                   type_parm->getDepth())) {
                parameter_pack_expansion_index = 0;
              }
            }
            template_argument_lifetimes.back().push_back(
                value_lifetimes_.GetTemplateArgumentLifetimes(
                    type_parm->getDepth(), index));
          } else {
            // Create a new ValueLifetimes of the type of the template
            // parameter, with lifetime `lifetime_`.
            // TODO(veluca): we need to propagate lifetime parameters here.
            template_argument_lifetimes.back().push_back(
                ValueLifetimes::Create(
                    arg.getAsType(),
                    [this](const clang::Expr*) { return this->lifetime_; })
                    .get());
          }
        } else {
          template_argument_lifetimes.back().push_back(std::nullopt);
        }
      }
    }

    return ObjectLifetimes(
        ret_lifetime,
        ValueLifetimes::ForRecord(type, std::move(template_argument_lifetimes),
                                  std::move(lifetime_params)));
  } else if (clang::QualType pointee_type = PointeeType(type);
             !pointee_type.isNull()) {
    if (type_lifetime_args.empty()) {
      llvm::report_fatal_error(
          llvm::Twine("didn't find type lifetimes for object of type " +
                      type.getAsString()));
    }
    std::string pointee_object_lifetime_parameter =
        std::move(type_lifetime_args.back());
    type_lifetime_args.pop_back();
    // Third case: pointer.
    return ObjectLifetimes(
        ret_lifetime, ValueLifetimes::ForPointerLikeType(
                          type, GetObjectLifetimesForTypeInContext(
                                    pointee_type, std::move(type_lifetime_args),
                                    pointee_object_lifetime_parameter)));
  }

  return ObjectLifetimes(ret_lifetime,
                         ValueLifetimes::ForLifetimeLessType(type));
}

std::string ObjectLifetimes::DebugString(
    const LifetimeFormatter& formatter) const {
  std::string inner_lifetimes = value_lifetimes_.DebugString(formatter);
  std::string lifetime = formatter(lifetime_);
  if (inner_lifetimes.empty()) {
    return lifetime;
  }
  return absl::StrCat(std::move(inner_lifetimes), ", ", std::move(lifetime));
}

ObjectLifetimes ObjectLifetimes::GetFieldOrBaseLifetimes(
    clang::QualType type,
    llvm::SmallVector<std::string> type_lifetime_args) const {
  return GetObjectLifetimesForTypeInContext(type, std::move(type_lifetime_args),
                                            "");
}

bool ObjectLifetimes::HasAny(
    const std::function<bool(Lifetime)>& predicate) const {
  return predicate(lifetime_) || value_lifetimes_.HasAny(predicate);
}

void ObjectLifetimes::SubstituteLifetimes(const LifetimeSubstitutions& subst) {
  lifetime_ = subst.Substitute(lifetime_);
  value_lifetimes_.SubstituteLifetimes(subst);
}

void ObjectLifetimes::Traverse(std::function<void(Lifetime&, Variance)> visitor,
                               Variance variance,
                               clang::QualType indirection_type) {
  assert(indirection_type.isNull() ||
         StripAttributes(indirection_type->getPointeeType().IgnoreParens()) ==
             Type());
  value_lifetimes_.Traverse(
      visitor, indirection_type.isNull() || indirection_type.isConstQualified()
                   ? kCovariant
                   : kInvariant);
  visitor(lifetime_, variance);
}

void ObjectLifetimes::Traverse(
    std::function<void(const Lifetime&, Variance)> visitor, Variance variance,
    clang::QualType indirection_type) const {
  const_cast<ObjectLifetimes*>(this)->Traverse(
      [&visitor](Lifetime& l, Variance v) { visitor(l, v); }, variance,
      indirection_type);
}

llvm::Expected<llvm::StringRef> EvaluateAsStringLiteral(
    const clang::Expr* expr, const clang::ASTContext& ast_context) {
  auto error = []() {
    return llvm::createStringError(
        llvm::inconvertibleErrorCode(),
        "cannot evaluate argument as a string literal");
  };

  clang::Expr::EvalResult eval_result;
  if (!expr->EvaluateAsConstantExpr(eval_result, ast_context) ||
      !eval_result.Val.isLValue()) {
    return error();
  }

  const auto* eval_result_expr =
      eval_result.Val.getLValueBase().dyn_cast<const clang::Expr*>();
  if (!eval_result_expr) {
    return error();
  }

  const auto* strlit = clang::dyn_cast<clang::StringLiteral>(eval_result_expr);
  if (!strlit) {
    return error();
  }

  return strlit->getString();
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

namespace llvm {

bool DenseMapInfo<clang::tidy::lifetimes::ValueLifetimes>::isEqual(
    const clang::tidy::lifetimes::ValueLifetimes& lhs,
    const clang::tidy::lifetimes::ValueLifetimes& rhs) {
  if (lhs.type_ != rhs.type_) {
    return false;
  }
  if ((lhs.pointee_lifetimes_ == nullptr) !=
      (rhs.pointee_lifetimes_ == nullptr)) {
    return false;
  }
  if (lhs.pointee_lifetimes_ &&
      !DenseMapInfo<clang::tidy::lifetimes::ObjectLifetimes>::isEqual(
          *lhs.pointee_lifetimes_, *rhs.pointee_lifetimes_)) {
    return false;
  }
  if (lhs.template_argument_lifetimes_.size() !=
      rhs.template_argument_lifetimes_.size()) {
    return false;
  }
  for (size_t i = 0; i < lhs.template_argument_lifetimes_.size(); i++) {
    if (lhs.template_argument_lifetimes_[i].size() !=
        rhs.template_argument_lifetimes_[i].size()) {
      return false;
    }
    for (size_t j = 0; j < lhs.template_argument_lifetimes_[i].size(); j++) {
      const auto& alhs = lhs.template_argument_lifetimes_[i][j];
      const auto& arhs = rhs.template_argument_lifetimes_[i][j];
      if (alhs.has_value() != arhs.has_value()) {
        return false;
      }
      if (alhs.has_value() && !isEqual(*alhs, *arhs)) {
        return false;
      }
    }
  }
  if (lhs.lifetime_parameters_by_name_.GetMapping() !=
      rhs.lifetime_parameters_by_name_.GetMapping()) {
    return false;
  }
  return true;
}

unsigned DenseMapInfo<clang::tidy::lifetimes::ValueLifetimes>::getHashValue(
    const clang::tidy::lifetimes::ValueLifetimes& value_lifetimes) {
  llvm::hash_code hash = 0;
  if (value_lifetimes.pointee_lifetimes_) {
    hash = DenseMapInfo<clang::tidy::lifetimes::ObjectLifetimes>::getHashValue(
        *value_lifetimes.pointee_lifetimes_);
  }
  for (const auto& lifetimes_at_depth :
       value_lifetimes.template_argument_lifetimes_) {
    for (const auto& tmpl_lifetime : lifetimes_at_depth) {
      if (tmpl_lifetime) {
        hash = hash_combine(hash, getHashValue(*tmpl_lifetime));
      }
    }
  }
  for (const auto& lifetime_arg :
       value_lifetimes.lifetime_parameters_by_name_.GetMapping()) {
    hash = hash_combine(hash, DenseMapInfo<llvm::StringRef>::getHashValue(
                                  lifetime_arg.first()));
    hash = hash_combine(
        hash, DenseMapInfo<clang::tidy::lifetimes::Lifetime>::getHashValue(
                  lifetime_arg.second));
  }
  return hash_combine(
      hash, DenseMapInfo<clang::QualType>::getHashValue(value_lifetimes.type_));
}

}  // namespace llvm

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/builtin_lifetimes.h"

#include <cassert>
#include <optional>
#include <string>

#include "lifetime_annotations/function_lifetimes.h"
#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/lifetime_annotations.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "clang/include/clang/AST/ASTContext.h"
#include "clang/include/clang/AST/Decl.h"
#include "clang/include/clang/AST/Type.h"
#include "clang/include/clang/AST/TypeLoc.h"
#include "clang/include/clang/Basic/Builtins.h"
#include "llvm/include/llvm/ADT/SmallVector.h"
#include "llvm/include/llvm/ADT/StringRef.h"
#include "llvm/include/llvm/Support/Error.h"

namespace clang {
namespace tidy {
namespace lifetimes {

namespace {

class ForwardAndMoveFactory : public FunctionLifetimeFactory {
  llvm::Expected<ValueLifetimes> CreateThisLifetimes(
      clang::QualType type, const clang::Expr*) const override {
    return CreateParamLifetimes(type, clang::TypeLoc());
  }
  llvm::Expected<ValueLifetimes> CreateParamLifetimes(
      clang::QualType type, clang::TypeLoc) const override {
    return ValueLifetimes::Create(
        type, [](const clang::Expr*) { return Lifetime::CreateVariable(); });
  }

  llvm::Expected<ValueLifetimes> CreateReturnLifetimes(
      clang::QualType type, clang::TypeLoc,
      const llvm::SmallVector<ValueLifetimes>& param_lifetimes,
      const std::optional<ValueLifetimes>& /*this_lifetimes*/) const override {
    assert(param_lifetimes.size() == 1);
    // `forward` and `move` convert from one type of reference to the other; the
    // lifetimes in the pointees of these references are the same.
    return ValueLifetimes::ForPointerLikeType(
        type, param_lifetimes[0].GetPointeeLifetimes());
  }
};

}  // namespace

FunctionLifetimesOrError GetBuiltinLifetimes(const clang::FunctionDecl* decl) {
  unsigned builtin_id = decl->getBuiltinID();
  const auto& builtin_info = decl->getASTContext().BuiltinInfo;
  assert(builtin_id != 0);

  if (!builtin_info.hasPtrArgsOrResult(builtin_id) &&
      !builtin_info.hasReferenceArgsOrResult(builtin_id)) {
    return FunctionLifetimes::CreateForDecl(
               decl,
               FunctionLifetimeFactorySingleCallback([](const clang::Expr*) {
                 assert(false);
                 return Lifetime();
               }))
        .get();
  }
  switch (builtin_id) {
    case clang::Builtin::BI__builtin_addressof:
      return ParseLifetimeAnnotations(decl, "a -> a").get();
    case clang::Builtin::BIstrtod:
    case clang::Builtin::BIstrtof:
      return ParseLifetimeAnnotations(decl, "a, (a, b)").get();
    case clang::Builtin::BIstrtoll:
    case clang::Builtin::BIstrtol:
      return ParseLifetimeAnnotations(decl, "a, (a, b), ()").get();
    case clang::Builtin::BI__builtin_memchr:
      return ParseLifetimeAnnotations(decl, "a, (), () -> a").get();
    case clang::Builtin::BI__builtin_strchr:
    case clang::Builtin::BI__builtin_strrchr:
      return ParseLifetimeAnnotations(decl, "a, () -> a").get();
    case clang::Builtin::BI__builtin_strstr:
    case clang::Builtin::BI__builtin_strpbrk:
      return ParseLifetimeAnnotations(decl, "a, b -> a").get();
    case clang::Builtin::BIforward:
    case clang::Builtin::BImove: {
      FunctionLifetimes result;
      return FunctionLifetimes::CreateForDecl(decl, ForwardAndMoveFactory())
          .get();
    }
    // TODO(veluca): figure out variadic functions.
    default:
      return FunctionAnalysisError("Unknown builtin: '" +
                                   builtin_info.getName(builtin_id) + "'");
  }
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

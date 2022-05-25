// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/points_to_map.h"

#include <string>
#include <utility>
#include <vector>

#include "absl/strings/str_format.h"
#include "absl/strings/str_join.h"
#include "clang/AST/Expr.h"

namespace clang {
namespace tidy {
namespace lifetimes {

bool PointsToMap::operator==(const PointsToMap& other) const {
  return pointer_points_tos_ == other.pointer_points_tos_ &&
         expr_objects_ == other.expr_objects_;
}

std::string PointsToMap::DebugString() const {
  std::vector<std::string> parts;
  for (const auto& [pointer, points_to] : pointer_points_tos_) {
    parts.push_back(absl::StrFormat("%s -> %s", pointer.DebugString(),
                                    points_to.DebugString()));
  }
  for (const auto& [expr, objects] : expr_objects_) {
    parts.push_back(absl::StrFormat("%s (%p) -> %s", expr->getStmtClassName(),
                                    expr, objects.DebugString()));
  }
  return absl::StrJoin(parts, "\n");
}

PointsToMap PointsToMap::Union(const PointsToMap& other) const {
  PointsToMap result;

  result.pointer_points_tos_ = pointer_points_tos_;
  for (const auto& [pointer, points_to] : other.pointer_points_tos_) {
    result.pointer_points_tos_[pointer].Add(points_to);
  }
  // TODO(mboehme): Do we even need to perform a union on expression object
  // sets?
  result.expr_objects_ = expr_objects_;
  for (const auto& [expr, objects] : other.expr_objects_) {
    result.expr_objects_[expr].Add(objects);
  }

  return result;
}

ObjectSet PointsToMap::GetPointerPointsToSet(Object pointer) const {
  auto iter = pointer_points_tos_.find(pointer);
  if (iter == pointer_points_tos_.end()) {
    return ObjectSet();
  }
  return iter->second;
}

void PointsToMap::SetPointerPointsToSet(Object pointer, ObjectSet points_to) {
  pointer_points_tos_[pointer] = std::move(points_to);
}

void PointsToMap::SetPointerPointsToSet(const ObjectSet& pointers,
                                        const ObjectSet& points_to) {
  for (Object pointer : pointers) {
    SetPointerPointsToSet(pointer, points_to);
  }
}

void PointsToMap::ExtendPointerPointsToSet(Object pointer,
                                           const ObjectSet& points_to) {
  ObjectSet& set = pointer_points_tos_[pointer];
  set.Add(points_to);
}

ObjectSet PointsToMap::GetPointerPointsToSet(const ObjectSet& pointers) const {
  ObjectSet result;
  for (Object pointer : pointers) {
    auto iter = pointer_points_tos_.find(pointer);
    if (iter != pointer_points_tos_.end()) {
      result.Add(iter->second);
    }
  }
  return result;
}

ObjectSet PointsToMap::GetExprObjectSet(const clang::Expr* expr) const {
  // We can't handle `ParenExpr`s like other `Expr`s because the CFG doesn't
  // contain `CFGStmt`s for them. Instead, if we encounter a `ParenExpr` here,
  // we simply return the object set for its subexpression.
  if (auto paren = clang::dyn_cast<clang::ParenExpr>(expr)) {
    expr = paren->getSubExpr();
  }

  assert(expr->isGLValue() || expr->getType()->isPointerType() ||
         expr->getType()->isArrayType() || expr->getType()->isFunctionType() ||
         expr->getType()->isBuiltinType());

  auto iter = expr_objects_.find(expr);
  if (iter == expr_objects_.end()) {
    llvm::errs() << "Didn't find object set for expression:\n";
    expr->dump();
    llvm::report_fatal_error("Didn't find object set for expression");
  }
  return iter->second;
}

void PointsToMap::SetExprObjectSet(const clang::Expr* expr, ObjectSet objects) {
  assert(expr->isGLValue() || expr->getType()->isPointerType() ||
         expr->getType()->isArrayType() || expr->getType()->isBuiltinType());
  expr_objects_[expr] = std::move(objects);
}

std::vector<Object> PointsToMap::GetAllPointersWithLifetime(
    Lifetime lifetime) const {
  std::vector<Object> result;
  for (const auto& [pointer, _] : pointer_points_tos_) {
    if (pointer.GetLifetime() == lifetime) {
      result.push_back(pointer);
    }
  }
  return result;
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/googlesql_value_nullability.h"

#include <cassert>

#include "absl/base/nullability.h"
#include "clang/AST/DeclCXX.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "clang/Analysis/FlowSensitive/Formula.h"
#include "clang/Analysis/FlowSensitive/StorageLocation.h"
#include "clang/Analysis/FlowSensitive/Value.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

static constexpr char kGoogleSqlIsNull[] = "googlesql_is_null";

}  // namespace

dataflow::LatticeJoinEffect GoogleSqlValueNullState::join(
    const GoogleSqlValueNullState& Other) {
  if (*this == Other) return dataflow::LatticeJoinEffect::Unchanged;
  if (IsNull == Other.IsNull) return dataflow::LatticeJoinEffect::Unchanged;
  IsNull = nullptr;
  return dataflow::LatticeJoinEffect::Changed;
}

static dataflow::StorageLocation& getOrAddGoogleSqlNullField(
    dataflow::RecordStorageLocation& Loc, dataflow::Environment& Env) {
  for (const auto& Entry : Loc.synthetic_fields()) {
    if (Entry.getKey() == kGoogleSqlIsNull) return *Entry.getValue();
  }
  auto& Ctx = Loc.getType()->getAsCXXRecordDecl()->getASTContext();
  auto& FieldLoc = Env.createStorageLocation(Ctx.BoolTy);
  Loc.addSyntheticField(kGoogleSqlIsNull, FieldLoc);
  return FieldLoc;
}

bool hasGoogleSqlValueNullState(const dataflow::RecordStorageLocation& Loc,
                                const dataflow::Environment& Env) {
  for (const auto& Entry : Loc.synthetic_fields()) {
    if (Entry.getKey() == kGoogleSqlIsNull) {
      return Env.getValue(*Entry.getValue()) != nullptr;
    }
  }
  return false;
}

GoogleSqlValueNullState getGoogleSqlValueNullState(
    const dataflow::RecordStorageLocation& Loc,
    const dataflow::Environment& Env) {
  const dataflow::StorageLocation* FieldLoc = nullptr;
  for (const auto& Entry : Loc.synthetic_fields()) {
    if (Entry.getKey() == kGoogleSqlIsNull) {
      FieldLoc = Entry.getValue();
      break;
    }
  }
  if (!FieldLoc) return GoogleSqlValueNullState::getTop();

  auto* Val = Env.get<dataflow::BoolValue>(*FieldLoc);
  if (!Val) return GoogleSqlValueNullState::getTop();

  return {&Val->formula()};
}

void initGoogleSqlValueNullState(
    dataflow::RecordStorageLocation& Loc, dataflow::Environment& Env,
    const dataflow::Formula* absl_nullable IsNull) {
  dataflow::StorageLocation& FieldLoc = getOrAddGoogleSqlNullField(Loc, Env);
  auto& A = Env.arena();
  assert(Env.getValue(FieldLoc) == nullptr);
  Env.setValue(FieldLoc,
               IsNull != nullptr ? A.makeBoolValue(*IsNull) : A.makeTopValue());
}

void setGoogleSqlValueNullState(dataflow::RecordStorageLocation& Loc,
                                dataflow::Environment& Env,
                                const dataflow::Formula* absl_nullable IsNull) {
  dataflow::StorageLocation& FieldLoc = getOrAddGoogleSqlNullField(Loc, Env);
  auto& A = Env.arena();
  Env.setValue(FieldLoc,
               IsNull != nullptr ? A.makeBoolValue(*IsNull) : A.makeTopValue());
}

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

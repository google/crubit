// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_SLICE_REF_HEADER_MODULES_DEPOT_PATH_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_SLICE_REF_HEADER_MODULES_DEPOT_PATH_H_

#include "support/rs_std/slice_ref.h"

inline int SliceRefFirstViaDepotPath(rs::SliceRef<const int> slice) {
  return slice[0];
}

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_SLICE_REF_HEADER_MODULES_DEPOT_PATH_H_

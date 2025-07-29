// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/internal/check_no_mutable_aliasing.h"

#include <algorithm>
#include <cstddef>

#include "absl/log/check.h"
#include "absl/types/span.h"

namespace crubit::internal {
namespace {

void SortPtrDatas(absl::Span<PtrData> unsorted) {
  std::sort(
      unsorted.begin(), unsorted.end(),
      [](const PtrData& a, const PtrData& b) { return a.start < b.start; });
}

}  // namespace

void CheckNoMutableAliasingSpans(absl::Span<PtrData> mut_ptrs,
                                 absl::Span<PtrData> const_ptrs) {
  CHECK(!HasMutableAliasingSpans(mut_ptrs, const_ptrs));
}

bool HasMutableAliasingSpans(absl::Span<PtrData> mut_ptrs,
                             absl::Span<PtrData> const_ptrs) {
  if (mut_ptrs.empty()) {
    return false;
  }
  SortPtrDatas(mut_ptrs);
  SortPtrDatas(const_ptrs);
  // Check that mutable references do not alias with one another.
  for (size_t i = 0; i + 1 < mut_ptrs.size(); ++i) {
    if (mut_ptrs[i].end > mut_ptrs[i + 1].start) {
      return true;
    }
  }
  // Check that const references and mutable references do not alias.
  auto mut_iter = mut_ptrs.begin();
  for (const PtrData& const_ptr : const_ptrs) {
    while (mut_iter->end <= const_ptr.start) {
      mut_iter++;
      if (mut_iter == mut_ptrs.end()) {
        return false;
      }
    }
    if (mut_iter->start < const_ptr.end) {
      return true;
    }
  }
  return false;
}

}  // namespace crubit::internal

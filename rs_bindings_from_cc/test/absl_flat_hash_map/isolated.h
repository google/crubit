// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_ISOLATED_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_ISOLATED_H_

#include <cstdint>
#include <string_view>

#include "absl/types/span.h"
#include "rs_bindings_from_cc/test/absl_flat_hash_map/flat_hash_map.h"
#include "support/annotations.h"
#include "support/rs_std/slice_ref.h"
#include "support/rs_std/str_ref.h"

namespace crubit::test {

using MapWithTwoParams = absl::flat_hash_map<int32_t, uint64_t>;
using MapWithThreeParams = absl::flat_hash_map<int32_t, uint64_t, 42>;

// Fake and non-functional bridged type.
struct CRUBIT_BRIDGE("Bridged", "BridgedAbi", "BridgedAbi") Bridged final {
  int i;
};

// Maps of non-layout-compatible bridged types get bindings, but without the
// methods that access their elements.
using MapWithBridgedKey = absl::flat_hash_map<Bridged, int>;
using MapWithBridgedValue = absl::flat_hash_map<int, Bridged>;

// Maps of view types. `absl::Span` and `rs_std::StrRef` have Rust types with
// lifetimes, so their maps get bindings, but without the methods that access
// their elements, because the maps' Rust types don't have lifetime parameters
// for their elements yet (b/478868411). `std::string_view` and
// `rs_std::SliceRef` are accessed through their lifetime-less layout-compatible
// Rust types (`raw_string_view` and `*const [T]`), so their maps get all of the
// methods.
using MapWithStringViewKey = absl::flat_hash_map<std::string_view, int32_t>;
using MapWithStringViewValue = absl::flat_hash_map<int32_t, std::string_view>;
using MapWithSpanValue =
    absl::flat_hash_map<int32_t, absl::Span<const int32_t>>;
using MapWithStrRefValue = absl::flat_hash_map<int32_t, rs_std::StrRef>;
using MapWithSliceRefValue =
    absl::flat_hash_map<int32_t, rs_std::SliceRef<const int32_t>>;

// Functions that return the elements of maps of view types, or references to
// them. These are stubs, because only their signatures matter.
//
// Returning `absl::Span` and `rs_std::SliceRef` by value also instantiates
// them, which is required to use them as the value types of maps.
inline std::string_view GetAtFive(MapWithStringViewValue map) { return {}; }
inline const std::string_view& GetRefAtFive(const MapWithStringViewValue& map) {
  static constexpr std::string_view kEmpty;
  return kEmpty;
}
inline absl::Span<const int32_t> GetSpanAtFive(MapWithSpanValue map) {
  return {};
}
inline rs_std::SliceRef<const int32_t> GetSliceRefAtFive(
    MapWithSliceRefValue map) {
  return {};
}

class Incomplete;

using MapWithIncompleteKey = absl::flat_hash_map<Incomplete, int>;
using MapWithIncompleteValue = absl::flat_hash_map<int, Incomplete>;

class NoDestructor final {
 private:
  ~NoDestructor();
};

using MapWithNoDestructorKey = absl::flat_hash_map<NoDestructor, int>;
using MapWithNoDestructorValue = absl::flat_hash_map<int, NoDestructor>;

class NoDelete final {
 private:
  void operator delete(void* ptr);
};

using MapWithNoDeleteKey = absl::flat_hash_map<NoDelete, int>;
using MapWithNoDeleteValue = absl::flat_hash_map<int, NoDelete>;

// `reserve`, `insert`, and `try_insert` construct elements from rvalues, so
// they aren't generated for element types that have neither a move constructor
// nor a copy constructor.
class NoMoveNoCopy final {
 public:
  NoMoveNoCopy() = default;
  NoMoveNoCopy(const NoMoveNoCopy&) = delete;
  NoMoveNoCopy(NoMoveNoCopy&&) = delete;
};

using MapWithNoMoveNoCopyKey = absl::flat_hash_map<NoMoveNoCopy, int>;
using MapWithNoMoveNoCopyValue = absl::flat_hash_map<int, NoMoveNoCopy>;

// An rvalue of a type without a move constructor binds to its copy
// constructor, so `reserve`, `insert`, and `try_insert` are generated.
class CopyOnly final {
 public:
  CopyOnly() = default;
  CopyOnly(const CopyOnly&) = default;
};

using MapWithCopyOnlyKey = absl::flat_hash_map<CopyOnly, int>;
using MapWithCopyOnlyValue = absl::flat_hash_map<int, CopyOnly>;

}  //  namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_ISOLATED_H_

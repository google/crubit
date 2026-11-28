// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// reexport_beside_function_golden

// clang-format off
#ifndef CC_BINDINGS_FROM_RS_TEST_USES_REEXPORT_BESIDE_FUNCTION_REEXPORT_BESIDE_FUNCTION_GOLDEN
#define CC_BINDINGS_FROM_RS_TEST_USES_REEXPORT_BESIDE_FUNCTION_REEXPORT_BESIDE_FUNCTION_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#pragma clang diagnostic ignored "-Wshadow"
#include "support/annotations_internal.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

#include "cc_bindings_from_rs/test/uses/reexport_beside_function/middle_crate.h"  // IWYU pragma: export

#pragma clang assume_nonnull begin

namespace reexport_beside_function {

struct CRUBIT_INTERNAL_RUST_TYPE(":: reexport_beside_function_golden :: Braced")
    [[clang::trivial_abi]] Braced final {
 public:
  ::std::int32_t value{};

 private:
  static void __crubit_field_offset_assertions();
};

}

namespace reexport_beside_function {

::std::int32_t Braced();

::std::int32_t Local();

}

namespace reexport_beside_function {

static_assert(
    sizeof(::reexport_beside_function::Braced) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::reexport_beside_function::Braced) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<::reexport_beside_function::Braced>);
static_assert(::std::is_trivially_move_constructible_v<
              ::reexport_beside_function::Braced>);
static_assert(
    ::std::is_trivially_move_assignable_v<::reexport_beside_function::Braced>);
inline void ::reexport_beside_function::Braced::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::reexport_beside_function::Braced;
  static_assert(0 == offsetof(__crubit_assert_type, value));
}
}

namespace reexport_beside_function {

using Local_ CRUBIT_INTERNAL_RUST_TYPE(":: middle_crate :: Local") =
    ::middle_crate::Local;
}

namespace reexport_beside_function {

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_Braced();
}
inline ::std::int32_t Braced() {
  return __crubit_internal::__crubit_thunk_Braced();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_Local();
}
inline ::std::int32_t Local() {
  return __crubit_internal::__crubit_thunk_Local();
}

}

#pragma clang assume_nonnull end

#pragma clang diagnostic pop
#endif  // CC_BINDINGS_FROM_RS_TEST_USES_REEXPORT_BESIDE_FUNCTION_REEXPORT_BESIDE_FUNCTION_GOLDEN

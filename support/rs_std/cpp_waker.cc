// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/cpp_waker.h"

namespace rs_std::internal {

extern "C" CppWaker* rs_std_cpp_waker_clone(CppWaker* in) {
  return in->Clone();
}

extern "C" void rs_std_cpp_waker_wake_and_destroy(CppWaker* in) {
  in->WakeAndDestroy();
}

extern "C" void rs_std_cpp_waker_wake_by_ref(CppWaker* in) { in->WakeByRef(); }

extern "C" void rs_std_cpp_waker_drop(CppWaker* in) { in->Destroy(); }

}  // namespace rs_std::internal

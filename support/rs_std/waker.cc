// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/rs_std/waker.h"

namespace rs_std::internal {

extern "C" Waker* rs_std_waker_clone(Waker* in) {
  return in->Clone();
}

extern "C" void rs_std_waker_wake_and_destroy(Waker* in) {
  in->WakeAndDestroy();
}

extern "C" void rs_std_waker_wake_by_ref(Waker* in) { in->WakeByRef(); }

extern "C" void rs_std_waker_drop(Waker* in) { in->Destroy(); }

}  // namespace rs_std::internal

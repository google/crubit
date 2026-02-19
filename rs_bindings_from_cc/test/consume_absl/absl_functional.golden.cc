// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/consume_absl:absl_functional
// Features: callables, custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#include "support/bridge.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"
#include "support/rs_std/dyn_callable.h"

#include <cstddef>
#include <memory>

#include "absl/functional/any_invocable_crubit_abi.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/consume_absl/absl_functional.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __CcTemplateInstN4absl12AnyInvocableIFvvOEEE_invoker(
    ::rs_std::internal_dyn_callable::TypeErasedState* state);
extern "C" void __CcTemplateInstN4absl12AnyInvocableIFvvOEEE_manager(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to);

extern "C" int __CcTemplateInstN4absl12AnyInvocableIKFiiEEE_invoker(
    ::rs_std::internal_dyn_callable::TypeErasedState* state, int param_0);
extern "C" void __CcTemplateInstN4absl12AnyInvocableIKFiiEEE_manager(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to);

extern "C" void
__rust_thunk___ZN24absl_functional_internal12CallVoidVoidEN4absl12AnyInvocableIFvvOEEE(
    const unsigned char* f) {
  ::crubit::Decoder __f_decoder(::crubit::AnyInvocableAbi<void() &&>::kSize, f);
  absl_functional_internal::CallVoidVoid(
      ::crubit::AnyInvocableAbi<void() &&>(
          [](absl::internal_any_invocable::FunctionToCall operation,
             absl::internal_any_invocable::TypeErasedState* from,
             absl::internal_any_invocable::TypeErasedState* to) noexcept {
            __CcTemplateInstN4absl12AnyInvocableIFvvOEEE_manager(operation,
                                                                 from, to);
          },
          [](::rs_std::internal_dyn_callable::TypeErasedState* state) -> void {
            __CcTemplateInstN4absl12AnyInvocableIFvvOEEE_invoker(state);
          })
          .Decode(__f_decoder));
}

static_assert((void (*)(class absl::AnyInvocable<void() &&>)) &
              ::absl_functional_internal::CallVoidVoid);

extern "C" void __rust_thunk___ZN24absl_functional_internal13ReturnIntVoidEv(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::AnyInvocableAbi<int(int) const>::kSize, __return_abi_buffer);
  ::crubit::AnyInvocableAbi<int(int) const>(
      [](absl::internal_any_invocable::FunctionToCall operation,
         absl::internal_any_invocable::TypeErasedState* from,
         absl::internal_any_invocable::TypeErasedState* to) noexcept {
        __CcTemplateInstN4absl12AnyInvocableIKFiiEEE_manager(operation, from,
                                                             to);
      },
      [](::rs_std::internal_dyn_callable::TypeErasedState* state,
         int param_0) -> int {
        return __CcTemplateInstN4absl12AnyInvocableIKFiiEEE_invoker(state,
                                                                    param_0);
      })
      .Encode(absl_functional_internal::ReturnIntVoid(), __return_encoder);
}

static_assert((class absl::AnyInvocable<int(int) const> (*)()) &
              ::absl_functional_internal::ReturnIntVoid);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string_view<wchar_t, std::char_traits<wchar_t>>) ==
    16);
static_assert(
    alignof(class std::basic_string_view<wchar_t, std::char_traits<wchar_t>>) ==
    8);

#pragma clang diagnostic pop

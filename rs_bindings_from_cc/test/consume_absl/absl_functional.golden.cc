// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/consume_absl:absl_functional
// Features: callables, supported, types

#include "support/bridge.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"
#include "support/rs_std/dyn_callable.h"

#include <cstddef>
#include <memory>
#include <utility>

#include "absl/functional/any_invocable_crubit_abi.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/consume_absl/absl_functional.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void
__crubit_invoker___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
    ::absl::internal_any_invocable::TypeErasedState* state);
extern "C" void
__crubit_manager___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" void
__crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKF8MyOptionIiES2_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
    ::absl::internal_any_invocable::TypeErasedState* state,
    unsigned char* param_0, unsigned char* out);
extern "C" void
__crubit_manager___CcTemplateInstN4absl12AnyInvocableIKF8MyOptionIiES2_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" int
__crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
    ::absl::internal_any_invocable::TypeErasedState* state, int param_0);
extern "C" void
__crubit_manager___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" void
__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
    ::absl::AnyInvocable<void() &&>* f) {
  std::move (*f)();
}
extern "C" void
__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKF8MyOptionIiES2_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
    ::absl::AnyInvocable<struct MyOption<int>(struct MyOption<int>) const>* f,
    unsigned char* param_0, unsigned char* out) {
  ::crubit::internal::Encode(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<int>>(
          ::crubit::TransmuteAbi<int>()),
      out,
      (*f)(::crubit::internal::Decode<
           ::crubit::MyOptionAbi<::crubit::TransmuteAbi<int>>>(
          ::crubit::MyOptionAbi<::crubit::TransmuteAbi<int>>(
              ::crubit::TransmuteAbi<int>()),
          param_0)));
}
extern "C" int
__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
    ::absl::AnyInvocable<int(int) const>* f, int param_0) {
  return (*f)(param_0);
}
extern "C" void __rust_thunk___Z12CallVoidVoidN4absl12AnyInvocableIFvvOEEE(
    const unsigned char* f) {
  ::crubit::Decoder __f_decoder(::crubit::AnyInvocableAbi<void() &&>::kSize, f);
  CallVoidVoid(
      ::crubit::AnyInvocableAbi<void() &&>(
          __crubit_manager___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional,
          [](::absl::internal_any_invocable::TypeErasedState* state) noexcept
              -> void {
            __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
                state);
          })
          .Decode(__f_decoder));
}

static_assert((void (*)(class absl::AnyInvocable<void() &&>)) & ::CallVoidVoid);

extern "C" void __rust_thunk___Z15ReturnIntMapperv(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::AnyInvocableAbi<int(int) const>::kSize, __return_abi_buffer);
  ::crubit::AnyInvocableAbi<int(int) const>(
      __crubit_manager___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional,
      [](::absl::internal_any_invocable::TypeErasedState* state,
         ::absl::internal_any_invocable::ForwardedParameterType<int>
             param_0) noexcept -> int {
        return __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
            state, param_0);
      })
      .Encode(ReturnIntMapper(), __return_encoder);
}

static_assert((class absl::AnyInvocable<int(int) const> (*)()) &
              ::ReturnIntMapper);

extern "C" void __rust_thunk___Z17MyOptionIntMapperv(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::AnyInvocableAbi<struct MyOption<int>(struct MyOption<int>)
                                    const>::kSize,
      __return_abi_buffer);
  ::crubit::AnyInvocableAbi<struct MyOption<int>(struct MyOption<int>) const>(
      __crubit_manager___CcTemplateInstN4absl12AnyInvocableIKF8MyOptionIiES2_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional,
      [](::absl::internal_any_invocable::TypeErasedState* state,
         ::absl::internal_any_invocable::ForwardedParameterType<
             struct MyOption<int>>
             param_0) noexcept -> struct MyOption<int> {
        unsigned char bridge_param_0
            [::crubit::MyOptionAbi<::crubit::TransmuteAbi<int>>::kSize];
        ::crubit::internal::Encode(
            ::crubit::MyOptionAbi<::crubit::TransmuteAbi<int>>(
                ::crubit::TransmuteAbi<int>()),
            bridge_param_0, param_0);
        unsigned char
            out[::crubit::MyOptionAbi<::crubit::TransmuteAbi<int>>::kSize];
        __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKF8MyOptionIiES2_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
            state, bridge_param_0, out);
        return ::crubit::internal::Decode<
            ::crubit::MyOptionAbi<::crubit::TransmuteAbi<int>>>(
            ::crubit::MyOptionAbi<::crubit::TransmuteAbi<int>>(
                ::crubit::TransmuteAbi<int>()),
            out);
      })
      .Encode(MyOptionIntMapper(), __return_encoder);
}

static_assert(
    (class absl::AnyInvocable<MyOption<int>(MyOption<int>) const> (*)()) &
    ::MyOptionIntMapper);

#pragma clang diagnostic pop

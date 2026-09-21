// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:callables_cc

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
#include "rs_bindings_from_cc/test/golden/callables.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void
__crubit_invoker___CcTemplateInstN4absl12AnyInvocableIFvRiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::TypeErasedState* state, int& param_0);
extern "C" void
__crubit_manager___CcTemplateInstN4absl12AnyInvocableIFvRiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" void
__crubit_invoker___CcTemplateInstN4absl12AnyInvocableIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::TypeErasedState* state);
extern "C" void
__crubit_manager___CcTemplateInstN4absl12AnyInvocableIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" void
__crubit_invoker___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::TypeErasedState* state);
extern "C" void
__crubit_manager___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" void
__crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKF13ABICompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::TypeErasedState* state,
    struct ABICompatible* param_0, struct ABICompatible* out);
extern "C" void
__crubit_manager___CcTemplateInstN4absl12AnyInvocableIKF13ABICompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" void
__crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKF16LayoutCompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::TypeErasedState* state,
    class LayoutCompatible* param_0, class LayoutCompatible* out);
extern "C" void
__crubit_manager___CcTemplateInstN4absl12AnyInvocableIKF16LayoutCompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" void
__crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKF7BridgedS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::TypeErasedState* state,
    unsigned char* param_0, unsigned char* out);
extern "C" void
__crubit_manager___CcTemplateInstN4absl12AnyInvocableIKF7BridgedS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" int
__crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::TypeErasedState* state, int param_0);
extern "C" void
__crubit_manager___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" void
__crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::TypeErasedState* state);
extern "C" void
__crubit_manager___CcTemplateInstN4absl12AnyInvocableIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" void
__crubit_invoker___CcTemplateInstN6rs_std11DynCallableIFvRiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::TypeErasedState* state, int& param_0);
extern "C" void
__crubit_manager___CcTemplateInstN6rs_std11DynCallableIFvRiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" void
__crubit_invoker___CcTemplateInstN6rs_std11DynCallableIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::TypeErasedState* state);
extern "C" void
__crubit_manager___CcTemplateInstN6rs_std11DynCallableIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" void
__crubit_invoker___CcTemplateInstN6rs_std11DynCallableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::TypeErasedState* state);
extern "C" void
__crubit_manager___CcTemplateInstN6rs_std11DynCallableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" void
__crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKF13ABICompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::TypeErasedState* state,
    struct ABICompatible* param_0, struct ABICompatible* out);
extern "C" void
__crubit_manager___CcTemplateInstN6rs_std11DynCallableIKF13ABICompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" void
__crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::TypeErasedState* state,
    class LayoutCompatible* param_0, class LayoutCompatible* out);
extern "C" void
__crubit_manager___CcTemplateInstN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" void
__crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKF7BridgedS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::TypeErasedState* state,
    unsigned char* param_0, unsigned char* out);
extern "C" void
__crubit_manager___CcTemplateInstN6rs_std11DynCallableIKF7BridgedS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" int
__crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::TypeErasedState* state, int param_0);
extern "C" void
__crubit_manager___CcTemplateInstN6rs_std11DynCallableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" void
__crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::TypeErasedState* state);
extern "C" void
__crubit_manager___CcTemplateInstN6rs_std11DynCallableIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::internal_any_invocable::FunctionToCall operation,
    ::absl::internal_any_invocable::TypeErasedState* from,
    ::absl::internal_any_invocable::TypeErasedState* to) noexcept;

extern "C" void
__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFvRiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::AnyInvocable<void(int&)>* f, int& param_0) {
  (*f)(param_0);
}
extern "C" void
__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::AnyInvocable<void()>* f) {
  (*f)();
}
extern "C" void
__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::AnyInvocable<void() &&>* f) {
  std::move (*f)();
}
extern "C" void
__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKF13ABICompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::AnyInvocable<struct ABICompatible(struct ABICompatible) const>* f,
    struct ABICompatible* param_0, struct ABICompatible* out) {
  new (out) struct ABICompatible((*f)(std::move(*param_0)));
}
extern "C" void
__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKF16LayoutCompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::AnyInvocable<class LayoutCompatible(class LayoutCompatible) const>*
        f,
    class LayoutCompatible* param_0, class LayoutCompatible* out) {
  new (out) class LayoutCompatible((*f)(std::move(*param_0)));
}
extern "C" void
__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKF7BridgedS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::AnyInvocable<struct Bridged(struct Bridged) const>* f,
    unsigned char* param_0, unsigned char* out) {
  ::crubit::internal::Encode(
      ::crubit::BridgedAbi(), out,
      (*f)(::crubit::internal::Decode<::crubit::BridgedAbi>(
          ::crubit::BridgedAbi(), param_0)));
}
extern "C" int
__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::AnyInvocable<int(int) const>* f, int param_0) {
  return (*f)(param_0);
}
extern "C" void
__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
    ::absl::AnyInvocable<void() const>* f) {
  (*f)();
}
extern "C" void __rust_thunk___Z11invoke_onceN6rs_std11DynCallableIFvvOEEE(
    const unsigned char* f) {
  ::crubit::Decoder __f_decoder(
      ::rs_std::internal_dyn_callable::DynCallableAbi<void() &&>::kSize, f);
  invoke_once(
      ::rs_std::internal_dyn_callable::DynCallableAbi<void() &&>(
          __crubit_manager___CcTemplateInstN6rs_std11DynCallableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc,
          [](::absl::internal_any_invocable::TypeErasedState* state) noexcept
              -> void {
            __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
                state);
          })
          .Decode(__f_decoder));
}

static_assert((void (*)(class rs_std::DynCallable<void() &&>)) & ::invoke_once);

extern "C" void __rust_thunk___Z6invokeN6rs_std11DynCallableIFvvEEE(
    const unsigned char* f) {
  ::crubit::Decoder __f_decoder(
      ::rs_std::internal_dyn_callable::DynCallableAbi<void()>::kSize, f);
  invoke(
      ::rs_std::internal_dyn_callable::DynCallableAbi<void()>(
          __crubit_manager___CcTemplateInstN6rs_std11DynCallableIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc,
          [](::absl::internal_any_invocable::TypeErasedState* state) noexcept
              -> void {
            __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
                state);
          })
          .Decode(__f_decoder));
}

static_assert((void (*)(class rs_std::DynCallable<void()>)) & ::invoke);

extern "C" void __rust_thunk___Z12invoke_constN6rs_std11DynCallableIKFvvEEE(
    const unsigned char* f) {
  ::crubit::Decoder __f_decoder(
      ::rs_std::internal_dyn_callable::DynCallableAbi<void() const>::kSize, f);
  invoke_const(
      ::rs_std::internal_dyn_callable::DynCallableAbi<void() const>(
          __crubit_manager___CcTemplateInstN6rs_std11DynCallableIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc,
          [](::absl::internal_any_invocable::TypeErasedState* state) noexcept
              -> void {
            __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
                state);
          })
          .Decode(__f_decoder));
}

static_assert((void (*)(class rs_std::DynCallable<void() const>)) &
              ::invoke_const);

extern "C" int __rust_thunk___Z7map_intN6rs_std11DynCallableIKFiiEEEi(
    const unsigned char* f, int arg) {
  ::crubit::Decoder __f_decoder(
      ::rs_std::internal_dyn_callable::DynCallableAbi<int(int) const>::kSize,
      f);
  return map_int(
      ::rs_std::internal_dyn_callable::DynCallableAbi<int(int) const>(
          __crubit_manager___CcTemplateInstN6rs_std11DynCallableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc,
          [](::absl::internal_any_invocable::TypeErasedState* state,
             ::absl::internal_any_invocable::ForwardedParameterType<int>
                 param_0) noexcept -> int {
            return __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
                state, param_0);
          })
          .Decode(__f_decoder),
      arg);
}

static_assert((int (*)(class rs_std::DynCallable<int(int) const>, int)) &
              ::map_int);

extern "C" void
__rust_thunk___Z11map_bridgedN6rs_std11DynCallableIKF7BridgedS1_EEES1_(
    unsigned char* __return_abi_buffer, const unsigned char* f,
    const unsigned char* arg) {
  ::crubit::Decoder __f_decoder(
      ::rs_std::internal_dyn_callable::DynCallableAbi<struct Bridged(
          struct Bridged) const>::kSize,
      f);
  ::crubit::Decoder __arg_decoder(::crubit::BridgedAbi::kSize, arg);
  ::crubit::Encoder __return_encoder(::crubit::BridgedAbi::kSize,
                                     __return_abi_buffer);
  ::crubit::BridgedAbi().Encode(
      map_bridged(
          ::rs_std::internal_dyn_callable::DynCallableAbi<struct Bridged(
              struct Bridged) const>(
              __crubit_manager___CcTemplateInstN6rs_std11DynCallableIKF7BridgedS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc,
              [](::absl::internal_any_invocable::TypeErasedState* state,
                 ::absl::internal_any_invocable::ForwardedParameterType<
                     struct Bridged>
                     param_0) noexcept -> struct Bridged {
                unsigned char bridge_param_0[::crubit::BridgedAbi::kSize];
                ::crubit::internal::Encode(::crubit::BridgedAbi(),
                                           bridge_param_0,
                                           ::std::move(param_0));
                unsigned char out[::crubit::BridgedAbi::kSize];
                __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKF7BridgedS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
                    state, bridge_param_0, out);
                return ::crubit::internal::Decode<::crubit::BridgedAbi>(
                    ::crubit::BridgedAbi(), out);
              })
              .Decode(__f_decoder),
          ::crubit::BridgedAbi().Decode(__arg_decoder)),
      __return_encoder);
}

static_assert(
    (struct Bridged (*)(class rs_std::DynCallable<Bridged(Bridged) const>,
                        struct Bridged)) &
    ::map_bridged);

static_assert(CRUBIT_SIZEOF(struct ABICompatible) == 4);
static_assert(alignof(struct ABICompatible) == 4);
static_assert(CRUBIT_OFFSET_OF(x, struct ABICompatible) == 0);

extern "C" void __rust_thunk___ZN13ABICompatibleC1Ev(
    struct ABICompatible* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___Z18map_abi_compatibleN6rs_std11DynCallableIKF13ABICompatibleS1_EEES1_(
    struct ABICompatible* __return, const unsigned char* f,
    struct ABICompatible* arg) {
  ::crubit::Decoder __f_decoder(
      ::rs_std::internal_dyn_callable::DynCallableAbi<struct ABICompatible(
          struct ABICompatible) const>::kSize,
      f);
  new (__return) auto(map_abi_compatible(
      ::rs_std::internal_dyn_callable::DynCallableAbi<struct ABICompatible(
          struct ABICompatible) const>(
          __crubit_manager___CcTemplateInstN6rs_std11DynCallableIKF13ABICompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc,
          [](::absl::internal_any_invocable::TypeErasedState* state,
             ::absl::internal_any_invocable::ForwardedParameterType<
                 struct ABICompatible>
                 param_0) noexcept -> struct ABICompatible {
            ::crubit::Slot<struct ABICompatible> stack_param_0(
                std::move(param_0));
            ::crubit::Slot<struct ABICompatible> out;
            __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKF13ABICompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
                state, stack_param_0.Get(), out.Get());
            return std::move(out).AssumeInitAndTakeValue();
          })
          .Decode(__f_decoder),
      std::move(*arg)));
}

static_assert((struct ABICompatible (*)(
                  class rs_std::DynCallable<ABICompatible(ABICompatible) const>,
                  struct ABICompatible)) &
              ::map_abi_compatible);

static_assert(CRUBIT_SIZEOF(class LayoutCompatible) == 4);
static_assert(alignof(class LayoutCompatible) == 4);

extern "C" void __rust_thunk___ZN16LayoutCompatible6CreateEi(
    class LayoutCompatible* __return, int x) {
  new (__return) auto(LayoutCompatible::Create(x));
}

static_assert((class LayoutCompatible (*)(int)) & ::LayoutCompatible::Create);

extern "C" int __rust_thunk___ZNK16LayoutCompatible3getEv(
    class LayoutCompatible const* __this) {
  return __this->get();
}

static_assert((int (::LayoutCompatible::*)() const) & ::LayoutCompatible::get);

extern "C" void
__rust_thunk___Z21map_layout_compatibleN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEES1_(
    class LayoutCompatible* __return, const unsigned char* f,
    class LayoutCompatible* arg) {
  ::crubit::Decoder __f_decoder(
      ::rs_std::internal_dyn_callable::DynCallableAbi<class LayoutCompatible(
          class LayoutCompatible) const>::kSize,
      f);
  new (__return) auto(map_layout_compatible(
      ::rs_std::internal_dyn_callable::DynCallableAbi<class LayoutCompatible(
          class LayoutCompatible) const>(
          __crubit_manager___CcTemplateInstN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc,
          [](::absl::internal_any_invocable::TypeErasedState* state,
             ::absl::internal_any_invocable::ForwardedParameterType<
                 class LayoutCompatible>
                 param_0) noexcept -> class LayoutCompatible {
            ::crubit::Slot<class LayoutCompatible> stack_param_0(
                std::move(param_0));
            ::crubit::Slot<class LayoutCompatible> out;
            __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
                state, stack_param_0.Get(), out.Get());
            return std::move(out).AssumeInitAndTakeValue();
          })
          .Decode(__f_decoder),
      std::move(*arg)));
}

static_assert(
    (class LayoutCompatible (*)(
        class rs_std::DynCallable<LayoutCompatible(LayoutCompatible) const>,
        class LayoutCompatible)) &
    ::map_layout_compatible);

extern "C" int
__rust_thunk___Z25callable_taking_referenceN6rs_std11DynCallableIFvRiEEEi(
    const unsigned char* f, int arg) {
  ::crubit::Decoder __f_decoder(
      ::rs_std::internal_dyn_callable::DynCallableAbi<void(int&)>::kSize, f);
  return callable_taking_reference(
      ::rs_std::internal_dyn_callable::DynCallableAbi<void(int&)>(
          __crubit_manager___CcTemplateInstN6rs_std11DynCallableIFvRiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc,
          [](::absl::internal_any_invocable::TypeErasedState* state,
             ::absl::internal_any_invocable::ForwardedParameterType<int&>
                 param_0) noexcept -> void {
            __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIFvRiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
                state, param_0);
          })
          .Decode(__f_decoder),
      arg);
}

static_assert((int (*)(class rs_std::DynCallable<void(int&)>, int)) &
              ::callable_taking_reference);

static_assert(CRUBIT_SIZEOF(class std::initializer_list<char32_t>) == 16);
static_assert(alignof(class std::initializer_list<char32_t>) == 8);

extern "C" void __rust_thunk__94afaa9d__ZNSt16initializer_listIDiEC1Ev(
    class std::initializer_list<char32_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" char32_t const*
__rust_thunk__38024295__ZNKSt16initializer_listIDiE4dataEv(
    class std::initializer_list<char32_t> const* __this) {
  return __this->data();
}

static_assert((char32_t const* (::std::initializer_list<char32_t>::*)() const) &
              ::std::initializer_list<char32_t>::data);

extern "C" size_t __rust_thunk__5ae212d8__ZNKSt16initializer_listIDiE4sizeEv(
    class std::initializer_list<char32_t> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::initializer_list<char32_t>::*)() const) &
              ::std::initializer_list<char32_t>::size);

extern "C" bool __rust_thunk__3f660dcd__ZNKSt16initializer_listIDiE5emptyEv(
    class std::initializer_list<char32_t> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::initializer_list<char32_t>::*)() const) &
              ::std::initializer_list<char32_t>::empty);

extern "C" char32_t const*
__rust_thunk__f52c3b14__ZNKSt16initializer_listIDiE5beginEv(
    class std::initializer_list<char32_t> const* __this) {
  return __this->begin();
}

static_assert((char32_t const* (::std::initializer_list<char32_t>::*)() const) &
              ::std::initializer_list<char32_t>::begin);

extern "C" char32_t const*
__rust_thunk__026f8cac__ZNKSt16initializer_listIDiE3endEv(
    class std::initializer_list<char32_t> const* __this) {
  return __this->end();
}

static_assert((char32_t const* (::std::initializer_list<char32_t>::*)() const) &
              ::std::initializer_list<char32_t>::end);

static_assert(CRUBIT_SIZEOF(class std::initializer_list<char16_t>) == 16);
static_assert(alignof(class std::initializer_list<char16_t>) == 8);

extern "C" void __rust_thunk__94afaa9d__ZNSt16initializer_listIDsEC1Ev(
    class std::initializer_list<char16_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" char16_t const*
__rust_thunk__38024295__ZNKSt16initializer_listIDsE4dataEv(
    class std::initializer_list<char16_t> const* __this) {
  return __this->data();
}

static_assert((char16_t const* (::std::initializer_list<char16_t>::*)() const) &
              ::std::initializer_list<char16_t>::data);

extern "C" size_t __rust_thunk__5ae212d8__ZNKSt16initializer_listIDsE4sizeEv(
    class std::initializer_list<char16_t> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::initializer_list<char16_t>::*)() const) &
              ::std::initializer_list<char16_t>::size);

extern "C" bool __rust_thunk__3f660dcd__ZNKSt16initializer_listIDsE5emptyEv(
    class std::initializer_list<char16_t> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::initializer_list<char16_t>::*)() const) &
              ::std::initializer_list<char16_t>::empty);

extern "C" char16_t const*
__rust_thunk__f52c3b14__ZNKSt16initializer_listIDsE5beginEv(
    class std::initializer_list<char16_t> const* __this) {
  return __this->begin();
}

static_assert((char16_t const* (::std::initializer_list<char16_t>::*)() const) &
              ::std::initializer_list<char16_t>::begin);

extern "C" char16_t const*
__rust_thunk__026f8cac__ZNKSt16initializer_listIDsE3endEv(
    class std::initializer_list<char16_t> const* __this) {
  return __this->end();
}

static_assert((char16_t const* (::std::initializer_list<char16_t>::*)() const) &
              ::std::initializer_list<char16_t>::end);

static_assert(CRUBIT_SIZEOF(class std::initializer_list<char>) == 16);
static_assert(alignof(class std::initializer_list<char>) == 8);

extern "C" void __rust_thunk__94afaa9d__ZNSt16initializer_listIcEC1Ev(
    class std::initializer_list<char>* __this) {
  crubit::construct_at(__this);
}

extern "C" char const*
__rust_thunk__38024295__ZNKSt16initializer_listIcE4dataEv(
    class std::initializer_list<char> const* __this) {
  return __this->data();
}

static_assert((char const* (::std::initializer_list<char>::*)() const) &
              ::std::initializer_list<char>::data);

extern "C" size_t __rust_thunk__5ae212d8__ZNKSt16initializer_listIcE4sizeEv(
    class std::initializer_list<char> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::initializer_list<char>::*)() const) &
              ::std::initializer_list<char>::size);

extern "C" bool __rust_thunk__3f660dcd__ZNKSt16initializer_listIcE5emptyEv(
    class std::initializer_list<char> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::initializer_list<char>::*)() const) &
              ::std::initializer_list<char>::empty);

extern "C" char const*
__rust_thunk__f52c3b14__ZNKSt16initializer_listIcE5beginEv(
    class std::initializer_list<char> const* __this) {
  return __this->begin();
}

static_assert((char const* (::std::initializer_list<char>::*)() const) &
              ::std::initializer_list<char>::begin);

extern "C" char const* __rust_thunk__026f8cac__ZNKSt16initializer_listIcE3endEv(
    class std::initializer_list<char> const* __this) {
  return __this->end();
}

static_assert((char const* (::std::initializer_list<char>::*)() const) &
              ::std::initializer_list<char>::end);

static_assert(sizeof(struct std::placeholders::__ph<10>) == 1);
static_assert(alignof(struct std::placeholders::__ph<10>) == 1);

extern "C" void __rust_thunk__63952cf3__ZNSt3__u12placeholders4__phILi10EEC1Ev(
    struct std::placeholders::__ph<10>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<1>) == 1);
static_assert(alignof(struct std::placeholders::__ph<1>) == 1);

extern "C" void __rust_thunk__63952cf3__ZNSt3__u12placeholders4__phILi1EEC1Ev(
    struct std::placeholders::__ph<1>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<2>) == 1);
static_assert(alignof(struct std::placeholders::__ph<2>) == 1);

extern "C" void __rust_thunk__63952cf3__ZNSt3__u12placeholders4__phILi2EEC1Ev(
    struct std::placeholders::__ph<2>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<3>) == 1);
static_assert(alignof(struct std::placeholders::__ph<3>) == 1);

extern "C" void __rust_thunk__63952cf3__ZNSt3__u12placeholders4__phILi3EEC1Ev(
    struct std::placeholders::__ph<3>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<4>) == 1);
static_assert(alignof(struct std::placeholders::__ph<4>) == 1);

extern "C" void __rust_thunk__63952cf3__ZNSt3__u12placeholders4__phILi4EEC1Ev(
    struct std::placeholders::__ph<4>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<5>) == 1);
static_assert(alignof(struct std::placeholders::__ph<5>) == 1);

extern "C" void __rust_thunk__63952cf3__ZNSt3__u12placeholders4__phILi5EEC1Ev(
    struct std::placeholders::__ph<5>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<6>) == 1);
static_assert(alignof(struct std::placeholders::__ph<6>) == 1);

extern "C" void __rust_thunk__63952cf3__ZNSt3__u12placeholders4__phILi6EEC1Ev(
    struct std::placeholders::__ph<6>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<7>) == 1);
static_assert(alignof(struct std::placeholders::__ph<7>) == 1);

extern "C" void __rust_thunk__63952cf3__ZNSt3__u12placeholders4__phILi7EEC1Ev(
    struct std::placeholders::__ph<7>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<8>) == 1);
static_assert(alignof(struct std::placeholders::__ph<8>) == 1);

extern "C" void __rust_thunk__63952cf3__ZNSt3__u12placeholders4__phILi8EEC1Ev(
    struct std::placeholders::__ph<8>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::placeholders::__ph<9>) == 1);
static_assert(alignof(struct std::placeholders::__ph<9>) == 1);

extern "C" void __rust_thunk__63952cf3__ZNSt3__u12placeholders4__phILi9EEC1Ev(
    struct std::placeholders::__ph<9>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::allocator<char32_t>) == 1);
static_assert(alignof(class std::allocator<char32_t>) == 1);

extern "C" void __rust_thunk__fc1f3eb6__ZNSt3__u9allocatorIDiEC1Ev(
    class std::allocator<char32_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" char32_t* __rust_thunk__38880fec__ZNSt3__u9allocatorIDiE8allocateEm(
    class std::allocator<char32_t>* __this, size_t __n) {
  return __this->allocate(__n);
}

static_assert((char32_t* (::std::allocator<char32_t>::*)(size_t)) &
              ::std::allocator<char32_t>::allocate);

extern "C" void __rust_thunk__8e3856fe__ZNSt3__u9allocatorIDiE10deallocateEPDim(
    class std::allocator<char32_t>* __this, char32_t* __p, size_t __n) {
  __this->deallocate(__p, __n);
}

static_assert((void (::std::allocator<char32_t>::*)(char32_t*, size_t)) &
              ::std::allocator<char32_t>::deallocate);

static_assert(sizeof(class std::allocator<char16_t>) == 1);
static_assert(alignof(class std::allocator<char16_t>) == 1);

extern "C" void __rust_thunk__fc1f3eb6__ZNSt3__u9allocatorIDsEC1Ev(
    class std::allocator<char16_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" char16_t* __rust_thunk__38880fec__ZNSt3__u9allocatorIDsE8allocateEm(
    class std::allocator<char16_t>* __this, size_t __n) {
  return __this->allocate(__n);
}

static_assert((char16_t* (::std::allocator<char16_t>::*)(size_t)) &
              ::std::allocator<char16_t>::allocate);

extern "C" void __rust_thunk__8e3856fe__ZNSt3__u9allocatorIDsE10deallocateEPDsm(
    class std::allocator<char16_t>* __this, char16_t* __p, size_t __n) {
  __this->deallocate(__p, __n);
}

static_assert((void (::std::allocator<char16_t>::*)(char16_t*, size_t)) &
              ::std::allocator<char16_t>::deallocate);

static_assert(sizeof(class std::allocator<char>) == 1);
static_assert(alignof(class std::allocator<char>) == 1);

extern "C" void __rust_thunk__fc1f3eb6__ZNSt3__u9allocatorIcEC1Ev(
    class std::allocator<char>* __this) {
  crubit::construct_at(__this);
}

extern "C" char* __rust_thunk__38880fec__ZNSt3__u9allocatorIcE8allocateEm(
    class std::allocator<char>* __this, size_t __n) {
  return __this->allocate(__n);
}

static_assert((char* (::std::allocator<char>::*)(size_t)) &
              ::std::allocator<char>::allocate);

extern "C" void __rust_thunk__8e3856fe__ZNSt3__u9allocatorIcE10deallocateEPcm(
    class std::allocator<char>* __this, char* __p, size_t __n) {
  __this->deallocate(__p, __n);
}

static_assert((void (::std::allocator<char>::*)(char*, size_t)) &
              ::std::allocator<char>::deallocate);

static_assert(CRUBIT_SIZEOF(class std::pmr::polymorphic_allocator<char32_t>) ==
              8);
static_assert(alignof(class std::pmr::polymorphic_allocator<char32_t>) == 8);

extern "C" void
__rust_thunk__1c80154d__ZNSt3__u3pmr21polymorphic_allocatorIDiEC1Ev(
    class std::pmr::polymorphic_allocator<char32_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__34795f77__ZNSt3__u3pmr21polymorphic_allocatorIDiEC1EPNS0_15memory_resourceE(
    class std::pmr::polymorphic_allocator<char32_t>* __this,
    class ::std::__u::pmr::memory_resource* __r) {
  crubit::construct_at(__this, __r);
}

static_assert(CRUBIT_SIZEOF(class std::pmr::polymorphic_allocator<char16_t>) ==
              8);
static_assert(alignof(class std::pmr::polymorphic_allocator<char16_t>) == 8);

extern "C" void
__rust_thunk__1c80154d__ZNSt3__u3pmr21polymorphic_allocatorIDsEC1Ev(
    class std::pmr::polymorphic_allocator<char16_t>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__34795f77__ZNSt3__u3pmr21polymorphic_allocatorIDsEC1EPNS0_15memory_resourceE(
    class std::pmr::polymorphic_allocator<char16_t>* __this,
    class ::std::__u::pmr::memory_resource* __r) {
  crubit::construct_at(__this, __r);
}

static_assert(CRUBIT_SIZEOF(class std::pmr::polymorphic_allocator<char>) == 8);
static_assert(alignof(class std::pmr::polymorphic_allocator<char>) == 8);

extern "C" void
__rust_thunk__1c80154d__ZNSt3__u3pmr21polymorphic_allocatorIcEC1Ev(
    class std::pmr::polymorphic_allocator<char>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__34795f77__ZNSt3__u3pmr21polymorphic_allocatorIcEC1EPNS0_15memory_resourceE(
    class std::pmr::polymorphic_allocator<char>* __this,
    class ::std::__u::pmr::memory_resource* __r) {
  crubit::construct_at(__this, __r);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string<char32_t, std::char_traits<char32_t>,
                                std::pmr::polymorphic_allocator<char32_t>>) ==
    32);
static_assert(
    alignof(
        class std::basic_string<char32_t, std::char_traits<char32_t>,
                                std::pmr::polymorphic_allocator<char32_t>>) ==
    8);

extern "C" void
__rust_thunk__cc656414__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1Ev(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__6cef0e61__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ERKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::pmr::polymorphic_allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__f3a6ff79__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ERKS6_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __str) {
  crubit::construct_at(__this, *__str);
}

extern "C" void
__rust_thunk__5e32cc92__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ERKS6_RKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __str,
    class std::pmr::polymorphic_allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__str, *__a);
}

extern "C" void
__rust_thunk__b37f0b9f__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1EOS6_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __str) {
  crubit::construct_at(__this, std::move(*__str));
}

extern "C" void
__rust_thunk__975a6aa4__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1EOS6_RKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __str,
    class std::pmr::polymorphic_allocator<char32_t> const* __a) {
  crubit::construct_at(__this, std::move(*__str), *__a);
}

extern "C" void
__rust_thunk__f18e20ba__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ESt16initializer_listIDiERKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::initializer_list<char32_t>* __il,
    class std::pmr::polymorphic_allocator<char32_t> const* __a) {
  crubit::construct_at(__this, std::move(*__il), *__a);
}

extern "C" void
__rust_thunk__dee1b799__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEED1Ev(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>*
        __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::pmr::polymorphic_allocator<char32_t>>*
__rust_thunk__fc4f0119__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSEOS6_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __str) {
  return std::addressof(__this->operator=(std::move(*__str)));
}

static_assert(
    (class std::basic_string<char32_t, std::char_traits<char32_t>,
                             std::pmr::polymorphic_allocator<char32_t>> &
     (::std::basic_string<char32_t, std::char_traits<char32_t>,
                          std::pmr::polymorphic_allocator<char32_t>>::*)(
         class std::basic_string<
             char32_t, std::char_traits<char32_t>,
             std::pmr::polymorphic_allocator<char32_t>>&&)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::operator=);

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::pmr::polymorphic_allocator<char32_t>>*
__rust_thunk__e8738afb__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSESt16initializer_listIDiE(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::initializer_list<char32_t>* __il) {
  return std::addressof(__this->operator=(std::move(*__il)));
}

static_assert(
    (class std::basic_string<char32_t, std::char_traits<char32_t>,
                             std::pmr::polymorphic_allocator<char32_t>> &
     (::std::basic_string<char32_t, std::char_traits<char32_t>,
                          std::pmr::polymorphic_allocator<char32_t>>::*)(
         class std::initializer_list<char32_t>)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::operator=);

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::pmr::polymorphic_allocator<char32_t>>*
__rust_thunk__15d3c9ea__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSEPKDi(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    char32_t const* __s) {
  return std::addressof(__this->operator=(__s));
}

static_assert(
    (class std::basic_string<char32_t, std::char_traits<char32_t>,
                             std::pmr::polymorphic_allocator<char32_t>> &
     (::std::basic_string<char32_t, std::char_traits<char32_t>,
                          std::pmr::polymorphic_allocator<char32_t>>::*)(
         char32_t const*)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::operator=);

extern "C" void
__rust_thunk__d4331a79__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE6cbeginEv(
    class std::__wrap_iter<const char32_t*>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __this) {
  new (__return) auto(__this->cbegin());
}

static_assert(
    (class std::__wrap_iter<const char32_t*> (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()
         const) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::cbegin);

extern "C" void
__rust_thunk__a36bce04__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE4cendEv(
    class std::__wrap_iter<const char32_t*>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __this) {
  new (__return) auto(__this->cend());
}

static_assert(
    (class std::__wrap_iter<const char32_t*> (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()
         const) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::cend);

extern "C" void
__rust_thunk__7913a9b7__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE7crbeginEv(
    class std::reverse_iterator<std::__wrap_iter<const char32_t*>>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __this) {
  new (__return) auto(__this->crbegin());
}

static_assert(
    (class std::reverse_iterator<std::__wrap_iter<const char32_t*>> (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()
         const) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::crbegin);

extern "C" void
__rust_thunk__e750602f__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5crendEv(
    class std::reverse_iterator<std::__wrap_iter<const char32_t*>>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __this) {
  new (__return) auto(__this->crend());
}

static_assert(
    (class std::reverse_iterator<std::__wrap_iter<const char32_t*>> (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()
         const) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::crend);

extern "C" void
__rust_thunk__df2538bf__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE7reserveEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>*
        __this) {
  __this->reserve();
}

static_assert(
    (void (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::reserve);

extern "C" bool
__rust_thunk__95317e8a__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5emptyEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string<char32_t, std::char_traits<char32_t>,
                               std::pmr::polymorphic_allocator<char32_t>>::*)()
         const) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::empty);

extern "C" char32_t const*
__rust_thunk__8c6ca6d3__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5c_strEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __this) {
  return __this->c_str();
}

static_assert(
    (char32_t const* (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()
         const) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::c_str);

extern "C" void
__rust_thunk__fcac084d__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE13get_allocatorEv(
    class std::pmr::polymorphic_allocator<char32_t>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __this) {
  new (__return) auto(__this->get_allocator());
}

static_assert(
    (class std::pmr::polymorphic_allocator<char32_t> (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()
         const) &
    ::std::basic_string<
        char32_t, std::char_traits<char32_t>,
        std::pmr::polymorphic_allocator<char32_t>>::get_allocator);

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::pmr::polymorphic_allocator<char32_t>>*
__rust_thunk__0aa74968__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSEDi(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    char32_t __c) {
  return std::addressof(__this->operator=(__c));
}

static_assert(
    (class std::basic_string<char32_t, std::char_traits<char32_t>,
                             std::pmr::polymorphic_allocator<char32_t>> &
     (::std::basic_string<char32_t, std::char_traits<char32_t>,
                          std::pmr::polymorphic_allocator<char32_t>>::*)(
         char32_t)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::operator=);

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::pmr::polymorphic_allocator<char32_t>>*
__rust_thunk__0d1a1c84__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSERKS6_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>> const*
        __str) {
  return std::addressof(__this->operator=(*__str));
}

static_assert(
    (class std::basic_string<char32_t, std::char_traits<char32_t>,
                             std::pmr::polymorphic_allocator<char32_t>> &
     (::std::basic_string<char32_t, std::char_traits<char32_t>,
                          std::pmr::polymorphic_allocator<char32_t>>::*)(
         class std::basic_string<
             char32_t, std::char_traits<char32_t>,
             std::pmr::polymorphic_allocator<char32_t>> const&)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::operator=);

extern "C" void
__rust_thunk__8aaa45bc__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE9push_backEDi(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    char32_t __c) {
  __this->push_back(__c);
}

static_assert(
    (void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                               std::pmr::polymorphic_allocator<char32_t>>::*)(
        char32_t)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::push_back);

extern "C" void
__rust_thunk__abad84a3__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE8pop_backEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>*
        __this) {
  __this->pop_back();
}

static_assert(
    (void (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::pop_back);

extern "C" void
__rust_thunk__bd109b75__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5clearEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>*
        __this) {
  __this->clear();
}

static_assert(
    (void (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::clear);

extern "C" void
__rust_thunk__9f6bf568__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE13shrink_to_fitEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>*
        __this) {
  __this->shrink_to_fit();
}

static_assert(
    (void (
        ::std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>::*)()) &
    ::std::basic_string<
        char32_t, std::char_traits<char32_t>,
        std::pmr::polymorphic_allocator<char32_t>>::shrink_to_fit);

extern "C" void
__rust_thunk__e6e7c835__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE4swapERS6_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::pmr::polymorphic_allocator<char32_t>>* __str) {
  __this->swap(*__str);
}

static_assert(
    (void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                               std::pmr::polymorphic_allocator<char32_t>>::*)(
        class std::basic_string<char32_t, std::char_traits<char32_t>,
                                std::pmr::polymorphic_allocator<char32_t>>&)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::pmr::polymorphic_allocator<char32_t>>::swap);

static_assert(
    CRUBIT_SIZEOF(class std::basic_string<char32_t, std::char_traits<char32_t>,
                                          std::allocator<char32_t>>) == 24);
static_assert(
    alignof(class std::basic_string<char32_t, std::char_traits<char32_t>,
                                    std::allocator<char32_t>>) == 8);

extern "C" void
__rust_thunk__cc656414__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1Ev(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__6cef0e61__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__f3a6ff79__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __str) {
  crubit::construct_at(__this, *__str);
}

extern "C" void
__rust_thunk__5e32cc92__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_RKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __str,
    class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__str, *__a);
}

extern "C" void
__rust_thunk__b37f0b9f__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1EOS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __str) {
  crubit::construct_at(__this, std::move(*__str));
}

extern "C" void
__rust_thunk__975a6aa4__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1EOS5_RKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __str,
    class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, std::move(*__str), *__a);
}

extern "C" void
__rust_thunk__87f9f9cf__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_mmRKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __str,
    size_t __pos, size_t __n, class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__str, __pos, __n, *__a);
}

extern "C" void
__rust_thunk__b62a039f__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_mRKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __str,
    size_t __pos, class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, *__str, __pos, *__a);
}

extern "C" void
__rust_thunk__f18e20ba__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ESt16initializer_listIDiERKS4_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::initializer_list<char32_t>* __il,
    class std::allocator<char32_t> const* __a) {
  crubit::construct_at(__this, std::move(*__il), *__a);
}

extern "C" void
__rust_thunk__dee1b799__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEED1Ev(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::allocator<char32_t>>*
__rust_thunk__fc4f0119__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSEOS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __str) {
  return std::addressof(__this->operator=(std::move(*__str)));
}

static_assert((class std::basic_string<char32_t, std::char_traits<char32_t>,
                                       std::allocator<char32_t>> &
               (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                    std::allocator<char32_t>>::*)(
                   class std::basic_string<char32_t, std::char_traits<char32_t>,
                                           std::allocator<char32_t>>&&)) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::operator=);

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::allocator<char32_t>>*
__rust_thunk__e8738afb__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSESt16initializer_listIDiE(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::initializer_list<char32_t>* __il) {
  return std::addressof(__this->operator=(std::move(*__il)));
}

static_assert((class std::basic_string<char32_t, std::char_traits<char32_t>,
                                       std::allocator<char32_t>> &
               (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                    std::allocator<char32_t>>::*)(
                   class std::initializer_list<char32_t>)) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::operator=);

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::allocator<char32_t>>*
__rust_thunk__15d3c9ea__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSEPKDi(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    char32_t const* __s) {
  return std::addressof(__this->operator=(__s));
}

static_assert(
    (class std::basic_string<char32_t, std::char_traits<char32_t>,
                             std::allocator<char32_t>> &
     (::std::basic_string<char32_t, std::char_traits<char32_t>,
                          std::allocator<char32_t>>::*)(char32_t const*)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::allocator<char32_t>>::operator=);

extern "C" void
__rust_thunk__d4331a79__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE6cbeginEv(
    class std::__wrap_iter<const char32_t*>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  new (__return) auto(__this->cbegin());
}

static_assert((class std::__wrap_iter<const char32_t*> (
                  ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                      std::allocator<char32_t>>::*)() const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::cbegin);

extern "C" void
__rust_thunk__a36bce04__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4cendEv(
    class std::__wrap_iter<const char32_t*>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  new (__return) auto(__this->cend());
}

static_assert((class std::__wrap_iter<const char32_t*> (
                  ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                      std::allocator<char32_t>>::*)() const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::cend);

extern "C" void
__rust_thunk__7913a9b7__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE7crbeginEv(
    class std::reverse_iterator<std::__wrap_iter<const char32_t*>>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  new (__return) auto(__this->crbegin());
}

static_assert((class std::reverse_iterator<std::__wrap_iter<const char32_t*>> (
                  ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                      std::allocator<char32_t>>::*)() const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::crbegin);

extern "C" void
__rust_thunk__e750602f__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5crendEv(
    class std::reverse_iterator<std::__wrap_iter<const char32_t*>>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  new (__return) auto(__this->crend());
}

static_assert((class std::reverse_iterator<std::__wrap_iter<const char32_t*>> (
                  ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                      std::allocator<char32_t>>::*)() const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::crend);

extern "C" size_t
__rust_thunk__cadd6fcf__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4sizeEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                           std::allocator<char32_t>>::*)()
                   const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::size);

extern "C" size_t
__rust_thunk__fc78dfa5__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE6lengthEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  return __this->length();
}

static_assert((size_t (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                           std::allocator<char32_t>>::*)()
                   const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::length);

extern "C" size_t
__rust_thunk__071656e6__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE8max_sizeEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  return __this->max_size();
}

static_assert((size_t (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                           std::allocator<char32_t>>::*)()
                   const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::max_size);

extern "C" size_t
__rust_thunk__1eec9210__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE8capacityEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  return __this->capacity();
}

static_assert((size_t (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                           std::allocator<char32_t>>::*)()
                   const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::capacity);

extern "C" bool
__rust_thunk__95317e8a__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5emptyEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                         std::allocator<char32_t>>::*)()
                   const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::empty);

extern "C" char32_t const*
__rust_thunk__113d1be2__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEixEm(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this,
    size_t __pos) {
  return std::addressof(__this->operator[](__pos));
}

static_assert(
    (char32_t const& (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                          std::allocator<char32_t>>::*)(size_t)
         const) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::allocator<char32_t>>::operator[]);

extern "C" char32_t*
__rust_thunk__1d5276bd__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEixEm(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    size_t __pos) {
  return std::addressof(__this->operator[](__pos));
}

static_assert(
    (char32_t& (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                    std::allocator<char32_t>>::*)(size_t)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::allocator<char32_t>>::operator[]);

extern "C" void
__rust_thunk__9968d93d__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE6substrEmm(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this,
    size_t __pos, size_t __n) {
  new (__return) auto(__this->substr(__pos, __n));
}

static_assert((class std::basic_string<char32_t, std::char_traits<char32_t>,
                                       std::allocator<char32_t>> (
                  ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                      std::allocator<char32_t>>::*)(size_t,
                                                                    size_t)
                   const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::substr);

extern "C" char32_t const*
__rust_thunk__8c6ca6d3__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5c_strEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  return __this->c_str();
}

static_assert((char32_t const* (
                  ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                      std::allocator<char32_t>>::*)() const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::c_str);

extern "C" void
__rust_thunk__fcac084d__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE13get_allocatorEv(
    class std::allocator<char32_t>* __return,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this) {
  new (__return) auto(__this->get_allocator());
}

static_assert((class std::allocator<char32_t> (
                  ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                      std::allocator<char32_t>>::*)() const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::get_allocator);

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::allocator<char32_t>>*
__rust_thunk__0aa74968__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSEDi(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    char32_t __c) {
  return std::addressof(__this->operator=(__c));
}

static_assert((class std::basic_string<char32_t, std::char_traits<char32_t>,
                                       std::allocator<char32_t>> &
               (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                    std::allocator<char32_t>>::*)(char32_t)) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::operator=);

extern "C" class std::basic_string<char32_t, std::char_traits<char32_t>,
                                   std::allocator<char32_t>>*
__rust_thunk__0d1a1c84__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSERKS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __str) {
  return std::addressof(__this->operator=(*__str));
}

static_assert((class std::basic_string<char32_t, std::char_traits<char32_t>,
                                       std::allocator<char32_t>> &
               (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                    std::allocator<char32_t>>::*)(
                   class std::basic_string<char32_t, std::char_traits<char32_t>,
                                           std::allocator<char32_t>> const&)) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::operator=);

extern "C" void
__rust_thunk__8aaa45bc__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE9push_backEDi(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    char32_t __c) {
  __this->push_back(__c);
}

static_assert(
    (void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                               std::allocator<char32_t>>::*)(char32_t)) &
    ::std::basic_string<char32_t, std::char_traits<char32_t>,
                        std::allocator<char32_t>>::push_back);

extern "C" void
__rust_thunk__abad84a3__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE8pop_backEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this) {
  __this->pop_back();
}

static_assert((void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                         std::allocator<char32_t>>::*)()) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::pop_back);

extern "C" void
__rust_thunk__bd109b75__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5clearEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this) {
  __this->clear();
}

static_assert((void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                         std::allocator<char32_t>>::*)()) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::clear);

extern "C" void
__rust_thunk__9f6bf568__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE13shrink_to_fitEv(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this) {
  __this->shrink_to_fit();
}

static_assert((void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                         std::allocator<char32_t>>::*)()) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::shrink_to_fit);

extern "C" size_t
__rust_thunk__15bee269__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4copyEPDimm(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>> const* __this,
    char32_t* __s, size_t __n, size_t __pos) {
  return __this->copy(__s, __n, __pos);
}

static_assert((size_t (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                           std::allocator<char32_t>>::*)(
                  char32_t*, size_t, size_t) const) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::copy);

extern "C" void
__rust_thunk__e6e7c835__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4swapERS5_(
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __this,
    class std::basic_string<char32_t, std::char_traits<char32_t>,
                            std::allocator<char32_t>>* __str) {
  __this->swap(*__str);
}

static_assert((void (::std::basic_string<char32_t, std::char_traits<char32_t>,
                                         std::allocator<char32_t>>::*)(
                  class std::basic_string<char32_t, std::char_traits<char32_t>,
                                          std::allocator<char32_t>>&)) &
              ::std::basic_string<char32_t, std::char_traits<char32_t>,
                                  std::allocator<char32_t>>::swap);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string<char16_t, std::char_traits<char16_t>,
                                std::pmr::polymorphic_allocator<char16_t>>) ==
    32);
static_assert(
    alignof(
        class std::basic_string<char16_t, std::char_traits<char16_t>,
                                std::pmr::polymorphic_allocator<char16_t>>) ==
    8);

extern "C" void
__rust_thunk__cc656414__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1Ev(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__6cef0e61__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ERKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::pmr::polymorphic_allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__f3a6ff79__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ERKS6_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __str) {
  crubit::construct_at(__this, *__str);
}

extern "C" void
__rust_thunk__5e32cc92__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ERKS6_RKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __str,
    class std::pmr::polymorphic_allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__str, *__a);
}

extern "C" void
__rust_thunk__b37f0b9f__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1EOS6_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __str) {
  crubit::construct_at(__this, std::move(*__str));
}

extern "C" void
__rust_thunk__975a6aa4__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1EOS6_RKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __str,
    class std::pmr::polymorphic_allocator<char16_t> const* __a) {
  crubit::construct_at(__this, std::move(*__str), *__a);
}

extern "C" void
__rust_thunk__f18e20ba__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ESt16initializer_listIDsERKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::initializer_list<char16_t>* __il,
    class std::pmr::polymorphic_allocator<char16_t> const* __a) {
  crubit::construct_at(__this, std::move(*__il), *__a);
}

extern "C" void
__rust_thunk__dee1b799__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEED1Ev(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>*
        __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::pmr::polymorphic_allocator<char16_t>>*
__rust_thunk__fc4f0119__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSEOS6_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __str) {
  return std::addressof(__this->operator=(std::move(*__str)));
}

static_assert(
    (class std::basic_string<char16_t, std::char_traits<char16_t>,
                             std::pmr::polymorphic_allocator<char16_t>> &
     (::std::basic_string<char16_t, std::char_traits<char16_t>,
                          std::pmr::polymorphic_allocator<char16_t>>::*)(
         class std::basic_string<
             char16_t, std::char_traits<char16_t>,
             std::pmr::polymorphic_allocator<char16_t>>&&)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::operator=);

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::pmr::polymorphic_allocator<char16_t>>*
__rust_thunk__e8738afb__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSESt16initializer_listIDsE(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::initializer_list<char16_t>* __il) {
  return std::addressof(__this->operator=(std::move(*__il)));
}

static_assert(
    (class std::basic_string<char16_t, std::char_traits<char16_t>,
                             std::pmr::polymorphic_allocator<char16_t>> &
     (::std::basic_string<char16_t, std::char_traits<char16_t>,
                          std::pmr::polymorphic_allocator<char16_t>>::*)(
         class std::initializer_list<char16_t>)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::operator=);

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::pmr::polymorphic_allocator<char16_t>>*
__rust_thunk__15d3c9ea__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSEPKDs(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    char16_t const* __s) {
  return std::addressof(__this->operator=(__s));
}

static_assert(
    (class std::basic_string<char16_t, std::char_traits<char16_t>,
                             std::pmr::polymorphic_allocator<char16_t>> &
     (::std::basic_string<char16_t, std::char_traits<char16_t>,
                          std::pmr::polymorphic_allocator<char16_t>>::*)(
         char16_t const*)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::operator=);

extern "C" void
__rust_thunk__d4331a79__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE6cbeginEv(
    class std::__wrap_iter<const char16_t*>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __this) {
  new (__return) auto(__this->cbegin());
}

static_assert(
    (class std::__wrap_iter<const char16_t*> (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()
         const) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::cbegin);

extern "C" void
__rust_thunk__a36bce04__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE4cendEv(
    class std::__wrap_iter<const char16_t*>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __this) {
  new (__return) auto(__this->cend());
}

static_assert(
    (class std::__wrap_iter<const char16_t*> (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()
         const) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::cend);

extern "C" void
__rust_thunk__7913a9b7__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE7crbeginEv(
    class std::reverse_iterator<std::__wrap_iter<const char16_t*>>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __this) {
  new (__return) auto(__this->crbegin());
}

static_assert(
    (class std::reverse_iterator<std::__wrap_iter<const char16_t*>> (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()
         const) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::crbegin);

extern "C" void
__rust_thunk__e750602f__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5crendEv(
    class std::reverse_iterator<std::__wrap_iter<const char16_t*>>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __this) {
  new (__return) auto(__this->crend());
}

static_assert(
    (class std::reverse_iterator<std::__wrap_iter<const char16_t*>> (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()
         const) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::crend);

extern "C" void
__rust_thunk__df2538bf__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE7reserveEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>*
        __this) {
  __this->reserve();
}

static_assert(
    (void (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::reserve);

extern "C" bool
__rust_thunk__95317e8a__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5emptyEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string<char16_t, std::char_traits<char16_t>,
                               std::pmr::polymorphic_allocator<char16_t>>::*)()
         const) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::empty);

extern "C" char16_t const*
__rust_thunk__8c6ca6d3__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5c_strEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __this) {
  return __this->c_str();
}

static_assert(
    (char16_t const* (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()
         const) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::c_str);

extern "C" void
__rust_thunk__fcac084d__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE13get_allocatorEv(
    class std::pmr::polymorphic_allocator<char16_t>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __this) {
  new (__return) auto(__this->get_allocator());
}

static_assert(
    (class std::pmr::polymorphic_allocator<char16_t> (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()
         const) &
    ::std::basic_string<
        char16_t, std::char_traits<char16_t>,
        std::pmr::polymorphic_allocator<char16_t>>::get_allocator);

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::pmr::polymorphic_allocator<char16_t>>*
__rust_thunk__0aa74968__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSEDs(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    char16_t __c) {
  return std::addressof(__this->operator=(__c));
}

static_assert(
    (class std::basic_string<char16_t, std::char_traits<char16_t>,
                             std::pmr::polymorphic_allocator<char16_t>> &
     (::std::basic_string<char16_t, std::char_traits<char16_t>,
                          std::pmr::polymorphic_allocator<char16_t>>::*)(
         char16_t)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::operator=);

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::pmr::polymorphic_allocator<char16_t>>*
__rust_thunk__0d1a1c84__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSERKS6_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>> const*
        __str) {
  return std::addressof(__this->operator=(*__str));
}

static_assert(
    (class std::basic_string<char16_t, std::char_traits<char16_t>,
                             std::pmr::polymorphic_allocator<char16_t>> &
     (::std::basic_string<char16_t, std::char_traits<char16_t>,
                          std::pmr::polymorphic_allocator<char16_t>>::*)(
         class std::basic_string<
             char16_t, std::char_traits<char16_t>,
             std::pmr::polymorphic_allocator<char16_t>> const&)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::operator=);

extern "C" void
__rust_thunk__8aaa45bc__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE9push_backEDs(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    char16_t __c) {
  __this->push_back(__c);
}

static_assert(
    (void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                               std::pmr::polymorphic_allocator<char16_t>>::*)(
        char16_t)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::push_back);

extern "C" void
__rust_thunk__abad84a3__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE8pop_backEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>*
        __this) {
  __this->pop_back();
}

static_assert(
    (void (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::pop_back);

extern "C" void
__rust_thunk__bd109b75__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5clearEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>*
        __this) {
  __this->clear();
}

static_assert(
    (void (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::clear);

extern "C" void
__rust_thunk__9f6bf568__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE13shrink_to_fitEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>*
        __this) {
  __this->shrink_to_fit();
}

static_assert(
    (void (
        ::std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>::*)()) &
    ::std::basic_string<
        char16_t, std::char_traits<char16_t>,
        std::pmr::polymorphic_allocator<char16_t>>::shrink_to_fit);

extern "C" void
__rust_thunk__e6e7c835__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE4swapERS6_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::pmr::polymorphic_allocator<char16_t>>* __str) {
  __this->swap(*__str);
}

static_assert(
    (void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                               std::pmr::polymorphic_allocator<char16_t>>::*)(
        class std::basic_string<char16_t, std::char_traits<char16_t>,
                                std::pmr::polymorphic_allocator<char16_t>>&)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::pmr::polymorphic_allocator<char16_t>>::swap);

static_assert(
    CRUBIT_SIZEOF(class std::basic_string<char16_t, std::char_traits<char16_t>,
                                          std::allocator<char16_t>>) == 24);
static_assert(
    alignof(class std::basic_string<char16_t, std::char_traits<char16_t>,
                                    std::allocator<char16_t>>) == 8);

extern "C" void
__rust_thunk__cc656414__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1Ev(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__6cef0e61__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__f3a6ff79__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __str) {
  crubit::construct_at(__this, *__str);
}

extern "C" void
__rust_thunk__5e32cc92__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_RKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __str,
    class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__str, *__a);
}

extern "C" void
__rust_thunk__b37f0b9f__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1EOS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __str) {
  crubit::construct_at(__this, std::move(*__str));
}

extern "C" void
__rust_thunk__975a6aa4__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1EOS5_RKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __str,
    class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, std::move(*__str), *__a);
}

extern "C" void
__rust_thunk__87f9f9cf__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_mmRKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __str,
    size_t __pos, size_t __n, class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__str, __pos, __n, *__a);
}

extern "C" void
__rust_thunk__b62a039f__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_mRKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __str,
    size_t __pos, class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, *__str, __pos, *__a);
}

extern "C" void
__rust_thunk__f18e20ba__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ESt16initializer_listIDsERKS4_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::initializer_list<char16_t>* __il,
    class std::allocator<char16_t> const* __a) {
  crubit::construct_at(__this, std::move(*__il), *__a);
}

extern "C" void
__rust_thunk__dee1b799__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEED1Ev(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::allocator<char16_t>>*
__rust_thunk__fc4f0119__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSEOS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __str) {
  return std::addressof(__this->operator=(std::move(*__str)));
}

static_assert((class std::basic_string<char16_t, std::char_traits<char16_t>,
                                       std::allocator<char16_t>> &
               (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                    std::allocator<char16_t>>::*)(
                   class std::basic_string<char16_t, std::char_traits<char16_t>,
                                           std::allocator<char16_t>>&&)) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::operator=);

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::allocator<char16_t>>*
__rust_thunk__e8738afb__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSESt16initializer_listIDsE(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::initializer_list<char16_t>* __il) {
  return std::addressof(__this->operator=(std::move(*__il)));
}

static_assert((class std::basic_string<char16_t, std::char_traits<char16_t>,
                                       std::allocator<char16_t>> &
               (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                    std::allocator<char16_t>>::*)(
                   class std::initializer_list<char16_t>)) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::operator=);

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::allocator<char16_t>>*
__rust_thunk__15d3c9ea__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSEPKDs(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    char16_t const* __s) {
  return std::addressof(__this->operator=(__s));
}

static_assert(
    (class std::basic_string<char16_t, std::char_traits<char16_t>,
                             std::allocator<char16_t>> &
     (::std::basic_string<char16_t, std::char_traits<char16_t>,
                          std::allocator<char16_t>>::*)(char16_t const*)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::allocator<char16_t>>::operator=);

extern "C" void
__rust_thunk__d4331a79__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE6cbeginEv(
    class std::__wrap_iter<const char16_t*>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  new (__return) auto(__this->cbegin());
}

static_assert((class std::__wrap_iter<const char16_t*> (
                  ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                      std::allocator<char16_t>>::*)() const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::cbegin);

extern "C" void
__rust_thunk__a36bce04__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4cendEv(
    class std::__wrap_iter<const char16_t*>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  new (__return) auto(__this->cend());
}

static_assert((class std::__wrap_iter<const char16_t*> (
                  ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                      std::allocator<char16_t>>::*)() const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::cend);

extern "C" void
__rust_thunk__7913a9b7__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE7crbeginEv(
    class std::reverse_iterator<std::__wrap_iter<const char16_t*>>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  new (__return) auto(__this->crbegin());
}

static_assert((class std::reverse_iterator<std::__wrap_iter<const char16_t*>> (
                  ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                      std::allocator<char16_t>>::*)() const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::crbegin);

extern "C" void
__rust_thunk__e750602f__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5crendEv(
    class std::reverse_iterator<std::__wrap_iter<const char16_t*>>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  new (__return) auto(__this->crend());
}

static_assert((class std::reverse_iterator<std::__wrap_iter<const char16_t*>> (
                  ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                      std::allocator<char16_t>>::*)() const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::crend);

extern "C" size_t
__rust_thunk__cadd6fcf__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4sizeEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  return __this->size();
}

static_assert((size_t (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                           std::allocator<char16_t>>::*)()
                   const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::size);

extern "C" size_t
__rust_thunk__fc78dfa5__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE6lengthEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  return __this->length();
}

static_assert((size_t (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                           std::allocator<char16_t>>::*)()
                   const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::length);

extern "C" size_t
__rust_thunk__071656e6__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE8max_sizeEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  return __this->max_size();
}

static_assert((size_t (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                           std::allocator<char16_t>>::*)()
                   const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::max_size);

extern "C" size_t
__rust_thunk__1eec9210__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE8capacityEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  return __this->capacity();
}

static_assert((size_t (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                           std::allocator<char16_t>>::*)()
                   const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::capacity);

extern "C" bool
__rust_thunk__95317e8a__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5emptyEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                         std::allocator<char16_t>>::*)()
                   const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::empty);

extern "C" char16_t const*
__rust_thunk__113d1be2__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEixEm(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this,
    size_t __pos) {
  return std::addressof(__this->operator[](__pos));
}

static_assert(
    (char16_t const& (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                          std::allocator<char16_t>>::*)(size_t)
         const) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::allocator<char16_t>>::operator[]);

extern "C" char16_t*
__rust_thunk__1d5276bd__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEixEm(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    size_t __pos) {
  return std::addressof(__this->operator[](__pos));
}

static_assert(
    (char16_t& (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                    std::allocator<char16_t>>::*)(size_t)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::allocator<char16_t>>::operator[]);

extern "C" void
__rust_thunk__9968d93d__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE6substrEmm(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this,
    size_t __pos, size_t __n) {
  new (__return) auto(__this->substr(__pos, __n));
}

static_assert((class std::basic_string<char16_t, std::char_traits<char16_t>,
                                       std::allocator<char16_t>> (
                  ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                      std::allocator<char16_t>>::*)(size_t,
                                                                    size_t)
                   const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::substr);

extern "C" char16_t const*
__rust_thunk__8c6ca6d3__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5c_strEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  return __this->c_str();
}

static_assert((char16_t const* (
                  ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                      std::allocator<char16_t>>::*)() const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::c_str);

extern "C" void
__rust_thunk__fcac084d__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE13get_allocatorEv(
    class std::allocator<char16_t>* __return,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this) {
  new (__return) auto(__this->get_allocator());
}

static_assert((class std::allocator<char16_t> (
                  ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                      std::allocator<char16_t>>::*)() const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::get_allocator);

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::allocator<char16_t>>*
__rust_thunk__0aa74968__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSEDs(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    char16_t __c) {
  return std::addressof(__this->operator=(__c));
}

static_assert((class std::basic_string<char16_t, std::char_traits<char16_t>,
                                       std::allocator<char16_t>> &
               (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                    std::allocator<char16_t>>::*)(char16_t)) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::operator=);

extern "C" class std::basic_string<char16_t, std::char_traits<char16_t>,
                                   std::allocator<char16_t>>*
__rust_thunk__0d1a1c84__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSERKS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __str) {
  return std::addressof(__this->operator=(*__str));
}

static_assert((class std::basic_string<char16_t, std::char_traits<char16_t>,
                                       std::allocator<char16_t>> &
               (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                    std::allocator<char16_t>>::*)(
                   class std::basic_string<char16_t, std::char_traits<char16_t>,
                                           std::allocator<char16_t>> const&)) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::operator=);

extern "C" void
__rust_thunk__8aaa45bc__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE9push_backEDs(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    char16_t __c) {
  __this->push_back(__c);
}

static_assert(
    (void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                               std::allocator<char16_t>>::*)(char16_t)) &
    ::std::basic_string<char16_t, std::char_traits<char16_t>,
                        std::allocator<char16_t>>::push_back);

extern "C" void
__rust_thunk__abad84a3__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE8pop_backEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this) {
  __this->pop_back();
}

static_assert((void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                         std::allocator<char16_t>>::*)()) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::pop_back);

extern "C" void
__rust_thunk__bd109b75__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5clearEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this) {
  __this->clear();
}

static_assert((void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                         std::allocator<char16_t>>::*)()) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::clear);

extern "C" void
__rust_thunk__9f6bf568__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE13shrink_to_fitEv(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this) {
  __this->shrink_to_fit();
}

static_assert((void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                         std::allocator<char16_t>>::*)()) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::shrink_to_fit);

extern "C" size_t
__rust_thunk__15bee269__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4copyEPDsmm(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>> const* __this,
    char16_t* __s, size_t __n, size_t __pos) {
  return __this->copy(__s, __n, __pos);
}

static_assert((size_t (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                           std::allocator<char16_t>>::*)(
                  char16_t*, size_t, size_t) const) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::copy);

extern "C" void
__rust_thunk__e6e7c835__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4swapERS5_(
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __this,
    class std::basic_string<char16_t, std::char_traits<char16_t>,
                            std::allocator<char16_t>>* __str) {
  __this->swap(*__str);
}

static_assert((void (::std::basic_string<char16_t, std::char_traits<char16_t>,
                                         std::allocator<char16_t>>::*)(
                  class std::basic_string<char16_t, std::char_traits<char16_t>,
                                          std::allocator<char16_t>>&)) &
              ::std::basic_string<char16_t, std::char_traits<char16_t>,
                                  std::allocator<char16_t>>::swap);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string<char, std::char_traits<char>,
                                std::pmr::polymorphic_allocator<char>>) == 32);
static_assert(
    alignof(class std::basic_string<char, std::char_traits<char>,
                                    std::pmr::polymorphic_allocator<char>>) ==
    8);

extern "C" void
__rust_thunk__cc656414__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1Ev(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__6cef0e61__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ERKS5_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::pmr::polymorphic_allocator<char> const* __a) {
  crubit::construct_at(__this, *__a);
}

extern "C" void
__rust_thunk__f3a6ff79__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ERKS6_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __str) {
  crubit::construct_at(__this, *__str);
}

extern "C" void
__rust_thunk__5e32cc92__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ERKS6_RKS5_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const* __str,
    class std::pmr::polymorphic_allocator<char> const* __a) {
  crubit::construct_at(__this, *__str, *__a);
}

extern "C" void
__rust_thunk__b37f0b9f__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1EOS6_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __str) {
  crubit::construct_at(__this, std::move(*__str));
}

extern "C" void
__rust_thunk__975a6aa4__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1EOS6_RKS5_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __str,
    class std::pmr::polymorphic_allocator<char> const* __a) {
  crubit::construct_at(__this, std::move(*__str), *__a);
}

extern "C" void
__rust_thunk__f18e20ba__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ESt16initializer_listIcERKS5_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::initializer_list<char>* __il,
    class std::pmr::polymorphic_allocator<char> const* __a) {
  crubit::construct_at(__this, std::move(*__il), *__a);
}

extern "C" void
__rust_thunk__dee1b799__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEED1Ev(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this) {
  std::destroy_at(__this);
}

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::pmr::polymorphic_allocator<char>>*
__rust_thunk__fc4f0119__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSEOS6_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __str) {
  return std::addressof(__this->operator=(std::move(*__str)));
}

static_assert(
    (class std::basic_string<char, std::char_traits<char>,
                             std::pmr::polymorphic_allocator<char>> &
     (::std::basic_string<char, std::char_traits<char>,
                          std::pmr::polymorphic_allocator<char>>::*)(
         class std::basic_string<char, std::char_traits<char>,
                                 std::pmr::polymorphic_allocator<char>>&&)) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::operator=);

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::pmr::polymorphic_allocator<char>>*
__rust_thunk__e8738afb__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSESt16initializer_listIcE(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::initializer_list<char>* __il) {
  return std::addressof(__this->operator=(std::move(*__il)));
}

static_assert(
    (class std::basic_string<char, std::char_traits<char>,
                             std::pmr::polymorphic_allocator<char>> &
     (::std::basic_string<char, std::char_traits<char>,
                          std::pmr::polymorphic_allocator<char>>::*)(
         class std::initializer_list<char>)) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::operator=);

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::pmr::polymorphic_allocator<char>>*
__rust_thunk__15d3c9ea__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSEPKc(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    char const* __s) {
  return std::addressof(__this->operator=(__s));
}

static_assert(
    (class std::basic_string<char, std::char_traits<char>,
                             std::pmr::polymorphic_allocator<char>> &
     (::std::basic_string<char, std::char_traits<char>,
                          std::pmr::polymorphic_allocator<char>>::*)(
         char const*)) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::operator=);

extern "C" void
__rust_thunk__d4331a79__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE6cbeginEv(
    class std::__wrap_iter<const char*>* __return,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __this) {
  new (__return) auto(__this->cbegin());
}

static_assert(
    (class std::__wrap_iter<const char*> (
        ::std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>::*)()
         const) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::cbegin);

extern "C" void
__rust_thunk__a36bce04__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE4cendEv(
    class std::__wrap_iter<const char*>* __return,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __this) {
  new (__return) auto(__this->cend());
}

static_assert(
    (class std::__wrap_iter<const char*> (
        ::std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>::*)()
         const) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::cend);

extern "C" void
__rust_thunk__7913a9b7__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE7crbeginEv(
    class std::reverse_iterator<std::__wrap_iter<const char*>>* __return,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __this) {
  new (__return) auto(__this->crbegin());
}

static_assert(
    (class std::reverse_iterator<std::__wrap_iter<const char*>> (
        ::std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>::*)()
         const) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::crbegin);

extern "C" void
__rust_thunk__e750602f__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5crendEv(
    class std::reverse_iterator<std::__wrap_iter<const char*>>* __return,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __this) {
  new (__return) auto(__this->crend());
}

static_assert(
    (class std::reverse_iterator<std::__wrap_iter<const char*>> (
        ::std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>::*)()
         const) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::crend);

extern "C" void
__rust_thunk__df2538bf__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE7reserveEv(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this) {
  __this->reserve();
}

static_assert(
    (void (::std::basic_string<char, std::char_traits<char>,
                               std::pmr::polymorphic_allocator<char>>::*)()) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::reserve);

extern "C" bool
__rust_thunk__95317e8a__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5emptyEv(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string<char, std::char_traits<char>,
                               std::pmr::polymorphic_allocator<char>>::*)()
         const) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::empty);

extern "C" char const*
__rust_thunk__8c6ca6d3__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5c_strEv(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __this) {
  return __this->c_str();
}

static_assert(
    (char const* (
        ::std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>::*)()
         const) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::c_str);

extern "C" void
__rust_thunk__fcac084d__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE13get_allocatorEv(
    class std::pmr::polymorphic_allocator<char>* __return,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __this) {
  new (__return) auto(__this->get_allocator());
}

static_assert(
    (class std::pmr::polymorphic_allocator<char> (
        ::std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>::*)()
         const) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::get_allocator);

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::pmr::polymorphic_allocator<char>>*
__rust_thunk__0aa74968__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSEc(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    char __c) {
  return std::addressof(__this->operator=(__c));
}

static_assert(
    (class std::basic_string<char, std::char_traits<char>,
                             std::pmr::polymorphic_allocator<char>> &
     (::std::basic_string<char, std::char_traits<char>,
                          std::pmr::polymorphic_allocator<char>>::*)(char)) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::operator=);

extern "C" class std::basic_string<char, std::char_traits<char>,
                                   std::pmr::polymorphic_allocator<char>>*
__rust_thunk__0d1a1c84__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSERKS6_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>> const*
        __str) {
  return std::addressof(__this->operator=(*__str));
}

static_assert(
    (class std::basic_string<char, std::char_traits<char>,
                             std::pmr::polymorphic_allocator<char>> &
     (::std::basic_string<char, std::char_traits<char>,
                          std::pmr::polymorphic_allocator<char>>::*)(
         class std::basic_string<
             char, std::char_traits<char>,
             std::pmr::polymorphic_allocator<char>> const&)) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::operator=);

extern "C" void
__rust_thunk__8aaa45bc__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE9push_backEc(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    char __c) {
  __this->push_back(__c);
}

static_assert(
    (void (::std::basic_string<char, std::char_traits<char>,
                               std::pmr::polymorphic_allocator<char>>::*)(
        char)) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::push_back);

extern "C" void
__rust_thunk__abad84a3__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE8pop_backEv(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this) {
  __this->pop_back();
}

static_assert(
    (void (::std::basic_string<char, std::char_traits<char>,
                               std::pmr::polymorphic_allocator<char>>::*)()) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::pop_back);

extern "C" void
__rust_thunk__bd109b75__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5clearEv(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this) {
  __this->clear();
}

static_assert(
    (void (::std::basic_string<char, std::char_traits<char>,
                               std::pmr::polymorphic_allocator<char>>::*)()) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::clear);

extern "C" void
__rust_thunk__9f6bf568__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE13shrink_to_fitEv(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this) {
  __this->shrink_to_fit();
}

static_assert(
    (void (::std::basic_string<char, std::char_traits<char>,
                               std::pmr::polymorphic_allocator<char>>::*)()) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::shrink_to_fit);

extern "C" void
__rust_thunk__e6e7c835__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE4swapERS6_(
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __this,
    class std::basic_string<char, std::char_traits<char>,
                            std::pmr::polymorphic_allocator<char>>* __str) {
  __this->swap(*__str);
}

static_assert(
    (void (::std::basic_string<char, std::char_traits<char>,
                               std::pmr::polymorphic_allocator<char>>::*)(
        class std::basic_string<char, std::char_traits<char>,
                                std::pmr::polymorphic_allocator<char>>&)) &
    ::std::basic_string<char, std::char_traits<char>,
                        std::pmr::polymorphic_allocator<char>>::swap);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<std::__wrap_iter<char32_t*>>) ==
    8);
static_assert(
    alignof(class std::reverse_iterator<std::__wrap_iter<char32_t*>>) == 8);

extern "C" void
__rust_thunk__ecf10b48__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<char32_t*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__5de499b5__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEC1ES3_(
    class std::reverse_iterator<std::__wrap_iter<char32_t*>>* __this,
    class std::__wrap_iter<char32_t*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__3ae3b005__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEE4baseEv(
    class std::__wrap_iter<char32_t*>* __return,
    class std::reverse_iterator<std::__wrap_iter<char32_t*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert((class std::__wrap_iter<char32_t*> (
                  ::std::reverse_iterator<std::__wrap_iter<char32_t*>>::*)()
                   const) &
              ::std::reverse_iterator<std::__wrap_iter<char32_t*>>::base);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<std::__wrap_iter<char16_t*>>) ==
    8);
static_assert(
    alignof(class std::reverse_iterator<std::__wrap_iter<char16_t*>>) == 8);

extern "C" void
__rust_thunk__ecf10b48__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<char16_t*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__5de499b5__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEC1ES3_(
    class std::reverse_iterator<std::__wrap_iter<char16_t*>>* __this,
    class std::__wrap_iter<char16_t*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__3ae3b005__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEE4baseEv(
    class std::__wrap_iter<char16_t*>* __return,
    class std::reverse_iterator<std::__wrap_iter<char16_t*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert((class std::__wrap_iter<char16_t*> (
                  ::std::reverse_iterator<std::__wrap_iter<char16_t*>>::*)()
                   const) &
              ::std::reverse_iterator<std::__wrap_iter<char16_t*>>::base);

static_assert(
    CRUBIT_SIZEOF(
        class std::reverse_iterator<std::__wrap_iter<const char32_t*>>) == 8);
static_assert(
    alignof(class std::reverse_iterator<std::__wrap_iter<const char32_t*>>) ==
    8);

extern "C" void
__rust_thunk__ecf10b48__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<const char32_t*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__5de499b5__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEC1ES4_(
    class std::reverse_iterator<std::__wrap_iter<const char32_t*>>* __this,
    class std::__wrap_iter<const char32_t*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__3ae3b005__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEE4baseEv(
    class std::__wrap_iter<const char32_t*>* __return,
    class std::reverse_iterator<std::__wrap_iter<const char32_t*>> const*
        __this) {
  new (__return) auto(__this->base());
}

static_assert(
    (class std::__wrap_iter<const char32_t*> (
        ::std::reverse_iterator<std::__wrap_iter<const char32_t*>>::*)()
         const) &
    ::std::reverse_iterator<std::__wrap_iter<const char32_t*>>::base);

static_assert(
    CRUBIT_SIZEOF(
        class std::reverse_iterator<std::__wrap_iter<const char16_t*>>) == 8);
static_assert(
    alignof(class std::reverse_iterator<std::__wrap_iter<const char16_t*>>) ==
    8);

extern "C" void
__rust_thunk__ecf10b48__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<const char16_t*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__5de499b5__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEC1ES4_(
    class std::reverse_iterator<std::__wrap_iter<const char16_t*>>* __this,
    class std::__wrap_iter<const char16_t*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__3ae3b005__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEE4baseEv(
    class std::__wrap_iter<const char16_t*>* __return,
    class std::reverse_iterator<std::__wrap_iter<const char16_t*>> const*
        __this) {
  new (__return) auto(__this->base());
}

static_assert(
    (class std::__wrap_iter<const char16_t*> (
        ::std::reverse_iterator<std::__wrap_iter<const char16_t*>>::*)()
         const) &
    ::std::reverse_iterator<std::__wrap_iter<const char16_t*>>::base);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<std::__wrap_iter<const char*>>) ==
    8);
static_assert(
    alignof(class std::reverse_iterator<std::__wrap_iter<const char*>>) == 8);

extern "C" void
__rust_thunk__ecf10b48__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<const char*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__5de499b5__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEC1ES4_(
    class std::reverse_iterator<std::__wrap_iter<const char*>>* __this,
    class std::__wrap_iter<const char*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__3ae3b005__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEE4baseEv(
    class std::__wrap_iter<const char*>* __return,
    class std::reverse_iterator<std::__wrap_iter<const char*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert((class std::__wrap_iter<const char*> (
                  ::std::reverse_iterator<std::__wrap_iter<const char*>>::*)()
                   const) &
              ::std::reverse_iterator<std::__wrap_iter<const char*>>::base);

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<std::__wrap_iter<char*>>) == 8);
static_assert(alignof(class std::reverse_iterator<std::__wrap_iter<char*>>) ==
              8);

extern "C" void
__rust_thunk__ecf10b48__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEC1Ev(
    class std::reverse_iterator<std::__wrap_iter<char*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__5de499b5__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEC1ES3_(
    class std::reverse_iterator<std::__wrap_iter<char*>>* __this,
    class std::__wrap_iter<char*>* __x) {
  crubit::construct_at(__this, std::move(*__x));
}

extern "C" void
__rust_thunk__3ae3b005__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEE4baseEv(
    class std::__wrap_iter<char*>* __return,
    class std::reverse_iterator<std::__wrap_iter<char*>> const* __this) {
  new (__return) auto(__this->base());
}

static_assert((class std::__wrap_iter<char*> (
                  ::std::reverse_iterator<std::__wrap_iter<char*>>::*)()
                   const) &
              ::std::reverse_iterator<std::__wrap_iter<char*>>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char32_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char32_t*>) == 8);

extern "C" void __rust_thunk__ecf10b48__ZNSt3__u16reverse_iteratorIPKDiEC1Ev(
    class std::reverse_iterator<const char32_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__5de499b5__ZNSt3__u16reverse_iteratorIPKDiEC1ES2_(
    class std::reverse_iterator<const char32_t*>* __this, char32_t const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" char32_t const*
__rust_thunk__3ae3b005__ZNKSt3__u16reverse_iteratorIPKDiE4baseEv(
    class std::reverse_iterator<const char32_t*> const* __this) {
  return __this->base();
}

static_assert((char32_t const* (::std::reverse_iterator<const char32_t*>::*)()
                   const) &
              ::std::reverse_iterator<const char32_t*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char16_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char16_t*>) == 8);

extern "C" void __rust_thunk__ecf10b48__ZNSt3__u16reverse_iteratorIPKDsEC1Ev(
    class std::reverse_iterator<const char16_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__5de499b5__ZNSt3__u16reverse_iteratorIPKDsEC1ES2_(
    class std::reverse_iterator<const char16_t*>* __this, char16_t const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" char16_t const*
__rust_thunk__3ae3b005__ZNKSt3__u16reverse_iteratorIPKDsE4baseEv(
    class std::reverse_iterator<const char16_t*> const* __this) {
  return __this->base();
}

static_assert((char16_t const* (::std::reverse_iterator<const char16_t*>::*)()
                   const) &
              ::std::reverse_iterator<const char16_t*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char8_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char8_t*>) == 8);

extern "C" void __rust_thunk__ecf10b48__ZNSt3__u16reverse_iteratorIPKDuEC1Ev(
    class std::reverse_iterator<const char8_t*>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const char*>) == 8);
static_assert(alignof(class std::reverse_iterator<const char*>) == 8);

extern "C" void __rust_thunk__ecf10b48__ZNSt3__u16reverse_iteratorIPKcEC1Ev(
    class std::reverse_iterator<const char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__5de499b5__ZNSt3__u16reverse_iteratorIPKcEC1ES2_(
    class std::reverse_iterator<const char*>* __this, char const* __x) {
  crubit::construct_at(__this, __x);
}

extern "C" char const*
__rust_thunk__3ae3b005__ZNKSt3__u16reverse_iteratorIPKcE4baseEv(
    class std::reverse_iterator<const char*> const* __this) {
  return __this->base();
}

static_assert((char const* (::std::reverse_iterator<const char*>::*)() const) &
              ::std::reverse_iterator<const char*>::base);

static_assert(CRUBIT_SIZEOF(class std::reverse_iterator<const wchar_t*>) == 8);
static_assert(alignof(class std::reverse_iterator<const wchar_t*>) == 8);

extern "C" void __rust_thunk__ecf10b48__ZNSt3__u16reverse_iteratorIPKwEC1Ev(
    class std::reverse_iterator<const wchar_t*>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<char32_t*>) == 8);
static_assert(alignof(class std::__wrap_iter<char32_t*>) == 8);

extern "C" void __rust_thunk__da1abd55__ZNSt3__u11__wrap_iterIPDiEC1Ev(
    class std::__wrap_iter<char32_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__b874af53__ZNKSt3__u11__wrap_iterIPDiEplEl(
    class std::__wrap_iter<char32_t*>* __return,
    class std::__wrap_iter<char32_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<char32_t*> (
                  ::std::__wrap_iter<char32_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<char32_t*>::operator+);

extern "C" class std::__wrap_iter<char32_t*>*
__rust_thunk__7995cc76__ZNSt3__u11__wrap_iterIPDiEpLEl(
    class std::__wrap_iter<char32_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<char32_t*> &
               (::std::__wrap_iter<char32_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char32_t*>::operator+=);

extern "C" void __rust_thunk__69b7f5af__ZNKSt3__u11__wrap_iterIPDiEmiEl(
    class std::__wrap_iter<char32_t*>* __return,
    class std::__wrap_iter<char32_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<char32_t*> (
                  ::std::__wrap_iter<char32_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<char32_t*>::operator-);

extern "C" class std::__wrap_iter<char32_t*>*
__rust_thunk__a05c7a73__ZNSt3__u11__wrap_iterIPDiEmIEl(
    class std::__wrap_iter<char32_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<char32_t*> &
               (::std::__wrap_iter<char32_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char32_t*>::operator-=);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<char16_t*>) == 8);
static_assert(alignof(class std::__wrap_iter<char16_t*>) == 8);

extern "C" void __rust_thunk__da1abd55__ZNSt3__u11__wrap_iterIPDsEC1Ev(
    class std::__wrap_iter<char16_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__b874af53__ZNKSt3__u11__wrap_iterIPDsEplEl(
    class std::__wrap_iter<char16_t*>* __return,
    class std::__wrap_iter<char16_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<char16_t*> (
                  ::std::__wrap_iter<char16_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<char16_t*>::operator+);

extern "C" class std::__wrap_iter<char16_t*>*
__rust_thunk__7995cc76__ZNSt3__u11__wrap_iterIPDsEpLEl(
    class std::__wrap_iter<char16_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<char16_t*> &
               (::std::__wrap_iter<char16_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char16_t*>::operator+=);

extern "C" void __rust_thunk__69b7f5af__ZNKSt3__u11__wrap_iterIPDsEmiEl(
    class std::__wrap_iter<char16_t*>* __return,
    class std::__wrap_iter<char16_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<char16_t*> (
                  ::std::__wrap_iter<char16_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<char16_t*>::operator-);

extern "C" class std::__wrap_iter<char16_t*>*
__rust_thunk__a05c7a73__ZNSt3__u11__wrap_iterIPDsEmIEl(
    class std::__wrap_iter<char16_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<char16_t*> &
               (::std::__wrap_iter<char16_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char16_t*>::operator-=);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<const char32_t*>) == 8);
static_assert(alignof(class std::__wrap_iter<const char32_t*>) == 8);

extern "C" void __rust_thunk__da1abd55__ZNSt3__u11__wrap_iterIPKDiEC1Ev(
    class std::__wrap_iter<const char32_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__b874af53__ZNKSt3__u11__wrap_iterIPKDiEplEl(
    class std::__wrap_iter<const char32_t*>* __return,
    class std::__wrap_iter<const char32_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<const char32_t*> (
                  ::std::__wrap_iter<const char32_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char32_t*>::operator+);

extern "C" class std::__wrap_iter<const char32_t*>*
__rust_thunk__7995cc76__ZNSt3__u11__wrap_iterIPKDiEpLEl(
    class std::__wrap_iter<const char32_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<const char32_t*> &
               (::std::__wrap_iter<const char32_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char32_t*>::operator+=);

extern "C" void __rust_thunk__69b7f5af__ZNKSt3__u11__wrap_iterIPKDiEmiEl(
    class std::__wrap_iter<const char32_t*>* __return,
    class std::__wrap_iter<const char32_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<const char32_t*> (
                  ::std::__wrap_iter<const char32_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char32_t*>::operator-);

extern "C" class std::__wrap_iter<const char32_t*>*
__rust_thunk__a05c7a73__ZNSt3__u11__wrap_iterIPKDiEmIEl(
    class std::__wrap_iter<const char32_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<const char32_t*> &
               (::std::__wrap_iter<const char32_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char32_t*>::operator-=);

extern "C" char32_t const*
__rust_thunk__09368604__ZNKSt3__u11__wrap_iterIPKDiEixEl(
    class std::__wrap_iter<const char32_t*> const* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((char32_t const& (::std::__wrap_iter<const char32_t*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<const char32_t*>::operator[]);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<const char16_t*>) == 8);
static_assert(alignof(class std::__wrap_iter<const char16_t*>) == 8);

extern "C" void __rust_thunk__da1abd55__ZNSt3__u11__wrap_iterIPKDsEC1Ev(
    class std::__wrap_iter<const char16_t*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__b874af53__ZNKSt3__u11__wrap_iterIPKDsEplEl(
    class std::__wrap_iter<const char16_t*>* __return,
    class std::__wrap_iter<const char16_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<const char16_t*> (
                  ::std::__wrap_iter<const char16_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char16_t*>::operator+);

extern "C" class std::__wrap_iter<const char16_t*>*
__rust_thunk__7995cc76__ZNSt3__u11__wrap_iterIPKDsEpLEl(
    class std::__wrap_iter<const char16_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<const char16_t*> &
               (::std::__wrap_iter<const char16_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char16_t*>::operator+=);

extern "C" void __rust_thunk__69b7f5af__ZNKSt3__u11__wrap_iterIPKDsEmiEl(
    class std::__wrap_iter<const char16_t*>* __return,
    class std::__wrap_iter<const char16_t*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<const char16_t*> (
                  ::std::__wrap_iter<const char16_t*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char16_t*>::operator-);

extern "C" class std::__wrap_iter<const char16_t*>*
__rust_thunk__a05c7a73__ZNSt3__u11__wrap_iterIPKDsEmIEl(
    class std::__wrap_iter<const char16_t*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<const char16_t*> &
               (::std::__wrap_iter<const char16_t*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char16_t*>::operator-=);

extern "C" char16_t const*
__rust_thunk__09368604__ZNKSt3__u11__wrap_iterIPKDsEixEl(
    class std::__wrap_iter<const char16_t*> const* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((char16_t const& (::std::__wrap_iter<const char16_t*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<const char16_t*>::operator[]);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<const char*>) == 8);
static_assert(alignof(class std::__wrap_iter<const char*>) == 8);

extern "C" void __rust_thunk__da1abd55__ZNSt3__u11__wrap_iterIPKcEC1Ev(
    class std::__wrap_iter<const char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__b874af53__ZNKSt3__u11__wrap_iterIPKcEplEl(
    class std::__wrap_iter<const char*>* __return,
    class std::__wrap_iter<const char*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<const char*> (
                  ::std::__wrap_iter<const char*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char*>::operator+);

extern "C" class std::__wrap_iter<const char*>*
__rust_thunk__7995cc76__ZNSt3__u11__wrap_iterIPKcEpLEl(
    class std::__wrap_iter<const char*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<const char*> &
               (::std::__wrap_iter<const char*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char*>::operator+=);

extern "C" void __rust_thunk__69b7f5af__ZNKSt3__u11__wrap_iterIPKcEmiEl(
    class std::__wrap_iter<const char*>* __return,
    class std::__wrap_iter<const char*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<const char*> (
                  ::std::__wrap_iter<const char*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const char*>::operator-);

extern "C" class std::__wrap_iter<const char*>*
__rust_thunk__a05c7a73__ZNSt3__u11__wrap_iterIPKcEmIEl(
    class std::__wrap_iter<const char*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<const char*> &
               (::std::__wrap_iter<const char*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const char*>::operator-=);

extern "C" char const* __rust_thunk__09368604__ZNKSt3__u11__wrap_iterIPKcEixEl(
    class std::__wrap_iter<const char*> const* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((char const& (::std::__wrap_iter<const char*>::*)(ptrdiff_t)
                   const) &
              ::std::__wrap_iter<const char*>::operator[]);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<char*>) == 8);
static_assert(alignof(class std::__wrap_iter<char*>) == 8);

extern "C" void __rust_thunk__da1abd55__ZNSt3__u11__wrap_iterIPcEC1Ev(
    class std::__wrap_iter<char*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__b874af53__ZNKSt3__u11__wrap_iterIPcEplEl(
    class std::__wrap_iter<char*>* __return,
    class std::__wrap_iter<char*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<char*> (::std::__wrap_iter<char*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<char*>::operator+);

extern "C" class std::__wrap_iter<char*>*
__rust_thunk__7995cc76__ZNSt3__u11__wrap_iterIPcEpLEl(
    class std::__wrap_iter<char*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<char*> &
               (::std::__wrap_iter<char*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char*>::operator+=);

extern "C" void __rust_thunk__69b7f5af__ZNKSt3__u11__wrap_iterIPcEmiEl(
    class std::__wrap_iter<char*>* __return,
    class std::__wrap_iter<char*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<char*> (::std::__wrap_iter<char*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<char*>::operator-);

extern "C" class std::__wrap_iter<char*>*
__rust_thunk__a05c7a73__ZNSt3__u11__wrap_iterIPcEmIEl(
    class std::__wrap_iter<char*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<char*> &
               (::std::__wrap_iter<char*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<char*>::operator-=);

static_assert(sizeof(class std::ratio<1000000000000000000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000000000000000000L, 1L>) == 1);

extern "C" void
__rust_thunk__1b14ee06__ZNSt3__u5ratioILl1000000000000000000ELl1EEC1Ev(
    class std::ratio<1000000000000000000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1000000000000000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000000000000000L, 1L>) == 1);

extern "C" void
__rust_thunk__1b14ee06__ZNSt3__u5ratioILl1000000000000000ELl1EEC1Ev(
    class std::ratio<1000000000000000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1000000000000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000000000000L, 1L>) == 1);

extern "C" void
__rust_thunk__1b14ee06__ZNSt3__u5ratioILl1000000000000ELl1EEC1Ev(
    class std::ratio<1000000000000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1000000000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000000000L, 1L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl1000000000ELl1EEC1Ev(
    class std::ratio<1000000000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1000000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000000L, 1L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl1000000ELl1EEC1Ev(
    class std::ratio<1000000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1000L, 1L>) == 1);
static_assert(alignof(class std::ratio<1000L, 1L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl1000ELl1EEC1Ev(
    class std::ratio<1000L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<100L, 1L>) == 1);
static_assert(alignof(class std::ratio<100L, 1L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl100ELl1EEC1Ev(
    class std::ratio<100L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<10L, 1L>) == 1);
static_assert(alignof(class std::ratio<10L, 1L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl10ELl1EEC1Ev(
    class std::ratio<10L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000000000000000000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000000000000000000L>) == 1);

extern "C" void
__rust_thunk__1b14ee06__ZNSt3__u5ratioILl1ELl1000000000000000000EEC1Ev(
    class std::ratio<1L, 1000000000000000000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000000000000000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000000000000000L>) == 1);

extern "C" void
__rust_thunk__1b14ee06__ZNSt3__u5ratioILl1ELl1000000000000000EEC1Ev(
    class std::ratio<1L, 1000000000000000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000000000000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000000000000L>) == 1);

extern "C" void
__rust_thunk__1b14ee06__ZNSt3__u5ratioILl1ELl1000000000000EEC1Ev(
    class std::ratio<1L, 1000000000000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000000000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000000000L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl1ELl1000000000EEC1Ev(
    class std::ratio<1L, 1000000000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000000L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl1ELl1000000EEC1Ev(
    class std::ratio<1L, 1000000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1000L>) == 1);
static_assert(alignof(class std::ratio<1L, 1000L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl1ELl1000EEC1Ev(
    class std::ratio<1L, 1000L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 100L>) == 1);
static_assert(alignof(class std::ratio<1L, 100L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl1ELl100EEC1Ev(
    class std::ratio<1L, 100L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 10L>) == 1);
static_assert(alignof(class std::ratio<1L, 10L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl1ELl10EEC1Ev(
    class std::ratio<1L, 10L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<1L, 1L>) == 1);
static_assert(alignof(class std::ratio<1L, 1L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl1ELl1EEC1Ev(
    class std::ratio<1L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<2629746L, 1L>) == 1);
static_assert(alignof(class std::ratio<2629746L, 1L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl2629746ELl1EEC1Ev(
    class std::ratio<2629746L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<31556952L, 1L>) == 1);
static_assert(alignof(class std::ratio<31556952L, 1L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl31556952ELl1EEC1Ev(
    class std::ratio<31556952L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<3600L, 1L>) == 1);
static_assert(alignof(class std::ratio<3600L, 1L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl3600ELl1EEC1Ev(
    class std::ratio<3600L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<604800L, 1L>) == 1);
static_assert(alignof(class std::ratio<604800L, 1L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl604800ELl1EEC1Ev(
    class std::ratio<604800L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<60L, 1L>) == 1);
static_assert(alignof(class std::ratio<60L, 1L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl60ELl1EEC1Ev(
    class std::ratio<60L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class std::ratio<86400L, 1L>) == 1);
static_assert(alignof(class std::ratio<86400L, 1L>) == 1);

extern "C" void __rust_thunk__1b14ee06__ZNSt3__u5ratioILl86400ELl1EEC1Ev(
    class std::ratio<86400L, 1L>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<int, std::ratio<2629746L, 1L>>) ==
    4);
static_assert(
    alignof(class std::chrono::duration<int, std::ratio<2629746L, 1L>>) == 4);

extern "C" void
__rust_thunk__38d19830__ZNSt3__u6chrono8durationIiNS_5ratioILl2629746ELl1EEEEC1Ev(
    class std::chrono::duration<int, std::ratio<2629746L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::chrono::duration<int, std::ratio<31556952L, 1L>>) == 4);
static_assert(
    alignof(class std::chrono::duration<int, std::ratio<31556952L, 1L>>) == 4);

extern "C" void
__rust_thunk__38d19830__ZNSt3__u6chrono8durationIiNS_5ratioILl31556952ELl1EEEEC1Ev(
    class std::chrono::duration<int, std::ratio<31556952L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<int, std::ratio<604800L, 1L>>) ==
    4);
static_assert(
    alignof(class std::chrono::duration<int, std::ratio<604800L, 1L>>) == 4);

extern "C" void
__rust_thunk__38d19830__ZNSt3__u6chrono8durationIiNS_5ratioILl604800ELl1EEEEC1Ev(
    class std::chrono::duration<int, std::ratio<604800L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<int, std::ratio<86400L, 1L>>) ==
    4);
static_assert(
    alignof(class std::chrono::duration<int, std::ratio<86400L, 1L>>) == 4);

extern "C" void
__rust_thunk__38d19830__ZNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEC1Ev(
    class std::chrono::duration<int, std::ratio<86400L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<long, std::ratio<3600L, 1L>>) ==
    8);
static_assert(
    alignof(class std::chrono::duration<long, std::ratio<3600L, 1L>>) == 8);

extern "C" void
__rust_thunk__38d19830__ZNSt3__u6chrono8durationIlNS_5ratioILl3600ELl1EEEEC1Ev(
    class std::chrono::duration<long, std::ratio<3600L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<long, std::ratio<60L, 1L>>) == 8);
static_assert(alignof(class std::chrono::duration<long, std::ratio<60L, 1L>>) ==
              8);

extern "C" void
__rust_thunk__38d19830__ZNSt3__u6chrono8durationIlNS_5ratioILl60ELl1EEEEC1Ev(
    class std::chrono::duration<long, std::ratio<60L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::chrono::duration<long long, std::ratio<1L, 1000000000L>>) ==
    8);
static_assert(
    alignof(
        class std::chrono::duration<long long, std::ratio<1L, 1000000000L>>) ==
    8);

extern "C" void
__rust_thunk__38d19830__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEC1Ev(
    class std::chrono::duration<long long, std::ratio<1L, 1000000000L>>*
        __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::chrono::duration<long long, std::ratio<1L, 1000000L>>) == 8);
static_assert(
    alignof(class std::chrono::duration<long long, std::ratio<1L, 1000000L>>) ==
    8);

extern "C" void
__rust_thunk__38d19830__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEC1Ev(
    class std::chrono::duration<long long, std::ratio<1L, 1000000L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::chrono::duration<long long, std::ratio<1L, 1000L>>) == 8);
static_assert(
    alignof(class std::chrono::duration<long long, std::ratio<1L, 1000L>>) ==
    8);

extern "C" void
__rust_thunk__38d19830__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000EEEEC1Ev(
    class std::chrono::duration<long long, std::ratio<1L, 1000L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::duration<long long, std::ratio<1L, 1L>>) ==
    8);
static_assert(
    alignof(class std::chrono::duration<long long, std::ratio<1L, 1L>>) == 8);

extern "C" void
__rust_thunk__38d19830__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEC1Ev(
    class std::chrono::duration<long long, std::ratio<1L, 1L>>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::chrono::time_point<
            std::chrono::steady_clock,
            std::chrono::duration<long long, std::ratio<1L, 1000000000L>>>) ==
    8);
static_assert(
    alignof(class std::chrono::time_point<
            std::chrono::steady_clock,
            std::chrono::duration<long long, std::ratio<1L, 1000000000L>>>) ==
    8);

extern "C" void
__rust_thunk__f893e3b4__ZNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEC1Ev(
    class std::chrono::time_point<
        std::chrono::steady_clock,
        std::chrono::duration<long long, std::ratio<1L, 1000000000L>>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__e9b011cf__ZNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEC1ERKS6_(
    class std::chrono::time_point<
        std::chrono::steady_clock,
        std::chrono::duration<long long, std::ratio<1L, 1000000000L>>>* __this,
    class std::chrono::duration<long long, std::ratio<1L, 1000000000L>> const*
        __d) {
  crubit::construct_at(__this, *__d);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::time_point<
                  std::chrono::system_clock,
                  std::chrono::duration<int, std::ratio<86400L, 1L>>>) == 4);
static_assert(alignof(class std::chrono::time_point<
                      std::chrono::system_clock,
                      std::chrono::duration<int, std::ratio<86400L, 1L>>>) ==
              4);

extern "C" void
__rust_thunk__f893e3b4__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEC1Ev(
    class std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<int, std::ratio<86400L, 1L>>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__e9b011cf__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEC1ERKS6_(
    class std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<int, std::ratio<86400L, 1L>>>* __this,
    class std::chrono::duration<int, std::ratio<86400L, 1L>> const* __d) {
  crubit::construct_at(__this, *__d);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::chrono::time_point<
            std::chrono::system_clock,
            std::chrono::duration<long long, std::ratio<1L, 1000000L>>>) == 8);
static_assert(
    alignof(class std::chrono::time_point<
            std::chrono::system_clock,
            std::chrono::duration<long long, std::ratio<1L, 1000000L>>>) == 8);

extern "C" void
__rust_thunk__f893e3b4__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEC1Ev(
    class std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long long, std::ratio<1L, 1000000L>>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__e9b011cf__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEC1ERKS6_(
    class std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long long, std::ratio<1L, 1000000L>>>* __this,
    class std::chrono::duration<long long, std::ratio<1L, 1000000L>> const*
        __d) {
  crubit::construct_at(__this, *__d);
}

static_assert(
    CRUBIT_SIZEOF(class std::chrono::time_point<
                  std::chrono::system_clock,
                  std::chrono::duration<long long, std::ratio<1L, 1L>>>) == 8);
static_assert(alignof(class std::chrono::time_point<
                      std::chrono::system_clock,
                      std::chrono::duration<long long, std::ratio<1L, 1L>>>) ==
              8);

extern "C" void
__rust_thunk__f893e3b4__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEC1Ev(
    class std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long long, std::ratio<1L, 1L>>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__e9b011cf__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEC1ERKS6_(
    class std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long long, std::ratio<1L, 1L>>>* __this,
    class std::chrono::duration<long long, std::ratio<1L, 1L>> const* __d) {
  crubit::construct_at(__this, *__d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<char32_t, false>) == 4);
static_assert(alignof(struct std::__atomic_base<char32_t, false>) == 4);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<char32_t, false>) ==
              0);

extern "C" struct std::__atomic_base<char32_t, false>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIDiLb0EEaSERKS1_(
    struct std::__atomic_base<char32_t, false>* __this,
    struct std::__atomic_base<char32_t, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char32_t, false> &
               (::std::__atomic_base<char32_t, false>::*)(
                   struct std::__atomic_base<char32_t, false> const&)) &
              ::std::__atomic_base<char32_t, false>::operator=);

extern "C" bool
__rust_thunk__db5691d0__ZNKSt3__u13__atomic_baseIDiLb0EE12is_lock_freeEv(
    struct std::__atomic_base<char32_t, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<char32_t, false>::*)() const) &
              ::std::__atomic_base<char32_t, false>::is_lock_free);

extern "C" void
__rust_thunk__962f68f8__ZNSt3__u13__atomic_baseIDiLb0EE10notify_oneEv(
    struct std::__atomic_base<char32_t, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<char32_t, false>::*)()) &
              ::std::__atomic_base<char32_t, false>::notify_one);

extern "C" void
__rust_thunk__726ee9df__ZNSt3__u13__atomic_baseIDiLb0EE10notify_allEv(
    struct std::__atomic_base<char32_t, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<char32_t, false>::*)()) &
              ::std::__atomic_base<char32_t, false>::notify_all);

extern "C" void __rust_thunk__aaa6a3c9__ZNSt3__u13__atomic_baseIDiLb0EEC1Ev(
    struct std::__atomic_base<char32_t, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9dff41f9__ZNSt3__u13__atomic_baseIDiLb0EEC1EDi(
    struct std::__atomic_base<char32_t, false>* __this, char32_t __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<char16_t, false>) == 2);
static_assert(alignof(struct std::__atomic_base<char16_t, false>) == 2);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<char16_t, false>) ==
              0);

extern "C" struct std::__atomic_base<char16_t, false>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIDsLb0EEaSERKS1_(
    struct std::__atomic_base<char16_t, false>* __this,
    struct std::__atomic_base<char16_t, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char16_t, false> &
               (::std::__atomic_base<char16_t, false>::*)(
                   struct std::__atomic_base<char16_t, false> const&)) &
              ::std::__atomic_base<char16_t, false>::operator=);

extern "C" bool
__rust_thunk__db5691d0__ZNKSt3__u13__atomic_baseIDsLb0EE12is_lock_freeEv(
    struct std::__atomic_base<char16_t, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<char16_t, false>::*)() const) &
              ::std::__atomic_base<char16_t, false>::is_lock_free);

extern "C" void
__rust_thunk__962f68f8__ZNSt3__u13__atomic_baseIDsLb0EE10notify_oneEv(
    struct std::__atomic_base<char16_t, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<char16_t, false>::*)()) &
              ::std::__atomic_base<char16_t, false>::notify_one);

extern "C" void
__rust_thunk__726ee9df__ZNSt3__u13__atomic_baseIDsLb0EE10notify_allEv(
    struct std::__atomic_base<char16_t, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<char16_t, false>::*)()) &
              ::std::__atomic_base<char16_t, false>::notify_all);

extern "C" void __rust_thunk__aaa6a3c9__ZNSt3__u13__atomic_baseIDsLb0EEC1Ev(
    struct std::__atomic_base<char16_t, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9dff41f9__ZNSt3__u13__atomic_baseIDsLb0EEC1EDs(
    struct std::__atomic_base<char16_t, false>* __this, char16_t __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<char8_t, false>) == 1);
static_assert(alignof(struct std::__atomic_base<char8_t, false>) == 1);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<char8_t, false>) == 0);

extern "C" struct std::__atomic_base<char8_t, false>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIDuLb0EEaSERKS1_(
    struct std::__atomic_base<char8_t, false>* __this,
    struct std::__atomic_base<char8_t, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char8_t, false> &
               (::std::__atomic_base<char8_t, false>::*)(
                   struct std::__atomic_base<char8_t, false> const&)) &
              ::std::__atomic_base<char8_t, false>::operator=);

extern "C" bool
__rust_thunk__db5691d0__ZNKSt3__u13__atomic_baseIDuLb0EE12is_lock_freeEv(
    struct std::__atomic_base<char8_t, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<char8_t, false>::*)() const) &
              ::std::__atomic_base<char8_t, false>::is_lock_free);

extern "C" void
__rust_thunk__962f68f8__ZNSt3__u13__atomic_baseIDuLb0EE10notify_oneEv(
    struct std::__atomic_base<char8_t, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<char8_t, false>::*)()) &
              ::std::__atomic_base<char8_t, false>::notify_one);

extern "C" void
__rust_thunk__726ee9df__ZNSt3__u13__atomic_baseIDuLb0EE10notify_allEv(
    struct std::__atomic_base<char8_t, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<char8_t, false>::*)()) &
              ::std::__atomic_base<char8_t, false>::notify_all);

extern "C" void __rust_thunk__aaa6a3c9__ZNSt3__u13__atomic_baseIDuLb0EEC1Ev(
    struct std::__atomic_base<char8_t, false>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::__atomic_base<signed char, false>) == 1);
static_assert(alignof(struct std::__atomic_base<signed char, false>) == 1);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<signed char, false>) ==
              0);

extern "C" struct std::__atomic_base<signed char, false>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIaLb0EEaSERKS1_(
    struct std::__atomic_base<signed char, false>* __this,
    struct std::__atomic_base<signed char, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<signed char, false> &
               (::std::__atomic_base<signed char, false>::*)(
                   struct std::__atomic_base<signed char, false> const&)) &
              ::std::__atomic_base<signed char, false>::operator=);

extern "C" bool
__rust_thunk__db5691d0__ZNKSt3__u13__atomic_baseIaLb0EE12is_lock_freeEv(
    struct std::__atomic_base<signed char, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<signed char, false>::*)() const) &
              ::std::__atomic_base<signed char, false>::is_lock_free);

extern "C" void
__rust_thunk__962f68f8__ZNSt3__u13__atomic_baseIaLb0EE10notify_oneEv(
    struct std::__atomic_base<signed char, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<signed char, false>::*)()) &
              ::std::__atomic_base<signed char, false>::notify_one);

extern "C" void
__rust_thunk__726ee9df__ZNSt3__u13__atomic_baseIaLb0EE10notify_allEv(
    struct std::__atomic_base<signed char, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<signed char, false>::*)()) &
              ::std::__atomic_base<signed char, false>::notify_all);

extern "C" void __rust_thunk__aaa6a3c9__ZNSt3__u13__atomic_baseIaLb0EEC1Ev(
    struct std::__atomic_base<signed char, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9dff41f9__ZNSt3__u13__atomic_baseIaLb0EEC1Ea(
    struct std::__atomic_base<signed char, false>* __this, signed char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<bool, false>) == 1);
static_assert(alignof(struct std::__atomic_base<bool, false>) == 1);
static_assert(CRUBIT_OFFSET_OF(__a_, struct std::__atomic_base<bool, false>) ==
              0);

extern "C" struct std::__atomic_base<bool, false>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIbLb0EEaSERKS1_(
    struct std::__atomic_base<bool, false>* __this,
    struct std::__atomic_base<bool, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<bool, false> &
               (::std::__atomic_base<bool, false>::*)(
                   struct std::__atomic_base<bool, false> const&)) &
              ::std::__atomic_base<bool, false>::operator=);

extern "C" bool
__rust_thunk__db5691d0__ZNKSt3__u13__atomic_baseIbLb0EE12is_lock_freeEv(
    struct std::__atomic_base<bool, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<bool, false>::*)() const) &
              ::std::__atomic_base<bool, false>::is_lock_free);

extern "C" void
__rust_thunk__962f68f8__ZNSt3__u13__atomic_baseIbLb0EE10notify_oneEv(
    struct std::__atomic_base<bool, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<bool, false>::*)()) &
              ::std::__atomic_base<bool, false>::notify_one);

extern "C" void
__rust_thunk__726ee9df__ZNSt3__u13__atomic_baseIbLb0EE10notify_allEv(
    struct std::__atomic_base<bool, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<bool, false>::*)()) &
              ::std::__atomic_base<bool, false>::notify_all);

extern "C" void __rust_thunk__aaa6a3c9__ZNSt3__u13__atomic_baseIbLb0EEC1Ev(
    struct std::__atomic_base<bool, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9dff41f9__ZNSt3__u13__atomic_baseIbLb0EEC1Eb(
    struct std::__atomic_base<bool, false>* __this, bool __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<char, false>) == 1);
static_assert(alignof(struct std::__atomic_base<char, false>) == 1);
static_assert(CRUBIT_OFFSET_OF(__a_, struct std::__atomic_base<char, false>) ==
              0);

extern "C" struct std::__atomic_base<char, false>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIcLb0EEaSERKS1_(
    struct std::__atomic_base<char, false>* __this,
    struct std::__atomic_base<char, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char, false> &
               (::std::__atomic_base<char, false>::*)(
                   struct std::__atomic_base<char, false> const&)) &
              ::std::__atomic_base<char, false>::operator=);

extern "C" bool
__rust_thunk__db5691d0__ZNKSt3__u13__atomic_baseIcLb0EE12is_lock_freeEv(
    struct std::__atomic_base<char, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<char, false>::*)() const) &
              ::std::__atomic_base<char, false>::is_lock_free);

extern "C" void
__rust_thunk__962f68f8__ZNSt3__u13__atomic_baseIcLb0EE10notify_oneEv(
    struct std::__atomic_base<char, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<char, false>::*)()) &
              ::std::__atomic_base<char, false>::notify_one);

extern "C" void
__rust_thunk__726ee9df__ZNSt3__u13__atomic_baseIcLb0EE10notify_allEv(
    struct std::__atomic_base<char, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<char, false>::*)()) &
              ::std::__atomic_base<char, false>::notify_all);

extern "C" void __rust_thunk__aaa6a3c9__ZNSt3__u13__atomic_baseIcLb0EEC1Ev(
    struct std::__atomic_base<char, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9dff41f9__ZNSt3__u13__atomic_baseIcLb0EEC1Ec(
    struct std::__atomic_base<char, false>* __this, char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<unsigned char, false>) == 1);
static_assert(alignof(struct std::__atomic_base<unsigned char, false>) == 1);
static_assert(CRUBIT_OFFSET_OF(
                  __a_, struct std::__atomic_base<unsigned char, false>) == 0);

extern "C" struct std::__atomic_base<unsigned char, false>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIhLb0EEaSERKS1_(
    struct std::__atomic_base<unsigned char, false>* __this,
    struct std::__atomic_base<unsigned char, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned char, false> &
               (::std::__atomic_base<unsigned char, false>::*)(
                   struct std::__atomic_base<unsigned char, false> const&)) &
              ::std::__atomic_base<unsigned char, false>::operator=);

extern "C" bool
__rust_thunk__db5691d0__ZNKSt3__u13__atomic_baseIhLb0EE12is_lock_freeEv(
    struct std::__atomic_base<unsigned char, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<unsigned char, false>::*)() const) &
              ::std::__atomic_base<unsigned char, false>::is_lock_free);

extern "C" void
__rust_thunk__962f68f8__ZNSt3__u13__atomic_baseIhLb0EE10notify_oneEv(
    struct std::__atomic_base<unsigned char, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<unsigned char, false>::*)()) &
              ::std::__atomic_base<unsigned char, false>::notify_one);

extern "C" void
__rust_thunk__726ee9df__ZNSt3__u13__atomic_baseIhLb0EE10notify_allEv(
    struct std::__atomic_base<unsigned char, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<unsigned char, false>::*)()) &
              ::std::__atomic_base<unsigned char, false>::notify_all);

extern "C" void __rust_thunk__aaa6a3c9__ZNSt3__u13__atomic_baseIhLb0EEC1Ev(
    struct std::__atomic_base<unsigned char, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9dff41f9__ZNSt3__u13__atomic_baseIhLb0EEC1Eh(
    struct std::__atomic_base<unsigned char, false>* __this,
    unsigned char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<int, false>) == 4);
static_assert(alignof(struct std::__atomic_base<int, false>) == 4);
static_assert(CRUBIT_OFFSET_OF(__a_, struct std::__atomic_base<int, false>) ==
              0);

extern "C" struct std::__atomic_base<int, false>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIiLb0EEaSERKS1_(
    struct std::__atomic_base<int, false>* __this,
    struct std::__atomic_base<int, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<int, false> &
               (::std::__atomic_base<int, false>::*)(
                   struct std::__atomic_base<int, false> const&)) &
              ::std::__atomic_base<int, false>::operator=);

extern "C" bool
__rust_thunk__db5691d0__ZNKSt3__u13__atomic_baseIiLb0EE12is_lock_freeEv(
    struct std::__atomic_base<int, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<int, false>::*)() const) &
              ::std::__atomic_base<int, false>::is_lock_free);

extern "C" void
__rust_thunk__962f68f8__ZNSt3__u13__atomic_baseIiLb0EE10notify_oneEv(
    struct std::__atomic_base<int, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<int, false>::*)()) &
              ::std::__atomic_base<int, false>::notify_one);

extern "C" void
__rust_thunk__726ee9df__ZNSt3__u13__atomic_baseIiLb0EE10notify_allEv(
    struct std::__atomic_base<int, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<int, false>::*)()) &
              ::std::__atomic_base<int, false>::notify_all);

extern "C" void __rust_thunk__aaa6a3c9__ZNSt3__u13__atomic_baseIiLb0EEC1Ev(
    struct std::__atomic_base<int, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9dff41f9__ZNSt3__u13__atomic_baseIiLb0EEC1Ei(
    struct std::__atomic_base<int, false>* __this, int __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned int, false>) ==
              4);
static_assert(alignof(struct std::__atomic_base<unsigned int, false>) == 4);
static_assert(CRUBIT_OFFSET_OF(
                  __a_, struct std::__atomic_base<unsigned int, false>) == 0);

extern "C" struct std::__atomic_base<unsigned int, false>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIjLb0EEaSERKS1_(
    struct std::__atomic_base<unsigned int, false>* __this,
    struct std::__atomic_base<unsigned int, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned int, false> &
               (::std::__atomic_base<unsigned int, false>::*)(
                   struct std::__atomic_base<unsigned int, false> const&)) &
              ::std::__atomic_base<unsigned int, false>::operator=);

extern "C" bool
__rust_thunk__db5691d0__ZNKSt3__u13__atomic_baseIjLb0EE12is_lock_freeEv(
    struct std::__atomic_base<unsigned int, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<unsigned int, false>::*)() const) &
              ::std::__atomic_base<unsigned int, false>::is_lock_free);

extern "C" void
__rust_thunk__962f68f8__ZNSt3__u13__atomic_baseIjLb0EE10notify_oneEv(
    struct std::__atomic_base<unsigned int, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<unsigned int, false>::*)()) &
              ::std::__atomic_base<unsigned int, false>::notify_one);

extern "C" void
__rust_thunk__726ee9df__ZNSt3__u13__atomic_baseIjLb0EE10notify_allEv(
    struct std::__atomic_base<unsigned int, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<unsigned int, false>::*)()) &
              ::std::__atomic_base<unsigned int, false>::notify_all);

extern "C" void __rust_thunk__aaa6a3c9__ZNSt3__u13__atomic_baseIjLb0EEC1Ev(
    struct std::__atomic_base<unsigned int, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9dff41f9__ZNSt3__u13__atomic_baseIjLb0EEC1Ej(
    struct std::__atomic_base<unsigned int, false>* __this, unsigned int __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<long, false>) == 8);
static_assert(alignof(struct std::__atomic_base<long, false>) == 8);
static_assert(CRUBIT_OFFSET_OF(__a_, struct std::__atomic_base<long, false>) ==
              0);

extern "C" struct std::__atomic_base<long, false>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIlLb0EEaSERKS1_(
    struct std::__atomic_base<long, false>* __this,
    struct std::__atomic_base<long, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<long, false> &
               (::std::__atomic_base<long, false>::*)(
                   struct std::__atomic_base<long, false> const&)) &
              ::std::__atomic_base<long, false>::operator=);

extern "C" bool
__rust_thunk__db5691d0__ZNKSt3__u13__atomic_baseIlLb0EE12is_lock_freeEv(
    struct std::__atomic_base<long, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<long, false>::*)() const) &
              ::std::__atomic_base<long, false>::is_lock_free);

extern "C" void
__rust_thunk__962f68f8__ZNSt3__u13__atomic_baseIlLb0EE10notify_oneEv(
    struct std::__atomic_base<long, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<long, false>::*)()) &
              ::std::__atomic_base<long, false>::notify_one);

extern "C" void
__rust_thunk__726ee9df__ZNSt3__u13__atomic_baseIlLb0EE10notify_allEv(
    struct std::__atomic_base<long, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<long, false>::*)()) &
              ::std::__atomic_base<long, false>::notify_all);

extern "C" void __rust_thunk__aaa6a3c9__ZNSt3__u13__atomic_baseIlLb0EEC1Ev(
    struct std::__atomic_base<long, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9dff41f9__ZNSt3__u13__atomic_baseIlLb0EEC1El(
    struct std::__atomic_base<long, false>* __this, long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned long, false>) ==
              8);
static_assert(alignof(struct std::__atomic_base<unsigned long, false>) == 8);
static_assert(CRUBIT_OFFSET_OF(
                  __a_, struct std::__atomic_base<unsigned long, false>) == 0);

extern "C" struct std::__atomic_base<unsigned long, false>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseImLb0EEaSERKS1_(
    struct std::__atomic_base<unsigned long, false>* __this,
    struct std::__atomic_base<unsigned long, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned long, false> &
               (::std::__atomic_base<unsigned long, false>::*)(
                   struct std::__atomic_base<unsigned long, false> const&)) &
              ::std::__atomic_base<unsigned long, false>::operator=);

extern "C" bool
__rust_thunk__db5691d0__ZNKSt3__u13__atomic_baseImLb0EE12is_lock_freeEv(
    struct std::__atomic_base<unsigned long, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<unsigned long, false>::*)() const) &
              ::std::__atomic_base<unsigned long, false>::is_lock_free);

extern "C" void
__rust_thunk__962f68f8__ZNSt3__u13__atomic_baseImLb0EE10notify_oneEv(
    struct std::__atomic_base<unsigned long, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<unsigned long, false>::*)()) &
              ::std::__atomic_base<unsigned long, false>::notify_one);

extern "C" void
__rust_thunk__726ee9df__ZNSt3__u13__atomic_baseImLb0EE10notify_allEv(
    struct std::__atomic_base<unsigned long, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<unsigned long, false>::*)()) &
              ::std::__atomic_base<unsigned long, false>::notify_all);

extern "C" void __rust_thunk__aaa6a3c9__ZNSt3__u13__atomic_baseImLb0EEC1Ev(
    struct std::__atomic_base<unsigned long, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9dff41f9__ZNSt3__u13__atomic_baseImLb0EEC1Em(
    struct std::__atomic_base<unsigned long, false>* __this,
    unsigned long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<short, false>) == 2);
static_assert(alignof(struct std::__atomic_base<short, false>) == 2);
static_assert(CRUBIT_OFFSET_OF(__a_, struct std::__atomic_base<short, false>) ==
              0);

extern "C" struct std::__atomic_base<short, false>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIsLb0EEaSERKS1_(
    struct std::__atomic_base<short, false>* __this,
    struct std::__atomic_base<short, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<short, false> &
               (::std::__atomic_base<short, false>::*)(
                   struct std::__atomic_base<short, false> const&)) &
              ::std::__atomic_base<short, false>::operator=);

extern "C" bool
__rust_thunk__db5691d0__ZNKSt3__u13__atomic_baseIsLb0EE12is_lock_freeEv(
    struct std::__atomic_base<short, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<short, false>::*)() const) &
              ::std::__atomic_base<short, false>::is_lock_free);

extern "C" void
__rust_thunk__962f68f8__ZNSt3__u13__atomic_baseIsLb0EE10notify_oneEv(
    struct std::__atomic_base<short, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<short, false>::*)()) &
              ::std::__atomic_base<short, false>::notify_one);

extern "C" void
__rust_thunk__726ee9df__ZNSt3__u13__atomic_baseIsLb0EE10notify_allEv(
    struct std::__atomic_base<short, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<short, false>::*)()) &
              ::std::__atomic_base<short, false>::notify_all);

extern "C" void __rust_thunk__aaa6a3c9__ZNSt3__u13__atomic_baseIsLb0EEC1Ev(
    struct std::__atomic_base<short, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9dff41f9__ZNSt3__u13__atomic_baseIsLb0EEC1Es(
    struct std::__atomic_base<short, false>* __this, short __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned short, false>) ==
              2);
static_assert(alignof(struct std::__atomic_base<unsigned short, false>) == 2);
static_assert(CRUBIT_OFFSET_OF(
                  __a_, struct std::__atomic_base<unsigned short, false>) == 0);

extern "C" struct std::__atomic_base<unsigned short, false>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseItLb0EEaSERKS1_(
    struct std::__atomic_base<unsigned short, false>* __this,
    struct std::__atomic_base<unsigned short, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned short, false> &
               (::std::__atomic_base<unsigned short, false>::*)(
                   struct std::__atomic_base<unsigned short, false> const&)) &
              ::std::__atomic_base<unsigned short, false>::operator=);

extern "C" bool
__rust_thunk__db5691d0__ZNKSt3__u13__atomic_baseItLb0EE12is_lock_freeEv(
    struct std::__atomic_base<unsigned short, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<unsigned short, false>::*)() const) &
              ::std::__atomic_base<unsigned short, false>::is_lock_free);

extern "C" void
__rust_thunk__962f68f8__ZNSt3__u13__atomic_baseItLb0EE10notify_oneEv(
    struct std::__atomic_base<unsigned short, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<unsigned short, false>::*)()) &
              ::std::__atomic_base<unsigned short, false>::notify_one);

extern "C" void
__rust_thunk__726ee9df__ZNSt3__u13__atomic_baseItLb0EE10notify_allEv(
    struct std::__atomic_base<unsigned short, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<unsigned short, false>::*)()) &
              ::std::__atomic_base<unsigned short, false>::notify_all);

extern "C" void __rust_thunk__aaa6a3c9__ZNSt3__u13__atomic_baseItLb0EEC1Ev(
    struct std::__atomic_base<unsigned short, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9dff41f9__ZNSt3__u13__atomic_baseItLb0EEC1Et(
    struct std::__atomic_base<unsigned short, false>* __this,
    unsigned short __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<wchar_t, false>) == 4);
static_assert(alignof(struct std::__atomic_base<wchar_t, false>) == 4);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<wchar_t, false>) == 0);

extern "C" struct std::__atomic_base<wchar_t, false>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIwLb0EEaSERKS1_(
    struct std::__atomic_base<wchar_t, false>* __this,
    struct std::__atomic_base<wchar_t, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<wchar_t, false> &
               (::std::__atomic_base<wchar_t, false>::*)(
                   struct std::__atomic_base<wchar_t, false> const&)) &
              ::std::__atomic_base<wchar_t, false>::operator=);

extern "C" bool
__rust_thunk__db5691d0__ZNKSt3__u13__atomic_baseIwLb0EE12is_lock_freeEv(
    struct std::__atomic_base<wchar_t, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<wchar_t, false>::*)() const) &
              ::std::__atomic_base<wchar_t, false>::is_lock_free);

extern "C" void
__rust_thunk__962f68f8__ZNSt3__u13__atomic_baseIwLb0EE10notify_oneEv(
    struct std::__atomic_base<wchar_t, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<wchar_t, false>::*)()) &
              ::std::__atomic_base<wchar_t, false>::notify_one);

extern "C" void
__rust_thunk__726ee9df__ZNSt3__u13__atomic_baseIwLb0EE10notify_allEv(
    struct std::__atomic_base<wchar_t, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<wchar_t, false>::*)()) &
              ::std::__atomic_base<wchar_t, false>::notify_all);

extern "C" void __rust_thunk__aaa6a3c9__ZNSt3__u13__atomic_baseIwLb0EEC1Ev(
    struct std::__atomic_base<wchar_t, false>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<long long, false>) == 8);
static_assert(alignof(struct std::__atomic_base<long long, false>) == 8);
static_assert(CRUBIT_OFFSET_OF(__a_,
                               struct std::__atomic_base<long long, false>) ==
              0);

extern "C" struct std::__atomic_base<long long, false>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIxLb0EEaSERKS1_(
    struct std::__atomic_base<long long, false>* __this,
    struct std::__atomic_base<long long, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<long long, false> &
               (::std::__atomic_base<long long, false>::*)(
                   struct std::__atomic_base<long long, false> const&)) &
              ::std::__atomic_base<long long, false>::operator=);

extern "C" bool
__rust_thunk__db5691d0__ZNKSt3__u13__atomic_baseIxLb0EE12is_lock_freeEv(
    struct std::__atomic_base<long long, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<long long, false>::*)() const) &
              ::std::__atomic_base<long long, false>::is_lock_free);

extern "C" void
__rust_thunk__962f68f8__ZNSt3__u13__atomic_baseIxLb0EE10notify_oneEv(
    struct std::__atomic_base<long long, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<long long, false>::*)()) &
              ::std::__atomic_base<long long, false>::notify_one);

extern "C" void
__rust_thunk__726ee9df__ZNSt3__u13__atomic_baseIxLb0EE10notify_allEv(
    struct std::__atomic_base<long long, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<long long, false>::*)()) &
              ::std::__atomic_base<long long, false>::notify_all);

extern "C" void __rust_thunk__aaa6a3c9__ZNSt3__u13__atomic_baseIxLb0EEC1Ev(
    struct std::__atomic_base<long long, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9dff41f9__ZNSt3__u13__atomic_baseIxLb0EEC1Ex(
    struct std::__atomic_base<long long, false>* __this, long long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(
    CRUBIT_SIZEOF(struct std::__atomic_base<unsigned long long, false>) == 8);
static_assert(alignof(struct std::__atomic_base<unsigned long long, false>) ==
              8);
static_assert(CRUBIT_OFFSET_OF(
                  __a_, struct std::__atomic_base<unsigned long long, false>) ==
              0);

extern "C" struct std::__atomic_base<unsigned long long, false>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIyLb0EEaSERKS1_(
    struct std::__atomic_base<unsigned long long, false>* __this,
    struct std::__atomic_base<unsigned long long, false> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert(
    (struct std::__atomic_base<unsigned long long, false> &
     (::std::__atomic_base<unsigned long long, false>::*)(
         struct std::__atomic_base<unsigned long long, false> const&)) &
    ::std::__atomic_base<unsigned long long, false>::operator=);

extern "C" bool
__rust_thunk__db5691d0__ZNKSt3__u13__atomic_baseIyLb0EE12is_lock_freeEv(
    struct std::__atomic_base<unsigned long long, false> const* __this) {
  return __this->is_lock_free();
}

static_assert((bool (::std::__atomic_base<unsigned long long, false>::*)()
                   const) &
              ::std::__atomic_base<unsigned long long, false>::is_lock_free);

extern "C" void
__rust_thunk__962f68f8__ZNSt3__u13__atomic_baseIyLb0EE10notify_oneEv(
    struct std::__atomic_base<unsigned long long, false>* __this) {
  __this->notify_one();
}

static_assert((void (::std::__atomic_base<unsigned long long, false>::*)()) &
              ::std::__atomic_base<unsigned long long, false>::notify_one);

extern "C" void
__rust_thunk__726ee9df__ZNSt3__u13__atomic_baseIyLb0EE10notify_allEv(
    struct std::__atomic_base<unsigned long long, false>* __this) {
  __this->notify_all();
}

static_assert((void (::std::__atomic_base<unsigned long long, false>::*)()) &
              ::std::__atomic_base<unsigned long long, false>::notify_all);

extern "C" void __rust_thunk__aaa6a3c9__ZNSt3__u13__atomic_baseIyLb0EEC1Ev(
    struct std::__atomic_base<unsigned long long, false>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__9dff41f9__ZNSt3__u13__atomic_baseIyLb0EEC1Ey(
    struct std::__atomic_base<unsigned long long, false>* __this,
    unsigned long long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<char32_t, true>) == 4);
static_assert(alignof(struct std::__atomic_base<char32_t, true>) == 4);

extern "C" struct std::__atomic_base<char32_t, true>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIDiLb1EEaSEOS1_(
    struct std::__atomic_base<char32_t, true>* __this,
    struct std::__atomic_base<char32_t, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<char32_t, true> &
               (::std::__atomic_base<char32_t, true>::*)(
                   struct std::__atomic_base<char32_t, true>&&)) &
              ::std::__atomic_base<char32_t, true>::operator=);

extern "C" struct std::__atomic_base<char32_t, true>*
__rust_thunk__e69cfcf4__ZNSt3__u13__atomic_baseIDiLb1EEaSERKS1_(
    struct std::__atomic_base<char32_t, true>* __this,
    struct std::__atomic_base<char32_t, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char32_t, true> &
               (::std::__atomic_base<char32_t, true>::*)(
                   struct std::__atomic_base<char32_t, true> const&)) &
              ::std::__atomic_base<char32_t, true>::operator=);

extern "C" void __rust_thunk__e6b4fa09__ZNSt3__u13__atomic_baseIDiLb1EEC1Ev(
    struct std::__atomic_base<char32_t, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__72113745__ZNSt3__u13__atomic_baseIDiLb1EEC1EDi(
    struct std::__atomic_base<char32_t, true>* __this, char32_t __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<char16_t, true>) == 2);
static_assert(alignof(struct std::__atomic_base<char16_t, true>) == 2);

extern "C" struct std::__atomic_base<char16_t, true>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIDsLb1EEaSEOS1_(
    struct std::__atomic_base<char16_t, true>* __this,
    struct std::__atomic_base<char16_t, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<char16_t, true> &
               (::std::__atomic_base<char16_t, true>::*)(
                   struct std::__atomic_base<char16_t, true>&&)) &
              ::std::__atomic_base<char16_t, true>::operator=);

extern "C" struct std::__atomic_base<char16_t, true>*
__rust_thunk__e69cfcf4__ZNSt3__u13__atomic_baseIDsLb1EEaSERKS1_(
    struct std::__atomic_base<char16_t, true>* __this,
    struct std::__atomic_base<char16_t, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char16_t, true> &
               (::std::__atomic_base<char16_t, true>::*)(
                   struct std::__atomic_base<char16_t, true> const&)) &
              ::std::__atomic_base<char16_t, true>::operator=);

extern "C" void __rust_thunk__e6b4fa09__ZNSt3__u13__atomic_baseIDsLb1EEC1Ev(
    struct std::__atomic_base<char16_t, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__72113745__ZNSt3__u13__atomic_baseIDsLb1EEC1EDs(
    struct std::__atomic_base<char16_t, true>* __this, char16_t __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<char8_t, true>) == 1);
static_assert(alignof(struct std::__atomic_base<char8_t, true>) == 1);

extern "C" struct std::__atomic_base<char8_t, true>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIDuLb1EEaSEOS1_(
    struct std::__atomic_base<char8_t, true>* __this,
    struct std::__atomic_base<char8_t, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<char8_t, true> &
               (::std::__atomic_base<char8_t, true>::*)(
                   struct std::__atomic_base<char8_t, true>&&)) &
              ::std::__atomic_base<char8_t, true>::operator=);

extern "C" struct std::__atomic_base<char8_t, true>*
__rust_thunk__e69cfcf4__ZNSt3__u13__atomic_baseIDuLb1EEaSERKS1_(
    struct std::__atomic_base<char8_t, true>* __this,
    struct std::__atomic_base<char8_t, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char8_t, true> &
               (::std::__atomic_base<char8_t, true>::*)(
                   struct std::__atomic_base<char8_t, true> const&)) &
              ::std::__atomic_base<char8_t, true>::operator=);

extern "C" void __rust_thunk__e6b4fa09__ZNSt3__u13__atomic_baseIDuLb1EEC1Ev(
    struct std::__atomic_base<char8_t, true>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct std::__atomic_base<signed char, true>) == 1);
static_assert(alignof(struct std::__atomic_base<signed char, true>) == 1);

extern "C" struct std::__atomic_base<signed char, true>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIaLb1EEaSEOS1_(
    struct std::__atomic_base<signed char, true>* __this,
    struct std::__atomic_base<signed char, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<signed char, true> &
               (::std::__atomic_base<signed char, true>::*)(
                   struct std::__atomic_base<signed char, true>&&)) &
              ::std::__atomic_base<signed char, true>::operator=);

extern "C" struct std::__atomic_base<signed char, true>*
__rust_thunk__e69cfcf4__ZNSt3__u13__atomic_baseIaLb1EEaSERKS1_(
    struct std::__atomic_base<signed char, true>* __this,
    struct std::__atomic_base<signed char, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<signed char, true> &
               (::std::__atomic_base<signed char, true>::*)(
                   struct std::__atomic_base<signed char, true> const&)) &
              ::std::__atomic_base<signed char, true>::operator=);

extern "C" void __rust_thunk__e6b4fa09__ZNSt3__u13__atomic_baseIaLb1EEC1Ev(
    struct std::__atomic_base<signed char, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__72113745__ZNSt3__u13__atomic_baseIaLb1EEC1Ea(
    struct std::__atomic_base<signed char, true>* __this, signed char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<char, true>) == 1);
static_assert(alignof(struct std::__atomic_base<char, true>) == 1);

extern "C" struct std::__atomic_base<char, true>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIcLb1EEaSEOS1_(
    struct std::__atomic_base<char, true>* __this,
    struct std::__atomic_base<char, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<char, true> &
               (::std::__atomic_base<char, true>::*)(
                   struct std::__atomic_base<char, true>&&)) &
              ::std::__atomic_base<char, true>::operator=);

extern "C" struct std::__atomic_base<char, true>*
__rust_thunk__e69cfcf4__ZNSt3__u13__atomic_baseIcLb1EEaSERKS1_(
    struct std::__atomic_base<char, true>* __this,
    struct std::__atomic_base<char, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<char, true> &
               (::std::__atomic_base<char, true>::*)(
                   struct std::__atomic_base<char, true> const&)) &
              ::std::__atomic_base<char, true>::operator=);

extern "C" void __rust_thunk__e6b4fa09__ZNSt3__u13__atomic_baseIcLb1EEC1Ev(
    struct std::__atomic_base<char, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__72113745__ZNSt3__u13__atomic_baseIcLb1EEC1Ec(
    struct std::__atomic_base<char, true>* __this, char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::__atomic_base<unsigned char, true>) == 1);
static_assert(alignof(struct std::__atomic_base<unsigned char, true>) == 1);

extern "C" struct std::__atomic_base<unsigned char, true>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIhLb1EEaSEOS1_(
    struct std::__atomic_base<unsigned char, true>* __this,
    struct std::__atomic_base<unsigned char, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<unsigned char, true> &
               (::std::__atomic_base<unsigned char, true>::*)(
                   struct std::__atomic_base<unsigned char, true>&&)) &
              ::std::__atomic_base<unsigned char, true>::operator=);

extern "C" struct std::__atomic_base<unsigned char, true>*
__rust_thunk__e69cfcf4__ZNSt3__u13__atomic_baseIhLb1EEaSERKS1_(
    struct std::__atomic_base<unsigned char, true>* __this,
    struct std::__atomic_base<unsigned char, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned char, true> &
               (::std::__atomic_base<unsigned char, true>::*)(
                   struct std::__atomic_base<unsigned char, true> const&)) &
              ::std::__atomic_base<unsigned char, true>::operator=);

extern "C" void __rust_thunk__e6b4fa09__ZNSt3__u13__atomic_baseIhLb1EEC1Ev(
    struct std::__atomic_base<unsigned char, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__72113745__ZNSt3__u13__atomic_baseIhLb1EEC1Eh(
    struct std::__atomic_base<unsigned char, true>* __this, unsigned char __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<int, true>) == 4);
static_assert(alignof(struct std::__atomic_base<int, true>) == 4);

extern "C" struct std::__atomic_base<int, true>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIiLb1EEaSEOS1_(
    struct std::__atomic_base<int, true>* __this,
    struct std::__atomic_base<int, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<int, true> &
               (::std::__atomic_base<int, true>::*)(
                   struct std::__atomic_base<int, true>&&)) &
              ::std::__atomic_base<int, true>::operator=);

extern "C" struct std::__atomic_base<int, true>*
__rust_thunk__e69cfcf4__ZNSt3__u13__atomic_baseIiLb1EEaSERKS1_(
    struct std::__atomic_base<int, true>* __this,
    struct std::__atomic_base<int, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<int, true> &
               (::std::__atomic_base<int, true>::*)(
                   struct std::__atomic_base<int, true> const&)) &
              ::std::__atomic_base<int, true>::operator=);

extern "C" void __rust_thunk__e6b4fa09__ZNSt3__u13__atomic_baseIiLb1EEC1Ev(
    struct std::__atomic_base<int, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__72113745__ZNSt3__u13__atomic_baseIiLb1EEC1Ei(
    struct std::__atomic_base<int, true>* __this, int __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned int, true>) ==
              4);
static_assert(alignof(struct std::__atomic_base<unsigned int, true>) == 4);

extern "C" struct std::__atomic_base<unsigned int, true>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIjLb1EEaSEOS1_(
    struct std::__atomic_base<unsigned int, true>* __this,
    struct std::__atomic_base<unsigned int, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<unsigned int, true> &
               (::std::__atomic_base<unsigned int, true>::*)(
                   struct std::__atomic_base<unsigned int, true>&&)) &
              ::std::__atomic_base<unsigned int, true>::operator=);

extern "C" struct std::__atomic_base<unsigned int, true>*
__rust_thunk__e69cfcf4__ZNSt3__u13__atomic_baseIjLb1EEaSERKS1_(
    struct std::__atomic_base<unsigned int, true>* __this,
    struct std::__atomic_base<unsigned int, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned int, true> &
               (::std::__atomic_base<unsigned int, true>::*)(
                   struct std::__atomic_base<unsigned int, true> const&)) &
              ::std::__atomic_base<unsigned int, true>::operator=);

extern "C" void __rust_thunk__e6b4fa09__ZNSt3__u13__atomic_baseIjLb1EEC1Ev(
    struct std::__atomic_base<unsigned int, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__72113745__ZNSt3__u13__atomic_baseIjLb1EEC1Ej(
    struct std::__atomic_base<unsigned int, true>* __this, unsigned int __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<long, true>) == 8);
static_assert(alignof(struct std::__atomic_base<long, true>) == 8);

extern "C" struct std::__atomic_base<long, true>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIlLb1EEaSEOS1_(
    struct std::__atomic_base<long, true>* __this,
    struct std::__atomic_base<long, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<long, true> &
               (::std::__atomic_base<long, true>::*)(
                   struct std::__atomic_base<long, true>&&)) &
              ::std::__atomic_base<long, true>::operator=);

extern "C" struct std::__atomic_base<long, true>*
__rust_thunk__e69cfcf4__ZNSt3__u13__atomic_baseIlLb1EEaSERKS1_(
    struct std::__atomic_base<long, true>* __this,
    struct std::__atomic_base<long, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<long, true> &
               (::std::__atomic_base<long, true>::*)(
                   struct std::__atomic_base<long, true> const&)) &
              ::std::__atomic_base<long, true>::operator=);

extern "C" void __rust_thunk__e6b4fa09__ZNSt3__u13__atomic_baseIlLb1EEC1Ev(
    struct std::__atomic_base<long, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__72113745__ZNSt3__u13__atomic_baseIlLb1EEC1El(
    struct std::__atomic_base<long, true>* __this, long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned long, true>) ==
              8);
static_assert(alignof(struct std::__atomic_base<unsigned long, true>) == 8);

extern "C" struct std::__atomic_base<unsigned long, true>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseImLb1EEaSEOS1_(
    struct std::__atomic_base<unsigned long, true>* __this,
    struct std::__atomic_base<unsigned long, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<unsigned long, true> &
               (::std::__atomic_base<unsigned long, true>::*)(
                   struct std::__atomic_base<unsigned long, true>&&)) &
              ::std::__atomic_base<unsigned long, true>::operator=);

extern "C" struct std::__atomic_base<unsigned long, true>*
__rust_thunk__e69cfcf4__ZNSt3__u13__atomic_baseImLb1EEaSERKS1_(
    struct std::__atomic_base<unsigned long, true>* __this,
    struct std::__atomic_base<unsigned long, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned long, true> &
               (::std::__atomic_base<unsigned long, true>::*)(
                   struct std::__atomic_base<unsigned long, true> const&)) &
              ::std::__atomic_base<unsigned long, true>::operator=);

extern "C" void __rust_thunk__e6b4fa09__ZNSt3__u13__atomic_baseImLb1EEC1Ev(
    struct std::__atomic_base<unsigned long, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__72113745__ZNSt3__u13__atomic_baseImLb1EEC1Em(
    struct std::__atomic_base<unsigned long, true>* __this, unsigned long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<short, true>) == 2);
static_assert(alignof(struct std::__atomic_base<short, true>) == 2);

extern "C" struct std::__atomic_base<short, true>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIsLb1EEaSEOS1_(
    struct std::__atomic_base<short, true>* __this,
    struct std::__atomic_base<short, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<short, true> &
               (::std::__atomic_base<short, true>::*)(
                   struct std::__atomic_base<short, true>&&)) &
              ::std::__atomic_base<short, true>::operator=);

extern "C" struct std::__atomic_base<short, true>*
__rust_thunk__e69cfcf4__ZNSt3__u13__atomic_baseIsLb1EEaSERKS1_(
    struct std::__atomic_base<short, true>* __this,
    struct std::__atomic_base<short, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<short, true> &
               (::std::__atomic_base<short, true>::*)(
                   struct std::__atomic_base<short, true> const&)) &
              ::std::__atomic_base<short, true>::operator=);

extern "C" void __rust_thunk__e6b4fa09__ZNSt3__u13__atomic_baseIsLb1EEC1Ev(
    struct std::__atomic_base<short, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__72113745__ZNSt3__u13__atomic_baseIsLb1EEC1Es(
    struct std::__atomic_base<short, true>* __this, short __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<unsigned short, true>) ==
              2);
static_assert(alignof(struct std::__atomic_base<unsigned short, true>) == 2);

extern "C" struct std::__atomic_base<unsigned short, true>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseItLb1EEaSEOS1_(
    struct std::__atomic_base<unsigned short, true>* __this,
    struct std::__atomic_base<unsigned short, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<unsigned short, true> &
               (::std::__atomic_base<unsigned short, true>::*)(
                   struct std::__atomic_base<unsigned short, true>&&)) &
              ::std::__atomic_base<unsigned short, true>::operator=);

extern "C" struct std::__atomic_base<unsigned short, true>*
__rust_thunk__e69cfcf4__ZNSt3__u13__atomic_baseItLb1EEaSERKS1_(
    struct std::__atomic_base<unsigned short, true>* __this,
    struct std::__atomic_base<unsigned short, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<unsigned short, true> &
               (::std::__atomic_base<unsigned short, true>::*)(
                   struct std::__atomic_base<unsigned short, true> const&)) &
              ::std::__atomic_base<unsigned short, true>::operator=);

extern "C" void __rust_thunk__e6b4fa09__ZNSt3__u13__atomic_baseItLb1EEC1Ev(
    struct std::__atomic_base<unsigned short, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__72113745__ZNSt3__u13__atomic_baseItLb1EEC1Et(
    struct std::__atomic_base<unsigned short, true>* __this,
    unsigned short __d) {
  crubit::construct_at(__this, __d);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<wchar_t, true>) == 4);
static_assert(alignof(struct std::__atomic_base<wchar_t, true>) == 4);

extern "C" struct std::__atomic_base<wchar_t, true>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIwLb1EEaSEOS1_(
    struct std::__atomic_base<wchar_t, true>* __this,
    struct std::__atomic_base<wchar_t, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<wchar_t, true> &
               (::std::__atomic_base<wchar_t, true>::*)(
                   struct std::__atomic_base<wchar_t, true>&&)) &
              ::std::__atomic_base<wchar_t, true>::operator=);

extern "C" struct std::__atomic_base<wchar_t, true>*
__rust_thunk__e69cfcf4__ZNSt3__u13__atomic_baseIwLb1EEaSERKS1_(
    struct std::__atomic_base<wchar_t, true>* __this,
    struct std::__atomic_base<wchar_t, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<wchar_t, true> &
               (::std::__atomic_base<wchar_t, true>::*)(
                   struct std::__atomic_base<wchar_t, true> const&)) &
              ::std::__atomic_base<wchar_t, true>::operator=);

extern "C" void __rust_thunk__e6b4fa09__ZNSt3__u13__atomic_baseIwLb1EEC1Ev(
    struct std::__atomic_base<wchar_t, true>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct std::__atomic_base<long long, true>) == 8);
static_assert(alignof(struct std::__atomic_base<long long, true>) == 8);

extern "C" struct std::__atomic_base<long long, true>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIxLb1EEaSEOS1_(
    struct std::__atomic_base<long long, true>* __this,
    struct std::__atomic_base<long long, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<long long, true> &
               (::std::__atomic_base<long long, true>::*)(
                   struct std::__atomic_base<long long, true>&&)) &
              ::std::__atomic_base<long long, true>::operator=);

extern "C" struct std::__atomic_base<long long, true>*
__rust_thunk__e69cfcf4__ZNSt3__u13__atomic_baseIxLb1EEaSERKS1_(
    struct std::__atomic_base<long long, true>* __this,
    struct std::__atomic_base<long long, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct std::__atomic_base<long long, true> &
               (::std::__atomic_base<long long, true>::*)(
                   struct std::__atomic_base<long long, true> const&)) &
              ::std::__atomic_base<long long, true>::operator=);

extern "C" void __rust_thunk__e6b4fa09__ZNSt3__u13__atomic_baseIxLb1EEC1Ev(
    struct std::__atomic_base<long long, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__72113745__ZNSt3__u13__atomic_baseIxLb1EEC1Ex(
    struct std::__atomic_base<long long, true>* __this, long long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(
    CRUBIT_SIZEOF(struct std::__atomic_base<unsigned long long, true>) == 8);
static_assert(alignof(struct std::__atomic_base<unsigned long long, true>) ==
              8);

extern "C" struct std::__atomic_base<unsigned long long, true>*
__rust_thunk__62b36b75__ZNSt3__u13__atomic_baseIyLb1EEaSEOS1_(
    struct std::__atomic_base<unsigned long long, true>* __this,
    struct std::__atomic_base<unsigned long long, true>* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct std::__atomic_base<unsigned long long, true> &
               (::std::__atomic_base<unsigned long long, true>::*)(
                   struct std::__atomic_base<unsigned long long, true>&&)) &
              ::std::__atomic_base<unsigned long long, true>::operator=);

extern "C" struct std::__atomic_base<unsigned long long, true>*
__rust_thunk__e69cfcf4__ZNSt3__u13__atomic_baseIyLb1EEaSERKS1_(
    struct std::__atomic_base<unsigned long long, true>* __this,
    struct std::__atomic_base<unsigned long long, true> const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert(
    (struct std::__atomic_base<unsigned long long, true> &
     (::std::__atomic_base<unsigned long long, true>::*)(
         struct std::__atomic_base<unsigned long long, true> const&)) &
    ::std::__atomic_base<unsigned long long, true>::operator=);

extern "C" void __rust_thunk__e6b4fa09__ZNSt3__u13__atomic_baseIyLb1EEC1Ev(
    struct std::__atomic_base<unsigned long long, true>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__72113745__ZNSt3__u13__atomic_baseIyLb1EEC1Ey(
    struct std::__atomic_base<unsigned long long, true>* __this,
    unsigned long long __d) {
  crubit::construct_at(__this, __d);
}

static_assert(sizeof(struct std::atomic<char8_t>) == 1);
static_assert(alignof(struct std::atomic<char8_t>) == 1);

extern "C" void
__rust_thunk__1ff537cd__ZNSt3__u6atomicIDuEC1EvQ26is_default_constructible_vIT_E(
    struct std::atomic<char8_t>* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct std::atomic<long>) == 8);
static_assert(alignof(struct std::atomic<long>) == 8);

extern "C" void
__rust_thunk__1ff537cd__ZNSt3__u6atomicIlEC1EvQ26is_default_constructible_vIT_E(
    struct std::atomic<long>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__6f568216__ZNSt3__u6atomicIlEC1El(
    struct std::atomic<long>* __this, long __d) {
  crubit::construct_at(__this, __d);
}

extern "C" long __rust_thunk__f67b7248__ZNSt3__u6atomicIlEaSEl(
    struct std::atomic<long>* __this, long __d) {
  return __this->operator=(__d);
}

static_assert((long (::std::atomic<long>::*)(long)) &
              ::std::atomic<long>::operator=);

static_assert(CRUBIT_SIZEOF(struct std::atomic<unsigned long>) == 8);
static_assert(alignof(struct std::atomic<unsigned long>) == 8);

extern "C" void
__rust_thunk__1ff537cd__ZNSt3__u6atomicImEC1EvQ26is_default_constructible_vIT_E(
    struct std::atomic<unsigned long>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__6f568216__ZNSt3__u6atomicImEC1Em(
    struct std::atomic<unsigned long>* __this, unsigned long __d) {
  crubit::construct_at(__this, __d);
}

extern "C" unsigned long __rust_thunk__f67b7248__ZNSt3__u6atomicImEaSEm(
    struct std::atomic<unsigned long>* __this, unsigned long __d) {
  return __this->operator=(__d);
}

static_assert((unsigned long (::std::atomic<unsigned long>::*)(unsigned long)) &
              ::std::atomic<unsigned long>::operator=);

static_assert(CRUBIT_SIZEOF(struct std::atomic<wchar_t>) == 4);
static_assert(alignof(struct std::atomic<wchar_t>) == 4);

extern "C" void
__rust_thunk__1ff537cd__ZNSt3__u6atomicIwEC1EvQ26is_default_constructible_vIT_E(
    struct std::atomic<wchar_t>* __this) {
  crubit::construct_at(__this);
}

static_assert(
    CRUBIT_SIZEOF(struct std::__allocation_result<char32_t*, unsigned long>) ==
    16);
static_assert(
    alignof(struct std::__allocation_result<char32_t*, unsigned long>) == 8);
static_assert(CRUBIT_OFFSET_OF(
                  ptr,
                  struct std::__allocation_result<char32_t*, unsigned long>) ==
              0);
static_assert(CRUBIT_OFFSET_OF(
                  count,
                  struct std::__allocation_result<char32_t*, unsigned long>) ==
              8);

extern "C" void
__rust_thunk__1e49e4b8__ZNSt3__u19__allocation_resultIPDimEC1ES1_m(
    struct std::__allocation_result<char32_t*, unsigned long>* __this,
    char32_t* __ptr, unsigned long __count) {
  crubit::construct_at(__this, __ptr, __count);
}

static_assert(
    CRUBIT_SIZEOF(struct std::__allocation_result<char16_t*, unsigned long>) ==
    16);
static_assert(
    alignof(struct std::__allocation_result<char16_t*, unsigned long>) == 8);
static_assert(CRUBIT_OFFSET_OF(
                  ptr,
                  struct std::__allocation_result<char16_t*, unsigned long>) ==
              0);
static_assert(CRUBIT_OFFSET_OF(
                  count,
                  struct std::__allocation_result<char16_t*, unsigned long>) ==
              8);

extern "C" void
__rust_thunk__1e49e4b8__ZNSt3__u19__allocation_resultIPDsmEC1ES1_m(
    struct std::__allocation_result<char16_t*, unsigned long>* __this,
    char16_t* __ptr, unsigned long __count) {
  crubit::construct_at(__this, __ptr, __count);
}

static_assert(
    CRUBIT_SIZEOF(struct std::__allocation_result<char*, unsigned long>) == 16);
static_assert(alignof(struct std::__allocation_result<char*, unsigned long>) ==
              8);
static_assert(CRUBIT_OFFSET_OF(
                  ptr, struct std::__allocation_result<char*, unsigned long>) ==
              0);
static_assert(CRUBIT_OFFSET_OF(
                  count,
                  struct std::__allocation_result<char*, unsigned long>) == 8);

extern "C" void
__rust_thunk__1e49e4b8__ZNSt3__u19__allocation_resultIPcmEC1ES1_m(
    struct std::__allocation_result<char*, unsigned long>* __this, char* __ptr,
    unsigned long __count) {
  crubit::construct_at(__this, __ptr, __count);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string_view<char32_t, std::char_traits<char32_t>>) ==
    16);
static_assert(
    alignof(
        class std::basic_string_view<char32_t, std::char_traits<char32_t>>) ==
    8);

extern "C" void
__rust_thunk__c6794005__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEC1Ev(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__607cfc11__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEC1EPKDi(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>* __this,
    char32_t const* __s) {
  crubit::construct_at(__this, __s);
}

extern "C" char32_t const*
__rust_thunk__a3b7ecc6__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5beginEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->begin();
}

static_assert(
    (char32_t const* (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::begin);

extern "C" char32_t const*
__rust_thunk__b3c512e7__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE3endEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->end();
}

static_assert(
    (char32_t const* (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::end);

extern "C" char32_t const*
__rust_thunk__c0007604__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6cbeginEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->cbegin();
}

static_assert(
    (char32_t const* (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::cbegin);

extern "C" char32_t const*
__rust_thunk__6aa06a54__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4cendEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->cend();
}

static_assert(
    (char32_t const* (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::cend);

extern "C" void
__rust_thunk__4ad9b4be__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6rbeginEv(
    class std::reverse_iterator<const char32_t*>* __return,
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  new (__return) auto(__this->rbegin());
}

static_assert(
    (class std::reverse_iterator<const char32_t*> (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::rbegin);

extern "C" void
__rust_thunk__38efc66f__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4rendEv(
    class std::reverse_iterator<const char32_t*>* __return,
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  new (__return) auto(__this->rend());
}

static_assert(
    (class std::reverse_iterator<const char32_t*> (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::rend);

extern "C" void
__rust_thunk__52def44c__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE7crbeginEv(
    class std::reverse_iterator<const char32_t*>* __return,
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  new (__return) auto(__this->crbegin());
}

static_assert(
    (class std::reverse_iterator<const char32_t*> (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::crbegin);

extern "C" void
__rust_thunk__973619b6__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5crendEv(
    class std::reverse_iterator<const char32_t*>* __return,
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  new (__return) auto(__this->crend());
}

static_assert(
    (class std::reverse_iterator<const char32_t*> (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::crend);

extern "C" size_t
__rust_thunk__6169413b__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4sizeEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->size();
}

static_assert(
    (size_t (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::size);

extern "C" size_t
__rust_thunk__545bbca3__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6lengthEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->length();
}

static_assert(
    (size_t (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::length);

extern "C" size_t
__rust_thunk__e11192f8__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE8max_sizeEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->max_size();
}

static_assert(
    (size_t (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::max_size);

extern "C" bool
__rust_thunk__56cf550f__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5emptyEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::empty);

extern "C" char32_t const*
__rust_thunk__4d296543__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEixEm(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this,
    size_t __pos) {
  return std::addressof(__this->operator[](__pos));
}

static_assert(
    (char32_t const& (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)(
        size_t) const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::operator[]);

extern "C" char32_t const*
__rust_thunk__55c0bcd4__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE2atEm(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this,
    size_t __pos) {
  return std::addressof(__this->at(__pos));
}

static_assert(
    (char32_t const& (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)(
        size_t) const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::at);

extern "C" char32_t const*
__rust_thunk__d835e380__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5frontEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return std::addressof(__this->front());
}

static_assert(
    (char32_t const& (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::front);

extern "C" char32_t const*
__rust_thunk__e1cff164__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4backEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return std::addressof(__this->back());
}

static_assert(
    (char32_t const& (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::back);

extern "C" char32_t const*
__rust_thunk__32365d7e__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4dataEv(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this) {
  return __this->data();
}

static_assert(
    (char32_t const* (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)()
         const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::data);

extern "C" void
__rust_thunk__d3f89ef7__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE13remove_prefixEm(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>* __this,
    size_t __n) {
  __this->remove_prefix(__n);
}

static_assert(
    (void (::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)(
        size_t)) &
    ::std::basic_string_view<char32_t,
                             std::char_traits<char32_t>>::remove_prefix);

extern "C" void
__rust_thunk__e7e427dc__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE13remove_suffixEm(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>* __this,
    size_t __n) {
  __this->remove_suffix(__n);
}

static_assert(
    (void (::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)(
        size_t)) &
    ::std::basic_string_view<char32_t,
                             std::char_traits<char32_t>>::remove_suffix);

extern "C" void
__rust_thunk__25969ade__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4swapERS3_(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>* __this,
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>*
        __other) {
  __this->swap(*__other);
}

static_assert(
    (void (::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)(
        class std::basic_string_view<char32_t, std::char_traits<char32_t>>&)) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::swap);

extern "C" size_t
__rust_thunk__edded9eb__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4copyEPDimm(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this,
    char32_t* __s, size_t __n, size_t __pos) {
  return __this->copy(__s, __n, __pos);
}

static_assert(
    (size_t (::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)(
        char32_t*, size_t, size_t) const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::copy);

extern "C" void
__rust_thunk__84f73dc7__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6substrEmm(
    class std::basic_string_view<char32_t, std::char_traits<char32_t>>*
        __return,
    class std::basic_string_view<char32_t, std::char_traits<char32_t>> const*
        __this,
    size_t __pos, size_t __n) {
  new (__return) auto(__this->substr(__pos, __n));
}

static_assert(
    (class std::basic_string_view<char32_t, std::char_traits<char32_t>> (
        ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::*)(
        size_t, size_t) const) &
    ::std::basic_string_view<char32_t, std::char_traits<char32_t>>::substr);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string_view<char16_t, std::char_traits<char16_t>>) ==
    16);
static_assert(
    alignof(
        class std::basic_string_view<char16_t, std::char_traits<char16_t>>) ==
    8);

extern "C" void
__rust_thunk__c6794005__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEC1Ev(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>*
        __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__607cfc11__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEC1EPKDs(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>* __this,
    char16_t const* __s) {
  crubit::construct_at(__this, __s);
}

extern "C" char16_t const*
__rust_thunk__a3b7ecc6__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5beginEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->begin();
}

static_assert(
    (char16_t const* (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::begin);

extern "C" char16_t const*
__rust_thunk__b3c512e7__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE3endEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->end();
}

static_assert(
    (char16_t const* (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::end);

extern "C" char16_t const*
__rust_thunk__c0007604__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6cbeginEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->cbegin();
}

static_assert(
    (char16_t const* (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::cbegin);

extern "C" char16_t const*
__rust_thunk__6aa06a54__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4cendEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->cend();
}

static_assert(
    (char16_t const* (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::cend);

extern "C" void
__rust_thunk__4ad9b4be__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6rbeginEv(
    class std::reverse_iterator<const char16_t*>* __return,
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  new (__return) auto(__this->rbegin());
}

static_assert(
    (class std::reverse_iterator<const char16_t*> (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::rbegin);

extern "C" void
__rust_thunk__38efc66f__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4rendEv(
    class std::reverse_iterator<const char16_t*>* __return,
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  new (__return) auto(__this->rend());
}

static_assert(
    (class std::reverse_iterator<const char16_t*> (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::rend);

extern "C" void
__rust_thunk__52def44c__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE7crbeginEv(
    class std::reverse_iterator<const char16_t*>* __return,
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  new (__return) auto(__this->crbegin());
}

static_assert(
    (class std::reverse_iterator<const char16_t*> (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::crbegin);

extern "C" void
__rust_thunk__973619b6__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5crendEv(
    class std::reverse_iterator<const char16_t*>* __return,
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  new (__return) auto(__this->crend());
}

static_assert(
    (class std::reverse_iterator<const char16_t*> (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::crend);

extern "C" size_t
__rust_thunk__6169413b__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4sizeEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->size();
}

static_assert(
    (size_t (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::size);

extern "C" size_t
__rust_thunk__545bbca3__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6lengthEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->length();
}

static_assert(
    (size_t (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::length);

extern "C" size_t
__rust_thunk__e11192f8__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE8max_sizeEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->max_size();
}

static_assert(
    (size_t (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::max_size);

extern "C" bool
__rust_thunk__56cf550f__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5emptyEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::empty);

extern "C" char16_t const*
__rust_thunk__4d296543__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEixEm(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this,
    size_t __pos) {
  return std::addressof(__this->operator[](__pos));
}

static_assert(
    (char16_t const& (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)(
        size_t) const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::operator[]);

extern "C" char16_t const*
__rust_thunk__55c0bcd4__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE2atEm(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this,
    size_t __pos) {
  return std::addressof(__this->at(__pos));
}

static_assert(
    (char16_t const& (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)(
        size_t) const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::at);

extern "C" char16_t const*
__rust_thunk__d835e380__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5frontEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return std::addressof(__this->front());
}

static_assert(
    (char16_t const& (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::front);

extern "C" char16_t const*
__rust_thunk__e1cff164__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4backEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return std::addressof(__this->back());
}

static_assert(
    (char16_t const& (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::back);

extern "C" char16_t const*
__rust_thunk__32365d7e__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4dataEv(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this) {
  return __this->data();
}

static_assert(
    (char16_t const* (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)()
         const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::data);

extern "C" void
__rust_thunk__d3f89ef7__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE13remove_prefixEm(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>* __this,
    size_t __n) {
  __this->remove_prefix(__n);
}

static_assert(
    (void (::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)(
        size_t)) &
    ::std::basic_string_view<char16_t,
                             std::char_traits<char16_t>>::remove_prefix);

extern "C" void
__rust_thunk__e7e427dc__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE13remove_suffixEm(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>* __this,
    size_t __n) {
  __this->remove_suffix(__n);
}

static_assert(
    (void (::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)(
        size_t)) &
    ::std::basic_string_view<char16_t,
                             std::char_traits<char16_t>>::remove_suffix);

extern "C" void
__rust_thunk__25969ade__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4swapERS3_(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>* __this,
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>*
        __other) {
  __this->swap(*__other);
}

static_assert(
    (void (::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)(
        class std::basic_string_view<char16_t, std::char_traits<char16_t>>&)) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::swap);

extern "C" size_t
__rust_thunk__edded9eb__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4copyEPDsmm(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this,
    char16_t* __s, size_t __n, size_t __pos) {
  return __this->copy(__s, __n, __pos);
}

static_assert(
    (size_t (::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)(
        char16_t*, size_t, size_t) const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::copy);

extern "C" void
__rust_thunk__84f73dc7__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6substrEmm(
    class std::basic_string_view<char16_t, std::char_traits<char16_t>>*
        __return,
    class std::basic_string_view<char16_t, std::char_traits<char16_t>> const*
        __this,
    size_t __pos, size_t __n) {
  new (__return) auto(__this->substr(__pos, __n));
}

static_assert(
    (class std::basic_string_view<char16_t, std::char_traits<char16_t>> (
        ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::*)(
        size_t, size_t) const) &
    ::std::basic_string_view<char16_t, std::char_traits<char16_t>>::substr);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>) ==
    16);
static_assert(
    alignof(class std::basic_string_view<char8_t, std::char_traits<char8_t>>) ==
    8);

extern "C" void
__rust_thunk__c6794005__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEC1Ev(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__4ad9b4be__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6rbeginEv(
    class std::reverse_iterator<const char8_t*>* __return,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  new (__return) auto(__this->rbegin());
}

static_assert(
    (class std::reverse_iterator<const char8_t*> (
        ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::rbegin);

extern "C" void
__rust_thunk__38efc66f__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4rendEv(
    class std::reverse_iterator<const char8_t*>* __return,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  new (__return) auto(__this->rend());
}

static_assert(
    (class std::reverse_iterator<const char8_t*> (
        ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::rend);

extern "C" void
__rust_thunk__52def44c__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE7crbeginEv(
    class std::reverse_iterator<const char8_t*>* __return,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  new (__return) auto(__this->crbegin());
}

static_assert(
    (class std::reverse_iterator<const char8_t*> (
        ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::crbegin);

extern "C" void
__rust_thunk__973619b6__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5crendEv(
    class std::reverse_iterator<const char8_t*>* __return,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  new (__return) auto(__this->crend());
}

static_assert(
    (class std::reverse_iterator<const char8_t*> (
        ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::crend);

extern "C" size_t
__rust_thunk__6169413b__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4sizeEv(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  return __this->size();
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::size);

extern "C" size_t
__rust_thunk__545bbca3__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6lengthEv(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  return __this->length();
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::length);

extern "C" size_t
__rust_thunk__e11192f8__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE8max_sizeEv(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  return __this->max_size();
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::max_size);

extern "C" bool
__rust_thunk__56cf550f__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5emptyEv(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this) {
  return __this->empty();
}

static_assert(
    (bool (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)()
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::empty);

extern "C" void
__rust_thunk__d3f89ef7__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13remove_prefixEm(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __this,
    size_t __n) {
  __this->remove_prefix(__n);
}

static_assert(
    (void (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        size_t)) &
    ::std::basic_string_view<char8_t,
                             std::char_traits<char8_t>>::remove_prefix);

extern "C" void
__rust_thunk__e7e427dc__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13remove_suffixEm(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __this,
    size_t __n) {
  __this->remove_suffix(__n);
}

static_assert(
    (void (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        size_t)) &
    ::std::basic_string_view<char8_t,
                             std::char_traits<char8_t>>::remove_suffix);

extern "C" void
__rust_thunk__25969ade__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4swapERS3_(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __other) {
  __this->swap(*__other);
}

static_assert(
    (void (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>&)) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::swap);

extern "C" void
__rust_thunk__84f73dc7__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6substrEmm(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __return,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    size_t __pos, size_t __n) {
  new (__return) auto(__this->substr(__pos, __n));
}

static_assert(
    (class std::basic_string_view<char8_t, std::char_traits<char8_t>> (
        ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(size_t,
                                                                         size_t)
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::substr);

extern "C" size_t
__rust_thunk__80c57053__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4findES3_m(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __s,
    size_t __pos) {
  return __this->find(std::move(*__s), __pos);
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>,
        size_t) const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::find);

extern "C" size_t
__rust_thunk__1e81e6dc__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5rfindES3_m(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __s,
    size_t __pos) {
  return __this->rfind(std::move(*__s), __pos);
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>,
        size_t) const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::rfind);

extern "C" size_t
__rust_thunk__ca228fa0__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13find_first_ofES3_m(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __s,
    size_t __pos) {
  return __this->find_first_of(std::move(*__s), __pos);
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>,
        size_t) const) &
    ::std::basic_string_view<char8_t,
                             std::char_traits<char8_t>>::find_first_of);

extern "C" size_t
__rust_thunk__107c7186__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE12find_last_ofES3_m(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __s,
    size_t __pos) {
  return __this->find_last_of(std::move(*__s), __pos);
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>,
        size_t) const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::find_last_of);

extern "C" size_t
__rust_thunk__a64bc5cb__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE17find_first_not_ofES3_m(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __s,
    size_t __pos) {
  return __this->find_first_not_of(std::move(*__s), __pos);
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>,
        size_t) const) &
    ::std::basic_string_view<char8_t,
                             std::char_traits<char8_t>>::find_first_not_of);

extern "C" size_t
__rust_thunk__cef24cb7__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE16find_last_not_ofES3_m(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __s,
    size_t __pos) {
  return __this->find_last_not_of(std::move(*__s), __pos);
}

static_assert(
    (size_t (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>,
        size_t) const) &
    ::std::basic_string_view<char8_t,
                             std::char_traits<char8_t>>::find_last_not_of);

extern "C" bool
__rust_thunk__17a496f2__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE11starts_withES3_(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __s) {
  return __this->starts_with(std::move(*__s));
}

static_assert(
    (bool (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>)
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::starts_with);

extern "C" bool
__rust_thunk__0633de84__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE9ends_withES3_(
    class std::basic_string_view<char8_t, std::char_traits<char8_t>> const*
        __this,
    class std::basic_string_view<char8_t, std::char_traits<char8_t>>* __s) {
  return __this->ends_with(std::move(*__s));
}

static_assert(
    (bool (::std::basic_string_view<char8_t, std::char_traits<char8_t>>::*)(
        class std::basic_string_view<char8_t, std::char_traits<char8_t>>)
         const) &
    ::std::basic_string_view<char8_t, std::char_traits<char8_t>>::ends_with);

#pragma clang diagnostic pop

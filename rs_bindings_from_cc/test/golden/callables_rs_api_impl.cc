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

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/callables.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __CcTemplateInstN6rs_std11DynCallableIFvvEEE(
    ::rs_std::internal_dyn_callable::UnmanagedZeroableCallable* state);

extern "C" void __CcTemplateInstN6rs_std11DynCallableIFvvOEEE(
    ::rs_std::internal_dyn_callable::UnmanagedZeroableCallable* state);

extern "C" void __CcTemplateInstN6rs_std11DynCallableIKF13ABICompatibleS1_EEE(
    ::rs_std::internal_dyn_callable::UnmanagedZeroableCallable* state,
    struct ABICompatible* param_0, struct ABICompatible* out);

extern "C" void
__CcTemplateInstN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEE(
    ::rs_std::internal_dyn_callable::UnmanagedZeroableCallable* state,
    class LayoutCompatible* param_0, class LayoutCompatible* out);

extern "C" void __CcTemplateInstN6rs_std11DynCallableIKF7BridgedS1_EEE(
    ::rs_std::internal_dyn_callable::UnmanagedZeroableCallable* state,
    unsigned char* param_0, unsigned char* out);

extern "C" int __CcTemplateInstN6rs_std11DynCallableIKFiiEEE(
    ::rs_std::internal_dyn_callable::UnmanagedZeroableCallable* state,
    int param_0);

extern "C" void __CcTemplateInstN6rs_std11DynCallableIKFvvEEE(
    ::rs_std::internal_dyn_callable::UnmanagedZeroableCallable* state);

extern "C" void __rust_thunk___Z11invoke_onceN6rs_std11DynCallableIFvvOEEE(
    const unsigned char* f) {
  invoke_once(::crubit::internal::Decode<
              ::rs_std::internal_dyn_callable::DynCallableAbi<void() &&>>(
      ::rs_std::internal_dyn_callable::DynCallableAbi<void() &&>(
          &__CcTemplateInstN6rs_std11DynCallableIFvvOEEE),
      f));
}

static_assert((void (*)(class rs_std::DynCallable<void() &&>)) & ::invoke_once);

extern "C" void __rust_thunk___Z6invokeN6rs_std11DynCallableIFvvEEE(
    const unsigned char* f) {
  invoke(::crubit::internal::Decode<
         ::rs_std::internal_dyn_callable::DynCallableAbi<void()>>(
      ::rs_std::internal_dyn_callable::DynCallableAbi<void()>(
          &__CcTemplateInstN6rs_std11DynCallableIFvvEEE),
      f));
}

static_assert((void (*)(class rs_std::DynCallable<void()>)) & ::invoke);

extern "C" void __rust_thunk___Z12invoke_constN6rs_std11DynCallableIKFvvEEE(
    const unsigned char* f) {
  invoke_const(::crubit::internal::Decode<
               ::rs_std::internal_dyn_callable::DynCallableAbi<void() const>>(
      ::rs_std::internal_dyn_callable::DynCallableAbi<void() const>(
          &__CcTemplateInstN6rs_std11DynCallableIKFvvEEE),
      f));
}

static_assert((void (*)(class rs_std::DynCallable<void() const>)) &
              ::invoke_const);

extern "C" int __rust_thunk___Z7map_intN6rs_std11DynCallableIKFiiEEEi(
    const unsigned char* f, int arg) {
  return map_int(
      ::crubit::internal::Decode<
          ::rs_std::internal_dyn_callable::DynCallableAbi<int(int) const>>(
          ::rs_std::internal_dyn_callable::DynCallableAbi<int(int) const>(
              &__CcTemplateInstN6rs_std11DynCallableIKFiiEEE),
          f),
      arg);
}

static_assert((int (*)(class rs_std::DynCallable<int(int) const>, int)) &
              ::map_int);

extern "C" void
__rust_thunk___Z11map_bridgedN6rs_std11DynCallableIKF7BridgedS1_EEES1_(
    unsigned char* __return_abi_buffer, const unsigned char* f,
    const unsigned char* arg) {
  ::crubit::internal::Encode<::crubit::BridgedAbi>(
      ::crubit::BridgedAbi(), __return_abi_buffer,
      map_bridged(
          ::crubit::internal::Decode<
              ::rs_std::internal_dyn_callable::DynCallableAbi<struct Bridged(
                  struct Bridged) const>>(
              ::rs_std::internal_dyn_callable::DynCallableAbi<struct Bridged(
                  struct Bridged) const>(
                  [](::rs_std::internal_dyn_callable::UnmanagedZeroableCallable*
                         state,
                     struct Bridged param_0) -> struct Bridged {
                    unsigned char bridge_param_0[::crubit::BridgedAbi::kSize];
                    ::crubit::internal::Encode(::crubit::BridgedAbi(),
                                               bridge_param_0, param_0);
                    unsigned char out[::crubit::BridgedAbi::kSize];
                    __CcTemplateInstN6rs_std11DynCallableIKF7BridgedS1_EEE(
                        state, bridge_param_0, out);
                    return ::crubit::internal::Decode<::crubit::BridgedAbi>(
                        ::crubit::BridgedAbi(), out);
                  }),
              f),
          ::crubit::internal::Decode<::crubit::BridgedAbi>(
              ::crubit::BridgedAbi(), arg)));
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
  new (__return) auto(map_abi_compatible(
      ::crubit::internal::Decode<
          ::rs_std::internal_dyn_callable::DynCallableAbi<struct ABICompatible(
              struct ABICompatible) const>>(
          ::rs_std::internal_dyn_callable::DynCallableAbi<struct ABICompatible(
              struct ABICompatible) const>(
              [](::rs_std::internal_dyn_callable::UnmanagedZeroableCallable*
                     state,
                 struct ABICompatible param_0) -> struct ABICompatible {
                ::crubit::Slot<struct ABICompatible> stack_param_0(
                    std::move(param_0));
                ::crubit::Slot<struct ABICompatible> out;
                __CcTemplateInstN6rs_std11DynCallableIKF13ABICompatibleS1_EEE(
                    state, stack_param_0.Get(), out.Get());
                return std::move(out).AssumeInitAndTakeValue();
              }),
          f),
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

static_assert((int (LayoutCompatible::*)() const) & ::LayoutCompatible::get);

extern "C" void
__rust_thunk___Z21map_layout_compatibleN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEES1_(
    class LayoutCompatible* __return, const unsigned char* f,
    class LayoutCompatible* arg) {
  new (__return) auto(map_layout_compatible(
      ::crubit::internal::Decode<
          ::rs_std::internal_dyn_callable::DynCallableAbi<
              class LayoutCompatible(class LayoutCompatible) const>>(
          ::rs_std::internal_dyn_callable::DynCallableAbi<
              class LayoutCompatible(class LayoutCompatible)
                  const>([](::rs_std::internal_dyn_callable::
                                UnmanagedZeroableCallable* state,
                            class LayoutCompatible param_0)
                             -> class LayoutCompatible {
            ::crubit::Slot<class LayoutCompatible> stack_param_0(
                std::move(param_0));
            ::crubit::Slot<class LayoutCompatible> out;
            __CcTemplateInstN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEE(
                state, stack_param_0.Get(), out.Get());
            return std::move(out).AssumeInitAndTakeValue();
          }),
          f),
      std::move(*arg)));
}

static_assert(
    (class LayoutCompatible (*)(
        class rs_std::DynCallable<LayoutCompatible(LayoutCompatible) const>,
        class LayoutCompatible)) &
    ::map_layout_compatible);

#pragma clang diagnostic pop

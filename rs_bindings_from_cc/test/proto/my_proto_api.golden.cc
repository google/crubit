// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/proto:my_proto_api

#include "support/bridge.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/fmt.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"
#include "support/rs_std/lossy_formatter_for_bindings.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/proto/my_proto_api.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___ZN4test11MakeRequestEl(
    unsigned char* __return_abi_buffer, int64_t num) {
  ::crubit::Encoder __return_encoder(
      ::crubit::BoxedAbi<my_package::MyMessage_Request>::kSize,
      __return_abi_buffer);
  ::crubit::BoxedAbi<my_package::MyMessage_Request>().Encode(
      test::MakeRequest(num), __return_encoder);
}

static_assert((class my_package::MyMessage_Request (*)(int64_t)) &
              ::test::MakeRequest);

extern "C" void __rust_thunk___ZN4test11ReturnValueEv(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::BoxedAbi<my_package::MyMessage>::kSize, __return_abi_buffer);
  ::crubit::BoxedAbi<my_package::MyMessage>().Encode(test::ReturnValue(),
                                                     __return_encoder);
}

static_assert((class my_package::MyMessage (*)()) & ::test::ReturnValue);

extern "C" int64_t
__rust_thunk___ZN4test16ExtractFromValueEN10my_package9MyMessageE(
    const unsigned char* msg) {
  ::crubit::Decoder __msg_decoder(
      ::crubit::BoxedAbi<my_package::MyMessage>::kSize, msg);
  return test::ExtractFromValue(
      ::crubit::BoxedAbi<my_package::MyMessage>().Decode(__msg_decoder));
}

static_assert((int64_t (*)(class my_package::MyMessage)) &
              ::test::ExtractFromValue);

extern "C" int64_t
__rust_thunk___ZN4test19ExtractFromConstPtrEPKN10my_package9MyMessageE(
    my_package::MyMessage const* msg) {
  return test::ExtractFromConstPtr(msg);
}

static_assert((int64_t (*)(my_package::MyMessage const*)) &
              ::test::ExtractFromConstPtr);

extern "C" int64_t
__rust_thunk___ZN4test19ExtractFromConstRefERKN10my_package9MyMessageE(
    my_package::MyMessage const* msg) {
  return test::ExtractFromConstRef(*msg);
}

static_assert((int64_t (*)(my_package::MyMessage const&)) &
              ::test::ExtractFromConstRef);

extern "C" int64_t
__rust_thunk___ZN4test21ExtractFromMutablePtrEPN10my_package9MyMessageE(
    my_package::MyMessage* msg) {
  return test::ExtractFromMutablePtr(msg);
}

static_assert((int64_t (*)(my_package::MyMessage*)) &
              ::test::ExtractFromMutablePtr);

extern "C" int64_t
__rust_thunk___ZN4test21ExtractFromMutableRefERN10my_package9MyMessageE(
    my_package::MyMessage* msg) {
  return test::ExtractFromMutableRef(*msg);
}

static_assert((int64_t (*)(my_package::MyMessage&)) &
              ::test::ExtractFromMutableRef);

extern "C" my_package::MyMessage* __rust_thunk___ZN4test12GetMutMsgPtrEv() {
  return test::GetMutMsgPtr();
}

static_assert((my_package::MyMessage * (*)()) & ::test::GetMutMsgPtr);

extern "C" my_package::MyMessage const*
__rust_thunk___ZN4test14GetConstMsgPtrEv() {
  return test::GetConstMsgPtr();
}

static_assert((my_package::MyMessage const* (*)()) & ::test::GetConstMsgPtr);

#pragma clang diagnostic pop

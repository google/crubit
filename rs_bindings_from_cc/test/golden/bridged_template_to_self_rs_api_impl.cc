// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:bridged_template_to_self_cc

#include "support/bridge.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/bridged_template_to_self.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct ns::Member) == 1);
static_assert(alignof(struct ns::Member) == 1);

extern "C" void __rust_thunk___ZN2ns6MemberC1ENS_5MyBoxINS_11InstructionEEE(
    struct ns::Member* __this, const unsigned char* instruction) {
  ::crubit::Decoder __instruction_decoder(
      ::crubit::MyBoxAbi<::crubit::TransmuteAbi<::ns::Instruction>>::kSize,
      instruction);
  crubit::construct_at(
      __this, ::crubit::MyBoxAbi<::crubit::TransmuteAbi<::ns::Instruction>>(
                  ::crubit::TransmuteAbi<::ns::Instruction>())
                  .Decode(__instruction_decoder));
}

static_assert(sizeof(class ns::Instruction) == 1);
static_assert(alignof(class ns::Instruction) == 1);

extern "C" void __rust_thunk___ZN2ns11InstructionC1Ev(
    class ns::Instruction* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN2ns11Instruction6CreateEPNS_6RegionE(
    unsigned char* __return_abi_buffer, class ns::Region* parent) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyBoxAbi<::crubit::TransmuteAbi<::ns::Instruction>>::kSize,
      __return_abi_buffer);
  ::crubit::MyBoxAbi<::crubit::TransmuteAbi<::ns::Instruction>>(
      ::crubit::TransmuteAbi<::ns::Instruction>())
      .Encode(ns::Instruction::Create(parent), __return_encoder);
}

static_assert((struct ns::MyBox<class ns::Instruction> (*)(class ns::Region*)) &
              ::ns::Instruction::Create);

static_assert(sizeof(class ns::Region) == 1);
static_assert(alignof(class ns::Region) == 1);

extern "C" void __rust_thunk___ZN2ns6RegionC1Ev(class ns::Region* __this) {
  crubit::construct_at(__this);
}

extern "C" class ns::Instruction*
__rust_thunk___ZN2ns6Region6AppendENS_5MyBoxINS_11InstructionEEE(
    class ns::Region* __this, const unsigned char* instruction) {
  ::crubit::Decoder __instruction_decoder(
      ::crubit::MyBoxAbi<::crubit::TransmuteAbi<::ns::Instruction>>::kSize,
      instruction);
  return __this->Append(
      ::crubit::MyBoxAbi<::crubit::TransmuteAbi<::ns::Instruction>>(
          ::crubit::TransmuteAbi<::ns::Instruction>())
          .Decode(__instruction_decoder));
}

static_assert((class ns::Instruction *
               (::ns::Region::*)(struct ns::MyBox<class ns::Instruction>)) &
              ::ns::Region::Append);

extern "C" void __rust_thunk___ZN2ns6Region3PopEv(
    unsigned char* __return_abi_buffer, class ns::Region* __this) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyBoxAbi<::crubit::TransmuteAbi<::ns::Instruction>>::kSize,
      __return_abi_buffer);
  ::crubit::MyBoxAbi<::crubit::TransmuteAbi<::ns::Instruction>>(
      ::crubit::TransmuteAbi<::ns::Instruction>())
      .Encode(__this->Pop(), __return_encoder);
}

static_assert((struct ns::MyBox<class ns::Instruction> (::ns::Region::*)()) &
              ::ns::Region::Pop);

#pragma clang diagnostic pop

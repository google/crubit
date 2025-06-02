// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_STATUS_BRIDGE_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_STATUS_BRIDGE_H_

#include "support/bridge.h"

#include <cstddef>
#include <cstdint>

#include "absl/status/status.h"
#include "absl/status/statusor.h"

namespace crubit {

struct StatusAbi {
  using Value = absl::Status;
  static constexpr size_t kSize = sizeof(uintptr_t);
  static void Encode(Value value, Encoder& encoder);
  static Value Decode(Decoder& decoder);
};

template <typename Abi>
struct StatusOrAbi {
  static_assert(is_crubit_abi<Abi>,
                "StatusOrAbi requires Abi to be is_crubit_abi");
  using Value = absl::StatusOr<typename Abi::Value>;
  static constexpr size_t kSize = StatusAbi::kSize + Abi::kSize;
  static void Encode(Value value, Encoder& encoder) {
    encoder.Encode<StatusAbi>(value.status());
    if (value.ok()) {
      encoder.Encode<Abi>(*std::move(value));
    }
  }
  static Value Decode(Decoder& decoder) {
    absl::Status status(decoder.Decode<StatusAbi>());
    if (status.ok()) {
      return decoder.Decode<Abi>();
    } else {
      return status;
    }
  }
};

}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_BRIDGE_REMOTE_STATUS_BRIDGE_H_

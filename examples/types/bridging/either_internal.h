// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_BRIDGING_EITHER_INTERNAL_H_
#define THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_BRIDGING_EITHER_INTERNAL_H_

#include "support/annotations.h"
#include "support/bridge.h"

#include <cstddef>

namespace either {

template <typename L, typename R>
struct CRUBIT_BRIDGE("either::Either", "either::EitherAbi",
                     "either::internal::EitherAbi") Either;

namespace internal {

template <typename LeftAbi, typename RightAbi>
  requires(crubit::is_crubit_abi<LeftAbi> && crubit::is_crubit_abi<RightAbi>)
struct EitherAbi {
  using Value = Either<typename LeftAbi::Value, typename RightAbi::Value>;

  static constexpr size_t kSize =
      sizeof(bool) +
      (LeftAbi::kSize > RightAbi::kSize ? LeftAbi::kSize : RightAbi::kSize);

  static void Encode(Value value, crubit::Encoder& encoder) {
    if (value.is_left) {
      encoder.EncodeTransmute(true);
      encoder.Encode<LeftAbi>(std::move(value.left));
    } else {
      encoder.EncodeTransmute(false);
      encoder.Encode<RightAbi>(std::move(value.right));
    }
  }

  static Value Decode(crubit::Decoder& decoder) {
    if (decoder.DecodeTransmute<bool>()) {
      return {
          .is_left = true,
          .left = decoder.Decode<LeftAbi>(),
      };
    } else {
      return {
          .is_left = false,
          .right = decoder.Decode<RightAbi>(),
      };
    }
  }

  LeftAbi left_abi;
  RightAbi right_abi;
};

template <typename L, typename R>
EitherAbi(L, R) -> EitherAbi<L, R>;

}  // namespace internal
}  // namespace either

#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_BRIDGING_EITHER_INTERNAL_H_

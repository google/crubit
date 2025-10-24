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

  void Encode(Value value, crubit::Encoder& encoder) && {
    if (value.is_left) {
      crubit::TransmuteAbi<bool>().Encode(true, encoder);
      std::move(left_abi).Encode(std::move(value.left), encoder);
    } else {
      crubit::TransmuteAbi<bool>().Encode(false, encoder);
      std::move(right_abi).Encode(std::move(value.right), encoder);
    }
  }

  Value Decode(crubit::Decoder& decoder) && {
    if (crubit::TransmuteAbi<bool>().Decode(decoder)) {
      return {
          .is_left = true,
          .left = std::move(left_abi).Decode(decoder),
      };
    } else {
      return {
          .is_left = false,
          .right = std::move(right_abi).Decode(decoder),
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

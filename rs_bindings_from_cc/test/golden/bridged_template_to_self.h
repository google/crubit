// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BRIDGED_TEMPLATE_TO_SELF_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BRIDGED_TEMPLATE_TO_SELF_H_

namespace ns {

// A bridged template, like `std::unique_ptr<T>`: importing `MyBox<T>`
// converts `T` before `MyBox<T>` itself is marked as imported.
template <typename T>
// clang-format off
struct
    [[clang::annotate("crubit_bridge_rust_name", "MyBox")]]
    [[clang::annotate("crubit_bridge_abi_rust", "MyBoxAbi")]]
    [[clang::annotate("crubit_bridge_abi_cpp", "::crubit::MyBoxAbi")]]
// clang-format on
MyBox {
  T* ptr;
};

class Instruction;
class Region;

// `Member` mentions `MyBox<Instruction>` before `Instruction` is defined.
// Importing that specialization imports `Instruction`, whose methods in turn
// mention `MyBox<Instruction>` again, while it is still being imported.
struct Member {
  explicit Member(MyBox<Instruction> instruction);
};

class Instruction {
 public:
  static MyBox<Instruction> Create(Region* parent);
};

class Region {
 public:
  Instruction* Append(MyBox<Instruction> instruction);
  MyBox<Instruction> Pop();
};

}  // namespace ns

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BRIDGED_TEMPLATE_TO_SELF_H_

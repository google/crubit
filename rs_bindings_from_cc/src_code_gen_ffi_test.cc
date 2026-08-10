// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/src_code_gen_ffi.h"

#include <utility>

#include "net/proto2/contrib/parse_proto/parse_text_proto.h"
#include "gtest/gtest.h"
#include "absl/flags/flag.h"
#include "rs_bindings_from_cc/cmdline_flags.h"

namespace crubit {

using ::proto2::contrib::parse_proto::ParseTextProtoOrDie;
using rs_bindings_from_cc::generate_bindings::GenerateBindingsRequest;

namespace {

TEST(SrcCodeGenFfiTest, NativeAndSerializedParity) {
  GenerateBindingsRequest request = ParseTextProtoOrDie(R"pb(
    ir_proto {
      current_target: "//test:target"
      top_level_items {
        key: "//test:target"
        value {}
      }
    }
    crubit_support_path_format: "<crubit/support/{header}>"
  )pb");

  // Native FFI path, which uses in-memory pointer passing.
  absl::SetFlag(&FLAGS_use_serialized_proto_ffi, false);
  auto response_native =
      GenerateBindingsProtoCall(GenerateBindingsRequest(request));

  // Serialized proto path, which uses bytes passing across FFI.
  absl::SetFlag(&FLAGS_use_serialized_proto_ffi, true);
  auto response_serialized = GenerateBindingsProtoCall(std::move(request));

  // Assert that both implementations produce the exact same output.
  EXPECT_EQ(response_native.rs_api(), response_serialized.rs_api());
  EXPECT_EQ(response_native.rs_api_impl(), response_serialized.rs_api_impl());
  EXPECT_EQ(response_native.fatal_errors(), response_serialized.fatal_errors());
}

}  // namespace
}  // namespace crubit

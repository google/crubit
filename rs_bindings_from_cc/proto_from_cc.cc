// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <utility>

#include "common/ffi_types.h"
#include "rs_bindings_from_cc/cmdline_flags.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_ffi_support.h"
#include "rs_bindings_from_cc/ir_from_cc_dependency.h"
#include "rs_bindings_from_cc/ir_generated.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/Support/FormatVariadic.h"

namespace crubit {

// This is intended to be called from Rust tests.
extern "C" ffi::IRProto proto_from_cc_dependency(
    FfiU8Slice target_triple, FfiU8Slice header_source,
    FfiU8Slice dependency_header_source, FfiU8Slice extra_feature,
    bool kythe_annotations, bool carcinize, bool lazy_import_alien_decls,
    bool check_importer_invariants) {
  auto ir = IrFromCcDependency(
      target_triple, header_source, dependency_header_source, extra_feature,
      kythe_annotations,
      carcinize ? CarcinizeMode::kStrict : CarcinizeMode::kOff,
      lazy_import_alien_decls, check_importer_invariants);

  if (!ir.ok()) {
    llvm::report_fatal_error(llvm::formatv("IrFromCc reported an error: {0}",
                                           ir.status().message()));
  }
  return *std::move(ir);
}

extern "C" ffi::IRProto crubit_test_make_ffi_ir() {
  ffi::IRProto ir;
  ir.set_current_target("//test:target");
  *ir.add_public_headers() = HeaderName("foo.h").AsFfi();
  *ir.add_public_headers() = HeaderName("bar.h").AsFfi();
  auto* item = (*ir.mutable_top_level_items())["//test:target"].Add();
  auto* func = item->mutable_func();
  func->set_id(42);
  func->set_is_inline(true);
  func->set_unique_name("c:@F@root_node");
  func->mutable_rs_name()->mutable_oper()->set_name("+=");
  return ir;
}

}  // namespace crubit

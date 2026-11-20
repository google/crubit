// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir_from_cc_dependency.h"

#include <new>
#include <string>
#include <utility>

#include "common/ffi_types.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/cmdline.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_from_cc.h"
#include "llvm/ADT/StringSet.h"
#include "llvm/Support/Error.h"

namespace crubit {

llvm::Expected<IR> IrFromCcDependency(
    FfiU8Slice target_triple, FfiU8Slice header_source,
    FfiU8Slice dependency_header_source, FfiU8Slice extra_feature,
    bool kythe_annotations, CarcinizeMode carcinize_mode,
    bool lazy_import_alien_decls, bool check_importer_invariants) {
  llvm::StringSet<> features = {"supported"};
  if (extra_feature.size != 0) {
    features.insert(std::string(StringViewFromFfiU8Slice(extra_feature)));
  }
  auto ir_or = IrFromCc({
      .extra_source_code_for_testing = StringViewFromFfiU8Slice(header_source),
      .current_target = BazelLabel{"//test:testing_target"},
      .virtual_headers_contents_for_testing =
          {{HeaderName(std::string(kDependencyHeaderName)),
            std::string(StringViewFromFfiU8Slice(dependency_header_source))}},
      .headers_to_targets = {{HeaderName(std::string(kDependencyHeaderName)),
                              BazelLabel{std::string(kDependencyTarget)}}},
      .clang_args =
          {
              // The version should be consistent with the one passed by the C++
              // toolchain.
              "-std=gnu++20",
              "-target",
              StringViewFromFfiU8Slice(target_triple),
          },
      .crubit_features = {{BazelLabel{std::string(kDependencyTarget)},
                           features},
                          {BazelLabel{"//test:testing_target"},
                           std::move(features)}},
      .kythe_annotations = kythe_annotations,
      .carcinize_mode = carcinize_mode,
      .lazy_import_alien_decls = lazy_import_alien_decls,
      .check_importer_invariants = check_importer_invariants,
  });
  if (!ir_or) {
    return ir_or.takeError();
  }
  return std::move(*ir_or);
}

// This is intended to be called from Rust tests.
extern "C" bool ir_from_cc_dependency(
    FfiU8Slice target_triple, FfiU8Slice header_source,
    FfiU8Slice dependency_header_source, FfiU8Slice extra_feature,
    bool kythe_annotations, bool carcinize, bool lazy_import_alien_decls,
    bool check_importer_invariants, ffi::IR* out_ir, FfiU8SliceBox* out_error) {
  auto ir = IrFromCcDependency(
      target_triple, header_source, dependency_header_source, extra_feature,
      kythe_annotations,
      carcinize ? CarcinizeMode::kStrict : CarcinizeMode::kOff,
      lazy_import_alien_decls, check_importer_invariants);

  if (!ir) {
    std::string err = llvm::toString(ir.takeError());
    new (out_error) FfiU8SliceBox(AllocFfiU8SliceBox(MakeFfiU8Slice(err)));
    return false;
  }
  new (out_ir) ffi::IR(*std::move(ir));
  return true;
}

extern "C" ffi::IR crubit_test_make_ffi_ir() {
  ffi::IR ir;
  ir.set_current_target("//test:target");
  *ir.add_public_headers() = HeaderName("foo.h").AsFfi();
  *ir.add_public_headers() = HeaderName("bar.h").AsFfi();
  auto& item = (*ir.mutable_top_level_items())["//test:target"].emplace_back();
  auto* func = item.mutable_func();
  func->set_id(42);
  func->set_is_inline(true);
  func->set_unique_name("c:@F@root_node");
  func->mutable_rs_name()->mutable_oper()->set_name("+=");
  return ir;
}

}  // namespace crubit

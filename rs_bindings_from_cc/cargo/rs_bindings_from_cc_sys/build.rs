// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

const PATH_TO_SRC_ROOT: &str = "../../..";

fn main() {
    crubit_build::compile_cc_lib(PATH_TO_SRC_ROOT, SOURCES).unwrap();
}

// TODO(danakj): Pull this out of the BUILD somehow?
//
// TODO(danakj): Split these up into separate Cargo targets so incremental
// builds of C++ changes are fast?
const SOURCES: &[&str] = &[
    "rs_bindings_from_cc/ast_consumer.cc",
    "rs_bindings_from_cc/ast_convert.cc",
    "rs_bindings_from_cc/ast_util.cc",
    "rs_bindings_from_cc/cmdline.cc",
    "rs_bindings_from_cc/collect_instantiations.cc",
    "rs_bindings_from_cc/collect_namespaces.cc",
    "rs_bindings_from_cc/frontend_action.cc",
    "rs_bindings_from_cc/generate_bindings_and_metadata.cc",
    "rs_bindings_from_cc/importer.cc",
    "rs_bindings_from_cc/ir.cc",
    "rs_bindings_from_cc/ir_from_cc.cc",
    "rs_bindings_from_cc/json_from_cc.cc",
    "rs_bindings_from_cc/recording_diagnostic_consumer.cc",
    "rs_bindings_from_cc/rs_bindings_from_cc.cc",
    "rs_bindings_from_cc/src_code_gen.cc",
    "rs_bindings_from_cc/type_map.cc",
    "rs_bindings_from_cc/importers/class_template.cc",
    "rs_bindings_from_cc/importers/cxx_record.cc",
    "rs_bindings_from_cc/importers/enum.cc",
    "rs_bindings_from_cc/importers/friend.cc",
    "rs_bindings_from_cc/importers/function.cc",
    "rs_bindings_from_cc/importers/function_template.cc",
    "rs_bindings_from_cc/importers/namespace.cc",
    "rs_bindings_from_cc/importers/type_alias.cc",
    "rs_bindings_from_cc/importers/type_map_override.cc",
];

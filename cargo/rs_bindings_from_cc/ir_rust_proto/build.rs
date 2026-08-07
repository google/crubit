// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated build.rs for the proto crate.

fn main() {
    println!("cargo:rerun-if-changed=../../../rs_bindings_from_cc/ir.proto");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut codegen = protobuf_codegen::CodeGen::new();
    codegen
        .output_dir(&out_dir)
        .input("ir.proto")
        .include("../../../rs_bindings_from_cc")
        .include("../../..")
        .include("../../../..")
        .include("../../../../..");
    codegen.dependency(vec![protobuf_codegen::Dependency {
        crate_name: "rs_bindings_from_cc_ir_rust_proto".to_string(),
        proto_files: vec![
            "rs_bindings_from_cc/ir.proto".into(),
            "rs_bindings_from_cc/ir.proto".into(),
        ],
        proto_import_paths: vec!["../../..".into(), "../../../../..".into()],
    }]);
    codegen.generate_and_compile().unwrap();
    let generated_path = format!("{out_dir}/ir.u.pb.rs");
    let target_path = format!("{out_dir}/ir.rs");
    if let Ok(content) = std::fs::read_to_string(&generated_path) {
        let fixed = content.replace("#![", "#[").replace("//!", "//");
        let _ = std::fs::write(&target_path, fixed);
    }
}

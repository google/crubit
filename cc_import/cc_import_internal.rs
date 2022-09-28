// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use import_internal::{ImportMacroInput, Mode};
use merged_namespaces::{JsonNamespaceHierarchy, MergedNamespaceHierarchy};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::Result;

pub struct CcImportMacroInput {
    pub(crate) import: ImportMacroInput,
}

impl Parse for CcImportMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(CcImportMacroInput { import: ImportMacroInput::parse(input)? })
    }
}

impl CcImportMacroInput {
    pub fn expand_imports(self) -> std::result::Result<TokenStream, Vec<syn::Error>> {
        let hierarchy = get_namespace_hierarchy();
        let mut tokens = import_internal::expand_imports(self.import, &Mode::NoRenaming)?;
        hierarchy.to_tokens(&mut tokens);
        Ok(tokens)
    }
}

fn get_namespace_hierarchy() -> MergedNamespaceHierarchy {
    let namespace_json_files = std::env::var("CC_IMPORT_NAMESPACES")
        .expect("Missing CC_IMPORT_NAMESPACES environment variable");
    let files: Vec<String> = serde_json::from_str(&namespace_json_files)
        .expect("Could not parse CC_IMPORT_NAMESPACES environment variable");

    let merged_hierarchy = files
        .iter()
        .map(|file_path| {
            let json_file_content = std::fs::read_to_string(file_path)
                .unwrap_or_else(|_| panic!("Couldn't read file {}", &file_path));
            let json_namespace_hierarchy: JsonNamespaceHierarchy =
                serde_json::from_str(&json_file_content)
                    .expect("Did not parse JSON content successfully");
            MergedNamespaceHierarchy::from_json_namespace_hierarchy(&json_namespace_hierarchy)
        })
        .reduce(|mut merged, next| {
            merged.merge(next);
            merged
        })
        .unwrap();
    merged_hierarchy
}

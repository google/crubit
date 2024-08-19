// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;

pub fn to_private_struct_path(input: TokenStream) -> Result<TokenStream, syn::Error> {
    validate_user_input(&input)?;
    let instantiations = read_instantiations_map()?;
    get_instantiation_struct_name(input, instantiations)
}

fn validate_user_input(_input: &TokenStream) -> Result<(), syn::Error> {
    // TODO(b/228299446): actually validate the user input and show useful error
    // messages
    Ok(())
}

fn read_instantiations_map() -> Result<HashMap<String, String>, syn::Error> {
    let path = env::var("CRUBIT_INSTANTIATIONS_FILE").map_err(|err| {
        make_syn_error(format!("Couldn't read 'CRUBIT_INSTANTIATIONS_FILE': {}.", err))
    })?;
    let file = File::open(&path).map_err(|err| {
        make_syn_error(format!("Couldn't read C++ instantiations from '{}': {}", path, err))
    })?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
        .map_err(|err| make_syn_error(format!("Couldn't deserialize JSON from {}: {}", path, err)))
}

fn get_instantiation_struct_name(
    input: TokenStream,
    instantiations: HashMap<String, String>,
) -> Result<TokenStream, syn::Error> {
    // In theory `TokenStream` -> `instantiation_name` translation could go through
    // `token_stream_printer::tokens_to_string`.  This route is not used because:
    // - The dependencies it would bring would run into b/216638047
    // - Extra functionality from that route is not needed (e.g. no need for
    //   `__COMMENT__`-aware or `__SPACE__`-aware processing, nor for special
    //   handling of `TokenTree::Group`).
    //
    // TODO(lukasza, hlopko): In the future, extra canonicalization might be
    // considered, so that `std::vector<int>`, and `std::vector<(int)>`, and
    // `std::vector<int32_t>` are treated as equivalent.
    //
    // TODO(lukasza, hlopko): More explicitly ensure that the same canonicalization
    // (e.g. TokenStream->String transformation) is used here and in
    // `rs_bindings_from_cc/collect_instantiations.rs`.
    let instantiation_name = input.to_string().replace(' ', "");

    match instantiations.get(&instantiation_name) {
        Some(concrete_struct_name) => {
            let ident = syn::parse_str::<syn::Ident>(concrete_struct_name)?;
            Ok(quote! { __cc_template_instantiations_rs_api :: #ident })
        }
        None => Err(make_syn_error(format!(
            "Couldn't find '{}' in the instantiations map {:?}",
            instantiation_name, instantiations
        ))),
    }
}

fn make_syn_error<T: Into<String>>(message: T) -> syn::Error {
    syn::Error::new(Span::call_site(), message.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
    use maplit::hashmap;
    use std::path::Path;

    fn get_error_from_read_instantiations_map(no_error_happened_msg: &str) -> String {
        read_instantiations_map().expect_err(no_error_happened_msg).to_string()
    }

    #[gtest]
    fn test_env_var_not_set() {
        let err_message =
            get_error_from_read_instantiations_map("The env var was unexpectedly set.");

        assert_eq!(
            err_message,
            "Couldn't read 'CRUBIT_INSTANTIATIONS_FILE': environment variable not found."
        );
    }

    #[gtest]
    fn test_instantiations_file_not_found() {
        env::set_var("CRUBIT_INSTANTIATIONS_FILE", "path/does/not/exist");

        let err_message =
            get_error_from_read_instantiations_map("The file was unexpectedly found.");

        assert_eq!(
            err_message,
            "Couldn't read C++ instantiations from 'path/does/not/exist': No such file or directory (os error 2)"
        );
    }

    #[gtest]
    fn test_instantiations_file_deserialization_error() {
        let path = Path::join(Path::new(&env::var("TEST_TMPDIR").unwrap()), "my_file.not_json");
        std::fs::write(&path, "definitely not json").unwrap();
        env::set_var("CRUBIT_INSTANTIATIONS_FILE", &path);

        let err_message = get_error_from_read_instantiations_map(
            "The file was unexpectedly deserialized successfully.",
        );

        assert_eq!(
            err_message,
            format!(
                "Couldn't deserialize JSON from {}: expected value at line 1 column 1",
                path.to_str().unwrap()
            )
        );
    }

    #[gtest]
    fn test_instantiations_deserialization_success() {
        let path = Path::join(Path::new(&env::var("TEST_TMPDIR").unwrap()), "instantiations.json");
        let key = "std::string<bool>";
        let value = "__CcTemplateInst_std_string_bool";
        std::fs::write(&path, serde_json::to_string(&hashmap! {key => value}).unwrap()).unwrap();
        env::set_var("CRUBIT_INSTANTIATIONS_FILE", &path);

        let deserialized_map =
            read_instantiations_map().expect("Expected successful deserialization.");

        assert_eq!(deserialized_map, hashmap! { key.to_string() => value.to_string() });
    }

    #[gtest]
    fn test_successful_expansion() {
        let expanded = get_instantiation_struct_name(
            quote! { std::vector<bool> },
            hashmap! {
                quote!{ std::vector<bool> }.to_string().replace(' ', "") => "__std_vector__bool__".to_string(),
            },
        )
        .unwrap();
        assert_eq!(
            expanded.to_string(),
            quote! {__cc_template_instantiations_rs_api::__std_vector__bool__}.to_string()
        );
    }

    #[gtest]
    fn test_parsing_valid_cc_instantiations() {
        validate_user_input(&quote! {vector<bool>}).unwrap();
        validate_user_input(&quote! {std::vector<bool>}).unwrap();
        validate_user_input(&quote! {::std::vector<bool>}).unwrap();
        validate_user_input(&quote! { vector<42> }).unwrap();
        validate_user_input(&quote! { vector<"a"> }).unwrap();
        validate_user_input(&quote! { vector<'a'> }).unwrap();
        validate_user_input(&quote! { vector<3.14> }).unwrap();
        validate_user_input(&quote! { vector<int*> }).unwrap();
        validate_user_input(&quote! { vector<42, "a", 'a', 3.14, int*> }).unwrap();
        validate_user_input(&quote! { Pair<int, Pair<int,int> > }).unwrap();
        validate_user_input(&quote! { A<B<C<int>>> }).unwrap();
        validate_user_input(&quote! { zip<short, int>::with<unsigned short, unsigned> }).unwrap();
    }
}

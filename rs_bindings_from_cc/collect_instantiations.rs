// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::{Context, Result};
use ffi_types::FfiU8Slice;
use ffi_types::FfiU8SliceBox;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use std::collections::HashSet;
use std::fs;
use std::panic::catch_unwind;
use std::path::PathBuf;
use std::process;

/// Parses given files and returns a Json list with all  C++ class
/// template instantiations requested by calls to the `cc_template!` macro.
///
/// This function panics on error.
///
/// # Safety
///
/// Expectations:
///    * function expects that param `json` is a FfiU8Slice for a valid array of
///      bytes with the given size.
///    * function expects that param `json` doesn't change during the call.
///
/// Ownership:
///    * function doesn't take ownership of (in other words it borrows) the
///      param `json`
///    * function passes ownership of the returned value to the caller
#[no_mangle]
pub unsafe extern "C" fn CollectInstantiationsImpl(json: FfiU8Slice) -> FfiU8SliceBox {
    catch_unwind(|| {
        let filenames: Vec<PathBuf> = serde_json::from_reader(json.as_slice())
            .with_context(|| {
                let json_str = std::str::from_utf8(json.as_slice()).unwrap();
                format!("Couldn't deserialize json '{}'", json_str)
            })
            .unwrap();
        let instantiations = collect_instantiations_impl(filenames).unwrap();
        let result_json = serde_json::to_string(&instantiations).unwrap();
        FfiU8SliceBox::from_boxed_slice(result_json.into_bytes().into_boxed_slice())
    })
    .unwrap_or_else(|_| process::abort())
}

fn collect_instantiations_impl(filenames: Vec<PathBuf>) -> Result<Vec<String>> {
    let mut result = HashSet::<String>::new();
    for filename in filenames {
        let content = fs::read_to_string(&filename)
            .with_context(|| format!("Couldn't read '{}'", filename.display()))?;
        let token_stream = syn::parse_str(&content)
            .with_context(|| format!("Couldn't parse the file '{}'", filename.display()))?;
        find_cc_template_calls(token_stream, &mut result);
    }
    let mut result_vec = result.into_iter().collect::<Vec<_>>();
    result_vec.sort();
    Ok(result_vec)
}

fn find_cc_template_calls(input: TokenStream, results: &mut HashSet<String>) {
    let mut iter = input.into_iter();
    while let Some(next) = iter.next() {
        // 3 token trees starting at the current 'next' ('cc_template', '!', 'group with
        // the macro body').
        let macro_tokens = std::iter::once(next.clone()).chain(iter.clone().take(2)).collect();
        if let Ok(m) = syn::parse2::<syn::Macro>(macro_tokens) {
            if m.path.is_ident("cc_template") {
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
                // `cc_template/cc_template_impl.rs`.
                let instantiation_name = m.tokens.to_string().replace(' ', "");
                results.insert(instantiation_name);
            }
        }
        if let TokenTree::Group(group) = next {
            find_cc_template_calls(group.stream(), results);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_noop() {
        assert!(collect_instantiations_impl(vec![]).unwrap().is_empty());
    }

    #[test]
    fn test_file_does_not_exist() {
        let err = collect_instantiations_impl(vec!["does/not/exist".into()]).unwrap_err();
        assert_eq!(
            format!("{:#}", err),
            "Couldn't read 'does/not/exist': No such file or directory (os error 2)"
        );
    }

    fn make_tmp_input_file(basename: &str, input: &str) -> PathBuf {
        let tmp: PathBuf = std::env::var("TEST_TMPDIR").unwrap().into();
        let result_file = tmp.join(basename);
        fs::write(&result_file, input).unwrap();
        result_file
    }

    fn write_file_and_collect_instantiations(input: TokenStream) -> Result<Vec<String>> {
        let file = make_tmp_input_file("file", &input.to_string());
        collect_instantiations_impl(vec![file])
    }

    #[test]
    fn test_file_doesnt_parse() {
        let input = make_tmp_input_file("does_not_parse", "This is not (Rust>!");
        let err = collect_instantiations_impl(vec![input.clone()]).unwrap_err();
        assert_eq!(
            format!("{:#}", err),
            format!("Couldn't parse the file '{}': lex error", input.display())
        );
    }

    #[test]
    fn test_single_template_parens() {
        let result =
            write_file_and_collect_instantiations(quote! { cc_template!(MyTemplate<int>) })
                .unwrap();
        assert_eq!(result, vec!["MyTemplate<int>".to_string()]);
    }

    #[test]
    fn test_single_template_brackets() {
        let result =
            write_file_and_collect_instantiations(quote! { cc_template![MyTemplate<int>] })
                .unwrap();
        assert_eq!(result, vec!["MyTemplate<int>".to_string()]);
    }

    #[test]
    fn test_single_template_curlies() {
        let result =
            write_file_and_collect_instantiations(quote! { cc_template!{MyTemplate<int>} })
                .unwrap();
        assert_eq!(result, vec!["MyTemplate<int>".to_string()]);
    }

    #[test]
    fn test_multiple_instantiations() {
        let result = write_file_and_collect_instantiations(quote! {
            cc_template!(MyTemplate<short>);
            cc_template!{MyTemplate<int>};
            cc_template![MyTemplate<long>];
        })
        .unwrap();
        assert_eq!(
            result,
            vec![
                "MyTemplate<int>".to_string(),
                "MyTemplate<long>".to_string(),
                "MyTemplate<short>".to_string(),
            ]
        );
    }

    #[test]
    fn test_instantiations_in_subgroups() {
        let result = write_file_and_collect_instantiations(quote! {
            fn my_rust_func(input: cc_template!(std::vector<Foo>)) ->
                    cc_template!{std::unique_ptr<absl::Time>} {
                <cc_template!(MyTemplate<42>)>::new()
            }
        })
        .unwrap();
        assert_eq!(
            result,
            vec![
                "MyTemplate<42>".to_string(),
                "std::unique_ptr<absl::Time>".to_string(),
                "std::vector<Foo>".to_string(),
            ]
        );
    }

    #[test]
    fn test_identical_instantiations() {
        let result = write_file_and_collect_instantiations(quote! {
            fn my_rust_func(input: cc_template!(std::vector<Foo>)) ->
                    cc_template!(std::vector<Foo>) {
                <cc_template!(std::vector<Foo>)>::new()
            }
        })
        .unwrap();
        assert_eq!(result, vec!["std::vector<Foo>".to_string(),]);
    }

    fn collect_instantiations_from_json(json: &str) -> String {
        let u8_slice = unsafe {
            CollectInstantiationsImpl(FfiU8Slice::from_slice(json.as_bytes())).into_boxed_slice()
        };
        std::str::from_utf8(&u8_slice).unwrap().to_string()
    }

    #[test]
    fn test_collect_instantiations_json() {
        let filename = make_tmp_input_file(
            "json",
            "cc_template!(std::vector<int>); cc_template!(std::vector<bool>);",
        );
        assert_eq!(
            collect_instantiations_from_json(&format!("[\"{}\"]", filename.display())),
            "[\"std::vector<bool>\",\"std::vector<int>\"]"
        );
    }
}

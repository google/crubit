// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Like `token_stream_matchers::assert_cc_matches!`, but expects `IR` instance
/// as input. The macro converts the instance to its corresponding struct
/// expression and matches the pattern against that.  See the documentation of
/// `token_stream_matchers` for more information.
///
/// Example:
/// ```rust
///    let ir = ir_from_cc(..., "struct SomeStruct {};').unwrap();
///    assert_ir_matches!(
///        ir,
///        quote! {
///            Record {
///                rs_name: "SomeStruct" ...
///            }
///        }
///    );
/// ```
#[macro_export]
macro_rules! assert_ir_matches {
    ($ir:expr, $pattern:expr $(,)*) => {
        $crate::internal::assert_ir_matches_fn(&$ir, &$pattern)
    };
}

/// Like `assert_ir_matches`, but asserts that the pattern does not match the
/// `IR`.
#[macro_export]
macro_rules! assert_ir_not_matches {
    ($ir:expr, $pattern:expr $(,)*) => {
        $crate::internal::assert_ir_not_matches_fn(&$ir, &$pattern)
    };
}

/// Like `assert_ir_matches!`, but expects a list of Items and a list of
/// TokenStreams as input. The macro converts each Item to its struct expression
/// and matches the corresponding pattern against that.
/// The use case for this macro is to compare a list of Items to expected
/// patterns, e.g when we want to confirm that the children items of an item
/// appear in a certain order.
#[macro_export]
macro_rules! assert_items_match {
    ($items:expr, $patterns:expr $(,)*) => {
        $crate::internal::assert_items_match_fn(&*$items, &*$patterns)
    };
}

/// Only used to make stuff needed by exported macros available
pub mod internal {

    use anyhow::Result;
    use ir::{Item, IR};
    use itertools::Itertools;
    pub use proc_macro2::TokenStream;
    use quote::quote;
    pub use token_stream_printer::{
        rs_tokens_to_formatted_string, rs_tokens_to_formatted_string_for_tests,
    };

    #[track_caller]
    pub fn assert_ir_matches_fn(ir: &IR, pattern: &TokenStream) {
        token_stream_matchers::internal::match_tokens(
            &ir_to_token_stream(ir).expect("Failed to convert IR to token stream"),
            pattern,
            ir_to_string,
        )
        .expect("input unexpectedly didn't match the pattern");
    }

    #[track_caller]
    pub fn assert_ir_not_matches_fn(ir: &IR, pattern: &TokenStream) {
        token_stream_matchers::internal::mismatch_tokens(
            &ir_to_token_stream(ir).expect("Failed to convert IR to token stream"),
            pattern,
            ir_to_string,
        )
        .unwrap()
    }

    fn ir_to_token_stream(ir: &IR) -> Result<TokenStream> {
        // derived debug impl doesn't emit commas after the last element of a group,
        // (for example `[a, b, c]`), but rustfmt automatically adds it (`[a, b,
        // c,]`). We use rustfmt to format the failure messages. So to make the
        // input token stream consistent with failure messages we format the
        // input token stream with rustfmt as well.
        Ok(ir_to_string(ir.flat_ir_debug_print().parse().unwrap())?.parse().unwrap())
    }

    fn ir_to_string(input: TokenStream) -> Result<String> {
        // Rustfmt refuses to format some kinds of invalid Rust code. Let's put our IR
        // struct expression into the body of a function, format it, and then remove
        // the function.
        let input_stream = quote! { fn make_rustfmt_happy() { #input } };
        let formatted = rs_tokens_to_formatted_string_for_tests(input_stream)?;
        let snippet = formatted
            .strip_prefix("fn make_rustfmt_happy() {\n")
            .unwrap()
            .strip_suffix("}\n")
            .unwrap()
            .lines()
            .map(|line| &line[4..])
            .join("\n");
        Ok(snippet)
    }

    #[track_caller]
    pub fn assert_items_match_fn(items: &[&Item], patterns: &[TokenStream]) {
        assert_eq!(items.len(), patterns.len());
        for (idx, (item, pattern)) in items.iter().zip(patterns).enumerate() {
            if match_item(item, pattern).is_err() {
                panic!("input at position `{idx}` unexpectedly didn't match the pattern")
            }
        }
    }

    fn match_item(item: &Item, pattern: &TokenStream) -> Result<()> {
        token_stream_matchers::internal::match_tokens(
            &item_to_token_stream(item)?,
            pattern,
            ir_to_string,
        )
    }

    fn item_to_token_stream(item: &Item) -> Result<TokenStream> {
        // derived debug impl doesn't emit commas after the last element of a group,
        // (for example `[a, b, c]`), but rustfmt automatically adds it (`[a, b,
        // c,]`). We use rustfmt to format the failure messages. So to make the
        // input token stream consistent with failure messages we format the
        // input token stream with rustfmt as well.
        Ok(ir_to_string(format! {"{:?}", item}.parse().unwrap())?.parse().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use googletest::prelude::*;
    use quote::quote;

    /// We aren't testing platform-specific details, just the matchers.
    fn ir_from_cc(header: &str) -> arc_anyhow::Result<ir::IR> {
        ir_testing::ir_from_cc(multiplatform_testing::Platform::X86Linux, header)
    }

    #[gtest]
    fn test_optional_trailing_comma() {
        assert_ir_matches!(ir_from_cc("").unwrap(), quote! { FlatIR { ... }});
        assert_ir_matches!(ir_from_cc("").unwrap(), quote! { FlatIR { ... }},);

        assert_ir_not_matches!(ir_from_cc("").unwrap(), quote! {this pattern is not in the ir});
        assert_ir_not_matches!(ir_from_cc("").unwrap(), quote! {this pattern is not in the ir},);
    }

    #[gtest]
    fn test_assert_ir_matches_assumes_trailing_commas_in_groups() {
        assert_ir_matches!(ir_from_cc("").unwrap(), quote! {{... , }});
    }

    #[gtest]
    fn test_assert_not_matches_accepts_not_matching_pattern() {
        assert_ir_not_matches!(ir_from_cc("").unwrap(), quote! {this pattern is not in the ir});
    }

    #[gtest]
    #[should_panic(expected = "input:\n\n```\nFlatIR {")]
    fn test_assert_ir_not_matches_panics_on_match() {
        assert_ir_not_matches!(ir_from_cc("").unwrap(), quote! {items});
    }

    #[gtest]
    #[should_panic]
    fn test_assert_ir_matches_panics_on_mismatch() {
        assert_ir_matches!(ir_from_cc("").unwrap(), quote! {this pattern is not in the ir});
    }
}

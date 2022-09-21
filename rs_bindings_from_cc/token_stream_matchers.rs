// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Asserts that the `input` contains the `pattern` as a subtree.
///
/// Pattern can use `...` wildcard in a group, then any content of the
/// `proc_macro2::Group` will match the pattern. Wildcards cannot match group
/// delimiters, and that therefore the tokens matched by a wildcard cannot
/// straddle a group boundary. If the wildcard is mixed with regular tokens the
/// wildcard can match 0 or many tokens and the matcher will backtrack and try
/// to find any possible match. Order of regular tokens is significant.
///
/// Examples where matching succeeds:
/// ```rust
///    assert_cc_matches!(
///       quote!{ void foo() {} },
///       quote!{ void foo() {} });
///    assert_cc_matches!(
///       quote!{ void foo() {} },
///       quote!{ foo() });
///    assert_cc_matches!(
///       quote!{ void foo() { bar(); baz(); qux(); } },
///       quote!{ void foo() { bar(); ... qux(); } });
///    // "backtracking example"
///    assert_cc_matches!(
///       quote!{ void foo() { a(); b(); c(); d(); c(); a(); },
///       quote!{ { a(); ... c(); a(); } });
/// ```
///
/// Example where matching fails:
/// ```rust
///    assert_cc_matches!(
///       quote!{ void foo() { bar(); baz(); } },
///       quote!{ void foo() { bar(); } });
/// assert_cc_matches!(
///       quote!{ void foo() { bar(); } },
///       quote!{ void ... bar() });
/// ```
#[macro_export]
macro_rules! assert_cc_matches {
    ($input:expr, $pattern:expr $(,)*) => {
        $crate::internal::match_tokens(&$input, &$pattern, &$crate::internal::tokens_to_string)
            .expect("input unexpectedly didn't match the pattern");
    };
}

/// Like `assert_cc_matches!`, but also formats the input in the error message
/// using rustfmt.
#[macro_export]
macro_rules! assert_rs_matches {
    ($input:expr, $pattern:expr $(,)*) => {
        $crate::internal::match_tokens(
                                    &$input,
                                    &$pattern,
                                    &$crate::internal::rs_tokens_to_formatted_string_for_tests,
                                )
                                .expect("input unexpectedly didn't match the pattern");
    };
}

/// Asserts that the `input` does not contain the `pattern`.
///
/// Pattern can use `...` wildcard. See `assert_cc_matches` for details.
#[macro_export]
macro_rules! assert_cc_not_matches {
    ($input:expr, $pattern:expr $(,)*) => {
        $crate::internal::mismatch_tokens(&$input, &$pattern, &$crate::internal::tokens_to_string)
            .unwrap();
    };
}

/// Like `assert_cc_not_matches!`, but also formats the input in the error
/// message using rustfmt.
#[macro_export]
macro_rules! assert_rs_not_matches {
    ($input:expr, $pattern:expr $(,)*) => {
        $crate::internal::mismatch_tokens(
                                    &$input,
                                    &$pattern,
                                    &$crate::internal::rs_tokens_to_formatted_string_for_tests,
                                )
                                .unwrap();
    };
}

/// Like `assert_cc_matches!`, but expects `IR` instance as input. The macro
/// converts the instance to its corresponding struct expression and matches the
/// pattern against that.
#[macro_export]
macro_rules! assert_ir_matches {
    ($ir:expr, $pattern:expr $(,)*) => {
        $crate::internal::match_ir(&$ir, &$pattern)
            .expect("input unexpectedly didn't match the pattern");
    };
}

/// Like `assert_ir_matches`, but asserts that the pattern does not match the
/// `IR`.
#[macro_export]
macro_rules! assert_ir_not_matches {
    ($ir:expr, $pattern:expr $(,)*) => {
        $crate::internal::mismatch_ir(&$ir, &$pattern).unwrap();
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
        assert_eq!($items.len(), $patterns.len());
                for (idx, (item, pattern)) in $items.into_iter().zip($patterns).enumerate() {
                    $crate::internal::match_item(&item, &pattern).expect(&format!(
                        "input at position {} unexpectedly didn't match the pattern",
                        &idx
                    ));
                }
    };
}

/// Only used to make stuff needed by exported macros available
pub mod internal {

    use arc_anyhow::{anyhow, Result};
    use ir::{Item, IR};
    use itertools::Itertools;
    pub use proc_macro2::TokenStream;
    use proc_macro2::TokenTree;
    use quote::quote;
    use std::iter;
    pub use token_stream_printer::{
        rs_tokens_to_formatted_string, rs_tokens_to_formatted_string_for_tests, tokens_to_string,
    };

    pub fn match_ir(ir: &IR, pattern: &TokenStream) -> Result<()> {
        match_tokens(&ir_to_token_stream(ir)?, pattern, &ir_to_string)
    }

    pub fn mismatch_ir(ir: &IR, pattern: &TokenStream) -> Result<()> {
        mismatch_tokens(&ir_to_token_stream(ir)?, pattern, &ir_to_string)
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

    pub fn match_item(item: &Item, pattern: &TokenStream) -> Result<()> {
        match_tokens(&item_to_token_stream(item)?, pattern, &ir_to_string)
    }

    fn item_to_token_stream(item: &Item) -> Result<TokenStream> {
        // derived debug impl doesn't emit commas after the last element of a group,
        // (for example `[a, b, c]`), but rustfmt automatically adds it (`[a, b,
        // c,]`). We use rustfmt to format the failure messages. So to make the
        // input token stream consistent with failure messages we format the
        // input token stream with rustfmt as well.
        Ok(ir_to_string(format! {"{:?}", item}.parse().unwrap())?.parse().unwrap())
    }

    #[derive(Debug)]
    enum MatchInfo {
        // Successful match with the suffix of the `input` stream that follows the match.
        Match { input_suffix: TokenStream },
        Mismatch(Mismatch),
    }

    #[derive(Debug)]
    struct Mismatch {
        match_length: usize,
        messages: Vec<String>,
    }

    impl Mismatch {
        fn for_no_partial_match() -> Self {
            Mismatch {
                match_length: 0,
                messages: vec![
                    "not even a partial match of the pattern throughout the input".to_string(),
                ],
            }
        }

        fn for_input_ended(
            match_length: usize,
            pattern_suffix: TokenStream,
            pattern: TokenStream,
            input: TokenStream,
        ) -> Self {
            Mismatch {
                match_length,
                messages: vec![
                    format!("expected '{}' but the input already ended", pattern_suffix),
                    format!("expected '{}' got '{}'", pattern, input),
                ],
            }
        }
    }

    pub fn match_tokens<ToStringFn>(
        input: &TokenStream,
        pattern: &TokenStream,
        to_string_fn: &ToStringFn,
    ) -> Result<()>
    where
        ToStringFn: Fn(TokenStream) -> Result<String>,
    {
        let iter = input.clone().into_iter();
        let mut best_mismatch = Mismatch::for_no_partial_match();

        let mut stack = vec![iter];
        while let Some(mut iter) = stack.pop() {
            loop {
                match match_prefix(iter.clone(), pattern.clone()) {
                    MatchInfo::Match { input_suffix: _ } => return Ok(()),
                    MatchInfo::Mismatch(mismatch) => {
                        if best_mismatch.match_length < mismatch.match_length {
                            best_mismatch = mismatch
                        }
                    }
                };
                if let Some(next) = iter.next() {
                    if let TokenTree::Group(ref group) = next {
                        stack.push(group.stream().into_iter());
                    };
                } else {
                    break;
                }
            }
        }

        assert!(!best_mismatch.messages.is_empty());
        let input_string = to_string_fn(input.clone())?;
        let mut error = anyhow!(format!("input:\n\n```\n{}\n```", input_string));
        for msg in best_mismatch.messages.into_iter().rev() {
            error = error.context(msg);
        }
        Err(error)
    }

    pub fn mismatch_tokens<ToStringFn>(
        input: &TokenStream,
        pattern: &TokenStream,
        to_string_fn: &ToStringFn,
    ) -> Result<()>
    where
        ToStringFn: Fn(TokenStream) -> Result<String>,
    {
        if match_tokens(input, pattern, to_string_fn).is_ok() {
            let input_string = to_string_fn(input.clone())?;
            Err(anyhow!(format!(
                "input unexpectedly matched the pattern. input:\n\n```\n{}\n```",
                input_string
            )))
        } else {
            Ok(())
        }
    }

    // This implementation uses naive backtracking algorithm that is in the worst
    // case O(2^n) in the number of wildcards. In practice this is not so bad
    // because wildcards only match their current group, they don't descend into
    // subtrees and they don't match outside. Still, it may be possible to
    // reimplement this using NFA and end up with simpler, more regular code
    // while still providing reasonable error messages on mismatch.
    // TODO(hlopko): Try to reimplement matching using NFA.
    fn match_prefix(
        input: impl Iterator<Item = TokenTree> + Clone,
        pattern: TokenStream,
    ) -> MatchInfo {
        let mut input_iter = input.clone();
        let mut pattern_iter = pattern.clone().into_iter().peekable();
        let mut match_counter = 0;
        let mut best_mismatch = Mismatch::for_no_partial_match();
        let mut update_best_mismatch = |mismatch: Mismatch| {
            if mismatch.match_length > best_mismatch.match_length {
                best_mismatch = mismatch;
            }
        };
        while let Some(actual_token) = input_iter.next() {
            if is_whitespace_token(&actual_token) {
                continue;
            }

            if starts_with_wildcard(to_stream(&pattern_iter)) {
                // branch off to matching the token after the wildcard
                match match_after_wildcard(
                    reinsert_token(input_iter.clone(), actual_token).into_iter(),
                    input.clone(),
                    skip_wildcard(pattern_iter.clone()),
                ) {
                    MatchInfo::Mismatch(mut mismatch) => {
                        mismatch.match_length += match_counter;
                        update_best_mismatch(mismatch);
                    }
                    match_info => {
                        return match_info;
                    }
                }
                // and if that didn't work, consume one more token by the wildcard
                continue;
            }

            if let Some(pattern_token) = pattern_iter.next() {
                if let MatchInfo::Mismatch(mut mismatch) = match_tree(&actual_token, &pattern_token)
                {
                    mismatch.messages.push(format!(
                        "expected '{}' got '{}'",
                        pattern,
                        input.collect::<TokenStream>()
                    ));
                    mismatch.match_length += match_counter;
                    return MatchInfo::Mismatch(mismatch);
                }
            } else {
                return MatchInfo::Match { input_suffix: reinsert_token(input_iter, actual_token) };
            }
            match_counter += 1;
        }

        if pattern_iter.peek().is_none() {
            return MatchInfo::Match { input_suffix: TokenStream::new() };
        }
        if is_wildcard(to_stream(&pattern_iter)) {
            return MatchInfo::Match { input_suffix: TokenStream::new() };
        }

        update_best_mismatch(Mismatch::for_input_ended(
            match_counter,
            to_stream(&pattern_iter),
            pattern,
            to_stream(&input),
        ));
        MatchInfo::Mismatch(best_mismatch)
    }

    fn match_after_wildcard(
        input_iter: impl Iterator<Item = TokenTree> + Clone,
        input: impl Iterator<Item = TokenTree> + Clone,
        pattern: TokenStream,
    ) -> MatchInfo {
        match match_prefix(input_iter.clone(), pattern.clone()) {
            MatchInfo::Match { input_suffix } if input_suffix.is_empty() => {
                MatchInfo::Match { input_suffix }
            }
            MatchInfo::Match { input_suffix } => {
                let match_input_length = input_iter.count() + 1;
                let suffix_length = input_suffix.into_iter().count();
                MatchInfo::Mismatch(Mismatch::for_input_ended(
                    match_input_length - suffix_length,
                    pattern.clone(),
                    pattern,
                    to_stream(&input),
                ))
            }
            mismatch => mismatch,
        }
    }

    fn to_stream(iter: &(impl Iterator<Item = TokenTree> + Clone)) -> TokenStream {
        iter.clone().collect::<TokenStream>()
    }

    fn reinsert_token(
        iter: impl Iterator<Item = TokenTree> + Clone,
        token: TokenTree,
    ) -> TokenStream {
        iter::once(token).chain(iter).collect::<TokenStream>()
    }

    fn is_whitespace_token(token: &TokenTree) -> bool {
        matches!(token, TokenTree::Ident(id) if id == "__NEWLINE__" || id == "__SPACE__")
    }

    fn is_wildcard(pattern: TokenStream) -> bool {
        format!("{}", pattern) == "..."
    }

    fn starts_with_wildcard(pattern: TokenStream) -> bool {
        format!("{}", pattern).starts_with("...")
    }

    fn skip_wildcard(pattern: impl Iterator<Item = TokenTree> + Clone) -> TokenStream {
        assert!(starts_with_wildcard(to_stream(&pattern)));
        pattern.skip(3).collect::<TokenStream>()
    }

    fn match_tree(actual_token: &TokenTree, pattern_token: &TokenTree) -> MatchInfo {
        match (actual_token, pattern_token) {
            (TokenTree::Group(ref actual_group), TokenTree::Group(ref pattern_group)) => {
                if actual_group.delimiter() != pattern_group.delimiter() {
                    return MatchInfo::Mismatch(Mismatch {
                        match_length: 0,
                        messages: vec![format!(
                            "expected delimiter {:?} for group '{}' but got {:?} for group '{}'",
                            pattern_group.delimiter(),
                            Into::<TokenStream>::into(pattern_token.clone()),
                            actual_group.delimiter(),
                            Into::<TokenStream>::into(actual_token.clone()),
                        )],
                    });
                }
                let match_info =
                    match_prefix(actual_group.stream().into_iter(), pattern_group.stream());
                match match_info {
                    MatchInfo::Match { input_suffix } => {
                        if input_suffix
                            .clone()
                            .into_iter()
                            .filter(|token| !is_whitespace_token(token))
                            .count()
                            != 0
                        {
                            MatchInfo::Mismatch(Mismatch {
                                match_length: 0,
                                messages: vec![format!(
                                    "matched the entire pattern but the input still contained '{}'",
                                    input_suffix
                                )],
                            })
                        } else {
                            MatchInfo::Match { input_suffix: TokenStream::new() }
                        }
                    }
                    mismatch => mismatch,
                }
            }
            (ref actual, ref pattern) => {
                let actual_src = format!("{}", actual);
                let pattern_src = format!("{}", pattern);
                if actual_src == pattern_src {
                    MatchInfo::Match { input_suffix: TokenStream::new() }
                } else {
                    MatchInfo::Mismatch(Mismatch {
                        match_length: 0,
                        messages: vec![format!("expected '{}' but got '{}'", pattern, actual)],
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::internal::*;
    use super::*;
    use ir_testing::ir_from_cc;
    use quote::quote;

    macro_rules! assert_rs_cc_matches {
        ($input:expr, $pattern:expr $(,)*) => {
            $crate::assert_cc_matches!($input, $pattern);
            $crate::assert_rs_matches!($input, $pattern);
        };
    }

    #[test]
    fn test_optional_trailing_comma() {
        assert_rs_matches!(quote! {x}, quote! {x});
        assert_rs_matches!(quote! {x}, quote! {x},);

        assert_cc_matches!(quote! {x}, quote! {x});
        assert_cc_matches!(quote! {x}, quote! {x},);

        assert_ir_matches!(ir_from_cc("").unwrap(), quote! {});
        assert_ir_matches!(ir_from_cc("").unwrap(), quote! {},);

        assert_rs_not_matches!(quote! {x}, quote! {y});
        assert_rs_not_matches!(quote! {x}, quote! {y},);

        assert_cc_not_matches!(quote! {x}, quote! {y});
        assert_cc_not_matches!(quote! {x}, quote! {y},);

        assert_ir_not_matches!(ir_from_cc("").unwrap(), quote! {this pattern is not in the ir});
        assert_ir_not_matches!(ir_from_cc("").unwrap(), quote! {this pattern is not in the ir},);
    }

    #[test]
    fn test_assert_ir_matches_assumes_trailing_commas_in_groups() {
        assert_ir_matches!(
            ir_from_cc("").unwrap(),
            quote! {{... items: [...], top_level_item_ids: [...], }}
        );
    }

    #[test]
    fn test_assert_not_matches_accepts_not_matching_pattern() {
        assert_cc_not_matches!(quote! { fn foo() {} }, quote! { fn bar() {} });
        assert_rs_not_matches!(quote! { fn foo() {} }, quote! { fn bar() {} });
        assert_ir_not_matches!(ir_from_cc("").unwrap(), quote! {this pattern is not in the ir});
    }

    #[test]
    #[should_panic(expected = "input:\n\n```\nfn foo(){  }\n")]
    fn test_assert_cc_not_matches_panics_on_match() {
        assert_cc_not_matches!(quote! { fn foo() {} }, quote! { fn foo() {} });
    }

    #[test]
    #[should_panic(expected = "input:\n\n```\nfn foo() {}\n\n```")]
    fn test_assert_rs_not_matches_panics_on_match() {
        assert_rs_not_matches!(quote! { fn foo() {} }, quote! { fn foo() {} });
    }

    #[test]
    #[should_panic(expected = "input:\n\n```\nFlatIR {")]
    fn test_assert_ir_not_matches_panics_on_match() {
        assert_ir_not_matches!(ir_from_cc("").unwrap(), quote! {items});
    }

    #[test]
    fn test_assert_cc_matches_accepts_matching_pattern() {
        assert_rs_cc_matches!(quote! { fn foo() {} }, quote! { fn foo() {} });
    }

    #[test]
    #[should_panic]
    fn test_assert_cc_matches_panics_on_mismatch() {
        assert_cc_matches!(quote! { fn foo() {} }, quote! { fn bar() {} });
    }

    #[test]
    #[should_panic]
    fn test_assert_rs_matches_panics_on_mismatch() {
        assert_rs_matches!(quote! { fn foo() {} }, quote! { fn bar() {} });
    }

    #[test]
    #[should_panic]
    fn test_assert_ir_matches_panics_on_mismatch() {
        assert_ir_matches!(ir_from_cc("").unwrap(), quote! {this pattern is not in the ir});
    }

    #[test]
    fn test_accept_siblings() {
        assert_rs_cc_matches!(quote! {a b c d}, quote! {a b c d});
        assert_rs_cc_matches!(quote! {a b c d}, quote! {a b});
        assert_rs_cc_matches!(quote! {a b c d}, quote! {b c});
        assert_rs_cc_matches!(quote! {a b c d}, quote! {c d});
    }

    #[test]
    fn test_accept_subtrees() {
        assert_rs_cc_matches!(quote! {impl SomeStruct { fn foo() {} }}, quote! {fn foo() {}});
    }

    #[test]
    #[should_panic]
    fn test_cc_reject_partial_subtree() {
        assert_cc_matches!(quote! {fn foo() {a(); b();}}, quote! {fn foo() { a(); }});
    }

    #[test]
    #[should_panic]
    fn test_rs_reject_partial_subtree() {
        assert_rs_matches!(quote! {fn foo() {a(); b();}}, quote! {fn foo() { a(); }});
    }

    #[test]
    fn test_cc_error_message() {
        assert_eq!(
            format!(
                "{:?}",
                match_tokens(
                    &quote! {struct A { int a; int b; };},
                    &quote! {struct B},
                    &tokens_to_string
                )
                .expect_err("unexpected match")
            ),
            "expected 'B' but got 'A'

Caused by:
    0: expected 'struct B' got 'struct A { int a ; int b ; } ;'
    1: input:\n       \n       ```
       struct A{ int a;int b; };
       ```"
        );
    }

    #[test]
    fn test_rustfmt_in_rs_error_message() {
        assert_eq!(
            format!(
                "{:?}",
                match_tokens(
                    &quote! {struct A { a: i64, b: i64 }},
                    &quote! {struct B},
                    &rs_tokens_to_formatted_string_for_tests,
                )
                .expect_err("unexpected match")
            ),
            "expected 'B' but got 'A'

Caused by:
    0: expected 'struct B' got 'struct A { a : i64 , b : i64 }'
    1: input:\n       \n       ```
       struct A {
           a: i64,
           b: i64,
       }\n       \n       ```"
        );
    }

    #[test]
    fn test_reject_unfinished_pattern() {
        assert_eq!(
            format!(
                "{:#}",
                match_tokens(
                    &quote! {fn foo() {}},
                    &quote! {fn foo() {} struct X {}},
                    &tokens_to_string
                )
                .expect_err("unexpected match")
            ),
            "expected 'struct X { }' but the input already ended: \
                expected 'fn foo () { } struct X { }' got 'fn foo () { }': \
                input:\n\n```\nfn foo(){  }\n```"
        );
    }

    #[test]
    fn test_reject_different_delimiters() {
        assert_eq!(
            format!(
                "{:#}",
                match_tokens(&quote! {fn foo() ()}, &quote! {fn foo() {}}, &tokens_to_string)
                    .expect_err("unexpected match")
            ),
            "expected delimiter Brace for group '{ }' but got Parenthesis for group '()': \
                expected 'fn foo () { }' got 'fn foo () ()': \
                input:\n\n```\nfn foo()()\n```"
        );
    }

    #[test]
    fn test_reject_mismatch_inside_group() {
        assert_eq!(
            format!(
                "{:#}",
                match_tokens(
                    &quote! {fn foo() { a: i64, b: i64 }},
                    &quote! {fn foo() { a: i64, c: i64 }},
                    &tokens_to_string
                )
                .expect_err("unexpected match")
            ),
            "expected 'c' but got 'b': \
            expected 'a : i64 , c : i64' got 'a : i64 , b : i64': \
            expected 'fn foo () { a : i64 , c : i64 }' got 'fn foo () { a : i64 , b : i64 }': \
            input:\n\n```\nfn foo(){ a:i64,b:i64 }\n```"
        );
    }

    #[test]
    fn test_accept_wildcard_in_group() {
        assert_rs_cc_matches!(
            quote! {fn foo() -> bool { return false; }},
            quote! {fn foo() -> bool {...}}
        );
    }

    #[test]
    fn test_ignore_newlines() {
        assert_rs_cc_matches!(
            quote! {__NEWLINE__ fn __NEWLINE__ foo __NEWLINE__ (
            __NEWLINE__ a __NEWLINE__ : __NEWLINE__ usize __NEWLINE__) {}},
            quote! {fn foo(a: usize) {}}
        );
    }

    #[test]
    fn test_ignore_space() {
        assert_rs_cc_matches!(
            quote! {__SPACE__ fn __SPACE__ foo __SPACE__ (
            __SPACE__ a __SPACE__ : __SPACE__ usize __SPACE__) {}},
            quote! {fn foo(a: usize) {}}
        );
    }

    #[test]
    fn test_reject_unfinished_input_inside_group() {
        assert_eq!(
            format!(
                "{:#}",
                match_tokens(
                    &quote! {impl Drop { fn drop(&mut self) { drop_impl(); }}},
                    &quote! {fn drop(&mut self) {}},
                    &tokens_to_string
                )
                .expect_err("unexpected match")
            ),
            "matched the entire pattern but the input still contained 'drop_impl () ;': \
                expected 'fn drop (& mut self) { }' got 'fn drop (& mut self) { drop_impl () ; }': \
                input:\n\n```\nimpl Drop{ fn drop(&mut self){ drop_impl(); } }\n```"
        );
        assert_eq!(
            format!(
                "{:#}",
                match_tokens(
                    &quote! {impl Drop { fn drop(&mut self) { drop_impl1(); drop_impl2(); }}},
                    &quote! {fn drop(&mut self) { drop_impl1(); }},
                    &tokens_to_string
                )
                .expect_err("unexpected match")
            ),
            "matched the entire pattern but the input still contained 'drop_impl2 () ;': \
                expected 'fn drop (& mut self) { drop_impl1 () ; }' \
                got 'fn drop (& mut self) { drop_impl1 () ; drop_impl2 () ; }': \
                input:\n\n```\nimpl Drop{ fn drop(&mut self){ \
                    drop_impl1();drop_impl2(); } }\n```"
        );
    }

    #[test]
    fn test_accept_unfinished_input_with_only_newlines() {
        assert_rs_cc_matches!(quote! {fn foo() { __NEWLINE__ }}, quote! {fn foo() {}});
        assert_rs_cc_matches!(quote! {fn foo() { a(); __NEWLINE__ }}, quote! {fn foo() { a(); }});
    }

    #[test]
    fn test_wildcard_in_the_beginning_of_the_group() {
        assert_rs_cc_matches!(quote! { [ a b c ] }, quote! { [ a ... ] });
        assert_rs_cc_matches!(quote! { [ a a b b c c ] }, quote! { [ a a ... ] });
    }
    #[test]
    fn test_wildcard_in_the_middle_of_the_group() {
        assert_rs_cc_matches!(quote! { [ a b c ] }, quote! { [ a ... c ] });
        assert_rs_cc_matches!(quote! { [ a a b b c c ] }, quote! { [ a a ... c c ] });
    }
    #[test]
    fn test_wildcard_in_the_end_of_the_group() {
        assert_rs_cc_matches!(quote! { [ a b c ] }, quote! { [ ... c ] });
        assert_rs_cc_matches!(quote! { [ a a b b c c ] }, quote! { [ ... c c ] });
    }

    #[test]
    fn test_wildcard_not_consuming_anything_in_group() {
        assert_rs_cc_matches!(quote! { [ a b c ] }, quote! { [ ... a b c ] });
        assert_rs_cc_matches!(quote! { [ a b c ] }, quote! { [ a ... b c ] });
        assert_rs_cc_matches!(quote! { [ a b c ] }, quote! { [ a b ... c ] });
        assert_rs_cc_matches!(quote! { [ a b c ] }, quote! { [ a b c ... ] });
    }

    #[test]
    fn test_error_message_shows_the_longest_match_with_wildcards() {
        assert_eq!(
            format!(
                "{:#}",
                match_tokens(&quote! {[ a b b ]}, &quote! { [ a ... c ]}, &tokens_to_string)
                    .expect_err("unexpected match")
            ),
            // the error message shows "longer match" with more tokens consumed by the wildcard
            "expected 'c' but got 'b': \
            expected 'c' got 'b b': \
            expected '[a ... c]' got '[a b b]': \
            input:\n\n```\n[a b b]\n```"
        );
        assert_eq!(
            format!(
                "{:#}",
                match_tokens(&quote! {[ a b b ]}, &quote! { [ a ... b c ]}, &tokens_to_string)
                    .expect_err("unexpected match")
            ),
            // the error message shows "longer match" with branching off the wildcard earlier
            "expected 'c' but got 'b': \
            expected 'b c' got 'b b': \
            expected '[a ... b c]' got '[a b b]': \
            input:\n\n```\n[a b b]\n```"
        );
    }
}

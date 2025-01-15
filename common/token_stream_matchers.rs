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
        $crate::internal::match_tokens(
            &$input,
            &$pattern,
            $crate::internal::cc_tokens_to_formatted_string_for_tests,
        )
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
            $crate::internal::rs_tokens_to_formatted_string_for_tests,
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
        $crate::internal::mismatch_tokens(
            &$input,
            &$pattern,
            $crate::internal::cc_tokens_to_formatted_string_for_tests,
        )
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
            $crate::internal::rs_tokens_to_formatted_string_for_tests,
        )
        .unwrap();
    };
}

/// Only used to make stuff needed by exported macros available
pub mod internal {

    use anyhow::{anyhow, Result};
    pub use proc_macro2::TokenStream;
    use proc_macro2::TokenTree;
    use std::iter;
    pub use token_stream_printer::{
        cc_tokens_to_formatted_string_for_tests, rs_tokens_to_formatted_string,
        rs_tokens_to_formatted_string_for_tests,
    };

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
                    "not even a partial match of the pattern throughout the input".to_string()
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

    pub fn match_tokens(
        input: &TokenStream,
        pattern: &TokenStream,
        to_string_fn: fn(TokenStream) -> Result<String>,
    ) -> Result<()> {
        // `match_tokens` behaves as if the `pattern` implicitly had a wildcard `...` at
        // the beginning and the end.  Therefore an empty `pattern` is most
        // likely a mistake.
        assert!(
            !pattern.is_empty(),
            "Empty `pattern` is unexpected, because it always matches. \
             (Maybe you used `// comment text` instead of `__COMMENT__ \"comment text\"? \
              Or maybe you want to use `TokenStream::is_empty`?)"
        );

        let iter = input.clone().into_iter();
        let mut best_mismatch = Mismatch::for_no_partial_match();

        let mut stack = vec![iter];
        while let Some(mut iter) = stack.pop() {
            loop {
                match match_prefix(iter.clone(), pattern.clone(), false) {
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

    pub fn mismatch_tokens(
        input: &TokenStream,
        pattern: &TokenStream,
        to_string_fn: fn(TokenStream) -> Result<String>,
    ) -> Result<()> {
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
        match_inside_group: bool,
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
                    match_inside_group,
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
                match_counter += 1;
            } else if match_inside_group {
                return MatchInfo::Match { input_suffix: reinsert_token(input_iter, actual_token) };
            } else {
                // If we are not inside a group, seeing the end of the pattern means that we
                // have matched the entire pattern.
                return MatchInfo::Match { input_suffix: TokenStream::new() };
            }
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
        match_inside_group: bool,
    ) -> MatchInfo {
        match match_prefix(input_iter.clone(), pattern.clone(), match_inside_group) {
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
                    match_prefix(actual_group.stream().into_iter(), pattern_group.stream(), true);
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
    use googletest::prelude::*;
    use quote::quote;

    macro_rules! assert_rs_cc_matches {
        ($input:expr, $pattern:expr $(,)*) => {
            $crate::assert_cc_matches!($input, $pattern);
            $crate::assert_rs_matches!($input, $pattern);
        };
    }

    #[gtest]
    fn test_optional_trailing_comma() {
        assert_rs_matches!(quote! {x}, quote! {x});
        assert_rs_matches!(quote! {x}, quote! {x},);

        assert_cc_matches!(quote! {x}, quote! {x});
        assert_cc_matches!(quote! {x}, quote! {x},);

        assert_rs_not_matches!(quote! {x}, quote! {y});
        assert_rs_not_matches!(quote! {x}, quote! {y},);

        assert_cc_not_matches!(quote! {x}, quote! {y});
        assert_cc_not_matches!(quote! {x}, quote! {y},);
    }

    #[gtest]
    fn test_assert_not_matches_accepts_not_matching_pattern() {
        assert_cc_not_matches!(quote! { fn foo() {} }, quote! { fn bar() {} });
        assert_rs_not_matches!(quote! { fn foo() {} }, quote! { fn bar() {} });
    }

    #[gtest]
    #[should_panic(expected = r#"input unexpectedly matched the pattern. input:

```
fn foo() {}
```"#)]
    fn test_assert_cc_not_matches_panics_on_match() {
        assert_cc_not_matches!(quote! { fn foo() {} }, quote! { fn foo() {} });
    }

    #[gtest]
    #[should_panic(expected = "input:\n\n```\nfn foo() {}\n\n```")]
    fn test_assert_rs_not_matches_panics_on_match() {
        assert_rs_not_matches!(quote! { fn foo() {} }, quote! { fn foo() {} });
    }

    #[gtest]
    fn test_assert_cc_matches_accepts_matching_pattern() {
        assert_rs_cc_matches!(quote! { fn foo() {} }, quote! { fn foo() {} });
    }

    #[gtest]
    #[should_panic]
    fn test_assert_cc_matches_panics_on_mismatch() {
        assert_cc_matches!(quote! { fn foo() {} }, quote! { fn bar() {} });
    }

    #[gtest]
    #[should_panic]
    fn test_assert_rs_matches_panics_on_mismatch() {
        assert_rs_matches!(quote! { fn foo() {} }, quote! { fn bar() {} });
    }

    #[gtest]
    fn test_accept_siblings() {
        assert_rs_cc_matches!(quote! {a b c d}, quote! {a b c d});
        assert_rs_cc_matches!(quote! {a b c d}, quote! {a b});
        assert_rs_cc_matches!(quote! {a b c d}, quote! {b c});
        assert_rs_cc_matches!(quote! {a b c d}, quote! {c d});
    }

    #[gtest]
    fn test_accept_subtrees() {
        assert_rs_cc_matches!(quote! {impl SomeStruct { fn foo() {} }}, quote! {fn foo() {}});
    }

    #[gtest]
    #[should_panic]
    fn test_cc_reject_partial_subtree() {
        assert_cc_matches!(quote! {fn foo() {a(); b();}}, quote! {fn foo() { a(); }});
    }

    #[gtest]
    #[should_panic]
    fn test_rs_reject_partial_subtree() {
        assert_rs_matches!(quote! {fn foo() {a(); b();}}, quote! {fn foo() { a(); }});
    }

    #[gtest]
    fn test_cc_error_message() {
        assert_eq!(
            format!(
                "{:?}",
                match_tokens(
                    &quote! {struct A { int a; int b; };},
                    &quote! {struct B},
                    cc_tokens_to_formatted_string_for_tests
                )
                .expect_err("unexpected match")
            ),
            r#"expected 'B' but got 'A'

Caused by:
    0: expected 'struct B' got 'struct A { int a ; int b ; } ;'
    1: input:
       
       ```
       struct A {
         int a;
         int b;
       };
       ```"#
        );
    }

    #[gtest]
    fn test_rustfmt_in_rs_error_message() {
        assert_eq!(
            format!(
                "{:?}",
                match_tokens(
                    &quote! {struct A { a: i64, b: i64 }},
                    &quote! {struct B},
                    rs_tokens_to_formatted_string_for_tests,
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

    #[gtest]
    fn test_reject_unfinished_pattern() {
        assert_eq!(
            format!(
                "{:#}",
                match_tokens(
                    &quote! {fn foo() {}},
                    &quote! {fn foo() {} struct X {}},
                    rs_tokens_to_formatted_string_for_tests
                )
                .expect_err("unexpected match")
            ),
            r#"expected 'struct X { }' but the input already ended: expected 'fn foo () { } struct X { }' got 'fn foo () { }': input:

```
fn foo() {}

```"#
        );
    }

    #[gtest]
    fn test_reject_different_delimiters() {
        assert_eq!(
            format!(
                "{:#}",
                match_tokens(
                    &quote! {fn foo() {}},
                    &quote! {fn foo() ()},
                    rs_tokens_to_formatted_string_for_tests
                )
                .expect_err("unexpected match")
            ),
            r#"expected delimiter Parenthesis for group '()' but got Brace for group '{ }': expected 'fn foo () ()' got 'fn foo () { }': input:

```
fn foo() {}

```"#
        );
    }

    #[gtest]
    fn test_reject_mismatch_inside_group() {
        assert_eq!(
            format!(
                "{:#}",
                match_tokens(
                    &quote! {fn foo() { let a = 1; let b = 2; }},
                    &quote! {fn foo() { let a = 1; let c = 2; }},
                    rs_tokens_to_formatted_string_for_tests
                )
                .expect_err("unexpected match")
            ),
            "expected 'c' but got 'b': \
             expected 'let a = 1 ; let c = 2 ;' got 'let a = 1 ; let b = 2 ;': \
             expected 'fn foo () { let a = 1 ; let c = 2 ; }' \
             got 'fn foo () { let a = 1 ; let b = 2 ; }': \
             input:\n\n```\nfn foo() {\n    let a = 1;\n    let b = 2;\n}\n\n```"
        );
    }

    #[gtest]
    fn test_accept_wildcard_in_group() {
        assert_rs_cc_matches!(
            quote! {fn foo() -> bool { return false; }},
            quote! {fn foo() -> bool {...}}
        );
    }

    #[gtest]
    fn test_ignore_newlines() {
        assert_rs_cc_matches!(
            quote! {__NEWLINE__ fn __NEWLINE__ foo __NEWLINE__ (
            __NEWLINE__ a __NEWLINE__ : __NEWLINE__ usize __NEWLINE__) {}},
            quote! {fn foo(a: usize) {}}
        );
    }

    #[gtest]
    fn test_ignore_space() {
        assert_rs_cc_matches!(
            quote! {__SPACE__ fn __SPACE__ foo __SPACE__ (
            __SPACE__ a __SPACE__ : __SPACE__ usize __SPACE__) {}},
            quote! {fn foo(a: usize) {}}
        );
    }

    #[gtest]
    fn test_reject_unfinished_input_inside_group() {
        assert_eq!(
            format!(
                "{:#}",
                match_tokens(
                    &quote! {impl Drop { fn drop(&mut self) { drop_impl(); }}},
                    &quote! {fn drop(&mut self) {}},
                    rs_tokens_to_formatted_string_for_tests
                )
                .expect_err("unexpected match")
            ),
            r#"matched the entire pattern but the input still contained 'drop_impl () ;': expected 'fn drop (& mut self) { }' got 'fn drop (& mut self) { drop_impl () ; }': input:

```
impl Drop {
    fn drop(&mut self) {
        drop_impl();
    }
}

```"#
        );
        assert_eq!(
            format!(
                "{:#}",
                match_tokens(
                    &quote! {impl Drop { fn drop(&mut self) { drop_impl1(); drop_impl2(); }}},
                    &quote! {fn drop(&mut self) { drop_impl1(); }},
                    rs_tokens_to_formatted_string_for_tests
                )
                .expect_err("unexpected match")
            ),
            r#"matched the entire pattern but the input still contained 'drop_impl2 () ;': expected 'fn drop (& mut self) { drop_impl1 () ; }' got 'fn drop (& mut self) { drop_impl1 () ; drop_impl2 () ; }': input:

```
impl Drop {
    fn drop(&mut self) {
        drop_impl1();
        drop_impl2();
    }
}

```"#
        );
    }

    #[gtest]
    fn test_accept_unfinished_input_with_only_newlines() {
        assert_rs_cc_matches!(quote! {fn foo() { __NEWLINE__ }}, quote! {fn foo() {}});
        assert_rs_cc_matches!(quote! {fn foo() { a(); __NEWLINE__ }}, quote! {fn foo() { a(); }});
    }

    #[gtest]
    fn test_wildcard_in_the_beginning_of_the_group() {
        assert_rs_cc_matches!(quote! { [ a b c ] }, quote! { [ ... c ] });
        assert_rs_cc_matches!(quote! { [ a a b b c c ] }, quote! { [ ... c c ] });
    }
    #[gtest]
    fn test_wildcard_in_the_middle_of_the_group() {
        assert_rs_cc_matches!(quote! { [ a b c ] }, quote! { [ a ... c ] });
        assert_rs_cc_matches!(quote! { [ a a b b c c ] }, quote! { [ a a ... c c ] });
    }
    #[gtest]
    fn test_wildcard_in_the_end_of_the_group() {
        assert_rs_cc_matches!(quote! { [ a b c ] }, quote! { [ a ... ] });
        assert_rs_cc_matches!(quote! { [ a a b b c c ] }, quote! { [ a a ... ] });
    }
    #[gtest]
    fn test_pattern_with_wildcards_must_cover_entire_group() {
        // pattern `[]` would not match the input
        assert_rs_cc_matches!(quote! { [ a a b b c c ] }, quote! { [ ... ] });
        // pattern `[... b]` would not match the input
        assert_rs_cc_matches!(quote! { [ a a b b c c ] }, quote! { [ ... c ] });
        // pattern `[b ...]` would not match the input
        assert_rs_cc_matches!(quote! { [ a a b b c c ] }, quote! { [ a ... ] });
    }

    #[gtest]
    fn test_wildcard_not_consuming_anything_in_group() {
        assert_rs_cc_matches!(quote! { [ a b c ] }, quote! { [ ... a b c ] });
        assert_rs_cc_matches!(quote! { [ a b c ] }, quote! { [ a ... b c ] });
        assert_rs_cc_matches!(quote! { [ a b c ] }, quote! { [ a b ... c ] });
        assert_rs_cc_matches!(quote! { [ a b c ] }, quote! { [ a b c ... ] });
    }

    #[gtest]
    fn test_multiple_wildcards() {
        assert_rs_cc_matches!(quote! { [ a b c d e f g ] }, quote! { [ a ... b ... c ... f ... ] });
        assert_rs_cc_matches!(quote! { [ a b c d e f g ] }, quote! { [ a ... b ... f ... g ] });
    }

    #[gtest]
    fn test_error_message_shows_the_longest_match_with_wildcards() {
        assert_eq!(
            format!(
                "{:#}",
                match_tokens(
                    &quote! { [ a b b ] },
                    &quote! { [ a ... c ]},
                    |tokens: TokenStream| Ok(tokens.to_string())
                )
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
                match_tokens(
                    &quote! {[ a b b ]},
                    &quote! { [ a ... b c ]},
                    |tokens: TokenStream| Ok(tokens.to_string())
                )
                .expect_err("unexpected match")
            ),
            // the error message shows "longer match" with branching off the wildcard earlier
            "expected 'c' but got 'b': \
            expected 'b c' got 'b b': \
            expected '[a ... b c]' got '[a b b]': \
            input:\n\n```\n[a b b]\n```"
        );
    }

    #[gtest]
    #[should_panic(expected = "Empty `pattern` is unexpected, because it always matches. \
             (Maybe you used `// comment text` instead of `__COMMENT__ \"comment text\"? \
              Or maybe you want to use `TokenStream::is_empty`?)")]
    fn test_assert_cc_matches_panics_when_pattern_is_empty() {
        assert_cc_matches!(
            quote! { foo bar },
            quote! {
                // This comment will be stripped by `quote!`, but some test assertions
                // mistakenly used the comment syntax instead of `__COMMENT__ "text"`
            },
        );
    }

    #[gtest]
    #[should_panic(expected = "Empty `pattern` is unexpected, because it always matches. \
             (Maybe you used `// comment text` instead of `__COMMENT__ \"comment text\"? \
              Or maybe you want to use `TokenStream::is_empty`?)")]
    fn test_assert_rs_matches_panics_when_pattern_is_empty() {
        assert_rs_matches!(
            quote! { foo bar },
            quote! {
                // This comment will be stripped by `quote!`, but some test assertions
                // mistakenly used the comment syntax instead of `__COMMENT__ "text"`
            },
        );
    }

    #[gtest]
    fn test_assert_rs_matches_does_not_need_trailing_wildcard() {
        assert_rs_matches!(
            quote! {
                fn f() -> f32 {}
                fn g() {}
            },
            quote! {
                fn ...() -> f32 {}
            }
        );
    }

    #[gtest]
    #[should_panic]
    fn test_assert_rs_matches_no_trailing_wildcard_inside_group() {
        assert_rs_matches!(
            quote! {
                fn f() -> f32 { return 1.0; }
            },
            quote! {
                fn f() -> f32 {}
            }
        );
    }
}

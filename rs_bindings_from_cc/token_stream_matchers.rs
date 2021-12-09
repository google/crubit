// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::{anyhow, Result};
use proc_macro2::{token_stream, Group, TokenStream, TokenTree};
use std::iter;
pub use token_stream_printer::{rs_tokens_to_formatted_string, tokens_to_string};

/// Asserts that the `input` contains the `pattern` as a subtree.
///
/// Pattern can use `...` wildcard in a group, then any content of the
/// `proc_macro2::Group` will match the pattern. Mixing wildcard and other
/// tokens within a single `proc_macro2::Group` is not (yet?) supported.
///
/// Examples where matching succeeds:
/// ```rust
///    assert_cc_matches!(
///       quote!{ void foo() {} },
///       quote!{ void foo() {} });
///    assert_cc_matches!(
///       quote!{ void foo() {} },
///       quote!{ foo() });
/// ```
///
/// Example where matching fails:
/// ```rust
///    assert_cc_matches!(
///       quote!{ void foo() { bar(); baz(); } },
///       quote!{ void foo() {...} });
/// ```
#[macro_export]
macro_rules! assert_cc_matches {
    ($input:expr, $pattern:expr) => {
        $crate::match_tokens(&$input, &$pattern, $crate::tokens_to_string)
            .expect("input unexpectedly didn't match the pattern");
    };
}

/// Like `assert_cc_matches!`, but also formats the input in the error message
/// using rustfmt.
#[macro_export]
macro_rules! assert_rs_matches {
    ($input:expr, $pattern:expr) => {
        $crate::match_tokens(&$input, &$pattern, $crate::rs_tokens_to_formatted_string)
            .expect("input unexpectedly didn't match the pattern");
    };
}

/// Asserts that the `input` does not contain the `pattern`.
///
/// Pattern can use `...` wildcard. See `assert_cc_matches` for details.
#[macro_export]
macro_rules! assert_cc_not_matches {
    ($input:expr, $pattern:expr) => {
        $crate::match_tokens(&$input, &$pattern, tokens_to_string)
            .expect_err("input unexpectedly matched the pattern");
    };
}

/// Like `assert_cc_not_matches!`, but also formats the input in the error
/// message using rustfmt.
#[macro_export]
macro_rules! assert_rs_not_matches {
    ($input:expr, $pattern:expr) => {
        $crate::match_tokens(&$input, &$pattern, rs_tokens_to_formatted_string)
            .expect_err("input unexpectedly matched the pattern");
    };
}

#[derive(Debug)]
enum MatchInfo {
    Match,
    Mismatch(Mismatch),
}

#[derive(Debug)]
struct Mismatch {
    match_length: usize,
    messages: Vec<String>,
}

pub fn match_tokens<ToStringFn>(
    input: &TokenStream,
    pattern: &TokenStream,
    to_string_fn: ToStringFn,
) -> Result<()>
where
    ToStringFn: Fn(TokenStream) -> Result<String>,
{
    let iter = input.clone().into_iter();
    let mut best_mismatch = Mismatch {
        match_length: 0,
        messages: vec!["not even a trivial match of the pattern throughout the input".to_string()],
    };

    let mut stack = vec![iter];
    while let Some(mut iter) = stack.pop() {
        loop {
            match match_prefix(iter.clone(), pattern.clone()) {
                (MatchInfo::Match, _) => return Ok(()),
                (MatchInfo::Mismatch(mismatch), _) => {
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

fn is_newline_token(token: &TokenTree) -> bool {
    matches!(token, TokenTree::Ident(ref id) if id == "__NEWLINE__")
}

fn is_wildcard(group: &Group) -> bool {
    format!("{}", group.stream()) == "..."
}

/// Returns `MatchInfo` and the suffix of the `input` token stream that follows
/// the (partial) match.
fn match_prefix(input: token_stream::IntoIter, pattern: TokenStream) -> (MatchInfo, TokenStream) {
    let mut input_iter = input.clone().peekable();
    let mut pattern_iter = pattern.clone().into_iter().peekable();
    let mut match_counter = 0;
    while let Some(actual_token) = input_iter.next() {
        if is_newline_token(&actual_token) {
            continue;
        }

        if let Some(pattern_token) = pattern_iter.next() {
            if let MatchInfo::Mismatch(mut mismatch) = match_tree(&actual_token, &pattern_token) {
                mismatch.messages.push(format!(
                    "expected '{}' got '{}'",
                    pattern,
                    input.collect::<TokenStream>()
                ));
                mismatch.match_length += match_counter;
                return (
                    MatchInfo::Mismatch(mismatch),
                    iter::once(actual_token).chain(input_iter).collect::<TokenStream>(),
                );
            }
        } else {
            return (
                MatchInfo::Match,
                iter::once(actual_token).chain(input_iter).collect::<TokenStream>(),
            );
        }
        match_counter += 1;
    }

    return if pattern_iter.peek().is_none() {
        (MatchInfo::Match, TokenStream::new())
    } else {
        (
            MatchInfo::Mismatch(Mismatch {
                match_length: match_counter,
                messages: vec![
                    format!(
                        "expected '{}' but the input already ended",
                        pattern_iter.collect::<TokenStream>()
                    ),
                    format!("expected '{}' got '{}'", pattern, input.collect::<TokenStream>()),
                ],
            }),
            TokenStream::new(),
        )
    };
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
            if is_wildcard(pattern_group) {
                return MatchInfo::Match;
            }
            let (match_info, actual_group_suffix) =
                match_prefix(actual_group.stream().into_iter(), pattern_group.stream());
            match match_info {
                MatchInfo::Match => {
                    if actual_group_suffix
                        .clone()
                        .into_iter()
                        .filter(|token| !is_newline_token(token))
                        .count()
                        != 0
                    {
                        MatchInfo::Mismatch(Mismatch {
                            match_length: 0,
                            messages: vec![format!(
                                "matched the entire pattern but the input still contained '{}'",
                                actual_group_suffix
                            )],
                        })
                    } else {
                        MatchInfo::Match
                    }
                }
                mismatch => mismatch,
            }
        }
        (ref actual, ref pattern) => {
            let actual_src = format!("{}", actual);
            let pattern_src = format!("{}", pattern);
            if actual_src == pattern_src {
                MatchInfo::Match
            } else {
                MatchInfo::Mismatch(Mismatch {
                    match_length: 0,
                    messages: vec![format!("expected '{}' but got '{}'", pattern, actual)],
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    macro_rules! assert_rs_cc_matches {
        ($input:expr, $pattern:expr) => {
            $crate::assert_cc_matches!($input, $pattern);
            $crate::assert_rs_matches!($input, $pattern);
        };
    }

    #[test]
    fn test_assert_cc_not_matches_accepts_not_matching_pattern() {
        assert_cc_not_matches!(quote! { fn foo() {} }, quote! { fn bar() {} });
        assert_rs_not_matches!(quote! { fn foo() {} }, quote! { fn bar() {} });
    }

    #[test]
    #[should_panic]
    fn test_assert_cc_not_matches_panics_on_match() {
        assert_cc_not_matches!(quote! { fn foo() {} }, quote! { fn foo() {} });
    }

    #[test]
    #[should_panic]
    fn test_assert_rs_not_matches_panics_on_match() {
        assert_rs_not_matches!(quote! { fn foo() {} }, quote! { fn foo() {} });
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
                    tokens_to_string
                )
                .expect_err("unexpected match")
            ),
            "expected 'B' but got 'A'

Caused by:
    0: expected 'struct B' got 'struct A { int a ; int b ; } ;'
    1: input:\n       \n       ```
       struct A{ int a ; int b ; };
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
                    rs_tokens_to_formatted_string
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
                    tokens_to_string
                )
                .expect_err("unexpected match")
            ),
            "expected 'struct X { }' but the input already ended: \
                expected 'fn foo () { } struct X { }' got 'fn foo () { }': \
                input:\n\n```\nfn foo(){ }\n```"
        );
    }

    #[test]
    fn test_reject_different_delimiters() {
        assert_eq!(
            format!(
                "{:#}",
                match_tokens(&quote! {fn foo() ()}, &quote! {fn foo() {}}, tokens_to_string)
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
                    tokens_to_string
                )
                .expect_err("unexpected match")
            ),
            "expected 'c' but got 'b': \
            expected 'a : i64 , c : i64' got 'a : i64 , b : i64': \
            expected 'fn foo () { a : i64 , c : i64 }' got 'fn foo () { a : i64 , b : i64 }': \
            input:\n\n```\nfn foo(){ a : i64 , b : i64 }\n```"
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
            __NEWLINE__ a: __NEWLINE__ usize) {}},
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
                    tokens_to_string
                )
                .expect_err("unexpected match")
            ),
            "matched the entire pattern but the input still contained 'drop_impl () ;': \
                expected 'fn drop (& mut self) { }' got 'fn drop (& mut self) { drop_impl () ; }': \
                input:\n\n```\nimpl Drop{ fn drop (& mut self) { drop_impl () ; } }\n```"
        );
        assert_eq!(
            format!(
                "{:#}",
                match_tokens(
                    &quote! {impl Drop { fn drop(&mut self) { drop_impl1(); drop_impl2(); }}},
                    &quote! {fn drop(&mut self) { drop_impl1(); }},
                    tokens_to_string
                )
                .expect_err("unexpected match")
            ),
            "matched the entire pattern but the input still contained 'drop_impl2 () ;': \
                expected 'fn drop (& mut self) { drop_impl1 () ; }' \
                got 'fn drop (& mut self) { drop_impl1 () ; drop_impl2 () ; }': \
                input:\n\n```\nimpl Drop{ fn drop (& mut self) { \
                    drop_impl1 () ; drop_impl2 () ; } }\n```"
        );
    }

    #[test]
    fn test_accept_unfinished_input_with_only_newlines() {
        assert_rs_cc_matches!(quote! {fn foo() { __NEWLINE__ }}, quote! {fn foo() {}});
        assert_rs_cc_matches!(quote! {fn foo() { a(); __NEWLINE__ }}, quote! {fn foo() { a(); }});
    }

    #[test]
    fn test_wildcard_in_the_middle_of_the_group_not_supported_yet() {
        assert_eq!(
            format!(
                "{:#}",
                match_tokens(
                    &quote! { fn foo() -> bool { return true; }},
                    &quote! { fn foo() -> bool { return ... } },
                    tokens_to_string
                )
                .expect_err("unexpected match")
            ),
            "expected '.' but got 'true': \
            expected 'return ...' got 'return true ;': \
            expected 'fn foo () -> bool { return ... }' got 'fn foo () -> bool { return true ; }': \
            input:\n\n```\nfn foo()->bool{ return true ; }\n```"
        );
    }
}

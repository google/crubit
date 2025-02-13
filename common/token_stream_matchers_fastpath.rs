// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//! A finite state automaton based approach for matching token streams.
//!
//! This is significantly faster than recursive backtracking, but the error
//! messages are not as good, so this is only used as a fast path.
//! When a failure message is required, we rerun the match with a recursive
//! backtracking implementation.
#![allow(clippy::result_unit_err)]
#![deny(missing_docs)]
use proc_macro2::Delimiter;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use std::rc::Rc;

/// Matches the pattern against the input, returning Ok if and only if the pattern matches.
///
/// Patterns follow the syntax described in token_stream_matchers.rs. For example,
/// `quote!{a ... b}` matches any sequence of tokens that contains an `a` somewhere before a `b`.
///
/// This doesn't return a failure message, because the failure messages it could generate are not
/// very good.
pub fn match_tokens_fast(input: &TokenStream, pattern: &TokenStream) -> Result<(), ()> {
    Pattern::from_tokenstream(pattern.clone(), true).recursive_glob_match(input.clone())
}

/// Returns true if the token represents whitespace in Crubit's token stringification algorithm.
#[must_use]
pub fn is_whitespace_token(token: &TokenTree) -> bool {
    matches!(token, TokenTree::Ident(id) if id == "__NEWLINE__" || id == "__SPACE__")
}

/// A pattern that can exist within a Group or at the top level.
#[derive(Clone, Debug)]
struct Pattern(Rc<[Atom]>);

#[derive(Clone, Debug)]
enum Atom {
    /// Any sequence of token trees.
    DotStar,
    /// Any sequence of token trees -- implicitly added to the start/end of
    /// the pattern.
    ImplicitDotStar,
    /// A single TokenTree
    TokenTree(TokenTree),
    /// A Group token tree (e.g. [], (), {}), with a pattern inside of it.
    Group(Delimiter, Pattern),
}

impl Pattern {
    fn from_tokenstream(tokens: TokenStream, is_root: bool) -> Self {
        let mut pattern = vec![];
        // Patterns implicitly start and end with a `...`.
        if is_root {
            pattern.push(Atom::ImplicitDotStar);
        }
        fn extract_triple_dot(pattern: &mut Vec<Atom>) {
            if pattern.len() < 3 {
                return;
            }
            for tt_pattern in &pattern[pattern.len() - 3..] {
                let Atom::TokenTree(TokenTree::Punct(x)) = tt_pattern else {
                    return;
                };
                if x.as_char() != '.' {
                    return;
                }
            }
            pattern.truncate(pattern.len() - 3);
            pattern.push(Atom::DotStar);
        }
        for token in tokens {
            match token {
                TokenTree::Group(g) => pattern
                    .push(Atom::Group(g.delimiter(), Self::from_tokenstream(g.stream(), false))),
                token_tree => pattern.push(Atom::TokenTree(token_tree)),
            }
            extract_triple_dot(&mut pattern);
        }
        if is_root {
            pattern.push(Atom::ImplicitDotStar);
        }
        Pattern(pattern.into())
    }

    /// Finite state glob algorithm, as described in e.g. https://research.swtch.com/glob.
    ///
    /// Note that patterns are globs, or close enough.
    /// The pattern `{... a ...}` only matches a token tree of length 1 (a
    /// Group), and whether it matches does not depend on context.
    ///
    /// So the glob algorithm works here as well. We can take the leftmost
    /// match, and give maximum freedom to subsequent match
    /// attempts.
    fn glob_match(&self, input: TokenStream) -> Result<(), ()> {
        #[derive(Copy, Clone, Debug)]
        struct State {
            /// px/nextPx from https://research.swtch.com/glob
            pattern: usize,
            /// nx/nextNx
            input: usize,
        }

        // TODO(jeanpierreda): Can we push this upwards into recursive_glob_match?
        let input: Vec<TokenTree> =
            input.into_iter().filter(|token| !is_whitespace_token(token)).collect();
        let mut state = State { pattern: 0, input: 0 };
        let mut backtrack = state;
        let can_backtrack =
            |backtrack: State| -> bool { backtrack.input > 0 && backtrack.input <= input.len() };
        while state.pattern < self.0.len() || state.input < input.len() {
            match self.0.get(state.pattern) {
                Some(Atom::TokenTree(tt)) => {
                    if let Some(next) = input.get(state.input) {
                        match match_tree(next, tt) {
                            Ok(()) => {
                                state.pattern += 1;
                                state.input += 1;
                                continue;
                            }
                            // we won't backtrack, so let's report this error up.
                            Err(e) if !can_backtrack(backtrack) => return Err(e),
                            // fall back to restart logic below.
                            Err(_e) => {}
                        }
                    }
                }
                Some(Atom::Group(delimiter, nested_pattern)) => {
                    if let Some(TokenTree::Group(g)) = input.get(state.input) {
                        if g.delimiter() == *delimiter {
                            match nested_pattern.glob_match(g.stream()) {
                                Ok(()) => {
                                    state.pattern += 1;
                                    state.input += 1;
                                    continue;
                                }
                                // we won't backtrack, so let's report this error up.
                                Err(e) if !can_backtrack(backtrack) => return Err(e),
                                // fall back to restart logic below.
                                Err(_e) => {}
                            }
                        }
                    }
                }
                Some(Atom::DotStar | Atom::ImplicitDotStar) => {
                    // Next time, restart here, but consuming an extra thing into the wildcard.
                    // (This guarantees forward progress!)
                    backtrack = state;
                    backtrack.input += 1;
                    state.pattern += 1;
                    continue;
                }
                None => {}
            }
            // restart, we failed.
            if can_backtrack(backtrack) {
                state = backtrack;
                continue;
            }
            if state.pattern < self.0.len() {
                // could not find self.0[state.pattern..]
                return Err(());
            } else {
                // pattern did not match against input[state.input..].
                return Err(());
            }
        }
        Ok(())
    }

    /// Like glob_match, but searches into groups.
    fn recursive_glob_match(&self, input: TokenStream) -> Result<(), ()> {
        if let Ok(()) = self.glob_match(input.clone()) {
            return Ok(());
        }

        let mut stack = vec![input];
        while let Some(input) = stack.pop() {
            for tt in input {
                if let TokenTree::Group(g) = tt {
                    if let Ok(()) = self.glob_match(g.stream()) {
                        return Ok(());
                    }
                    stack.push(g.stream());
                }
            }
        }
        Err(())
    }
}

/// Returns Ok if the two token trees are equal, otherwise returns Err.
fn match_tree(actual: &TokenTree, pattern: &TokenTree) -> Result<(), ()> {
    // Note: this is only called if the `actual` is a Group, or neither are groups,
    // so it should hopefully be pretty fast most of the time.
    let actual_src = format!("{}", actual);
    let pattern_src = format!("{}", pattern);
    if actual_src == pattern_src {
        Ok(())
    } else {
        Err(())
    }
}

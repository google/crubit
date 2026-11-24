// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Token};

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}

impl Parse for Date {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let year: syn::LitInt = input.parse()?;
        input.parse::<Token![-]>()?;
        let month: syn::LitInt = input.parse()?;
        input.parse::<Token![-]>()?;
        let day: syn::LitInt = input.parse()?;

        Ok(Date {
            year: year.base10_parse()?,
            month: month.base10_parse()?,
            day: day.base10_parse()?,
        })
    }
}

impl quote::ToTokens for Date {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let year = proc_macro2::Literal::u32_unsuffixed(self.year);
        let month = proc_macro2::Literal::u32_unsuffixed(self.month);
        let day = proc_macro2::Literal::u32_unsuffixed(self.day);
        tokens.extend(quote! {#year-#month-#day });
    }
}

/// Maps the last nightly date included in each stable release to that release.
///
/// Each date is the latest nightly `commit-date` (as reported by `rustc -vV`, which is what
/// `rustversion` compares against) included in the release, i.e. the date of the release's branch
/// point from main. This is usually the day before the corresponding nightly's name:
/// `nightly-2026-08-15` is `1.99.0-nightly (d453bdd8f 2026-08-14)`, and `nightly-2026-08-16` is the
/// first `1.100.0-nightly`. When nightlies around the branch point were skipped (e.g. 2026-07-03
/// and 2026-07-04), the branch point can be later than the last published nightly, so verify
/// against the actual stable toolchain.
///
/// Note that a stable release's own `commit-date` is its release build date (~6 weeks after the
/// branch point), so it cannot be used for this purpose.
static RELEASES: &[(Date, &str)] = &[
    // <internal link> start numeric=yes
    (Date { year: 2026, month: 7, day: 3 }, "1.98"),
    (Date { year: 2026, month: 8, day: 14 }, "1.99"),
    (Date { year: 2026, month: 9, day: 25 }, "1.100"),
    (Date { year: 2026, month: 11, day: 6 }, "1.101"),
    // fallback
    (Date { year: 9999, month: 99, day: 99 }, "1.102"),
    // <internal link> end
];

fn lookup_version(arg: &Date) -> &'static str {
    let index = match RELEASES.binary_search_by_key(arg, |&(date, _)| date) {
        Ok(i) => i,
        Err(i) => i,
    };
    RELEASES[index].1
}

#[derive(PartialEq, Eq, Debug)]
enum Predicate {
    Since(Date),
    Before(Date),
    Wildcard,
}

impl Predicate {
    fn to_rustversion_expr(&self) -> Option<proc_macro2::TokenStream> {
        match self {
            Predicate::Since(date) => {
                let version = proc_macro2::TokenStream::from_str(lookup_version(date))
                    .expect("RELEASES entries are valid version literals");
                Some(quote!(any(all(nightly, since(#date)), all(not(nightly), since(#version)))))
            }
            Predicate::Before(date) => {
                let version = proc_macro2::TokenStream::from_str(lookup_version(date))
                    .expect("RELEASES entries are valid version literals");
                Some(quote!(any(all(nightly, before(#date)), all(not(nightly), before(#version)))))
            }
            Predicate::Wildcard => None,
        }
    }
}

impl Parse for Predicate {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![_]) {
            input.parse::<Token![_]>()?;
            return Ok(Predicate::Wildcard);
        }
        let ident: syn::Ident = input.parse()?;
        let content;
        syn::parenthesized!(content in input);
        let date: Date = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("unexpected tokens after date"));
        }
        match ident.to_string().as_str() {
            "since" => Ok(Predicate::Since(date)),
            "before" => Ok(Predicate::Before(date)),
            _ => Err(syn::Error::new(ident.span(), "expected `since`, `before`, or `_`")),
        }
    }
}

enum ArmBody {
    Braced(Vec<syn::Stmt>),
    Expr(Box<syn::Expr>),
}

struct SelectArm {
    predicate: Predicate,
    body: ArmBody,
}

struct SelectInput {
    arms: Vec<SelectArm>,
}

impl Parse for SelectInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut arms = Vec::new();
        while !input.is_empty() {
            let predicate: Predicate = input.parse()?;
            input.parse::<Token![=>]>()?;
            let body = if input.peek(syn::token::Brace) {
                let content;
                syn::braced!(content in input);
                let stmts = syn::Block::parse_within(&content)?;
                let _ = input.parse::<Option<Token![,]>>()?;
                ArmBody::Braced(stmts)
            } else {
                let expr: syn::Expr = input.parse()?;
                let expr_is_block = matches!(
                    expr,
                    syn::Expr::Block(_)
                        | syn::Expr::If(_)
                        | syn::Expr::Match(_)
                        | syn::Expr::Loop(_)
                        | syn::Expr::While(_)
                        | syn::Expr::ForLoop(_)
                        | syn::Expr::Unsafe(_)
                );
                if !input.is_empty() {
                    if expr_is_block {
                        let _ = input.parse::<Option<Token![,]>>()?;
                    } else {
                        input.parse::<Token![,]>()?;
                    }
                }
                ArmBody::Expr(Box::new(expr))
            };
            if predicate == Predicate::Wildcard && !input.is_empty() {
                return Err(input.error("`_` wildcard arm must be the last arm"));
            }
            arms.push(SelectArm { predicate, body });
        }
        Ok(SelectInput { arms })
    }
}

fn make_attr(conds: &[proc_macro2::TokenStream]) -> proc_macro2::TokenStream {
    if conds.is_empty() {
        quote!()
    } else {
        quote!(#[::rustversion::macro_internal::all(#(#conds),*)])
    }
}

fn token_count(ts: proc_macro2::TokenStream) -> usize {
    ts.into_iter()
        .map(|tt| match tt {
            proc_macro2::TokenTree::Group(g) => 1 + token_count(g.stream()),
            _ => 1,
        })
        .sum()
}

fn expand_select(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    // A nested `rustversion::select!` always has strictly fewer tokens than its enclosing
    // `rustversion::select!`, so using the token count guarantees unique helper macro names.
    let count = token_count(input.clone());
    let helper_macro = proc_macro2::Ident::new(
        &format!("__rustversion_select_{count}"),
        proc_macro2::Span::call_site(),
    );
    let parsed: SelectInput = syn::parse2(input)?;
    let used_as_expr = parsed.arms.iter().any(|arm| match &arm.body {
        ArmBody::Expr(_) => true,
        ArmBody::Braced(stmts) => match stmts.last() {
            Some(syn::Stmt::Expr(_, None)) => true,
            Some(syn::Stmt::Macro(m)) => m.semi_token.is_none(),
            _ => false,
        },
    });

    let mut out = proc_macro2::TokenStream::new();
    let mut prev_conds: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut has_wildcard = false;

    for arm in parsed.arms {
        let mut conds: Vec<proc_macro2::TokenStream> =
            prev_conds.iter().map(|c| quote!(not(#c))).collect();
        if let Some(cond) = arm.predicate.to_rustversion_expr() {
            conds.push(cond.clone());
            prev_conds.push(cond);
        } else {
            has_wildcard = true;
        }
        let attr = make_attr(&conds);

        if used_as_expr {
            let body = match arm.body {
                ArmBody::Expr(expr) => quote!(#expr),
                ArmBody::Braced(stmts) => quote!({ #(#stmts)* }),
            };
            out.extend(quote! {
                #attr
                macro_rules! #helper_macro { () => { #body } }
            });
        } else if let ArmBody::Braced(stmts) = arm.body {
            for stmt in stmts {
                out.extend(quote! {
                    #attr
                    #stmt
                });
            }
        }
    }

    if !has_wildcard {
        let not_prev: Vec<proc_macro2::TokenStream> =
            prev_conds.iter().map(|c| quote!(not(#c))).collect();
        let attr = make_attr(&not_prev);
        let err = quote!(::core::compile_error!(
            "none of the predicates in this `rustversion::select!` evaluated to true"
        ));
        if used_as_expr {
            out.extend(quote! {
                #attr
                macro_rules! #helper_macro { () => { #err } }
            });
        } else {
            out.extend(quote! {
                #attr
                #err;
            });
        }
    }

    if used_as_expr {
        Ok(quote! {
            {
                #out
                #helper_macro!()
            }
        })
    } else {
        Ok(out)
    }
}

#[proc_macro_attribute]
pub fn since(args: TokenStream, input: TokenStream) -> TokenStream {
    let date = parse_macro_input!(args as Date);
    let input: proc_macro2::TokenStream = input.into();
    let version = proc_macro2::TokenStream::from_str(lookup_version(&date)).unwrap();

    quote! {
        #[::rustversion::macro_internal::any(all(nightly, since(#date)), all(not(nightly), since(#version)))]
        #input
    }.into()
}

#[proc_macro_attribute]
pub fn before(args: TokenStream, input: TokenStream) -> TokenStream {
    let date = parse_macro_input!(args as Date);
    let input: proc_macro2::TokenStream = input.into();
    let version = proc_macro2::TokenStream::from_str(lookup_version(&date)).unwrap();

    quote! {
        #[::rustversion::macro_internal::any(all(nightly, before(#date)), all(not(nightly), before(#version)))]
        #input
    }.into()
}

#[proc_macro]
pub fn select(input: TokenStream) -> TokenStream {
    expand_select(input.into()).unwrap_or_else(syn::Error::into_compile_error).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_releases_sorted() {
        for w in RELEASES.windows(2) {
            expect_lt!(w[0].0, w[1].0);
        }
    }

    #[gtest]
    fn test_select_items_expansion() {
        let out = expand_select(quote! {
            since(2026-09-27) => { use a::B; }
            since(2026-08-09) => { use c::D; }
            _ => { use e::F; }
        })
        .unwrap();
        let expected = quote! {
            #[::rustversion::macro_internal::all(
                any(all(nightly, since(2026-9-27)), all(not(nightly), since(1.101)))
            )]
            use a::B;
            #[::rustversion::macro_internal::all(
                not(any(all(nightly, since(2026-9-27)), all(not(nightly), since(1.101)))),
                any(all(nightly, since(2026-8-9)), all(not(nightly), since(1.99)))
            )]
            use c::D;
            #[::rustversion::macro_internal::all(
                not(any(all(nightly, since(2026-9-27)), all(not(nightly), since(1.101)))),
                not(any(all(nightly, since(2026-8-9)), all(not(nightly), since(1.99))))
            )]
            use e::F;
        };
        expect_eq!(out.to_string(), expected.to_string());
    }

    #[gtest]
    fn test_select_wildcard_not_last_errors() {
        let err = expand_select(quote! {
            _ => old_fn(),
            since(2026-08-15) => new_fn(),
        })
        .unwrap_err();
        expect_that!(err.to_string(), contains_substring("last arm"));
    }
}

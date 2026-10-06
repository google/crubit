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
    (Date { year: 2026, month: 4, day: 10 }, "1.96"),
    (Date { year: 2026, month: 5, day: 22 }, "1.97"),
    (Date { year: 2026, month: 7, day: 3 }, "1.98"),
    (Date { year: 2026, month: 8, day: 14 }, "1.99"),
    (Date { year: 2026, month: 9, day: 25 }, "1.100"),
    // fallback
    (Date { year: 9999, month: 99, day: 99 }, "1.101"),
    // <internal link> end
];

fn lookup_version(arg: &Date) -> &'static str {
    let index = match RELEASES.binary_search_by_key(arg, |&(date, _)| date) {
        Ok(i) => i,
        Err(i) => i,
    };
    RELEASES[index].1
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_releases_sorted() {
        for w in RELEASES.windows(2) {
            assert!(w[0].0 < w[1].0, "{:?} >= {:?}", w[0].0, w[1].0);
        }
    }
}

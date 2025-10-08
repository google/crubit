// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::{anyhow, ensure, Result};
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::rc::Rc;

use dyn_format::Format;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CcConstQualifier {
    Mut,
    Const,
}

/// Returns Some(..) if the cpp_type is a C++ pointer type.
pub fn is_cpp_pointer_type(cpp_type: TokenStream) -> Option<CcConstQualifier> {
    let mut first = None;
    let mut last = None;
    let mut prev_last = None;
    for token in cpp_type.into_iter() {
        first = if first.is_none() { Some(token.clone()) } else { first };
        prev_last = last;
        last = Some(token);
    }

    match (first, prev_last, last) {
        (Some(TokenTree::Ident(first)), _, Some(TokenTree::Punct(last)))
            if first == "const" && last.as_char() == '*' =>
        {
            Some(CcConstQualifier::Const)
        }
        (_, Some(TokenTree::Ident(prev_last)), Some(TokenTree::Punct(last)))
            if prev_last == "const" && last.as_char() == '*' =>
        {
            Some(CcConstQualifier::Const)
        }
        (_, _, Some(TokenTree::Punct(last))) if last.as_char() == '*' => {
            Some(CcConstQualifier::Mut)
        }
        _ => None,
    }
}

/// Returns true if the given identifier is a C++ reserved keyword according to
/// https://en.cppreference.com/w/cpp/keyword
pub fn is_cpp_reserved_keyword(ident: &str) -> bool {
    static RESERVED_CC_KEYWORDS: phf::Set<&'static str> = phf::phf_set! {
        "alignas",
        "alignof",
        "and",
        "and_eq",
        "asm",
        "atomic_cancel",
        "atomic_commit",
        "atomic_noexcept",
        "auto",
        "bitand",
        "bitor",
        "bool",
        "break",
        "case",
        "catch",
        "char",
        "char8_t",
        "char16_t",
        "char32_t",
        "class",
        "compl",
        "concept",
        "const",
        "consteval",
        "constexpr",
        "constinit",
        "const_cast",
        "continue",
        "co_await",
        "co_return",
        "co_yield",
        "decltype",
        "default",
        "delete",
        "do",
        "double",
        "dynamic_cast",
        "else",
        "enum",
        "explicit",
        "export",
        "extern",
        "false",
        "float",
        "for",
        "friend",
        "goto",
        "if",
        "inline",
        "int",
        "long",
        "mutable",
        "namespace",
        "new",
        "noexcept",
        "not",
        "not_eq",
        "nullptr",
        "operator",
        "or",
        "or_eq",
        "private",
        "protected",
        "public",
        "reflexpr",
        "register",
        "reinterpret_cast",
        "requires",
        "return",
        "short",
        "signed",
        "sizeof",
        "static",
        "static_assert",
        "static_cast",
        "struct",
        "switch",
        "synchronized",
        "template",
        "this",
        "thread_local",
        "throw",
        "true",
        "try",
        "typedef",
        "typeid",
        "typename",
        "union",
        "unsigned",
        "using",
        "virtual",
        "void",
        "volatile",
        "wchar_t",
        "while",
        "xor",
        "xor_eq",
    };

    RESERVED_CC_KEYWORDS.contains(ident)
}

/// If `ident` is a reserved C++ keyword, returns a string with an underscore appended to it.
/// Otherwise, returns `ident`.
pub fn unkeyword_cpp_ident(ident: &str) -> Cow<'_, str> {
    if is_cpp_reserved_keyword(ident) {
        Cow::Owned(format!("{ident}_"))
    } else {
        Cow::Borrowed(ident)
    }
}

/// Formats a C++ identifier. Panics if `ident` is a C++ reserved keyword.
#[track_caller]
pub fn expect_format_cc_ident(ident: &str) -> Ident {
    format_cc_ident(ident)
        .unwrap_or_else(|err| panic!("Can't format `{ident}` as a C++ identifier: {err}"))
}

/// Fallibly parses a [`proc_macro2::Ident`] where Rust keywords are permitted.
///
/// * Unlike [`Ident::new`], this is fallible and returns Err if it fails.
/// * Unlike [`syn::parse_str::<Ident>`], Rust keywords are permitted, making this suitable for
///   parsing identifiers that that Rust doesn't allow but C++ does, like "move".
fn parse_any(ident: &str) -> syn::Result<Ident> {
    syn::parse::Parser::parse_str(<Ident as syn::ext::IdentExt>::parse_any, ident)
}

/// Formats a C++ (qualified) identifier. Returns an error when `ident` is a C++
/// reserved keyword or is an invalid identifier.
pub fn format_cc_ident(ident: &str) -> Result<Ident> {
    ensure!(!ident.is_empty(), "Empty string is not a valid C++ identifier");
    ensure!(
        !is_cpp_reserved_keyword(ident),
        "`{ident}` is a C++ reserved keyword and can't be used as a C++ identifier",
    );
    // Explicitly mapping the error via `anyhow!`, because `LexError` is not `Sync`
    // (required for `anyhow::Error` to implement `From<LexError>`) and
    // therefore we can't just use `?`.
    parse_any(ident)
        .map_err(|lex_error| anyhow!("Can't format `{ident}` as a C++ identifier: {lex_error}"))
}

/// Formats a C++ type name. Panics if `name` is not a valid type name in C++.
#[track_caller]
pub fn expect_format_cc_type_name(name: &str) -> TokenStream {
    format_cc_type_name(name).expect("IR should only contain valid C++ types")
}

/// Formats a C++ type name. Returns an error when `name` is a C++
/// reserved keyword or otherwise an invalid type name.
pub fn format_cc_type_name(name: &str) -> Result<TokenStream> {
    check_valid_cc_name(name)?;
    match name.parse() {
        Ok(name) => Ok(name),
        Err(_) => {
            // Sometimes valid C++ type names do not parse as TokenStreams.
            // For example, this can happen in the case of literals that are not
            // valid Rust: IntTemplateStruct<'\f'>.
            //
            // As a last ditch effort, we convert the identifier into a string literal.
            // We could use quote! to do the escaping, but instead we do it manually to
            // mirror the code in token_stream_printer.
            let name = name.replace("\\", "\\\\").replace('"', "\\\"");
            let name = format!("__LITERALLY__ \"{name}\"");
            name.parse()
                .map_err(|lex_error| anyhow!("Can't format `{name}` as a C++ type: {lex_error}"))
        }
    }
}

/// Makes an 'Ident' to be used in the Rust source code. Escapes Rust keywords.
/// Panics if `ident` is empty or is otherwise an invalid identifier.
pub fn make_rs_ident(ident: &str) -> Ident {
    // TODO(https://github.com/dtolnay/syn/pull/1098): Remove the hardcoded list once syn recognizes
    // newly added keywords.
    // NOTE: the above PR was accepted for 2021 and 2018 editions, but `try` still isn't escaped,
    // so we may need to tweak something.
    if matches!(ident, "gen" | "async" | "await" | "try" | "dyn") {
        return format_ident!("r#{}", ident);
    }
    match syn::parse_str::<syn::Ident>(ident) {
        Ok(_) => format_ident!("{}", ident),
        Err(_) => format_ident!("r#{}", ident),
    }
}

pub fn check_valid_cc_name(name: &str) -> Result<()> {
    // C++ doesn't have an equivalent of
    // https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html and therefore
    // an error is returned when `ident` is a C++ reserved keyword.
    ensure!(
        !is_cpp_reserved_keyword(name),
        "`{}` is a C++ reserved keyword and can't be used as a C++ identifier",
        name
    );

    // https://en.cppreference.com/w/cpp/language/identifiers says that "A valid identifier must
    // begin with a non-digit character (Latin letter, underscore, or Unicode
    // character of class XID_Start)".  One motivation for this check is to
    // explicitly catch names of tuple fields (e.g. `some_tuple.0`).
    let first_char =
        name.chars().next().ok_or_else(|| anyhow!("Empty string is not a valid C++ identifier"))?;
    ensure!(
        unicode_ident::is_xid_start(first_char) || first_char == '_' || first_char == ':',
        "The following character can't be used as a start of a C++ identifier: {first_char}",
    );

    Ok(())
}

/// Escapes characters that may not appear in a C++ or Rust identifier.
///
/// The implemented escaping algorithm guarantess that different inputs will
/// always produce different outputs (i.e. unique symbols will remain unique
/// after escaping).  Other than that, the implemented escaping algorithm is
/// somewhat arbitrary and should be treated as an implementation detail and not
/// depended upon.
///
/// This transformation allows using escaped symbol names as part of Rust and/or
/// C++ identifiers. In particular note that in practice Rust uses `$` and `.`
/// characters in symbols - for example: "_ZN58_$LT$rust_out..
/// Point$u20$as$u20$core..default..Default$GT$7default17h144069f0ad7be325E".
pub fn escape_non_identifier_chars(symbol: &str) -> String {
    // EXTRA_CAPACITY_PREDICTION has been haphazardly chosen based on a single
    // example encountered in practice where there were 16 characters that needed
    // escaping: 2 x '_', 8 x '$', 6 x '.': "_ZN58_$LT$rust_out..
    // Point$u20$as$u20$core..default..Default$GT$7default17h144069f0ad7be325E"
    const EXTRA_CAPACITY_PREDICTION: usize = 20;
    let mut result = String::with_capacity(symbol.len() + EXTRA_CAPACITY_PREDICTION);

    for (i, c) in symbol.chars().enumerate() {
        match c {
            '_' => result.push_str("_u"),
            '$' => result.push_str("_d"),
            '.' => result.push_str("_p"),
            c => {
                let is_valid_identifier_char = if i == 0 {
                    // `is_xid_start` doesn't cover `'_'` character, but it is okay because we
                    // explicitly handle this character in a match branch above.
                    unicode_ident::is_xid_start(c)
                } else {
                    unicode_ident::is_xid_continue(c)
                };
                if is_valid_identifier_char {
                    result.push(c);
                } else {
                    _ = write!(&mut result, "_x{:08x}", c as u32);
                };
            }
        }
    }

    result
}

/// Representation of `foo::bar::baz` where each component is either the name
/// of a C++ namespace, or the name of a Rust module.
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct NamespaceQualifier {
    // Outer to innermost
    pub namespaces: Vec<Rc<str>>,
    // Outer to innermost. Paired as (rs_name, cc_name)
    pub nested_records: Vec<(Rc<str>, Rc<str>)>,
}

impl NamespaceQualifier {
    /// Constructs a new `NamespaceQualifier` from a sequence of names.
    pub fn new<T: Into<Rc<str>>>(iter: impl IntoIterator<Item = T>) -> Self {
        // TODO(b/258265044): Catch most (all if possible) error conditions early.  For
        // example:
        // - Panic early if any strings are empty, or are not Rust identifiers
        // - Report an error early if any strings are C++ reserved keywords
        // This may make `format_for_cc` and `format_namespace_bound_cc_tokens` infallible.
        Self { namespaces: iter.into_iter().map(Into::into).collect(), nested_records: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.namespaces.is_empty() && self.nested_records.is_empty()
    }

    /// Returns an iterator over the Rust modules in the namespace qualifier.
    /// Parent records are converted to snake case.
    pub fn parts_with_snake_case_record_names(&self) -> impl Iterator<Item = Ident> + use<'_> {
        self.namespaces.iter().map(|ns| make_rs_ident(ns)).chain(
            self.nested_records
                .iter()
                .map(|(rs_name, _cc_name)| make_rs_ident(&rs_name.to_string().to_snake_case())),
        )
    }

    pub fn parts(&self) -> impl Iterator<Item = &Rc<str>> + use<'_> {
        self.namespaces.iter().chain(self.nested_records.iter().map(|(_rs_name, cc_name)| cc_name))
    }

    /// Returns `foo::bar::baz::` (escaping Rust keywords as needed).
    pub fn format_for_rs(&self) -> TokenStream {
        let rust_parts = self.parts_with_snake_case_record_names();
        quote! { #(#rust_parts::)* }
    }

    /// Returns `foo::bar::baz::` (reporting errors for C++ keywords).
    pub fn format_for_cc(&self) -> Result<TokenStream> {
        let namespace_cc_idents = self.cc_idents()?;
        Ok(quote! { #(#namespace_cc_idents::)* })
    }

    pub fn cc_idents(&self) -> Result<Vec<Ident>> {
        self.parts().map(|ns| format_cc_ident(ns)).collect()
    }
}

/// `CcInclude` represents a single `#include ...` directive in C++.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CcInclude {
    /// Represents a system header, e.g., `cstdef`, which will be included by
    /// angular brackets.
    SystemHeader(Rc<str>),
    /// Represents a user header, which will be included by quotes.
    UserHeader(Rc<str>),
    /// Represents an `#include` for Crubit C++ support library headers: the
    /// format specifier for what comes after `#include` and path of the support
    /// library header.
    SupportLibHeader(Format<1>, Rc<str>),
}

impl CcInclude {
    /// Creates a `CcInclude` that represents `#include <array>` and provides
    /// C++ types like `std::array`.
    /// https://en.cppreference.com/w/cpp/header/array
    pub fn array() -> Self {
        Self::SystemHeader("array".into())
    }

    /// Creates a `CcInclude` that represents `#include <cstddef>` and provides
    /// C++ types like `std::size_t` or `std::ptrdiff_t`.  See
    /// https://en.cppreference.com/w/cpp/header/cstddef
    pub fn cstddef() -> Self {
        Self::SystemHeader("cstddef".into())
    }

    /// Creates a `CcInclude` that represents `#include <cstdint>` and provides
    /// C++ types like `std::int16_t` or `std::uint32_t`.  See
    /// https://en.cppreference.com/w/cpp/header/cstdint
    pub fn cstdint() -> Self {
        Self::SystemHeader("cstdint".into())
    }

    /// Creates a `CcInclude` that represents `#include <memory>`.
    /// See https://en.cppreference.com/w/cpp/header/memory
    pub fn memory() -> Self {
        Self::SystemHeader("memory".into())
    }

    /// Creates a `CcInclude` that represents `#include <utility>` and provides
    /// C++ functions like `std::move` and C++ types like `std::tuple`.
    /// See https://en.cppreference.com/w/cpp/header/utility
    pub fn utility() -> Self {
        Self::SystemHeader("utility".into())
    }

    /// Creates a `CcInclude` that represents `#include <tuple>` and provides
    /// C++ `std::tuple`, `std::tie`, etc.
    /// See https://en.cppreference.com/w/cpp/header/utility
    pub fn tuple() -> Self {
        Self::SystemHeader("tuple".into())
    }

    /// Creates a `CcInclude` that represents `#include <optional>` and provides
    /// C++ `std::optional`.
    /// See https://en.cppreference.com/w/cpp/header/optional
    pub fn optional() -> Self {
        Self::SystemHeader("optional".into())
    }

    /// Creates a `CcInclude` that represents `#include <type_traits>` and
    /// provides C++ APIs like `std::is_trivially_copy_constructible_v`.
    /// See https://en.cppreference.com/w/cpp/header/type_traits
    pub fn type_traits() -> Self {
        Self::SystemHeader("type_traits".into())
    }

    /// Creates a user include: `#include "some/path/to/header.h"`.
    pub fn user_header(path: Rc<str>) -> Self {
        Self::UserHeader(path)
    }

    /// Creates a `CcInclude` and detects whether it's a system header or a user
    /// header based on the path.
    ///
    /// System headers are included by angular brackets, e.g., `#include <cstddef>`.
    /// User headers are included by quotes, e.g., `#include "some/path/to/header.h"`.
    pub fn from_path(path: &str) -> Self {
        match (path.starts_with("<"), path.ends_with(">")) {
            (true, true) => Self::SystemHeader(Rc::from(&path[1..path.len() - 1])),
            _ => Self::UserHeader(Rc::from(path)),
        }
    }

    /// Creates a support library header include based on the specified format.
    /// E.g., `\"{header}\"` and `hdr.h` produces `#include "hdr.h"`.
    pub fn support_lib_header(format: Format<1>, path: Rc<str>) -> Self {
        Self::SupportLibHeader(format, path)
    }
}

impl ToTokens for CcInclude {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::SystemHeader(path) => {
                let path: TokenStream = path
                    .parse()
                    .expect("`pub` API of `CcInclude` guarantees validity of system includes");
                quote! { __HASH_TOKEN__ include < #path > __NEWLINE__ }.to_tokens(tokens)
            }
            Self::UserHeader(path) => {
                quote! { __HASH_TOKEN__ include #path __NEWLINE__ }.to_tokens(tokens)
            }
            Self::SupportLibHeader(format, path) => {
                let full_path: TokenStream = format
                    .format(&[&*path])
                    .parse()
                    .expect("Failed to parse support lib `#include` path");
                quote! { __HASH_TOKEN__ include #full_path __NEWLINE__ }.to_tokens(tokens)
            }
        }
    }
}

/// Formats a set of `CcInclude`s, trying to follow the guidance from
/// [the Google C++ Style Guide](https://google.github.io/styleguide/cppguide.html#Names_and_Order_of_Includes).
pub fn format_cc_includes(set_of_includes: &BTreeSet<CcInclude>) -> TokenStream {
    let mut tokens = TokenStream::default();
    let mut iter = set_of_includes.iter().peekable();
    while let Some(include) = iter.next() {
        include.to_tokens(&mut tokens);

        // Add an empty line between system headers and user headers.
        if let (CcInclude::SystemHeader(_), Some(CcInclude::UserHeader(_))) = (include, iter.peek())
        {
            quote! { __NEWLINE__ }.to_tokens(&mut tokens)
        }
    }
    tokens
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use googletest::prelude::*;
    use itertools::Itertools;
    use quote::{quote, ToTokens};
    use token_stream_matchers::{assert_cc_matches, assert_rs_matches};
    use token_stream_printer::cc_tokens_to_formatted_string_for_tests;

    #[gtest]
    fn test_format_cc_ident_basic() {
        assert_cc_matches!(format_cc_ident("foo").unwrap().to_token_stream(), quote! { foo });
    }

    #[gtest]
    fn test_format_cc_ident_exotic_xid_start() {
        assert_cc_matches!(format_cc_ident("Łukasz").unwrap().to_token_stream(), quote! { Łukasz });
    }

    #[gtest]
    fn test_format_cc_ident_underscore() {
        assert_cc_matches!(format_cc_ident("_").unwrap().to_token_stream(), quote! { _ });
    }

    #[gtest]
    fn test_format_cc_ident_reserved_rust_keyword() {
        assert_cc_matches!(format_cc_ident("impl").unwrap().to_token_stream(), quote! { impl });
    }

    #[gtest]
    fn test_format_cc_ident_reserved_cc_keyword() {
        let err = format_cc_ident("reinterpret_cast").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("`reinterpret_cast`"));
        assert!(msg.contains("C++ reserved keyword"));
    }

    #[gtest]
    fn test_format_cc_ident_unqualified_identifiers() {
        // https://en.cppreference.com/w/cpp/language/identifiers#Unqualified_identifiers

        // These may appear in `IR::Func::name`.
        assert_cc_matches!(format_cc_type_name("operator==").unwrap(), quote! { operator== });
        assert_cc_matches!(format_cc_type_name("operator new").unwrap(), quote! { operator new });

        // This may appear in `IR::Record::cc_name` (although in practice these will
        // be namespace-qualified most of the time).
        assert_cc_matches!(
            format_cc_type_name("MyTemplate<int>").unwrap(),
            quote! { MyTemplate<int> }
        );
    }

    /// https://en.cppreference.com/w/cpp/language/identifiers#Qualified_identifiers
    ///
    /// This may appear in `IR::Record::cc_name`, or in
    /// `crubit_annotate::cpp_layout_equivalent(cpp_type=...)`.
    #[gtest]
    fn test_format_cc_ident_qualified_identifiers() {
        assert_cc_matches!(
            format_cc_type_name("std::vector<int>").unwrap(),
            quote! { std::vector<int> }
        );
        assert_cc_matches!(
            format_cc_type_name("::std::vector<int>").unwrap(),
            quote! { ::std::vector<int> }
        );
    }

    #[gtest]
    fn test_format_cc_ident_empty() {
        let err = format_cc_ident("").unwrap_err();
        let msg = err.to_string();
        assert_eq!(msg, "Empty string is not a valid C++ identifier");
    }

    #[gtest]
    fn test_format_cc_ident_invalid_first_char() {
        let tests = vec![
            // `0` and `1 are field names in `struct RustStruct(i32, u16)`.
            "0",
            // `~MyClass` is a valid unqualified identifier in C++, but it is okay if
            // `format_cc_ident` rejects it, because `format_cc_ident` is not used to format
            // destructor names.
            "~MyClass",
            // We used to trim leading and/or trailing whitespace, but stricter validation
            // of leading whitespace seems desirable.
            r#" operator "" _km "#,
            // Other tests
            "(foo",
            "(foo)",
        ];
        for test in tests.into_iter() {
            let err = format_cc_ident(test).unwrap_err();
            let actual_msg = err.to_string();
            let expected_msg = format!("Can't format `{test}` as a C++ identifier: ");
            expect_that!(actual_msg, starts_with(expected_msg));
        }
    }

    #[gtest]
    fn test_make_rs_ident_basic() {
        let id = make_rs_ident("foo");
        assert_rs_matches!(quote! { #id }, quote! { foo });
    }

    #[gtest]
    fn test_make_rs_ident_reserved_cc_keyword() {
        let id = make_rs_ident("reinterpret_cast");
        assert_rs_matches!(quote! { #id }, quote! { reinterpret_cast });
    }

    #[gtest]
    fn test_make_rs_ident_reserved_rust_keyword() {
        let id = make_rs_ident("impl");
        assert_rs_matches!(quote! { #id }, quote! { r#impl });
    }

    #[gtest]
    #[should_panic]
    fn test_make_rs_ident_unfinished_group() {
        make_rs_ident("(foo"); // No closing `)`.
    }

    #[gtest]
    #[should_panic]
    fn test_make_rs_ident_empty() {
        make_rs_ident("");
    }

    #[gtest]
    fn test_cc_include_to_tokens_for_system_header() {
        let include = CcInclude::cstddef();
        assert_cc_matches!(
            quote! { #include },
            quote! {
                __HASH_TOKEN__ include <cstddef>
            }
        );
    }

    #[gtest]
    fn test_cc_include_to_tokens_for_user_header() {
        let include = CcInclude::user_header("some/path/to/header.h".into());
        assert_cc_matches!(
            quote! { #include },
            quote! {
                __HASH_TOKEN__ include "some/path/to/header.h"
            }
        );
    }

    #[gtest]
    fn test_cc_include_ord() {
        let cstddef = CcInclude::cstddef();
        let memory = CcInclude::memory();
        let a = CcInclude::user_header("a.h".into());
        let b = CcInclude::user_header("b.h".into());
        assert!(cstddef < memory);
        assert!(cstddef < a);
        assert!(cstddef < b);
        assert!(memory < a);
        assert!(memory < b);
        assert!(a < b);
    }

    #[gtest]
    fn test_format_cc_includes() {
        let includes = [
            CcInclude::cstddef(),
            CcInclude::memory(),
            CcInclude::user_header("a.h".into()),
            CcInclude::user_header("b.h".into()),
        ]
        .into_iter()
        .collect::<BTreeSet<_>>();

        let tokens = format_cc_includes(&includes);
        let actual =
            cc_tokens_to_formatted_string_for_tests(quote! { __NEWLINE__ #tokens }).unwrap();
        assert_eq!(
            actual,
            r#"
#include <cstddef>
#include <memory>

#include "a.h"
#include "b.h"
"#
        );
    }

    #[gtest]
    fn test_namespace_qualifier_empty() {
        let ns = NamespaceQualifier::new::<&str>([]);
        let actual_rs = ns.format_for_rs();
        assert!(actual_rs.is_empty());
        let actual_cc = ns.format_for_cc().unwrap();
        assert!(actual_cc.is_empty());
    }

    #[gtest]
    fn test_namespace_qualifier_basic() {
        let ns = NamespaceQualifier::new(["foo", "bar"]);
        let actual_rs = ns.format_for_rs();
        assert_rs_matches!(actual_rs, quote! { foo::bar:: });
        let actual_cc = ns.format_for_cc().unwrap();
        assert_cc_matches!(actual_cc, quote! { foo::bar:: });
    }

    #[gtest]
    fn test_namespace_qualifier_reserved_cc_keyword() {
        let ns = NamespaceQualifier::new(["foo", "impl", "bar"]);
        let actual_rs = ns.format_for_rs();
        assert_rs_matches!(actual_rs, quote! { foo :: r#impl :: bar :: });
        let actual_cc = ns.format_for_cc().unwrap();
        assert_cc_matches!(actual_cc, quote! { foo::impl::bar:: });
    }

    #[gtest]
    fn test_namespace_qualifier_reserved_rust_keyword() {
        let ns = NamespaceQualifier::new(["foo", "reinterpret_cast", "bar"]);
        let actual_rs = ns.format_for_rs();
        assert_rs_matches!(actual_rs, quote! { foo :: reinterpret_cast :: bar :: });
        let cc_error = ns.format_for_cc().unwrap_err();
        let msg = cc_error.to_string();
        assert!(msg.contains("`reinterpret_cast`"));
        assert!(msg.contains("C++ reserved keyword"));
    }

    #[gtest]
    fn test_format_cc_include_support_lib_header() {
        let tests = vec![
            (
                "\"crubit/support/path/for/test/{header}\"",
                "header.h",
                "\"crubit/support/path/for/test/header.h\"",
            ),
            (
                "\"crubit/support/path/for/test/{header}\"",
                "subdir/header.h",
                "\"crubit/support/path/for/test/subdir/header.h\"",
            ),
            (
                "<crubit/support/path/for/test/{header}>",
                "header.h",
                "<crubit/support/path/for/test/header.h>",
            ),
            ("\"{header}\"", "header.h", "\"header.h\""),
        ];

        for (support_path_format, header, expected_output) in tests.iter() {
            let support_path_format =
                Format::parse_with_metavars(support_path_format, &["header"]).unwrap();
            let header =
                CcInclude::support_lib_header(support_path_format, header.to_string().into());
            let mut actual_tokens = TokenStream::default();
            header.to_tokens(&mut actual_tokens);

            let expected_output: TokenStream =
                expected_output.parse().expect("Failed to convert expected_output to TokenStream");

            assert_cc_matches!(
                actual_tokens,
                quote! {
                  __HASH_TOKEN__ include #expected_output
                }
            );
        }
    }

    #[gtest]
    fn test_escape_non_identifier_chars() {
        let tests = vec![
            ("", ""),
            ("foo", "foo"),
            ("0abc", "_x00000030abc"),
            ("abc$xyz", "abc_dxyz"),
            ("abc.xyz", "abc_pxyz"),
            ("abc_xyz", "abc_uxyz"),
            ("abc🦀xyz", "abc_x0001f980xyz"),
            // With an escaping scheme like `$` => "_d", `<utf8 dd80 char>` => "_dd80", the
            // following 2 tests would fail the injectivity requirement (they both would map to
            // "_dd80"):
            ("$d80", "_dd80"),
            ("\u{740}", "_x00000740"),
        ];

        for (input, expected_output) in tests.iter() {
            let actual_output = escape_non_identifier_chars(input);
            assert_eq!(&actual_output, expected_output);
        }

        // Asserting that each distinct, unique test input should result in a unique,
        // non-duplicated output.  (This can be seen as a rather lightweight and
        // indirect verification of the injectivity requirement.)
        let duplicate_expectations =
            tests.iter().map(|(_, expected)| *expected).duplicates().collect_vec();
        let empty_vec: Vec<&'static str> = vec![];
        assert_eq!(empty_vec, duplicate_expectations);
    }

    #[gtest]
    fn test_is_cpp_pointer_type() {
        let tests = vec![
            ("Foo", None),
            ("Foo*", Some(CcConstQualifier::Mut)),
            ("Foo const*", Some(CcConstQualifier::Const)),
            ("const Foo*", Some(CcConstQualifier::Const)),
            ("::foo::bar::Fizz * ", Some(CcConstQualifier::Mut)),
            ("::foo::bar::Fizz const *", Some(CcConstQualifier::Const)),
            ("const ::foo::bar::Fizz *", Some(CcConstQualifier::Const)),
        ];

        for (input, expected_output) in tests.into_iter() {
            let actual_output = is_cpp_pointer_type(input.parse().unwrap());
            assert_eq!(actual_output, expected_output);
        }
    }
}

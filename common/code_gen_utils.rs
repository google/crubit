// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use anyhow::{anyhow, ensure, Result};
use itertools::Itertools;
use once_cell::sync::Lazy;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

/// Formats a C++ identifier. Returns an error when `ident` is a C++ reserved
/// keyword or is an invalid identifier.
pub fn format_cc_ident(ident: &str) -> Result<TokenStream> {
    ensure!(!ident.is_empty(), "Empty string is not a valid C++ identifier");

    // C++ doesn't have an equivalent of
    // https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html and therefore
    // an error is returned when `ident` is a C++ reserved keyword.
    ensure!(
        !RESERVED_CC_KEYWORDS.contains(ident),
        "`{}` is a C++ reserved keyword and can't be used as a C++ identifier",
        ident
    );

    // https://en.cppreference.com/w/cpp/language/identifiers says that "A valid identifier must
    // begin with a non-digit character (Latin letter, underscore, or Unicode
    // character of class XID_Start)".  One motivation for this check is to
    // explicitly catch names of tuple fields (e.g. `some_tuple.0`).
    let first_char = ident.chars().next().expect("!is_empty checked above");
    ensure!(
        unicode_ident::is_xid_start(first_char) || first_char == '_',
        "The following character can't be used as a start of a C++ identifier: {first_char}",
    );

    ident.parse().map_err(
        // Explicitly mapping the error via `anyhow!`, because `LexError` is not `Sync`
        // (required for `anyhow::Error` to implement `From<LexError>`) and
        // therefore we can't just use `?`.
        |lex_error| anyhow!("Can't format `{ident}` as a C++ identifier: {lex_error}"),
    )
}

/// Makes an 'Ident' to be used in the Rust source code. Escapes Rust keywords.
/// Panics if `ident` is empty or is otherwise an invalid identifier.
pub fn make_rs_ident(ident: &str) -> Ident {
    // TODO(https://github.com/dtolnay/syn/pull/1098): Remove the hardcoded list once syn recognizes
    // 2018 and 2021 keywords.
    if ["async", "await", "try", "dyn"].contains(&ident) {
        return format_ident!("r#{}", ident);
    }
    match syn::parse_str::<syn::Ident>(ident) {
        Ok(_) => format_ident!("{}", ident),
        Err(_) => format_ident!("r#{}", ident),
    }
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
                    result.push_str("_x");
                    result.push_str(&format!("{:08x}", c as u32));
                };
            }
        }
    }

    result
}

/// Representation of `foo::bar::baz` where each component is either the name
/// of a C++ namespace, or the name of a Rust module.
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct NamespaceQualifier(Vec<Rc<str>>);

impl NamespaceQualifier {
    /// Constructs a new `NamespaceQualifier` from a sequence of names.
    pub fn new<T: Into<Rc<str>>>(iter: impl IntoIterator<Item = T>) -> Self {
        // TODO(b/258265044): Catch most (all if possible) error conditions early.  For
        // example:
        // - Panic early if any strings are empty, or are not Rust identifiers
        // - Report an error early if any strings are C++ reserved keywords
        // This may make `format_for_cc`, `format_with_cc_body`, and
        // `format_namespace_bound_cc_tokens` infallible.
        Self(iter.into_iter().map(Into::into).collect())
    }

    /// Returns `foo::bar::baz::` (escaping Rust keywords as needed).
    pub fn format_for_rs(&self) -> TokenStream {
        let namespace_rs_idents = self.0.iter().map(|ns| make_rs_ident(ns));
        quote! { #(#namespace_rs_idents::)* }
    }

    /// Returns `foo::bar::baz::` (reporting errors for C++ keywords).
    pub fn format_for_cc(&self) -> Result<TokenStream> {
        let namespace_cc_idents = self.cc_idents()?;
        Ok(quote! { #(#namespace_cc_idents::)* })
    }

    fn format_with_cc_body(&self, body: TokenStream) -> Result<TokenStream> {
        if self.0.is_empty() {
            Ok(body)
        } else {
            let namespace_cc_idents = self.cc_idents()?;
            Ok(quote! {
                __NEWLINE__ namespace #(#namespace_cc_idents)::* { __NEWLINE__
                    #body
                __NEWLINE__ }  __NEWLINE__
            })
        }
    }

    fn cc_idents(&self) -> Result<Vec<TokenStream>> {
        self.0.iter().map(|ns| format_cc_ident(ns)).collect()
    }
}

/// `format_namespace_bound_cc_tokens` formats a sequence of namespace-bound
/// snippets.  For example, `[(ns, tokens)]` will be formatted as:
///
///     ```
///     namespace ns {
///     #tokens
///     }
///     ```
///
/// `format_namespace_bound_cc_tokens` tries to give a nice-looking output - for
/// example it combines consecutive items that belong to the same namespace,
/// when given `[(ns, tokens1), (ns, tokens2)]` as input:
///
///     ```
///     namespace ns {
///     #tokens1
///     #tokens2
///     }
///     ```
///
/// `format_namespace_bound_cc_tokens` also knows that top-level items (e.g.
/// ones where `NamespaceQualifier` doesn't contain any namespace names) should
/// be emitted at the top-level (not nesting them under a `namespace` keyword).
/// For example, `[(toplevel_ns, tokens)]` will be formatted as just:
///
///     ```
///     #tokens
///     ```
pub fn format_namespace_bound_cc_tokens(
    iter: impl IntoIterator<Item = (NamespaceQualifier, TokenStream)>,
) -> TokenStream {
    let iter = iter
        .into_iter()
        .coalesce(|(ns1, mut tokens1), (ns2, tokens2)| {
            // Coallesce tokens if subsequent items belong to the same namespace.
            if ns1 == ns2 {
                tokens1.extend(tokens2);
                Ok((ns1, tokens1))
            } else {
                Err(((ns1, tokens1), (ns2, tokens2)))
            }
        })
        .map(|(ns, tokens)| {
            ns.format_with_cc_body(tokens).unwrap_or_else(|err| {
                let name = ns.0.iter().join("::");
                let err = format!("Failed to format namespace name `{name}`: {err}");
                quote! { __COMMENT__ #err }
            })
        });

    // Using fully-qualified syntax to avoid the warning that `intersperse`
    // may be added to the standard library in the future.
    //
    // TODO(https://github.com/rust-lang/rust/issues/79524): Use `.intersperse(...)` syntax once
    // 1) this stdlib feature gets stabilized and
    // 2) the method with conflicting name gets removed from `itertools`.
    let iter = itertools::Itertools::intersperse(iter, quote! { __NEWLINE__ __NEWLINE__ });

    iter.collect()
}

/// `CcInclude` represents a single `#include ...` directive in C++.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CcInclude {
    /// Represents a system header, e.g., `cstdef`, which will be included by
    /// angular brackets.
    SystemHeader(&'static str),
    /// Represents a user header, which will be included by quotes.
    UserHeader(Rc<str>),
    /// Represents an `#include` for Crubit C++ support library headers: the
    /// format specifier for what comes after `#include` and path of the support
    /// library header.
    SupportLibHeader(Rc<str>, Rc<str>),
}

impl CcInclude {
    /// Creates a `CcInclude` that represents `#include <cstddef>` and provides
    /// C++ types like `std::size_t` or `std::ptrdiff_t`.  See
    /// https://en.cppreference.com/w/cpp/header/cstddef
    pub fn cstddef() -> Self {
        Self::SystemHeader("cstddef")
    }

    /// Creates a `CcInclude` that represents `#include <cstdint>` and provides
    /// C++ types like `std::int16_t` or `std::uint32_t`.  See
    /// https://en.cppreference.com/w/cpp/header/cstdint
    pub fn cstdint() -> Self {
        Self::SystemHeader("cstdint")
    }

    /// Creates a `CcInclude` that represents `#include <memory>`.
    /// See https://en.cppreference.com/w/cpp/header/memory
    pub fn memory() -> Self {
        Self::SystemHeader("memory")
    }

    /// Creates a `CcInclude` that represents `#include <utility>` and provides
    /// C++ functions like `std::move` and C++ types like `std::tuple`.
    /// See https://en.cppreference.com/w/cpp/header/utility
    pub fn utility() -> Self {
        Self::SystemHeader("utility")
    }

    /// Creates a `CcInclude` that represents `#include <type_traits>` and
    /// provides C++ APIs like `std::is_trivially_copy_constructible_v`.
    /// See https://en.cppreference.com/w/cpp/header/type_traits
    pub fn type_traits() -> Self {
        Self::SystemHeader("type_traits")
    }

    /// Creates a user include: `#include "some/path/to/header.h"`.
    pub fn user_header(path: Rc<str>) -> Self {
        Self::UserHeader(path)
    }

    /// Creates a support library header include based on the specified format.
    /// E.g., `\"{header}\"` and `hdr.h` produces `#include "hdr.h"`.
    pub fn support_lib_header(format: Rc<str>, path: Rc<str>) -> Self {
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
                    .replace("{header}", path)
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

static RESERVED_CC_KEYWORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    // `RESERVED_CC_KEYWORDS` are based on https://en.cppreference.com/w/cpp/keyword
    [
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
    ]
    .into_iter()
    .collect()
});

#[cfg(test)]
pub mod tests {
    use super::*;
    use quote::quote;
    use token_stream_matchers::{assert_cc_matches, assert_rs_matches};
    use token_stream_printer::cc_tokens_to_formatted_string_for_tests;

    #[test]
    fn test_format_cc_ident_basic() {
        assert_cc_matches!(format_cc_ident("foo").unwrap(), quote! { foo });
    }

    #[test]
    fn test_format_cc_ident_exotic_xid_start() {
        assert_cc_matches!(format_cc_ident("Łukasz").unwrap(), quote! { Łukasz });
    }

    #[test]
    fn test_format_cc_ident_underscore() {
        assert_cc_matches!(format_cc_ident("_").unwrap(), quote! { _ });
    }

    #[test]
    fn test_format_cc_ident_reserved_rust_keyword() {
        assert_cc_matches!(format_cc_ident("impl").unwrap(), quote! { impl });
    }

    #[test]
    fn test_format_cc_ident_reserved_cc_keyword() {
        let err = format_cc_ident("reinterpret_cast").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("`reinterpret_cast`"));
        assert!(msg.contains("C++ reserved keyword"));
    }

    #[test]
    fn test_format_cc_ident_unparseable_identifier() {
        let err = format_cc_ident("foo)").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("Can't format `foo)` as a C++ identifier"));
        assert!(msg.contains("cannot parse"));
    }

    #[test]
    fn test_format_cc_ident_unqualified_identifiers() {
        // https://en.cppreference.com/w/cpp/language/identifiers#Unqualified_identifiers

        // These may appear in `IR::Func::name`.
        assert_cc_matches!(format_cc_ident("operator==").unwrap(), quote! { operator== });
        assert_cc_matches!(format_cc_ident("operator new").unwrap(), quote! { operator new });

        // This may appear in `IR::Record::cc_name` (although in practice these will
        // be namespace-qualified most of the time).
        assert_cc_matches!(format_cc_ident("MyTemplate<int>").unwrap(), quote! { MyTemplate<int> });
    }

    #[test]
    fn test_format_cc_ident_qualified_identifiers() {
        // https://en.cppreference.com/w/cpp/language/identifiers#Qualified_identifiers

        // This may appear in `IR::Record::cc_name`.
        assert_cc_matches!(
            format_cc_ident("std::vector<int>").unwrap(),
            quote! { std::vector<int> }
        );
    }

    #[test]
    fn test_format_cc_ident_empty() {
        let err = format_cc_ident("").unwrap_err();
        let msg = err.to_string();
        assert_eq!(msg, "Empty string is not a valid C++ identifier");
    }

    #[test]
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
            " foo",
            // Other tests
            "(foo",
            "(foo)",
        ];
        for test in tests.into_iter() {
            let err = format_cc_ident(test).unwrap_err();
            let actual_msg = err.to_string();
            let c = test.chars().next().unwrap();
            let expected_msg = format!(
                "The following character can't be used as a start of a C++ identifier: {c}"
            );
            assert_eq!(actual_msg, expected_msg);
        }
    }

    #[test]
    fn test_make_rs_ident_basic() {
        let id = make_rs_ident("foo");
        assert_rs_matches!(quote! { #id }, quote! { foo });
    }

    #[test]
    fn test_make_rs_ident_reserved_cc_keyword() {
        let id = make_rs_ident("reinterpret_cast");
        assert_rs_matches!(quote! { #id }, quote! { reinterpret_cast });
    }

    #[test]
    fn test_make_rs_ident_reserved_rust_keyword() {
        let id = make_rs_ident("impl");
        assert_rs_matches!(quote! { #id }, quote! { r#impl });
    }

    #[test]
    #[should_panic]
    fn test_make_rs_ident_unfinished_group() {
        make_rs_ident("(foo"); // No closing `)`.
    }

    #[test]
    #[should_panic]
    fn test_make_rs_ident_empty() {
        make_rs_ident("");
    }

    #[test]
    fn test_cc_include_to_tokens_for_system_header() {
        let include = CcInclude::cstddef();
        assert_cc_matches!(
            quote! { #include },
            quote! {
                __HASH_TOKEN__ include <cstddef>
            }
        );
    }

    #[test]
    fn test_cc_include_to_tokens_for_user_header() {
        let include = CcInclude::user_header("some/path/to/header.h".into());
        assert_cc_matches!(
            quote! { #include },
            quote! {
                __HASH_TOKEN__ include "some/path/to/header.h"
            }
        );
    }

    #[test]
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

    #[test]
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

    #[test]
    fn test_namespace_qualifier_empty() {
        let ns = NamespaceQualifier::new::<&str>([]);
        let actual_rs = ns.format_for_rs();
        assert!(actual_rs.is_empty());
        let actual_cc = ns.format_for_cc().unwrap();
        assert!(actual_cc.is_empty());
    }

    #[test]
    fn test_namespace_qualifier_basic() {
        let ns = NamespaceQualifier::new(["foo", "bar"]);
        let actual_rs = ns.format_for_rs();
        assert_rs_matches!(actual_rs, quote! { foo::bar:: });
        let actual_cc = ns.format_for_cc().unwrap();
        assert_cc_matches!(actual_cc, quote! { foo::bar:: });
    }

    #[test]
    fn test_namespace_qualifier_reserved_cc_keyword() {
        let ns = NamespaceQualifier::new(["foo", "impl", "bar"]);
        let actual_rs = ns.format_for_rs();
        assert_rs_matches!(actual_rs, quote! { foo :: r#impl :: bar :: });
        let actual_cc = ns.format_for_cc().unwrap();
        assert_cc_matches!(actual_cc, quote! { foo::impl::bar:: });
    }

    #[test]
    fn test_namespace_qualifier_reserved_rust_keyword() {
        let ns = NamespaceQualifier::new(["foo", "reinterpret_cast", "bar"]);
        let actual_rs = ns.format_for_rs();
        assert_rs_matches!(actual_rs, quote! { foo :: reinterpret_cast :: bar :: });
        let cc_error = ns.format_for_cc().unwrap_err();
        let msg = cc_error.to_string();
        assert!(msg.contains("`reinterpret_cast`"));
        assert!(msg.contains("C++ reserved keyword"));
    }

    #[test]
    fn test_namespace_qualifier_format_with_cc_body_top_level_namespace() {
        let ns = NamespaceQualifier::new::<&str>([]);
        assert_cc_matches!(
            ns.format_with_cc_body(quote! { cc body goes here }).unwrap(),
            quote! { cc body goes here },
        );
    }

    #[test]
    fn test_namespace_qualifier_format_with_cc_body_nested_namespace() {
        let ns = NamespaceQualifier::new(["foo", "bar", "baz"]);
        assert_cc_matches!(
            ns.format_with_cc_body(quote! { cc body goes here }).unwrap(),
            quote! {
                namespace foo::bar::baz {
                    cc body goes here
                }  // namespace foo::bar::baz
            },
        );
    }

    #[test]
    fn test_format_namespace_bound_cc_tokens() {
        let top_level = NamespaceQualifier::new::<&str>([]);
        let m1 = NamespaceQualifier::new(["m1"]);
        let m2 = NamespaceQualifier::new(["m2"]);
        let input = [
            (top_level.clone(), quote! { void f0a(); }),
            (m1.clone(), quote! { void f1a(); }),
            (m1.clone(), quote! { void f1b(); }),
            (top_level.clone(), quote! { void f0b(); }),
            (top_level.clone(), quote! { void f0c(); }),
            (m2.clone(), quote! { void f2a(); }),
            (m1.clone(), quote! { void f1c(); }),
            (m1.clone(), quote! { void f1d(); }),
        ];
        assert_cc_matches!(
            format_namespace_bound_cc_tokens(input),
            quote! {
                void f0a();

                namespace m1 {
                void f1a();
                void f1b();
                }  // namespace m1

                void f0b();
                void f0c();

                namespace m2 {
                void f2a();
                }

                namespace m1 {
                void f1c();
                void f1d();
                }  // namespace m1
            },
        );
    }

    #[test]
    fn test_format_namespace_bound_cc_tokens_with_reserved_cpp_keywords() {
        let working_module = NamespaceQualifier::new(["foo", "working_module", "bar"]);
        let broken_module = NamespaceQualifier::new(["foo", "reinterpret_cast", "bar"]);
        let input = vec![
            (broken_module.clone(), quote! { void broken_module_f1(); }),
            (broken_module.clone(), quote! { void broken_module_f2(); }),
            (working_module.clone(), quote! { void working_module_f3(); }),
            (working_module.clone(), quote! { void working_module_f4(); }),
            (broken_module.clone(), quote! { void broken_module_f5(); }),
            (broken_module.clone(), quote! { void broken_module_f6(); }),
            (working_module.clone(), quote! { void working_module_f7(); }),
            (working_module.clone(), quote! { void working_module_f8(); }),
        ];
        let broken_module_msg = "Failed to format namespace name `foo::reinterpret_cast::bar`: \
                                 `reinterpret_cast` is a C++ reserved keyword \
                                 and can't be used as a C++ identifier";
        assert_cc_matches!(
            format_namespace_bound_cc_tokens(input),
            quote! {
                __COMMENT__ #broken_module_msg

                namespace foo::working_module::bar {
                void working_module_f3();
                void working_module_f4();
                }  // namespace foo::working_module::bar

                // TODO(lukasza): Repeating the error message below seems somewhat undesirable.
                // OTOH fixing this seems low priority, given that errors when formatting namespace
                // names should be fairly rare.  And fixing this requires extra work and effort,
                // especially if we want to:
                // 1) coallesce the 2 chunks of the `working_module`
                // 2) avoid reordering where the `broken_module` error comment appears.
                __COMMENT__ #broken_module_msg

                namespace foo::working_module::bar {
                void working_module_f7();
                void working_module_f8();
                }  // namespace foo::working_module::bar
            },
        );
    }

    #[test]
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
            let header = CcInclude::support_lib_header(
                support_path_format.to_string().into(),
                header.to_string().into(),
            );
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

    #[test]
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
}

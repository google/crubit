// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![deny(missing_docs)]

//! Crubit annotations for Rust APIs.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::collections::{hash_map::Entry, HashMap};
use syn::parse::{Parse, ParseStream};
use syn::token;
use syn::{parse_macro_input, Ident, LitStr};

enum KeyValueValue {
    Single(LitStr),
    List(Vec<LitStr>),
}

impl KeyValueValue {
    fn single(&self) -> Option<&LitStr> {
        match self {
            KeyValueValue::Single(lit) => Some(lit),
            _ => None,
        }
    }
}

/// A single `ident="string literal"` or `ident=["string literal", ...]` pair.
struct KeyValue {
    key: Ident,
    value: KeyValueValue,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let key = Ident::parse(input)?;
        token::Eq::parse(input)?;
        let value = if input.peek(syn::token::Bracket) {
            let content;
            syn::bracketed!(content in input);
            let mut vec = Vec::new();
            while !content.is_empty() {
                vec.push(<LitStr as Parse>::parse(&content)?);
                if !content.is_empty() {
                    token::Comma::parse(&content)?;
                }
            }
            KeyValueValue::List(vec)
        } else {
            KeyValueValue::Single(<LitStr as Parse>::parse(input)?)
        };
        Ok(KeyValue { key, value })
    }
}

/// A comma-separated list of `ident="string literal"` or `ident=["..."]` pairs.
struct KeyValueList(Vec<KeyValue>);

impl Parse for KeyValueList {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut entries = Vec::new();
        while !input.is_empty() {
            entries.push(KeyValue::parse(input)?);
            if !input.is_empty() {
                token::Comma::parse(input)?;
            }
        }
        Ok(Self(entries))
    }
}

fn combine(a: &mut syn::Result<()>, b: syn::Error) {
    if let Err(a) = a {
        a.combine(b);
    } else {
        *a = Err(b);
    }
}

impl KeyValueList {
    /// Returns an error if any non-repeatable key is duplicated, if an expected key is not present,
    /// or if extra keys are present.
    fn check_keys_allow_repeatable(
        &self,
        expected_keys: &[&str],
        repeatable_keys: &[&str],
    ) -> syn::Result<()> {
        let mut key_seen: HashMap<String, bool> =
            expected_keys.iter().map(|key| (key.to_string(), false)).collect();
        let mut maybe_error = syn::Result::Ok(());
        for entry in &self.0 {
            let key_string = entry.key.to_string();
            match key_seen.entry(key_string) {
                Entry::Vacant(key_string_entry) => {
                    combine(
                        &mut maybe_error,
                        syn::Error::new(
                            entry.key.span(),
                            format!(
                                "Unexpected key `{}` provided to `crubit_annotate`",
                                key_string_entry.key()
                            ),
                        ),
                    );
                }
                Entry::Occupied(mut already_seen) => {
                    if *already_seen.get()
                        && !repeatable_keys.contains(&already_seen.key().as_str())
                    {
                        combine(
                            &mut maybe_error,
                            syn::Error::new(
                                entry.key.span(),
                                format!(
                                    "Duplicate key `{}` provided to `crubit_annotate`",
                                    entry.key
                                ),
                            ),
                        );
                    }
                    *already_seen.get_mut() = true;
                }
            }
        }
        for (key, seen) in key_seen {
            if !seen {
                combine(
                    &mut maybe_error,
                    syn::Error::new(
                        Span::call_site(),
                        format!("Expected key `{}` not provided to `crubit_annotate`, expected one of {:?}", key, expected_keys),
                    ),
                );
            }
        }
        maybe_error
    }

    fn check_keys(&self, expected_keys: &[&str]) -> syn::Result<()> {
        self.check_keys_allow_repeatable(expected_keys, &[])
    }

    /// Transforms this `KeyValueList` into doc comments before the token stream so that
    /// it can be consumed by Crubit via the Rust HIR.
    ///
    /// Rust HIR only exposes user-defined attributes that are either doc comments or tool
    /// attributes, and tool attributes are unstable and require an additional crate level attribute
    /// to declare.
    ///
    /// The entries appear as `#[doc="CRUBIT_ANNOTATE: key=value"]`.
    fn to_doc_comments(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        for entry in &self.0 {
            let key_str = entry.key.to_string();
            match &entry.value {
                KeyValueValue::Single(val) => {
                    tokens.extend(key_value_to_doc_comment(&key_str, &val.value()));
                }
                KeyValueValue::List(vals) => {
                    for val in vals {
                        tokens.extend(key_value_to_doc_comment(&key_str, &val.value()));
                    }
                }
            }
        }
        tokens
    }
}

/// Creates a doc comment for a single `key=value` pair.
///
/// The values appear as `#[doc="CRUBIT_ANNOTATE: key=value"]`.
fn key_value_to_doc_comment(key: &str, value: &str) -> TokenStream {
    let value = format!("CRUBIT_ANNOTATE: {}={}", key, value);
    TokenStream::from(quote! { #[doc=#value] })
}

fn key_value_list_with_keys_to_doc_comment(
    attribute: TokenStream,
    expected_keys: &[&str],
) -> TokenStream {
    key_value_list_with_keys_and_repeatable_to_doc_comment(attribute, expected_keys, &[])
}

fn key_value_list_with_keys_and_repeatable_to_doc_comment(
    attribute: TokenStream,
    expected_keys: &[&str],
    repeatable_keys: &[&str],
) -> TokenStream {
    let attribute_args = parse_macro_input!(attribute as KeyValueList);
    if let Err(error) = attribute_args.check_keys_allow_repeatable(expected_keys, repeatable_keys) {
        return TokenStream::from(error.into_compile_error());
    }
    attribute_args.to_doc_comments()
}

/// Creates a TokenStream that prepends the computed prefix onto the given body.
///
/// This function encourages users to write all compiler-error-driven early-returns in the body of
/// `make_prefix_fn`. This ensures that the body is always emitted regardless of whether
/// the attribute itself successfully parses.
fn make_prefix_for(body: TokenStream, make_prefix_fn: impl FnOnce() -> TokenStream) -> TokenStream {
    let mut output = make_prefix_fn();
    output.extend([body]);
    output
}

/// Marks a Rust type as having equivalent layout to a particular pre-defined C++ type.
///
/// This annotation prevents Crubit from generating a C++ type for the Rust type,
/// instead binding it directly to the C++ type.
///
/// The annotation accepts two string arguments:
///
/// * `cpp_type`: The fully-qualified name of the C++ type to which the Rust type is equivalent.
/// * `include_path`: The path to the header file that defines the C++ type.
///
/// Example:
///
/// ```rs
/// #[crubit_annotate::cpp_layout_equivalent(
///     cpp_type="::std::string",
///     include_path="<string>",
/// )]
/// pub struct CppString {
///     ...
/// }
/// ```
#[proc_macro_attribute]
pub fn cpp_layout_equivalent(attribute: TokenStream, input: TokenStream) -> TokenStream {
    make_prefix_for(input, || {
        key_value_list_with_keys_and_repeatable_to_doc_comment(
            attribute,
            &["cpp_type", "include_path"],
            &["include_path"],
        )
    })
}

/// Marks a Rust type alias as a specialization of a generic type for C++.
///
/// This annotation accepts two string arguments:
/// * `cpp_type`: The fully-qualified name of the C++ type for this specialization.
/// * `include_path`: The path to the header file that defines the C++ type.
///
/// Example:
/// ```rs
/// #[crubit_annotate::cpp_specialization(
///     cpp_type="::std::string",
///     include_path="<string>",
/// )]
/// pub type StringAlias = MyGenericString<()>;
/// ```
#[proc_macro_attribute]
pub fn cpp_specialization(attribute: TokenStream, input: TokenStream) -> TokenStream {
    make_prefix_for(input, || {
        let attribute_args = parse_macro_input!(attribute as KeyValueList);
        if let Err(error) = attribute_args
            .check_keys_allow_repeatable(&["cpp_type", "include_path"], &["include_path"])
        {
            return TokenStream::from(error.into_compile_error());
        }
        let mut tokens = attribute_args.to_doc_comments();
        tokens.extend(key_value_to_doc_comment("specializes_cpp_type", ""));
        tokens
    })
}

/// Marks a Rust type as being by-value convertible to a particular pre-defined C++ type.
///
/// This annotation prevents Crubit from generating a C++ type for the Rust type,
/// instead binding it directly to the C++ type.
///
/// This annotation accepts four string arguments:
///
/// * `cpp_type`: The fully-qualified name of the C++ type to which the Rust type is convertible.
/// * `include_path`: The path to the header file that defines the C++ type.
/// * `cpp_to_rust_converter`: The name of the `extern "C"` C++ to Rust converter function.
/// * `rust_to_cpp_converter`: The name of the `extern "C"` Rust to C++ converter function.
///
/// Both function arguments must be `extern "C"` functions that accept two void pointers: the first
/// for the input and the second for the output.
///
/// Example:
///
/// ```rs
/// #[crubit_annotate::cpp_convertible(
///     cpp_type="::std::string",
///     include_path="<string>",
///     cpp_to_rust_converter="cpp_string_to_rust_string",
///     rust_to_cpp_converter="rust_string_to_cpp_string",
/// )]
/// pub struct CppString {
///     value: String,
/// }
///
/// #[unsafe(no_mangle)]
/// pub unsafe extern "C" fn rust_string_to_cpp_string(input: *const c_void, output: *mut c_void) {
///     todo!()
/// }
///
/// #[unsafe(no_mangle)]
/// pub unsafe extern "C" fn cpp_string_to_rust_string(input: *const c_void, output: *mut c_void) {
///     todo!()
/// }
/// ```
///
#[proc_macro_attribute]
pub fn cpp_convertible(attribute: TokenStream, input: TokenStream) -> TokenStream {
    make_prefix_for(input, || {
        key_value_list_with_keys_and_repeatable_to_doc_comment(
            attribute,
            &["cpp_type", "include_path", "cpp_to_rust_converter", "rust_to_cpp_converter"],
            &["include_path"],
        )
    })
}

/// Composable bridging.
#[proc_macro_attribute]
pub fn cpp_bridge(attribute: TokenStream, input: TokenStream) -> TokenStream {
    make_prefix_for(input, || {
        key_value_list_with_keys_to_doc_comment(
            attribute,
            &["cpp_type", "bridge_abi_cpp", "bridge_abi_rust"],
        )
    })
}

/// Marks a Rust item as having a different name when used from C++.
///
/// This allows for renaming Rust functions and types names that are not C++-compatible, such as
/// `new`.
///
/// For instance, the following annotation indicates that the Rust function
/// `new` should be renamed to `Create` in C++:
///
/// ```
/// #[crubit_annotate::cpp_name("Create")]
/// pub fn new() -> i32 { 4 }
/// ```
#[proc_macro_attribute]
pub fn cpp_name(attribute: TokenStream, input: TokenStream) -> TokenStream {
    make_prefix_for(input, || {
        let attribute_args = parse_macro_input!(attribute as LitStr);
        key_value_to_doc_comment("cpp_name", &attribute_args.value())
    })
}

/// Marks a Rust struct to be translated into a C++ enum or enum class.
///
/// This annotation accepts a single string `kind` argument, which must be either `enum` or `enum
/// class`.
///
/// Structs with this annotation must be repr-transparent structs with a single primitive field.
/// The structs also cannot have any methods, as the translated C++ enum cannot have methods.
///
/// Example:
///
/// ```rs
/// #[crubit_annotate::cpp_enum(kind="enum class")]
/// #[repr(transparent)]
/// pub struct MyEnum(i32);
///
/// impl MyEnum {
///     pub const VARIANT_0: MyEnum = MyEnum(0);
///     pub const VARIANT_1: MyEnum = MyEnum(1);
///     // ...
/// }
/// ```
///
/// This will generate (approximately) the following C++ code:
///
/// ```c++
/// enum class MyEnum : std::int32_t {
///     VARIANT_0 = 0,
///     VARIANT_1 = 1,
///     // ...
/// };
/// ```
#[proc_macro_attribute]
pub fn cpp_enum(attribute: TokenStream, input: TokenStream) -> TokenStream {
    make_prefix_for(input, || {
        let attribute_args = parse_macro_input!(attribute as KeyValueList);
        if let Err(error) = attribute_args.check_keys(&["kind"]) {
            return TokenStream::from(error.into_compile_error());
        }
        let [kind] = &attribute_args.0[..] else { unreachable!() };
        let Some(lit) = kind.value.single() else {
            return TokenStream::from(
                syn::Error::new(kind.key.span(), "Expected a single string literal for `kind`")
                    .into_compile_error(),
            );
        };
        let kind_str = lit.value();
        if kind_str != "enum" && kind_str != "enum class" {
            return TokenStream::from(
                syn::Error::new(
                    lit.span(),
                    format!(
                        "Invalid `kind` value `{}` for `cpp_enum` annotation. \
                        Expected \"enum\" or \"enum class\".",
                        kind_str
                    ),
                )
                .into_compile_error(),
            );
        }
        key_value_to_doc_comment("cpp_enum", &kind_str)
    })
}

/// Enforces that a Rust function or type receives C++ bindings.
///
/// Example:
///
/// ```rs
/// #[crubit_annotate::must_bind]
/// pub fn my_function() -> i32 {...}
/// ```
///
/// By default, Crubit will gracefully omit bindings for functions and types that cannot be bound to
/// C++. Applying `#[must_bind]` to the item ensures that Crubit will fail at bindings generation
/// time if the item cannot be bound to C++.
///
/// This can be useful for ensuring that particular functions or types are usable from C++.
#[proc_macro_attribute]
pub fn must_bind(attribute: TokenStream, input: TokenStream) -> TokenStream {
    make_prefix_for(input, || {
        if !attribute.is_empty() {
            return TokenStream::from(
                syn::Error::new(
                    attribute.into_iter().next().unwrap().span().into(),
                    "The `must_bind` annotation does not accept any arguments.",
                )
                .into_compile_error(),
            );
        }
        key_value_to_doc_comment("must_bind", "")
    })
}

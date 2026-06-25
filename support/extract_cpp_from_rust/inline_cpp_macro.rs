// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn inline_cpp(input: TokenStream) -> TokenStream {
    let span = proc_macro::Span::call_site();
    let file = span.file();
    let line = span.line();
    let col = span.column();

    let target = std::env::var("CRUBIT_TARGET").unwrap_or_default();
    let name_str = inline_cpp_utils::compute_thunk_name(&target, &file, line, col);
    let thunk_name = quote::format_ident!("{}", name_str);

    let input2 = proc_macro2::TokenStream::from(input);
    let output2 = match parse_and_expand(input2, thunk_name) {
        Ok(expanded) => expanded,
        Err(err_msg) => {
            let msg = err_msg.as_str();
            quote! { compile_error!(#msg); }
        }
    };
    TokenStream::from(output2)
}

#[derive(Clone)]
struct TokenCursor {
    tokens: std::rc::Rc<[proc_macro2::TokenTree]>,
    index: usize,
}

impl TokenCursor {
    fn new(tokens: proc_macro2::TokenStream) -> Self {
        TokenCursor { tokens: tokens.into_iter().collect(), index: 0 }
    }

    fn get(&self, offset: usize) -> Option<&proc_macro2::TokenTree> {
        self.tokens.get(self.index + offset)
    }

    fn next(&mut self) -> Option<proc_macro2::TokenTree> {
        let res = self.tokens.get(self.index)?.clone();
        self.index += 1;
        Some(res)
    }
}

struct Param {
    name: proc_macro2::Ident,
    rust_type: proc_macro2::TokenStream,
}

fn strip_cv_qualifiers(s: &str) -> (bool, String) {
    let mut words: Vec<&str> = s.split_whitespace().collect();
    let mut is_const = false;
    words.retain(|w| {
        if *w == "const" {
            is_const = true;
            false
        } else {
            *w != "volatile"
        }
    });
    (is_const, words.join(" "))
}

fn map_cpp_type_to_rust_str(cpp_type: &str) -> Result<String, String> {
    let s = cpp_type.trim().to_string();

    if s.ends_with('*') || s.ends_with('&') {
        let is_ptr = s.ends_with('*');
        let base = s[..s.len() - 1].trim();
        let (is_const, stripped) = strip_cv_qualifiers(base);
        if is_ptr && stripped == "void" {
            return Ok(if is_const {
                "*const std::ffi::c_void".to_string()
            } else {
                "*mut std::ffi::c_void".to_string()
            });
        }
        let rust_base = map_cpp_type_to_rust_str(&stripped)?;
        return Ok(match (is_ptr, is_const) {
            (true, true) => format!("*const {}", rust_base),
            (true, false) => format!("*mut {}", rust_base),
            (false, true) => format!("&{}", rust_base),
            (false, false) => format!("&mut {}", rust_base),
        });
    }

    let (_is_const, stripped) = strip_cv_qualifiers(&s);
    let mapped_base = match stripped.as_str() {
        "int" => "i32",
        "unsigned int" | "unsigned" => "u32",
        "long" => "i64",
        "unsigned long" => "u64",
        "long long" => "i64",
        "unsigned long long" => "u64",
        "short" => "i16",
        "unsigned short" => "u16",
        "char" => "std::os::raw::c_char",
        "unsigned char" => "u8",
        "signed char" => "i8",
        "float" => "f32",
        "double" => "f64",
        "bool" => "bool",
        "void" => "()",
        "rust::Str" => "&str",
        "uint8_t" | "std::uint8_t" => "u8",
        "int8_t" | "std::int8_t" => "i8",
        "uint16_t" | "std::uint16_t" => "u16",
        "int16_t" | "std::int16_t" => "i16",
        "uint32_t" | "std::uint32_t" => "u32",
        "int32_t" | "std::int32_t" => "i32",
        "uint64_t" | "std::uint64_t" => "u64",
        "int64_t" | "std::int64_t" => "i64",
        "size_t" | "std::size_t" => "usize",
        "ptrdiff_t" | "std::ptrdiff_t" => "isize",
        "rust::String" => "String",
        "rust::Vec<int>" | "rust::Vec<std::int32_t>" | "rust::Vec<int32_t>" => "Vec<i32>",
        "rust::Vec<double>" => "Vec<f64>",
        "rust::Vec<float>" => "Vec<f32>",
        "rust::Vec<uint8_t>" | "rust::Vec<std::uint8_t>" => "Vec<u8>",
        "rust::Option<uint8_t>" | "rust::Option<std::uint8_t>" => "Option<u8>",
        "rust::Result<rust::Option<uint8_t>,rust::String>"
        | "rust::Result<rust::Option<std::uint8_t>,rust::String>" => "Result<Option<u8>, String>",
        "rust::Vec<rust::Result<rust::Option<uint8_t>,rust::String>>"
        | "rust::Vec<rust::Result<rust::Option<std::uint8_t>,rust::String>>" => {
            "Vec<Result<Option<u8>, String>>"
        }
        other => {
            return Err(format!(
                "Unsupported type '{}'. Use Dual-Type Firewall: `[x: FooInt as \"Foo<int>\"]` where FooInt is imported from the bindings crate.",
                other
            ));
        }
    };

    Ok(mapped_base.to_string())
}

fn map_cpp_tokens_to_rust(
    tokens: &[proc_macro2::TokenTree],
) -> Result<proc_macro2::TokenStream, String> {
    let mut cpp_type = tokens.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(" ");
    for (from, to) in &[
        (" *", "*"),
        (" &", "&"),
        (" : : ", "::"),
        (":: ", "::"),
        (" ::", "::"),
        (" < ", "<"),
        ("< ", "<"),
        (" <", "<"),
        (" > ", ">"),
        (" >", ">"),
        ("> ", ">"),
        (" ,", ","),
        (", ", ","),
    ] {
        cpp_type = cpp_type.replace(from, to);
    }
    let cpp_type = cpp_type.trim();

    let rust_type_str = map_cpp_type_to_rust_str(cpp_type)?;
    rust_type_str
        .parse::<proc_macro2::TokenStream>()
        .map_err(|e| format!("Failed to parse mapped Rust type '{}': {}", rust_type_str, e))
}

fn parse_and_expand(
    input: proc_macro2::TokenStream,
    thunk_name: proc_macro2::Ident,
) -> Result<proc_macro2::TokenStream, String> {
    let mut cursor = TokenCursor::new(input);

    let new_format = matches!(cursor.get(0), Some(proc_macro2::TokenTree::Group(g)) if g.delimiter() == proc_macro2::Delimiter::Parenthesis)
        && matches!(cursor.get(1), Some(proc_macro2::TokenTree::Punct(p)) if p.as_char() == '-')
        && matches!(cursor.get(2), Some(proc_macro2::TokenTree::Punct(p)) if p.as_char() == '>');

    if !new_format {
        return Ok(quote! {
            {
                unsafe extern "C" {
                    fn #thunk_name();
                }
                unsafe { #thunk_name() }
            }
        });
    }

    let p_group = match cursor.next() {
        Some(proc_macro2::TokenTree::Group(g)) => g,
        _ => unreachable!(),
    };
    cursor.next();
    cursor.next();

    let mut ret_tokens = Vec::new();
    let mut body_group = None;
    while let Some(tok) = cursor.next() {
        match tok {
            proc_macro2::TokenTree::Group(ref g)
                if g.delimiter() == proc_macro2::Delimiter::Brace =>
            {
                body_group = Some(g.clone());
                break;
            }
            other => {
                ret_tokens.push(other);
            }
        }
    }

    if body_group.is_none() {
        return Err("Expected braced body after return type".to_string());
    }

    let param_tokens: Vec<proc_macro2::TokenTree> = p_group.stream().into_iter().collect();
    let param_chunks: Vec<&[proc_macro2::TokenTree]> = param_tokens
        .split(|t| matches!(t, proc_macro2::TokenTree::Punct(p) if p.as_char() == ','))
        .filter(|chunk| !chunk.is_empty())
        .collect();

    let mut params = Vec::new();
    for chunk in param_chunks {
        let name_token = match chunk.last() {
            Some(proc_macro2::TokenTree::Ident(id)) => id.clone(),
            _ => return Err("Parameter name must be an identifier".to_string()),
        };
        let type_tokens = &chunk[..chunk.len() - 1];
        if type_tokens.is_empty() {
            return Err(format!("Missing C++ type for parameter '{}'", name_token));
        }
        let rust_type = map_cpp_tokens_to_rust(type_tokens)?;
        params.push(Param { name: name_token, rust_type });
    }

    if ret_tokens.is_empty() {
        return Err("Expected C++ return type before braced body".to_string());
    }
    let return_rust_type = map_cpp_tokens_to_rust(&ret_tokens)?;

    let thunk_args = params.iter().map(|p| {
        let (n, t) = (&p.name, &p.rust_type);
        quote! { #n: <#t as ::into_cxx::IntoCxx>::TargetAbi }
    });

    let thunk_ret = quote! {
        <#return_rust_type as ::into_cxx::IntoRust>::TargetAbi
    };

    let closure_args = params.iter().map(|p| {
        let (n, t) = (&p.name, &p.rust_type);
        quote! { #n: #t }
    });

    let thunk_invoc_args = params.iter().map(|p| {
        let (n, t) = (&p.name, &p.rust_type);
        quote! { <#t as ::into_cxx::IntoCxx>::coerce(#n) }
    });

    Ok(quote! {
        {
            unsafe extern "C" {
                fn #thunk_name(
                    #( #thunk_args ),*
                ) -> #thunk_ret;
            }
            move |#( #closure_args ),*| -> #return_rust_type {
                unsafe {
                    <#return_rust_type as ::into_cxx::IntoRust>::rustify(
                        #thunk_name(
                            #( #thunk_invoc_args ),*
                        )
                    )
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
        use super::*;
    use googletest::prelude::*;
    use quote::format_ident;
    use std::str::FromStr;

    fn parse_stream(s: &str) -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::from_str(s).unwrap()
    }

    #[gtest]
    fn test_map_cpp_tokens_to_rust() {
        let check = |src: &str| {
            let stream = parse_stream(src);
            let tokens: Vec<_> = stream.into_iter().collect();
            map_cpp_tokens_to_rust(&tokens).unwrap().to_string()
        };

        let check_err = |src: &str| {
            let stream = parse_stream(src);
            let tokens: Vec<_> = stream.into_iter().collect();
            map_cpp_tokens_to_rust(&tokens).unwrap_err()
        };

        expect_eq!(check("int"), "i32");
        expect_eq!(check("double"), "f64");
        expect_eq!(check("rust :: Str"), "& str");
        expect_eq!(check("const char *"), "* const std :: os :: raw :: c_char");
        expect_eq!(check("char *"), "* mut std :: os :: raw :: c_char");
        expect_eq!(check("const int"), "i32");
        expect_eq!(check("int *"), "* mut i32");
        expect_eq!(check("const int &"), "& i32");
        expect_eq!(check("void *"), "* mut std :: ffi :: c_void");
        expect_eq!(check("const void *"), "* const std :: ffi :: c_void");
        expect_eq!(check("void"), "()");
        expect_eq!(
            check("rust::Vec<rust::Result<rust::Option<uint8_t>, rust::String>>"),
            "Vec < Result < Option < u8 >, String >>"
        );
        expect_eq!(check("std::int32_t"), "i32");
        expect_eq!(check("std::size_t"), "usize");

        expect_true!(check_err("std::vector<int>").contains("Unsupported type"));
        expect_true!(check_err("const std::vector<const int*>&").contains("Unsupported type"));
        expect_true!(check_err("math_utils::Point").contains("Unsupported type"));
    }

    #[gtest]
    fn test_parse_and_expand_legacy() {
        let input = parse_stream("test_global_val = 99;");
        let thunk_name = format_ident!("__some_thunk");
        let expected = quote! {
            {
                unsafe extern "C" {
                    fn __some_thunk();
                }
                unsafe { __some_thunk() }
            }
        };
        expect_eq!(parse_and_expand(input, thunk_name).unwrap().to_string(), expected.to_string());
    }

    #[gtest]
    fn test_parse_and_expand_new_syntax() {
        let input = parse_stream("(int x, double y) -> int { return x + y; }");
        let thunk_name = format_ident!("__some_thunk");
        let expected = quote! {
            {
                unsafe extern "C" {
                    fn __some_thunk(
                        x: <i32 as ::into_cxx::IntoCxx>::TargetAbi,
                        y: <f64 as ::into_cxx::IntoCxx>::TargetAbi
                    ) -> <i32 as ::into_cxx::IntoRust>::TargetAbi;
                }
                move |x: i32, y: f64| -> i32 {
                    unsafe {
                        <i32 as ::into_cxx::IntoRust>::rustify(
                            __some_thunk(
                                <i32 as ::into_cxx::IntoCxx>::coerce(x),
                                <f64 as ::into_cxx::IntoCxx>::coerce(y)
                            )
                        )
                    }
                }
            }
        };
        expect_eq!(parse_and_expand(input, thunk_name).unwrap().to_string(), expected.to_string());
    }
}

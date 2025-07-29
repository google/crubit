// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(rustc_private)]

use code_gen_utils::format_cc_includes;
use database::{SugaredTy, TypeLocation};
use generate_bindings::format_type::{format_ty_for_cc, format_ty_for_rs};
use generate_bindings::generate_function::get_fn_sig;
use proc_macro2::TokenStream;
use quote::quote;
use run_compiler_test_support::{find_def_id_by_name, run_compiler_for_testing};
use rustc_hir::FnRetTy;
use rustc_middle::ty::TyCtxt;
use test_helpers::bindings_db_for_tests;
use token_stream_matchers::assert_cc_matches;

fn test_ty<TestFn, Expectation>(
    type_location: TypeLocation,
    testcases: &[(&str, Expectation)],
    preamble: TokenStream,
    test_fn: TestFn,
) where
    TestFn: for<'tcx> Fn(
            /* testcase_description: */ &str,
            TyCtxt<'tcx>,
            SugaredTy<'tcx>,
            &Expectation,
        ) + Sync,
    Expectation: Sync,
{
    fn input_to_string(input: &str, preamble: &TokenStream, type_location: TypeLocation) -> String {
        let ty_tokens: TokenStream = input.parse().unwrap();
        let input = match type_location {
            TypeLocation::FnReturn => quote! {
                #preamble
                pub fn test_function() -> #ty_tokens { unimplemented!() }
            },
            TypeLocation::FnParam { .. } => quote! {
                #preamble
                pub fn test_function(_arg: #ty_tokens) { unimplemented!() }
            },
            TypeLocation::Const | TypeLocation::Other => unimplemented!(),
        };
        input.to_string()
    }

    fn get_test_function_ty<'tcx>(
        tcx: TyCtxt<'tcx>,
        type_location: TypeLocation,
    ) -> SugaredTy<'tcx> {
        let (sig_mid, sig_hir_opt) =
            get_fn_sig(tcx, find_def_id_by_name(tcx, "test_function").to_def_id());
        let sig_hir = sig_hir_opt.unwrap();
        match type_location {
            TypeLocation::FnReturn => {
                let FnRetTy::Return(ty_hir) = sig_hir.output else {
                    unreachable!(
                        "HIR return type should be fully specified, got: {:?}",
                        sig_hir.output
                    );
                };
                SugaredTy::new(sig_mid.output(), Some(ty_hir))
            }
            TypeLocation::FnParam { .. } => {
                SugaredTy::new(sig_mid.inputs()[0], Some(&sig_hir.inputs[0]))
            }
            TypeLocation::Const | TypeLocation::Other => unimplemented!(),
        }
    }

    for (index, (input, expected)) in testcases.iter().enumerate() {
        let desc = format!("test #{index}: test input: `{input}`");
        let input = input_to_string(input, &preamble, type_location);
        run_compiler_for_testing(input, |tcx| {
            let ty = get_test_function_ty(tcx, type_location);
            test_fn(&desc, tcx, ty, expected);
        });
    }
}

/// `test_format_ret_ty_for_cc_successes` provides test coverage for cases
/// where `format_ty_for_cc` takes `TypeLocation::FnReturn` and returns
/// an `Ok(...)`.  Additional testcases are covered by
/// `test_format_ty_for_cc_successes`.
#[test]
fn test_format_ret_ty_for_cc_successes() {
    let testcases = [
        // ( <Rust type>, <expected C++ type> )
        ("bool", "bool"), // TyKind::Bool
        ("()", "void"),
        // TODO(b/254507801): Expect `crubit::Never` instead (see the bug for more
        // details).
        ("!", "void"),
        (
            "extern \"C\" fn (f32, f32) -> f32",
            "crubit :: type_identity_t < float (float , float) > &",
        ),
    ];
    test_ty(TypeLocation::FnReturn, &testcases, quote! {}, |desc, tcx, ty, expected| {
        let actual = {
            let db = bindings_db_for_tests(tcx);
            let cc_snippet = format_ty_for_cc(&db, ty, TypeLocation::FnReturn).unwrap();
            cc_snippet.tokens.to_string()
        };
        let expected = expected.parse::<TokenStream>().unwrap().to_string();
        assert_eq!(actual, expected, "{desc}");
    });
}

/// `test_format_ty_for_cc_successes` provides test coverage for cases where
/// `format_ty_for_cc` returns an `Ok(...)`.
///
/// Note that using `std::int8_t` (instead of `::std::int8_t`) has been an
/// explicit decision. The "Google C++ Style Guide" suggests to "avoid
/// nested namespaces that match well-known top-level namespaces" and "in
/// particular, [...] not create any nested std namespaces.".  It
/// seems desirable if the generated bindings conform to this aspect of the
/// style guide, because it makes things easier for *users* of these
/// bindings.
#[test]
fn test_format_ty_for_cc_successes() {
    struct FormatCcExpectation {
        expected_tokens: &'static str,
        expected_includes: Vec<&'static str>,
        expected_prereq_def: Option<&'static str>,
        expected_prereq_fwd_decl: Option<&'static str>,
    }

    // Helper macro to create a `FormatCcExpectation`s. Handles all a variation of
    // relevant fields (e.g. expected includes or forward decls).
    macro_rules! case {
        (rs: $input_rust_ty:expr, cc: $expected_cc_ty:expr, includes: [$($includes:expr),*], prereq_def: $expected_prereq_def:expr, prereq_fwd_decl: $expected_prereq_fwd_decl:expr) => {
            (
                $input_rust_ty,
                FormatCcExpectation {
                    expected_tokens: $expected_cc_ty,
                    expected_includes: Vec::<&'static str>::from([$($includes),*]),
                    expected_prereq_def: $expected_prereq_def,
                    expected_prereq_fwd_decl: $expected_prereq_fwd_decl,
                }
        )
        };
        (rs: $input_rust_ty:expr, cc: $expected_cc_ty:expr) => {
            case!(rs: $input_rust_ty, cc: $expected_cc_ty, includes: [], prereq_def: None, prereq_fwd_decl: None)
        };
        (rs: $input_rust_ty:expr, cc: $expected_cc_ty:expr, includes: [$($includes:expr),*]) => {
            case!(rs: $input_rust_ty, cc: $expected_cc_ty, includes: [$($includes),*], prereq_def: None, prereq_fwd_decl: None)
        };
        (rs: $input_rust_ty:expr, cc: $expected_cc_ty:expr, includes: [$($includes:expr),*], prereq_def: $expected_prereq_def:expr) => {
            case!(rs: $input_rust_ty, cc: $expected_cc_ty, includes: [$($includes),*], prereq_def: Some($expected_prereq_def), prereq_fwd_decl: None)
        };
        (rs: $input_rust_ty:expr, cc: $expected_cc_ty:expr, includes: [$($includes:expr),*], prereq_fwd_decl: $expected_prereq_fwd_decl:expr) => {
            case!(rs: $input_rust_ty, cc: $expected_cc_ty, includes: [$($includes),*], prereq_def: None, prereq_fwd_decl: Some($expected_prereq_fwd_decl))
        };
    }

    let testcases = [
        case!(rs: "bool", cc:  "bool"),
        case!(rs: "f32", cc: "float"),
        case!(rs: "f64", cc: "double"),
        // The ffi aliases are special-cased to refer to the C++ fundamental integer types,
        // if the type alias information is not lost (e.g. from generics).
        case!(rs: "std::ffi::c_char", cc:  "char"),
        case!(rs: "::std::ffi::c_char", cc:  "char"),
        case!(rs: "core::ffi::c_char", cc:  "char"),
        case!(rs: "::core::ffi::c_char", cc:  "char"),
        case!(rs: "std::ffi::c_uchar", cc: "unsigned char"),
        case!(rs: "std::ffi::c_longlong", cc: "long long"),
        case!(rs: "c_char", cc:  "char"),
        // Simple pointers/references do not lose the type alias information.
        case!(rs: "*const std::ffi::c_uchar", cc: "unsigned char const *"),
        case!(
            rs: "&'static std::ffi::c_uchar",
            cc: "unsigned char const * [[clang :: annotate_type (\"lifetime\" , \"static\")]]"
        ),
        // Generics lose type alias information.
        case!(rs: "Identity<std::ffi::c_longlong>", cc: "std::int64_t", includes: ["<cstdint>"]),
        case!(rs: "i8", cc: "std::int8_t", includes: ["<cstdint>"]),
        case!(rs: "i16", cc:  "std::int16_t", includes: ["<cstdint>"]),
        case!(rs: "i32", cc:  "std::int32_t", includes: ["<cstdint>"]),
        case!(rs: "i64", cc:  "std::int64_t", includes: ["<cstdint>"]),
        case!(rs: "isize", cc: "std::intptr_t", includes: ["<cstdint>"]),
        case!(rs: "u8", cc: "std::uint8_t", includes: ["<cstdint>"]),
        case!(rs: "u16", cc: "std::uint16_t", includes: ["<cstdint>"]),
        case!(rs: "u32", cc: "std::uint32_t", includes: ["<cstdint>"]),
        case!(rs: "u64", cc: "std::uint64_t", includes: ["<cstdint>"]),
        case!(rs: "usize", cc: "std::uintptr_t", includes: ["<cstdint>"]),
        case!(
            rs: "char",
            cc: "rs_std::char_",
            includes: ["<crubit/support/for/tests/rs_std/char.h>"]
        ),
        case!(rs: "SomeStruct", cc: "::rust_out::SomeStruct", includes: [],  prereq_def: "SomeStruct"),
        case!(rs: "SomeEnum", cc: "::rust_out::SomeEnum", includes: [], prereq_def: "SomeEnum"),
        case!(rs: "SomeUnion", cc: "::rust_out::SomeUnion", includes: [], prereq_def: "SomeUnion"),
        case!(rs: "*const i32", cc: "std :: int32_t const *", includes: ["<cstdint>"]),
        case!(rs: "*mut i32", cc: "std :: int32_t *", includes: ["<cstdint>"]),
        case!(
            rs: "&'static i32",
            cc: "std :: int32_t const * [[clang :: annotate_type (\"lifetime\" , \"static\")]]",
            includes: ["<cstdint>"]
        ),
        case!(
            rs: "&'static mut i32",
            cc: "std :: int32_t * [[clang :: annotate_type (\"lifetime\" , \"static\")]]",
            includes: ["<cstdint>"]
        ),
        // Slice pointers:
        case!(
            rs: "*const [i8]",
            cc: "rs_std::SliceRef<const std::int8_t>",
            includes: ["<cstdint>", "<crubit/support/for/tests/rs_std/slice_ref.h>"]
        ),
        case!(
            rs: "*mut [i64]",
            cc: "rs_std::SliceRef<std::int64_t>",
            includes: ["<cstdint>", "<crubit/support/for/tests/rs_std/slice_ref.h>"]
        ),
        case!(
            rs: "*const [c_char]",
            cc: "rs_std::SliceRef<const char>",
            includes: ["<crubit/support/for/tests/rs_std/slice_ref.h>"]
        ),
        case!(
            rs: "*mut [SomeStruct]",
            cc: "rs_std::SliceRef< ::rust_out::SomeStruct>",
            includes: [ "<crubit/support/for/tests/rs_std/slice_ref.h>"],
            prereq_def: "SomeStruct"

        ),
        // `SomeStruct` is a `fwd_decls` prerequisite (not `defs` prerequisite):
        case!(
            rs: "*mut SomeStruct",
            cc: "::rust_out::SomeStruct*",
            includes: [],
            prereq_fwd_decl: "SomeStruct"
        ),
        // Testing propagation of deeper/nested `fwd_decls`:
        case!(
            rs: "*mut *mut SomeStruct",
            cc: ":: rust_out :: SomeStruct * *",
            includes: [],
            prereq_fwd_decl: "SomeStruct"
        ),
        // Testing propagation of `const` / `mut` qualifiers:
        case!(rs: "*mut *const f32", cc: "float const * *"),
        case!(rs: "*const *mut f32", cc: "float * const *"),
        // Rust function pointers are non-nullable, so when function pointers are used as a
        // parameter type (i.e. in `TypeLocation::FnParam`) then we can translate to
        // generate a C++ function *reference*, rather than a C++ function *pointer*.
        case!(
            rs: "extern \"C\" fn (f32, f32) -> f32",
            cc: "crubit :: type_identity_t < float (float , float) > &",
            includes: ["<crubit/support/for/tests/internal/cxx20_backports.h>"]
        ),
        // Unsafe extern "C" function pointers are, to C++, just function pointers.
        case!(
            rs: "unsafe extern \"C\" fn(f32, f32) -> f32",
            cc: "crubit :: type_identity_t < float (float , float) > &",
            includes: ["<crubit/support/for/tests/internal/cxx20_backports.h>"]
        ),
        // Nested function pointer (i.e. `TypeLocation::Other`) means that
        // we need to generate a C++ function *pointer*, rather than a C++
        // function *reference*.
        case!(
            rs: "*const extern \"C\" fn (f32, f32) -> f32",
            cc: "crubit :: type_identity_t < float (float , float) > * const *",
            includes: ["<crubit/support/for/tests/internal/cxx20_backports.h>"]
        ),
        // Extra parens/sugar are expected to be ignored:
        case!(rs: "(bool)", cc: "bool"),
        // References to MaybeUninit:
        case!(
            rs: "*const std::mem::MaybeUninit<i32>",
            cc: "std :: int32_t const *",
            includes: ["<cstdint>"]
        ),
        case!(
            rs: "&mut std::mem::MaybeUninit<i32>",
            cc: "std :: int32_t &",
            includes: ["<cstdint>"]
        ),
        case!(
            rs: "()",
            cc: "std::tuple < >",
            includes: ["<tuple>"]
        ),
        case!(
            rs: "(i32,)",
            cc: "std::tuple<std::int32_t>",
            includes: ["<cstdint>", "<tuple>"]
        ),
        case!(
            rs: "(i32, i32)",
            cc: "std::tuple<std::int32_t, std::int32_t>",
            includes: ["<cstdint>", "<tuple>"]
        ),
    ];
    let preamble = quote! {
        #![allow(unused_parens)]
        #![feature(register_tool)]
        #![register_tool(__crubit)]

        pub struct SomeStruct {
            pub x: i32,
            pub y: i32,
        }
        pub enum SomeEnum {
            Cartesian{x: f64, y: f64},
            Polar{angle: f64, dist: f64},
        }
        pub union SomeUnion {
            pub x: i32,
            pub y: i32,
        }

        #[allow(unused)]
        type Identity<T> = T;

        pub use core::ffi::c_char;
        // TODO(b/283258442): Correctly handle something like this:
        // pub type MyChar = core::ffi::c_char;
    };
    test_ty(
        TypeLocation::FnParam { is_self_param: false, elided_is_output: false },
        &testcases,
        preamble,
        |desc,
         tcx,
         ty,
         FormatCcExpectation {
             expected_tokens,
             expected_includes,
             expected_prereq_def,
             expected_prereq_fwd_decl,
         }| {
            let (actual_tokens, actual_prereqs) = {
                let db = bindings_db_for_tests(tcx);
                let s = format_ty_for_cc(
                    &db,
                    ty,
                    TypeLocation::FnParam { is_self_param: false, elided_is_output: false },
                )
                .unwrap();
                (s.tokens.to_string(), s.prereqs)
            };
            let (actual_includes, actual_prereq_defs, actual_prereq_fwd_decls) =
                (actual_prereqs.includes, actual_prereqs.defs, actual_prereqs.fwd_decls);

            let expected_tokens = expected_tokens.parse::<TokenStream>().unwrap().to_string();
            assert_eq!(actual_tokens, expected_tokens, "{desc}");

            assert!(
                expected_includes.len() == actual_includes.len(),
                "{desc}: `actual_includes` is unexpectedly not of the same length as `expected_includes`. actual_includes: {actual_includes:#?}; expected_includes: {expected_includes:#?}"
            );

            if expected_includes.len() > 0 {
                let expected_includes = expected_includes
                    .into_iter()
                    .map(|include| include.parse::<TokenStream>().unwrap())
                    .collect::<Vec<_>>();
                assert_cc_matches!(
                    format_cc_includes(&actual_includes),
                    quote! { #( __HASH_TOKEN__ include #expected_includes )*}
                );
            }

            if let Some(expected_prereq_def) = expected_prereq_def {
                let expected_def_id = find_def_id_by_name(tcx, expected_prereq_def).to_def_id();
                assert_eq!(1, actual_prereq_defs.len());
                assert_eq!(expected_def_id, actual_prereq_defs.into_iter().next().unwrap());
            } else {
                assert!(
                    actual_prereq_defs.is_empty(),
                    "{desc}: `actual_prereq_defs` is unexpectedly non-empty",
                );
            }

            if let Some(expected_prereq_fwd_decl) = expected_prereq_fwd_decl {
                let expected_def_id =
                    find_def_id_by_name(tcx, expected_prereq_fwd_decl).to_def_id();
                assert_eq!(1, actual_prereq_fwd_decls.len());
                assert_eq!(expected_def_id, actual_prereq_fwd_decls.into_iter().next().unwrap());
            } else {
                assert!(
                    actual_prereq_fwd_decls.is_empty(),
                    "{desc}: `actual_prereq_fwd_decls` is unexpectedly non-empty",
                );
            }
        },
    );
}

/// `test_format_ty_for_cc_failures` provides test coverage for cases where
/// `format_ty_for_cc` returns an `Err(...)`.
///
/// It seems okay to have no test coverage for now for the following types
/// (which should never be encountered when generating bindings and where
/// `format_ty_for_cc` should panic):
/// - TyKind::Closure
/// - TyKind::Error
/// - TyKind::FnDef
/// - TyKind::Infer
///
/// TODO(lukasza): Add test coverage (here and in the "for_rs" flavours)
/// for:
/// - TyKind::Bound
/// - TyKind::Dynamic (`dyn Eq`)
/// - TyKind::Foreign (`extern type T`)
/// - https://doc.rust-lang.org/beta/unstable-book/language-features/generators.html:
///   TyKind::Generator, TyKind::GeneratorWitness
/// - TyKind::Param
/// - TyKind::Placeholder
#[test]
fn test_format_ty_for_cc_failures() {
    let testcases = [
        // ( <Rust type>, <expected error message> )
        (
            // TODO(b/254507801): Expect `crubit::Never` instead (see the bug for more
            // details).
            "!", // TyKind::Never
            "The never type `!` is only supported as a return type (b/254507801)",
        ),
        (
            "&'static &'static i32", // TyKind::Ref (nested reference - referent of reference)
            "Failed to format the referent of the reference type `&'static &'static i32`: \
             Can't format `&'static i32`, because references are only supported \
             in function parameter types, return types, and consts (b/286256327)",
        ),
        (
            "extern \"C\" fn (&i32)", // TyKind::Ref (nested reference - underneath fn ptr)
            "Generic function pointers are not supported yet (b/259749023)",
        ),
        (
            "[i32; 42]", // TyKind::Array
            "The following Rust type is not supported yet: [i32; 42]",
        ),
        (
            // Check that the failure for slices is about not being supported and not failed
            // asserts about ABI and layout.
            "&'static [i32]", // TyKind::Slice (nested underneath TyKind::Ref)
            "Failed to format the referent of the reference type `&'static [i32]`: \
             The following Rust type is not supported yet: [i32]",
        ),
        (
            "impl Eq", // TyKind::Alias
            "The following Rust type is not supported yet: impl Eq",
        ),
        (
            "fn(i32) -> i32", // TyKind::FnPtr (default ABI = "Rust")
            "Function pointers can't have a thunk: \
             Any calling convention other than `extern \"C\"` requires a thunk",
        ),
        (
            "extern \"C\" fn (SomeStruct, f32) -> f32",
            "Function pointers can't have a thunk: Type of parameter #0 requires a thunk",
        ),
        (
            "extern \"C\" fn (f32, f32) -> SomeStruct",
            "Function pointers can't have a thunk: Return type requires a thunk",
        ),
        // TODO(b/254094650): Consider mapping this to Clang's (and GCC's) `__int128`
        // or to `absl::in128`.
        ("i128", "C++ doesn't have a standard equivalent of `i128` (b/254094650)"),
        ("u128", "C++ doesn't have a standard equivalent of `u128` (b/254094650)"),
        ("ConstGenericStruct<42>", "Generic types are not supported yet (b/259749095)"),
        ("TypeGenericStruct<u8>", "Generic types are not supported yet (b/259749095)"),
        (
            // This double-checks that TyKind::Adt(..., substs) are present
            // even if the type parameter argument is not explicitly specified
            // (here it comes from the default: `...Struct<T = u8>`).
            "TypeGenericStruct",
            "Generic types are not supported yet (b/259749095)",
        ),
        (
            "std::cmp::Ordering",
            "Type `std::cmp::Ordering` comes from the `core` crate, \
             but no `--crate-header` was specified for this crate",
        ),
        ("Option<i8>", "Generic types are not supported yet (b/259749095)"),
        (
            // TODO(b/258261328): Once cross-crate bindings are supported we should try
            // to test them via a test crate that we control (rather than testing via
            // implementation details of the std crate).
            "core::alloc::LayoutError",
            "Type `std::alloc::LayoutError` comes from the `core` crate, but no `--crate-header` was specified for this crate",
        ),
        (
            "*const Option<i8>",
            "Failed to format the pointee \
             of the pointer type `*const std::option::Option<i8>`: \
             Generic types are not supported yet (b/259749095)",
        ),
    ];
    let preamble = quote! {
        #![feature(never_type)]

        #[repr(C)]
        pub struct SomeStruct {
            pub x: i32,
            pub y: i32,
        }

        pub struct ConstGenericStruct<const N: usize> {
            pub arr: [u8; N],
        }

        pub struct TypeGenericStruct<T = u8> {
            pub t: T,
        }
    };
    test_ty(
        TypeLocation::FnParam { is_self_param: false, elided_is_output: false },
        &testcases,
        preamble,
        |desc, tcx, ty, expected_msg| {
            let db = bindings_db_for_tests(tcx);
            let anyhow_err = format_ty_for_cc(
                &db,
                ty,
                TypeLocation::FnParam { is_self_param: false, elided_is_output: false },
            )
            .expect_err(&format!("Expecting error for: {desc}"));
            let actual_msg = format!("{anyhow_err:#}");
            assert_eq!(&actual_msg, *expected_msg, "{desc}");
        },
    );
}

#[test]
fn test_format_ty_for_rs_successes() {
    // Test coverage for cases where `format_ty_for_rs` returns an `Ok(...)`.
    let testcases = [
        // ( <Rust type>, <expected Rust spelling for ..._cc_api_impl.rs> )
        ("bool", "bool"),
        ("f32", "f32"),
        ("f64", "f64"),
        ("i8", "i8"),
        ("i16", "i16"),
        ("i32", "i32"),
        ("i64", "i64"),
        ("i128", "i128"),
        ("isize", "isize"),
        ("u8", "u8"),
        ("u16", "u16"),
        ("u32", "u32"),
        ("u64", "u64"),
        ("u128", "u128"),
        ("usize", "usize"),
        ("char", "char"),
        ("!", "!"),
        ("()", "()"),
        // ADTs:
        ("SomeStruct", "::rust_out::SomeStruct"),
        ("SomeEnum", "::rust_out::SomeEnum"),
        ("SomeUnion", "::rust_out::SomeUnion"),
        // Type from another crate:
        ("std::cmp::Ordering", "::core::cmp::Ordering"),
        // `const` and `mut` pointers:
        ("*const i32", "*const i32"),
        ("*mut i32", "*mut i32"),
        // References:
        ("&i32", "& '__anon1 i32"),
        ("&mut i32", "& '__anon1 mut i32"),
        ("&'_ i32", "& '__anon1 i32"),
        ("&'static i32", "& 'static i32"),
        // Pointer to an ADT:
        ("*mut SomeStruct", "* mut :: rust_out :: SomeStruct"),
        ("extern \"C\" fn(i32) -> i32", "extern \"C\" fn(i32) -> i32"),
        // Pointer to a Slice:
        ("*mut [i32]", "*mut [i32]"),
        // str reference:
        ("&'static str", "& 'static str"),
        // MaybeUninit:
        ("&'static std::mem::MaybeUninit<i32>", "& 'static std :: mem :: MaybeUninit < i32 >"),
        (
            "&'static mut std::mem::MaybeUninit<i32>",
            "& 'static mut std :: mem :: MaybeUninit < i32 >",
        ),
        ("*const std::mem::MaybeUninit<i32>", "*const std::mem::MaybeUninit<i32>"),
        ("*mut std::mem::MaybeUninit<i32>", "*mut std::mem::MaybeUninit<i32>"),
        ("LifetimeGenericStruct<'static>", "::rust_out::LifetimeGenericStruct< 'static >"),
    ];
    let preamble = quote! {
        #![feature(never_type)]

        pub struct SomeStruct {
            pub x: i32,
            pub y: i32,
        }
        pub enum SomeEnum {
            Cartesian{x: f64, y: f64},
            Polar{angle: f64, dist: f64},
        }
        pub union SomeUnion {
            pub x: i32,
            pub y: i32,
        }
        pub struct LifetimeGenericStruct<'a> {
            pub reference: &'a u8,
        }
    };
    test_ty(
        TypeLocation::FnParam { is_self_param: false, elided_is_output: false },
        &testcases,
        preamble,
        |desc, tcx, ty, expected_tokens| {
            let db = bindings_db_for_tests(tcx);
            let actual_tokens = format_ty_for_rs(&db, ty.mid()).unwrap().to_string();
            let expected_tokens = expected_tokens.parse::<TokenStream>().unwrap().to_string();
            assert_eq!(actual_tokens, expected_tokens, "{desc}");
        },
    );
}

#[test]
fn test_format_ty_for_rs_failures() {
    // This test provides coverage for cases where `format_ty_for_rs` returns an
    // `Err(...)`.
    let testcases = [
        // ( <Rust type>, <expected error message> )
        (
            "[i32; 42]", // TyKind::Array
            "The following Rust type is not supported yet: [i32; 42]",
        ),
        (
            "impl Eq", // TyKind::Alias
            "The following Rust type is not supported yet: impl Eq",
        ),
        (
            "Option<i8>", // TyKind::Adt - generic + different crate
            "Generic types are not supported yet (b/259749095)",
        ),
    ];
    let preamble = quote! {};
    test_ty(
        TypeLocation::FnParam { is_self_param: false, elided_is_output: false },
        &testcases,
        preamble,
        |desc, tcx, ty, expected_err| {
            let db = bindings_db_for_tests(tcx);
            let anyhow_err =
                format_ty_for_rs(&db, ty.mid()).expect_err(&format!("Expecting error for: {desc}"));
            let actual_err = format!("{anyhow_err:#}");
            assert_eq!(&actual_err, *expected_err, "{desc}");
        },
    );
}

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(rustc_private)]

use code_gen_utils::{CcInclude, NamespaceQualifier};
use generate_bindings::format_namespace_bound_cc_tokens;
use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::quote;
use run_compiler_test_support::run_compiler_for_testing;
use rustc_span::symbol::Symbol;
use test_helpers::{bindings_db_for_tests, test_format_item, test_generated_bindings};
use token_stream_matchers::{
    assert_cc_matches, assert_cc_not_matches, assert_rs_matches, assert_rs_not_matches,
};
use token_stream_printer::cc_tokens_to_formatted_string_for_tests;

/// This test covers only a single example of a function that should get a
/// C++ binding. The test focuses on verification that the output from
/// `generate_function` gets propagated all the way to
/// `GenerateBindings::new`. Additional coverage of how functions are
/// formatted is provided by `test_format_item_..._fn_...` tests (which
/// work at the `generate_function` level).
#[test]
fn test_generated_bindings_fn_no_mangle_extern_c() {
    let test_src = r#"
            #[unsafe(no_mangle)]
            pub extern "C" fn public_function() {
                println!("foo");
            }
        "#;
    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        assert_cc_matches!(
            bindings.cc_api,
            quote! {
                extern "C" void public_function();
            }
        );

        // No Rust thunks should be generated in this test scenario.
        assert_rs_not_matches!(bindings.cc_api_impl, quote! { public_function });
    });
}

/// Tests that `toposort` is used to reorder item bindings.
#[test]
fn test_generated_bindings_prereq_defs_field_deps_require_reordering() {
    let test_src = r#"
            #![allow(dead_code)]

            // In the generated bindings `Outer` needs to come *after* `Inner`.
            pub struct Outer(Inner);
            pub struct Inner(bool);
        "#;
    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        assert_cc_matches!(
            bindings.cc_api,
            quote! {
                namespace rust_out {
                ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(1) [[clang::trivial_abi]] Inner final {
                      ... union { ... bool __field0; }; ...
                    };
                ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(1) [[clang::trivial_abi]] Outer final {
                      ... union { ... ::rust_out::Inner __field0; }; ...
                    };
                ...
                }  // namespace rust_out
            }
        );
    });
}

/// Tests that item bindings have a stable order.
//  Because the source location of both calls to the proc macro are the same,
//  "span" will be the same, which means that without a stable sort for items
//  where the span is equivalent, the output will be non-deterministic.
#[test]
fn test_generated_bindings_have_stable_order() {
    fn idents_in_stream(stream: TokenStream, idents: &mut Vec<Ident>) {
        for tree in stream {
            match tree {
                TokenTree::Group(group) => idents_in_stream(group.stream(), idents),
                TokenTree::Ident(ident) => idents.push(ident),
                TokenTree::Punct(_) | TokenTree::Literal(_) => {}
            }
        }
    }

    /// Returns the index in `idents` of `{name} final`.
    #[track_caller]
    fn get_final_decl_position(idents: &[Ident], name: &str, debug_string: &str) -> usize {
        let Some((i, _ident)) = idents.iter().enumerate().find(|(i, ident)| {
            *ident == name && idents.get(i + 1).map(|next| next == "final").unwrap_or(false)
        }) else {
            panic!("`{name} final` declaration not found in input:\n{debug_string}")
        };
        i
    }

    let test_src = r#"
            #![allow(dead_code)]

            macro_rules! make_struct {
            ($ty: ty, $name:ident) => {
            pub struct $name {
                value: $ty
                }
            }
            }

            make_struct!(i32, HelloF32);
            make_struct!(f32, HelloI32);
        "#;

    const NUM_ITERATIONS: u8 = 5;
    // Check that the ordering of `HelloI32` and `HelloF32` is consistent across
    // all iterations.
    let mut i32_was_first: Option<bool> = None;
    for _ in 0..NUM_ITERATIONS {
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            let cc_api: TokenStream = bindings.cc_api;
            let cc_api_debug_string =
                cc_tokens_to_formatted_string_for_tests(cc_api.clone()).unwrap();

            let mut idents = Vec::new();
            idents_in_stream(cc_api, &mut idents);

            let i32_decl_position =
                get_final_decl_position(&idents, "HelloI32", &cc_api_debug_string);
            let f32_decl_position =
                get_final_decl_position(&idents, "HelloF32", &cc_api_debug_string);
            let i32_is_first = i32_decl_position < f32_decl_position;
            if let Some(i32_was_first) = i32_was_first {
                if i32_is_first != i32_was_first {
                    panic!("`HelloI32` declaration ordering is inconsistent.");
                }
            } else {
                i32_was_first = Some(i32_is_first);
            }
        });
    }
}

/// Tests that a forward declaration is present when it is required to
/// preserve the original source order.  In this test the
/// `CcPrerequisites::fwd_decls` dependency comes from a pointer parameter.
#[test]
fn test_generated_bindings_prereq_fwd_decls_for_ptr_param() {
    let test_src = r#"
            #![allow(dead_code)]

            // To preserve original API order we need to forward declare S.
            pub fn f(_: *const S) {}
            pub struct S(bool);
        "#;
    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        assert_cc_matches!(
            bindings.cc_api,
            quote! {
                namespace rust_out {
                    ...
                    // Verifying the presence of this forward declaration
                    // it the essence of this test.  The order of the items
                    // below also matters.
                    struct S;
                    ...
                    void f(::rust_out::S const* __param_0);
                    ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(...) [[clang::trivial_abi]] S final { ... }
                    ...
                    inline void f(::rust_out::S const* __param_0) { ... }
                    ...
                }  // namespace rust_out
            }
        );
    });
}

/// Tests that a forward declaration is present when it is required to
/// preserve the original source order.  In this test the
/// `CcPrerequisites::fwd_decls` dependency comes from a
/// function declaration that has a parameter that takes a struct by value.
#[test]
fn test_generated_bindings_prereq_fwd_decls_for_cpp_fn_decl() {
    let test_src = r#"
            #[unsafe(no_mangle)]
            pub extern "C" fn f(s: S) -> bool { s.0 }

            #[repr(C)]
            pub struct S(bool);
        "#;

    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        assert_cc_matches!(
            bindings.cc_api,
            quote! {
                namespace rust_out {
                    ...
                    // Verifying the presence of this forward declaration
                    // is the essence of this test.  The order also matters:
                    // 1. The fwd decl of `S` should come first,
                    // 2. Declaration of `f` and definition of `S` should come next
                    //    (in their original order - `f` first and then `S`).
                    struct S;
                    ...
                    // `CcPrerequisites` of `f` declaration below (the main api of `f`) should
                    // include `S` as a `fwd_decls` edge, rather than as a `defs` edge.
                    bool f(::rust_out::S s);
                    ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(...) [[clang::trivial_abi]] S final { ... }
                    ...
                }  // namespace rust_out
            }
        );
    });
}

/// This test verifies that a forward declaration for a given ADT is only
/// emitted once (and not once for every API item that requires the
/// forward declaration as a prerequisite).
#[test]
fn test_generated_bindings_prereq_fwd_decls_no_duplication() {
    let test_src = r#"
            #![allow(dead_code)]

            // All three functions below require a forward declaration of S.
            pub fn f1(_: *const S) {}
            pub fn f2(_: *const S) {}
            pub fn f3(_: *const S) {}

            pub struct S(bool);

            // This function also includes S in its CcPrerequisites::fwd_decls
            // (although here it is not required, because the definition of S
            // is already available above).
            pub fn f4(_: *const S) {}
        "#;
    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap().cc_api.to_string();

        // Only a single forward declaration is expected.
        assert_eq!(1, bindings.matches("struct S ;").count(), "bindings = {bindings}");
    });
}

/// This test verifies that forward declarations are emitted in a
/// deterministic order. The particular order doesn't matter _that_
/// much, but it definitely shouldn't change every time
/// `cc_bindings_from_rs` is invoked again.  The current order preserves
/// the original source order of the Rust API items.
#[test]
fn test_generated_bindings_prereq_fwd_decls_deterministic_order() {
    let test_src = r#"
            #![allow(dead_code)]

            // To try to mix things up, the bindings for the functions below
            // will *ask* for forward declarations in a different order:
            // * Different from the order in which the forward declarations
            //   are expected to be *emitted* (the original source order).
            // * Different from alphabetical order.
            pub fn f1(_: *const b::S3) {}
            pub fn f2(_: *const a::S2) {}
            pub fn f3(_: *const a::S1) {}

            pub mod a {
                pub struct S1(bool);
                pub struct S2(bool);
            }

            pub mod b {
                pub struct S3(bool);
            }
        "#;
    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        assert_cc_matches!(
            bindings.cc_api,
            quote! {
                namespace rust_out {
                    ...
                    // Verifying that we get the same order in each test
                    // run is the essence of this test.
                    namespace a {
                    struct S1;
                    struct S2;
                    }
                    namespace b {
                    struct S3;
                    }
                    ...
                    void f1 ...
                    void f2 ...
                    void f3 ...

                    namespace a { ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(...) [[clang::trivial_abi]] S1 final { ... } ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(...) [[clang::trivial_abi]] S2 final { ... } ...
                    } ...
                    namespace b { ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(...) [[clang::trivial_abi]] S3 final { ... } ...
                    } ...
                }  // namespace rust_out
            }
        );
    });
}

/// This test verifies that forward declarations are not emitted if they are
/// not needed (e.g. if bindings the given `struct` or other ADT have
/// already been defined earlier).  In particular, we don't want to emit
/// forward declarations for *all* `structs` (regardless if they are
/// needed or not).
#[test]
fn test_generated_bindings_prereq_fwd_decls_not_needed_because_of_initial_order() {
    let test_src = r#"
            #[allow(dead_code)]

            pub struct S(bool);

            // S is already defined above - no need for forward declaration in C++.
            pub fn f(_s: *const S) {}
        "#;
    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        assert_cc_not_matches!(bindings.cc_api, quote! { struct S; });
        assert_cc_matches!(bindings.cc_api, quote! { void f(::rust_out::S const* _s); });
    });
}

/// This test verifies that a method declaration doesn't ask for a forward
/// declaration to the struct.
#[test]
fn test_generated_bindings_prereq_fwd_decls_not_needed_inside_struct_definition() {
    let test_src = r#"
            #![allow(dead_code)]

            pub struct S {
                // This shouldn't require a fwd decl of S.
                field: *const S,
            }

            impl S {
                // This shouldn't require a fwd decl of S.
                pub fn create() -> S { Self{ field: std::ptr::null() } }
            }
        "#;
    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        assert_cc_not_matches!(bindings.cc_api, quote! { struct S; });
        assert_cc_matches!(
            bindings.cc_api,
            quote! {
                static ::rust_out::S create(); ...
                union { ... ::rust_out::S const* field; }; ...
            }
        );
    });
}

#[test]
fn test_generated_bindings_module_basics() {
    let test_src = r#"
            pub mod some_module {
                pub fn some_func() {}
            }
        "#;
    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        assert_cc_matches!(
            bindings.cc_api,
            quote! {
                namespace rust_out {
                    namespace some_module {
                        ...
                        inline void some_func() { ... }
                        ...
                    }  // namespace some_module
                }  // namespace rust_out
            }
        );
        assert_rs_matches!(
            bindings.cc_api_impl,
            quote! {
                #[unsafe(no_mangle)]
                unsafe extern "C"
                fn ...() -> () {
                    unsafe { ::rust_out::some_module::some_func() }
                }
            }
        );
    });
}

#[test]
fn test_generated_bindings_module_name_is_cpp_reserved_keyword() {
    let test_src = r#"
            pub mod reinterpret_cast {
                pub fn working_module_f1() {}
                pub fn working_module_f2() {}
            }
        "#;
    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();

        assert_cc_matches!(
            bindings.cc_api,
            quote! {
                namespace rust_out {
                    namespace reinterpret_cast_ {
                        ...
                        void working_module_f1();
                        ...
                        void working_module_f2();
                        ...
                    }  // namespace reinterpret_cast_

                }  // namespace rust_out
            }
        );
    });
}

/// `test_generated_bindings_non_pub_items` verifies that non-public items
/// are not present/propagated into the generated bindings.
#[test]
fn test_generated_bindings_non_pub_items() {
    let test_src = r#"
            #![allow(dead_code)]

            extern "C" fn private_function() {
                println!("foo");
            }

            struct PrivateStruct {
                x: i32,
                y: i32,
            }

            pub struct PublicStruct(i32);

            impl PublicStruct {
                fn private_method() {}
            }

            pub mod public_module {
                fn priv_func_in_pub_module() {}
            }

            mod private_module {
                pub fn pub_func_in_priv_module() { priv_func_in_priv_module() }
                fn priv_func_in_priv_module() {}
            }
        "#;
    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        assert_cc_not_matches!(bindings.cc_api, quote! { private_function });
        assert_rs_not_matches!(bindings.cc_api_impl, quote! { private_function });
        assert_cc_not_matches!(bindings.cc_api, quote! { PrivateStruct });
        assert_rs_not_matches!(bindings.cc_api_impl, quote! { PrivateStruct });
        assert_cc_not_matches!(bindings.cc_api, quote! { private_method });
        assert_rs_not_matches!(bindings.cc_api_impl, quote! { private_method });
        assert_cc_not_matches!(bindings.cc_api, quote! { priv_func_in_priv_module });
        assert_rs_not_matches!(bindings.cc_api_impl, quote! { priv_func_in_priv_module });
        assert_cc_not_matches!(bindings.cc_api, quote! { priv_func_in_pub_module });
        assert_rs_not_matches!(bindings.cc_api_impl, quote! { priv_func_in_pub_module });
        assert_cc_not_matches!(bindings.cc_api, quote! { private_module });
        assert_rs_not_matches!(bindings.cc_api_impl, quote! { private_module });
        assert_cc_not_matches!(bindings.cc_api, quote! { pub_func_in_priv_module });
        assert_rs_not_matches!(bindings.cc_api_impl, quote! { pub_func_in_priv_module });
    });
}

#[test]
fn test_generated_bindings_top_level_items() {
    let test_src = "pub fn public_function() {}";
    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        let expected_comment_txt =
            "Automatically @generated C++ bindings for the following Rust crate:\n\
             rust_out\n\
             Features: experimental, supported";
        assert_cc_matches!(
            bindings.cc_api,
            quote! {
                __COMMENT__ #expected_comment_txt
                ...
                __HASH_TOKEN__ pragma once
                ...
                namespace rust_out {
                    ...
                }
            }
        );
        assert_cc_matches!(
            bindings.cc_api_impl,
            quote! {
                __COMMENT__ #expected_comment_txt
            }
        );
    })
}

#[test]
fn test_format_item_reexport_private_type() {
    let test_src = r#"
        #![allow(dead_code)]
        mod test_mod {
            pub struct ReExportedStruct{
                pub field: i32
            }
            pub struct NotReExportedStruct{
                pub field: i32
            }
        }

        pub use crate::test_mod::ReExportedStruct as Z;
        pub use crate::test_mod::ReExportedStruct as X;
        pub use crate::test_mod::ReExportedStruct as Y;
        #[allow(unused_imports)]
        use crate::test_mod::ReExportedStruct as PrivateUse;
        "#;
    test_format_item(test_src, "NotReExportedStruct", |result| {
        let result = result.unwrap();
        assert!(result.is_none());
    });

    test_format_item(test_src, "PrivateUse", |result| {
        let result = result.unwrap();
        assert!(result.is_none());
    });

    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        assert_cc_matches!(
            bindings.cc_api,
            quote! {
                ...
                namespace __crubit_internal {
                ...
                struct CRUBIT_INTERNAL_RUST_TYPE(":: rust_out :: X") alignas(4)
                [[clang::trivial_abi]] ReExportedStruct final
                ...
                }
            }
        );

        assert_rs_matches!(
            bindings.cc_api_impl,
            quote! {
                const _: () = assert!(::std::mem::size_of::<::rust_out::X>() == 4);
            }
        );

        assert_rs_not_matches!(bindings.cc_api_impl, quote! { ::rust_out::Y });
        assert_rs_not_matches!(bindings.cc_api_impl, quote! { ::rust_out::Z });
    });
}

#[test]
fn test_generated_bindings_module_deprecated_no_args() {
    let test_src = r#"
            #[deprecated]
            pub mod some_module {
                pub fn some_function() {}
            }
        "#;
    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        assert_cc_matches!(
            bindings.cc_api,
            quote! {
                ...
                    namespace [[deprecated]] some_module {
                        ...
                    }  // namespace some_module
                ...
            }
        );
    });
}

#[test]
fn test_generated_bindings_module_deprecated_with_message() {
    let test_src = r#"
            #[deprecated = "Use other_module instead"]
            pub mod some_module {
                pub fn some_function() {}
            }
        "#;
    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        assert_cc_matches!(
            bindings.cc_api,
            quote! {
                ...
                    namespace
                    [[deprecated("Use other_module instead")]]
                    some_module {
                        ...
                    }  // namespace some_module
                ...
            }
        );
    });
}

#[test]
fn test_generated_bindings_module_deprecated_named_args() {
    let test_src = r#"
            #[deprecated(since = "3.14", note = "Use other_module instead")]
            pub mod some_module {
                pub fn some_function() {}
            }
        "#;
    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        assert_cc_matches!(
            bindings.cc_api,
            quote! {
                ...
                    namespace
                    [[deprecated("Use other_module instead")]]
                    some_module {
                        ...
                    }  // namespace some_module
                ...
            }
        );
    });
}

#[test]
fn test_format_bridged_type_pointer_like_errors() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_type=const CppType*"]
            #[doc="CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h"]
            pub struct MissingReprTransparent {
                pub cpp_type: *const core::ffi::c_void,
            }

            #[unsafe(no_mangle)]
            pub fn with_missing_repr_transparent(_: MissingReprTransparent) {}

            #[doc="CRUBIT_ANNOTATE: cpp_type=const CppType*"]
            #[doc="CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h"]
            #[repr(transparent)]
            pub struct NotPointerLike {
                pub value: i32,
            }

            #[unsafe(no_mangle)]
            pub fn not_pointer_like(_: NotPointerLike) {}
    "#;

    test_format_item(test_src, "with_missing_repr_transparent", |result| {
        let err = result.unwrap_err();
        assert_eq!(
            err,
            "Error handling parameter #0 of type `MissingReprTransparent`: Can't convert \
            MissingReprTransparent to a C++ pointer as it's not `repr(transparent)`"
        );
    });

    test_format_item(test_src, "not_pointer_like", |result| {
        let err = result.unwrap_err();
        assert_eq!(
            err,
            "Error handling parameter #0 of type `NotPointerLike`: Can't convert \
            NotPointerLike to a C++ pointer as its layout is not pointer-like. To be \
            considered pointer-like it may only have one non-ZST field that needs to be a C \
            ABI compatible pointer."
        );
    });
}

#[test]
fn test_format_bridged_func_arg_pointer_like() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_type=const CppType*"]
            #[doc="CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h"]
            #[repr(transparent)]
            pub struct RustTypeView {
                pub cpp_type: *const core::ffi::c_void,
            }

            #[unsafe(no_mangle)]
            pub fn foo(_: RustTypeView) {}
    "#;
    test_format_item(test_src, "RustTypeView", |result| {
        let err = result.unwrap_err();
        assert_eq!(
            err,
            "Type bindings for RustTypeView suppressed \
                due to being mapped to an existing C++ type (const CppType*)"
        );
    });

    test_format_item(test_src, "foo", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;

        assert_eq!(main_api.prereqs.includes.len(), 1);

        assert_cc_matches!(
            main_api.tokens,
            quote! {
                void foo(const CppType* __param_0);
            }
        );

        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                namespace __crubit_internal {
                    extern "C" void __crubit_thunk_foo(const CppType*);
                }

                inline void foo(const CppType* __param_0) {
                    return __crubit_internal::__crubit_thunk_foo(__param_0);
                }
            }
        );

        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                #[unsafe(no_mangle)]
                unsafe extern "C" fn __crubit_thunk_foo(__param_0: *const core::ffi::c_void) -> () {
                    unsafe {
                        let __param_0 = {
                            let mut __crubit_temp = ::core::mem::MaybeUninit::<::rust_out::RustTypeView>::uninit();
                            __crubit_temp.write(::core::mem::transmute(__param_0));
                            __crubit_temp.assume_init()
                        };
                        ::rust_out::foo(__param_0)
                    }
                }
            }
        );
    });
}

#[test]
fn test_format_bridged_func_arg_by_pointer() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_type=CppType const*"]
            #[doc="CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h"]
            #[doc="CRUBIT_ANNOTATE: cpp_to_rust_converter=cpp_pointer_to_rust_struct"]
            #[doc="CRUBIT_ANNOTATE: rust_to_cpp_converter=rust_struct_to_cpp_pointer"]
            #[repr(transparent)]
            pub struct RustTypeView {
                pub cpp_type: *const core::ffi::c_void,
            }

            #[unsafe(no_mangle)]
            pub fn foo(_: RustTypeView) {}
    "#;
    test_format_item(test_src, "RustTypeView", |result| {
        let err = result.unwrap_err();
        assert_eq!(
            err,
            "Type bindings for RustTypeView suppressed \
                due to being mapped to an existing C++ type (CppType const*)"
        );
    });

    test_format_item(test_src, "foo", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;

        assert_eq!(main_api.prereqs.includes.len(), 1);

        assert_cc_matches!(
            main_api.tokens,
            quote! {
                void foo(CppType const* __param_0);
            }
        );

        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                namespace __crubit_internal {
                    extern "C" void __crubit_thunk_foo(CppType const*);
                }

                inline void foo(CppType const* __param_0) {
                    return __crubit_internal::__crubit_thunk_foo(__param_0);
                }
            }
        );

        let extern_c_decl = result.rs_details.extern_c_decls.first().unwrap();
        assert_eq!(extern_c_decl.symbol, Symbol::intern("cpp_pointer_to_rust_struct"));
        assert_rs_matches!(
            extern_c_decl.decl,
            quote! {
                fn cpp_pointer_to_rust_struct(cpp_in: *const core::ffi::c_void,
                    rs_out: *mut core::ffi::c_void);
            }
        );

        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                #[unsafe(no_mangle)]
                unsafe extern "C" fn __crubit_thunk_foo(__param_0: *const core::ffi::c_void) -> () {
                    unsafe {
                        let __param_0 = {
                            let mut __crubit_temp = ::core::mem::MaybeUninit::<::rust_out::RustTypeView>::uninit();
                            cpp_pointer_to_rust_struct(
                                __param_0,
                                __crubit_temp.as_mut_ptr() as *mut core::ffi::c_void
                            );
                            __crubit_temp.assume_init()
                        };
                        ::rust_out::foo(__param_0)
                    }
                }
            }
        );
    });
}

#[test]
fn test_format_bridged_func_arg_by_value() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_type=cpp_ns::CppType"]
            #[doc="CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h"]
            #[doc="CRUBIT_ANNOTATE: cpp_to_rust_converter=convert_cpp_to_rust_type"]
            #[doc="CRUBIT_ANNOTATE: rust_to_cpp_converter=convert_rust_to_cpp_type"]
            pub struct RustType {
                pub x: i32,
            }

            #[unsafe(no_mangle)]
            pub fn foo(_a: RustType) {}
    "#;
    test_format_item(test_src, "RustType", |result| {
        let err = result.unwrap_err();
        assert_eq!(
            err,
            "Type bindings for RustType suppressed \
                due to being mapped to an existing C++ type (cpp_ns::CppType)"
        );
    });
    test_format_item(test_src, "foo", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;

        assert_eq!(main_api.prereqs.includes.len(), 1);
        assert_eq!(
            *main_api.prereqs.includes.first().unwrap(),
            CcInclude::user_header("cpp_ns/cpp_type.h".into())
        );

        assert_eq!(result.rs_details.extern_c_decls.len(), 1);

        let extern_c_decl = result.rs_details.extern_c_decls.first().unwrap();
        assert_eq!(extern_c_decl.symbol, Symbol::intern("convert_cpp_to_rust_type"));
        assert_rs_matches!(
            extern_c_decl.decl,
            quote! {
                fn convert_cpp_to_rust_type(cpp_in: *const core::ffi::c_void,
                    rs_out: *mut core::ffi::c_void);
            }
        );

        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                #[unsafe(no_mangle)]
                unsafe extern "C" fn __crubit_thunk_foo(_a: *const core::ffi::c_void) -> () {
                    unsafe {
                        let _a = {
                            let mut __crubit_temp = ::core::mem::MaybeUninit::<::rust_out::RustType>::uninit();
                            convert_cpp_to_rust_type(_a, __crubit_temp.as_mut_ptr() as *mut core::ffi::c_void);
                            __crubit_temp.assume_init()
                        };
                        ::rust_out::foo(_a)
                    }
                }
            }
        );

        assert_cc_matches!(
            main_api.tokens,
            quote! {
                void foo(cpp_ns::CppType _a);
            }
        );

        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                namespace __crubit_internal {
                    extern "C" void __crubit_thunk_foo(cpp_ns::CppType*);
                }

                inline void foo(cpp_ns::CppType _a) {
                    return __crubit_internal::__crubit_thunk_foo(&_a);
                }
            }
        );
    });
}

#[test]
fn test_format_bridged_return_type_pointer_like() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_type=CppType*"]
            #[doc="CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h"]
            #[repr(transparent)]
            pub struct RustTypeOwned {
                pub cpp_type: *mut core::ffi::c_void,
            }

            #[unsafe(no_mangle)]
            pub fn foo() -> RustTypeOwned { todo!() }
    "#;
    test_format_item(test_src, "RustTypeOwned", |result| {
        let err = result.unwrap_err();
        assert_eq!(
            err,
            "Type bindings for RustTypeOwned suppressed \
                due to being mapped to an existing C++ type (CppType*)"
        );
    });
    test_format_item(test_src, "foo", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;

        assert_eq!(main_api.prereqs.includes.len(), 1);

        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                #[unsafe(no_mangle)]
                unsafe extern "C" fn __crubit_thunk_foo(__ret_ptr: *mut core::ffi::c_void) -> () {
                    unsafe {
                        let __rs_return_value = ::rust_out::foo();
                        (__ret_ptr as *mut ::rust_out::RustTypeOwned).write(__rs_return_value);
                    }
                }
            }
        );

        assert_cc_matches!(
            main_api.tokens,
            quote! {
                CppType* foo();
            }
        );

        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                namespace __crubit_internal {
                    extern "C" void __crubit_thunk_foo(CppType** __ret_ptr);
                }

                inline CppType* foo() {
                  union __return_value_crubit_return_union {
                    constexpr __return_value_crubit_return_union() {}
                    ~__return_value_crubit_return_union() { std::destroy_at(&this->val); }
                    CppType* val;
                  } __return_value_ret_val_holder;
                  auto* __return_value_storage = &__return_value_ret_val_holder.val;
                  __crubit_internal::__crubit_thunk_foo(__return_value_storage);
                  return std::move(__return_value_ret_val_holder.val);
                }
            }
        );
    })
}

#[test]
fn test_format_brided_type_deduplicate_extern_c_decls() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_type=CppType*"]
            #[doc="CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h"]
            #[doc="CRUBIT_ANNOTATE: rust_to_cpp_converter=rust_struct_to_cpp_pointer"]
            #[doc="CRUBIT_ANNOTATE: cpp_to_rust_converter=cpp_pointer_to_rust_struct"]
            pub struct RustType1 {
                pub cpp_type: *const core::ffi::c_void,
            }

            #[doc="CRUBIT_ANNOTATE: cpp_type=CppType*"]
            #[doc="CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h"]
            #[doc="CRUBIT_ANNOTATE: rust_to_cpp_converter=rust_struct_to_cpp_pointer"]
            #[doc="CRUBIT_ANNOTATE: cpp_to_rust_converter=cpp_pointer_to_rust_struct"]
            pub struct RustType2 {
                pub cpp_type: *const core::ffi::c_void,
            }

            #[unsafe(no_mangle)]
            pub fn foo(_: RustType1, _: RustType2) { todo!() }
    "#;
    test_format_item(test_src, "foo", |result| {
        let result = result.unwrap().unwrap();

        assert_eq!(result.rs_details.extern_c_decls.len(), 1);
        let extern_c_decl = result.rs_details.extern_c_decls.first().unwrap();
        assert_eq!(extern_c_decl.symbol, Symbol::intern("cpp_pointer_to_rust_struct"));
        assert_rs_matches!(
            extern_c_decl.decl,
            quote! {
                fn cpp_pointer_to_rust_struct(cpp_in: *const core::ffi::c_void,
                    rs_out: *mut core::ffi::c_void);
            }
        );
    });
}

#[test]
fn test_format_bridged_return_type_by_pointer() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_type=CppType*"]
            #[doc="CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h"]
            #[doc="CRUBIT_ANNOTATE: rust_to_cpp_converter=rust_struct_to_cpp_pointer"]
            #[doc="CRUBIT_ANNOTATE: cpp_to_rust_converter=cpp_pointer_to_rust_struct"]
            pub struct RustTypeOwned {
                pub cpp_type: *const core::ffi::c_void,
            }

            #[unsafe(no_mangle)]
            pub fn foo() -> RustTypeOwned { todo!() }
    "#;
    test_format_item(test_src, "RustTypeOwned", |result| {
        let err = result.unwrap_err();
        assert_eq!(
            err,
            "Type bindings for RustTypeOwned suppressed \
                due to being mapped to an existing C++ type (CppType*)"
        );
    });
    test_format_item(test_src, "foo", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;

        assert_eq!(main_api.prereqs.includes.len(), 1);

        let extern_c_decl = result.rs_details.extern_c_decls.first().unwrap();
        assert_eq!(extern_c_decl.symbol, Symbol::intern("rust_struct_to_cpp_pointer"));
        assert_rs_matches!(
            extern_c_decl.decl,
            quote! {
                fn rust_struct_to_cpp_pointer(
                    rs_in: *const core::ffi::c_void,
                    cpp_out: *mut core::ffi::c_void
                );
            }
        );

        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                #[unsafe(no_mangle)]
                unsafe extern "C" fn __crubit_thunk_foo(__ret_ptr: *mut core::ffi::c_void) -> () {
                    unsafe {
                        let __rs_return_value = ::rust_out::foo();
                        rust_struct_to_cpp_pointer(
                            std::ptr::from_ref(&__rs_return_value) as *const core::ffi::c_void,
                            __ret_ptr
                        );
                    }
                }
            }
        );

        assert_cc_matches!(
            main_api.tokens,
            quote! {
                CppType* foo();
            }
        );

        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                namespace __crubit_internal {
                    extern "C" void __crubit_thunk_foo(CppType** __ret_ptr);
                }

                inline CppType* foo() {
                    union __return_value_crubit_return_union {
                      constexpr __return_value_crubit_return_union() {}
                      ~__return_value_crubit_return_union() { std::destroy_at(&this->val); }
                      CppType* val;
                    } __return_value_ret_val_holder;
                    auto* __return_value_storage = &__return_value_ret_val_holder.val;
                    __crubit_internal::__crubit_thunk_foo(__return_value_storage);
                    return std::move(__return_value_ret_val_holder.val);
                }
            }
        );
    })
}

#[test]
fn test_format_bridged_return_type_by_value() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_type=cpp_ns::CppType"]
            #[doc="CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h"]
            #[doc="CRUBIT_ANNOTATE: rust_to_cpp_converter=rust_to_cpp_converter"]
            #[doc="CRUBIT_ANNOTATE: cpp_to_rust_converter=cpp_to_rust_converter"]
            pub struct RustType {
                pub x: i32,
            }

            #[unsafe(no_mangle)]
            pub fn foo() -> RustType {
                RustType { x: 10 }
            }
    "#;
    test_format_item(test_src, "RustType", |result| {
        let err = result.unwrap_err();
        assert_eq!(
            err,
            "Type bindings for RustType suppressed \
                due to being mapped to an existing C++ type (cpp_ns::CppType)"
        );
    });
    test_format_item(test_src, "foo", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;

        assert_eq!(main_api.prereqs.includes.len(), 1);
        assert_eq!(
            *main_api.prereqs.includes.first().unwrap(),
            CcInclude::user_header("cpp_ns/cpp_type.h".into())
        );

        let extern_c_decl = result.rs_details.extern_c_decls.first().unwrap();
        assert_eq!(extern_c_decl.symbol, Symbol::intern("rust_to_cpp_converter"));
        assert_rs_matches!(
            extern_c_decl.decl,
            quote! {
                fn rust_to_cpp_converter(rs_in: *const core::ffi::c_void,
                    cpp_out: *mut core::ffi::c_void);
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                #[unsafe(no_mangle)]
                unsafe extern "C" fn __crubit_thunk_foo(__ret_ptr: *mut core::ffi::c_void) -> () {
                    unsafe {
                        let __rs_return_value = ::rust_out::foo();
                        rust_to_cpp_converter(
                            std::ptr::from_ref(&__rs_return_value) as *const core::ffi::c_void,
                            __ret_ptr
                        );
                    }
                }
            }
        );

        assert_cc_matches!(
            main_api.tokens,
            quote! {
                cpp_ns::CppType foo();
            }
        );

        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                namespace __crubit_internal {
                    extern "C" void __crubit_thunk_foo(cpp_ns::CppType* __ret_ptr);
                }

                inline cpp_ns::CppType foo() {
                    union __return_value_crubit_return_union {
                      constexpr __return_value_crubit_return_union() {}
                      ~__return_value_crubit_return_union() { std::destroy_at(&this->val); }
                      cpp_ns::CppType val;
                    } __return_value_ret_val_holder;
                    auto* __return_value_storage = &__return_value_ret_val_holder.val;
                    __crubit_internal::__crubit_thunk_foo(__return_value_storage);
                    return std::move(__return_value_ret_val_holder.val);
                }
            }
        );
    })
}

#[test]
fn test_bridged_type_unsupported() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_type=cpp_ns::CppType"]
            #[doc="CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h"]
            #[doc="CRUBIT_ANNOTATE: rust_to_cpp_converter=convert_rust_to_cpp_type"]
            #[doc="CRUBIT_ANNOTATE: cpp_to_rust_converter=convert_cpp_to_rust_type"]
            pub struct RustType {
                pub x: i32,
            }

            #[unsafe(no_mangle)]
            pub fn unsupported_thunk_arg(_: fn() -> RustType) {}

            #[unsafe(no_mangle)]
            pub fn unsupported_return_ref() -> &'static RustType { todo!(); }

            #[unsafe(no_mangle)]
            pub fn unsupported_return_ptr() -> *const RustType { todo!(); }

            #[unsafe(no_mangle)]
            pub fn unsupported_accept_ref<'a>(_: &'a RustType) {}

            #[unsafe(no_mangle)]
            pub fn unsupported_accept_ptr(_: *const RustType) {}
    "#;

    test_format_item(test_src, "unsupported_thunk_arg", |result| {
        let err = result.unwrap_err();
        assert_eq!(
            err,
            "Error handling parameter #0 of type `fn() -> RustType`: Function pointers can't \
            have a thunk: Any calling convention other than `extern \"C\"` requires a thunk"
        );
    });

    test_format_item(test_src, "unsupported_return_ref", |result| {
        let err = result.unwrap_err();
        assert_eq!(
            err,
            "Can't format reference type `&'static RustType` because the referent is a \
            bridged type. Passing bridged types by reference is not supported."
        );
    });

    test_format_item(test_src, "unsupported_return_ptr", |result| {
        let err = result.unwrap_err();
        assert_eq!(
            err,
            "Can't format pointer type `*const RustType` because the pointee is a bridged \
            type. Passing bridged types by pointer is not supported."
        );
    });

    test_format_item(test_src, "unsupported_accept_ref", |result| {
        let err = result.unwrap_err();
        assert_eq!(
            err,
            "Can't format reference type `&'a RustType` because the referent is a \
            bridged type. Passing bridged types by reference is not supported."
        );
    });

    test_format_item(test_src, "unsupported_accept_ptr", |result| {
        let err = result.unwrap_err();
        assert_eq!(
            err,
            "Can't format pointer type `*const RustType` because the pointee is a bridged \
            type. Passing bridged types by pointer is not supported."
        );
    });
}

#[test]
fn test_format_item_slice() {
    let test_src = r#"
            pub fn foo(_a: *const [u32], _b: *const [u8], _c: *mut [i16], _d: *mut [bool]) { todo!() }
        "#;
    test_format_item(test_src, "foo", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert_cc_matches!(
            main_api.tokens,
            quote! {
              void
              foo(
                rs_std::SliceRef<const std::uint32_t> _a,
                rs_std::SliceRef<const std::uint8_t> _b,
                rs_std::SliceRef<std::int16_t> _c,
                rs_std::SliceRef<bool> _d
              );
            }
        );
    });
}

#[test]
fn test_format_item_static_method() {
    let test_src = r#"
            #![allow(dead_code)]

            /// No-op `f32` placeholder is used, because ZSTs are not supported
            /// (b/258259459).
            pub struct Math(f32);

            impl Math {
                pub fn add_i32(x: f32, y: f32) -> f32 {
                    x + y
                }
            }
        "#;
    test_format_item(test_src, "Math", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... Math final {
                    ...
                    public:
                      ...
                      static float add_i32(float x, float y);
                    ...
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                namespace __crubit_internal {
                    extern "C" float ... (float, float);
                }
                inline float Math::add_i32(float x, float y) {
                  return __crubit_internal::...(x, y);
                }
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                #[unsafe(no_mangle)]
                unsafe extern "C" fn ...(x: f32, y: f32) -> f32 {
                    unsafe { ::rust_out::Math::add_i32(x, y) }
                }
            }
        );
    });
}

#[test]
fn test_format_item_static_method_with_generic_type_parameters() {
    let test_src = r#"
            #![allow(dead_code)]

            /// No-op `f32` placeholder is used, because ZSTs are not supported
            /// (b/258259459).
            pub struct SomeStruct(f32);

            impl SomeStruct {
                // To make this testcase distinct / non-overlapping wrt
                // test_format_item_static_method_with_generic_lifetime_parameters
                // `t` is taken by value below.
                pub fn generic_method<T: Clone>(t: T) -> T {
                    t.clone()
                }
            }
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let unsupported_msg = "Error generating bindings for `SomeStruct::generic_method` \
                               defined at <crubit_unittests.rs>;l=12: \
                               Generic functions are not supported yet (b/259749023)";
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... SomeStruct final {
                    ...
                    __COMMENT__ #unsupported_msg
                    ...
                };
                ...
            }
        );
        assert_cc_not_matches!(result.cc_details.tokens, quote! { SomeStruct::generic_method },);
        assert_rs_not_matches!(result.rs_details.tokens, quote! { generic_method },);
    });
}

#[test]
fn test_format_item_static_method_with_generic_lifetime_parameters_at_fn_level() {
    let test_src = r#"
            #![allow(dead_code)]

            /// No-op `f32` placeholder is used, because ZSTs are not supported
            /// (b/258259459).
            pub struct SomeStruct(f32);

            impl SomeStruct {
                pub fn fn_taking_reference<'a>(x: &'a i32) -> i32 { *x }
            }
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... SomeStruct final {
                    ...
                    static std::int32_t fn_taking_reference(
                        std::int32_t const* [[clang::annotate_type("lifetime", "a")]] x);
                    ...
                };
                ...
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                namespace __crubit_internal {
                extern "C" std::int32_t ...(
                    std::int32_t const* [[clang::annotate_type("lifetime", "a")]]);
                }
                inline std::int32_t SomeStruct::fn_taking_reference(
                    std::int32_t const* [[clang::annotate_type("lifetime", "a")]] x) {
                  return __crubit_internal::...(x);
                }
            },
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                #[unsafe(no_mangle)]
                unsafe extern "C" fn ...(x: &'static i32) -> i32 {
                    unsafe { ::rust_out::SomeStruct::fn_taking_reference(x) }
                }
            },
        );
    });
}

#[test]
fn test_format_item_static_method_with_generic_lifetime_parameters_at_impl_level() {
    let test_src = r#"
            #![allow(dead_code)]

            /// No-op `f32` placeholder is used, because ZSTs are not supported
            /// (b/258259459).
            pub struct SomeStruct(f32);

            impl<'a> SomeStruct {
                pub fn fn_taking_reference(x: &'a i32) -> i32 { *x }
            }
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... SomeStruct final {
                    ...
                    static std::int32_t fn_taking_reference(
                        std::int32_t const* [[clang::annotate_type("lifetime", "a")]] x);
                    ...
                };
                ...
            }
        );
        assert_cc_matches!(result.cc_details.tokens, quote! { SomeStruct::fn_taking_reference },);
        assert_rs_matches!(result.rs_details.tokens, quote! { fn_taking_reference },);
    });
}

fn test_format_item_method_taking_self_by_value(test_src: &str) {
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... SomeStruct final {
                    ...
                    float into_f32() &&;
                    ...
                };
                ...
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                namespace __crubit_internal {
                extern "C" float ...(::rust_out::SomeStruct*);
                }
                inline float SomeStruct::into_f32() && {
                  auto&& self = *this;
                  return __crubit_internal::...(&self);
                }
            },
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                ...
                #[unsafe(no_mangle)]
                unsafe extern "C" fn ...(__self: &'static mut ::core::mem::MaybeUninit<::rust_out::SomeStruct>) -> f32 {
                    unsafe {
                        let __self = __self.assume_init_read();
                        ::rust_out::SomeStruct::into_f32(__self)
                    }
                }
                ...
            },
        );
    });
}

#[test]
fn test_format_item_method_taking_self_by_value_implicit_type() {
    let test_src = r#"
            pub struct SomeStruct(pub f32);

            impl SomeStruct {
                pub fn into_f32(self) -> f32 {
                    self.0
                }
            }
        "#;
    test_format_item_method_taking_self_by_value(test_src);
}

/// One difference from
/// `test_format_item_method_taking_self_by_value_implicit_type` is that
/// `fn_sig.decl.implicit_self` is `ImplicitSelfKind::None` here (vs
/// `ImplicitSelfKind::Imm` in the other test).
#[test]
fn test_format_item_method_taking_self_by_value_explicit_type() {
    let test_src = r#"
            pub struct SomeStruct(pub f32);

            impl SomeStruct {
                pub fn into_f32(self: SomeStruct) -> f32 {
                    self.0
                }
            }
        "#;
    test_format_item_method_taking_self_by_value(test_src);
}

fn test_format_item_method_taking_self_by_const_ref(test_src: &str) {
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... SomeStruct final {
                    ...
                    float get_f32() const;
                    ...
                };
                ...
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                namespace __crubit_internal {
                extern "C" float ...(::rust_out::SomeStruct const&);
                }
                inline float SomeStruct::get_f32() const {
                  auto&& self = *this;
                  return __crubit_internal::...(self);
                }
            },
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                #[unsafe(no_mangle)]
                unsafe extern "C" fn ...(__self: &'static ::rust_out::SomeStruct) -> f32 {
                    unsafe { ::rust_out::SomeStruct::get_f32(__self) }
                }
                ...
            },
        );
    });
}

#[test]
fn test_format_item_method_taking_self_by_const_ref_implicit_type() {
    let test_src = r#"
            pub struct SomeStruct(pub f32);

            impl SomeStruct {
                pub fn get_f32(&self) -> f32 {
                    self.0
                }
            }
        "#;
    test_format_item_method_taking_self_by_const_ref(test_src);
}

#[test]
fn test_format_item_method_taking_self_by_const_ref_explicit_type() {
    let test_src = r#"
            pub struct SomeStruct(pub f32);

            impl SomeStruct {
                pub fn get_f32(self: &SomeStruct) -> f32 {
                    self.0
                }
            }
        "#;
    test_format_item_method_taking_self_by_const_ref(test_src);
}

fn test_format_item_method_taking_self_by_mutable_ref(test_src: &str) {
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... SomeStruct final {
                    ...
                    void set_f32(float new_value);
                    ...
                };
                ...
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                namespace __crubit_internal {
                extern "C" void ...(::rust_out::SomeStruct&, float);
                }
                inline void SomeStruct::set_f32(float new_value) {
                  auto&& self = *this;
                  return __crubit_internal::...(self, new_value);
                }
            },
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                #[unsafe(no_mangle)]
                unsafe extern "C" fn ...(
                    __self: &'static mut ::rust_out::SomeStruct,
                    new_value: f32
                ) -> () {
                    unsafe { ::rust_out::SomeStruct::set_f32(__self, new_value) }
                }
                ...
            },
        );
    });
}

#[test]
fn test_format_item_method_taking_self_by_mutable_ref_implicit_type() {
    let test_src = r#"
            pub struct SomeStruct(pub f32);

            impl SomeStruct {
                pub fn set_f32(&mut self, new_value: f32) {
                    self.0 = new_value;
                }
            }
        "#;
    test_format_item_method_taking_self_by_mutable_ref(test_src);
}

#[test]
fn test_format_item_method_taking_self_by_mutable_ref_explicit_type() {
    let test_src = r#"
            pub struct SomeStruct(pub f32);

            impl SomeStruct {
                pub fn set_f32(self: &mut SomeStruct, new_value: f32) {
                    self.0 = new_value;
                }
            }
        "#;
    test_format_item_method_taking_self_by_mutable_ref(test_src);
}

#[test]
fn test_format_item_method_taking_self_by_arc() {
    let test_src = r#"
            use std::sync::Arc;

            pub struct SomeStruct(pub f32);

            impl SomeStruct {
                pub fn get_f32(self: Arc<Self>) -> f32 {
                    self.0
                }
            }
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let unsupported_msg = "Error generating bindings for `SomeStruct::get_f32` \
                               defined at <crubit_unittests.rs>;l=7: \
                               Unsupported `self` type `std::sync::Arc<SomeStruct>`";
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... SomeStruct final {
                    ...
                    __COMMENT__ #unsupported_msg
                    ...
                };
                ...
            }
        );
        assert_cc_not_matches!(result.cc_details.tokens, quote! { SomeStruct::get_f32 },);
        assert_rs_not_matches!(result.rs_details.tokens, quote! { get_f32 },);
    });
}

#[test]
fn test_format_item_method_taking_self_by_pinned_mut_ref() {
    let test_src = r#"
            use core::pin::Pin;

            pub struct SomeStruct(f32);

            impl SomeStruct {
                pub fn set_f32(mut self: Pin<&mut Self>, f: f32) {
                    self.0 = f;
                }
            }
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let unsupported_msg = "Error generating bindings for `SomeStruct::set_f32` \
                               defined at <crubit_unittests.rs>;l=7: \
                               Unsupported `self` type `std::pin::Pin<&'__anon1 mut SomeStruct>`";
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... SomeStruct final {
                    ...
                    __COMMENT__ #unsupported_msg
                    ...
                };
                ...
            }
        );
        assert_cc_not_matches!(result.cc_details.tokens, quote! { SomeStruct::set_f32 },);
        assert_rs_not_matches!(result.rs_details.tokens, quote! { set_f32 },);
    });
}

#[test]
fn test_format_item_struct_with_default_constructor() {
    let test_src = r#"
            #![allow(dead_code)]

            #[derive(Default)]
            pub struct Point(i32, i32);
        "#;
    test_format_item(test_src, "Point", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... Point final {
                    ...
                    public:
                      __COMMENT__ "Default::default"
                      Point();
                    ...
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                namespace __crubit_internal {
                    extern "C" void ...(::rust_out::Point* __ret_ptr);
                }
                inline Point::Point() {
                    ...(this);
                }
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                #[unsafe(no_mangle)]
                unsafe extern "C" fn ...(
                    __ret_ptr: *mut core::ffi::c_void
                ) -> () {
                    unsafe {
                        let __rs_return_value =
                            <::rust_out::Point as ::core::default::Default>::default();
                        (__ret_ptr as *mut ::rust_out::Point).write(__rs_return_value);
                    }
                }
            }
        );
    });
}

#[test]
fn test_format_item_struct_with_copy_trait() {
    let test_src = r#"
            #![allow(dead_code)]

            #[derive(Clone, Copy)]
            pub struct Point(i32, i32);
        "#;
    let msg = "Rust types that are `Copy` get trivial, `default` C++ copy constructor \
               and assignment operator.";
    test_format_item(test_src, "Point", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... Point final {
                    ...
                    public:
                      ...
                      __COMMENT__ #msg
                      Point(const Point&) = default;
                      Point& operator=(const Point&) = default;
                      ...
                };
            }
        );

        // Trivial copy doesn't require any C++ details except `static_assert`s.
        assert_cc_not_matches!(result.cc_details.tokens, quote! { Point::Point(const Point&) },);
        assert_cc_not_matches!(result.cc_details.tokens, quote! { Point::operator=(const Point&) },);
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                static_assert(std::is_trivially_copy_constructible_v<Point>);
                static_assert(std::is_trivially_copy_assignable_v<Point>);
            },
        );

        // Trivial copy doesn't require any Rust details.
        assert_rs_not_matches!(result.rs_details.tokens, quote! { Copy });
        assert_rs_not_matches!(result.rs_details.tokens, quote! { copy });
    });
}

/// Test of `generate_copy_ctor_and_assignment_operator` when the ADT
/// implements a `Clone` trait.
///
/// Notes:
/// * `Copy` trait is covered in `test_format_item_struct_with_copy_trait`.
/// * The test below implements `clone` and uses the default `clone_from`.
#[test]
fn test_format_item_struct_with_clone_trait() {
    let test_src = r#"
            #![allow(dead_code)]

            pub struct Point(i32, i32);
            impl Clone for Point {
                fn clone(&self) -> Self {
                    unimplemented!()
                }
            }
        "#;
    test_format_item(test_src, "Point", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... Point final {
                    ...
                    public:
                      ...
                      __COMMENT__ "Clone::clone"
                      Point(const Point&);

                      __COMMENT__ "Clone::clone_from"
                      Point& operator=(const Point&);
                    ...
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                namespace __crubit_internal {
                extern "C" void ...(::rust_out::Point const&, ::rust_out::Point* __ret_ptr);
                }
                namespace __crubit_internal {
                extern "C" void ...(::rust_out::Point&, ::rust_out::Point const&);
                }
                inline Point::Point(const Point& other) {
                  __crubit_internal::...(other, this);
                }
                inline Point& Point::operator=(const Point& other) {
                  if (this != &other) {
                    __crubit_internal::...(*this, other);
                  }
                  return *this;
                }
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                #[unsafe(no_mangle)]
                unsafe extern "C" fn ...(
                    __self: &'static ::rust_out::Point,
                    __ret_ptr: *mut core::ffi::c_void
                ) -> () {
                    unsafe {
                        let __rs_return_value =
                            <::rust_out::Point as ::core::clone::Clone>::clone(__self);
                        (__ret_ptr as *mut ::rust_out::Point).write(__rs_return_value);
                    }
                }
                #[unsafe(no_mangle)]
                unsafe extern "C" fn ...(
                    __self: &'static mut ::rust_out::Point,
                    source: &'static ::rust_out::Point
                ) -> () {
                    unsafe { <::rust_out::Point as ::core::clone::Clone>::clone_from(__self, source) }
                }
            }
        );
    });
}

fn test_format_item_struct_with_custom_drop_and_no_default_nor_clone_impl(
    test_src: &str,
    pass_by_value_line_number: i32,
) {
    test_format_item(test_src, "TypeUnderTest", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let move_deleted_msg = "C++ moves are deleted \
                                because there's no non-destructive implementation available.";
        let pass_by_value_msg = format!(
            "Error generating bindings for `TypeUnderTest::pass_by_value` \
                    defined at <crubit_unittests.rs>;l={pass_by_value_line_number}: \
             Can't pass a type by value without a move constructor"
        );
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... TypeUnderTest final {
                    ...
                    public:
                      ...
                      __COMMENT__ "Drop::drop"
                      ~TypeUnderTest();

                      __COMMENT__ #move_deleted_msg
                      TypeUnderTest(TypeUnderTest&&) = delete;
                      TypeUnderTest& operator=(TypeUnderTest&&) = delete;
                      ...
                      __COMMENT__ #pass_by_value_msg
                      ...
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                namespace __crubit_internal {
                // `drop` thunk decl
                extern "C" void ...(::rust_out::TypeUnderTest&);
                }
                inline TypeUnderTest::~TypeUnderTest() {
                  __crubit_internal::...(*this);
                }
            }
        );
        assert_cc_not_matches!(result.cc_details.tokens, quote! { pass_by_value });
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                ...
                #[unsafe(no_mangle)]
                extern "C" fn ...(
                    __self: &'static mut ::core::mem::MaybeUninit<::rust_out::TypeUnderTest>
                ) {
                    unsafe { __self.assume_init_drop() };
                }
                ...
            }
        );
        assert_rs_not_matches!(result.rs_details.tokens, quote! { pass_by_value });
    });
}

#[test]
fn test_format_item_struct_with_custom_drop_impl_and_no_default_nor_clone_impl() {
    let test_src = r#"
            pub struct TypeUnderTest {
                pub x: i32,
                pub y: i32,
            }

            impl Drop for TypeUnderTest {
                fn drop(&mut self) {}
            }

            impl TypeUnderTest {
                pub fn return_by_value() -> Self { unimplemented!() }
                pub fn pass_by_value(_: Self) { unimplemented!() }
            }
        "#;
    let pass_by_value_line_number = 13;
    test_format_item_struct_with_custom_drop_and_no_default_nor_clone_impl(
        test_src,
        pass_by_value_line_number,
    );
}

#[test]
fn test_format_item_struct_with_custom_drop_glue_and_no_default_nor_clone_impl() {
    let test_src = r#"
            #![allow(dead_code)]

            // `i32` is present to avoid hitting the ZST checks related to (b/258259459)
            struct StructWithCustomDropImpl(i32);

            impl Drop for StructWithCustomDropImpl {
                fn drop(&mut self) {
                    println!("dropping!");
                }
            }

            pub struct TypeUnderTest {
                field: StructWithCustomDropImpl,
            }

            impl TypeUnderTest {
                pub fn return_by_value() -> Self { unimplemented!() }
                pub fn pass_by_value(_: Self) { unimplemented!() }
            }
        "#;
    let pass_by_value_line_number = 19;
    test_format_item_struct_with_custom_drop_and_no_default_nor_clone_impl(
        test_src,
        pass_by_value_line_number,
    );
}

fn test_format_item_struct_with_custom_drop_and_with_default_impl(test_src: &str) {
    test_format_item(test_src, "TypeUnderTest", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... TypeUnderTest final {
                    ...
                    public:
                      ...
                      __COMMENT__ "Drop::drop"
                      ~TypeUnderTest();
                      TypeUnderTest(TypeUnderTest&&);
                      TypeUnderTest& operator=(
                          TypeUnderTest&&);
                      ...
                      static ::rust_out::TypeUnderTest pass_by_value();
                      ...
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                namespace __crubit_internal {
                // `drop` thunk decl
                extern "C" void ...(::rust_out::TypeUnderTest&);
                }
                inline TypeUnderTest::~TypeUnderTest() {
                  __crubit_internal::...(*this);
                }
                inline TypeUnderTest::TypeUnderTest(
                    TypeUnderTest&& other)
                    : TypeUnderTest() {
                  *this = std::move(other);
                }
                inline TypeUnderTest& TypeUnderTest::operator=(
                    TypeUnderTest&& other) {
                  crubit::MemSwap(*this, other);
                  return *this;
                }
                namespace __crubit_internal {  // `pass_by_value` thunk decl
                extern "C" void ...(::rust_out::TypeUnderTest* __ret_ptr);
                }
                inline ::rust_out::TypeUnderTest TypeUnderTest::pass_by_value() {
                  crubit::Slot<::rust_out::TypeUnderTest> __return_value_ret_val_holder;
                  auto* __return_value_storage = __return_value_ret_val_holder.Get();
                  __crubit_internal::...(__return_value_storage);
                  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
                }
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                ...
                #[unsafe(no_mangle)]
                unsafe extern "C" fn ...(
                    __self: &'static mut ::core::mem::MaybeUninit<::rust_out::TypeUnderTest>
                ) {
                    unsafe { __self.assume_init_drop() };
                }
                #[unsafe(no_mangle)]
                unsafe extern "C" fn ...(
                    __ret_ptr: *mut core::ffi::c_void
                ) -> () {
                    unsafe {
                        let __rs_return_value = ::rust_out::TypeUnderTest::pass_by_value();
                        (__ret_ptr as *mut ::rust_out::TypeUnderTest).write(__rs_return_value);
                    }
                }
                ...
            }
        );
    });
}

#[test]
fn test_format_item_struct_with_custom_drop_impl_and_with_default_impl() {
    let test_src = r#"
            #[derive(Default)]
            pub struct TypeUnderTest {
                pub x: i32,
                pub y: i32,
            }

            impl Drop for TypeUnderTest {
                fn drop(&mut self) {}
            }

            impl TypeUnderTest {
                pub fn pass_by_value() -> Self { unimplemented!() }
            }
        "#;
    test_format_item_struct_with_custom_drop_and_with_default_impl(test_src);
}

#[test]
fn test_format_item_struct_with_custom_drop_glue_and_with_default_impl() {
    let test_src = r#"
            #![allow(dead_code)]

            // `i32` is present to avoid hitting the ZST checks related to (b/258259459)
            #[derive(Default)]
            struct StructWithCustomDropImpl(i32);

            impl Drop for StructWithCustomDropImpl {
                fn drop(&mut self) {
                    println!("dropping!");
                }
            }

            #[derive(Default)]
            pub struct TypeUnderTest {
                field: StructWithCustomDropImpl,
            }

            impl TypeUnderTest {
                pub fn pass_by_value() -> Self { unimplemented!() }
            }
        "#;
    test_format_item_struct_with_custom_drop_and_with_default_impl(test_src);
}

fn test_format_item_struct_with_custom_drop_and_no_default_and_clone(test_src: &str) {
    test_format_item(test_src, "TypeUnderTest", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... TypeUnderTest final {
                    ...
                    public:
                      ...
                      __COMMENT__ "Drop::drop"
                      ~TypeUnderTest();
                      ...
                      static ::rust_out::TypeUnderTest pass_by_value();
                      ...
                };
            }
        );

        // Implicit, but not `=default`-ed move constructor and move assignment
        // operator.
        assert_cc_not_matches!(main_api.tokens, quote! { TypeUnderTest(TypeUnderTest&&) });
        assert_cc_not_matches!(main_api.tokens, quote! { operator=(TypeUnderTest&&) });
        // No definition of a custom move constructor nor move assignment operator.
        assert_cc_not_matches!(result.cc_details.tokens, quote! { TypeUnderTest(TypeUnderTest&&) },);
        assert_cc_not_matches!(result.cc_details.tokens, quote! { operator=(TypeUnderTest&&) },);

        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                namespace __crubit_internal {
                // `drop` thunk decl
                extern "C" void ...(::rust_out::TypeUnderTest&);
                }
                ...
                namespace __crubit_internal {  // `pass_by_value` thunk decl
                extern "C" void ...(::rust_out::TypeUnderTest* __ret_ptr);
                }
                inline ::rust_out::TypeUnderTest TypeUnderTest::pass_by_value() {
                    crubit::Slot<::rust_out::TypeUnderTest> __return_value_ret_val_holder;
                    auto* __return_value_storage = __return_value_ret_val_holder.Get();
                    __crubit_internal::__crubit_thunk_pass_uby_uvalue(__return_value_storage);
                    return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
                }
                ...
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                ...
                #[unsafe(no_mangle)]
                extern "C" fn ...(
                    __self: &'static mut ::core::mem::MaybeUninit<::rust_out::TypeUnderTest>
                ) {
                    unsafe { __self.assume_init_drop() };
                }
                ...
                #[unsafe(no_mangle)]
                unsafe extern "C" fn ...(__ret_ptr: *mut core::ffi::c_void) -> () {
                    unsafe {
                        let __rs_return_value = ::rust_out::TypeUnderTest::pass_by_value();
                        (__ret_ptr as *mut ::rust_out::TypeUnderTest).write(__rs_return_value);
                    }
                }
                ...
            }
        );
    });
}

#[test]
fn test_format_item_struct_with_custom_drop_impl_and_no_default_and_clone() {
    let test_src = r#"
            #[derive(Clone)]
            pub struct TypeUnderTest {
                pub x: i32,
                pub y: i32,
            }

            impl Drop for TypeUnderTest {
                fn drop(&mut self) {}
            }

            impl TypeUnderTest {
                pub fn pass_by_value() -> Self { unimplemented!() }
            }
        "#;
    test_format_item_struct_with_custom_drop_and_no_default_and_clone(test_src);
}

#[test]
fn test_format_item_struct_with_custom_drop_glue_and_no_default_and_clone() {
    let test_src = r#"
            #![allow(dead_code)]

            // `i32` is present to avoid hitting the ZST checks related to (b/258259459)
            #[derive(Clone)]
            struct StructWithCustomDropImpl(i32);

            impl Drop for StructWithCustomDropImpl {
                fn drop(&mut self) {
                    println!("dropping!");
                }
            }

            #[derive(Clone)]
            pub struct TypeUnderTest {
                field: StructWithCustomDropImpl,
            }

            impl TypeUnderTest {
                pub fn pass_by_value() -> Self { unimplemented!() }
            }
        "#;
    test_format_item_struct_with_custom_drop_and_no_default_and_clone(test_src);
}

#[test]
fn test_format_item_unsupported_struct_with_custom_drop_and_default_and_nonunpin() {
    let test_src = r#"
            #![feature(negative_impls)]

            #[derive(Default)]
            pub struct SomeStruct {
                pub x: i32,
                pub y: i32,
            }

            impl !Unpin for SomeStruct {}

            impl Drop for SomeStruct {
                fn drop(&mut self) {}
            }

            impl SomeStruct {
                pub fn return_by_value() -> Self { unimplemented!() }
                pub fn pass_by_value(_: Self) { unimplemented!() }
            }
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let move_deleted_msg = "C++ moves are deleted \
                                because there's no non-destructive implementation available.";
        let pass_by_value_msg = "Error generating bindings for `SomeStruct::pass_by_value` \
                    defined at <crubit_unittests.rs>;l=18: \
             Can't pass a type by value without a move constructor";
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... SomeStruct final {
                    ...
                    public:
                      ...
                      __COMMENT__ "Default::default"
                      SomeStruct();

                      __COMMENT__ "Drop::drop"
                      ~SomeStruct();

                      __COMMENT__ #move_deleted_msg
                      SomeStruct(SomeStruct&&) = delete;
                      SomeStruct& operator=(SomeStruct&&) = delete;
                      ...
                      static ::rust_out::SomeStruct return_by_value();
                      __COMMENT__ #pass_by_value_msg
                      ...
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                ...
                namespace __crubit_internal {
                 // `default` thunk decl
                extern "C" void ...(::rust_out::SomeStruct* __ret_ptr);
                }
                inline SomeStruct::SomeStruct() {
                  __crubit_internal::...(this);
                }
                namespace __crubit_internal {
                // `drop` thunk decl
                extern "C" void ...(::rust_out::SomeStruct&);
                }
                inline SomeStruct::~SomeStruct() {
                  __crubit_internal::...(*this);
                }
                ...
            }
        );
        assert_cc_not_matches!(result.cc_details.tokens, quote! { pass_by_value });
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                ...
                #[unsafe(no_mangle)]
                unsafe extern "C" fn ...(
                    __ret_ptr: *mut core::ffi::c_void
                ) -> () {
                    unsafe {
                        let __rs_return_value = <::rust_out::SomeStruct as ::core::default::Default>::default();
                        (__ret_ptr as *mut ::rust_out::SomeStruct).write(__rs_return_value);
                    }
                }
                #[unsafe(no_mangle)]
                extern "C" fn ...(
                    __self: &'static mut ::core::mem::MaybeUninit<::rust_out::SomeStruct>
                ) {
                    unsafe { __self.assume_init_drop() };
                }
                ...
            }
        );
        assert_rs_not_matches!(result.rs_details.tokens, quote! { pass_by_value });
    });
}

#[test]
fn test_format_item_source_loc_macro_rules() {
    let test_src = r#"
        #![allow(dead_code)]

        macro_rules! some_tuple_struct_macro_for_testing_source_loc {
            () => {
                /// Some doc on SomeTupleStructMacroForTesingSourceLoc.
                pub struct SomeTupleStructMacroForTesingSourceLoc(i32);
            };
        }

        some_tuple_struct_macro_for_testing_source_loc!();
    "#;
    test_format_item(test_src, "SomeTupleStructMacroForTesingSourceLoc", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let source_loc_comment = " Some doc on SomeTupleStructMacroForTesingSourceLoc.\n\n\
                                  Generated from: <crubit_unittests.rs>;l=7";
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                __COMMENT__ #source_loc_comment
                struct ... SomeTupleStructMacroForTesingSourceLoc final {
                    ...
                }
                ...
            },
        );
    });
}

#[test]
fn test_format_item_source_loc_with_no_doc_comment() {
    let test_src = r#"
        #![allow(dead_code)]

        pub struct SomeTupleStructWithNoDocComment(i32);
    "#;
    test_format_item(test_src, "SomeTupleStructWithNoDocComment", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let comment = "Generated from: <crubit_unittests.rs>;l=4";
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                __COMMENT__ #comment
                struct ... SomeTupleStructWithNoDocComment final {
                    ...
                }
                ...
            },
        );
    });
}

#[test]
fn test_format_item_unsupported_static_value() {
    let test_src = r#"
            #[unsafe(no_mangle)]
            pub static STATIC_VALUE: i32 = 42;
        "#;
    test_format_item(test_src, "STATIC_VALUE", |result| {
        let err = result.unwrap_err();
        assert_eq!(err, "Unsupported rustc_hir::hir::DefKind: Static { safety: Safe, mutability: Not, nested: false }");
    });
}

#[test]
fn test_format_item_use_normal_type() {
    let test_src = r#"
        pub mod test_mod {
            pub struct S{
                pub field: i32
            }
        }

        pub use test_mod::S as G;
        "#;
    test_format_item(test_src, "G", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                using G CRUBIT_INTERNAL_RUST_TYPE(":: rust_out :: test_mod :: S") = ::rust_out::test_mod::S;
            }
        );
    });
}

#[test]
fn test_generate_bindings_use_list_items() {
    let test_src = r#"
        pub mod test_mod {
            pub struct X{
                pub field: i32
            }
            pub struct Y{
                pub field: i32
            }
        }

        pub use test_mod::{X, Y};
        "#;

    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        assert_cc_matches!(
            bindings.cc_api,
            quote! {
                using X CRUBIT_INTERNAL_RUST_TYPE(":: rust_out :: test_mod :: X") = ::rust_out::test_mod::X;
                using Y CRUBIT_INTERNAL_RUST_TYPE(":: rust_out :: test_mod :: Y") = ::rust_out::test_mod::Y;
            }
        );
    });
}

#[test]
fn test_generate_bindings_use_glob() {
    let test_src = r#"
        pub mod test_mod {
            pub struct X{
                pub field: i32
            }
            pub struct Y{
                pub field: i32
            }
        }

        pub use test_mod::*;
        "#;

    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        assert_cc_matches!(
            bindings.cc_api,
            quote! {
                using X CRUBIT_INTERNAL_RUST_TYPE (":: rust_out :: test_mod :: X") = ::rust_out::test_mod::X;
                using Y CRUBIT_INTERNAL_RUST_TYPE (":: rust_out :: test_mod :: Y") = ::rust_out::test_mod::Y;
            }
        );
    });
}

#[test]
fn test_format_item_type_alias() {
    let test_src = r#"
            pub type TypeAlias = i32;
        "#;
    test_format_item(test_src, "TypeAlias", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                using TypeAlias CRUBIT_INTERNAL_RUST_TYPE(":: rust_out :: TypeAlias") = std::int32_t;
            }
        );
    });
}

#[test]
fn test_format_item_type_alias_should_give_underlying_type() {
    let test_src = r#"
            pub type TypeAlias1 = i32;
            pub type TypeAlias2 = TypeAlias1;
        "#;
    test_format_item(test_src, "TypeAlias2", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                using TypeAlias2 CRUBIT_INTERNAL_RUST_TYPE(":: rust_out :: TypeAlias2") = std::int32_t;
            }
        );
    });
}

#[test]
fn test_format_item_private_type_alias_wont_generate_bindings() {
    let test_src = r#"
        #[allow(dead_code)]
        type TypeAlias = i32;
        "#;
    test_format_item(test_src, "TypeAlias", |result| {
        let result = result.unwrap();
        assert!(result.is_none());
    });
}

#[test]
fn test_format_item_pub_type_alias_on_private_type_wont_generate_bindings() {
    let test_src = r#"
        #![allow(private_interfaces)]
        struct SomeStruct;
        pub type TypeAlias = SomeStruct;
        "#;
    test_format_item(test_src, "TypeAlias", |result| {
        let err = result.unwrap_err();
        assert_eq!(err, "Not a public or a supported reexported type (b/262052635).");
    });
}

#[test]
fn test_format_item_unsupported_generic_type_alias() {
    let test_src = r#"
        pub type TypeAlias<T> = T;
        "#;
    test_format_item(test_src, "TypeAlias", |result| {
        let err = result.unwrap_err();
        assert_eq!(err, "The following Rust type is not supported yet: T");
    });
}

#[test]
fn test_format_item_unsupported_type_without_direct_existence() {
    let test_src = r#"
        pub trait Evil {
            type Type;
        }

        const _ : () = {
            pub struct NamelessType;
            impl Evil for i64 {
                type Type = NamelessType;
            }
        };
        pub type EvilAlias = <i64 as Evil>::Type;
        "#;
    test_format_item(test_src, "EvilAlias", |result| {
        let err = result.unwrap_err();
        assert_eq!(err, "The following Rust type is not supported yet: <i64 as Evil>::Type");
    });
}

#[test]
fn test_format_item_type_alias_deprecated() {
    let test_src = r#"
            #[deprecated = "Use `OtherTypeAlias` instead"]
            pub type TypeAlias = i32;
        "#;
    test_format_item(test_src, "TypeAlias", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                using TypeAlias
                    CRUBIT_INTERNAL_RUST_TYPE(":: rust_out :: TypeAlias")
                    [[deprecated("Use `OtherTypeAlias` instead")]]
                    = std::int32_t;
            }
        );
    });
}

#[test]
fn test_format_item_generate_bindings_for_top_level_type_alias() {
    let test_src = r#"
        #![feature(inherent_associated_types)]
        #![allow(incomplete_features)]
        #![allow(dead_code)]
        pub struct Evil {
            dumb: i32,
        }

        impl Evil {
            pub type Type = i64;
        }
        pub type EvilAlias = Evil::Type;
    "#;
    test_format_item(test_src, "Evil", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_not_matches!(
            main_api.tokens,
            quote! {
                std::int64_t
            }
        );
    });
}

#[test]
fn test_format_namespace_bound_cc_tokens() {
    run_compiler_for_testing("", |tcx| {
        let db = bindings_db_for_tests(tcx);
        let top_level = NamespaceQualifier::new::<&str>([]);
        let m1 = NamespaceQualifier::new(["m1"]);
        let m2 = NamespaceQualifier::new(["m2"]);
        let input = [
            (None, top_level.clone(), quote! { void f0a(); }),
            (None, m1.clone(), quote! { void f1a(); }),
            (None, m1.clone(), quote! { void f1b(); }),
            (None, top_level.clone(), quote! { void f0b(); }),
            (None, top_level.clone(), quote! { void f0c(); }),
            (None, m2.clone(), quote! { void f2a(); }),
            (None, m1.clone(), quote! { void f1c(); }),
            (None, m1.clone(), quote! { void f1d(); }),
        ];
        assert_cc_matches!(
            format_namespace_bound_cc_tokens(&db, input, tcx),
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
    });
}

#[test]
fn test_multiple_attributes() {
    let test_src = r#"
    #[must_use = "Must use"]
    #[deprecated = "Deprecated"]
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }"#;

    test_format_item(test_src, "add", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                [[nodiscard("Must use")]] [[deprecated("Deprecated")]] std::int32_t add(std::int32_t x, std::int32_t y);
                    ...
            }
        )
    })
}

#[test]
fn test_repr_c_union_fields_impl_clone() {
    let test_src = r#"
    #[repr(C)]
    pub union SomeUnion {
        pub x: u32,
    }

    impl Clone for SomeUnion {
        fn clone(&self) -> SomeUnion {
            return SomeUnion {x: 1}
        }
    }

    const _: () = assert!(std::mem::size_of::<SomeUnion>() == 4);
    const _: () = assert!(std::mem::align_of::<SomeUnion>() == 4);
    "#;

    test_format_item(test_src, "SomeUnion", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                union CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] SomeUnion final {
                    public:
                        ...
                        __COMMENT__ "Clone::clone"
                        SomeUnion(const SomeUnion&);

                        __COMMENT__ "Clone::clone_from"
                        SomeUnion& operator=(const SomeUnion&);
                    ...
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                ...
                static_assert(std::is_trivially_destructible_v<SomeUnion>);
                static_assert(std::is_trivially_move_constructible_v<SomeUnion>);
                static_assert(std::is_trivially_move_assignable_v<SomeUnion>);
                ...
                inline SomeUnion::SomeUnion(const SomeUnion& other) {...}
                inline SomeUnion& SomeUnion::operator=(const SomeUnion& other) {...}
                ...
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                #[unsafe(no_mangle)]
                unsafe extern "C" fn ...(
                    __self: &'static ::rust_out::SomeUnion,
                    __ret_ptr: *mut core::ffi::c_void
                ) -> () {
                    unsafe {
                        let __rs_return_value = <::rust_out::SomeUnion as ::core::clone::Clone>::clone(__self);
                        (__ret_ptr as *mut ::rust_out::SomeUnion).write(__rs_return_value);
                    }
                }
                #[unsafe(no_mangle)]
                unsafe extern "C" fn ...(
                    __self: &'static mut ::rust_out::SomeUnion,
                    source: &'static ::rust_out::SomeUnion
                ) -> () {
                    unsafe { <::rust_out::SomeUnion as ::core::clone::Clone>::clone_from(__self, source) }
                }
            }
        );
    })
}

#[test]
fn test_repr_c_union_fields_impl_drop() {
    let test_src = r#"
    #[repr(C)]
    pub union SomeUnion {
        pub x: u32,
    }

    impl Drop for SomeUnion {
        fn drop(&mut self) {
            println!(":)")
        }
    }

    const _: () = assert!(std::mem::size_of::<SomeUnion>() == 4);
    const _: () = assert!(std::mem::align_of::<SomeUnion>() == 4);
    "#;

    test_format_item(test_src, "SomeUnion", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                union CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] SomeUnion final {
                    public:
                        ...
                        __COMMENT__ "Drop::drop"
                        ~SomeUnion();

                        ...
                        SomeUnion(SomeUnion&&) = delete;
                        SomeUnion& operator=(SomeUnion&&) = delete;
                        ...
                    ...
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                ...
                inline SomeUnion::~SomeUnion() {...}
                ...
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                ...
                extern "C" fn ... (__self: &'static mut ::core::mem::MaybeUninit<::rust_out::SomeUnion>...) { unsafe { __self.assume_init_drop() }; }
                ...
            }
        );
    })
}

#[test]
fn test_repr_c_enum_drop() {
    let test_src = r#"
    #[repr(C, i32)]
    pub enum SomeEnum {
        A(i32),
        B{x: u32},
        C,
        D{foo: i32, bar: i32} = 3,
    }

    impl Drop for SomeEnum {
        fn drop(&mut self) {
            println!(":)")
        }
    }

    const _: () = assert!(std::mem::size_of::<SomeEnum>() == 12);
    const _: () = assert!(std::mem::align_of::<SomeEnum>() == 4);
    "#;

    test_format_item(test_src, "SomeEnum", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct CRUBIT_INTERNAL_RUST_TYPE(...) ... [[clang::trivial_abi]] SomeEnum final {
                    public:
                        ...
                        __COMMENT__ "Drop::drop"
                        ~SomeEnum();

                        ...
                        SomeEnum(SomeEnum&&) = delete;
                        SomeEnum& operator=(SomeEnum&&) = delete;
                        ...
                    ...
                };
            }
        );
    })
}

#[test]
fn test_repr_c_enum_clone() {
    let test_src = r#"
    #[repr(C, i32)]
    pub enum SomeEnum {
        A(i32),
        B{x: u32},
        C,
        D{foo: i32, bar: i32} = 3,
    }

    impl Clone for SomeEnum {
        fn clone(&self) -> SomeEnum {
            return SomeEnum::A(1)
        }
    }

    const _: () = assert!(std::mem::size_of::<SomeEnum>() == 12);
    const _: () = assert!(std::mem::align_of::<SomeEnum>() == 4);
    "#;

    test_format_item(test_src, "SomeEnum", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct CRUBIT_INTERNAL_RUST_TYPE(...) ... [[clang::trivial_abi]] SomeEnum final {
                    public:
                        ...
                        __COMMENT__ "Clone::clone"
                        SomeEnum(const SomeEnum&);

                        __COMMENT__ "Clone::clone_from"
                        SomeEnum& operator=(const SomeEnum&);
                    ...
                };
            }
        );
    })
}

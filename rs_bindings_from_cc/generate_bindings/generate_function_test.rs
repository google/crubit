// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Result;
use code_gen_utils::make_rs_ident;
use database::code_snippet::BindingsTokens;
use database::rs_snippet::{format_generic_params, Lifetime};
use generate_function_thunk::thunk_ident;
use googletest::prelude::{assert_that, contains_substring, gtest, OrFail as _};
use ir::{Func, Item, UnqualifiedIdentifier};
use ir_testing::{retrieve_func, with_lifetime_macros};
use multiplatform_ir_testing::{ir_from_cc, ir_from_cc_dependency};
use quote::quote;
use test_generators::generate_bindings_tokens_for_test;
use token_stream_matchers::{
    assert_cc_matches, assert_cc_not_matches, assert_rs_matches, assert_rs_not_matches,
};
use token_stream_printer::rs_tokens_to_formatted_string_for_tests;

#[gtest]
fn test_simple_function() -> Result<()> {
    let ir = ir_from_cc("int Add(int a, int b);")?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[inline(always)]
            pub fn Add(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int {
                unsafe { crate::detail::__rust_thunk___Z3Addii(a, b) }
            }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            mod detail {
                #[allow(unused_imports)]
                use super::*;
                unsafe extern "C" {
                    #[link_name = "_Z3Addii"]
                    pub(crate) unsafe fn __rust_thunk___Z3Addii(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int;
                }
            }
        }
    );

    assert_cc_not_matches!(rs_api_impl, quote! {__rust_thunk___Z3Addii});

    Ok(())
}

#[gtest]
fn test_inline_function() -> Result<()> {
    let ir = ir_from_cc("inline int Add(int a, int b);")?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[inline(always)]
            pub fn Add(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int {
                unsafe { crate::detail::__rust_thunk___Z3Addii(a, b) }
            }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            mod detail {
                #[allow(unused_imports)]
                use super::*;
                unsafe extern "C" {
                    pub(crate) unsafe fn __rust_thunk___Z3Addii(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int;
                }
            }
        }
    );

    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" int __rust_thunk___Z3Addii(int a, int b) {
                return Add(a, b);
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_simple_function_with_types_from_other_target() -> Result<()> {
    let ir = ir_from_cc_dependency(
        "inline ReturnStruct DoSomething(ParamStruct param);",
        "struct ReturnStruct final {}; struct ParamStruct final {};",
    )?;

    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[inline(always)]
            pub fn DoSomething(mut param: dependency::ParamStruct) -> dependency::ReturnStruct {
                unsafe {
                    let mut __return = ::core::mem::MaybeUninit::<dependency::ReturnStruct>::uninit();
                    crate::detail::__rust_thunk___Z11DoSomething11ParamStruct(
                       &raw mut __return as *mut ::core::ffi::c_void,
                       &mut param
                    );
                    __return.assume_init()
                }
            }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
        mod detail {
            #[allow(unused_imports)]
            use super::*;
            unsafe extern "C" {
                pub(crate) unsafe fn __rust_thunk___Z11DoSomething11ParamStruct(
                    __return: *mut ::core::ffi::c_void,
                    param: &mut dependency::ParamStruct
                );
            }
        }}
    );

    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___Z11DoSomething11ParamStruct(
                    struct ReturnStruct* __return, struct ParamStruct* param) {
                new (__return) auto(DoSomething(std::move(*param)));
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_ref_to_struct_in_thunk_impls() -> Result<()> {
    let ir = ir_from_cc("struct S{}; inline void foo(S& s) {} ")?;
    let rs_api_impl = generate_bindings_tokens_for_test(ir)?.rs_api_impl;
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___Z3fooR1S(struct S* s) {
                foo(*s);
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_const_ref_to_struct_in_thunk_impls() -> Result<()> {
    let ir = ir_from_cc("struct S{}; inline void foo(const S& s) {} ")?;
    let rs_api_impl = generate_bindings_tokens_for_test(ir)?.rs_api_impl;
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___Z3fooRK1S(const struct S* s) {
                foo(*s);
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_unsigned_int_in_thunk_impls() -> Result<()> {
    let ir = ir_from_cc("inline void foo(unsigned int i) {} ")?;
    let rs_api_impl = generate_bindings_tokens_for_test(ir)?.rs_api_impl;
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___Z3fooj(unsigned int i) {
                foo(i);
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_record_static_methods_qualify_call_in_thunk() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        struct SomeStruct {
            static inline int some_func() { return 42; }
        }; "#,
    )?;

    assert_cc_matches!(
        generate_bindings_tokens_for_test(ir)?.rs_api_impl,
        quote! {
            extern "C" int __rust_thunk___ZN10SomeStruct9some_funcEv() {
                return SomeStruct::some_func();
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_record_instance_methods_deref_this_in_thunk() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        struct SomeStruct {
            inline int some_func(int arg) const { return 42 + arg; }
        }; "#,
    )?;

    assert_cc_matches!(
        generate_bindings_tokens_for_test(ir)?.rs_api_impl,
        quote! {
            extern "C" int __rust_thunk___ZNK10SomeStruct9some_funcEi(
                    const struct SomeStruct* __this, int arg) {
                return __this->some_func(arg);
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_ptr_func() -> Result<()> {
    let ir = ir_from_cc(r#" inline int* Deref(int*const* p); "#)?;

    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[inline(always)]
            pub unsafe fn Deref(p: *const *mut ::core::ffi::c_int) -> *mut ::core::ffi::c_int {
                crate::detail::__rust_thunk___Z5DerefPKPi(p)
            }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            mod detail {
                #[allow(unused_imports)]
                use super::*;
                unsafe extern "C" {
                    pub(crate) unsafe fn __rust_thunk___Z5DerefPKPi(p: *const *mut ::core::ffi::c_int) -> *mut ::core::ffi::c_int;
                }
            }
        }
    );

    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" int* __rust_thunk___Z5DerefPKPi(int* const * p) {
                return Deref(p);
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_const_char_ptr_func() -> Result<()> {
    // This is a regression test: We used to include the "const" in the name
    // of the CcType, which caused a panic in the code generator
    // ('"const char" is not a valid Ident').
    // It's therefore important that f() is inline so that we need to
    // generate a thunk for it (where we then process the CcType).
    let ir = ir_from_cc(r#" inline void f(const signed char *str); "#)?;

    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[inline(always)]
            pub unsafe fn f(str: *const ::core::ffi::c_schar) {
                crate::detail::__rust_thunk___Z1fPKa(str)
            }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            extern "C" {
                pub(crate) unsafe fn __rust_thunk___Z1fPKa(str: *const ::core::ffi::c_schar);
            }
        }
    );

    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___Z1fPKa(signed char const * str){ f(str); }
        }
    );
    Ok(())
}

#[gtest]
fn test_func_ptr_thunk() -> Result<()> {
    // Using an `inline` keyword forces generation of a C++ thunk in
    // `rs_api_impl` (i.e. exercises `format_cpp_type` and similar code).
    let ir = ir_from_cc(
        r#"
        int multiply(int x, int y);
        inline int (*inline_get_pointer_to_function())(int, int) {
            return multiply;
        }
    "#,
    )?;
    let rs_api_impl = generate_bindings_tokens_for_test(ir)?.rs_api_impl;
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" crubit::type_identity_t<int(int , int)>*
            __rust_thunk___Z30inline_get_pointer_to_functionv() {
                return inline_get_pointer_to_function();
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_doc_comment_func() -> Result<()> {
    let ir = ir_from_cc(
        "
    // Doc Comment
    // with two lines
    int func();",
    )?;

    assert_rs_matches!(
        generate_bindings_tokens_for_test(ir)?.rs_api,
        // leading space is intentional so there is a space between /// and the text of the
        // comment
        quote! {
            #[doc = " Doc Comment\n with two lines\n \n Generated from: ir_from_cc_virtual_header.h;l=6"]
            #[inline(always)]
            pub fn func
        }
    );

    Ok(())
}

/// Trivial types (at least those that are mapped to Copy rust types) do not
/// get a Drop impl.
#[gtest]
fn test_impl_drop_trivial() -> Result<()> {
    let ir = ir_from_cc(
        r#"struct Trivial final {
            ~Trivial() = default;
            int x;
        };"#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_not_matches!(rs_api, quote! {impl Drop});
    assert_rs_not_matches!(rs_api, quote! {impl ::ctor::PinnedDrop});
    assert_rs_matches!(rs_api, quote! {pub x: ::core::ffi::c_int});
    assert_cc_not_matches!(rs_api_impl, quote! { std::destroy_at });
    Ok(())
}

#[gtest]
fn test_impl_default_explicitly_defaulted_constructor() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct DefaultedConstructor final {
            DefaultedConstructor() = default;
        };"#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            impl Default for DefaultedConstructor {
                #[inline(always)]
                fn default() -> Self {
                    let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                    unsafe {
                        crate::detail::__rust_thunk___ZN20DefaultedConstructorC1Ev(&raw mut tmp as *mut ::core::ffi::c_void);
                        tmp.assume_init()
                    }
                }
            }
        }
    );
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___ZN20DefaultedConstructorC1Ev(
                    struct DefaultedConstructor* __this) {
                crubit::construct_at(__this);
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_impl_clone_that_propagates_lifetime() -> Result<()> {
    // This test covers the case where a single lifetime applies to 1)
    // the `__this` parameter and 2) other constructor parameters. For
    // example, maybe the newly constructed object needs to have the
    // same lifetime as the constructor's parameter. (This might require
    // annotating the whole C++ struct with a lifetime, so maybe the
    // example below is not fully realistic/accurate...).
    let ir = ir_from_cc(&with_lifetime_macros(
        r#"#pragma clang lifetime_elision
        struct Foo final {
            Foo(const int& $a i) $a;
        };"#,
    ))?;
    let ctor: &Func = ir
        .items()
        .filter_map(|item| match item {
            Item::Func(func) => Some(&**func),
            _ => None,
        })
        .find(|f| {
            matches!(&f.rs_name, UnqualifiedIdentifier::Constructor)
                && f.params
                    .get(1)
                    .map(|p| p.identifier.identifier.as_ref() == "i")
                    .unwrap_or_default()
        })
        .unwrap();
    {
        // Double-check that the test scenario set up above uses the same lifetime
        // for both of the constructor's parameters: `__this` and `i`.
        assert_eq!(ctor.params.len(), 2);
        let this_lifetime = ctor.params[0].type_.variant.as_pointer().unwrap().lifetime.unwrap();
        let i_lifetime = ctor.params[1].type_.variant.as_pointer().unwrap().lifetime.unwrap();
        assert_eq!(i_lifetime, this_lifetime);
    }

    // Before cl/423346348 the generated Rust code would incorrectly look
    // like this (note the mismatched 'a and 'b lifetimes):
    //     fn from<'b>(i: &'a i32) -> Self
    // After this CL, this scenario will result in an explicit error.
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_not_matches!(rs_api, quote! {impl From});
    assert_rs_matches!(rs_api, {
        let txt = "Generated from: ir_from_cc_virtual_header.h;l=34\n\
                       Error while generating bindings for item 'Foo::Foo':\n\
                       The lifetime of `__this` is \
                           unexpectedly also used by another parameter: &'a::core::ffi::c_int";
        quote! { __COMMENT__ #txt }
    });
    Ok(())
}

#[gtest]
fn test_impl_default_non_trivial_struct() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct NonTrivialStructWithConstructors final {
            NonTrivialStructWithConstructors();
            ~NonTrivialStructWithConstructors();  // Non-trivial
        };"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_not_matches!(rs_api, quote! {impl Default});
    Ok(())
}

#[gtest]
fn test_impl_from_for_1_arg_constructor() -> Result<()> {
    for explicit_qualifier in ["", "explicit"] {
        let ir = ir_from_cc(&format!(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {{
                {explicit_qualifier} SomeStruct(int i);  // implicit - no `explicit` keyword
            }};"#,
        ))?;
        let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl From<::core::ffi::c_int> for SomeStruct {
                    #[inline(always)]
                    fn from(i: ::core::ffi::c_int) -> Self {
                        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                        unsafe {
                            crate::detail::__rust_thunk___ZN10SomeStructC1Ei(&raw mut tmp as *mut ::core::ffi::c_void, i);
                            tmp.assume_init()
                        }
                    }
                }
            }
        );
    }
    Ok(())
}

#[gtest]
fn test_impl_from_for_implicit_conversion_from_reference() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct SomeOtherStruct final { int i; };
        struct StructUnderTest final {
            StructUnderTest(const SomeOtherStruct& other);  // implicit - no `explicit` keyword
        };"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    // This is a regression test for b/223800038: We want to ensure that the
    // code says `impl<'b>` (instead of incorrectly declaring that lifetime
    // in `fn from<'b>`).
    assert_rs_matches!(
        rs_api,
        quote! {
            impl<'b> From<&'b crate::SomeOtherStruct> for StructUnderTest {
                #[inline(always)]
                fn from(other: &'b crate::SomeOtherStruct) -> Self {
                    let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                    unsafe {
                        crate::detail::__rust_thunk___ZN15StructUnderTestC1ERK15SomeOtherStruct(
                            &raw mut tmp as *mut ::core::ffi::c_void, other);
                        tmp.assume_init()
                    }
                }
            }
        },
    );
    Ok(())
}

/// Methods with missing lifetimes for `self` should give a useful error
/// message.
#[gtest]
fn test_eq_nolifetime() -> Result<()> {
    // Missing lifetimes currently only causes hard errors for trait impls,
    // not For inherent methods.
    let ir = ir_from_cc("struct SomeStruct{SomeStruct& operator=(const SomeStruct&);};")?;

    let rs_api =
        rs_tokens_to_formatted_string_for_tests(generate_bindings_tokens_for_test(ir)?.rs_api)?;
    assert!(rs_api.contains(
        "// Error while generating bindings for item 'SomeStruct::operator=':\n\
         // `self` has no lifetime. Use lifetime annotations or \
            `#pragma clang lifetime_elision` to create bindings for this function."
    ));
    Ok(())
}

#[gtest]
fn test_impl_eq_for_member_function() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct SomeStruct final {
            inline bool operator==(const SomeStruct& other) const {
                return i == other.i;
            }
            int i;
        };"#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            impl PartialEq for SomeStruct {
                #[inline(always)]
                fn eq<'a, 'b>(&'a self, other: &'b Self) -> bool {
                    unsafe { crate::detail::__rust_thunk___ZNK10SomeStructeqERKS_(self, other) }
                }
            }
        }
    );
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" bool __rust_thunk___ZNK10SomeStructeqERKS_(
                    const struct SomeStruct* __this, const struct SomeStruct* other) {
                return __this->operator==(*other);
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_impl_eq_for_free_function() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        namespace ns {
            struct SomeStruct final { int i; };
        }
        bool operator==(const ns::SomeStruct& lhs, const ns::SomeStruct& rhs) {
            return lhs.i == rhs.i;
        }
        "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            impl PartialEq for crate::ns::SomeStruct {
                #[inline(always)]
                fn eq<'a, 'b>(&'a self, rhs: &'b Self) -> bool {
                    unsafe { crate::detail::__rust_thunk___ZeqRKN2ns10SomeStructES2_(self, rhs) }
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_impl_eq_for_free_function_different_types() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct SomeStruct final { int i; };
        struct SomeOtherStruct final { int i; };
        bool operator==(const SomeStruct& lhs, const SomeOtherStruct& rhs) {
            return lhs.i == rhs.i;
        }"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            impl PartialEq<crate::SomeOtherStruct> for crate::SomeStruct {
                #[inline(always)]
                fn eq<'a, 'b>(&'a self, rhs: &'b crate::SomeOtherStruct) -> bool {
                    unsafe { crate::detail::__rust_thunk___ZeqRK10SomeStructRK15SomeOtherStruct(self, rhs) }
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_impl_eq_for_free_function_by_value() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct SomeStruct final { int i; };
        bool operator==(SomeStruct lhs, SomeStruct rhs) {
            return lhs.i == rhs.i;
        }"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            impl PartialEq for crate::SomeStruct {
                #[inline(always)]
                fn eq(&self, rhs: &Self) -> bool {
                    unsafe {
                        crate::detail::__rust_thunk___Zeq10SomeStructS_(&mut self.clone(), &mut rhs.clone())
                    }
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_impl_lt_for_member_function() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct SomeStruct final {
            inline bool operator==(const SomeStruct& other) const {
                return i == other.i;
            }
            inline bool operator<(const SomeStruct& other) const {
                return i < other.i;
            }
            int i;
        };"#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            impl PartialOrd for SomeStruct {
                #[inline(always)]
                fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    if self == other {
                        return Some(core::cmp::Ordering::Equal);
                    }
                    if self < other {
                        return Some(core::cmp::Ordering::Less);
                    }
                    if other < self {
                        return Some(core::cmp::Ordering::Greater);
                    }
                    None
                }
                #[inline(always)]
                fn lt<'a, 'b>(&'a self, other: &'b Self) -> bool {
                    unsafe { crate::detail::__rust_thunk___ZNK10SomeStructltERKS_(self, other) }
                }
            }
        }
    );
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" bool __rust_thunk___ZNK10SomeStructltERKS_(
                    const struct SomeStruct* __this, const struct SomeStruct* other) {
                return __this->operator<(*other);
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_impl_lt_for_free_function() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct SomeStruct final {
            inline bool operator==(const SomeStruct& other) const {
                return i == other.i;
            }
            int i;
        };
        bool operator<(const SomeStruct& lhs, const SomeStruct& rhs) {
            return lhs.i < rhs.i;
        }"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            impl PartialOrd for crate::SomeStruct {
                #[inline(always)]
                fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    if self == other {
                        return Some(core::cmp::Ordering::Equal);
                    }
                    if self < other {
                        return Some(core::cmp::Ordering::Less);
                    }
                    if other < self {
                        return Some(core::cmp::Ordering::Greater);
                    }
                    None
                }
                #[inline(always)]
                fn lt<'a, 'b>(&'a self, rhs: &'b Self) -> bool {
                    unsafe { crate::detail::__rust_thunk___ZltRK10SomeStructS1_(self, rhs) }
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_impl_lt_for_free_function_by_value() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct SomeStruct final { int i; };
        bool operator==(SomeStruct lhs, SomeStruct rhs) {
            return lhs.i == rhs.i;
        }
        bool operator<(SomeStruct lhs, SomeStruct rhs) {
            return lhs.i < rhs.i;
        }"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            impl PartialOrd for crate::SomeStruct {
                #[inline(always)]
                fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    if self == other {
                        return Some(core::cmp::Ordering::Equal);
                    }
                    if self < other {
                        return Some(core::cmp::Ordering::Less);
                    }
                    if other < self {
                        return Some(core::cmp::Ordering::Greater);
                    }
                    None
                }
                #[inline(always)]
                fn lt(& self, rhs: &Self) -> bool {
                    unsafe { crate::detail::__rust_thunk___Zlt10SomeStructS_(
                            &mut self.clone(), &mut rhs.clone()) }
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_assign() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        #pragma clang lifetime_elision
        struct SomeStruct {
            ~SomeStruct();
            SomeStruct& operator=(const SomeStruct& other);
        };"#,
    )?;
    let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            impl<'b> ::ctor::Assign<&'b Self> for SomeStruct {
                #[inline(always)]
                fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, other: &'b Self) {
                    unsafe {
                        crate::detail::__rust_thunk___ZN10SomeStructaSERKS_(self, other);
                    }
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_assign_nonreference_other() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        #pragma clang lifetime_elision
        struct SomeStruct {
            ~SomeStruct();
            SomeStruct& operator=(int other);
        };"#,
    )?;
    let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            impl<'b> ::ctor::Assign<&'b Self> for SomeStruct {
                #[inline(always)]
                fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
                    unsafe {
                        crate::detail::__rust_thunk___ZN10SomeStructaSERKS_(self, __param_0);
                    }
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_assign_nonreference_return() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        #pragma clang lifetime_elision
        struct SomeStruct {
            ~SomeStruct();
            int operator=(const SomeStruct& other);
        };"#,
    )?;
    let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            impl<'b> ::ctor::Assign<&'b Self> for SomeStruct {
                #[inline(always)]
                fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, other: &'b Self) {
                    unsafe {
                        crate::detail::__rust_thunk___ZN10SomeStructaSERKS_(self, other);
                    }
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_impl_eq_non_const_member_function() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct SomeStruct final {
            bool operator==(const SomeStruct& other) /* no `const` here */;
        };"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_not_matches!(rs_api, quote! {impl PartialEq});
    Ok(())
}

#[gtest]
fn test_impl_lt_different_operands() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct SomeStruct1 final {
            int i;
        };
        struct SomeStruct2 final {
            inline bool operator==(const SomeStruct1& other) const {
                return i == other.i;
            }
            inline bool operator<(const SomeStruct1& other) const {
                return i < other.i;
            };
            int i;
        };"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_not_matches!(rs_api, quote! {impl PartialOrd});
    Ok(())
}

#[gtest]
fn test_impl_lt_non_const_member_function() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct SomeStruct final {
            inline bool operator==(const SomeStruct& other) const {
                return i == other.i;
            }
            int i;
            bool operator<(const SomeStruct& other) /* no `const` here */;
        };"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_not_matches!(rs_api, quote! {impl PartialOrd});
    Ok(())
}

#[gtest]
fn test_impl_lt_rhs_by_value() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct SomeStruct final {
            inline bool operator==(const SomeStruct& other) const {
                return i == other.i;
            }
            int i;
            bool operator<(SomeStruct other) const;
        };"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_not_matches!(rs_api, quote! {impl PartialOrd});
    Ok(())
}

#[gtest]
fn test_impl_lt_missing_eq_impl() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct SomeStruct final {
            inline bool operator<(const SomeStruct& other) const {
                return i < other.i;
            }
            int i;
        };"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_not_matches!(rs_api, quote! {impl PartialOrd});
    Ok(())
}

#[gtest]
fn test_thunk_ident_function() -> Result<()> {
    let ir = ir_from_cc("inline int foo() {}")?;
    let func = retrieve_func(&ir, "foo");
    assert_eq!(thunk_ident(func), make_rs_ident("__rust_thunk___Z3foov"));
    Ok(())
}

#[gtest]
fn test_thunk_ident_special_names() {
    let ir = ir_from_cc("struct Class {};").unwrap();

    let destructor = ir.get_functions_by_name(&UnqualifiedIdentifier::Destructor).next().unwrap();
    assert_eq!(thunk_ident(destructor), make_rs_ident("__rust_thunk___ZN5ClassD1Ev"));

    let default_constructor = ir
        .get_functions_by_name(&UnqualifiedIdentifier::Constructor)
        .find(|f| f.params.len() == 1)
        .unwrap();
    assert_eq!(thunk_ident(default_constructor), make_rs_ident("__rust_thunk___ZN5ClassC1Ev"));
}

#[gtest]
fn test_elided_lifetimes() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
      struct S final {
        int& f(int& i);
      };"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            pub fn f<'a, 'b>(&'a mut self, i: &'b mut ::core::ffi::c_int) -> &'a mut ::core::ffi::c_int { ... }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            pub(crate) unsafe fn __rust_thunk___ZN1S1fERi<'a, 'b>(__this: &'a mut crate::S, i: &'b mut ::core::ffi::c_int)
                -> &'a mut ::core::ffi::c_int;
        }
    );
    Ok(())
}

#[gtest]
fn test_annotated_lifetimes() -> Result<()> {
    let ir = ir_from_cc(&with_lifetime_macros(
        r#"
      int& $a f(int& $a i1, int& $a i2);
      "#,
    ))?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            pub fn f<'a>(i1: &'a mut ::core::ffi::c_int, i2: &'a mut ::core::ffi::c_int) -> &'a mut ::core::ffi::c_int { ... }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            pub(crate) unsafe fn __rust_thunk___Z1fRiS_<'a>(i1: &'a mut ::core::ffi::c_int, i2: &'a mut ::core::ffi::c_int)
                -> &'a mut ::core::ffi::c_int;
        }
    );
    Ok(())
}

#[gtest]
fn test_format_generic_params() -> Result<()> {
    assert!(
        format_generic_params(/* lifetimes= */ &[], std::iter::empty::<syn::Ident>()).is_empty(),
    );

    let idents = ["T1", "T2"].iter().map(|s| make_rs_ident(s));
    assert_rs_matches!(format_generic_params(/* lifetimes= */ &[], idents), quote! { < T1, T2 > });

    let lifetimes = ["a", "b", "_"].iter().map(|s| Lifetime::new(s)).collect::<Vec<_>>();
    assert_rs_matches!(
        format_generic_params(&lifetimes, std::iter::empty::<syn::Ident>()),
        quote! { < 'a, 'b > }
    );

    Ok(())
}

#[gtest]
fn test_overloaded_functions() -> Result<()> {
    // TODO(b/213280424): We don't support creating bindings for overloaded
    // functions yet, except in the case of overloaded constructors with a
    // single parameter.
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision
            void f() {}
            void f(int i) {}
            struct S1 final {
              void f() {}
              void f(int i) {}
            };
            struct S2 final {
              void f();
            };
            struct S3 final {
              S3(int i);
              S3(double d);
            };

            namespace foo { void not_overloaded(); }
            namespace bar { void not_overloaded(); }
        "#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;

    // Cannot overload free functions.
    assert_cc_matches!(rs_api, {
        let txt = "Generated from: ir_from_cc_virtual_header.h;l=4\n\
                       Error while generating bindings for item 'f':\n\
                       Cannot generate bindings for overloaded function";
        quote! { __COMMENT__ #txt }
    });
    assert_rs_not_matches!(rs_api, quote! {pub fn f()});
    assert_rs_not_matches!(rs_api, quote! {pub fn f(i: ::core::ffi::c_int)});

    assert_cc_matches!(rs_api, {
        let txt = "Generated from: ir_from_cc_virtual_header.h;l=7\n\
                       Error while generating bindings for item 'S1::f':\n\
                       Cannot generate bindings for overloaded function";
        quote! { __COMMENT__ #txt }
    });
    assert_rs_not_matches!(rs_api, quote! {pub fn f(... S1 ...)});

    // And thunks aren't generated for either.
    assert_cc_not_matches!(rs_api_impl, quote! {f});

    // But we can import member functions that have the same name as a free
    // function.
    assert_rs_matches!(rs_api, quote! {pub fn f<'a>(&'a mut self)});

    // We can also import overloaded single-parameter constructors.
    assert_rs_matches!(rs_api, quote! {impl From<::core::ffi::c_int> for S3});
    assert_rs_matches!(rs_api, quote! {impl From<f64> for S3});

    // And we can import functions that have the same name + signature, but that are
    // in 2 different namespaces.
    assert_rs_matches!(rs_api, quote! { pub fn not_overloaded() });
    Ok(())
}

/// !Unpin references should not be pinned.
#[gtest]
fn test_nonunpin_ref_param() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#"
        #pragma clang lifetime_elision
        struct S {~S();};
        void Function(const S& s);
    "#,
    )?)?
    .rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            fn Function<'a>(s: &'a crate::S) { ... }
        }
    );
    Ok(())
}

/// !Unpin mut references must be pinned.
#[gtest]
fn test_nonunpin_mut_param() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#"
        #pragma clang lifetime_elision
        struct S {~S();};
        void Function(S& s);
    "#,
    )?)?
    .rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            fn Function<'a>(s: ::core::pin::Pin<&'a mut crate::S>) { ... }
        }
    );
    Ok(())
}

/// !Unpin &self should not be pinned.
#[gtest]
fn test_nonunpin_ref_self() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#"
        #pragma clang lifetime_elision
        struct S {
          ~S();
          void Function() const;
        };
    "#,
    )?)?
    .rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            fn Function<'a>(&'a self) { ... }
        }
    );
    Ok(())
}

/// !Unpin &mut self must be pinned.
#[gtest]
fn test_nonunpin_mut_self() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#"
        #pragma clang lifetime_elision
        struct S {
          ~S();
          void Function();
        };
    "#,
    )?)?
    .rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            fn Function<'a>(self: ::core::pin::Pin<&'a mut Self>) { ... }
        }
    );
    Ok(())
}

/// Drop::drop must not use self : Pin<...>.
#[gtest]
fn test_nonunpin_drop() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#"
        struct S {~S();};
    "#,
    )?)?
    .rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) { ... }
        }
    );
    Ok(())
}

#[gtest]
fn test_nonunpin_0_arg_constructor() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        // This type must be `!Unpin`.
        struct HasConstructor {
            explicit HasConstructor() {}
            ~HasConstructor();
        };"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(rs_api, quote! {#[::ctor::recursively_pinned(PinnedDrop)]});
    assert_rs_matches!(
        rs_api,
        quote! {
            impl ::ctor::CtorNew<()> for HasConstructor {
                type CtorType = impl ::ctor::Ctor<Output = Self>;

                #[inline(always)]
                fn ctor_new(args: ()) -> Self::CtorType {
                    let () = args;
                    unsafe {
                        ::ctor::FnCtor::new(move |dest: *mut Self| {
                            crate::detail::__rust_thunk___ZN14HasConstructorC1Ev(dest as *mut ::core::ffi::c_void);
                        })
                    }
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_nonunpin_1_arg_constructor() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        // This type must be `!Unpin`.
        struct HasConstructor {
            explicit HasConstructor(unsigned char input) {}
            ~HasConstructor();
        };"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(rs_api, quote! {#[::ctor::recursively_pinned(PinnedDrop)]});
    assert_rs_matches!(
        rs_api,
        quote! {
            impl ::ctor::CtorNew<::core::ffi::c_uchar> for HasConstructor {
                type CtorType = impl ::ctor::Ctor<Output = Self>;

                #[inline (always)]
                fn ctor_new(args: ::core::ffi::c_uchar) -> Self::CtorType {
                    let input = args;
                    unsafe {
                        ::ctor::FnCtor::new(move |dest: *mut Self| {
                            crate::detail::__rust_thunk___ZN14HasConstructorC1Eh(dest as *mut ::core::ffi::c_void, input);
                        })
                    }
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_nonunpin_2_arg_constructor() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        // This type must be `!Unpin`.
        struct HasConstructor {
            explicit HasConstructor(unsigned char input1, signed char input2) {}
            ~HasConstructor();
        };"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(rs_api, quote! {#[::ctor::recursively_pinned(PinnedDrop)]});
    assert_rs_matches!(
        rs_api,
        quote! {
            impl ::ctor::CtorNew<(::core::ffi::c_uchar, ::core::ffi::c_schar)> for HasConstructor {
                type CtorType = impl ::ctor::Ctor<Output = Self>;

                #[inline (always)]
                fn ctor_new(args: (::core::ffi::c_uchar, ::core::ffi::c_schar)) -> Self::CtorType {
                    let (input1, input2) = args;
                    unsafe {
                        ::ctor::FnCtor::new(move |dest: *mut Self| {
                            crate::detail::__rust_thunk___ZN14HasConstructorC1Eha(dest as *mut ::core::ffi::c_void, input1, input2);
                        })
                    }
                }
            }
        }
    );
    Ok(())
}

/// Traits which monomorphize the `Ctor` parameter into the caller must
/// synthesize an RvalueReference parameter, with an appropriate
/// lifetime parameter.
#[gtest]
fn test_nonunpin_by_value_params() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        // This type must be `!Unpin`.
        struct HasConstructor {
            // int& x is here to create a 'b lifetime, which collides with a synthesized
            // lifetime name. But that's OK! We handle collisions!
            // (`a` would also work, but that's just because the left hand doesn't know what
            // the right is doing: the `a` lifetime is present in some places, but eventually
            // removed from the public interface.)
            explicit HasConstructor(const int& x, HasConstructor y, HasConstructor b) {}
            ~HasConstructor();
        };"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(rs_api, quote! {#[::ctor::recursively_pinned(PinnedDrop)]});
    assert_rs_matches!(
        rs_api,
        quote! {
            impl <'b, 'y, 'b_2> ::ctor::CtorNew<(
                &'b ::core::ffi::c_int,
                ::ctor::RvalueReference<'y, Self>,
                ::ctor::RvalueReference<'b_2, Self>)
            > for HasConstructor {
                // The captures are why we need explicit lifetimes for the two rvalue reference
                // parameters.
                type CtorType = impl ::ctor::Ctor<Output = Self> + use<'b, 'y, 'b_2>;

                #[inline (always)]
                fn ctor_new(args: (
                    &'b ::core::ffi::c_int,
                    ::ctor::RvalueReference<'y, Self>,
                    ::ctor::RvalueReference<'b_2, Self>)
                ) -> Self::CtorType {
                    let (x, y, b) = args;
                    unsafe {
                        ::ctor::FnCtor::new(move |dest: *mut Self| {
                            crate::detail::__rust_thunk___ZN14HasConstructorC1ERKiS_S_(dest as *mut ::core::ffi::c_void, x, y, b);
                        })
                    }
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_nonunpin_return() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        // This type must be `!Unpin`.
        struct Nontrivial {~Nontrivial();};

        Nontrivial ReturnsByValue(const int& x, const int& y);
        "#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            pub fn ReturnsByValue<'a, 'b>(x: &'a ::core::ffi::c_int, y: &'b ::core::ffi::c_int)
            -> impl ::ctor::Ctor<Output=crate::Nontrivial> + use<'a, 'b> {
                unsafe {
                    ::ctor::FnCtor::new(move |dest: *mut crate::Nontrivial| {
                        crate::detail::__rust_thunk___Z14ReturnsByValueRKiS0_(dest as *mut ::core::ffi::c_void, x, y);
                    })
                }

            }
        }
    );

    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___Z14ReturnsByValueRKiS0_(
                    struct Nontrivial* __return, int const* x, int const* y) {
                new(__return) auto(ReturnsByValue(*x, *y));
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_nonunpin_const_return() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        // This type must be `!Unpin`.
        struct Nontrivial {~Nontrivial();};

        const Nontrivial ReturnsByValue(const int& x, const int& y);
        "#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            pub fn ReturnsByValue<'a, 'b>(x: &'a ::core::ffi::c_int, y: &'b ::core::ffi::c_int)
            -> impl ::ctor::Ctor<Output=crate::Nontrivial> + use<'a, 'b> {
                unsafe {
                    ::ctor::FnCtor::new(move |dest: *mut crate::Nontrivial| {
                        crate::detail::__rust_thunk___Z14ReturnsByValueRKiS0_(dest as *mut ::core::ffi::c_void, x, y);
                    })
                }

            }
        }
    );

    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___Z14ReturnsByValueRKiS0_(
                    struct Nontrivial* __return, int const* x, int const* y) {
                new(__return) auto(ReturnsByValue(*x, *y));
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_unpin_by_value_param() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct Trivial final {
          int trivial_field;
        };

        void foo(Trivial param);
        "#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[inline(always)]
            pub fn foo(mut param: crate::Trivial) {
                unsafe { crate::detail::__rust_thunk___Z3foo7Trivial(&mut param) }
            }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            pub(crate) unsafe fn __rust_thunk___Z3foo7Trivial(param: &mut crate::Trivial);
        }
    );
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___Z3foo7Trivial(struct Trivial* param) {
                foo(std::move(*param));
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_unpin_by_value_return() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct Trivial final {
          int trivial_field;
        };

        Trivial foo();
        "#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[inline(always)]
            pub fn foo() -> crate::Trivial {
                unsafe {
                    let mut __return = ::core::mem::MaybeUninit::<crate::Trivial>::uninit();
                    crate::detail::__rust_thunk___Z3foov(&raw mut __return as *mut ::core::ffi::c_void);
                    __return.assume_init()
                }
            }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            pub(crate) unsafe fn __rust_thunk___Z3foov(
                __return: *mut ::core::ffi::c_void
            );
        }
    );
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___Z3foov(struct Trivial* __return) {
                new (__return) auto(foo());
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_unpin_rvalue_ref_qualified_method() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct TrivialWithRvalueRefQualifiedMethod final {
          void rvalue_ref_qualified_method() &&;
        };
        "#,
    )?;
    let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[inline(always)]
            pub fn rvalue_ref_qualified_method<'a>(self: ::ctor::RvalueReference<'a, Self>) {
                unsafe {
                    crate::detail::__rust_thunk___ZNO35TrivialWithRvalueRefQualifiedMethod27rvalue_ref_qualified_methodEv(self)
                }
            }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            #[link_name = "_ZNO35TrivialWithRvalueRefQualifiedMethod27rvalue_ref_qualified_methodEv"]
            pub(crate) unsafe fn __rust_thunk___ZNO35TrivialWithRvalueRefQualifiedMethod27rvalue_ref_qualified_methodEv < 'a > (__this :
                :: ctor :: RvalueReference < 'a , crate :: TrivialWithRvalueRefQualifiedMethod >) ;
        }
    );
    Ok(())
}

#[gtest]
fn test_unpin_rvalue_ref_const_qualified_method() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct TrivialWithRvalueRefConstQualifiedMethod final {
          void rvalue_ref_const_qualified_method() const &&;
        };
        "#,
    )?;
    let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[inline(always)]
            pub fn rvalue_ref_const_qualified_method<'a>(self: ::ctor::ConstRvalueReference<'a, Self>) {
                unsafe {
                    crate::detail::__rust_thunk___ZNKO40TrivialWithRvalueRefConstQualifiedMethod33rvalue_ref_const_qualified_methodEv(self)
                }
            }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            #[link_name = "_ZNKO40TrivialWithRvalueRefConstQualifiedMethod33rvalue_ref_const_qualified_methodEv"]
            pub(crate) unsafe fn __rust_thunk___ZNKO40TrivialWithRvalueRefConstQualifiedMethod33rvalue_ref_const_qualified_methodEv < 'a > (__this :
                :: ctor :: ConstRvalueReference < 'a , crate :: TrivialWithRvalueRefConstQualifiedMethod >) ;
        }
    );
    Ok(())
}

/// Assignment is special in that it discards the return type.
/// So if the return type is !Unpin, it needs to emplace!() it.
#[gtest]
fn test_nonunpin_return_assign() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        // This type must be `!Unpin`.
        struct Nontrivial {
            ~Nontrivial();
            Nontrivial operator=(const Nontrivial& other);
        };
        "#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            impl<'b> ::ctor::Assign<&'b Self> for Nontrivial {
                #[inline(always)]
                fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, other: &'b Self) {
                    unsafe {
                        let _ = ::ctor::emplace!(::ctor::FnCtor::new(
                            move |dest: *mut Self| {
                                crate::detail::__rust_thunk___ZN10NontrivialaSERKS_(
                                    dest as *mut ::core::ffi::c_void,
                                    self,
                                    other
                                );
                            }
                        ));
                    }
                }
            }
        }
    );

    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___ZN10NontrivialaSERKS_(
                struct Nontrivial* __return, struct Nontrivial* __this,
                const struct Nontrivial* other
            ) {
                new(__return) auto(__this->operator=(*other));
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_nonunpin_param() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        // This type must be `!Unpin`.
        struct Nontrivial {
            Nontrivial(Nontrivial&&);
            ~Nontrivial();
        };

        void TakesByValue(Nontrivial x);
        "#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            pub fn TakesByValue(x: impl ::ctor::Ctor<Output=crate::Nontrivial>) {
                unsafe {
                    crate::detail::__rust_thunk___Z12TakesByValue10Nontrivial(::core::pin::Pin::into_inner_unchecked(::ctor::emplace!(x)))
                }
            }
        }
    );

    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___Z12TakesByValue10Nontrivial(struct Nontrivial*x) {
                TakesByValue(std::move(*x));
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_nonunpin_trait_param() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        // This type must be `!Unpin`.
        struct Nontrivial {
            Nontrivial(Nontrivial&&);
            Nontrivial& operator=(Nontrivial) {}
            ~Nontrivial();
        };

        struct Trivial final {
            /*implicit*/ Trivial(Nontrivial) {}
        };
        "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            impl<'__param_0> From<::ctor::RvalueReference<'__param_0, crate::Nontrivial>> for Trivial {
                #[inline(always)]
                fn from(__param_0: ::ctor::RvalueReference<'__param_0, crate::Nontrivial>) -> Self {
                    let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                    unsafe {
                        crate::detail::__rust_thunk___ZN7TrivialC1E10Nontrivial(
                            &raw mut tmp as *mut ::core::ffi::c_void,
                            __param_0
                        );
                        tmp.assume_init()
                    }
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_nonmovable_param() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        // This type must be `!Unpin` and non-move constructible.
        struct Nonmovable {
            Nonmovable(Nonmovable&&) = delete;
        };

        void TakesByValue(Nonmovable) {}
        "#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    // Bindings for TakesByValue cannot be generated.
    assert_rs_matches!(rs_api, quote! {TakesByValue<'error>});
    assert_cc_not_matches!(rs_api_impl, quote! {TakesByValue});
    Ok(())
}

#[gtest]
fn test_invalid_unsafe_annotation_causes_fatal_error() -> googletest::Result<()> {
    let ir = ir_from_cc(
        r#"
        struct Trivial final {
            [[clang::annotate("crubit_override_unsafe", true)]]
            ~Trivial();
        };
        "#,
    )
    .or_fail()?;
    let error_message = generate_bindings_tokens_for_test(ir).err().or_fail()?.to_string();
    assert_that!(
        error_message,
        contains_substring("Unsafe annotations on destructors are not supported")
    );
    Ok(())
}

#[gtest]
fn test_function_returning_rvalue_reference() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct SomeStruct final {
            // Inline to force generation (and test coverage) of C++ thunks.
            inline SomeStruct&& GetRValueReference() {
              return static_cast<SomeStruct&&>(*this);
            }
            int field;
        };
        "#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            impl SomeStruct {
                ...
                #[inline(always)]
                pub fn GetRValueReference<'a>(&'a mut self)
                        -> ::ctor::RvalueReference<'a, crate::SomeStruct> {
                    unsafe {
                        crate::detail::__rust_thunk___ZN10SomeStruct18GetRValueReferenceEv(self)
                    }
                }
            }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            extern "C" {
                ...
                pub(crate) unsafe fn __rust_thunk___ZN10SomeStruct18GetRValueReferenceEv<'a>(
                        __this: &'a mut crate::SomeStruct
                   ) -> ::ctor::RvalueReference<'a, crate::SomeStruct>;
                ...
            }
        }
    );

    // Note that you can't just convert directly from xvalue to lvalue:
    //
    //     return &static_cast<SomeStruct&>(__this->GetRValueReference());
    //
    // For the above, Clang will emit an error that "non-const lvalue reference to
    // type 'struct SomeStruct' cannot bind to a temporary of type
    // 'SomeStruct'" (This is somewhat misleading, because there are no
    // temporaries here).  We must first bind the return value to a name
    // (`lvalue` below), so that it becomes an lvalue. Only then can it be
    // converted to a pointer.
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" struct SomeStruct*
            __rust_thunk___ZN10SomeStruct18GetRValueReferenceEv(struct SomeStruct* __this) {
                struct SomeStruct&& lvalue = __this->GetRValueReference();
                return &lvalue;
            }
        }
    );

    Ok(())
}

#[gtest]
fn test_c_abi_compatible_type_by_value_with_move() -> Result<()> {
    let ir = ir_from_cc(
        r#"
            typedef int MyTypedefDecl;

            inline void f(MyTypedefDecl a, void* b, int c) {}
        "#,
    )?;
    let BindingsTokens { rs_api_impl, .. } = generate_bindings_tokens_for_test(ir)?;
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___Z1fiPvi(MyTypedefDecl a, void* b, int c) {
                f(a, b, c);
            }
        }
    );
    Ok(())
}

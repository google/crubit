// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::{anyhow, Result};
use database::code_snippet::BindingsTokens;
use database::rs_snippet::{Mutability, RsTypeKind};
use database::BindingsGenerator;
use googletest::prelude::gtest;
use ir_testing::{retrieve_func, with_lifetime_macros};
use multiplatform_ir_testing::{ir_from_cc, ir_from_cc_dependency};
use quote::quote;
use static_assertions::{assert_impl_all, assert_not_impl_any};
use test_generators::{generate_bindings_tokens_for_test, TestDbFactory};
use token_stream_matchers::{
    assert_cc_matches, assert_cc_not_matches, assert_rs_matches, assert_rs_not_matches,
};
use token_stream_printer::rs_tokens_to_formatted_string_for_tests;

#[gtest]
fn test_disable_thread_safety_warnings() -> Result<()> {
    let ir = ir_from_cc("inline void foo() {}")?;
    let rs_api_impl = generate_bindings_tokens_for_test(ir)?.rs_api_impl;
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            ...
            __HASH_TOKEN__ pragma clang diagnostic push
            __HASH_TOKEN__ pragma clang diagnostic ignored "-Wthread-safety-analysis"
            ...

            __HASH_TOKEN__ pragma clang diagnostic pop
            ...
        }
    );
    Ok(())
}

#[gtest]
fn test_func_ptr_where_params_are_primitive_types() -> Result<()> {
    let ir = ir_from_cc(r#" int (*get_ptr_to_func())(float, double); "#)?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[inline(always)]
            pub fn get_ptr_to_func() -> Option<extern "C" fn (f32, f64) -> ::core::ffi::c_int> {
                unsafe { crate::detail::__rust_thunk___Z15get_ptr_to_funcv() }
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
                    #[link_name = "_Z15get_ptr_to_funcv"]
                    pub(crate) unsafe fn __rust_thunk___Z15get_ptr_to_funcv()
                    -> Option<extern "C" fn(f32, f64) -> ::core::ffi::c_int>;
                }
            }
        }
    );
    // Verify that no C++ thunk got generated.
    assert_cc_not_matches!(rs_api_impl, quote! { __rust_thunk___Z15get_ptr_to_funcv });

    // TODO(b/217419782): Add another test for more exotic calling conventions /
    // abis.

    // TODO(b/276461979): Add another test for pointer to a function that requires
    // thunks - e.g. because it takes/returns structs value. See also
    // b/276461979 and <internal link>

    Ok(())
}

#[gtest]
fn test_func_ref() -> Result<()> {
    let ir = ir_from_cc(r#" int (&get_ref_to_func())(float, double); "#)?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[inline(always)]
            pub fn get_ref_to_func() -> extern "C" fn (f32, f64) -> ::core::ffi::c_int {
                unsafe { crate::detail::__rust_thunk___Z15get_ref_to_funcv() }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_func_ptr_with_non_static_lifetime() -> Result<()> {
    let ir = ir_from_cc(&with_lifetime_macros(
        r#"
        int (* $a get_ptr_to_func())(float, double); "#,
    ))?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_cc_matches!(rs_api, {
        let txt = "Generated from: ir_from_cc_virtual_header.h;l=33\n\
                       Error while generating bindings for function 'get_ptr_to_func':\n\
                       Unable to get lifetime annotations: Type may not be annotated with lifetimes";
        quote! { __COMMENT__ #txt }
    });
    Ok(())
}

#[gtest]
fn test_func_ptr_where_params_are_raw_ptrs() -> Result<()> {
    let ir = ir_from_cc(r#" const int* (*get_ptr_to_func())(const int*); "#)?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[inline(always)]
            pub fn get_ptr_to_func() -> Option<unsafe extern "C" fn (*const ::core::ffi::c_int) -> *const ::core::ffi::c_int> {
                unsafe { crate::detail::__rust_thunk___Z15get_ptr_to_funcv() }
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
                    #[link_name = "_Z15get_ptr_to_funcv"]
                    pub(crate) unsafe fn __rust_thunk___Z15get_ptr_to_funcv()
                    -> Option<unsafe extern "C" fn(*const ::core::ffi::c_int) -> *const ::core::ffi::c_int>;
                }
            }
        }
    );
    // Verify that no C++ thunk got generated.
    assert_cc_not_matches!(rs_api_impl, quote! { __rust_thunk___Z15get_ptr_to_funcv });

    // TODO(b/217419782): Add another test where params (and the return
    // type) are references with lifetimes.  Something like this:
    //     #pragma clang lifetime_elision
    //     const int& (*get_ptr_to_func())(const int&, const int&); "#)?;
    // 1) Need to investigate why this fails - seeing raw pointers in Rust seems to
    //    indicate that no lifetimes are present at the `importer.cc` level. Maybe
    //    lifetime elision doesn't support this scenario? Unclear how to explicitly
    //    apply [[clang::annotate("lifetimes", "a, b -> a")]] to the _inner_
    //    function.
    // 2) It is important to have 2 reference parameters, so see if the problem of
    //    passing `lifetimes` by value would have been caught - see:
    //    cl/428079010/depot/rs_bindings_from_cc/
    // importer.cc?version=s6#823

    // TODO(b/217419782): Decide what to do if the C++ pointer is *not*
    // annotated with a lifetime - emit `unsafe fn(...) -> ...` in that
    // case?

    Ok(())
}

mod custom_abi_tests {
    use super::*;
    use ir_matchers::assert_ir_matches;
    #[gtest]
    fn test_func_ptr_with_custom_abi() -> Result<()> {
        if multiplatform_testing::test_platform() != multiplatform_testing::Platform::X86Linux {
            return Ok(());
        }
        let ir = ir_from_cc(r#" int (*get_ptr_to_func())(float, double) [[clang::vectorcall]]; "#)?;

        // Verify that the test input correctly represents what we intend to
        // test - we want [[clang::vectorcall]] to apply to the returned
        // function pointer, but *not* apply to the `get_ptr_to_func` function.
        assert_ir_matches!(
            ir,
            quote! {
                Func(Func {
                    cc_name: "get_ptr_to_func", ...
                    return_type: CcType {
                        variant: FuncPointer {
                            non_null: false,
                            call_conv: X86VectorCall, ...
                        }, ...
                    }, ...
                    has_c_calling_convention: true, ...
                }),
            }
        );

        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
        // Check that the custom "vectorcall" ABI gets propagated into the
        // return type (i.e. into `extern "vectorcall" fn`).
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn get_ptr_to_func() -> Option<extern "vectorcall" fn (f32, f64) -> ::core::ffi::c_int> {
                    unsafe { crate::detail::__rust_thunk___Z15get_ptr_to_funcv() }
                }
            }
        );

        // The usual `extern "C"` ABI should be used for "get_ptr_to_func".
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    unsafe extern "C" {
                        #[link_name = "_Z15get_ptr_to_funcv"]
                        pub(crate) unsafe fn __rust_thunk___Z15get_ptr_to_funcv()
                        -> Option<extern "vectorcall" fn(f32, f64) -> ::core::ffi::c_int>;
                    }
                }
            }
        );

        // Verify that no C++ thunk got generated.
        assert_cc_not_matches!(rs_api_impl, quote! { __rust_thunk___Z15get_ptr_to_funcv });
        Ok(())
    }

    #[gtest]
    fn test_func_ptr_with_custom_abi_thunk() -> Result<()> {
        if multiplatform_testing::test_platform() != multiplatform_testing::Platform::X86Linux {
            return Ok(());
        }
        // Using an `inline` keyword forces generation of a C++ thunk in
        // `rs_api_impl` (i.e. exercises `format_cpp_type`,
        // `format_cc_call_conv_as_clang_attribute` and similar code).
        let ir = ir_from_cc(
            r#"
            inline int (*inline_get_ptr_to_func())(float, double) [[clang::vectorcall]];
        "#,
        )?;

        // Verify that the test input correctly represents what we intend to
        // test - we want [[clang::vectorcall]] to apply to the returned
        // function pointer, but *not* apply to the `get_ptr_to_func` function.
        assert_ir_matches!(
            ir,
            quote! {
                Func(Func {
                    cc_name: "inline_get_ptr_to_func", ...
                    return_type: CcType {
                        variant: FuncPointer {
                            non_null: false,
                            call_conv: X86VectorCall, ...
                        }, ...
                    }, ...
                    has_c_calling_convention: true, ...
                }),
            }
        );

        // This test is quite similar to `test_func_ptr_thunk` - the main
        // difference is verification of the `__attribute__((vectorcall))` in
        // the expected signature of the generated thunk below.
        let rs_api_impl = generate_bindings_tokens_for_test(ir)?.rs_api_impl;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" crubit::type_identity_t<
                        int(float , double) __attribute__((vectorcall))
                    >* __rust_thunk___Z22inline_get_ptr_to_funcv() {
                    return inline_get_ptr_to_func();
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_custom_abi_thunk() -> Result<()> {
        if multiplatform_testing::test_platform() != multiplatform_testing::Platform::X86Linux {
            return Ok(());
        }
        let ir = ir_from_cc(
            r#"
            float f_vectorcall_calling_convention(float p1, float p2) [[clang::vectorcall]];
            double f_c_calling_convention(double p1, double p2);
        "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn f_vectorcall_calling_convention(p1: f32, p2: f32) -> f32 {
                    unsafe {
                        crate::detail::__rust_thunk___Z31f_vectorcall_calling_conventionff(p1, p2)
                    }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn f_c_calling_convention(p1: f64, p2: f64) -> f64 {
                    unsafe { crate::detail::__rust_thunk___Z22f_c_calling_conventiondd(p1, p2) }
                }
            }
        );
        // `link_name` (i.e. no thunk) for `f_c_calling_convention`. No
        // `link_name` (i.e. indicates presence of a thunk) for
        // `f_vectorcall_calling_convention`.
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    unsafe extern "C" {
                        pub(crate) unsafe fn __rust_thunk___Z31f_vectorcall_calling_conventionff(
                            p1: f32, p2: f32) -> f32;
                        #[link_name = "_Z22f_c_calling_conventiondd"]
                        pub(crate) unsafe fn __rust_thunk___Z22f_c_calling_conventiondd(
                            p1: f64, p2: f64) -> f64;
                    }
                }
            }
        );
        // C++ thunk needed for `f_vectorcall_calling_convention`.
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" float __rust_thunk___Z31f_vectorcall_calling_conventionff(
                    float p1, float p2) {
                        return f_vectorcall_calling_convention(p1, p2);
                }
            }
        );
        // No C++ thunk expected for `f_c_calling_convention`.
        assert_cc_not_matches!(rs_api_impl, quote! { f_c_calling_convention ( ... ) });
        Ok(())
    }
}

#[gtest]
fn test_item_order() -> Result<()> {
    let ir = ir_from_cc(
        "int first_func();
         struct FirstStruct {};
         int second_func();
         struct SecondStruct {};",
    )?;

    let rs_api =
        rs_tokens_to_formatted_string_for_tests(generate_bindings_tokens_for_test(ir)?.rs_api)?;

    let idx = |s: &str| rs_api.find(s).ok_or_else(|| anyhow!("'{}' missing", s));

    let f1 = idx("fn first_func")?;
    let f2 = idx("fn second_func")?;
    let s1 = idx("struct FirstStruct")?;
    let s2 = idx("struct SecondStruct")?;
    let t1 = idx("fn __rust_thunk___Z10first_funcv")?;
    let t2 = idx("fn __rust_thunk___Z11second_funcv")?;

    assert!(f1 < s1);
    assert!(s1 < f2);
    assert!(f2 < s2);
    assert!(s2 < t1);
    assert!(t1 < t2);

    Ok(())
}

/// At the least, a trivial type should have no drop impl if or until we add
/// empty drop impls.
#[gtest]
fn test_no_impl_drop() -> Result<()> {
    let ir = ir_from_cc("struct Trivial {};")?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_not_matches!(rs_api, quote! {impl Drop});
    assert_rs_not_matches!(rs_api, quote! {impl ::ctor::PinnedDrop});
    Ok(())
}

/// User-defined destructors *must* become Drop impls with ManuallyDrop
/// fields
#[gtest]
fn test_impl_drop_user_defined_destructor() -> Result<()> {
    let ir = ir_from_cc(
        r#" struct NontrivialStruct { ~NontrivialStruct(); };
        struct UserDefinedDestructor {
            ~UserDefinedDestructor();
            int x;
            NontrivialStruct nts;
        };"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            impl ::ctor::PinnedDrop for UserDefinedDestructor {
                #[inline(always)]
                unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
                    crate::detail::__rust_thunk___ZN21UserDefinedDestructorD1Ev(self)
                }
            }
        }
    );
    assert_rs_matches!(rs_api, quote! {pub x: ::core::ffi::c_int,});
    assert_rs_matches!(
        rs_api,
        quote! {pub nts: ::core::mem::ManuallyDrop<crate::NontrivialStruct>,}
    );
    Ok(())
}

/// nontrivial types without user-defined destructors should invoke
/// the C++ destructor to preserve the order of field destructions.
#[gtest]
fn test_impl_drop_nontrivial_member_destructor() -> Result<()> {
    // TODO(jeanpierreda): This would be cleaner if the UserDefinedDestructor code were
    // omitted. For example, we simulate it so that UserDefinedDestructor
    // comes from another library.
    let ir = ir_from_cc(
        r#"struct UserDefinedDestructor final {
            ~UserDefinedDestructor();
        };
        struct TrivialStruct final { int i; };
        struct NontrivialMembers final {
            UserDefinedDestructor udd;
            TrivialStruct ts;
            int x;
        };"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            impl ::ctor::PinnedDrop for NontrivialMembers {
                #[inline(always)]
                unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
                    crate::detail::__rust_thunk___ZN17NontrivialMembersD1Ev(self)
                }
            }
        }
    );
    assert_rs_matches!(rs_api, quote! {pub x: ::core::ffi::c_int,});
    assert_rs_matches!(rs_api, quote! {pub ts: crate::TrivialStruct,});
    assert_rs_matches!(
        rs_api,
        quote! {pub udd: ::core::mem::ManuallyDrop<crate::UserDefinedDestructor>,}
    );
    Ok(())
}

#[gtest]
fn test_type_alias() -> Result<()> {
    let ir = ir_from_cc(
        r#"
            // MyTypedefDecl doc comment
            typedef int MyTypedefDecl;

            using MyTypeAliasDecl = int;
            using MyTypeAliasDecl_Alias = MyTypeAliasDecl;

            struct S final {};
            using S_Alias = S;
            using S_Alias_Alias = S_Alias;

            inline void f(MyTypedefDecl t) {}
        "#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[doc = " MyTypedefDecl doc comment\n \n Generated from: ir_from_cc_virtual_header.h;l=5"]
            pub type MyTypedefDecl = ::core::ffi::c_int;
        }
    );
    assert_rs_matches!(rs_api, quote! { pub type MyTypeAliasDecl = ::core::ffi::c_int; });
    assert_rs_matches!(rs_api, quote! { pub type MyTypeAliasDecl_Alias = crate::MyTypeAliasDecl; });
    assert_rs_matches!(rs_api, quote! { pub type S_Alias = crate::S; });
    assert_rs_matches!(rs_api, quote! { pub type S_Alias_Alias = crate::S_Alias; });
    assert_rs_matches!(rs_api, quote! { pub fn f(t: crate::MyTypedefDecl) });
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___Z1fi(MyTypedefDecl t) { f(t); }
        }
    );
    Ok(())
}

#[gtest]
fn test_rs_type_kind_implements_copy() -> Result<()> {
    let template = r#" LIFETIMES
        struct [[clang::trivial_abi]] TrivialStruct final { int i; };
        struct [[clang::trivial_abi]] UserDefinedCopyConstructor final {
            UserDefinedCopyConstructor(const UserDefinedCopyConstructor&);
        };
        using IntAlias = int;
        using TrivialAlias = TrivialStruct;
        using NonTrivialAlias = UserDefinedCopyConstructor;
        void func(PARAM_TYPE some_param);
    "#;
    assert_impl_all!(i32: Copy);
    assert_impl_all!(&i32: Copy);
    assert_not_impl_any!(&mut i32: Copy);
    assert_impl_all!(Option<&i32>: Copy);
    assert_not_impl_any!(Option<&mut i32>: Copy);
    assert_impl_all!(*const i32: Copy);
    assert_impl_all!(*mut i32: Copy);
    struct Test {
        // Test inputs:
        cc: &'static str,
        lifetimes: bool,
        // Expected test outputs:
        rs: &'static str,
        is_copy: bool,
    }
    let tests = vec![
        // Validity of the next few tests is verified via
        // `assert_[not_]impl_all!` static assertions above.
        Test { cc: "int", lifetimes: true, rs: ":: core :: ffi :: c_int", is_copy: true },
        Test {
            cc: "const int&",
            lifetimes: true,
            rs: "& 'a :: core :: ffi :: c_int",
            is_copy: true,
        },
        Test {
            cc: "int&",
            lifetimes: true,
            rs: "& 'a mut :: core :: ffi :: c_int",
            is_copy: false,
        },
        Test {
            cc: "const int*",
            lifetimes: true,
            rs: "Option < & 'a :: core :: ffi :: c_int >",
            is_copy: true,
        },
        Test {
            cc: "int*",
            lifetimes: true,
            rs: "Option < & 'a mut :: core :: ffi :: c_int >",
            is_copy: false,
        },
        Test {
            cc: "const int*",
            lifetimes: false,
            rs: "* const :: core :: ffi :: c_int",
            is_copy: true,
        },
        Test { cc: "int*", lifetimes: false, rs: "* mut :: core :: ffi :: c_int", is_copy: true },
        Test { cc: "void*", lifetimes: false, rs: "* mut :: core :: ffi :: c_void", is_copy: true },
        Test {
            cc: "const void*",
            lifetimes: false,
            rs: "* const :: core :: ffi :: c_void",
            is_copy: true,
        },
        Test {
            cc: "void* const*",
            lifetimes: false,
            rs: "* const * mut :: core :: ffi :: c_void",
            is_copy: true,
        },
        // Tests below have been thought-through and verified "manually".
        // TrivialStruct is expected to derive Copy.
        Test { cc: "TrivialStruct", lifetimes: true, rs: "crate :: TrivialStruct", is_copy: true },
        Test {
            cc: "UserDefinedCopyConstructor",
            lifetimes: true,
            rs: "crate :: UserDefinedCopyConstructor",
            is_copy: false,
        },
        Test { cc: "IntAlias", lifetimes: true, rs: "crate :: IntAlias", is_copy: true },
        Test { cc: "TrivialAlias", lifetimes: true, rs: "crate :: TrivialAlias", is_copy: true },
        Test {
            cc: "NonTrivialAlias",
            lifetimes: true,
            rs: "crate :: NonTrivialAlias",
            is_copy: false,
        },
    ];
    for test in tests.iter() {
        let test_name = format!("cc='{}', lifetimes={}", test.cc, test.lifetimes);
        let cc_input = template.replace("PARAM_TYPE", test.cc).replace(
            "LIFETIMES",
            if test.lifetimes { "#pragma clang lifetime_elision" } else { "" },
        );
        let db_factory = TestDbFactory::from_cc(&cc_input)?;
        let db = db_factory.make_db();
        let ir = db.ir();

        let f = retrieve_func(&ir, "func");
        let t = db.rs_type_kind(f.params[0].type_.clone())?;

        let fmt = t.to_token_stream(&db).to_string();
        assert_eq!(test.rs, fmt, "Testing: {}", test_name);

        assert_eq!(test.is_copy, t.implements_copy(), "Testing: {}", test_name);
    }
    Ok(())
}

#[gtest]
fn test_rs_type_kind_is_shared_ref_to_with_lifetimes() -> Result<()> {
    let cc_input = "#pragma clang lifetime_elision
        struct SomeStruct {};
        void foo(const SomeStruct& foo_param);
        void bar(SomeStruct& bar_param);";
    let db_factory = TestDbFactory::from_cc(cc_input)?;
    let db = db_factory.make_db();
    let ir = db.ir();
    let record = ir.records().next().unwrap();
    let foo_func = retrieve_func(&ir, "foo");
    let bar_func = retrieve_func(&ir, "bar");

    // const-ref + lifetimes in C++  ===>  shared-ref in Rust
    assert_eq!(foo_func.params.len(), 1);
    let foo_param = &foo_func.params[0];
    assert_eq!(foo_param.identifier.identifier.as_ref(), "foo_param");
    let foo_type = db.rs_type_kind(foo_param.type_.clone())?;
    assert!(foo_type.is_shared_ref_to(record));
    assert!(matches!(foo_type, RsTypeKind::Reference { mutability: Mutability::Const, .. }));

    // non-const-ref + lifetimes in C++  ===>  mutable-ref in Rust
    assert_eq!(bar_func.params.len(), 1);
    let bar_param = &bar_func.params[0];
    assert_eq!(bar_param.identifier.identifier.as_ref(), "bar_param");
    let bar_type = db.rs_type_kind(bar_param.type_.clone())?;
    assert!(!bar_type.is_shared_ref_to(record));
    assert!(matches!(bar_type, RsTypeKind::Reference { mutability: Mutability::Mut, .. }));

    Ok(())
}

#[gtest]
fn test_rs_type_kind_is_shared_ref_to_without_lifetimes() -> Result<()> {
    let cc_input = "struct SomeStruct {};
         void foo(const SomeStruct& foo_param);";
    let db_factory = TestDbFactory::from_cc(cc_input)?;
    let db = db_factory.make_db();
    let ir = db.ir();
    let record = ir.records().next().unwrap();
    let foo_func = retrieve_func(&ir, "foo");

    // const-ref + *no* lifetimes in C++  ===>  const-pointer in Rust
    assert_eq!(foo_func.params.len(), 1);
    let foo_param = &foo_func.params[0];
    assert_eq!(foo_param.identifier.identifier.as_ref(), "foo_param");
    let foo_type = db.rs_type_kind(foo_param.type_.clone())?;
    assert!(!foo_type.is_shared_ref_to(record));
    assert!(matches!(foo_type, RsTypeKind::Pointer { mutability: Mutability::Const, .. }));

    Ok(())
}

#[gtest]
fn test_rs_type_kind_lifetimes() -> Result<()> {
    let cc_input = r#"
        #pragma clang lifetime_elision
        using TypeAlias = int&;
        struct SomeStruct {};
        void foo(int a, int& b, int&& c, int* d, int** e, TypeAlias f, SomeStruct g); "#;
    let db_factory = TestDbFactory::from_cc(cc_input)?;
    let db = db_factory.make_db();
    let ir = db.ir();
    let func = retrieve_func(&ir, "foo");
    let ret = db.rs_type_kind(func.return_type.clone())?;
    let a = db.rs_type_kind(func.params[0].type_.clone())?;
    let b = db.rs_type_kind(func.params[1].type_.clone())?;
    let c = db.rs_type_kind(func.params[2].type_.clone())?;
    let d = db.rs_type_kind(func.params[3].type_.clone())?;
    let e = db.rs_type_kind(func.params[4].type_.clone())?;
    let f = db.rs_type_kind(func.params[5].type_.clone())?;
    let g = db.rs_type_kind(func.params[6].type_.clone())?;

    assert_eq!(0, ret.lifetimes().count()); // No lifetimes on `void`.
    assert_eq!(0, a.lifetimes().count()); // No lifetimes on `int`.
    assert_eq!(1, b.lifetimes().count()); // `&'a i32` has a single lifetime.
    assert_eq!(1, c.lifetimes().count()); // `RvalueReference<'a, i32>` has a single lifetime.
    assert_eq!(1, d.lifetimes().count()); // `Option<&'b i32>` has a single lifetime.
    assert_eq!(2, e.lifetimes().count()); // `&'c Option<&'d i32>` has two lifetimes.
    assert_eq!(1, f.lifetimes().count()); // Lifetime of underlying type should show through.
    assert_eq!(0, g.lifetimes().count()); // No lifetimes on structs (yet).
    Ok(())
}

#[gtest]
fn test_rs_type_kind_lifetimes_raw_ptr() -> Result<()> {
    let cc_input = "void foo(int* a);";
    let db_factory = TestDbFactory::from_cc(cc_input)?;
    let db = db_factory.make_db();
    let ir = db.ir();
    let f = retrieve_func(&ir, "foo");
    let a = db.rs_type_kind(f.params[0].type_.clone())?;
    assert_eq!(0, a.lifetimes().count()); // No lifetimes on `int*`.
    Ok(())
}

#[gtest]
fn test_rs_type_kind_rejects_func_ptr_that_returns_struct_by_value() -> Result<()> {
    let cc_input = r#"
        struct SomeStruct {
          int field;
        };
        SomeStruct (*get_ptr_to_func())();
    "#;
    let db_factory = TestDbFactory::from_cc(cc_input)?;
    let db = db_factory.make_db();
    let ir = db.ir();
    let f = retrieve_func(&ir, "get_ptr_to_func");

    // Expecting an error, because passing a struct by value requires a thunk and
    // function pointers don't have a thunk.
    let err = db.rs_type_kind(f.return_type.clone()).unwrap_err();
    let msg = err.to_string();
    assert_eq!(
        msg,
        "Either the return type or some of the parameter types require \
                an FFI thunk (and function pointers don't have a thunk)",
    );
    Ok(())
}

#[gtest]
fn test_rs_type_kind_rejects_func_ptr_that_takes_struct_by_value() -> Result<()> {
    let cc_input = r#"
        struct SomeStruct {
          int field;
        };
        void (*get_ptr_to_func())(SomeStruct);
    "#;
    let db_factory = TestDbFactory::from_cc(cc_input)?;
    let db = db_factory.make_db();
    let ir = db.ir();
    let f = retrieve_func(&ir, "get_ptr_to_func");

    // Expecting an error, because passing a struct by value requires a thunk and
    // function pointers don't have a thunk.
    let err = db.rs_type_kind(f.return_type.clone()).unwrap_err();
    let msg = err.to_string();
    assert_eq!(
        msg,
        "Either the return type or some of the parameter types require \
                an FFI thunk (and function pointers don't have a thunk)",
    );
    Ok(())
}

#[gtest]
fn test_rust_keywords_are_escaped_in_rs_api_file() -> Result<()> {
    let ir = ir_from_cc("struct type { int dyn; };")?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(rs_api, quote! { struct r#type { ... r#dyn: ::core::ffi::c_int ... } });
    Ok(())
}

#[gtest]
fn test_rust_keywords_are_not_escaped_in_rs_api_impl_file() -> Result<()> {
    let ir = ir_from_cc("struct type { int dyn; };")?;
    let rs_api_impl = generate_bindings_tokens_for_test(ir)?.rs_api_impl;
    assert_cc_matches!(
        rs_api_impl,
        quote! { static_assert(CRUBIT_OFFSET_OF(dyn, struct type) ... ) }
    );
    Ok(())
}

#[gtest]
fn test_namespace_module_items() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#"
        namespace test_namespace_bindings {
            int func();
            struct S {};
            namespace inner {
                int inner_func();
                struct InnerS {};
            }
        }
    "#,
    )?)?
    .rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            pub mod test_namespace_bindings {
                ...
                pub fn func() -> ::core::ffi::c_int { ... }
                ...
                pub struct S { ... }
                ...
                pub mod inner {
                    ...
                    pub fn inner_func() -> ::core::ffi::c_int { ... }
                    ...
                    pub struct InnerS { ... }
                    ...
                }
                ...
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_detail_outside_of_namespace_module() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#"
        namespace test_namespace_bindings {
            int f();
        }
    "#,
    )?)?
    .rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            pub mod test_namespace_bindings {
                ...
            }
            ...
            mod detail {
                #[allow(unused_imports)]
                use super::*;
                unsafe extern "C" {
                    #[link_name = "_ZN23test_namespace_bindings1fEv"]
                    pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings1fEv() -> ::core::ffi::c_int;
                }
            }
            ...
        }
    );
    Ok(())
}

#[gtest]
fn test_assertions_outside_of_namespace_module() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#"
        namespace test_namespace_bindings {
            struct S {
                int i;
            };
        }
    "#,
    )?)?
    .rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            pub mod test_namespace_bindings {
                ...
            }
            ...
            const _: () = {
                ...
                assert!(::core::mem::size_of::<crate::test_namespace_bindings::S>() == 4);
                assert!(::core::mem::align_of::<crate::test_namespace_bindings::S>() == 4);
                ...
                assert!(::core::mem::offset_of!(crate::test_namespace_bindings::S, i) == 0);
                ...
            };
        }
    );
    Ok(())
}

#[gtest]
fn test_reopened_namespaces() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#"
    namespace test_namespace_bindings {
    namespace inner {}
    }  // namespace test_namespace_bindings

    namespace test_namespace_bindings {
    namespace inner {}
    }  // namespace test_namespace_bindings"#,
    )?)?
    .rs_api;

    assert_rs_matches!(
        rs_api,
        quote! {
            ...
            pub mod test_namespace_bindings {
                pub mod inner {} ...
            }
            ...
        }
    );
    Ok(())
}

#[gtest]
fn test_qualified_identifiers_in_impl_file() -> Result<()> {
    let rs_api_impl = generate_bindings_tokens_for_test(ir_from_cc(
        r#"
    namespace test_namespace_bindings {
        inline void f() {};
        struct S final {};
    }
    inline void useS(test_namespace_bindings::S s) {};"#,
    )?)?
    .rs_api_impl;

    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___ZN23test_namespace_bindings1fEv() {
                test_namespace_bindings::f();
            }
            ...
            extern "C" void __rust_thunk___Z4useSN23test_namespace_bindings1SE(
                    struct test_namespace_bindings::S* s) {
                useS(std::move(*s));
            }
            ...
        }
    );
    Ok(())
}

#[gtest]
fn test_inline_namespace() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#"
        namespace test_namespace_bindings {
            inline namespace inner {
                struct MyStruct final {};
            }
            void processMyStruct(MyStruct s);
        }
        void processMyStructOutsideNamespace(test_namespace_bindings::inner::MyStruct s);
        void processMyStructSkipInlineNamespaceQualifier(test_namespace_bindings::MyStruct s);
        "#,
    )?)?
    .rs_api;

    assert_rs_matches!(
        rs_api,
        quote! {
            ...
            pub mod test_namespace_bindings {
                ...
                pub mod inner {
                    ...
                    pub struct MyStruct {...} ...
                }
                __HASH_TOKEN__[allow(unused_imports)]
                pub use inner::*;
                ...
                pub fn processMyStruct(
                    mut s: crate::test_namespace_bindings::inner::MyStruct)
                ...
            }
            ...
            pub fn processMyStructOutsideNamespace(
                mut s: crate::test_namespace_bindings::inner::MyStruct)
            ...
            pub fn processMyStructSkipInlineNamespaceQualifier(
                mut s: crate::test_namespace_bindings::inner::MyStruct)
            ...
        }
    );
    Ok(())
}

#[gtest]
fn test_inline_namespace_not_marked_inline() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#"
        inline namespace my_inline {}
        namespace foo {}
        namespace my_inline {  // still an inline namespace!
            struct MyStruct final {};
        }
        "#,
    )?)?
    .rs_api;

    assert_rs_matches!(
        rs_api,
        quote! {
           ...
           pub mod foo {}
           pub mod my_inline {
               ...
               pub struct MyStruct {...}
               ...
           }
           __HASH_TOKEN__[allow(unused_imports)]
           pub use my_inline::*;
           ...
        }
    );
    Ok(())
}

/// Enumerators with unknown attributes on otherwise-ok enums are omitted.
///
/// This is hard to test any other way than token comparison!
#[gtest]
fn test_supported_unknown_attr_enumerator() -> Result<()> {
    let mut ir = ir_from_cc(
        r#"
        enum Enum {
            kHidden [[deprecated]],
        };
        "#,
    )?;
    *ir.target_crubit_features_mut(&ir.current_target().clone()) =
        crubit_feature::CrubitFeature::Supported.into();
    let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(rs_api, quote! {pub struct Enum});
    assert_rs_not_matches!(rs_api, quote! {kHidden});
    Ok(())
}

/// Namespaces with an unknown attribute are not present in supported.
///
/// This is hard to test any other way than token comparison, because it's
/// hard to test for the nonexistence of a module.
#[gtest]
fn test_supported_unknown_attr_namespace() -> Result<()> {
    for nested_notpresent in ["struct NotPresent {};", "struct NotPresent;", "enum NotPresent {};"]
    {
        let mut ir = ir_from_cc(&format!(
            r#"
            namespace [[deprecated]] unknown_attr_namespace {{
                {nested_notpresent}
            }}
            extern "C" {{
                void NotPresent(unknown_attr_namespace::NotPresent);
                unknown_attr_namespace::NotPresent AlsoNotPresent();
            }}
            "#
        ))?;
        *ir.target_crubit_features_mut(&ir.current_target().clone()) =
            crubit_feature::CrubitFeature::Supported.into();
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;
        // The namespace, and everything in it or using it, will be missing from the
        // output.
        assert_rs_not_matches!(rs_api, quote! {NotPresent});
        assert_rs_not_matches!(rs_api, quote! {AlsoNotPresent});
        assert_rs_not_matches!(rs_api, quote! {unknown_attr_namespace});
    }
    Ok(())
}

/// Namespaces with an unknown attribute are still merged with the same
/// namespace with no unknown attribute.
#[gtest]
fn test_supported_unknown_attr_namespace_merge() -> Result<()> {
    let mut ir = ir_from_cc(
        r#"
        namespace unknown_attr_namespace {
            enum Present {};
        }
        namespace [[deprecated]] unknown_attr_namespace {
            enum NotPresent {};
        }
        namespace unknown_attr_namespace {
            enum AlsoPresent {};
        }
        "#,
    )?;
    *ir.target_crubit_features_mut(&ir.current_target().clone()) =
        crubit_feature::CrubitFeature::Supported.into();
    let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;
    // The namespace, and everything in it or using it, will be missing from the
    // output.
    assert_rs_not_matches!(rs_api, quote! {NotPresent});
    assert_rs_matches!(rs_api, quote! {Present});
    assert_rs_matches!(rs_api, quote! {AlsoPresent});
    assert_rs_matches!(rs_api, quote! {unknown_attr_namespace});
    Ok(())
}

/// Namespaces with an unknown attribute are not present in supported, but
/// their typedefs are.
#[gtest]
fn test_supported_unknown_attr_namespace_typedef() -> Result<()> {
    let mut ir = ir_from_cc(
        r#"
        namespace [[deprecated]] unknown_attr_namespace {
            using NotPresent = int;
        }
        extern "C" {
            void Func(unknown_attr_namespace::NotPresent x);
            unknown_attr_namespace::NotPresent Func2();
        }
        "#,
    )?;
    *ir.target_crubit_features_mut(&ir.current_target().clone()) =
        crubit_feature::CrubitFeature::Supported.into();
    let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;
    // The namespace, and everything in it or using it, will be missing from the
    // output.
    assert_rs_not_matches!(rs_api, quote! {NotPresent});
    assert_rs_matches!(rs_api, quote! {pub fn Func(x: ::core::ffi::c_int)});
    assert_rs_matches!(rs_api, quote! {pub fn Func2() -> ::core::ffi::c_int});
    Ok(())
}

/// The default crubit feature set currently doesn't include supported.
#[gtest]
fn test_default_crubit_features_disabled_supported() -> Result<()> {
    for (item, kind) in [
        ("extern \"C\" void NotPresent() {}", "function"),
        ("struct NotPresent {};", "struct"),
        ("extern \"C\" int NotPresent() {}", "function"),
    ] {
        let mut ir = ir_from_cc(item)?;
        ir.target_crubit_features_mut(&ir.current_target().clone()).clear();
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
        assert_rs_not_matches!(rs_api, quote! {NotPresent});
        assert_cc_not_matches!(rs_api_impl, quote! {NotPresent});
        let contents = rs_tokens_to_formatted_string_for_tests(rs_api)?;
        // using a string comparison and leaving off the end, because the exact reason
        // why differs per item.
        let expected = &format!("\
            // Generated from: ir_from_cc_virtual_header.h;l=3\n\
            // Error while generating bindings for {kind} 'NotPresent':\n\
            // Can't generate bindings for NotPresent, because of missing required features (<internal link>):\n\
            // //test:testing_target needs [//features:supported] for NotPresent");
        assert!(contents.contains(expected), "Missing expected string: {contents}\n")
    }
    Ok(())
}

/// The default crubit feature set currently doesn't include wrapper.
/// (Note that all experimental features are intended to also be included in
/// the wrapper feature set, so this subsumes any `disabled_experimental`
/// test.)
#[gtest]
fn test_default_crubit_features_disabled_wrapper() -> Result<()> {
    let mut ir = ir_from_cc("struct NotPresent;")?;
    ir.target_crubit_features_mut(&ir.current_target().clone()).clear();
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_not_matches!(rs_api, quote! {NotPresent});
    assert_cc_not_matches!(rs_api_impl, quote! {NotPresent});
    let expected = "\
        Error while generating bindings for struct 'NotPresent':\n\
        Can't generate bindings for NotPresent, because of missing required features (<internal link>):\n\
        //test:testing_target needs [//features:wrapper] for NotPresent (incomplete type)";
    assert_rs_matches!(rs_api, quote! { __COMMENT__ #expected});
    Ok(())
}

#[gtest]
fn test_default_crubit_features_disabled_dependency_supported_function_parameter() -> Result<()> {
    let mut ir = ir_from_cc_dependency(
        "void Func(NotPresent);",
        /*dependency=*/ "struct NotPresent {};",
    )?;
    ir.target_crubit_features_mut(&ir::BazelLabel("//test:dependency".into())).clear();
    *ir.target_crubit_features_mut(&ir.current_target().clone()) =
        crubit_feature::CrubitFeature::Supported.into();
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_not_matches!(rs_api, quote! {Func});
    assert_cc_not_matches!(rs_api_impl, quote! {Func});
    Ok(())
}

#[gtest]
fn test_default_crubit_features_disabled_dependency_wrapper_function_parameter() -> Result<()> {
    let mut ir = ir_from_cc_dependency(
        "void Func(NotPresent);",
        "template <typename T> struct NotPresentTemplate {T x;}; using NotPresent = NotPresentTemplate<int>;",
    )?;
    ir.target_crubit_features_mut(&ir::BazelLabel("//test:dependency".into())).clear();
    *ir.target_crubit_features_mut(&ir.current_target().clone()) =
        crubit_feature::CrubitFeature::Supported.into();
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_not_matches!(rs_api, quote! {Func});
    assert_cc_not_matches!(rs_api_impl, quote! {Func});
    Ok(())
}

#[gtest]
fn test_default_crubit_features_disabled_dependency_supported_function_return_type() -> Result<()> {
    let mut ir = ir_from_cc_dependency("NotPresent Func();", "struct NotPresent {};")?;
    ir.target_crubit_features_mut(&ir::BazelLabel("//test:dependency".into())).clear();
    *ir.target_crubit_features_mut(&ir.current_target().clone()) =
        crubit_feature::CrubitFeature::Supported.into();
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_not_matches!(rs_api, quote! {Func});
    assert_cc_not_matches!(rs_api_impl, quote! {Func});
    Ok(())
}

#[gtest]
fn test_default_crubit_features_disabled_dependency_wrapper_function_return_type() -> Result<()> {
    let mut ir = ir_from_cc_dependency(
        "NotPresent Func();",
        "template <typename T> struct NotPresentTemplate {T x;}; using NotPresent = NotPresentTemplate<int>;")?;
    ir.target_crubit_features_mut(&ir::BazelLabel("//test:dependency".into())).clear();
    *ir.target_crubit_features_mut(&ir.current_target().clone()) =
        crubit_feature::CrubitFeature::Supported.into();
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_not_matches!(rs_api, quote! {Func});
    assert_cc_not_matches!(rs_api_impl, quote! {Func});
    Ok(())
}

#[gtest]
fn test_default_crubit_features_disabled_dependency_struct() -> Result<()> {
    for dependency in ["struct NotPresent {signed char x;};", "using NotPresent = signed char;"] {
        let mut ir = ir_from_cc_dependency("struct Present {NotPresent field;};", dependency)?;
        ir.target_crubit_features_mut(&ir::BazelLabel("//test:dependency".into())).clear();
        *ir.target_crubit_features_mut(&ir.current_target().clone()) =
            crubit_feature::CrubitFeature::Supported.into();
        let BindingsTokens { rs_api, rs_api_impl: _ } = generate_bindings_tokens_for_test(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub struct Present {
                    ...
                    pub(crate) field: [::core::mem::MaybeUninit<u8>; 1],
                }
            }
        );
    }
    Ok(())
}

#[gtest]
fn test_default_crubit_features_disabled_template_explicit_specialization() -> Result<()> {
    let mut ir = ir_from_cc(
        r#"
        template <typename T>
        struct X {
            T t;
        };

        template <>
        struct X<int> {
            int val;
            X<int>() : val(42) {}
        };

        inline X<int> NotPresent() { return X<int>(); }"#,
    )?;
    *ir.target_crubit_features_mut(&ir.current_target().clone()) =
        crubit_feature::CrubitFeature::Supported.into();
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_not_matches!(rs_api, quote! {NotPresent});
    assert_cc_not_matches!(rs_api_impl, quote! {NotPresent});
    Ok(())
}

#[gtest]
fn test_default_crubit_features_disabled_variadic_function() -> Result<()> {
    let mut ir = ir_from_cc(
        r#"
        int sprintf(char* str, const char* format, ...);
        "#,
    )?;
    *ir.target_crubit_features_mut(&ir.current_target().clone()) =
        crubit_feature::CrubitFeature::Supported.into();
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_not_matches!(rs_api, quote! {sprintf});
    assert_cc_not_matches!(rs_api_impl, quote! {sprintf});
    Ok(())
}

#[gtest]
fn test_type_map_override_assert() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#" #pragma clang lifetime_elision
            // Broken class: uses i32 but has size 1.
            // (These asserts would fail if this were compiled.)
            class [[clang::annotate("crubit_internal_rust_type", "i32")]] Class final {};"#,
    )?)?
    .rs_api;

    assert_rs_matches!(
        rs_api,
        quote! {
            assert!(::core::mem::size_of::<i32>() == 1);
        }
    );

    assert_rs_matches!(
        rs_api,
        quote! {
            assert!(::core::mem::align_of::<i32>() == 1);
        }
    );
    Ok(())
}

#[gtest]
fn test_type_map_override_c_abi_incompatible() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#" #pragma clang lifetime_elision
            // Broken class: uses i32 but has size 1.
            // (These asserts would fail if this were compiled.)
            class [[clang::annotate("crubit_internal_rust_type", "i8")]] MyI8 {unsigned char field;};
            MyI8 Make();"#,
    )?)?
    .rs_api;

    assert_rs_matches!(
        rs_api,
        quote! {
            pub fn Make() -> i8 {...}
        }
    );

    assert_rs_matches!(
        rs_api,
        quote! {
            pub(crate) unsafe fn __rust_thunk___Z4Makev(__return: *mut ::core::ffi::c_void);
        }
    );
    Ok(())
}

#[gtest]
fn test_type_map_override_c_abi_compatible() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#" #pragma clang lifetime_elision
            class
                [[clang::annotate("crubit_internal_rust_type", "i8")]]
                [[clang::annotate("crubit_internal_same_abi")]]
                MyI8 {unsigned char field;};
            MyI8 Make();"#,
    )?)?
    .rs_api;

    assert_rs_matches!(
        rs_api,
        quote! {
            pub fn Make() -> i8 {...}
        }
    );

    assert_rs_matches!(
        rs_api,
        quote! {
            pub(crate) unsafe fn __rust_thunk___Z4Makev() -> i8;
        }
    );
    Ok(())
}

/// We cannot generate size/align assertions for incomplete types.
#[gtest]
fn test_type_map_override_assert_incomplete() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#" #pragma clang lifetime_elision
            // Broken class: uses i32 but has size 1.
            // (These asserts would fail if this were compiled.)
            class [[clang::annotate("crubit_internal_rust_type", "i32")]] Incomplete;
        "#,
    )?)?
    .rs_api;

    assert_rs_not_matches!(
        rs_api,
        quote! {
        const _: () = { ... ::core::mem::size_of::<i32>() ... } }
    );

    assert_rs_not_matches!(
        rs_api,
        quote! {
        const _: () = { ... ::core::mem::align_of::<i32>() ... }}
    );
    Ok(())
}

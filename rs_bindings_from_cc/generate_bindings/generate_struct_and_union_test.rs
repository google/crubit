// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::Result;
use code_gen_utils::make_rs_ident;
use database::code_snippet::BindingsTokens;
use generate_struct_and_union::generate_derives;
use googletest::prelude::gtest;
use ir_testing::with_lifetime_macros;
use multiplatform_ir_testing::{ir_from_cc, ir_from_cc_dependency, ir_record};
use proc_macro2::TokenStream;
use quote::quote;
use test_generators::generate_bindings_tokens_for_test;
use token_stream_matchers::{
    assert_cc_matches, assert_cc_not_matches, assert_rs_matches, assert_rs_not_matches,
};
use token_stream_printer::tokens_to_string;

#[gtest]
fn test_template_in_dependency_and_alias_in_current_target() -> Result<()> {
    // See also the test with the same name in `ir_from_cc_test.rs`.
    let ir = {
        let dependency_src = r#" #pragma clang lifetime_elision
                template <typename T>
                struct MyTemplate {
                    ~MyTemplate();
                    T GetValue() { return field; }
                    T field;
                }; "#;
        let current_target_src = r#" #pragma clang lifetime_elision
                using MyAliasOfTemplate = MyTemplate<int>; "#;
        ir_from_cc_dependency(current_target_src, dependency_src)?
    };

    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(C)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=MyTemplate < int >"]
            pub struct __CcTemplateInst10MyTemplateIiE {
                pub field: ::core::ffi::c_int,
            }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            impl __CcTemplateInst10MyTemplateIiE {
                #[doc = " Generated from: test/dependency_header.h;l=5"]
                #[inline(always)]
                pub fn GetValue<'a>(self: ... Pin<&'a mut Self>) -> ::core::ffi::c_int { unsafe {
                    crate::detail::__rust_thunk___ZN10MyTemplateIiE8GetValueEv__2f_2ftest_3atesting_5ftarget(
                        self)
                }}
            }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            pub type MyAliasOfTemplate = crate::__CcTemplateInst10MyTemplateIiE;
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            mod detail { ...  unsafe extern "C" {
                ...
                pub(crate) unsafe fn
                __rust_thunk___ZN10MyTemplateIiE8GetValueEv__2f_2ftest_3atesting_5ftarget<'a>(
                    __this: ... Pin<&'a mut crate::__CcTemplateInst10MyTemplateIiE>
                ) -> ::core::ffi::c_int;
                ...
            } }
        }
    );
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C"
            int __rust_thunk___ZN10MyTemplateIiE8GetValueEv__2f_2ftest_3atesting_5ftarget(
                    struct MyTemplate<int>* __this) {
                return __this->GetValue();
            }
        }
    );

    Ok(())
}

#[gtest]
fn test_template_with_out_of_line_definition() -> Result<()> {
    // See also an end-to-end test in the `test/templates/out_of_line_definition`
    // directory.
    let ir = ir_from_cc(
        r#" #pragma clang lifetime_elision
            template <typename T>
            class MyTemplate final {
             public:
              static MyTemplate Create(T value);
              const T& value() const;

             private:
              T value_;
            };

            using MyTypeAlias = MyTemplate<int>; "#,
    )?;

    let BindingsTokens { rs_api_impl, .. } = generate_bindings_tokens_for_test(ir)?;

    // Even though the member functions above are *not* defined inline (e.g.
    // IR::Func::is_inline is false), they still need to have thunks generated for
    // them (to force/guarantee that the class template and its members get
    // instantiated).  This is also covered in the following end-to-end
    // tests:
    // - test/templates/out_of_line_definition/ - without a thunk, the template
    //   won't be instantiated and Rust bindings won't be able to call the member
    //   function (there will be no instantiation of the member function in the C++
    //   object files)
    // - test/templates/definition_in_cc/ - the instantiation happens in the .cc
    //   file and therefore the thunk is not *required* (but it doesn't hurt to have
    //   the thunk)
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void
            __rust_thunk___ZN10MyTemplateIiE6CreateEi__2f_2ftest_3atesting_5ftarget(
                class MyTemplate<int>* __return, int value) {
              new (__return) auto(MyTemplate<int>::Create(value));
            }
        }
    );
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" int const*
            __rust_thunk___ZNK10MyTemplateIiE5valueEv__2f_2ftest_3atesting_5ftarget(
                    class MyTemplate<int> const * __this) {
                return std::addressof(__this->value());
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_simple_struct() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        #pragma clang lifetime_elision
        struct SomeStruct final {
            ~SomeStruct() {}
            int public_int;
          protected:
            int protected_int;
          private:
           int private_int;
        };
    "#,
    )?;

    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[::ctor::recursively_pinned(PinnedDrop)]
            #[repr(C, align(4))]
            #[doc="CRUBIT_ANNOTATE: cpp_type=SomeStruct"]
            pub struct SomeStruct {
                __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
                pub public_int: ::core::ffi::c_int,
                #[doc = " Reason for representing this field as a blob of bytes:\n Types of non-public C++ fields can be elided away"]
                pub(crate) protected_int: [::core::mem::MaybeUninit<u8>; 4],
                #[doc = " Reason for representing this field as a blob of bytes:\n Types of non-public C++ fields can be elided away"]
                pub(crate) private_int: [::core::mem::MaybeUninit<u8>; 4],
            }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            const _ : () = {
                ...
                assert!(::core::mem::size_of::<crate::SomeStruct>() == 12);
                assert!(::core::mem::align_of::<crate::SomeStruct>() == 4);
                static_assertions::assert_impl_all!(crate::SomeStruct: Drop);
                static_assertions::assert_not_impl_any!(crate::SomeStruct: Copy);
                assert!(::core::mem::offset_of!(crate::SomeStruct, public_int) == 0);
                assert!(::core::mem::offset_of!(crate::SomeStruct, protected_int) == 4);
                assert!(::core::mem::offset_of!(crate::SomeStruct, private_int) == 8);
                ...
            };
        }
    );
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___ZN10SomeStructD1Ev(struct SomeStruct * __this) {
                std::destroy_at(__this);
            }
        }
    );
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            static_assert(CRUBIT_SIZEOF(struct SomeStruct) == 12);
            static_assert(alignof(struct SomeStruct) == 4);
            static_assert(CRUBIT_OFFSET_OF(public_int, struct SomeStruct) == 0);
        }
    );
    Ok(())
}

#[gtest]
fn test_struct_vs_class() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        #pragma clang lifetime_elision
        struct SomeStruct final {
            SomeStruct() {}
            int field;
        };
        class SomeClass final {
          public:
            SomeClass() {}
            int field;
        };
    "#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;

    // A Rust `struct` is generated for both `SomeStruct` and `SomeClass`.
    assert_rs_matches!(rs_api, quote! { pub struct SomeStruct },);
    assert_rs_matches!(rs_api, quote! { pub struct SomeClass },);

    // But in C++ we still should refer to `struct SomeStruct` and `class
    // SomeClass`. See also b/238212337.
    assert_cc_matches!(rs_api_impl, quote! { struct SomeStruct * __this });
    assert_cc_matches!(rs_api_impl, quote! { class SomeClass * __this });
    assert_cc_matches!(
        rs_api_impl,
        quote! { static_assert(CRUBIT_SIZEOF(struct SomeStruct) == 4); }
    );
    assert_cc_matches!(rs_api_impl, quote! { static_assert(CRUBIT_SIZEOF(class SomeClass) == 4); });
    Ok(())
}

#[gtest]
fn test_struct_vs_typedefed_struct() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        #pragma clang lifetime_elision
        struct SomeStruct final {
          int x;
        } __attribute__((aligned(16)));
        typedef struct {
          int x;
        } SomeAnonStruct __attribute__((aligned(16)));
    "#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;

    // A `struct` is generated for both `SomeStruct` and `SomeAnonStruct`, both
    // in Rust and in C++.
    assert_rs_matches!(rs_api, quote! { pub struct SomeStruct },);
    assert_rs_matches!(rs_api, quote! { pub struct SomeAnonStruct },);
    assert_rs_matches!(rs_api_impl, quote! { struct SomeStruct * __this },);
    assert_rs_matches!(rs_api_impl, quote! { SomeAnonStruct * __this },);

    // In C++, both have align == 16, but size for `SomeAnonStruct` is not aligned.
    // `SomeAnonStruct` won't have `struct` in the assert.
    assert_cc_matches!(rs_api_impl, quote! { static_assert(alignof(struct SomeStruct) == 16); });
    assert_cc_matches!(rs_api_impl, quote! { static_assert(alignof(SomeAnonStruct) == 16); });
    assert_cc_matches!(
        rs_api_impl,
        quote! { static_assert(CRUBIT_SIZEOF(struct SomeStruct) == 16); }
    );
    assert_cc_matches!(rs_api_impl, quote! { static_assert(CRUBIT_SIZEOF(SomeAnonStruct) == 16); });

    // In Rust, both have align == 16 and size == 16.
    assert_rs_matches!(
        rs_api,
        quote! { assert!(::core::mem::size_of::<crate::SomeStruct>() == 16); }
    );
    assert_rs_matches!(
        rs_api,
        quote! { assert!(::core::mem::align_of::<crate::SomeStruct>() == 16); }
    );
    assert_rs_matches!(
        rs_api,
        quote! { assert!(::core::mem::size_of::<crate::SomeAnonStruct>() == 16); }
    );
    assert_rs_matches!(
        rs_api,
        quote! { assert!(::core::mem::align_of::<crate::SomeAnonStruct>() == 16); }
    );

    Ok(())
}

#[gtest]
fn test_record_with_unsupported_field_type() -> Result<()> {
    // Using a volatile field because it's currently not supported.
    // But... any other unsupported type would also work for this test.
    let ir = ir_from_cc(
        r#"
        struct StructWithUnsupportedField {
          // Doc comment for `my_field`.
          volatile int my_field;
        };
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(C, align(4))]
            #[doc="CRUBIT_ANNOTATE: cpp_type=StructWithUnsupportedField"]
            pub struct StructWithUnsupportedField {
                #[doc = " Doc comment for `my_field`.\n \n Reason for representing this field as a blob of bytes:\n Unsupported `volatile` qualifier: volatile int"]
                pub(crate) my_field: [::core::mem::MaybeUninit<u8>; 4],
            }
            ...
            const _: () = {
                ...
                assert!(
                ::core::mem::offset_of!(crate::StructWithUnsupportedField, my_field) == 0);
                ...
            };
        }
    );
    Ok(())
}

/// This is a regression test for b/283835873 where the alignment of the
/// generated struct was wrong/missing.
#[gtest]
fn test_struct_with_only_bitfields() -> Result<()> {
    let ir = ir_from_cc(
        r#"
            struct SomeStruct {
              char32_t code_point : 31;
              enum : char32_t {
                ok = 0,
                error = 1
              } status : 1;
            };
            static_assert(sizeof(SomeStruct) == 4);
            static_assert(alignof(SomeStruct) == 4);
        "#,
    )?;
    let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
           #[repr(C, align(4))]
            #[doc="CRUBIT_ANNOTATE: cpp_type=SomeStruct"]
           pub struct SomeStruct { ...  }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            const _: () = {
                ...
                assert!(::core::mem::size_of::<crate::SomeStruct>() == 4);
                assert!(::core::mem::align_of::<crate::SomeStruct>() == 4);
                ...
            };
        }
    );
    Ok(())
}

#[gtest]
fn test_struct_with_unnamed_bitfield_member() -> Result<()> {
    // This test input causes `field_decl->getName()` to return an empty string.
    // This example is based on `struct timex` from bits/timex.h
    let ir = ir_from_cc(
        r#"
        struct SomeStruct {
            int first_field;
            int :32;
            int last_field;
        }; "#,
    )?;
    let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(C, align(4))]
            #[doc="CRUBIT_ANNOTATE: cpp_type=SomeStruct"]
            pub struct SomeStruct {
                pub first_field: ::core::ffi::c_int, ...
                __bitfields1: [::core::mem::MaybeUninit<u8>; 4],
                pub last_field: ::core::ffi::c_int,
            }
            ...
            const _: () = {
                ...
                assert!(::core::mem::offset_of!(crate::SomeStruct, first_field) == 0);
                assert!(::core::mem::offset_of!(crate::SomeStruct, last_field) == 8);
                ...
            };
        }
    );
    Ok(())
}

/// Classes with a non-public destructor shouldn't be constructible, not
/// even via Copy/Clone.
#[gtest]
fn test_trivial_nonpublic_destructor() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct Indestructible final {
          Indestructible() = default;
          Indestructible(int);
          Indestructible(const Indestructible&) = default;
          void Foo() const;
         private:
          ~Indestructible() = default;
        };

        Indestructible ReturnsValue();
        void TakesValue(Indestructible);
        void TakesReference(const Indestructible& x);
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    // It isn't available by value:
    assert_rs_matches!(rs_api, quote! {impl<'error> Default});
    assert_rs_matches!(rs_api, quote! {impl<'error> From});
    assert_rs_matches!(rs_api, quote! {impl<'error> Clone});
    assert_rs_not_matches!(rs_api, quote! {ReturnsValue});
    assert_rs_matches!(rs_api, quote! {TakesValue<'error>});
    // ... but it is otherwise available:
    assert_rs_matches!(rs_api, quote! {struct Indestructible});
    assert_rs_matches!(rs_api, quote! {fn Foo<'a>(&'a self)});
    assert_rs_matches!(rs_api, quote! {fn TakesReference<'a>(x: &'a crate::Indestructible)});
    Ok(())
}

#[gtest]
fn test_nontrivial_nonpublic_destructor() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct Indestructible final {
          Indestructible() = default;
          Indestructible(int);
          Indestructible(const Indestructible&) = default;
          void Foo() const;
         private:
          ~Indestructible() {}
        };

        Indestructible ReturnsValue();
        void TakesValue(Indestructible);
        void TakesReference(const Indestructible& x);
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    // It isn't available by value:
    assert_rs_not_matches!(rs_api, quote! {CtorNew});
    assert_rs_not_matches!(rs_api, quote! {ReturnsValue});
    assert_rs_matches!(rs_api, quote! {TakesValue<'error>});
    // ... but it is otherwise available:
    assert_rs_matches!(rs_api, quote! {struct Indestructible});
    assert_rs_matches!(rs_api, quote! {fn Foo<'a>(&'a self)});
    assert_rs_matches!(rs_api, quote! {fn TakesReference<'a>(x: &'a crate::Indestructible)});
    Ok(())
}

/// trivial abstract structs shouldn't be constructible, not even via
/// Copy/Clone.
///
/// Right now, a struct can only be Copy/Clone if it's final, but that
/// restriction will likely be lifted later.
#[gtest]
fn test_trivial_abstract_by_value() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct Abstract final {
          Abstract() = default;
          Abstract(int);
          Abstract(const Abstract&) = default;
          virtual void Foo() const = 0;
          void Nonvirtual() const;
        };
        void TakesAbstract(const Abstract& a);
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    // It isn't available by value:
    assert_rs_not_matches!(rs_api, quote! {Default});
    assert_rs_not_matches!(rs_api, quote! {From});
    assert_rs_not_matches!(rs_api, quote! {derive ( ... Copy ... )});
    assert_rs_not_matches!(rs_api, quote! {derive ( ... Clone ... )});
    // ... but it is otherwise available:
    assert_rs_matches!(rs_api, quote! {struct Abstract});
    assert_rs_matches!(rs_api, quote! {fn Foo<'a>(&'a self)});
    assert_rs_matches!(rs_api, quote! {fn Nonvirtual<'a>(&'a self)});
    assert_rs_matches!(rs_api, quote! {fn TakesAbstract<'a>(a: &'a crate::Abstract)});
    Ok(())
}

#[gtest]
fn test_nontrivial_abstract_by_value() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct Abstract final {
          Abstract() {};
          Abstract(int);
          Abstract(const Abstract&) {}
          virtual void Foo() const = 0;
          void Nonvirtual() const;
        };
        void TakesAbstract(const Abstract& a);
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_not_matches!(rs_api, quote! {CtorNew});
    // ... but it is otherwise available:
    assert_rs_matches!(rs_api, quote! {struct Abstract});
    assert_rs_matches!(rs_api, quote! {fn Foo<'a>(&'a self)});
    assert_rs_matches!(rs_api, quote! {fn Nonvirtual<'a>(&'a self)});
    assert_rs_matches!(rs_api, quote! {fn TakesAbstract<'a>(a: &'a crate::Abstract)});
    Ok(())
}

#[gtest]
fn test_struct_with_unnamed_struct_and_union_members() -> Result<()> {
    // This test input causes `field_decl->getName()` to return an empty string.
    // See also:
    // - https://en.cppreference.com/w/c/language/struct: "[...] an unnamed member
    //   of a struct whose type is a struct without name is known as anonymous
    //   struct."
    // - https://rust-lang.github.io/rfcs/2102-unnamed-fields.html
    let ir = ir_from_cc(
        r#"
        struct StructWithUnnamedMembers {
          int first_field;

          struct {
            int anonymous_struct_field_1;
            int anonymous_struct_field_2;
          };
          union {
            int anonymous_union_field_1;
            int anonymous_union_field_2;
          };

          int last_field;
        }; "#,
    )?;
    let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;
    // Once anonymous structs and unions are supported, `__unnamed_field1` and `__unnamed_field2`
    // should have a real, usable type.
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(C, align(4))]
            #[doc="CRUBIT_ANNOTATE: cpp_type=StructWithUnnamedMembers"]
            pub struct StructWithUnnamedMembers {
               pub first_field: ::core::ffi::c_int,
               #[doc =" Reason for representing this field as a blob of bytes:\n Unsupported type 'StructWithUnnamedMembers::(anonymous struct at ./ir_from_cc_virtual_header.h:7:11)': No generated bindings found for ''"]
               pub(crate) __unnamed_field1: [::core::mem::MaybeUninit<u8>; 8],
               #[doc =" Reason for representing this field as a blob of bytes:\n Unsupported type 'StructWithUnnamedMembers::(anonymous union at ./ir_from_cc_virtual_header.h:11:11)': No generated bindings found for ''"]
               pub(crate) __unnamed_field2: [::core::mem::MaybeUninit<u8>; 4],
               pub last_field: ::core::ffi::c_int,
            }
            ...
            const _: () = {
                ...
                assert!(::core::mem::offset_of!(
                    crate::StructWithUnnamedMembers, first_field) == 0);
                assert!(::core::mem::offset_of!(
                   crate::StructWithUnnamedMembers, __unnamed_field1) == 4);
                assert!(::core::mem::offset_of!(
                   crate::StructWithUnnamedMembers, __unnamed_field2) == 12);
                assert!(::core::mem::offset_of!(
                   crate::StructWithUnnamedMembers, last_field) == 16);
                ...
            };
        }
    );
    Ok(())
}

#[track_caller]
fn assert_derives(record: &ir::Record, expected: &[&str]) {
    let derives: Vec<TokenStream> = generate_derives(record).0;
    let formatted_derives: Vec<String> =
        derives.into_iter().map(|d| tokens_to_string(d).unwrap()).collect();
    assert_eq!(formatted_derives, expected);
}

#[gtest]
fn test_copy_derives() {
    let record = ir_record("S");
    assert_derives(&record, &["Clone", "Copy", "::ctor::MoveAndAssignViaCopy"]);
}

#[gtest]
fn test_copy_derives_not_is_trivial_abi() {
    let mut record = ir_record("S");
    record.is_trivial_abi = false;
    assert_derives(&record, &[]);
}

#[gtest]
fn test_copy_derives_ctor_deleted() {
    let mut record = ir_record("S");
    record.copy_constructor = ir::SpecialMemberFunc::Unavailable;
    assert_derives(&record, &[]);
}

#[gtest]
fn test_copy_derives_ctor_nontrivial_members() {
    let mut record = ir_record("S");
    record.copy_constructor = ir::SpecialMemberFunc::NontrivialMembers;
    assert_derives(&record, &[]);
}

#[gtest]
fn test_copy_derives_ctor_nontrivial_self() {
    let mut record = ir_record("S");
    record.copy_constructor = ir::SpecialMemberFunc::NontrivialUserDefined;
    assert_derives(&record, &[]);
}

/// In Rust, a Drop type cannot be Copy.
#[gtest]
fn test_copy_derives_dtor_nontrivial_self() {
    let mut record = ir_record("S");
    for definition in
        [ir::SpecialMemberFunc::NontrivialUserDefined, ir::SpecialMemberFunc::NontrivialMembers]
    {
        record.destructor = definition;
        assert_derives(&record, &["Clone"]);
    }
}

/// If a base class is a bridge type, it doesn't exist at all, and can't be upcasted to.
#[gtest]
fn test_bridged_base_class() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        struct [[clang::annotate("crubit_bridge_type", "BridgedBase"),
            clang::annotate("crubit_bridge_type_rust_to_cpp_converter",
                         "rust_to_cpp_converter"),
            clang::annotate("crubit_bridge_type_cpp_to_rust_converter",
                         "cpp_to_rust_converter")]] Base {
            int x;
        };

        struct Derived : Base {};
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_not_matches!(rs_api, quote! { Base });
    assert_rs_not_matches!(rs_api, quote! { BridgedBase });
    Ok(())
}

#[gtest]
fn test_base_class_subobject_layout() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        // We make `Base` non-POD to force `Derived::z` to live inside the tail padding of `Base`.
        // On the Itanium ABI, this would not happen if `Base` were a POD type.
        struct Base {
          Base() {}
          __INT64_TYPE__ x;
          char y;
        };
        struct Derived final : Base {
          __INT16_TYPE__ z;
        };
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(C, align(8))]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Derived"]
            pub struct Derived {
                __non_field_data: [::core::mem::MaybeUninit<u8>; 10],
                pub z: ::core::ffi::c_short,
            }
        }
    );
    Ok(())
}

/// The same as test_base_class_subobject_layout, but with multiple
/// inheritance.
#[gtest]
fn test_base_class_multiple_inheritance_subobject_layout() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        class Base1 {__INT64_TYPE__ x;};
        class Base2 {char y;};
        struct Derived final : Base1, Base2 {__INT16_TYPE__ z;};
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(C, align(8))]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Derived"]
            pub struct Derived {
                __non_field_data: [::core::mem::MaybeUninit<u8>; 10],
                pub z: ::core::ffi::c_short,
            }
        }
    );
    Ok(())
}

/// The same as test_base_class_subobject_layout, but with a chain of
/// inheritance.
#[gtest]
fn test_base_class_deep_inheritance_subobject_layout() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        class Base1 {__INT64_TYPE__ x;};
        class Base2 : Base1 {char y;};
        struct Derived final : Base2 {__INT16_TYPE__ z;};
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(C, align(8))]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Derived"]
            pub struct Derived {
                __non_field_data: [::core::mem::MaybeUninit<u8>; 10],
                pub z: ::core::ffi::c_short,
            }
        }
    );
    Ok(())
}

/// For derived classes with no data members, we can't use the offset of the
/// first member to determine the size of the base class subobjects.
#[gtest]
fn test_base_class_subobject_fieldless_layout() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        class Base {__INT64_TYPE__ x; char y;};
        struct Derived final : Base {};
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(C, align(8))]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Derived"]
            pub struct Derived {
                __non_field_data: [::core::mem::MaybeUninit<u8>; 16],
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_base_class_subobject_empty_fieldless() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        class Base {};
        struct Derived final : Base {};
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(C)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Derived"]
            pub struct Derived {
                ...
                __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_base_class_subobject_empty() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        class Base {};
        struct Derived final : Base {
            __INT16_TYPE__ x;
        };
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[doc="CRUBIT_ANNOTATE: cpp_type=Derived"]
            pub struct Derived {
                pub x: ::core::ffi::c_short,
            }
        }
    );
    Ok(())
}

/// Non-aggregate structs can't be directly initialized, because we add
/// a zero-sized private field to the bindings.
#[gtest]
fn test_non_aggregate_struct_private_field() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        struct NonAggregate {
            NonAggregate() {}

            __INT16_TYPE__ x = 0;
        };
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            pub struct NonAggregate {
                __non_field_data:  [::core::mem::MaybeUninit<u8>; 0],
                pub x: ::core::ffi::c_short,
            }
        }
    );
    Ok(())
}

/// When a field is [[no_unique_address]], it occupies the space up to the
/// next field.
#[gtest]
fn test_no_unique_address() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        class Field1 {__INT64_TYPE__ x;};
        class Field2 {char y;};
        struct Struct final {
            [[no_unique_address]] Field1 field1;
            [[no_unique_address]] Field2 field2;
            __INT16_TYPE__ z;
        };
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(C, align(8))]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Struct"]
            pub struct Struct {
                ...
                pub(crate) field1: [::core::mem::MaybeUninit<u8>; 8],
                ...
                pub(crate) field2: [::core::mem::MaybeUninit<u8>; 2],
                pub z: ::core::ffi::c_short,
            }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! {
            impl Struct {
                pub fn field1(&self) -> &crate::Field1 {
                    unsafe {
                        let ptr = (self as *const Self as *const u8).offset(0);
                        &*(ptr as *const crate::Field1)
                    }
                }
                pub fn field2(&self) -> &crate::Field2 {
                    unsafe {
                        let ptr = (self as *const Self as *const u8).offset(8);
                        &*(ptr as *const crate::Field2)
                    }
                }
            }
        }
    );
    Ok(())
}

/// When a [[no_unique_address]] field is the last one, it occupies the rest
/// of the object.
#[gtest]
fn test_no_unique_address_last_field() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        class Field1 {__INT64_TYPE__ x;};
        class Field2 {char y;};
        struct Struct final {
            [[no_unique_address]] Field1 field1;
            [[no_unique_address]] Field2 field2;
        };
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(C, align(8))]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Struct"]
            pub struct Struct {
                ...
                pub(crate) field1: [::core::mem::MaybeUninit<u8>; 8],
                ...
                pub(crate) field2: [::core::mem::MaybeUninit<u8>; 8],
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_no_unique_address_empty() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        class Field {};
        struct Struct final {
            // Doc comment for no_unique_address empty class type field.
            [[no_unique_address]] Field field;
            int x;
        };
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(C)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Struct"]
            pub struct Struct {
                pub x: ::core::ffi::c_int,
            }
            ...
            impl Struct {
              # [doc = " Doc comment for no_unique_address empty class type field."]
              pub fn field(&self) -> &crate::Field {
                    unsafe {
                        let ptr = (self as *const Self as *const u8).offset(0);
                        &*(ptr as *const crate::Field)
                    }
                  }
            }
            ...
        }
    );
    Ok(())
}

#[gtest]
fn test_base_class_subobject_empty_last_field() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        class Field {};
        struct Struct final {
            // Doc comment for no_unique_address empty class type field.
            [[no_unique_address]] Field field;
        };
    "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(C)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=Struct"]
            pub struct Struct {}
            ...
            impl Struct {
              # [doc = " Doc comment for no_unique_address empty class type field."]
              pub fn field(&self) -> &crate::Field {
                  unsafe {
                      let ptr = (self as *const Self as *const u8).offset(0);
                      &*(ptr as *const crate::Field)
                  }
              }
          }
          ...
        }
    );
    Ok(())
}

#[gtest]
fn test_doc_comment_record() -> Result<()> {
    let ir = ir_from_cc(
        "// Doc Comment\n\
        //\n\
        //  * with bullet\n\
        struct SomeStruct final {\n\
            // Field doc\n\
            int field;\
        };",
    )?;

    assert_rs_matches!(
        generate_bindings_tokens_for_test(ir)?.rs_api,
        quote! {
            #[doc = " Doc Comment\n \n  * with bullet\n \n Generated from: ir_from_cc_virtual_header.h;l=6"]
            #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
            #[repr(C)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=SomeStruct"]
            pub struct SomeStruct {
                # [doc = " Field doc"]
                pub field: ::core::ffi::c_int,
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_basic_union() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        #pragma clang lifetime_elision
        union SomeUnion {
            int some_field;
            long long some_bigger_field;
        };
        "#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;

    assert_rs_matches!(
        rs_api,
        quote! {
            #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
            #[repr(C)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=SomeUnion"]
            pub union SomeUnion {
                pub some_field: ::core::ffi::c_int,
                pub some_bigger_field: ::core::ffi::c_longlong,
            }
        }
    );
    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___ZN9SomeUnionC1Ev(union SomeUnion*__this) {...}
        }
    );
    assert_cc_matches!(rs_api_impl, quote! { static_assert(CRUBIT_SIZEOF(union SomeUnion)==8) });
    assert_cc_matches!(rs_api_impl, quote! { static_assert(alignof(union SomeUnion)==8) });
    assert_cc_matches!(
        rs_api_impl,
        quote! { static_assert(CRUBIT_OFFSET_OF(some_field, union SomeUnion)==0) }
    );
    assert_cc_matches!(
        rs_api_impl,
        quote! { static_assert(CRUBIT_OFFSET_OF(some_bigger_field, union SomeUnion)==0) }
    );
    Ok(())
}

#[gtest]
fn test_union_with_opaque_field() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        union MyUnion {
            char first_field[56];
            int second_field;
          };
        "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;

    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(C, align(4))]
            #[doc="CRUBIT_ANNOTATE: cpp_type=MyUnion"]
            pub union MyUnion { ...
                first_field: [::core::mem::MaybeUninit<u8>; 56],
                pub second_field: ::core::ffi::c_int,
            }
        }
    );

    assert_rs_matches!(
        rs_api,
        quote! {
            const _: () = {
                ...
                assert!(::core::mem::size_of::<crate::MyUnion>() == 56);
                assert!(::core::mem::align_of::<crate::MyUnion>() == 4);
                ...
            };
        }
    );
    Ok(())
}

#[gtest]
fn test_currently_no_offset_assertions_for_unions() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        union SomeUnion {
            int some_field;
            long long some_bigger_field;
        };
        "#,
    )?;
    let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;

    assert_rs_matches!(
        rs_api,
        quote! {
            const _: () = {
                ...
                assert!(::core::mem::offset_of!(
                    crate::SomeUnion, some_field) == 0);
                assert!(::core::mem::offset_of!(
                    crate::SomeUnion, some_bigger_field) == 0);
                ...
            };
        }
    );
    Ok(())
}

#[gtest]
fn test_union_with_private_fields() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        union SomeUnionWithPrivateFields {
          public:
            int public_field;
          private:
            long long private_field;
        };
        "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;

    assert_rs_matches!(
        rs_api,
        quote! {
            #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
            #[repr(C, align(8))]
            #[doc="CRUBIT_ANNOTATE: cpp_type=SomeUnionWithPrivateFields"]
            pub union SomeUnionWithPrivateFields {
                pub public_field: ::core::ffi::c_int,
                #[doc = " Reason for representing this field as a blob of bytes:\n Types of non-public C++ fields can be elided away"]
                pub(crate) private_field: [::core::mem::MaybeUninit<u8>; 8],
            }
        }
    );

    assert_rs_matches!(
        rs_api,
        quote! {
            const _: () = {
                ...
                assert!(::core::mem::size_of::<crate::SomeUnionWithPrivateFields>() == 8);
                assert!(::core::mem::align_of::<crate::SomeUnionWithPrivateFields>() == 8);
                static_assertions::assert_impl_all!(crate::SomeUnionWithPrivateFields: Copy,Clone);
                static_assertions::assert_not_impl_any!(crate::SomeUnionWithPrivateFields: Drop);
                ...
            };
        }
    );
    Ok(())
}

#[gtest]
fn test_nontrivial_unions() -> Result<()> {
    let ir = ir_from_cc_dependency(
        r#"
        union UnionWithNontrivialField {
            NonTrivialStruct my_field;
        };
        "#,
        r#"
        struct NonTrivialStruct {
            NonTrivialStruct(NonTrivialStruct&&);
        };
        "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;

    assert_rs_not_matches!(rs_api, quote! {derive ( ... Copy ... )});
    assert_rs_not_matches!(rs_api, quote! {derive ( ... Clone ... )});
    assert_rs_matches!(
        rs_api,
        quote! {
            #[::ctor::recursively_pinned]
            #[repr(C)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=UnionWithNontrivialField"]
            pub union UnionWithNontrivialField { ... }
        }
    );
    Ok(())
}

#[gtest]
fn test_empty_struct() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        struct EmptyStruct final {};
        "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;

    assert_rs_matches!(
        rs_api,
        quote! {
            #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
            #[repr(C)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=EmptyStruct"]
            pub struct EmptyStruct {
                ...
                __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
            }
        }
    );

    assert_rs_matches!(
        rs_api,
        quote! {
            const _: () = {
                ...
                assert!(::core::mem::size_of::<crate::EmptyStruct>() == 1);
                assert!(::core::mem::align_of::<crate::EmptyStruct>() == 1);
                ...
            };
        }
    );

    Ok(())
}

#[gtest]
fn test_empty_union() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        union EmptyUnion {};
        "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;

    assert_rs_matches!(
        rs_api,
        quote! {
            #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
            #[repr(C)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=EmptyUnion"]
            pub union EmptyUnion {
                ...
                __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
            }
        }
    );

    assert_rs_matches!(
        rs_api,
        quote! {
            const _: () = {
                ...
                assert!(::core::mem::size_of::<crate::EmptyUnion>() == 1);
                assert!(::core::mem::align_of::<crate::EmptyUnion>() == 1);
                ...
            };
        }
    );

    Ok(())
}

#[gtest]
fn test_union_field_with_nontrivial_destructor() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        struct NontrivialStruct { ~NontrivialStruct(); };
        union UnionWithNontrivialField {
            int trivial_field;
            NontrivialStruct nontrivial_field;
        };
        "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;

    assert_rs_matches!(
        rs_api,
        quote! {
            #[repr(C)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=UnionWithNontrivialField"]
            pub union UnionWithNontrivialField {
                pub trivial_field: ::core::ffi::c_int,
                pub nontrivial_field: ::core::mem::ManuallyDrop<crate::NontrivialStruct>,
            }
        }
    );

    assert_rs_matches!(
        rs_api,
        quote! {
            const _: () = {
                ...
                assert!(::core::mem::size_of::<crate::UnionWithNontrivialField>() == 4);
                assert!(::core::mem::align_of::<crate::UnionWithNontrivialField>() == 4);
                ...
            };
        }
    );
    Ok(())
}

#[gtest]
fn test_union_with_constructors() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        #pragma clang lifetime_elision
        union UnionWithDefaultConstructors {
            int a;
        };
        "#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;

    assert_rs_matches!(
        rs_api,
        quote! {
            #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
            #[repr(C)]
            #[doc="CRUBIT_ANNOTATE: cpp_type=UnionWithDefaultConstructors"]
            pub union UnionWithDefaultConstructors {
                pub a: ::core::ffi::c_int,
            }
        }
    );

    assert_rs_matches!(
        rs_api,
        quote! {
            impl Default for UnionWithDefaultConstructors {
                #[inline(always)]
                fn default() -> Self {
                    let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                    unsafe {
                        crate::detail::__rust_thunk___ZN28UnionWithDefaultConstructorsC1Ev(&raw mut tmp as *mut _);
                        tmp.assume_init()
                    }
                }
            }
        }
    );
    Ok(())
}

#[gtest]
fn test_unambiguous_public_bases() -> Result<()> {
    let ir = ir_from_cc_dependency(
        "
        struct VirtualBase {};
        struct PrivateBase {};
        struct ProtectedBase {};
        struct UnambiguousPublicBase {};
        struct AmbiguousPublicBase {};
        struct MultipleInheritance : UnambiguousPublicBase, AmbiguousPublicBase {};
        struct Derived : private PrivateBase, protected ProtectedBase, MultipleInheritance, AmbiguousPublicBase, virtual VirtualBase {};
    ",
        "",
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            unsafe impl oops::Inherits<crate::VirtualBase> for crate::Derived {
                unsafe fn upcast_ptr(derived: *const Self) -> *const crate::VirtualBase {
                    crate::detail::__crubit_dynamic_upcast__7Derived__to__11VirtualBase___2f_2ftest_3atesting_5ftarget(derived)
                }
            }
        }
    );
    assert_rs_matches!(
        rs_api,
        quote! { unsafe impl oops::Inherits<crate::UnambiguousPublicBase> for crate::Derived }
    );
    assert_rs_matches!(
        rs_api,
        quote! { unsafe impl oops::Inherits<crate::MultipleInheritance> for crate::Derived }
    );
    assert_rs_not_matches!(
        rs_api,
        quote! {unsafe impl oops::Inherits<crate::PrivateBase> for crate::Derived}
    );
    assert_rs_not_matches!(
        rs_api,
        quote! {unsafe impl oops::Inherits<crate::ProtectedBase> for crate::Derived}
    );
    assert_rs_not_matches!(
        rs_api,
        quote! {unsafe impl oops::Inherits<crate::AmbiguousPublicBase> for crate::Derived}
    );
    Ok(())
}

/// Contrary to intuitions: a base class conversion is ambiguous even if the
/// ambiguity is from a private base class cast that you can't even
/// perform.
///
/// Explanation (courtesy James Dennett):
///
/// > Once upon a time, there was a rule in C++ that changing all access
/// > specifiers to "public" would not change the meaning of code.
/// > That's no longer true, but some of its effects can still be seen.
///
/// So, we need to be sure to not allow casting to privately-ambiguous
/// bases.
#[gtest]
fn test_unambiguous_public_bases_private_ambiguity() -> Result<()> {
    let ir = ir_from_cc_dependency(
        "
        struct Base {};
        struct Intermediate : public Base {};
        struct Derived : Base, private Intermediate {};
    ",
        "",
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_not_matches!(rs_api, quote! { unsafe impl oops::Inherits<crate::Base> for Derived });
    Ok(())
}

#[gtest]
fn test_virtual_thunk() -> Result<()> {
    let ir = ir_from_cc("struct Polymorphic { virtual void Foo(); };")?;

    assert_cc_matches!(
        generate_bindings_tokens_for_test(ir)?.rs_api_impl,
        quote! {
            extern "C" void __rust_thunk___ZN11Polymorphic3FooEv(struct Polymorphic * __this)
        }
    );
    Ok(())
}

/// A trivially relocatable final struct is safe to use in Rust as normal,
/// and is Unpin.
#[gtest]
fn test_no_negative_impl_unpin() -> Result<()> {
    let ir = ir_from_cc("struct Trivial final {};")?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_not_matches!(rs_api, quote! {#[::ctor::recursively_pinned]});
    Ok(())
}

#[gtest]
fn test_no_aligned_attr() {
    let ir = ir_from_cc("struct SomeStruct {};").unwrap();
    let rs_api = generate_bindings_tokens_for_test(ir).unwrap().rs_api;

    assert_rs_matches! {rs_api, quote! {
        #[repr(C)]
        #[doc="CRUBIT_ANNOTATE: cpp_type=SomeStruct"]
        pub struct SomeStruct { ... }
    }};
}

#[gtest]
fn test_aligned_attr() {
    let ir = ir_from_cc("struct SomeStruct {} __attribute__((aligned(64)));").unwrap();
    let rs_api = generate_bindings_tokens_for_test(ir).unwrap().rs_api;

    assert_rs_matches! {rs_api, quote! {
       #[repr(C, align(64))]
        #[doc="CRUBIT_ANNOTATE: cpp_type=SomeStruct"]
       pub struct SomeStruct { ... }
      }
    };
}

#[gtest]
fn test_forward_declared() -> Result<()> {
    let ir = ir_from_cc(
        r#"#pragma clang lifetime_elision
        struct ForwardDeclared;"#,
    )?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;
    assert_rs_matches!(
        rs_api,
        quote! {
            forward_declare::forward_declare!(pub ForwardDeclared = forward_declare::symbol!("ForwardDeclared"));
        }
    );
    assert_rs_not_matches!(rs_api, quote! {struct ForwardDeclared});
    Ok(())
}

#[gtest]
fn test_private_struct_not_present() -> Result<()> {
    let ir = ir_from_cc(&with_lifetime_macros(
        r#"#pragma clang lifetime_elision
        template <typename T> class MyTemplate {};
        class HasPrivateType {
         private:
          struct PrivateType {
            using Foo = MyTemplate<PrivateType>;
            Foo* get();
          };
         protected:
          HasPrivateType(MyTemplate<PrivateType> x) {}
        };"#,
    ))?;
    let rs_api = generate_bindings_tokens_for_test(ir)?.rs_api;

    assert_rs_not_matches!(
        rs_api,
        quote! { __CcTemplateInst10MyTemplateIN14HasPrivateType11PrivateTypeEE }
    );
    Ok(())
}

#[gtest]
fn test_implicit_template_specializations_are_sorted_by_mangled_name() -> Result<()> {
    let bindings = generate_bindings_tokens_for_test(ir_from_cc(
        r#"
            template <typename T>
            struct MyStruct {
                T getT();
            };

            using Alias1 = MyStruct<int>;
            using Alias2 = MyStruct<double>;

            namespace test_namespace_bindings {
                using Alias3 = MyStruct<bool>;
            }
            "#,
    )?)?;

    // Mangled name order: bool < double < int
    let my_struct_bool = make_rs_ident("__CcTemplateInst8MyStructIbE");
    let my_struct_double = make_rs_ident("__CcTemplateInst8MyStructIdE");
    let my_struct_int = make_rs_ident("__CcTemplateInst8MyStructIiE");

    assert_rs_matches!(
        &bindings.rs_api,
        quote! {
            ...
            pub struct #my_struct_bool {...}
            ...
            pub struct #my_struct_double {...}
            ...
            pub struct #my_struct_int {...}
            ...
            const _: () = {
                ...
                assert!(::core::mem::size_of::<crate::#my_struct_bool>() == 1);
                ...
                assert!(::core::mem::size_of::<crate::#my_struct_double>() == 1);
                ...
                assert!(::core::mem::size_of::<crate::#my_struct_int>() == 1);
                ...
            }
            ...
        }
    );

    // User defined methods in mangled name order
    let my_struct_bool_method =
        make_rs_ident("__rust_thunk___ZN8MyStructIbE4getTEv__2f_2ftest_3atesting_5ftarget");
    let my_struct_double_method =
        make_rs_ident("__rust_thunk___ZN8MyStructIdE4getTEv__2f_2ftest_3atesting_5ftarget");
    let my_struct_int_method =
        make_rs_ident("__rust_thunk___ZN8MyStructIiE4getTEv__2f_2ftest_3atesting_5ftarget");

    assert_cc_matches!(
        &bindings.rs_api_impl,
        quote! {
            ...
            extern "C" bool #my_struct_bool_method(struct MyStruct<bool>*__this) {...} ...
            extern "C" double #my_struct_double_method(struct MyStruct<double>*__this) {...} ...
            extern "C" int #my_struct_int_method(struct MyStruct<int>*__this) {...} ...
        }
    );
    Ok(())
}

#[gtest]
fn test_implicit_template_specialization_namespace_qualifier() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#" #pragma clang lifetime_elision
            namespace test_namespace_bindings {
                template <typename T>
                struct MyTemplate final {
                    T value_;
                };

                using MyTypeAlias = MyTemplate<int>;
            }"#,
    )?)?
    .rs_api;

    assert_rs_matches!(
        rs_api,
        quote! {
            ...
            pub mod test_namespace_bindings {
                ...
                pub type MyTypeAlias = crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE;
                ...
            }
            ...
            pub struct __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
                pub value_: ::core::ffi::c_int,
            }
            ...
        }
    );
    Ok(())
}

#[gtest]
fn test_derived_class_inherits_unambiguous_public_functions_bases() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#"
        namespace test{
        class Base1 {
          public:
            void NonColliding();
            void Colliding();
        };

        class Base2 {
          public:
            void Colliding();
          private:
            void PrivateFunc();
        };

        class Derived : public Base1, public Base2 {
        };
        }
        "#,
    )?)?
    .rs_api;

    assert_rs_matches!(
        rs_api,
        quote! {
            ...
            impl Derived {
                ...
                #[inline(always)]
                pub unsafe fn NonColliding(__this: *mut Self) {
                    crate::detail::__rust_thunk___ZN4test5Base112NonCollidingEv(oops::UnsafeUpcast::<_>::unsafe_upcast(__this))
                }
            }
            ...
        }
    );
    Ok(())
}

#[gtest]
fn test_member_in_derived_class_overwrites_inherited_ones() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#"
        namespace test{
        class Base1 {
          public:
            void Colliding();
        };

        class Derived : public Base1 {
          public:
            void Colliding();
        };
        }
        "#,
    )?)?
    .rs_api;

    assert_rs_matches!(
        rs_api,
        quote! {
            ...
            impl Derived {
                ...
                #[inline(always)]
                pub unsafe fn Colliding(__this: *mut Self) {
                    crate::detail::__rust_thunk___ZN4test7Derived9CollidingEv(__this)
                }
            }
            ...
        }
    );
    Ok(())
}

#[gtest]
fn test_forward_declared_class_template_specialization_symbol() -> Result<()> {
    let rs_api = generate_bindings_tokens_for_test(ir_from_cc(
        r#"
        namespace test_namespace_bindings {
          template <typename T>
          struct MyTemplate {
            void processT(T t);
          };

          struct Param {};

          template<> struct MyTemplate<Param>;

          using MyTypeAlias = MyTemplate<Param>;
        }"#,
    )?)?
    .rs_api;

    assert_rs_matches!(
        rs_api,
        quote! {
            ...
            forward_declare::forward_declare!(pub __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_5ParamEEE = forward_declare::symbol!("test_namespace_bindings :: MyTemplate < test_namespace_bindings :: Param >"));
            ...
        }
    );
    Ok(())
}

/// Unsupported fields on supported structs are replaced with opaque blobs.
///
/// This is hard to test any other way than token comparison!
#[gtest]
fn test_supported_suppressed_field_types() -> Result<()> {
    // Ideally we'd use a cross-platform test, but it's hard to craft an unsupported
    // type that is still returned successfully by db.rs_type_kind(), and so
    // results in a secondary failure when we check afterwards for the
    // required features for the type.
    if multiplatform_testing::test_platform() != multiplatform_testing::Platform::X86Linux {
        return Ok(()); // vectorcall only exists on x86_64, not e.g. aarch64
    }
    let mut ir = ir_from_cc(
        r#"
        struct Trivial {
            // An example of a field which has a type that is not supported,
            // but _is_ successfully retrieved by db.rs_type_kind().
            void(*hidden_field)() [[clang::vectorcall]];
        };
    
    "#,
    )?;
    *ir.target_crubit_features_mut(&ir.current_target().clone()) =
        crubit_feature::CrubitFeature::Supported.into();
    let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
        struct Trivial {
            ...
            pub(crate) hidden_field: [::core::mem::MaybeUninit<u8>; 8],
            ...
        }}
    );
    Ok(())
}

/// By value nontrivial fields are replaced with opaque blobs, even if
/// they're supported! For pointers, they are not replaced.
#[gtest]
fn test_supported_nontrivial_field() -> Result<()> {
    let mut ir = ir_from_cc(
        r#"
        struct [[clang::trivial_abi]] Inner {~Inner();};
        struct [[clang::trivial_abi]] Outer {Inner inner_field; Inner* inner_ptr_field;};
        "#,
    )?;
    *ir.target_crubit_features_mut(&ir.current_target().clone()) =
        crubit_feature::CrubitFeature::Supported.into();
    let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;
    // Note: inner is a supported type, so it isn't being replaced by a blob because
    // it's unsupporter or anything.
    assert_rs_matches!(rs_api, quote! {pub struct Inner});
    // But it _is_ being replaced by a blob!
    assert_rs_matches!(
        rs_api,
        quote! {
        pub struct Outer {
            ...
            pub(crate) inner_field: [::core::mem::MaybeUninit<u8>; 8],
            pub inner_ptr_field: *mut crate::Inner,
        }}
    );
    Ok(())
}

#[gtest]
fn test_supported_no_unique_address_field() -> Result<()> {
    let mut ir = ir_from_cc(
        r#"
        struct Struct final {
            [[no_unique_address]] char field;
        };
    "#,
    )?;
    *ir.target_crubit_features_mut(&ir.current_target().clone()) =
        crubit_feature::CrubitFeature::Supported.into();
    let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(
        rs_api,
        quote! {
            pub struct Struct {
                ...
                pub(crate) field: [::core::mem::MaybeUninit<u8>; 1],
            }
        }
    );
    assert_rs_not_matches!(rs_api, quote! {pub fn field});
    Ok(())
}

#[gtest]
fn test_nested_type_definitions() -> Result<()> {
    for nested_type in ["enum Present {};", "struct Present {};"] {
        let mut ir = ir_from_cc(&format!(
            r#"
                struct SomeStruct final {{
                    {nested_type}
                }};
                SomeStruct::Present* AlsoPresent();
            "#
        ))?;
        *ir.target_crubit_features_mut(&ir.current_target().clone()) =
            crubit_feature::CrubitFeature::Supported.into();
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens_for_test(ir)?;
        assert_rs_matches!(rs_api, quote! { Present });
        assert_rs_matches!(rs_api, quote! { AlsoPresent });
    }
    Ok(())
}

/// Unlike other nested type definitions, typedefs can use the aliased type
/// instead.
#[gtest]
fn test_typedef_member() -> Result<()> {
    let ir = ir_from_cc(
        r#"
        struct SomeStruct final {
          typedef int Type;
        };
        inline SomeStruct::Type Function() {return 0;}
    "#,
    )?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_matches!(rs_api, quote! { pub fn Function() -> crate::some_struct::Type { ... } },);

    assert_cc_matches!(
        rs_api_impl,
        quote! {
            extern "C" SomeStruct::Type __rust_thunk___Z8Functionv(){ return Function(); }
        },
    );
    Ok(())
}

#[gtest]
fn test_struct_from_other_target() -> Result<()> {
    let ir = ir_from_cc_dependency("// intentionally empty", "struct SomeStruct {};")?;
    let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens_for_test(ir)?;
    assert_rs_not_matches!(rs_api, quote! { SomeStruct });
    assert_cc_not_matches!(rs_api_impl, quote! { SomeStruct });
    Ok(())
}

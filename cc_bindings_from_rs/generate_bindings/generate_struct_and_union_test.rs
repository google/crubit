// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use quote::quote;
use test_helpers::{test_format_item, test_generated_bindings};
use token_stream_matchers::{assert_cc_matches, assert_rs_matches};

/// The `test_generated_bindings_struct` test covers only a single example
/// of an ADT (struct/enum/union) that should get a C++ binding.
/// Additional coverage of how items are formatted is provided by
/// `test_format_item_..._struct_...`, `test_format_item_..._enum_...`,
/// and `test_format_item_..._union_...` tests.
///
/// We don't want to duplicate coverage already provided by
/// `test_format_item_struct_with_fields`, but we do want to verify that
/// * `format_crate` will actually find and process the struct
///   (`test_format_item_...` doesn't cover this aspect - it uses a
///   test-only `find_def_id_by_name` instead)
/// * The actual shape of the bindings still looks okay at this level.
#[test]
fn test_generated_bindings_struct() {
    let test_src = r#"
            pub struct Point {
                pub x: i32,
                pub y: i32,
            }
        "#;
    test_generated_bindings(test_src, |bindings| {
        let bindings = bindings.unwrap();
        assert_cc_matches!(
            bindings.cc_api,
            quote! {
                namespace rust_out {
                    ...
                    struct CRUBIT_INTERNAL_RUST_TYPE(":: rust_out :: Point") alignas(4) [[clang::trivial_abi]] Point final {
                        // No point replicating test coverage of
                        // `test_format_item_struct_with_fields`.
                        ...
                    };
                    static_assert(sizeof(Point) == 8, ...);
                    static_assert(alignof(Point) == 4, ...);
                    ... // Other static_asserts are covered by
                        // `test_format_item_struct_with_fields`
                }  // namespace rust_out
            }
        );
        assert_rs_matches!(
            bindings.cc_api_impl,
            quote! {
                // No point replicating test coverage of
                // `test_format_item_struct_with_fields`.
                const _: () = assert!(::std::mem::size_of::<::rust_out::Point>() == 8);
                const _: () = assert!(::std::mem::align_of::<::rust_out::Point>() == 4);
                const _: () = assert!(::core::mem::offset_of!(::rust_out::Point, x) == 0);
                const _: () = assert!(::core::mem::offset_of!(::rust_out::Point, y) == 4);
            }
        );
    });
}

#[test]
fn test_format_bridged_type_in_generic_types() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_type = cpp_ns::CppType"]
            #[doc="CRUBIT_ANNOTATE: include_path = cpp_ns/cpp_type.h"]
            #[doc="CRUBIT_ANNOTATE: rust_to_cpp_converter = convert_rust_to_cpp_type"]
            #[doc="CRUBIT_ANNOTATE: cpp_to_rust_converter = convert_cpp_to_rust_type"]
            pub struct RustType {
                pub x: i32,
            }

            #[unsafe(no_mangle)]
            pub fn foo(_: Box<RustType>) {}

            #[unsafe(no_mangle)]
            pub fn bar(_: Option<Box<Result<RustType, ()>>>) {}
    "#;
    test_format_item(test_src, "foo", |result| {
        let err = result.unwrap_err();
        assert_eq!(
            err,
            "Error handling parameter #0 of type `std::boxed::Box<RustType>`: \
            Can't format ADT as it has a generic type `RustType` that is a bridged type"
        );
    });

    test_format_item(test_src, "bar", |result| {
        let err = result.unwrap_err();
        assert_eq!(
            err,
            "Error handling parameter #0 of type `std::option::Option<std::boxed::Box<std::result::Result<RustType, ()>>>`: Can't format ADT as it has a generic type \
                `RustType` that is a bridged type"
        );
    });
}

#[test]
fn test_format_struct_cpp_name() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_name=Bar"]
            pub struct Foo {
                pub x: i32,
            }
        "#;
    test_format_item(test_src, "Foo", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());

        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                const _: () = assert!(::std::mem::size_of::<::rust_out::Foo>() == 4);
                const _: () = assert!(::std::mem::align_of::<::rust_out::Foo>() == 4);
                const _: () = assert!(::core::mem::offset_of!(::rust_out::Foo, x) == 0);
            }
        );

        assert_cc_matches!(
            main_api.tokens,
            quote! {
                struct CRUBIT_INTERNAL_RUST_TYPE(":: rust_out :: Foo") alignas(4)
                [[clang::trivial_abi]] Bar final
            }
        );
    });
}

#[test]
fn test_format_item_unsupported_type_generic_struct() {
    let test_src = r#"
            pub struct Point<T> {
                pub x: T,
                pub y: T,
            }
        "#;
    test_format_item(test_src, "Point", |result| {
        let err = result.unwrap_err();
        assert_eq!(err, "Generic types are not supported yet (b/259749095)");
    });
}

#[test]
fn test_format_item_unsupported_lifetime_generic_struct() {
    let test_src = r#"
            pub struct Point<'a> {
                pub x: &'a i32,
                pub y: &'a i32,
            }

            impl<'a> Point<'a> {
                // Some lifetimes are bound at the `impl` / `struct` level (the lifetime is
                // hidden underneath the `Self` type), and some at the `fn` level.
                pub fn new<'b, 'c>(_x: &'b i32, _y: &'c i32) -> Self { unimplemented!() }
            }
        "#;
    test_format_item(test_src, "Point", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());

        assert_cc_matches!(
            main_api.tokens,
            quote! {
                struct CRUBIT_INTERNAL_RUST_TYPE(":: rust_out :: Point") alignas(8)
                [[clang::trivial_abi]] Point final
                {
                 public:
                  ...
                  static ::rust_out::Point new_(
                    std::int32_t const* [[clang::annotate_type("lifetime", "b")]] _x,
                    std::int32_t const* [[clang::annotate_type("lifetime", "c")]] _y);
                  ...
                };
            }
        );
    });
}

#[test]
fn test_format_item_unsupported_generic_enum() {
    let test_src = r#"
            pub enum Point<T> {
                Cartesian{x: T, y: T},
                Polar{angle: T, dist: T},
            }
        "#;
    test_format_item(test_src, "Point", |result| {
        let err = result.unwrap_err();
        assert_eq!(err, "Generic types are not supported yet (b/259749095)");
    });
}

#[test]
fn test_format_item_unsupported_generic_union() {
    let test_src = r#"
            pub union SomeUnion<T> {
                pub x: std::mem::ManuallyDrop<T>,
                pub y: i32,
            }
        "#;
    test_format_item(test_src, "SomeUnion", |result| {
        let err = result.unwrap_err();
        assert_eq!(err, "Generic types are not supported yet (b/259749095)");
    });
}

/// This is a test for a regular struct - a struct with named fields.
/// https://doc.rust-lang.org/reference/items/structs.html refers to this kind of struct as
/// `StructStruct` or "nominal struct type".
#[test]
fn test_format_item_struct_with_fields() {
    let test_src = r#"
            pub struct SomeStruct {
                pub x: i32,
                pub y: i32,
            }

            const _: () = assert!(std::mem::size_of::<SomeStruct>() == 8);
            const _: () = assert!(std::mem::align_of::<SomeStruct>() == 4);
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] SomeStruct final {
                    public:
                        __COMMENT__ "`SomeStruct` doesn't implement the `Default` trait"
                        SomeStruct() = delete;

                        __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                        ~SomeStruct() = default;
                        SomeStruct(SomeStruct&&) = default;
                        SomeStruct& operator=(SomeStruct&&) = default;

                        __COMMENT__ "`SomeStruct` doesn't implement the `Clone` trait"
                        SomeStruct(const SomeStruct&) = delete;
                        SomeStruct& operator=(const SomeStruct&) = delete;

                        SomeStruct(::crubit::UnsafeRelocateTag, SomeStruct&& value) {
                          memcpy(this, &value, sizeof(value));
                        }
                    public:
                        union { ... std::int32_t x; };
                        union { ... std::int32_t y; };
                    private:
                        static void __crubit_field_offset_assertions();
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                static_assert(sizeof(SomeStruct) == 8, ...);
                static_assert(alignof(SomeStruct) == 4, ...);
                static_assert(std::is_trivially_destructible_v<SomeStruct>);
                static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
                static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
                inline void SomeStruct::__crubit_field_offset_assertions() {
                  static_assert(0 == offsetof(SomeStruct, x));
                  static_assert(4 == offsetof(SomeStruct, y));
                }
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                const _: () = assert!(::std::mem::size_of::<::rust_out::SomeStruct>() == 8);
                const _: () = assert!(::std::mem::align_of::<::rust_out::SomeStruct>() == 4);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, x) == 0);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, y) == 4);
            }
        );
    });
}

/// This is a test for `TupleStruct` or "tuple struct" - for more details
/// please refer to https://doc.rust-lang.org/reference/items/structs.html
#[test]
fn test_format_item_struct_with_tuple() {
    let test_src = r#"
            pub struct TupleStruct(pub i32, pub i32);
            const _: () = assert!(std::mem::size_of::<TupleStruct>() == 8);
            const _: () = assert!(std::mem::align_of::<TupleStruct>() == 4);
        "#;
    test_format_item(test_src, "TupleStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] TupleStruct final {
                    public:
                        __COMMENT__ "`TupleStruct` doesn't implement the `Default` trait"
                        TupleStruct() = delete;

                        __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                        ~TupleStruct() = default;
                        TupleStruct(TupleStruct&&) = default;
                        TupleStruct& operator=(TupleStruct&&) = default;

                        __COMMENT__ "`TupleStruct` doesn't implement the `Clone` trait"
                        TupleStruct(const TupleStruct&) = delete;
                        TupleStruct& operator=(const TupleStruct&) = delete;
                        TupleStruct(::crubit::UnsafeRelocateTag, TupleStruct&& value) {
                          memcpy(this, &value, sizeof(value));
                        }
                    public:
                        union { ... std::int32_t __field0; };
                        union { ... std::int32_t __field1; };
                    private:
                        static void __crubit_field_offset_assertions();
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                static_assert(sizeof(TupleStruct) == 8, ...);
                static_assert(alignof(TupleStruct) == 4, ...);
                static_assert(std::is_trivially_destructible_v<TupleStruct>);
                static_assert(std::is_trivially_move_constructible_v<TupleStruct>);
                static_assert(std::is_trivially_move_assignable_v<TupleStruct>);
                inline void TupleStruct::__crubit_field_offset_assertions() {
                  static_assert(0 == offsetof(TupleStruct, __field0));
                  static_assert(4 == offsetof(TupleStruct, __field1));
                }
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                const _: () = assert!(::std::mem::size_of::<::rust_out::TupleStruct>() == 8);
                const _: () = assert!(::std::mem::align_of::<::rust_out::TupleStruct>() == 4);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::TupleStruct, 0) == 0);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::TupleStruct, 1) == 4);
            }
        );
    });
}

/// This test the scenario where Rust lays out field in a different order
/// than the source order.
#[test]
fn test_format_item_struct_with_reordered_field_offsets() {
    let test_src = r#"
            pub struct SomeStruct {
                pub field1: i16,
                pub field2: i32,
                pub field3: i16,
            }

            const _: () = assert!(std::mem::size_of::<SomeStruct>() == 8);
            const _: () = assert!(std::mem::align_of::<SomeStruct>() == 4);
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] SomeStruct final {
                    ...
                    // The particular order below is not guaranteed,
                    // so we may need to adjust this test assertion
                    // (if Rust changes how it lays out the fields).
                    public:
                        union { ... std::int32_t field2; };
                        union { ... std::int16_t field1; };
                        union { ... std::int16_t field3; };
                    private:
                        static void __crubit_field_offset_assertions();
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                static_assert(sizeof(SomeStruct) == 8, ...);
                static_assert(alignof(SomeStruct) == 4, ...);
                static_assert(std::is_trivially_destructible_v<SomeStruct>);
                static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
                static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
                inline void SomeStruct::__crubit_field_offset_assertions() {
                  static_assert(0 == offsetof(SomeStruct, field2));
                  static_assert(4 == offsetof(SomeStruct, field1));
                  static_assert(6 == offsetof(SomeStruct, field3));
                }
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                const _: () = assert!(::std::mem::size_of::<::rust_out::SomeStruct>() == 8);
                const _: () = assert!(::std::mem::align_of::<::rust_out::SomeStruct>() == 4);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, field2)
                                       == 0);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, field1)
                                       == 4);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, field3)
                                       == 6);
            }
        );
    });
}

/// Tuple fields must not be bridged to std::tuple, because that is not layout-compatible.
#[test]
fn test_format_item_struct_with_tuple_fields() {
    let test_src = r#"
            pub struct SomeStruct {
                pub tuple_field: (i32,),
                pub empty_tuple_field: (),
            }
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                {
                    ...
                    unsigned char tuple_field[4];
                    __COMMENT__ "Skipped bindings for field `empty_tuple_field`: ZST fields are not supported (b/258259459)"
                    ...
                }
            }
        );
    });
}

#[test]
fn test_format_item_struct_with_packed_layout() {
    let test_src = r#"
            #[repr(packed(1))]
            pub struct SomeStruct {
                pub field1: u16,
                pub field2: u32,
            }
            const _: () = assert!(::std::mem::size_of::<SomeStruct>() == 6);
            const _: () = assert!(::std::mem::align_of::<SomeStruct>() == 1);
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(1) [[clang::trivial_abi]] __attribute__((packed)) SomeStruct final {
                    ...
                    public:
                        union { ... std::uint16_t field1; };
                        union { ... std::uint32_t field2; };
                    private:
                        static void __crubit_field_offset_assertions();
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                static_assert(sizeof(SomeStruct) == 6, ...);
                static_assert(alignof(SomeStruct) == 1, ...);
                static_assert(std::is_trivially_destructible_v<SomeStruct>);
                static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
                static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
                inline void SomeStruct::__crubit_field_offset_assertions() {
                  static_assert(0 == offsetof(SomeStruct, field1));
                  static_assert(2 == offsetof(SomeStruct, field2));
                }
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                const _: () = assert!(::std::mem::size_of::<::rust_out::SomeStruct>() == 6);
                const _: () = assert!(::std::mem::align_of::<::rust_out::SomeStruct>() == 1);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, field1)
                                       == 0);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, field2)
                                       == 2);
            }
        );
    });
}

#[test]
fn test_format_item_struct_with_explicit_padding_in_generated_code() {
    let test_src = r#"
            pub struct SomeStruct {
                pub f1: u8,
                pub f2: u32,
            }
            const _: () = assert!(::std::mem::size_of::<SomeStruct>() == 8);
            const _: () = assert!(::std::mem::align_of::<SomeStruct>() == 4);
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] SomeStruct final {
                    ...
                    public:
                        union { ... std::uint32_t f2; };
                        union { ... std::uint8_t f1; };
                    private: unsigned char __padding0[3];
                    private:
                        static void __crubit_field_offset_assertions();
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                static_assert(sizeof(SomeStruct) == 8, ...);
                static_assert(alignof(SomeStruct) == 4, ...);
                static_assert(std::is_trivially_destructible_v<SomeStruct>);
                static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
                static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
                inline void SomeStruct::__crubit_field_offset_assertions() {
                  static_assert(0 == offsetof(SomeStruct, f2));
                  static_assert(4 == offsetof(SomeStruct, f1));
                }
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                const _: () = assert!(::std::mem::size_of::<::rust_out::SomeStruct>() == 8);
                const _: () = assert!(::std::mem::align_of::<::rust_out::SomeStruct>() == 4);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, f2) == 0);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, f1) == 4);
            }
        );
    });
}

#[test]
fn test_format_item_struct_with_explicit_padding_on_private_field_in_generated_code() {
    let test_src = r#"
            pub struct SomeStruct {
                #[allow(dead_code)]
                f1: u8,
                pub f2: u32,
            }
            const _: () = assert!(::std::mem::size_of::<SomeStruct>() == 8);
            const _: () = assert!(::std::mem::align_of::<SomeStruct>() == 4);
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] SomeStruct final {
                    ...
                    public:
                        union { ... std::uint32_t f2; };
                    private:
                        union { ... std::uint8_t f1; };
                        unsigned char __padding0[3];
                    private:
                        static void __crubit_field_offset_assertions();
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                static_assert(sizeof(SomeStruct) == 8, ...);
                static_assert(alignof(SomeStruct) == 4, ...);
                static_assert(std::is_trivially_destructible_v<SomeStruct>);
                static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
                static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
                inline void SomeStruct::__crubit_field_offset_assertions() {
                  static_assert(0 == offsetof(SomeStruct, f2));
                  static_assert(4 == offsetof(SomeStruct, f1));
                }
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                const _: () = assert!(::std::mem::size_of::<::rust_out::SomeStruct>() == 8);
                const _: () = assert!(::std::mem::align_of::<::rust_out::SomeStruct>() == 4);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, f2) == 0);
            }
        );
    });
}

#[test]
fn test_format_item_unsupported_struct_with_name_that_is_reserved_keyword() {
    let test_src = r#"
            #[allow(non_camel_case_types)]
            pub struct reinterpret_cast {
                pub x: i32,
                pub y: i32,
            }
        "#;
    test_format_item(test_src, "reinterpret_cast", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                struct ... reinterpret_cast_ final
            }
        );
    });
}

#[test]
fn test_format_item_struct_with_unsupported_field_type() {
    let test_src = r#"
            pub struct SomeStruct {
                pub successful_field: i32,
                pub unsupported_field: Option<[i32; 3]>,
            }
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let broken_field_msg = "Field type has been replaced with a blob of bytes: \
                                Generic types are not supported yet (b/259749095)";
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... SomeStruct final {
                    ...
                    private:
                        __COMMENT__ #broken_field_msg
                        unsigned char unsupported_field[16];
                    public:
                        union { ... std::int32_t successful_field; };
                    private:
                        static void __crubit_field_offset_assertions();
                };
                ...
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                static_assert(sizeof(SomeStruct) == 20, ...);
                static_assert(alignof(SomeStruct) == 4, ...);
                static_assert(std::is_trivially_destructible_v<SomeStruct>);
                static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
                static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
                inline void SomeStruct::__crubit_field_offset_assertions() {
                  static_assert(0 == offsetof(SomeStruct, unsupported_field));
                  static_assert(16 == offsetof(SomeStruct, successful_field));
                }
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                const _: () = assert!(::std::mem::size_of::<::rust_out::SomeStruct>() == 20);
                const _: () = assert!(::std::mem::align_of::<::rust_out::SomeStruct>() == 4);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct,
                                                             unsupported_field) == 0);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct,
                                                             successful_field) == 16);
            }
        );
    });
}

/// This test verifies how reference type fields are represented in the
/// generated bindings.  See b/286256327.
///
/// In some of the past discussions we tentatively decided that the
/// generated bindings shouldn't use C++ references in fields - instead
/// a C++ pointer should be used.  One reason is that C++ references
/// cannot be assigned to (i.e. rebound), and therefore C++ pointers
/// more accurately represent the semantics of Rust fields.  The pointer
/// type should probably use some form of C++ annotations to mark it as
/// non-nullable.
#[test]
fn test_format_item_struct_with_unsupported_field_of_reference_type() {
    let test_src = r#"
            // `'static` lifetime can be used in a non-generic struct - this let's us
            // test reference fieles without requiring support for generic structs.
            pub struct NonGenericSomeStruct {
                pub reference_field: &'static i32,
            }
        "#;
    test_format_item(test_src, "NonGenericSomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let broken_field_msg = "Field type has been replaced with a blob of bytes: \
                                Can't format `&'static i32`, because references \
                                are only supported in function parameter types, \
                                return types, and consts (b/286256327)";
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                private:
                    __COMMENT__ #broken_field_msg
                    unsigned char reference_field[8];
                ...
            }
        );
    });
}

/// This test verifies that `generate_trait_thunks(..., drop_trait_id,
/// ...).expect(...)` won't panic - the `generate_adt_core` needs to
/// verify that formatting of the fully qualified C++ name of the struct
/// works fine.
#[test]
fn test_format_item_unsupported_struct_with_custom_drop_impl_in_reserved_name_module() {
    let test_src = r#"
            // This mimics the name of a public module used by
            // `icu_locid` in `extensions/mod.rs`.
            pub mod private {
                #[derive(Default)]
                pub struct SomeStruct {
                    pub x: i32,
                    pub y: i32,
                }

                impl Drop for SomeStruct {
                    fn drop(&mut self) {}
                }
            }
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let cc_details = &result.cc_details;
        assert_cc_matches!(
            cc_details.tokens,
            quote! {
                ::rust_out::private_::SomeStruct
            }
        );
    });
}

/// This test covers how ZSTs (zero-sized-types) are handled.
/// https://doc.rust-lang.org/reference/items/structs.html refers to this kind of struct as a
/// "unit-like struct".
#[test]
fn test_format_item_unsupported_struct_zero_sized_type_with_no_fields() {
    let test_src = r#"
            pub struct ZeroSizedType1;
            pub struct ZeroSizedType2();
            pub struct ZeroSizedType3{}
        "#;
    for name in ["ZeroSizedType1", "ZeroSizedType2", "ZeroSizedType3"] {
        test_format_item(test_src, name, |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Zero-sized types (ZSTs) are not supported (b/258259459)");
        });
    }
}

#[test]
fn test_format_item_unsupported_struct_with_only_zero_sized_type_fields() {
    let test_src = r#"
            pub struct ZeroSizedType;
            pub struct SomeStruct {
                pub zst1: ZeroSizedType,
                pub zst2: ZeroSizedType,
            }
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let err = result.unwrap_err();
        assert_eq!(err, "Zero-sized types (ZSTs) are not supported (b/258259459)",);
    });
}

#[test]
fn test_format_item_unsupported_struct_with_some_zero_sized_type_fields() {
    let test_src = r#"
            pub struct ZeroSizedType;
            pub struct SomeStruct {
                pub zst1: ZeroSizedType,
                pub successful_field: i32,
                pub zst2: ZeroSizedType,
            }
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let broken_field_msg_zst1 =
            "Skipped bindings for field `zst1`: ZST fields are not supported (b/258259459)";
        let broken_field_msg_zst2 =
            "Skipped bindings for field `zst2`: ZST fields are not supported (b/258259459)";

        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... SomeStruct final {
                    ...
                    public:
                        union { ... std::int32_t successful_field; };
                    __COMMENT__ #broken_field_msg_zst1
                    __COMMENT__ #broken_field_msg_zst2
                    private:
                        static void __crubit_field_offset_assertions();
                };
                ...
            }
        );

        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                static_assert(sizeof(SomeStruct) == 4, ...);
                static_assert(alignof(SomeStruct) == 4, ...);
                static_assert(std::is_trivially_destructible_v<SomeStruct>);
                static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
                static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
                inline void SomeStruct::__crubit_field_offset_assertions() {
                static_assert(0 == offsetof(SomeStruct, successful_field));
                }
            }
        );

        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                const _: () = assert!(::std::mem::size_of::<::rust_out::SomeStruct>() == 4);
                const _: () = assert!(::std::mem::align_of::<::rust_out::SomeStruct>() == 4);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, successful_field) == 0);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, zst1) == 4);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeStruct, zst2) == 4);

            }
        );
    });
}

#[test]
fn test_format_item_struct_with_dynamically_sized_field() {
    let test_src = r#"
            #![allow(dead_code)]
            pub struct DynamicallySizedStruct {
                /// Having a non-ZST field avoids hitting the following error:
                /// "Zero-sized types (ZSTs) are not supported (b/258259459)"
                _non_zst_field: f32,
                _dynamically_sized_field: [i32],
            }
        "#;
    test_format_item(test_src, "DynamicallySizedStruct", |result| {
        let err = result.unwrap_err();
        assert_eq!(err, "Bindings for dynamically sized types are not supported.");
    });
}

#[test]
fn test_format_item_struct_fields_with_doc_comments() {
    let test_src = r#"
            pub struct SomeStruct {
                /// Documentation of `successful_field`.
                pub successful_field: i32,

                /// Documentation of `unsupported_field`.
                pub unsupported_field: Option<[i32; 3]>,
            }
        "#;
    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let comment_for_successful_field = " Documentation of `successful_field`.\n\n\
              Generated from: <crubit_unittests.rs>;l=4";
        let comment_for_unsupported_field = "Field type has been replaced with a blob of bytes: \
             Generic types are not supported yet (b/259749095)";
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... SomeStruct final {
                    ...
                    private:
                        __COMMENT__ #comment_for_unsupported_field
                        unsigned char unsupported_field[16];
                    public:
                        union {
                            __COMMENT__ #comment_for_successful_field
                            std::int32_t successful_field;
                        };
                    private:
                        static void __crubit_field_offset_assertions();
                };
                ...
            }
        );
    });
}

/// This is a test for an enum that only has `EnumItemDiscriminant` items
/// (and doesn't have `EnumItemTuple` or `EnumItemStruct` items).  See
/// also https://doc.rust-lang.org/reference/items/enumerations.html
#[test]
fn test_format_item_enum_with_only_discriminant_items() {
    let test_src = r#"
            pub enum SomeEnum {
                Red,
                Green = 123,
                Blue,
            }

            const _: () = assert!(std::mem::size_of::<SomeEnum>() == 1);
            const _: () = assert!(std::mem::align_of::<SomeEnum>() == 1);
        "#;
    test_format_item(test_src, "SomeEnum", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let no_fields_msg = "Field type has been replaced with a blob of bytes: \
                             No support for bindings of individual non-repr(C) `enum`s";
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(1) [[clang::trivial_abi]] SomeEnum final {
                    public:
                        __COMMENT__ "`SomeEnum` doesn't implement the `Default` trait"
                        SomeEnum() = delete;

                        __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                        ~SomeEnum() = default;
                        SomeEnum(SomeEnum&&) = default;
                        SomeEnum& operator=(SomeEnum&&) = default;

                        __COMMENT__ "`SomeEnum` doesn't implement the `Clone` trait"
                        SomeEnum(const SomeEnum&) = delete;
                        SomeEnum& operator=(const SomeEnum&) = delete;

                        SomeEnum(::crubit::UnsafeRelocateTag, SomeEnum&& value) {
                          memcpy(this, &value, sizeof(value));
                        }
                    private:
                        __COMMENT__ #no_fields_msg
                        unsigned char __opaque_blob_of_bytes[1];
                    private:
                        static void __crubit_field_offset_assertions();
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                static_assert(sizeof(SomeEnum) == 1, ...);
                static_assert(alignof(SomeEnum) == 1, ...);
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                const _: () = assert!(::std::mem::size_of::<::rust_out::SomeEnum>() == 1);
                const _: () = assert!(::std::mem::align_of::<::rust_out::SomeEnum>() == 1);
            }
        );
    });
}

/// This is a test for an enum that has `EnumItemTuple` and `EnumItemStruct`
/// items. See also https://doc.rust-lang.org/reference/items/enumerations.html
#[test]
fn test_format_item_enum_with_tuple_and_struct_items() {
    let test_src = r#"
            pub enum Point {
                Cartesian(f32, f32),
                Polar{ dist: f32, angle: f32 },
            }

            const _: () = assert!(std::mem::size_of::<Point>() == 12);
            const _: () = assert!(std::mem::align_of::<Point>() == 4);
        "#;
    test_format_item(test_src, "Point", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let no_fields_msg = "Field type has been replaced with a blob of bytes: \
                             No support for bindings of individual non-repr(C) `enum`s";
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct CRUBIT_INTERNAL_RUST_TYPE(...) alignas(4) [[clang::trivial_abi]] Point final {
                    public:
                        __COMMENT__ "`Point` doesn't implement the `Default` trait"
                        Point() = delete;

                        __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                        ~Point() = default;
                        Point(Point&&) = default;
                        Point& operator=(Point&&) = default;

                        __COMMENT__ "`Point` doesn't implement the `Clone` trait"
                        Point(const Point&) = delete;
                        Point& operator=(const Point&) = delete;

                        Point(::crubit::UnsafeRelocateTag, Point&& value) {
                          memcpy(this, &value, sizeof(value));
                        }
                    private:
                        __COMMENT__ #no_fields_msg
                        unsigned char __opaque_blob_of_bytes[12];
                    private:
                        static void __crubit_field_offset_assertions();
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                static_assert(sizeof(Point) == 12, ...);
                static_assert(alignof(Point) == 4, ...);
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                const _: () = assert!(::std::mem::size_of::<::rust_out::Point>() == 12);
                const _: () = assert!(::std::mem::align_of::<::rust_out::Point>() == 4);
            }
        );
    });
}

/// This test covers how zero-variant enums are handled.  See also
/// https://doc.rust-lang.org/reference/items/enumerations.html#zero-variant-enums
#[test]
fn test_format_item_unsupported_enum_zero_variants() {
    let test_src = r#"
            pub enum ZeroVariantEnum {}
        "#;
    test_format_item(test_src, "ZeroVariantEnum", |result| {
        let err = result.unwrap_err();
        assert_eq!(err, "Zero-sized types (ZSTs) are not supported (b/258259459)");
    });
}

/// This is a test for a `union`.  See also
/// https://doc.rust-lang.org/reference/items/unions.html
#[test]
fn test_format_item_union() {
    let test_src = r#"
            pub union SomeUnion {
                pub i: i32,
                pub f: f64,
            }

            const _: () = assert!(std::mem::size_of::<SomeUnion>() == 8);
            const _: () = assert!(std::mem::align_of::<SomeUnion>() == 8);
        "#;
    test_format_item(test_src, "SomeUnion", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                union CRUBIT_INTERNAL_RUST_TYPE(...) alignas(8) [[clang::trivial_abi]] SomeUnion final {
                    public:
                        __COMMENT__ "`SomeUnion` doesn't implement the `Default` trait"
                        SomeUnion() = delete;

                        __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                        ~SomeUnion() = default;
                        SomeUnion(SomeUnion&&) = default;
                        SomeUnion& operator=(SomeUnion&&) = default;

                        __COMMENT__ "`SomeUnion` doesn't implement the `Clone` trait"
                        SomeUnion(const SomeUnion&) = delete;
                        SomeUnion& operator=(const SomeUnion&) = delete;
                    ...
                    struct {
                        ...
                        std::int32_t value;
                    } i;
                    ...
                    struct {
                        ...
                        double value;
                    } f;
                    private:
                        static void __crubit_field_offset_assertions();
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                static_assert(sizeof(SomeUnion) == 8, ...);
                static_assert(alignof(SomeUnion) == 8, ...);
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                const _: () = assert!(::std::mem::size_of::<::rust_out::SomeUnion>() == 8);
                const _: () = assert!(::std::mem::align_of::<::rust_out::SomeUnion>() == 8);
            }
        );
    });
}

#[test]
fn test_format_item_doc_comments_union() {
    let test_src = r#"
        /// Doc for some union.
        pub union SomeUnionWithDocs {
            /// Doc for a field in a union.
            pub i: i32,
            pub f: f64
        }
    "#;
    test_format_item(test_src, "SomeUnionWithDocs", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let comment = " Doc for some union.\n\n\
                       Generated from: <crubit_unittests.rs>;l=3";
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                __COMMENT__ #comment
                union ... SomeUnionWithDocs final {
                    ...
                }
                ...
            }
        );
    });
}

#[test]
fn test_format_item_doc_comments_enum() {
    let test_src = r#"
        /** Doc for some enum. */
        pub enum SomeEnumWithDocs {
            Kind1(i32),
        }
    "#;
    test_format_item(test_src, "SomeEnumWithDocs", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let comment = " Doc for some enum. \n\n\
                        Generated from: <crubit_unittests.rs>;l=3";
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                __COMMENT__ #comment
                struct ... SomeEnumWithDocs final {
                    ...
                }
                ...
            }
        );
    });
}

#[test]
fn test_format_item_doc_comments_struct() {
    let test_src = r#"
        #![allow(dead_code)]
        #[doc = "Doc for some struct."]
        pub struct SomeStructWithDocs {
            #[doc = "Doc for first field."]
            some_field : i32,
        }
    "#;
    test_format_item(test_src, "SomeStructWithDocs", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let comment = "Doc for some struct.\n\n\
                       Generated from: <crubit_unittests.rs>;l=4";
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                __COMMENT__ #comment
                struct ... SomeStructWithDocs final {
                    ...
                }
                ...
            }
        );
    });
}

#[test]
fn test_format_item_doc_comments_tuple_struct() {
    let test_src = r#"
        #![allow(dead_code)]

        /// Doc for some tuple struct.
        pub struct SomeTupleStructWithDocs(i32);
    "#;
    test_format_item(test_src, "SomeTupleStructWithDocs", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        let comment = " Doc for some tuple struct.\n\n\
                       Generated from: <crubit_unittests.rs>;l=5";
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                __COMMENT__ #comment
                struct ... SomeTupleStructWithDocs final {
                    ...
                }
                ...
            },
        );
    });
}

#[test]
fn test_cpp_enum_plain() {
    let test_src = r#"
    #[doc="CRUBIT_ANNOTATE: cpp_enum=enum"]
    #[repr(transparent)]
    pub struct Color(i32);

    impl Color {
        pub const RED: Color = Color(0);
        pub const BLUE: Color = Color(2);
    }
    "#;

    test_format_item(test_src, "Color", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                enum ... Color : std::int32_t {
                    RED = 0,
                    BLUE = 2,
                };
            }
        );
    });
}

#[test]
fn test_cpp_enum_class() {
    let test_src = r#"
    #![feature(register_tool)]
    #![register_tool(__crubit)]

    #[doc="CRUBIT_ANNOTATE: cpp_enum=enum class"]
    #[repr(transparent)]
    pub struct Color(u8);

    impl Color {
        pub const RED: Color = Color(0);
        pub const BLUE: Color = Color(2);
    }
    "#;

    test_format_item(test_src, "Color", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                enum class ... Color : std::uint8_t {
                    RED = 0,
                    BLUE = 2,
                };
            }
        );
    });
}

#[test]
fn test_cpp_enum_with_attributes() {
    let test_src = r#"
    #![feature(register_tool)]
    #![register_tool(__crubit)]
    #![allow(deprecated)]
    #![allow(unused)]

    #[doc="CRUBIT_ANNOTATE: cpp_enum=enum class"]
    #[repr(transparent)]
    #[deprecated(note="Use NewColor")]
    #[must_use]
    pub struct Color(i32);

    impl Color {
        pub const RED: Color = Color(0);
        pub const BLUE: Color = Color(2);
    }
    "#;

    test_format_item(test_src, "Color", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                enum class ... [[nodiscard]] [[deprecated("Use NewColor")]] ... Color : std::int32_t {
                    RED = 0,
                    BLUE = 2,
                };
            }
        );
    });
}

#[test]
#[should_panic]
fn test_cpp_enum_fails_if_not_repr_transparent() {
    let test_src = r#"
    #![feature(register_tool)]
    #![register_tool(__crubit)]

    #[doc="CRUBIT_ANNOTATE: cpp_enum=enum class"]
    pub struct Color(i32);

    impl Color {
        pub const RED: Color = Color(0);
        pub const BLUE: Color = Color(2);
    }
    "#;

    test_format_item(test_src, "Color", |_result| {});
}

#[test]
#[should_panic]
fn test_cpp_enum_fails_if_implements_method() {
    let test_src = r#"
    #![feature(register_tool)]
    #![register_tool(__crubit)]

    #[doc="CRUBIT_ANNOTATE: cpp_enum=enum class"]
    #[repr(transparent)]
    pub struct Color(i32);

    impl Color {
        pub const RED: Color = Color(0);
        pub const BLUE: Color = Color(2);

        pub fn f(&self) -> i32 {
            0
        }
    }
    "#;

    test_format_item(test_src, "Color", |_result| {});
}

#[test]
#[should_panic]
fn test_cpp_enum_fails_for_rust_union() {
    let test_src = r#"
    #![feature(transparent_unions)]

    #[doc="CRUBIT_ANNOTATE: cpp_enum=enum class"]
    #[repr(transparent)]
    pub union Color {
        value: i32,
    }
    "#;

    test_format_item(test_src, "Color", |_result| {});
}

#[test]
#[should_panic]
fn test_cpp_enum_fails_for_rust_enum() {
    let test_src = r#"
    #[doc="CRUBIT_ANNOTATE: cpp_enum=enum class"]
    #[repr(transparent)]
    enum Color {
        Value(i32),
    }
    "#;

    test_format_item(test_src, "Color", |_result| {});
}

#[test]
fn test_repr_c_enum_fields() {
    let test_src = r#"
    #[repr(C, i32)]
    pub enum SomeEnum {
        A(i32),
        B{x: u32},
        C,
        D{foo: i32, bar: i32} = 3,
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
                        __COMMENT__ "`SomeEnum` doesn't implement the `Default` trait"
                        SomeEnum() = delete;
                        ...
                        __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                        ~SomeEnum() = default;
                        SomeEnum(SomeEnum&&) = default;
                        SomeEnum& operator=(SomeEnum&&) = default;

                        __COMMENT__ "`SomeEnum` doesn't implement the `Clone` trait"
                        SomeEnum(const SomeEnum&) = delete;
                        SomeEnum& operator=(const SomeEnum&) = delete;
                        ...
                        struct alignas(...) __crubit_A_struct {
                            public:
                                std::int32_t __field0;
                        };
                        ...
                        struct alignas(...) __crubit_B_struct {
                            public:
                                std::uint32_t x;
                        };
                        ...
                        __COMMENT__ "Variant C has no size, so no struct is generated."
                        ...
                        struct alignas(...) __crubit_D_struct {
                            public:
                                std::int32_t foo;
                                std::int32_t bar;
                        };
                        ...
                        enum class Tag : std::int32_t {
                            A = 0,
                            B = 1,
                            C = 2,
                            D = 3,
                        };
                        ...
                        public:
                            Tag tag;
                        ...
                        public:
                            union {
                                __crubit_A_struct A;
                                __crubit_B_struct B;
                                __crubit_D_struct D;
                            };
                        ...
                    ...
                };
            }
        );
    })
}

#[test]
fn test_repr_c_enum_with_zst() {
    let test_src = r#"
    #[repr(C, i32)]
    pub enum SomeEnum {
        A(()),
    }

    const _: () = assert!(std::mem::size_of::<SomeEnum>() == 4);
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
                        __COMMENT__ "`SomeEnum` doesn't implement the `Default` trait"
                        SomeEnum() = delete;
                        ...
                        __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                        ~SomeEnum() = default;
                        SomeEnum(SomeEnum&&) = default;
                        SomeEnum& operator=(SomeEnum&&) = default;

                        __COMMENT__ "`SomeEnum` doesn't implement the `Clone` trait"
                        SomeEnum(const SomeEnum&) = delete;
                        SomeEnum& operator=(const SomeEnum&) = delete;
                        ...
                        __COMMENT__ "Variant A has no size, so no struct is generated."
                        ...
                        enum class Tag : std::int32_t {
                            A = 0,
                        };
                        ...
                        public:
                            Tag tag;
                        ...
                    ...
                };
            }
        );
    })
}

#[test]
fn test_repr_c_union_fields() {
    let test_src = r#"
    #[repr(C)]
    pub union SomeUnion {
        pub x: u16,
        pub y: u32,
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
                        __COMMENT__ "`SomeUnion` doesn't implement the `Default` trait"
                        SomeUnion() = delete;
                        ...
                        __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                        ~SomeUnion() = default;
                        SomeUnion(SomeUnion&&) = default;
                        SomeUnion& operator=(SomeUnion&&) = default;

                        __COMMENT__ "`SomeUnion` doesn't implement the `Clone` trait"
                        SomeUnion(const SomeUnion&) = delete;
                        SomeUnion& operator=(const SomeUnion&) = delete;
                        ...
                        std::uint16_t x;
                        ...
                        std::uint32_t y;

                    private:
                        static void __crubit_field_offset_assertions();
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                static_assert(sizeof(SomeUnion) == 4, ...);
                static_assert(alignof(SomeUnion) == 4, ...);
                static_assert(std::is_trivially_destructible_v<SomeUnion>);
                static_assert(std::is_trivially_move_constructible_v<SomeUnion>);
                static_assert(std::is_trivially_move_assignable_v<SomeUnion>);
                inline void SomeUnion::__crubit_field_offset_assertions() {
                  static_assert(0 == offsetof(SomeUnion, x));
                  static_assert(0 == offsetof(SomeUnion, y));
                }
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                const _: () = assert!(::std::mem::size_of::<::rust_out::SomeUnion>() == 4);
                const _: () = assert!(::std::mem::align_of::<::rust_out::SomeUnion>() == 4);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeUnion, x) == 0);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeUnion, y) == 0);
            }
        );
    })
}

#[test]
fn test_union_fields() {
    let test_src = r#"
    pub union SomeUnion {
        pub x: u16,
        pub y: u32,
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
                        __COMMENT__ "`SomeUnion` doesn't implement the `Default` trait"
                        SomeUnion() = delete;
                        ...
                        __COMMENT__ "No custom `Drop` impl and no custom \"drop glue\" required"
                        ~SomeUnion() = default;
                        SomeUnion(SomeUnion&&) = default;
                        SomeUnion& operator=(SomeUnion&&) = default;

                        __COMMENT__ "`SomeUnion` doesn't implement the `Clone` trait"
                        SomeUnion(const SomeUnion&) = delete;
                        SomeUnion& operator=(const SomeUnion&) = delete;
                        ...
                        struct {
                            ...
                            std::uint16_t value;
                        } x;
                        ...
                        struct {
                            ...
                            std::uint32_t value;
                        } y;
                    private:
                        static void __crubit_field_offset_assertions();
                };
            }
        );

        // Note: we don't check for offsets here, because we don't know necessarily know
        // what the offset will be.
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                static_assert(sizeof(SomeUnion) == 4, ...);
                static_assert(alignof(SomeUnion) == 4, ...);
                static_assert(std::is_trivially_destructible_v<SomeUnion>);
                static_assert(std::is_trivially_move_constructible_v<SomeUnion>);
                static_assert(std::is_trivially_move_assignable_v<SomeUnion>);
                inline void SomeUnion::__crubit_field_offset_assertions() {
                  ...
                }
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                const _: () = assert!(::std::mem::size_of::<::rust_out::SomeUnion>() == 4);
                const _: () = assert!(::std::mem::align_of::<::rust_out::SomeUnion>() == 4);
                ...
            }
        );
    })
}

#[test]
fn test_repr_c_union_unknown_fields() {
    let test_src = r#"
    #[repr(C)]
    pub union SomeUnion {
        pub z: std::mem::ManuallyDrop<i64>,
    }

    const _: () = assert!(std::mem::size_of::<SomeUnion>() == 8);
    const _: () = assert!(std::mem::align_of::<SomeUnion>() == 8);
    "#;

    test_format_item(test_src, "SomeUnion", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                union CRUBIT_INTERNAL_RUST_TYPE(...) alignas(8) [[clang::trivial_abi]] SomeUnion final {
                    public:
                        ...
                    private:
                        __COMMENT__ "Field type has been replaced with a blob of bytes: Generic types are not supported yet (b/259749095)"
                        unsigned char z[8];
                    ...
                };
            }
        );
        assert_cc_matches!(
            result.cc_details.tokens,
            quote! {
                static_assert(sizeof(SomeUnion) == 8, ...);
                static_assert(alignof(SomeUnion) == 8, ...);
                ...
            }
        );
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                const _: () = assert!(::std::mem::size_of::<::rust_out::SomeUnion>() == 8);
                const _: () = assert!(::std::mem::align_of::<::rust_out::SomeUnion>() == 8);
                const _: () = assert!( ::core::mem::offset_of!(::rust_out::SomeUnion, z) == 0);
            }
        );
    })
}

#[test]
fn test_format_cpp_name_for_struct() {
    let test_src = r#"
            #[doc="CRUBIT_ANNOTATE: cpp_type=cpp_ns::CppType"]
            pub struct RustType {
                pub x: i32,
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
}

#[test]
fn test_must_use_attr_for_struct_no_msg() {
    let test_src = r#"
    #[must_use]
    pub struct SomeStruct {
        pub x: u32,
        pub y: u32,
    }"#;

    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... [[nodiscard]] ... SomeStruct final {
                    ...
                };
            }
        )
    })
}

#[test]
fn test_format_item_rename_field_with_conflicting_name() {
    let test_src = r#"
    pub struct X {
        pub a: i32,
        b: i32,
        #[allow(dead_code)]
        c: i32,
    }

    impl X {
        pub fn a(&self) -> i32 {
            self.a
        }
        pub fn b(&self) -> i32 {
            self.b
        }
    }
    "#;

    test_format_item(test_src, "X", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                std::int32_t a_;
            }
        );
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                std::int32_t b_;
            }
        );
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                std::int32_t c;
            }
        );
        // Check that the fields are not renamed in the Rust side.
        assert_rs_matches!(
            result.rs_details.tokens,
            quote! {
                ::core::mem::offset_of!(::rust_out::X, a) == 0
            }
        );
    })
}

#[test]
fn test_must_use_attr_for_struct_msg() {
    let test_src = r#"
    #[must_use = "foo"]
    pub struct SomeStruct {
        pub x: u32,
        pub y: u32,
    }"#;

    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... [[nodiscard("foo")]] ... SomeStruct final {
                    ...
                };
            }
        )
    })
}

#[test]
fn test_must_use_attr_for_enum_no_msg() {
    let test_src = r#"
    #[must_use]
    pub enum SomeEnum {
        A(i32),
        B(u32),
    }"#;

    test_format_item(test_src, "SomeEnum", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... [[nodiscard]] ... SomeEnum final {
                    ...
                };
            }
        )
    })
}

#[test]
fn test_must_use_attr_for_enum_msg() {
    let test_src = r#"
    #[must_use = "foo"]
    pub enum SomeEnum {
        A(i32),
        B(u32),
    }"#;

    test_format_item(test_src, "SomeEnum", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                struct ... [[nodiscard("foo")]] ... SomeEnum final {
                    ...
                };
            }
        )
    })
}

#[test]
fn test_must_use_attr_for_union_no_msg() {
    let test_src = r#"
    #[must_use]
    pub union SomeUnion {
        pub x: u32,
        pub y: u32,
    }"#;

    test_format_item(test_src, "SomeUnion", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                union ... [[nodiscard]] ... SomeUnion final {
                    ...
                };
            }
        )
    })
}
#[test]
fn test_must_use_attr_for_union_msg() {
    let test_src = r#"
    #[must_use = "foo"]
    pub union SomeUnion {
        pub x: u32,
        pub y: u32,
    }"#;

    test_format_item(test_src, "SomeUnion", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                ...
                union ... [[nodiscard("foo")]] ... SomeUnion final {
                    ...
                };
            }
        )
    })
}

#[test]
fn test_deprecated_attr_for_struct_no_args() {
    let test_src = r#"
    #[deprecated]
    pub struct SomeStruct {
        pub x: u32,
        pub y: u32,
    }"#;

    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                struct ... [[deprecated]] ... SomeStruct final {
                    ...
                };
            }
        )
    })
}

#[test]
fn test_deprecated_attr_for_struct_with_message() {
    let test_src = r#"
    #[deprecated = "Use AnotherStruct instead"]
    pub struct SomeStruct {
        pub x: u32,
        pub y: u32,
    }"#;

    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                struct ... [[deprecated("Use AnotherStruct instead")]] ... SomeStruct final {
                    ...
                };
            }
        )
    })
}

#[test]
fn test_deprecated_attr_for_struct_with_named_args() {
    let test_src = r#"
    #[deprecated(since = "3.14", note = "Use AnotherStruct instead")]
    pub struct SomeStruct {
        pub x: u32,
        pub y: u32,
    }"#;

    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                struct ... [[deprecated("Use AnotherStruct instead")]] ... SomeStruct final {
                    ...
                };
            }
        )
    })
}

#[test]
fn test_deprecated_attr_for_union_with_named_args() {
    let test_src = r#"
    #[deprecated(since = "3.14", note = "Use AnotherUnion instead")]
    pub struct SomeUnion {
        pub x: u32,
        pub y: u32,
    }"#;

    test_format_item(test_src, "SomeUnion", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                struct ... [[deprecated("Use AnotherUnion instead")]] ... SomeUnion final {
                    ...
                };
            }
        )
    })
}

#[test]
fn test_deprecated_attr_for_enum_with_named_args() {
    let test_src = r#"
    #[deprecated(since = "3.14", note = "Use AnotherEnum instead")]
    pub enum SomeEnum {
        Integer(i32),
        FloatingPoint(f64),
    }"#;

    test_format_item(test_src, "SomeEnum", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                struct ... [[deprecated("Use AnotherEnum instead")]] ... SomeEnum final {
                    ...
                };
            }
        )
    })
}

#[test]
fn test_deprecated_attr_for_struct_fields() {
    let test_src = r#"
    pub struct SomeStruct {
        #[deprecated = "Use `y` instead"]
        pub x: u32,

        pub y: u32,
    }"#;

    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                struct ... SomeStruct final {
                    ...
                    union {
                        ...
                        [[deprecated("Use `y` instead")]] std::uint32_t x;
                    }
                    ...
                    union {
                        ...
                        std::uint32_t y;
                    }
                    ...
                };
            }
        )
    })
}

#[test]
fn test_deprecated_attr_for_impl_block() {
    let test_src = r#"
    pub struct SomeStruct {
        pub x: u32,
        pub y: u32,
    }

    #[deprecated = "Use AnotherStruct instead"]
    impl SomeStruct {
        pub fn sum(&self) -> u32 {
            self.x + self.y
        }

        pub fn product(&self) -> u32 {
            self.x * self.y
        }
    }"#;

    test_format_item(test_src, "SomeStruct", |result| {
        let result = result.unwrap().unwrap();
        let main_api = &result.main_api;
        assert!(!main_api.prereqs.is_empty());
        assert_cc_matches!(
            main_api.tokens,
            quote! {
                struct ... SomeStruct final {
                    ...
                    ... [[deprecated("Use AnotherStruct instead")]] std::uint32_t sum() const ...
                    ...
                    ... [[deprecated("Use AnotherStruct instead")]] std::uint32_t product() const ...
                    ...
                };
            }
        )
    })
}

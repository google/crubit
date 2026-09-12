// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// structs_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () =
    assert!(::std::mem::size_of::<::structs_golden::abi_classification::StructFloat>() == 16);
const _: () =
    assert!(::std::mem::align_of::<::structs_golden::abi_classification::StructFloat>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(f: f32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::structs_golden::abi_classification::StructFloat::create(f);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_multiply(
    x: *mut ::structs_golden::abi_classification::StructFloat,
    y: *mut ::structs_golden::abi_classification::StructFloat,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let x = x.read();
        let y = y.read();
        let __rs_return_value = ::structs_golden::abi_classification::StructFloat::multiply(x, y);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_inspect(
    s: *mut ::structs_golden::abi_classification::StructFloat,
) -> f32 {
    unsafe {
        let s = s.read();
        ::structs_golden::abi_classification::StructFloat::inspect(s)
    }
}
const _: () =
    assert!(::std::mem::size_of::<::structs_golden::abi_classification::StructInteger>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::structs_golden::abi_classification::StructInteger>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(i: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::structs_golden::abi_classification::StructInteger::create(i);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_multiply(
    x: *mut ::structs_golden::abi_classification::StructInteger,
    y: *mut ::structs_golden::abi_classification::StructInteger,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let x = x.read();
        let y = y.read();
        let __rs_return_value = ::structs_golden::abi_classification::StructInteger::multiply(x, y);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_inspect(
    s: *mut ::structs_golden::abi_classification::StructInteger,
) -> i32 {
    unsafe {
        let s = s.read();
        ::structs_golden::abi_classification::StructInteger::inspect(s)
    }
}
const _: () =
    assert!(::std::mem::size_of::<::structs_golden::abi_classification::StructMemory>() == 5);
const _: () =
    assert!(::std::mem::align_of::<::structs_golden::abi_classification::StructMemory>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(i: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::structs_golden::abi_classification::StructMemory::create(i);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_multiply(
    x: *mut ::structs_golden::abi_classification::StructMemory,
    y: *mut ::structs_golden::abi_classification::StructMemory,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let x = x.read();
        let y = y.read();
        let __rs_return_value = ::structs_golden::abi_classification::StructMemory::multiply(x, y);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_inspect(
    s: *mut ::structs_golden::abi_classification::StructMemory,
) -> i32 {
    unsafe {
        let s = s.read();
        ::structs_golden::abi_classification::StructMemory::inspect(s)
    }
}
const _: () = assert!(
    ::std::mem::size_of::<::structs_golden::aggregate_initialization::AnnotatedTwoDrops>() == 24
);
const _: () = assert!(
    ::std::mem::align_of::<::structs_golden::aggregate_initialization::AnnotatedTwoDrops>() == 4
);
const _: () = assert!(
    ::core::mem::offset_of!(::structs_golden::aggregate_initialization::AnnotatedTwoDrops, 0) == 0
);
const _: () = assert!(
    ::core::mem::offset_of!(::structs_golden::aggregate_initialization::AnnotatedTwoDrops, 1) == 12
);
const _: () = assert!(
    ::std::mem::size_of::<::structs_golden::aggregate_initialization::BasicAggregate>() == 8
);
const _: () = assert!(
    ::std::mem::align_of::<::structs_golden::aggregate_initialization::BasicAggregate>() == 4
);
const _: () = assert!(
    ::core::mem::offset_of!(::structs_golden::aggregate_initialization::BasicAggregate, x) == 0
);
const _: () = assert!(
    ::core::mem::offset_of!(::structs_golden::aggregate_initialization::BasicAggregate, y) == 4
);
const _: () = assert!(
    ::std::mem::size_of::<::structs_golden::aggregate_initialization::CustomDropStruct>() == 4
);
const _: () = assert!(
    ::std::mem::align_of::<::structs_golden::aggregate_initialization::CustomDropStruct>() == 4
);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_ustructs_ugolden_x0000003a_x0000003aaggregate_uinitialization_x0000003a_x0000003aCustomDropStruct(
    __self: *mut ::structs_golden::aggregate_initialization::CustomDropStruct,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(x: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            ::structs_golden::aggregate_initialization::CustomDropStruct::create(x);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(::structs_golden::aggregate_initialization::CustomDropStruct, x) == 0
);
const _: () = assert!(
    ::std::mem::size_of::<::structs_golden::aggregate_initialization::NonExhaustiveStruct>() == 8
);
const _: () = assert!(
    ::std::mem::align_of::<::structs_golden::aggregate_initialization::NonExhaustiveStruct>() == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    x: i32,
    y: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::structs_golden::aggregate_initialization::NonExhaustiveStruct::create(x, y);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(::structs_golden::aggregate_initialization::NonExhaustiveStruct, x)
        == 0
);
const _: () = assert!(
    ::core::mem::offset_of!(::structs_golden::aggregate_initialization::NonExhaustiveStruct, y)
        == 4
);
const _: () = assert!(
    ::std::mem::size_of::<::structs_golden::aggregate_initialization::SingleDropField>() == 12
);
const _: () = assert!(
    ::std::mem::align_of::<::structs_golden::aggregate_initialization::SingleDropField>() == 4
);
const _: () = assert!(
    ::core::mem::offset_of!(::structs_golden::aggregate_initialization::SingleDropField, 0) == 0
);
const _: () = assert!(
    ::std::mem::size_of::<::structs_golden::aggregate_initialization::StructWithPrivateField>()
        == 8
);
const _: () = assert!(
    ::std::mem::align_of::<::structs_golden::aggregate_initialization::StructWithPrivateField>()
        == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    x: i32,
    y: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::structs_golden::aggregate_initialization::StructWithPrivateField::create(x, y);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(::structs_golden::aggregate_initialization::StructWithPrivateField, x)
        == 0
);
const _: () = assert!(
    ::std::mem::size_of::<::structs_golden::aggregate_initialization::TupleAggregate>() == 16
);
const _: () = assert!(
    ::std::mem::align_of::<::structs_golden::aggregate_initialization::TupleAggregate>() == 8
);
const _: () = assert!(
    ::core::mem::offset_of!(::structs_golden::aggregate_initialization::TupleAggregate, 1) == 0
);
const _: () = assert!(
    ::core::mem::offset_of!(::structs_golden::aggregate_initialization::TupleAggregate, 0) == 8
);
const _: () = assert!(
    ::std::mem::size_of::<::structs_golden::aggregate_initialization::UnannotatedTwoDrops>() == 24
);
const _: () = assert!(
    ::std::mem::align_of::<::structs_golden::aggregate_initialization::UnannotatedTwoDrops>() == 4
);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_ustructs_ugolden_x0000003a_x0000003aaggregate_uinitialization_x0000003a_x0000003aUnannotatedTwoDrops(
    __self: *mut ::structs_golden::aggregate_initialization::UnannotatedTwoDrops,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
const _: () = assert!(
    ::core::mem::offset_of!(::structs_golden::aggregate_initialization::UnannotatedTwoDrops, 0)
        == 0
);
const _: () = assert!(
    ::core::mem::offset_of!(::structs_golden::aggregate_initialization::UnannotatedTwoDrops, 1)
        == 12
);
const _: () = assert!(::std::mem::size_of::<::structs_golden::default_repr::Point>() == 8);
const _: () = assert!(::std::mem::align_of::<::structs_golden::default_repr::Point>() == 4);
const _: () = assert!(::core::mem::offset_of!(::structs_golden::default_repr::Point, x) == 0);
const _: () = assert!(::core::mem::offset_of!(::structs_golden::default_repr::Point, y) == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    x: i32,
    y: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::structs_golden::default_repr::create(x, y);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ux(p: *mut ::structs_golden::default_repr::Point) -> i32 {
    unsafe {
        let p = p.read();
        ::structs_golden::default_repr::get_x(p)
    }
}
const _: () = assert!(::std::mem::size_of::<::structs_golden::display::DisplayStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::structs_golden::display::DisplayStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_ToString_uto_ustring_ustructs_ugolden_x0000003a_x0000003adisplay_x0000003a_x0000003aDisplayStruct(
    __self: &'static ::structs_golden::display::DisplayStruct,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::structs_golden::display::DisplayStruct as ::alloc::string::ToString>::to_string(
                __self,
            );
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::structs_golden::display::DisplayStruct, value) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(value: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::structs_golden::display::create(value);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () =
    assert!(::std::mem::size_of::<::structs_golden::interior_mutability::SomeStruct>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::structs_golden::interior_mutability::SomeStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_ustructs_ugolden_x0000003a_x0000003ainterior_umutability_x0000003a_x0000003aSomeStruct(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=<::structs_golden::interior_mutability::SomeStruct as::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::structs_golden::interior_mutability::SomeStruct, field) == 0);
const _: () = assert!(
    ::std::mem::size_of::<::structs_golden::keyword_named_fields_and_methods::AField>() == 4
);
const _: () = assert!(
    ::std::mem::align_of::<::structs_golden::keyword_named_fields_and_methods::AField>() == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_operator(
    __self: &'static ::structs_golden::keyword_named_fields_and_methods::AField,
) -> i32 {
    unsafe { ::structs_golden::keyword_named_fields_and_methods::AField::operator(__self) }
}
const _: () = assert!(
    ::std::mem::size_of::<::structs_golden::nested_ptr_type_mutability_qualifiers::SomeStruct>()
        == 8
);
const _: () = assert!(
    ::std::mem::align_of::<::structs_golden::nested_ptr_type_mutability_qualifiers::SomeStruct>()
        == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_ustructs_ugolden_x0000003a_x0000003anested_uptr_utype_umutability_uqualifiers_x0000003a_x0000003aSomeStruct(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=<::structs_golden::nested_ptr_type_mutability_qualifiers::SomeStruct as::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(
        ::structs_golden::nested_ptr_type_mutability_qualifiers::SomeStruct,
        mut_const_ptr
    ) == 0
);
const _: () = assert!(
    ::core::mem::offset_of!(
        ::structs_golden::nested_ptr_type_mutability_qualifiers::SomeStruct,
        const_mut_ptr
    ) == 4
);
const _: () = assert!(::std::mem::size_of::<::structs_golden::non_cpp_movable::Point>() == 8);
const _: () = assert!(::std::mem::align_of::<::structs_golden::non_cpp_movable::Point>() == 4);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_ustructs_ugolden_x0000003a_x0000003anon_ucpp_umovable_x0000003a_x0000003aPoint(
    __self: *mut ::structs_golden::non_cpp_movable::Point,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
const _: () = assert!(::core::mem::offset_of!(::structs_golden::non_cpp_movable::Point, x) == 0);
const _: () = assert!(::core::mem::offset_of!(::structs_golden::non_cpp_movable::Point, y) == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    x: i32,
    y: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::structs_golden::non_cpp_movable::create(x, y);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ux(
    p: &'static ::structs_golden::non_cpp_movable::Point,
) -> i32 {
    unsafe { ::structs_golden::non_cpp_movable::get_x(p) }
}
const _: () = assert!(::std::mem::size_of::<::structs_golden::repr_c::Point>() == 8);
const _: () = assert!(::std::mem::align_of::<::structs_golden::repr_c::Point>() == 4);
const _: () = assert!(::core::mem::offset_of!(::structs_golden::repr_c::Point, x) == 0);
const _: () = assert!(::core::mem::offset_of!(::structs_golden::repr_c::Point, y) == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    x: i32,
    y: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::structs_golden::repr_c::create(x, y);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ux(p: *mut ::structs_golden::repr_c::Point) -> i32 {
    unsafe {
        let p = p.read();
        ::structs_golden::repr_c::get_x(p)
    }
}
const _: () = assert!(
    ::std::mem::size_of::<
        ::structs_golden::struct_by_float_passing_with_no_cc_definition::StructFloat,
    >() == 16
);
const _: () = assert!(
    ::std::mem::align_of::<
        ::structs_golden::struct_by_float_passing_with_no_cc_definition::StructFloat,
    >() == 8
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_no_umangle_ucreate(
    f: f32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::structs_golden::struct_by_float_passing_with_no_cc_definition::no_mangle_create(f);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_no_umangle_uinspect(
    s: *mut ::structs_golden::struct_by_float_passing_with_no_cc_definition::StructFloat,
) -> f32 {
    unsafe {
        let s = s.read();
        ::structs_golden::struct_by_float_passing_with_no_cc_definition::no_mangle_inspect(s)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_no_umangle_umultiply(
    x: *mut ::structs_golden::struct_by_float_passing_with_no_cc_definition::StructFloat,
    y: *mut ::structs_golden::struct_by_float_passing_with_no_cc_definition::StructFloat,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let x = x.read();
        let y = y.read();
        let __rs_return_value =
            ::structs_golden::struct_by_float_passing_with_no_cc_definition::no_mangle_multiply(
                x, y,
            );
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(
    ::std::mem::size_of::<::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat>()
        == 16
);
const _: () = assert!(
    ::std::mem::align_of::<::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat>()
        == 8
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustructs_ugolden_x0000003a_x0000003astruct_uby_ufloat_upassing_uwith_uno_uthunk_x0000003a_x0000003aStructFloat(
    __self: &'static ::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=<::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat as::core::clone::Clone>::clone(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustructs_ugolden_x0000003a_x0000003astruct_uby_ufloat_upassing_uwith_uno_uthunk_x0000003a_x0000003aStructFloat(
    __self: &'static mut ::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat,
    source: &'static ::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat,
) -> () {
    unsafe {
        <::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat as::core::clone::Clone>::clone_from(__self,source)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_struct_uby_ufloat_upassing_uwith_uno_uthunk_u_uthunkless_ucreate(
    f: f32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::structs_golden::struct_by_float_passing_with_no_thunk::thunkless_create(f);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_struct_uby_ufloat_upassing_uwith_uno_uthunk_u_uthunkless_uinspect(
    s: *mut ::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat,
) -> f32 {
    unsafe {
        let s = s.read();
        ::structs_golden::struct_by_float_passing_with_no_thunk::thunkless_inspect(s)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_struct_uby_ufloat_upassing_uwith_uno_uthunk_u_uthunkless_umultiply(
    x: *mut ::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat,
    y: *mut ::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let x = x.read();
        let y = y.read();
        let __rs_return_value =
            ::structs_golden::struct_by_float_passing_with_no_thunk::thunkless_multiply(x, y);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () =
    assert!(::std::mem::size_of::<::structs_golden::unsupported_types::SomeStruct>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::structs_golden::unsupported_types::SomeStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(x: *mut char, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let x = x.read();
        let __rs_return_value = ::structs_golden::unsupported_types::SomeStruct::create(x);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(::structs_golden::unsupported_types::SomeStruct, unsupported_field)
        == 0
);
const _: () = assert!(::std::mem::size_of::<::structs_golden::zst_fields::ZstFields>() == 4);
const _: () = assert!(::std::mem::align_of::<::structs_golden::zst_fields::ZstFields>() == 4);
const _: () = assert!(::core::mem::offset_of!(::structs_golden::zst_fields::ZstFields, value) == 0);
const _: () = assert!(::core::mem::offset_of!(::structs_golden::zst_fields::ZstFields, zst1) == 4);
const _: () = assert!(::core::mem::offset_of!(::structs_golden::zst_fields::ZstFields, zst2) == 4);
const _: () = assert!(::core::mem::offset_of!(::structs_golden::zst_fields::ZstFields, zst3) == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(value: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::structs_golden::zst_fields::create(value);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uvalue(
    x: *mut ::structs_golden::zst_fields::ZstFields,
) -> i32 {
    unsafe {
        let x = x.read();
        ::structs_golden::zst_fields::get_value(x)
    }
}

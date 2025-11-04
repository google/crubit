// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// structs_golden
// Features: infer_operator_lifetimes, non_unpin_ctor, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

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
        (__ret_ptr as *mut ::structs_golden::repr_c::Point).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ux(
    p: &'static mut ::core::mem::MaybeUninit<::structs_golden::repr_c::Point>,
) -> i32 {
    unsafe {
        let p = p.assume_init_read();
        ::structs_golden::repr_c::get_x(p)
    }
}
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
        (__ret_ptr as *mut ::structs_golden::default_repr::Point).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ux(
    p: &'static mut ::core::mem::MaybeUninit<::structs_golden::default_repr::Point>,
) -> i32 {
    unsafe {
        let p = p.assume_init_read();
        ::structs_golden::default_repr::get_x(p)
    }
}
const _: () = assert!(::std::mem::size_of::<::structs_golden::non_cpp_movable::Point>() == 8);
const _: () = assert!(::std::mem::align_of::<::structs_golden::non_cpp_movable::Point>() == 4);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::structs_golden::non_cpp_movable::Point>,
) {
    unsafe { __self.assume_init_drop() };
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
        (__ret_ptr as *mut ::structs_golden::non_cpp_movable::Point).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ux(
    p: &'static ::structs_golden::non_cpp_movable::Point,
) -> i32 {
    unsafe { ::structs_golden::non_cpp_movable::get_x(p) }
}
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
        (__ret_ptr as *mut ::structs_golden::zst_fields::ZstFields).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uvalue(
    x: &'static mut ::core::mem::MaybeUninit<::structs_golden::zst_fields::ZstFields>,
) -> i32 {
    unsafe {
        let x = x.assume_init_read();
        ::structs_golden::zst_fields::get_value(x)
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
        (__ret_ptr as *mut ::structs_golden::abi_classification::StructInteger)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_multiply(
    x: &'static mut ::core::mem::MaybeUninit<::structs_golden::abi_classification::StructInteger>,
    y: &'static mut ::core::mem::MaybeUninit<::structs_golden::abi_classification::StructInteger>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let x = x.assume_init_read();
        let y = y.assume_init_read();
        let __rs_return_value = ::structs_golden::abi_classification::StructInteger::multiply(x, y);
        (__ret_ptr as *mut ::structs_golden::abi_classification::StructInteger)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_inspect(
    s: &'static mut ::core::mem::MaybeUninit<::structs_golden::abi_classification::StructInteger>,
) -> i32 {
    unsafe {
        let s = s.assume_init_read();
        ::structs_golden::abi_classification::StructInteger::inspect(s)
    }
}
const _: () =
    assert!(::std::mem::size_of::<::structs_golden::abi_classification::StructFloat>() == 16);
const _: () =
    assert!(::std::mem::align_of::<::structs_golden::abi_classification::StructFloat>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(f: f32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::structs_golden::abi_classification::StructFloat::create(f);
        (__ret_ptr as *mut ::structs_golden::abi_classification::StructFloat)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_multiply(
    x: &'static mut ::core::mem::MaybeUninit<::structs_golden::abi_classification::StructFloat>,
    y: &'static mut ::core::mem::MaybeUninit<::structs_golden::abi_classification::StructFloat>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let x = x.assume_init_read();
        let y = y.assume_init_read();
        let __rs_return_value = ::structs_golden::abi_classification::StructFloat::multiply(x, y);
        (__ret_ptr as *mut ::structs_golden::abi_classification::StructFloat)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_inspect(
    s: &'static mut ::core::mem::MaybeUninit<::structs_golden::abi_classification::StructFloat>,
) -> f32 {
    unsafe {
        let s = s.assume_init_read();
        ::structs_golden::abi_classification::StructFloat::inspect(s)
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
        (__ret_ptr as *mut ::structs_golden::abi_classification::StructMemory)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_multiply(
    x: &'static mut ::core::mem::MaybeUninit<::structs_golden::abi_classification::StructMemory>,
    y: &'static mut ::core::mem::MaybeUninit<::structs_golden::abi_classification::StructMemory>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let x = x.assume_init_read();
        let y = y.assume_init_read();
        let __rs_return_value = ::structs_golden::abi_classification::StructMemory::multiply(x, y);
        (__ret_ptr as *mut ::structs_golden::abi_classification::StructMemory)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_inspect(
    s: &'static mut ::core::mem::MaybeUninit<::structs_golden::abi_classification::StructMemory>,
) -> i32 {
    unsafe {
        let s = s.assume_init_read();
        ::structs_golden::abi_classification::StructMemory::inspect(s)
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
        (__ret_ptr
            as *mut ::structs_golden::struct_by_float_passing_with_no_cc_definition::StructFloat)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_no_umangle_umultiply(
    x: &'static mut ::core::mem::MaybeUninit<
        ::structs_golden::struct_by_float_passing_with_no_cc_definition::StructFloat,
    >,
    y: &'static mut ::core::mem::MaybeUninit<
        ::structs_golden::struct_by_float_passing_with_no_cc_definition::StructFloat,
    >,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let x = x.assume_init_read();
        let y = y.assume_init_read();
        let __rs_return_value =
            ::structs_golden::struct_by_float_passing_with_no_cc_definition::no_mangle_multiply(
                x, y,
            );
        (__ret_ptr
            as *mut ::structs_golden::struct_by_float_passing_with_no_cc_definition::StructFloat)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_no_umangle_uinspect(
    s: &'static mut ::core::mem::MaybeUninit<
        ::structs_golden::struct_by_float_passing_with_no_cc_definition::StructFloat,
    >,
) -> f32 {
    unsafe {
        let s = s.assume_init_read();
        ::structs_golden::struct_by_float_passing_with_no_cc_definition::no_mangle_inspect(s)
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
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=<::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat as::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
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
        (__ret_ptr as *mut ::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_struct_uby_ufloat_upassing_uwith_uno_uthunk_u_uthunkless_umultiply(
    x: &'static mut ::core::mem::MaybeUninit<
        ::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat,
    >,
    y: &'static mut ::core::mem::MaybeUninit<
        ::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat,
    >,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let x = x.assume_init_read();
        let y = y.assume_init_read();
        let __rs_return_value =
            ::structs_golden::struct_by_float_passing_with_no_thunk::thunkless_multiply(x, y);
        (__ret_ptr as *mut ::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_struct_uby_ufloat_upassing_uwith_uno_uthunk_u_uthunkless_uinspect(
    s: &'static mut ::core::mem::MaybeUninit<
        ::structs_golden::struct_by_float_passing_with_no_thunk::StructFloat,
    >,
) -> f32 {
    unsafe {
        let s = s.assume_init_read();
        ::structs_golden::struct_by_float_passing_with_no_thunk::thunkless_inspect(s)
    }
}
const _: () = assert!(
    ::std::mem::size_of::<::structs_golden::nested_ptr_type_mutability_qualifiers::SomeStruct>()
        == 16
);
const _: () = assert!(
    ::std::mem::align_of::<::structs_golden::nested_ptr_type_mutability_qualifiers::SomeStruct>()
        == 8
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value=<::structs_golden::nested_ptr_type_mutability_qualifiers::SomeStruct as::core::default::Default>::default();
        (__ret_ptr as *mut ::structs_golden::nested_ptr_type_mutability_qualifiers::SomeStruct)
            .write(__rs_return_value);
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
    ) == 8
);
const _: () =
    assert!(::std::mem::size_of::<::structs_golden::interior_mutability::SomeStruct>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::structs_golden::interior_mutability::SomeStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value=<::structs_golden::interior_mutability::SomeStruct as::core::default::Default>::default();
        (__ret_ptr as *mut ::structs_golden::interior_mutability::SomeStruct)
            .write(__rs_return_value);
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::structs_golden::interior_mutability::SomeStruct, field) == 0);
const _: () =
    assert!(::std::mem::size_of::<::structs_golden::unsupported_types::SomeStruct>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::structs_golden::unsupported_types::SomeStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::structs_golden::unsupported_types::SomeStruct as ::core::default::Default>::default(
            );
        (__ret_ptr as *mut ::structs_golden::unsupported_types::SomeStruct)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(x: char, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::structs_golden::unsupported_types::SomeStruct::create(x);
        (__ret_ptr as *mut ::structs_golden::unsupported_types::SomeStruct)
            .write(__rs_return_value);
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(::structs_golden::unsupported_types::SomeStruct, unsupported_field)
        == 0
);
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

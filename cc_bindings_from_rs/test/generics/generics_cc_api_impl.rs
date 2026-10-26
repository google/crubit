// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// generics_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::core::mem::size_of::<::generics_golden::NotPartialEqOrDefault>() == 4);
const _: () = assert!(::core::mem::align_of::<::generics_golden::NotPartialEqOrDefault>() == 4);
const _: () = assert!(::core::mem::offset_of!(::generics_golden::NotPartialEqOrDefault, 0) == 0);
const _: () =
    assert!(::core::mem::size_of::<::generics_golden::OuterWithNestedGenericField>() == 4);
const _: () =
    assert!(::core::mem::align_of::<::generics_golden::OuterWithNestedGenericField>() == 4);
const _: () =
    assert!(::core::mem::offset_of!(::generics_golden::OuterWithNestedGenericField, field) == 0);
const _: () = assert!(::core::mem::size_of::<::generics_golden::PartialEqAndDefault>() == 4);
const _: () = assert!(::core::mem::align_of::<::generics_golden::PartialEqAndDefault>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
    __self: &'static ::generics_golden::PartialEqAndDefault,
    other: &'static ::generics_golden::PartialEqAndDefault,
) -> bool {
    unsafe {
        <::generics_golden::PartialEqAndDefault as ::core::cmp::PartialEq<
            ::generics_golden::PartialEqAndDefault,
        >>::eq(__self, other)
    }
}
const _: () = assert!(::core::mem::offset_of!(::generics_golden::PartialEqAndDefault, 0) == 0);
const _: () = assert!(::core::mem::size_of::<::generics_golden::SelfReferentialViaPhantom>() == 4);
const _: () = assert!(::core::mem::align_of::<::generics_golden::SelfReferentialViaPhantom>() == 4);
const _: () =
    assert!(::core::mem::offset_of!(::generics_golden::SelfReferentialViaPhantom, phantom) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_accept_ugeneric_uwith_udrop_uunmovable(
    _x: *mut ::generics_golden::GenericWithDrop<::generics_golden::NotPartialEqOrDefault>,
) -> () {
    unsafe {
        let _x = _x.read();
        ::generics_golden::accept_generic_with_drop_unmovable(_x)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_instantiate_uderived_ugeneric_uat_ui32(
    __ret_ptr: *mut ::generics_golden::DerivedGeneric<i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::generics_golden::instantiate_derived_generic_at_i32());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_instantiate_uderived_ugeneric_uat_unot_upartial_ueq_uor_udefault(
    __ret_ptr: *mut ::generics_golden::DerivedGeneric<::generics_golden::NotPartialEqOrDefault>,
) -> () {
    unsafe {
        __ret_ptr
            .write(::generics_golden::instantiate_derived_generic_at_not_partial_eq_or_default());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_instantiate_ugeneric_uenum(
    __ret_ptr: *mut ::generics_golden::GenericEnum<i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::generics_golden::instantiate_generic_enum());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_instantiate_ugeneric_ustruct_uat_ui32(
    __ret_ptr: *mut ::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>,
) -> () {
    unsafe {
        __ret_ptr.write(::generics_golden::instantiate_generic_struct_at_i32());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_instantiate_ugeneric_ustruct_uat_unot_upartial_ueq_uor_udefault(
    __ret_ptr: *mut ::generics_golden::GenericStruct<::generics_golden::NotPartialEqOrDefault>,
) -> () {
    unsafe {
        __ret_ptr
            .write(::generics_golden::instantiate_generic_struct_at_not_partial_eq_or_default());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_instantiate_ugeneric_ustruct_uat_uprimitive_ui32(
    __ret_ptr: *mut ::generics_golden::GenericStruct<i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::generics_golden::instantiate_generic_struct_at_primitive_i32());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_instantiate_ugeneric_uunion(
    __ret_ptr: *mut ::generics_golden::GenericUnion<u32>,
) -> () {
    unsafe {
        __ret_ptr.write(::generics_golden::instantiate_generic_union());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_instantiate_ugeneric_uwith_udrop_umovable(
    __ret_ptr: *mut ::generics_golden::GenericWithDrop<i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::generics_golden::instantiate_generic_with_drop_movable());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_instantiate_ugeneric_uwith_udrop_uunmovable(
    __ret_ptr: *mut ::generics_golden::GenericWithDrop<::generics_golden::NotPartialEqOrDefault>,
) -> () {
    unsafe {
        __ret_ptr.write(::generics_golden::instantiate_generic_with_drop_unmovable());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_instantiate_ulifetime_uand_utype_ugeneric(
    x: &'static i32,
    __ret_ptr: *mut ::generics_golden::LifetimeAndTypeGeneric<'static, i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::generics_golden::instantiate_lifetime_and_type_generic(x));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_instantiate_umulti_ugeneric_uat_ubool_uf64(
    __ret_ptr: *mut ::generics_golden::MultiGenericStruct<bool, f64>,
) -> () {
    unsafe {
        __ret_ptr.write(::generics_golden::instantiate_multi_generic_at_bool_f64());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_instantiate_umulti_ugeneric_uat_ubool_ui32(
    __ret_ptr: *mut ::generics_golden::MultiGenericStruct<bool, i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::generics_golden::instantiate_multi_generic_at_bool_i32());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_instantiate_uping(
    __ret_ptr: *mut ::generics_golden::Ping<i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::generics_golden::instantiate_ping());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_instantiate_uself_ureferential_uphantom(
    __ret_ptr: *mut ::generics_golden::SelfReferentialViaPhantom,
) -> () {
    unsafe {
        __ret_ptr.write(::generics_golden::instantiate_self_referential_phantom());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_instantiate_usub_ugeneric(
    __ret_ptr: *mut ::generics_golden::RenamedSubGeneric<i32>,
) -> () {
    unsafe {
        __ret_ptr.write(::generics_golden::instantiate_sub_generic());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_instantiate_uwith_uarray(
    __ret_ptr: *mut ::generics_golden::GenericStruct<[i32; 4]>,
) -> () {
    unsafe {
        __ret_ptr.write(::generics_golden::instantiate_with_array());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_use_ualiased_ugeneric(
    x: *mut ::generics_golden::GenericStruct<i64>,
    __ret_ptr: *mut ::generics_golden::GenericStruct<i64>,
) -> () {
    unsafe {
        let x = x.read();
        __ret_ptr.write(::generics_golden::use_aliased_generic(x));
    }
}
const _: () = assert!(
    ::core::mem::size_of::<
        ::generics_golden::DerivedGeneric<::generics_golden::NotPartialEqOrDefault>,
    >() == 4
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::generics_golden::DerivedGeneric<::generics_golden::NotPartialEqOrDefault>,
    >() == 4
);
const _: () = assert!(
    ::core::mem::offset_of!(
        ::generics_golden::DerivedGeneric<::generics_golden::NotPartialEqOrDefault>,
        value
    ) == 0
);
const _: () = assert!(::core::mem::size_of::<::generics_golden::DerivedGeneric<i32>>() == 4);
const _: () = assert!(::core::mem::align_of::<::generics_golden::DerivedGeneric<i32>>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_ugenerics_ugolden_x0000003a_x0000003aDerivedGeneric_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aDerivedGeneric_x0000003cT_x0000003e_ui32(
    __self: &'static ::generics_golden::DerivedGeneric<i32>,
    other: &'static ::generics_golden::DerivedGeneric<i32>,
) -> bool {
    unsafe {
        <::generics_golden::DerivedGeneric<i32> as ::core::cmp::PartialEq<
            ::generics_golden::DerivedGeneric<i32>,
        >>::eq(__self, other)
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Hash_uhash_ugenerics_ugolden_x0000003a_x0000003aDerivedGeneric_x0000003ci32_x0000003e(
    self_: &::generics_golden::DerivedGeneric<i32>,
) -> u64 {
    ::crubit_support::hash::hash_u64(self_)
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Ord_ucmp_ugenerics_ugolden_x0000003a_x0000003aDerivedGeneric_x0000003ci32_x0000003e(
    lhs: &::generics_golden::DerivedGeneric<i32>,
    rhs: &::generics_golden::DerivedGeneric<i32>,
) -> i8 {
    <::generics_golden::DerivedGeneric<i32> as ::core::cmp::Ord>::cmp(lhs, rhs) as i8
}
const _: () = assert!(::core::mem::offset_of!(::generics_golden::DerivedGeneric<i32>, value) == 0);
const _: () = assert!(::core::mem::size_of::<::generics_golden::GenericEnum<i32>>() == 8);
const _: () = assert!(::core::mem::align_of::<::generics_golden::GenericEnum<i32>>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Value_ui32(
    __param_0: i32,
    __ret_ptr: *mut ::generics_golden::GenericEnum<i32>,
) -> () {
    unsafe {
        __ret_ptr.write(<::generics_golden::GenericEnum<i32>>::Value(__param_0));
    }
}
const _: () = assert!(::core::mem::size_of::<::generics_golden::GenericStruct<[i32; 4]>>() == 16);
const _: () = assert!(::core::mem::align_of::<::generics_golden::GenericStruct<[i32; 4]>>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_udefault_u_x0000005bi32_x0000003b_x000000204_x0000005d(
    __self: &'static ::generics_golden::GenericStruct<[i32; 4]>,
) -> bool {
    unsafe { <::generics_golden::GenericStruct<[i32; 4]>>::is_default(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new_u_x0000005bi32_x0000003b_x000000204_x0000005d(
    value: *mut [i32; 4],
    __ret_ptr: *mut ::generics_golden::GenericStruct<[i32; 4]>,
) -> () {
    unsafe {
        let value = value.read();
        __ret_ptr.write(<::generics_golden::GenericStruct<[i32; 4]>>::new(value));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uref_u_x0000005bi32_x0000003b_x000000204_x0000005d(
    __self: &'static ::generics_golden::GenericStruct<[i32; 4]>,
) -> &'static [i32; 4] {
    unsafe { <::generics_golden::GenericStruct<[i32; 4]>>::get_ref(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_set_u_x0000005bi32_x0000003b_x000000204_x0000005d(
    __self: &'static mut ::generics_golden::GenericStruct<[i32; 4]>,
    value: *mut [i32; 4],
) -> () {
    unsafe {
        let value = value.read();
        <::generics_golden::GenericStruct<[i32; 4]>>::set(__self, value)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_uinner_u_x0000005bi32_x0000003b_x000000204_x0000005d(
    __self: *mut ::generics_golden::GenericStruct<[i32; 4]>,
    __ret_ptr: *mut [i32; 4],
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr.write(<::generics_golden::GenericStruct<[i32; 4]>>::into_inner(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_type_usize_u_x0000005bi32_x0000003b_x000000204_x0000005d(
) -> usize {
    unsafe { <::generics_golden::GenericStruct<[i32; 4]>>::type_size() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_borrow_uwith_uexplicit_ult_u_x0000005bi32_x0000003b_x000000204_x0000005d(
    __self: &'static ::generics_golden::GenericStruct<[i32; 4]>,
) -> &'static [i32; 4] {
    unsafe { <::generics_golden::GenericStruct<[i32; 4]>>::borrow_with_explicit_lt(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_umulti_u_x0000005bi32_x0000003b_x000000204_x0000005d(
    __self: &'static ::generics_golden::GenericStruct<[i32; 4]>,
    __ret_ptr: *mut ::generics_golden::MultiGenericStruct<[i32; 4], [i32; 4]>,
) -> () {
    unsafe {
        __ret_ptr.write(<::generics_golden::GenericStruct<[i32; 4]>>::into_multi(__self));
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::generics_golden::GenericStruct<[i32; 4]>, value) == 0);
const _: () = assert!(
    ::core::mem::size_of::<::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>>(
    ) == 4
);
const _: () = assert!(
    ::core::mem::align_of::<::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>>(
    ) == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
    value: *mut ::generics_golden::GenericStruct<u32>,
    __ret_ptr: *mut ::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>,
) -> () {
    unsafe {
        let value = value.read();
        __ret_ptr.write(
            <::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>>::new(value),
        );
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uref_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
    __self: &'static ::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>,
) -> &'static ::generics_golden::GenericStruct<u32> {
    unsafe {
        <::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>>::get_ref(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_set_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
    __self: &'static mut ::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>,
    value: *mut ::generics_golden::GenericStruct<u32>,
) -> () {
    unsafe {
        let value = value.read();
        <::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>>::set(
            __self, value,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_uinner_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
    __self: *mut ::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>,
    __ret_ptr: *mut ::generics_golden::GenericStruct<u32>,
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr.write(
            <::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>>::into_inner(
                __self,
            ),
        );
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_type_usize_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
) -> usize {
    unsafe {
        <::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>>::type_size()
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_borrow_uwith_uexplicit_ult_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
    __self: &'static ::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>,
) -> &'static ::generics_golden::GenericStruct<u32> {
    unsafe {
        <::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>>::borrow_with_explicit_lt(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Add_uadd_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
    __self: *mut ::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>,
    rhs: *mut ::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>,
    __ret_ptr: *mut ::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>,
) -> () {
    unsafe {
        let __self = __self.read();
        let rhs = rhs.read();
        __ret_ptr.write(<::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>as::core::ops::Add<::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>>>::add(__self,rhs));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_ToString_uto_ustring_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cgenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e_x0000003e(
    __self: &'static ::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>,
    __ret_ptr: *mut ::alloc::string::String,
) -> () {
    unsafe {
        __ret_ptr.write(<::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>as::alloc::string::ToString>::to_string(__self));
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(
        ::generics_golden::GenericStruct<::generics_golden::GenericStruct<u32>>,
        value
    ) == 0
);
const _: () = assert!(
    ::core::mem::size_of::<
        ::generics_golden::GenericStruct<::generics_golden::NotPartialEqOrDefault>,
    >() == 4
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::generics_golden::GenericStruct<::generics_golden::NotPartialEqOrDefault>,
    >() == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault(
    value: *mut ::generics_golden::NotPartialEqOrDefault,
    __ret_ptr: *mut ::generics_golden::GenericStruct<::generics_golden::NotPartialEqOrDefault>,
) -> () {
    unsafe {
        let value = value.read();
        __ret_ptr.write(
            <::generics_golden::GenericStruct<::generics_golden::NotPartialEqOrDefault>>::new(
                value,
            ),
        );
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uref_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault(
    __self: &'static ::generics_golden::GenericStruct<::generics_golden::NotPartialEqOrDefault>,
) -> &'static ::generics_golden::NotPartialEqOrDefault {
    unsafe {
        <::generics_golden::GenericStruct<::generics_golden::NotPartialEqOrDefault>>::get_ref(
            __self,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_set_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault(
    __self: &'static mut ::generics_golden::GenericStruct<::generics_golden::NotPartialEqOrDefault>,
    value: *mut ::generics_golden::NotPartialEqOrDefault,
) -> () {
    unsafe {
        let value = value.read();
        <::generics_golden::GenericStruct<::generics_golden::NotPartialEqOrDefault>>::set(
            __self, value,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_uinner_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault(
    __self: *mut ::generics_golden::GenericStruct<::generics_golden::NotPartialEqOrDefault>,
    __ret_ptr: *mut ::generics_golden::NotPartialEqOrDefault,
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr.write(<::generics_golden::GenericStruct<::generics_golden::NotPartialEqOrDefault>>::into_inner(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_type_usize_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault(
) -> usize {
    unsafe {
        <::generics_golden::GenericStruct<::generics_golden::NotPartialEqOrDefault>>::type_size()
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_borrow_uwith_uexplicit_ult_ugenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault(
    __self: &'static ::generics_golden::GenericStruct<::generics_golden::NotPartialEqOrDefault>,
) -> &'static ::generics_golden::NotPartialEqOrDefault {
    unsafe {
        <::generics_golden::GenericStruct<::generics_golden::NotPartialEqOrDefault>>::borrow_with_explicit_lt(__self)
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(
        ::generics_golden::GenericStruct<::generics_golden::NotPartialEqOrDefault>,
        value
    ) == 0
);
const _: () = assert!(
    ::core::mem::size_of::<::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>>(
    ) == 4
);
const _: () = assert!(
    ::core::mem::align_of::<::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>>(
    ) == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_deref_uvalue_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
    __self: &'static ::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>,
) -> &'static i32 {
    unsafe {
        <::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>>::deref_value(
            __self,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_udefault_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
    __self: &'static ::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>,
) -> bool {
    unsafe {
        <::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>>::is_default(
            __self,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
    value: *mut ::generics_golden::PartialEqAndDefault,
    __ret_ptr: *mut ::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>,
) -> () {
    unsafe {
        let value = value.read();
        __ret_ptr.write(
            <::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>>::new(value),
        );
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uref_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
    __self: &'static ::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>,
) -> &'static ::generics_golden::PartialEqAndDefault {
    unsafe {
        <::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>>::get_ref(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_set_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
    __self: &'static mut ::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>,
    value: *mut ::generics_golden::PartialEqAndDefault,
) -> () {
    unsafe {
        let value = value.read();
        <::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>>::set(
            __self, value,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_uinner_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
    __self: *mut ::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>,
    __ret_ptr: *mut ::generics_golden::PartialEqAndDefault,
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr.write(
            <::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>>::into_inner(
                __self,
            ),
        );
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_type_usize_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
) -> usize {
    unsafe {
        <::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>>::type_size()
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_borrow_uwith_uexplicit_ult_ugenerics_ugolden_x0000003a_x0000003aPartialEqAndDefault(
    __self: &'static ::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>,
) -> &'static ::generics_golden::PartialEqAndDefault {
    unsafe {
        <::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>>::borrow_with_explicit_lt(__self)
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(
        ::generics_golden::GenericStruct<::generics_golden::PartialEqAndDefault>,
        value
    ) == 0
);
const _: () = assert!(::core::mem::size_of::<::generics_golden::GenericStruct<i32>>() == 4);
const _: () = assert!(::core::mem::align_of::<::generics_golden::GenericStruct<i32>>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_udefault_ui32(
    __self: &'static ::generics_golden::GenericStruct<i32>,
) -> bool {
    unsafe { <::generics_golden::GenericStruct<i32>>::is_default(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new_ui32(
    value: i32,
    __ret_ptr: *mut ::generics_golden::GenericStruct<i32>,
) -> () {
    unsafe {
        __ret_ptr.write(<::generics_golden::GenericStruct<i32>>::new(value));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uref_ui32(
    __self: &'static ::generics_golden::GenericStruct<i32>,
) -> &'static i32 {
    unsafe { <::generics_golden::GenericStruct<i32>>::get_ref(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_set_ui32(
    __self: &'static mut ::generics_golden::GenericStruct<i32>,
    value: i32,
) -> () {
    unsafe { <::generics_golden::GenericStruct<i32>>::set(__self, value) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_uinner_ui32(
    __self: *mut ::generics_golden::GenericStruct<i32>,
) -> i32 {
    unsafe {
        let __self = __self.read();
        <::generics_golden::GenericStruct<i32>>::into_inner(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_type_usize_ui32() -> usize {
    unsafe { <::generics_golden::GenericStruct<i32>>::type_size() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_borrow_uwith_uexplicit_ult_ui32(
    __self: &'static ::generics_golden::GenericStruct<i32>,
) -> &'static i32 {
    unsafe { <::generics_golden::GenericStruct<i32>>::borrow_with_explicit_lt(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_umulti_ui32(
    __self: &'static ::generics_golden::GenericStruct<i32>,
    __ret_ptr: *mut ::generics_golden::MultiGenericStruct<i32, i32>,
) -> () {
    unsafe {
        __ret_ptr.write(<::generics_golden::GenericStruct<i32>>::into_multi(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_add_uone(
    __self: &'static ::generics_golden::GenericStruct<i32>,
) -> i32 {
    unsafe { <::generics_golden::GenericStruct<i32>>::add_one(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_concrete_uid() -> i32 {
    unsafe { <::generics_golden::GenericStruct<i32>>::concrete_id() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Add_uadd_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ui32(
    __self: *mut ::generics_golden::GenericStruct<i32>,
    rhs: *mut ::generics_golden::GenericStruct<i32>,
    __ret_ptr: *mut ::generics_golden::GenericStruct<i32>,
) -> () {
    unsafe {
        let __self = __self.read();
        let rhs = rhs.read();
        __ret_ptr.write(<::generics_golden::GenericStruct<i32> as ::core::ops::Add<
            ::generics_golden::GenericStruct<i32>,
        >>::add(__self, rhs));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_ToString_uto_ustring_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003ci32_x0000003e(
    __self: &'static ::generics_golden::GenericStruct<i32>,
    __ret_ptr: *mut ::alloc::string::String,
) -> () {
    unsafe {
        __ret_ptr.write(
            <::generics_golden::GenericStruct<i32> as ::alloc::string::ToString>::to_string(__self),
        );
    }
}
const _: () = assert!(::core::mem::offset_of!(::generics_golden::GenericStruct<i32>, value) == 0);
const _: () = assert!(::core::mem::size_of::<::generics_golden::GenericStruct<i64>>() == 8);
const _: () = assert!(::core::mem::align_of::<::generics_golden::GenericStruct<i64>>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_udefault_ui64(
    __self: &'static ::generics_golden::GenericStruct<i64>,
) -> bool {
    unsafe { <::generics_golden::GenericStruct<i64>>::is_default(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new_ui64(
    value: i64,
    __ret_ptr: *mut ::generics_golden::GenericStruct<i64>,
) -> () {
    unsafe {
        __ret_ptr.write(<::generics_golden::GenericStruct<i64>>::new(value));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uref_ui64(
    __self: &'static ::generics_golden::GenericStruct<i64>,
) -> &'static i64 {
    unsafe { <::generics_golden::GenericStruct<i64>>::get_ref(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_set_ui64(
    __self: &'static mut ::generics_golden::GenericStruct<i64>,
    value: i64,
) -> () {
    unsafe { <::generics_golden::GenericStruct<i64>>::set(__self, value) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_uinner_ui64(
    __self: *mut ::generics_golden::GenericStruct<i64>,
) -> i64 {
    unsafe {
        let __self = __self.read();
        <::generics_golden::GenericStruct<i64>>::into_inner(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_type_usize_ui64() -> usize {
    unsafe { <::generics_golden::GenericStruct<i64>>::type_size() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_borrow_uwith_uexplicit_ult_ui64(
    __self: &'static ::generics_golden::GenericStruct<i64>,
) -> &'static i64 {
    unsafe { <::generics_golden::GenericStruct<i64>>::borrow_with_explicit_lt(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_umulti_ui64(
    __self: &'static ::generics_golden::GenericStruct<i64>,
    __ret_ptr: *mut ::generics_golden::MultiGenericStruct<i64, i64>,
) -> () {
    unsafe {
        __ret_ptr.write(<::generics_golden::GenericStruct<i64>>::into_multi(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_concrete_uid() -> i32 {
    unsafe { <::generics_golden::GenericStruct<i64>>::concrete_id() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Add_uadd_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ui64(
    __self: *mut ::generics_golden::GenericStruct<i64>,
    rhs: *mut ::generics_golden::GenericStruct<i64>,
    __ret_ptr: *mut ::generics_golden::GenericStruct<i64>,
) -> () {
    unsafe {
        let __self = __self.read();
        let rhs = rhs.read();
        __ret_ptr.write(<::generics_golden::GenericStruct<i64> as ::core::ops::Add<
            ::generics_golden::GenericStruct<i64>,
        >>::add(__self, rhs));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_ToString_uto_ustring_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003ci64_x0000003e(
    __self: &'static ::generics_golden::GenericStruct<i64>,
    __ret_ptr: *mut ::alloc::string::String,
) -> () {
    unsafe {
        __ret_ptr.write(
            <::generics_golden::GenericStruct<i64> as ::alloc::string::ToString>::to_string(__self),
        );
    }
}
const _: () = assert!(::core::mem::offset_of!(::generics_golden::GenericStruct<i64>, value) == 0);
const _: () = assert!(::core::mem::size_of::<::generics_golden::GenericStruct<u32>>() == 4);
const _: () = assert!(::core::mem::align_of::<::generics_golden::GenericStruct<u32>>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_udefault_uu32(
    __self: &'static ::generics_golden::GenericStruct<u32>,
) -> bool {
    unsafe { <::generics_golden::GenericStruct<u32>>::is_default(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new_uu32(
    value: u32,
    __ret_ptr: *mut ::generics_golden::GenericStruct<u32>,
) -> () {
    unsafe {
        __ret_ptr.write(<::generics_golden::GenericStruct<u32>>::new(value));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uref_uu32(
    __self: &'static ::generics_golden::GenericStruct<u32>,
) -> &'static u32 {
    unsafe { <::generics_golden::GenericStruct<u32>>::get_ref(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_set_uu32(
    __self: &'static mut ::generics_golden::GenericStruct<u32>,
    value: u32,
) -> () {
    unsafe { <::generics_golden::GenericStruct<u32>>::set(__self, value) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_uinner_uu32(
    __self: *mut ::generics_golden::GenericStruct<u32>,
) -> u32 {
    unsafe {
        let __self = __self.read();
        <::generics_golden::GenericStruct<u32>>::into_inner(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_type_usize_uu32() -> usize {
    unsafe { <::generics_golden::GenericStruct<u32>>::type_size() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_borrow_uwith_uexplicit_ult_uu32(
    __self: &'static ::generics_golden::GenericStruct<u32>,
) -> &'static u32 {
    unsafe { <::generics_golden::GenericStruct<u32>>::borrow_with_explicit_lt(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_umulti_uu32(
    __self: &'static ::generics_golden::GenericStruct<u32>,
    __ret_ptr: *mut ::generics_golden::MultiGenericStruct<u32, u32>,
) -> () {
    unsafe {
        __ret_ptr.write(<::generics_golden::GenericStruct<u32>>::into_multi(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Add_uadd_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cT_x0000003e_uu32(
    __self: *mut ::generics_golden::GenericStruct<u32>,
    rhs: *mut ::generics_golden::GenericStruct<u32>,
    __ret_ptr: *mut ::generics_golden::GenericStruct<u32>,
) -> () {
    unsafe {
        let __self = __self.read();
        let rhs = rhs.read();
        __ret_ptr.write(<::generics_golden::GenericStruct<u32> as ::core::ops::Add<
            ::generics_golden::GenericStruct<u32>,
        >>::add(__self, rhs));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_ToString_uto_ustring_ugenerics_ugolden_x0000003a_x0000003aGenericStruct_x0000003cu32_x0000003e(
    __self: &'static ::generics_golden::GenericStruct<u32>,
    __ret_ptr: *mut ::alloc::string::String,
) -> () {
    unsafe {
        __ret_ptr.write(
            <::generics_golden::GenericStruct<u32> as ::alloc::string::ToString>::to_string(__self),
        );
    }
}
const _: () = assert!(::core::mem::offset_of!(::generics_golden::GenericStruct<u32>, value) == 0);
const _: () = assert!(::core::mem::size_of::<::generics_golden::GenericUnion<u32>>() == 4);
const _: () = assert!(::core::mem::align_of::<::generics_golden::GenericUnion<u32>>() == 4);
const _: () = assert!(::core::mem::offset_of!(::generics_golden::GenericUnion<u32>, val) == 0);
const _: () = assert!(::core::mem::offset_of!(::generics_golden::GenericUnion<u32>, raw) == 0);
const _: () = assert!(
    ::core::mem::size_of::<
        ::generics_golden::GenericWithDrop<::generics_golden::NotPartialEqOrDefault>,
    >() == 4
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::generics_golden::GenericWithDrop<::generics_golden::NotPartialEqOrDefault>,
    >() == 4
);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003cgenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault_x0000003e(
    __self: *mut ::generics_golden::GenericWithDrop<::generics_golden::NotPartialEqOrDefault>,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_ugenerics_ugolden_x0000003a_x0000003aWrapper_x0000003cgenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003cgenerics_ugolden_x0000003a_x0000003aNotPartialEqOrDefault_x0000003e(
    __self: *mut ::generics_golden::Wrapper<::generics_golden::NotPartialEqOrDefault>,
    __ret_ptr: *mut ::generics_golden::GenericWithDrop<::generics_golden::NotPartialEqOrDefault>,
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr.write(<::generics_golden::Wrapper<::generics_golden::NotPartialEqOrDefault>as::core::convert::Into<::generics_golden::GenericWithDrop<::generics_golden::NotPartialEqOrDefault>>>::into(__self));
    }
}
const _: () = assert!(
    ::core::mem::offset_of!(
        ::generics_golden::GenericWithDrop<::generics_golden::NotPartialEqOrDefault>,
        value
    ) == 0
);
const _: () = assert!(::core::mem::size_of::<::generics_golden::GenericWithDrop<i32>>() == 4);
const _: () = assert!(::core::mem::align_of::<::generics_golden::GenericWithDrop<i32>>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003ci32_x0000003e(
    __ret_ptr: *mut ::generics_golden::GenericWithDrop<i32>,
) -> () {
    unsafe {
        __ret_ptr.write(<::generics_golden::GenericWithDrop<i32>as::core::default::Default>::default());
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003ci32_x0000003e(
    __self: *mut ::generics_golden::GenericWithDrop<i32>,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003ci32_x0000003e_ugenerics_ugolden_x0000003a_x0000003aWrapper_x0000003ci32_x0000003e(
    __self: *mut ::generics_golden::GenericWithDrop<i32>,
    __ret_ptr: *mut ::generics_golden::Wrapper<i32>,
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr.write(<::generics_golden::GenericWithDrop<i32> as ::core::convert::Into<
            ::generics_golden::Wrapper<i32>,
        >>::into(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_ugenerics_ugolden_x0000003a_x0000003aWrapper_x0000003ci32_x0000003e_ugenerics_ugolden_x0000003a_x0000003aGenericWithDrop_x0000003ci32_x0000003e(
    __self: *mut ::generics_golden::Wrapper<i32>,
    __ret_ptr: *mut ::generics_golden::GenericWithDrop<i32>,
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr.write(<::generics_golden::Wrapper<i32> as ::core::convert::Into<
            ::generics_golden::GenericWithDrop<i32>,
        >>::into(__self));
    }
}
const _: () = assert!(::core::mem::offset_of!(::generics_golden::GenericWithDrop<i32>, value) == 0);
const _: () =
    assert!(::core::mem::size_of::<::generics_golden::LifetimeAndTypeGeneric<'static, i32>>() == 8);
const _: () = assert!(
    ::core::mem::align_of::<::generics_golden::LifetimeAndTypeGeneric<'static, i32>>() == 8
);
const _: () = assert!(
    ::core::mem::offset_of!(::generics_golden::LifetimeAndTypeGeneric<'static, i32>, reference)
        == 0
);
const _: () =
    assert!(::core::mem::size_of::<::generics_golden::MultiGenericStruct<bool, f64>>() == 32);
const _: () =
    assert!(::core::mem::align_of::<::generics_golden::MultiGenericStruct<bool, f64>>() == 8);
const _: () =
    assert!(::core::mem::offset_of!(::generics_golden::MultiGenericStruct<bool,f64>,seconds) == 0);
const _: () =
    assert!(::core::mem::offset_of!(::generics_golden::MultiGenericStruct<bool,f64>,first) == 24);
const _: () =
    assert!(::core::mem::size_of::<::generics_golden::MultiGenericStruct<bool, i32>>() == 32);
const _: () =
    assert!(::core::mem::align_of::<::generics_golden::MultiGenericStruct<bool, i32>>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_first_uand_ulen_ubool(
    __self: &'static ::generics_golden::MultiGenericStruct<bool, i32>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let (__rs_return_value_0, __rs_return_value_1) =
            <::generics_golden::MultiGenericStruct<bool, i32>>::first_and_len(__self);
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        ::core::ptr::write(__ret_ptr_0 as *mut _, __rs_return_value_0);
        ::core::ptr::write(__ret_ptr_1 as *mut _, __rs_return_value_1);
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::generics_golden::MultiGenericStruct<bool,i32>,seconds) == 0);
const _: () =
    assert!(::core::mem::offset_of!(::generics_golden::MultiGenericStruct<bool,i32>,first) == 24);
const _: () = assert!(
    ::core::mem::size_of::<
        ::generics_golden::PhantomGeneric<::generics_golden::SelfReferentialViaPhantom>,
    >() == 4
);
const _: () = assert!(
    ::core::mem::align_of::<
        ::generics_golden::PhantomGeneric<::generics_golden::SelfReferentialViaPhantom>,
    >() == 4
);
const _: () = assert!(
    ::core::mem::offset_of!(
        ::generics_golden::PhantomGeneric<::generics_golden::SelfReferentialViaPhantom>,
        id
    ) == 0
);
const _: () = assert!(
    ::core::mem::offset_of!(
        ::generics_golden::PhantomGeneric<::generics_golden::SelfReferentialViaPhantom>,
        marker
    ) == 4
);
const _: () = assert!(::core::mem::size_of::<::generics_golden::Ping<i32>>() == 4);
const _: () = assert!(::core::mem::align_of::<::generics_golden::Ping<i32>>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_to_upong_ui32(
    __self: &'static ::generics_golden::Ping<i32>,
    __ret_ptr: *mut ::generics_golden::Pong<::generics_golden::Wrapper<i32>>,
) -> () {
    unsafe {
        __ret_ptr.write(<::generics_golden::Ping<i32>>::to_pong(__self));
    }
}
const _: () = assert!(::core::mem::offset_of!(::generics_golden::Ping<i32>, 0) == 0);
const _: () = assert!(::core::mem::size_of::<::generics_golden::RenamedSubGeneric<i32>>() == 4);
const _: () = assert!(::core::mem::align_of::<::generics_golden::RenamedSubGeneric<i32>>() == 4);
const _: () =
    assert!(::core::mem::offset_of!(::generics_golden::RenamedSubGeneric<i32>, item) == 0);
const _: () = assert!(
    ::core::mem::size_of::<::generics_golden::MultiGenericStruct<[i32; 4], [i32; 4]>>() == 40
);
const _: () = assert!(
    ::core::mem::align_of::<::generics_golden::MultiGenericStruct<[i32; 4], [i32; 4]>>() == 8
);
const _: () = assert!(
    ::core::mem::offset_of!(::generics_golden::MultiGenericStruct<[i32;4],[i32;4]>,seconds) == 0
);
const _: () = assert!(
    ::core::mem::offset_of!(::generics_golden::MultiGenericStruct<[i32;4],[i32;4]>,first) == 24
);
const _: () =
    assert!(::core::mem::size_of::<::generics_golden::MultiGenericStruct<i32, i32>>() == 32);
const _: () =
    assert!(::core::mem::align_of::<::generics_golden::MultiGenericStruct<i32, i32>>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_first_uand_ulen_ui32(
    __self: &'static ::generics_golden::MultiGenericStruct<i32, i32>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let (__rs_return_value_0, __rs_return_value_1) =
            <::generics_golden::MultiGenericStruct<i32, i32>>::first_and_len(__self);
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        ::core::ptr::write(__ret_ptr_0 as *mut _, __rs_return_value_0);
        ::core::ptr::write(__ret_ptr_1 as *mut _, __rs_return_value_1);
    }
}
const _: () =
    assert!(::core::mem::offset_of!(::generics_golden::MultiGenericStruct<i32,i32>,seconds) == 0);
const _: () =
    assert!(::core::mem::offset_of!(::generics_golden::MultiGenericStruct<i32,i32>,first) == 24);
const _: () =
    assert!(::core::mem::size_of::<::generics_golden::MultiGenericStruct<i64, i64>>() == 32);
const _: () =
    assert!(::core::mem::align_of::<::generics_golden::MultiGenericStruct<i64, i64>>() == 8);
const _: () =
    assert!(::core::mem::offset_of!(::generics_golden::MultiGenericStruct<i64,i64>,seconds) == 0);
const _: () =
    assert!(::core::mem::offset_of!(::generics_golden::MultiGenericStruct<i64,i64>,first) == 24);
const _: () =
    assert!(::core::mem::size_of::<::generics_golden::MultiGenericStruct<u32, u32>>() == 32);
const _: () =
    assert!(::core::mem::align_of::<::generics_golden::MultiGenericStruct<u32, u32>>() == 8);
const _: () =
    assert!(::core::mem::offset_of!(::generics_golden::MultiGenericStruct<u32,u32>,seconds) == 0);
const _: () =
    assert!(::core::mem::offset_of!(::generics_golden::MultiGenericStruct<u32,u32>,first) == 24);
const _: () = assert!(
    ::core::mem::size_of::<::generics_golden::Pong<::generics_golden::Wrapper<i32>>>() == 4
);
const _: () = assert!(
    ::core::mem::align_of::<::generics_golden::Pong<::generics_golden::Wrapper<i32>>>() == 4
);
const _: () = assert!(
    ::core::mem::offset_of!(::generics_golden::Pong<::generics_golden::Wrapper<i32>>, 0) == 0
);
const _: () = assert!(
    ::core::mem::size_of::<::generics_golden::Wrapper<::generics_golden::NotPartialEqOrDefault>>()
        == 4
);
const _: () = assert!(
    ::core::mem::align_of::<::generics_golden::Wrapper<::generics_golden::NotPartialEqOrDefault>>()
        == 4
);
const _: () = assert!(
    ::core::mem::offset_of!(
        ::generics_golden::Wrapper<::generics_golden::NotPartialEqOrDefault>,
        0
    ) == 0
);
const _: () = assert!(::core::mem::size_of::<::generics_golden::Wrapper<i32>>() == 4);
const _: () = assert!(::core::mem::align_of::<::generics_golden::Wrapper<i32>>() == 4);
const _: () = assert!(::core::mem::offset_of!(::generics_golden::Wrapper<i32>, 0) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cf64_x0000003e(
    __self: &'static ::alloc::vec::Vec<f64>,
    __ret_ptr: *mut ::alloc::vec::Vec<f64>,
) -> () {
    unsafe {
        __ret_ptr.write(<::alloc::vec::Vec<f64> as ::core::clone::Clone>::clone(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cf64_x0000003e(
    __self: &'static mut ::alloc::vec::Vec<f64>,
    source: &'static ::alloc::vec::Vec<f64>,
) -> () {
    unsafe { <::alloc::vec::Vec<f64> as ::core::clone::Clone>::clone_from(__self, source) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
    __self: &'static ::alloc::vec::Vec<i32>,
    __ret_ptr: *mut ::alloc::vec::Vec<i32>,
) -> () {
    unsafe {
        __ret_ptr.write(<::alloc::vec::Vec<i32> as ::core::clone::Clone>::clone(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
    __self: &'static mut ::alloc::vec::Vec<i32>,
    source: &'static ::alloc::vec::Vec<i32>,
) -> () {
    unsafe { <::alloc::vec::Vec<i32> as ::core::clone::Clone>::clone_from(__self, source) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003c_x0000005bi32_x0000003b_x000000204_x0000005d_x0000003e(
    __self: &'static ::alloc::vec::Vec<[i32; 4]>,
    __ret_ptr: *mut ::alloc::vec::Vec<[i32; 4]>,
) -> () {
    unsafe {
        __ret_ptr.write(<::alloc::vec::Vec<[i32; 4]> as ::core::clone::Clone>::clone(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003c_x0000005bi32_x0000003b_x000000204_x0000005d_x0000003e(
    __self: &'static mut ::alloc::vec::Vec<[i32; 4]>,
    source: &'static ::alloc::vec::Vec<[i32; 4]>,
) -> () {
    unsafe { <::alloc::vec::Vec<[i32; 4]> as ::core::clone::Clone>::clone_from(__self, source) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci64_x0000003e(
    __self: &'static ::alloc::vec::Vec<i64>,
    __ret_ptr: *mut ::alloc::vec::Vec<i64>,
) -> () {
    unsafe {
        __ret_ptr.write(<::alloc::vec::Vec<i64> as ::core::clone::Clone>::clone(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci64_x0000003e(
    __self: &'static mut ::alloc::vec::Vec<i64>,
    source: &'static ::alloc::vec::Vec<i64>,
) -> () {
    unsafe { <::alloc::vec::Vec<i64> as ::core::clone::Clone>::clone_from(__self, source) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu32_x0000003e(
    __self: &'static ::alloc::vec::Vec<u32>,
    __ret_ptr: *mut ::alloc::vec::Vec<u32>,
) -> () {
    unsafe {
        __ret_ptr.write(<::alloc::vec::Vec<u32> as ::core::clone::Clone>::clone(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu32_x0000003e(
    __self: &'static mut ::alloc::vec::Vec<u32>,
    source: &'static ::alloc::vec::Vec<u32>,
) -> () {
    unsafe { <::alloc::vec::Vec<u32> as ::core::clone::Clone>::clone_from(__self, source) }
}

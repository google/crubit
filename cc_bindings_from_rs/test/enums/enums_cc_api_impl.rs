// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// enums_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::enums_golden::qr_error::QrError>() == 24);
const _: () = assert!(::std::mem::align_of::<::enums_golden::qr_error::QrError>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Structured(
    __param_0: &'static mut ::core::mem::MaybeUninit<::enums_golden::qr_error::StructuredQrError>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __param_0 = __param_0.assume_init_read();
        let __rs_return_value = ::enums_golden::qr_error::QrError::Structured(__param_0);
        (__ret_ptr as *mut ::enums_golden::qr_error::QrError).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_udata_utoo_ulong(
    __self: &'static ::enums_golden::qr_error::QrError,
) -> bool {
    unsafe { ::enums_golden::qr_error::QrError::is_data_too_long(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq(
    __self: &'static ::enums_golden::qr_error::QrError,
    other: &'static ::enums_golden::qr_error::QrError,
) -> bool {
    unsafe {
        <::enums_golden::qr_error::QrError as ::core::cmp::PartialEq<
            ::enums_golden::qr_error::QrError,
        >>::eq(__self, other)
    }
}
const _: () = assert!(::std::mem::size_of::<::enums_golden::qr_error::StructuredQrError>() == 24);
const _: () = assert!(::std::mem::align_of::<::enums_golden::qr_error::StructuredQrError>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_TotalMismatch(
    __param_0: usize,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::enums_golden::qr_error::StructuredQrError::TotalMismatch(__param_0);
        (__ret_ptr as *mut ::enums_golden::qr_error::StructuredQrError).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_SeqGreaterThanTotal(
    __param_0: u8,
    __param_1: u8,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::enums_golden::qr_error::StructuredQrError::SeqGreaterThanTotal(__param_0, __param_1);
        (__ret_ptr as *mut ::enums_golden::qr_error::StructuredQrError).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_LengthMismatch(
    __param_0: usize,
    __param_1: usize,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::enums_golden::qr_error::StructuredQrError::LengthMismatch(__param_0, __param_1);
        (__ret_ptr as *mut ::enums_golden::qr_error::StructuredQrError).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_UnsupportedVersion(
    __param_0: i16,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::enums_golden::qr_error::StructuredQrError::UnsupportedVersion(__param_0);
        (__ret_ptr as *mut ::enums_golden::qr_error::StructuredQrError).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_SplitMax16(
    __param_0: usize,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::enums_golden::qr_error::StructuredQrError::SplitMax16(__param_0);
        (__ret_ptr as *mut ::enums_golden::qr_error::StructuredQrError).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq(
    __self: &'static ::enums_golden::qr_error::StructuredQrError,
    other: &'static ::enums_golden::qr_error::StructuredQrError,
) -> bool {
    unsafe {
        <::enums_golden::qr_error::StructuredQrError as ::core::cmp::PartialEq<
            ::enums_golden::qr_error::StructuredQrError,
        >>::eq(__self, other)
    }
}
const _: () = assert!(::std::mem::size_of::<::enums_golden::repr_128::ReprI128>() == 16);
const _: () = assert!(::std::mem::align_of::<::enums_golden::repr_128::ReprI128>() == 16);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_umin_ui128(
    __self: &'static ::enums_golden::repr_128::ReprI128,
) -> bool {
    unsafe { ::enums_golden::repr_128::ReprI128::is_min_i128(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_umax_ui128(
    __self: &'static ::enums_golden::repr_128::ReprI128,
) -> bool {
    unsafe { ::enums_golden::repr_128::ReprI128::is_max_i128(__self) }
}
const _: () = assert!(::std::mem::size_of::<::enums_golden::repr_128::ReprU128>() == 16);
const _: () = assert!(::std::mem::align_of::<::enums_golden::repr_128::ReprU128>() == 16);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_umax_uu128(
    __self: &'static ::enums_golden::repr_128::ReprU128,
) -> bool {
    unsafe { ::enums_golden::repr_128::ReprU128::is_max_u128(__self) }
}
const _: () = assert!(::std::mem::size_of::<::enums_golden::repr_c::MyEnum>() == 40);
const _: () = assert!(::std::mem::align_of::<::enums_golden::repr_c::MyEnum>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::enums_golden::repr_c::MyEnum as ::core::default::Default>::default();
        (__ret_ptr as *mut ::enums_golden::repr_c::MyEnum).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_E(
    __param_0: &'static mut ::core::mem::MaybeUninit<::alloc::string::String>,
    __param_1: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __param_0 = __param_0.assume_init_read();
        let __rs_return_value = ::enums_golden::repr_c::MyEnum::E(__param_0, __param_1);
        (__ret_ptr as *mut ::enums_golden::repr_c::MyEnum).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_A(
    __param_0: i32,
    __param_1: i64,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::enums_golden::repr_c::MyEnum::A(__param_0, __param_1);
        (__ret_ptr as *mut ::enums_golden::repr_c::MyEnum).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::enums_golden::repr_c::MyEnum>,
) {
    unsafe { __self.assume_init_drop() };
}
const _: () =
    assert!(::std::mem::size_of::<::enums_golden::repr_c::ReprCWithExtremeDiscriminants>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::enums_golden::repr_c::ReprCWithExtremeDiscriminants>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_uminus_uone(
    __self: &'static ::enums_golden::repr_c::ReprCWithExtremeDiscriminants,
) -> bool {
    unsafe { ::enums_golden::repr_c::ReprCWithExtremeDiscriminants::is_minus_one(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_uminus_utwo(
    __self: &'static ::enums_golden::repr_c::ReprCWithExtremeDiscriminants,
) -> bool {
    unsafe { ::enums_golden::repr_c::ReprCWithExtremeDiscriminants::is_minus_two(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_umin_ui32(
    __self: &'static ::enums_golden::repr_c::ReprCWithExtremeDiscriminants,
) -> bool {
    unsafe { ::enums_golden::repr_c::ReprCWithExtremeDiscriminants::is_min_i32(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_umax_ui32(
    __self: &'static ::enums_golden::repr_c::ReprCWithExtremeDiscriminants,
) -> bool {
    unsafe { ::enums_golden::repr_c::ReprCWithExtremeDiscriminants::is_max_i32(__self) }
}
const _: () =
    assert!(::std::mem::size_of::<::enums_golden::repr_c::ReprCWithSingleNoPayloadVariant>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::enums_golden::repr_c::ReprCWithSingleNoPayloadVariant>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_usingle_uvariant(
    __self: &'static ::enums_golden::repr_c::ReprCWithSingleNoPayloadVariant,
) -> bool {
    unsafe { ::enums_golden::repr_c::ReprCWithSingleNoPayloadVariant::is_single_variant(__self) }
}
const _: () = assert!(
    ::std::mem::size_of::<::enums_golden::repr_c_clone_active_variant::CloneActiveVariant>() == 8
);
const _: () = assert!(
    ::std::mem::align_of::<::enums_golden::repr_c_clone_active_variant::CloneActiveVariant>() == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value=<::enums_golden::repr_c_clone_active_variant::CloneActiveVariant as::core::default::Default>::default();
        (__ret_ptr as *mut ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_A(__param_0: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant::A(__param_0);
        (__ret_ptr as *mut ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_B(__param_0: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant::B(__param_0);
        (__ret_ptr as *mut ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_C(__param_0: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant::C(__param_0);
        (__ret_ptr as *mut ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=<::enums_golden::repr_c_clone_active_variant::CloneActiveVariant as::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant,
    source: &'static ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant,
) -> () {
    unsafe {
        <::enums_golden::repr_c_clone_active_variant::CloneActiveVariant as::core::clone::Clone>::clone_from(__self,source)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_ua(
    e: &'static ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant,
) -> bool {
    unsafe { ::enums_golden::repr_c_clone_active_variant::is_a(e) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_ub(
    e: &'static ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant,
) -> bool {
    unsafe { ::enums_golden::repr_c_clone_active_variant::is_b(e) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_uc(
    e: &'static ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant,
) -> bool {
    unsafe { ::enums_golden::repr_c_clone_active_variant::is_c(e) }
}
const _: () =
    assert!(::std::mem::size_of::<::enums_golden::repr_c_clone_counter::CloneCount>() == 16);
const _: () =
    assert!(::std::mem::align_of::<::enums_golden::repr_c_clone_counter::CloneCount>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::enums_golden::repr_c_clone_counter::CloneCount as ::core::default::Default>::default(
            );
        (__ret_ptr as *mut ::enums_golden::repr_c_clone_counter::CloneCount)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::enums_golden::repr_c_clone_counter::CloneCount,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::enums_golden::repr_c_clone_counter::CloneCount as ::core::clone::Clone>::clone(
                __self,
            );
        (__ret_ptr as *mut ::enums_golden::repr_c_clone_counter::CloneCount)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::enums_golden::repr_c_clone_counter::CloneCount,
    source: &'static ::enums_golden::repr_c_clone_counter::CloneCount,
) -> () {
    unsafe {
        <::enums_golden::repr_c_clone_counter::CloneCount as ::core::clone::Clone>::clone_from(
            __self, source,
        )
    }
}
const _: () = assert!(::std::mem::size_of::<::enums_golden::repr_c_drop::DropMe>() == 16);
const _: () = assert!(::std::mem::align_of::<::enums_golden::repr_c_drop::DropMe>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::enums_golden::repr_c_drop::DropMe as ::core::default::Default>::default();
        (__ret_ptr as *mut ::enums_golden::repr_c_drop::DropMe).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_A(__param_0: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::enums_golden::repr_c_drop::DropMe::A(__param_0);
        (__ret_ptr as *mut ::enums_golden::repr_c_drop::DropMe).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_B(__param_0: i64, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::enums_golden::repr_c_drop::DropMe::B(__param_0);
        (__ret_ptr as *mut ::enums_golden::repr_c_drop::DropMe).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::enums_golden::repr_c_drop::DropMe>,
) {
    unsafe { __self.assume_init_drop() };
}
const _: () =
    assert!(::std::mem::size_of::<::enums_golden::repr_int::IntReprEnumWithNoPayload>() == 4);
const _: () =
    assert!(::std::mem::align_of::<::enums_golden::repr_int::IntReprEnumWithNoPayload>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_uno_upayload1(
    __self: &'static ::enums_golden::repr_int::IntReprEnumWithNoPayload,
) -> bool {
    unsafe { ::enums_golden::repr_int::IntReprEnumWithNoPayload::is_no_payload1(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_uno_upayload2(
    __self: &'static ::enums_golden::repr_int::IntReprEnumWithNoPayload,
) -> bool {
    unsafe { ::enums_golden::repr_int::IntReprEnumWithNoPayload::is_no_payload2(__self) }
}
const _: () = assert!(
    ::std::mem::size_of::<::enums_golden::repr_int::IntReprWithSingleNoPayloadVariant>() == 4
);
const _: () = assert!(
    ::std::mem::align_of::<::enums_golden::repr_int::IntReprWithSingleNoPayloadVariant>() == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_usingle_uvariant(
    __self: &'static ::enums_golden::repr_int::IntReprWithSingleNoPayloadVariant,
) -> bool {
    unsafe {
        ::enums_golden::repr_int::IntReprWithSingleNoPayloadVariant::is_single_variant(__self)
    }
}
const _: () = assert!(::std::mem::size_of::<::enums_golden::repr_int::NegReprIntEnum>() == 1);
const _: () = assert!(::std::mem::align_of::<::enums_golden::repr_int::NegReprIntEnum>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_uminus_uone(
    __self: &'static ::enums_golden::repr_int::NegReprIntEnum,
) -> bool {
    unsafe { ::enums_golden::repr_int::NegReprIntEnum::is_minus_one(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_uminus_utwo(
    __self: &'static ::enums_golden::repr_int::NegReprIntEnum,
) -> bool {
    unsafe { ::enums_golden::repr_int::NegReprIntEnum::is_minus_two(__self) }
}
const _: () = assert!(::std::mem::size_of::<::enums_golden::repr_rust::RustReprEnum>() == 12);
const _: () = assert!(::std::mem::align_of::<::enums_golden::repr_rust::RustReprEnum>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_TuplePayloadVariant(
    __param_0: i32,
    __param_1: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::enums_golden::repr_rust::RustReprEnum::TuplePayloadVariant(__param_0, __param_1);
        (__ret_ptr as *mut ::enums_golden::repr_rust::RustReprEnum).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uvariant_unumber(
    __self: &'static ::enums_golden::repr_rust::RustReprEnum,
) -> i32 {
    unsafe { ::enums_golden::repr_rust::RustReprEnum::get_variant_number(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_utuple_upayload_uvariant(
    __self: &'static ::enums_golden::repr_rust::RustReprEnum,
) -> bool {
    unsafe { ::enums_golden::repr_rust::RustReprEnum::is_tuple_payload_variant(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ufirst_uitem_ufrom_utuple_upayload(
    __self: &'static ::enums_golden::repr_rust::RustReprEnum,
) -> i32 {
    unsafe { ::enums_golden::repr_rust::RustReprEnum::get_first_item_from_tuple_payload(__self) }
}
const _: () = assert!(
    ::std::mem::size_of::<
        ::enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods,
    >() == 8
);
const _: () = assert!(
    ::std::mem::align_of::<
        ::enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods,
    >() == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_MakeNoPayloadVariant(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value=::enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods::MakeNoPayloadVariant();
        (__ret_ptr
            as *mut ::enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_MakeTuplePayloadVariant(
    i: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=::enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods::MakeTuplePayloadVariant(i);
        (__ret_ptr
            as *mut ::enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_MakeStructPayloadVariant(
    x: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=::enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods::MakeStructPayloadVariant(x);
        (__ret_ptr
            as *mut ::enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods)
            .write(__rs_return_value);
    }
}
const _: () = assert!(
    ::std::mem::size_of::<::enums_golden::repr_rust::RustReprWithSingleTuplePayloadVariant>() == 4
);
const _: () = assert!(
    ::std::mem::align_of::<::enums_golden::repr_rust::RustReprWithSingleTuplePayloadVariant>() == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_SingleVariant(
    __param_0: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            ::enums_golden::repr_rust::RustReprWithSingleTuplePayloadVariant::SingleVariant(
                __param_0,
            );
        (__ret_ptr as *mut ::enums_golden::repr_rust::RustReprWithSingleTuplePayloadVariant)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_usingle_uitem_ufrom_utuple_upayload(
    __self: &'static ::enums_golden::repr_rust::RustReprWithSingleTuplePayloadVariant,
) -> i32 {
    unsafe {
        ::enums_golden::repr_rust::RustReprWithSingleTuplePayloadVariant::get_single_item_from_tuple_payload(__self)
    }
}

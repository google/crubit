// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rs_ops_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::rs_ops_golden::MyInt>() == 4);
const _: () = assert!(::std::mem::align_of::<::rs_ops_golden::MyInt>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(value: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::rs_ops_golden::MyInt::new(value);
        (__ret_ptr as *mut ::rs_ops_golden::MyInt).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static ::rs_ops_golden::MyInt,
    other: &'static ::rs_ops_golden::MyInt,
) -> bool {
    unsafe {
        <::rs_ops_golden::MyInt as ::core::cmp::PartialEq<::rs_ops_golden::MyInt>>::eq(
            __self, other,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Add_uadd_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    rhs: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let rhs = rhs.assume_init_read();
        let __rs_return_value =
            <::rs_ops_golden::MyInt as ::core::ops::Add<::rs_ops_golden::MyInt>>::add(__self, rhs);
        (__ret_ptr as *mut ::rs_ops_golden::MyInt).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_AddAssign_uadd_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::rs_ops_golden::MyInt,
    rhs: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
) -> () {
    unsafe {
        let rhs = rhs.assume_init_read();
        <::rs_ops_golden::MyInt as ::core::ops::AddAssign<::rs_ops_golden::MyInt>>::add_assign(
            __self, rhs,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_BitAnd_ubitand_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    rhs: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let rhs = rhs.assume_init_read();
        let __rs_return_value = <::rs_ops_golden::MyInt as ::core::ops::BitAnd<
            ::rs_ops_golden::MyInt,
        >>::bitand(__self, rhs);
        (__ret_ptr as *mut ::rs_ops_golden::MyInt).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_BitAndAssign_ubitand_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::rs_ops_golden::MyInt,
    rhs: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
) -> () {
    unsafe {
        let rhs = rhs.assume_init_read();
        <::rs_ops_golden::MyInt as ::core::ops::BitAndAssign<::rs_ops_golden::MyInt>>::bitand_assign(
            __self, rhs,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_BitOr_ubitor_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    rhs: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let rhs = rhs.assume_init_read();
        let __rs_return_value = <::rs_ops_golden::MyInt as ::core::ops::BitOr<
            ::rs_ops_golden::MyInt,
        >>::bitor(__self, rhs);
        (__ret_ptr as *mut ::rs_ops_golden::MyInt).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_BitOrAssign_ubitor_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::rs_ops_golden::MyInt,
    rhs: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
) -> () {
    unsafe {
        let rhs = rhs.assume_init_read();
        <::rs_ops_golden::MyInt as ::core::ops::BitOrAssign<::rs_ops_golden::MyInt>>::bitor_assign(
            __self, rhs,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_BitXor_ubitxor_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    rhs: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let rhs = rhs.assume_init_read();
        let __rs_return_value = <::rs_ops_golden::MyInt as ::core::ops::BitXor<
            ::rs_ops_golden::MyInt,
        >>::bitxor(__self, rhs);
        (__ret_ptr as *mut ::rs_ops_golden::MyInt).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_BitXorAssign_ubitxor_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::rs_ops_golden::MyInt,
    rhs: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
) -> () {
    unsafe {
        let rhs = rhs.assume_init_read();
        <::rs_ops_golden::MyInt as ::core::ops::BitXorAssign<::rs_ops_golden::MyInt>>::bitxor_assign(
            __self, rhs,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Div_udiv_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    rhs: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let rhs = rhs.assume_init_read();
        let __rs_return_value =
            <::rs_ops_golden::MyInt as ::core::ops::Div<::rs_ops_golden::MyInt>>::div(__self, rhs);
        (__ret_ptr as *mut ::rs_ops_golden::MyInt).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_DivAssign_udiv_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::rs_ops_golden::MyInt,
    rhs: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
) -> () {
    unsafe {
        let rhs = rhs.assume_init_read();
        <::rs_ops_golden::MyInt as ::core::ops::DivAssign<::rs_ops_golden::MyInt>>::div_assign(
            __self, rhs,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Mul_umul_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    rhs: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let rhs = rhs.assume_init_read();
        let __rs_return_value =
            <::rs_ops_golden::MyInt as ::core::ops::Mul<::rs_ops_golden::MyInt>>::mul(__self, rhs);
        (__ret_ptr as *mut ::rs_ops_golden::MyInt).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_MulAssign_umul_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::rs_ops_golden::MyInt,
    rhs: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
) -> () {
    unsafe {
        let rhs = rhs.assume_init_read();
        <::rs_ops_golden::MyInt as ::core::ops::MulAssign<::rs_ops_golden::MyInt>>::mul_assign(
            __self, rhs,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Neg_uneg_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::rs_ops_golden::MyInt as ::core::ops::Neg>::neg(__self);
        (__ret_ptr as *mut ::rs_ops_golden::MyInt).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Not_unot_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::rs_ops_golden::MyInt as ::core::ops::Not>::not(__self);
        (__ret_ptr as *mut ::rs_ops_golden::MyInt).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Rem_urem_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    rhs: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let rhs = rhs.assume_init_read();
        let __rs_return_value =
            <::rs_ops_golden::MyInt as ::core::ops::Rem<::rs_ops_golden::MyInt>>::rem(__self, rhs);
        (__ret_ptr as *mut ::rs_ops_golden::MyInt).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_RemAssign_urem_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::rs_ops_golden::MyInt,
    rhs: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
) -> () {
    unsafe {
        let rhs = rhs.assume_init_read();
        <::rs_ops_golden::MyInt as ::core::ops::RemAssign<::rs_ops_golden::MyInt>>::rem_assign(
            __self, rhs,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Shl_ushl_urs_uops_ugolden_x0000003a_x0000003aMyInt_ui32(
    __self: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    rhs: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::rs_ops_golden::MyInt as ::core::ops::Shl<i32>>::shl(__self, rhs);
        (__ret_ptr as *mut ::rs_ops_golden::MyInt).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_ShlAssign_ushl_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_ui32(
    __self: &'static mut ::rs_ops_golden::MyInt,
    rhs: i32,
) -> () {
    unsafe { <::rs_ops_golden::MyInt as ::core::ops::ShlAssign<i32>>::shl_assign(__self, rhs) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Shr_ushr_urs_uops_ugolden_x0000003a_x0000003aMyInt_ui32(
    __self: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    rhs: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::rs_ops_golden::MyInt as ::core::ops::Shr<i32>>::shr(__self, rhs);
        (__ret_ptr as *mut ::rs_ops_golden::MyInt).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_ShrAssign_ushr_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_ui32(
    __self: &'static mut ::rs_ops_golden::MyInt,
    rhs: i32,
) -> () {
    unsafe { <::rs_ops_golden::MyInt as ::core::ops::ShrAssign<i32>>::shr_assign(__self, rhs) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Sub_usub_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    rhs: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let rhs = rhs.assume_init_read();
        let __rs_return_value =
            <::rs_ops_golden::MyInt as ::core::ops::Sub<::rs_ops_golden::MyInt>>::sub(__self, rhs);
        (__ret_ptr as *mut ::rs_ops_golden::MyInt).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_SubAssign_usub_uassign_urs_uops_ugolden_x0000003a_x0000003aMyInt_urs_uops_ugolden_x0000003a_x0000003aMyInt(
    __self: &'static mut ::rs_ops_golden::MyInt,
    rhs: &'static mut ::core::mem::MaybeUninit<::rs_ops_golden::MyInt>,
) -> () {
    unsafe {
        let rhs = rhs.assume_init_read();
        <::rs_ops_golden::MyInt as ::core::ops::SubAssign<::rs_ops_golden::MyInt>>::sub_assign(
            __self, rhs,
        )
    }
}
const _: () = assert!(::core::mem::offset_of!(::rs_ops_golden::MyInt, value) == 0);

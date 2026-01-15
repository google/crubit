// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// traits_golden
// Features: assume_lifetimes, custom_ffi_types, experimental, non_unpin_ctor, std_unique_ptr, std_vector, supported, wrapper

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () = assert!(::std::mem::size_of::<::traits_golden::Foo>() == 8);
const _: () = assert!(::std::mem::align_of::<::traits_golden::Foo>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <::traits_golden::Foo as ::core::default::Default>::default();
        (__ret_ptr as *mut ::traits_golden::Foo).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(x: i32, y: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::traits_golden::Foo::new(x, y);
        (__ret_ptr as *mut ::traits_golden::Foo).write(__rs_return_value);
    }
}
const _: () = assert!(::std::mem::size_of::<::traits_golden::LifetimeStruct>() == 8);
const _: () = assert!(::std::mem::align_of::<::traits_golden::LifetimeStruct>() == 8);
const _: () = assert!(::std::mem::size_of::<::traits_golden::MyStruct>() == 4);
const _: () = assert!(::std::mem::align_of::<::traits_golden::MyStruct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <::traits_golden::MyStruct as ::core::default::Default>::default();
        (__ret_ptr as *mut ::traits_golden::MyStruct).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(x: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::traits_golden::MyStruct::new(x);
        (__ret_ptr as *mut ::traits_golden::MyStruct).write(__rs_return_value);
    }
}
const _: () = assert!(::std::mem::size_of::<::traits_golden::MyStruct2>() == 4);
const _: () = assert!(::std::mem::align_of::<::traits_golden::MyStruct2>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <::traits_golden::MyStruct2 as ::core::default::Default>::default();
        (__ret_ptr as *mut ::traits_golden::MyStruct2).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_MyTrait_udo_usomething(
    __self: &'static ::traits_golden::MyStruct,
) -> i32 {
    unsafe { <::traits_golden::MyStruct as ::traits_golden::MyTrait>::do_something(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_MyTrait_uconsume_uself(
    __self: &'static mut ::core::mem::MaybeUninit<::traits_golden::MyStruct>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        <::traits_golden::MyStruct as ::traits_golden::MyTrait>::consume_self(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_MyTrait_ureturn_uself(
    __self: &'static ::traits_golden::MyStruct,
) -> &'static ::traits_golden::MyStruct {
    unsafe { <::traits_golden::MyStruct as ::traits_golden::MyTrait>::return_self(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_MyTrait_uno_uself() -> i32 {
    unsafe { <::traits_golden::MyStruct as ::traits_golden::MyTrait>::no_self() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_MyTrait_utake_uand_ureturn_uother_utypes(
    __self: &'static ::traits_golden::MyStruct,
    x: &'static mut ::core::mem::MaybeUninit<::traits_golden::Foo>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let x = x.assume_init_read();
        let __rs_return_value =
            <::traits_golden::MyStruct as ::traits_golden::MyTrait>::take_and_return_other_types(
                __self, x,
            );
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0 as *mut i32).write(__rs_return_value_0);
        (__ret_ptr_1 as *mut i32).write(__rs_return_value_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_MyTrait_udo_usomething(
    __self: &'static ::traits_golden::MyStruct2,
) -> i32 {
    unsafe { <::traits_golden::MyStruct2 as ::traits_golden::MyTrait>::do_something(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_MyTrait_uconsume_uself(
    __self: &'static mut ::core::mem::MaybeUninit<::traits_golden::MyStruct2>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        <::traits_golden::MyStruct2 as ::traits_golden::MyTrait>::consume_self(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_MyTrait_ureturn_uself(
    __self: &'static ::traits_golden::MyStruct2,
) -> &'static ::traits_golden::MyStruct2 {
    unsafe { <::traits_golden::MyStruct2 as ::traits_golden::MyTrait>::return_self(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_MyTrait_uno_uself() -> i32 {
    unsafe { <::traits_golden::MyStruct2 as ::traits_golden::MyTrait>::no_self() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_MyTrait_utake_uand_ureturn_uother_utypes(
    __self: &'static ::traits_golden::MyStruct2,
    x: &'static mut ::core::mem::MaybeUninit<::traits_golden::Foo>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let x = x.assume_init_read();
        let __rs_return_value =
            <::traits_golden::MyStruct2 as ::traits_golden::MyTrait>::take_and_return_other_types(
                __self, x,
            );
        let (__rs_return_value_0, __rs_return_value_1) = __rs_return_value;
        let [__ret_ptr_0, __ret_ptr_1] = *(__ret_ptr as *mut [*mut core::ffi::c_void; 2usize]);
        (__ret_ptr_0 as *mut i32).write(__rs_return_value_0);
        (__ret_ptr_1 as *mut i32).write(__rs_return_value_1);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_DifferentTraitSameName_udo_usomething(
    __self: &'static ::traits_golden::MyStruct,
) -> i32 {
    unsafe {
        <::traits_golden::MyStruct as ::traits_golden::DifferentTraitSameName>::do_something(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_LifetimeTrait_utrait_udo_usomething(
    __self: &'static ::traits_golden::LifetimeStruct<'static>,
) -> &'static i32 {
    unsafe {
        <::traits_golden::LifetimeStruct as ::traits_golden::LifetimeTrait>::trait_do_something(
            __self,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_LifetimeTrait_ufunction_udo_usomething(
    __self: &'static ::traits_golden::LifetimeStruct<'static>,
) -> &'static i32 {
    unsafe {
        <::traits_golden::LifetimeStruct as ::traits_golden::LifetimeTrait>::function_do_something(
            __self,
        )
    }
}

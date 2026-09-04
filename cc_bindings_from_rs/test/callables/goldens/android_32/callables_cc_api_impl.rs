// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// callables_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::callables_golden::CallbackHolder>() == 8);
const _: () = assert!(::std::mem::align_of::<::callables_golden::CallbackHolder>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_ucallables_ugolden_x0000003a_x0000003aCallbackHolder(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::callables_golden::CallbackHolder as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_ucallables_ugolden_x0000003a_x0000003aCallbackHolder(
    __self: *mut ::callables_golden::CallbackHolder,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::callables_golden::CallbackHolder::new();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_set_ucallback(
    __self: &'static mut ::callables_golden::CallbackHolder,
    f: ::bridge_rust::FnPayload,
) -> () {
    unsafe {
        ::callables_golden::CallbackHolder::set_callback(
            __self,
            ::alloc::boxed::Box::new(move || {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void) -> () =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe {
                    __invoker(f.data());
                }
            }),
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call(
    __self: &'static ::callables_golden::CallbackHolder,
) -> () {
    unsafe { ::callables_golden::CallbackHolder::call(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_drop_ucallback(
    __self: &'static mut ::callables_golden::CallbackHolder,
) -> () {
    unsafe { ::callables_golden::CallbackHolder::drop_callback(__self) }
}
const _: () = assert!(::std::mem::size_of::<::callables_golden::CppMovableDrop>() == 4);
const _: () = assert!(::std::mem::align_of::<::callables_golden::CppMovableDrop>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_ucallables_ugolden_x0000003a_x0000003aCppMovableDrop(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::callables_golden::CppMovableDrop as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_ucallables_ugolden_x0000003a_x0000003aCppMovableDrop(
    __self: *mut ::callables_golden::CppMovableDrop,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
const _: () = assert!(::core::mem::offset_of!(::callables_golden::CppMovableDrop, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::callables_golden::NonCppMovable>() == 4);
const _: () = assert!(::std::mem::align_of::<::callables_golden::NonCppMovable>() == 4);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_ucallables_ugolden_x0000003a_x0000003aNonCppMovable(
    __self: *mut ::callables_golden::NonCppMovable,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
const _: () = assert!(::core::mem::offset_of!(::callables_golden::NonCppMovable, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::callables_golden::Point>() == 8);
const _: () = assert!(::std::mem::align_of::<::callables_golden::Point>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_PartialEq_ueq_ucallables_ugolden_x0000003a_x0000003aPoint_ucallables_ugolden_x0000003a_x0000003aPoint(
    __self: &'static ::callables_golden::Point,
    other: &'static ::callables_golden::Point,
) -> bool {
    unsafe {
        <::callables_golden::Point as ::core::cmp::PartialEq<::callables_golden::Point>>::eq(
            __self, other,
        )
    }
}
const _: () = assert!(::core::mem::offset_of!(::callables_golden::Point, x) == 0);
const _: () = assert!(::core::mem::offset_of!(::callables_golden::Point, y) == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uand_ureturn_unon_umovable(
    f: ::bridge_rust::FnRefPayload,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::callables_golden::call_and_return_non_movable(move || {
            let __invoker: unsafe extern "C" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
            ) -> () = unsafe { ::core::mem::transmute(f.invoker()) };
            let mut __ret_storage =
                ::core::mem::MaybeUninit::<::callables_golden::NonCppMovable>::uninit();
            unsafe {
                __invoker(f.data(), __ret_storage.as_mut_ptr() as *mut _);
                __ret_storage.assume_init()
            }
        });
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uand_ureturn_unon_umovable_ubox_ufn(
    f: ::bridge_rust::FnPayload,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::callables_golden::call_and_return_non_movable_box_fn(
            ::alloc::boxed::Box::new(move || {
                let __invoker: unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> () = unsafe { ::core::mem::transmute(f.invoker()) };
                let mut __ret_storage =
                    ::core::mem::MaybeUninit::<::callables_golden::NonCppMovable>::uninit();
                unsafe {
                    __invoker(f.data(), __ret_storage.as_mut_ptr() as *mut _);
                    __ret_storage.assume_init()
                }
            }),
        );
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uand_ureturn_unon_umovable_uboxed(
    f: ::bridge_rust::FnPayload,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::callables_golden::call_and_return_non_movable_boxed(
            ::alloc::boxed::Box::new(move || {
                let __invoker: unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> () = unsafe { ::core::mem::transmute(f.invoker()) };
                let mut __ret_storage =
                    ::core::mem::MaybeUninit::<::callables_golden::NonCppMovable>::uninit();
                unsafe {
                    __invoker(f.data(), __ret_storage.as_mut_ptr() as *mut _);
                    __ret_storage.assume_init()
                }
            }),
        );
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_ubox_udyn_ufn(f: ::bridge_rust::FnPayload, x: i32) -> i32 {
    unsafe {
        ::callables_golden::call_box_dyn_fn(
            ::alloc::boxed::Box::new(move |__arg_0: i32| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> i32 =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe { __invoker(f.data(), __arg_0) }
            }),
            x,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_ubox_udyn_ufn_umut(
    f: ::bridge_rust::FnPayload,
    x: i32,
) -> i32 {
    unsafe {
        ::callables_golden::call_box_dyn_fn_mut(
            ::alloc::boxed::Box::new(move |__arg_0: i32| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> i32 =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe { __invoker(f.data(), __arg_0) }
            }),
            x,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_ubox_udyn_ufn_uonce(
    f: ::bridge_rust::FnPayload,
    x: i32,
) -> i32 {
    unsafe {
        ::callables_golden::call_box_dyn_fn_once(
            ::alloc::boxed::Box::new(move |__arg_0: i32| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> i32 =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe { __invoker(f.data(), __arg_0) }
            }),
            x,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_udyn_ufn(f: ::bridge_rust::FnRefPayload, x: i32) -> i32 {
    unsafe {
        ::callables_golden::call_dyn_fn(
            &move |__arg_0: i32| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> i32 =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe { __invoker(f.data(), __arg_0) }
            },
            x,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_udyn_ufn_umut(
    f: ::bridge_rust::FnRefPayload,
    x: i32,
) -> i32 {
    unsafe {
        ::callables_golden::call_dyn_fn_mut(
            &mut move |__arg_0: i32| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> i32 =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe { __invoker(f.data(), __arg_0) }
            },
            x,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uimpl_ufn(f: ::bridge_rust::FnRefPayload, x: i32) -> i32 {
    unsafe {
        ::callables_golden::call_impl_fn(
            move |__arg_0: i32| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> i32 =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe { __invoker(f.data(), __arg_0) }
            },
            x,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uimpl_ufn_umut(
    f: ::bridge_rust::FnRefPayload,
    x: i32,
) -> i32 {
    unsafe {
        ::callables_golden::call_impl_fn_mut(
            move |__arg_0: i32| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> i32 =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe { __invoker(f.data(), __arg_0) }
            },
            x,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uimpl_ufn_uonce(
    f: ::bridge_rust::FnRefPayload,
    x: i32,
) -> i32 {
    unsafe {
        ::callables_golden::call_impl_fn_once(
            move |__arg_0: i32| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> i32 =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe { __invoker(f.data(), __arg_0) }
            },
            x,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uimpl_ufn_uonce_ustatic(
    f: ::bridge_rust::FnPayload,
    x: i32,
) -> i32 {
    unsafe {
        ::callables_golden::call_impl_fn_once_static(
            move |__arg_0: i32| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> i32 =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe { __invoker(f.data(), __arg_0) }
            },
            x,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uimpl_uwith_utuple_uoption(
    f: ::bridge_rust::FnPayload,
) -> i32 {
    unsafe {
        ::callables_golden::call_impl_with_tuple_option(
            move |__arg_0: (i32, ::core::option::Option<i32>)| {
                let mut __arg_0 = ::core::mem::ManuallyDrop::new(__arg_0);
                let __invoker: unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut (i32, ::core::option::Option<i32>),
                    *mut core::ffi::c_void,
                ) -> () = unsafe { ::core::mem::transmute(f.invoker()) };
                let mut __ret_storage =
                    ::core::mem::MaybeUninit::<(i32, ::core::option::Option<i32>)>::uninit();
                unsafe {
                    __invoker(
                        f.data(),
                        &mut *__arg_0 as *mut _,
                        __ret_storage.as_mut_ptr() as *mut _,
                    );
                    __ret_storage.assume_init()
                }
            },
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uint_uto_upoint(
    f: ::bridge_rust::FnRefPayload,
    x: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::callables_golden::call_int_to_point(
            move |__arg_0: i32| {
                let __invoker: unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    i32,
                    *mut core::ffi::c_void,
                ) -> () = unsafe { ::core::mem::transmute(f.invoker()) };
                let mut __ret_storage =
                    ::core::mem::MaybeUninit::<::callables_golden::Point>::uninit();
                unsafe {
                    __invoker(f.data(), __arg_0, __ret_storage.as_mut_ptr() as *mut _);
                    __ret_storage.assume_init()
                }
            },
            x,
        );
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_upoint_umut(
    f: ::bridge_rust::FnRefPayload,
    pt: *mut ::callables_golden::Point,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let pt = pt.read();
        let __rs_return_value = ::callables_golden::call_point_mut(
            move |__arg_0: ::callables_golden::Point| {
                let mut __arg_0 = ::core::mem::ManuallyDrop::new(__arg_0);
                let __invoker: unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut ::callables_golden::Point,
                    *mut core::ffi::c_void,
                ) -> () = unsafe { ::core::mem::transmute(f.invoker()) };
                let mut __ret_storage =
                    ::core::mem::MaybeUninit::<::callables_golden::Point>::uninit();
                unsafe {
                    __invoker(
                        f.data(),
                        &mut *__arg_0 as *mut _,
                        __ret_storage.as_mut_ptr() as *mut _,
                    );
                    __ret_storage.assume_init()
                }
            },
            pt,
        );
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_upoint_uonce_ustatic(
    f: ::bridge_rust::FnPayload,
    pt: *mut ::callables_golden::Point,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let pt = pt.read();
        let __rs_return_value = ::callables_golden::call_point_once_static(
            move |__arg_0: ::callables_golden::Point| {
                let mut __arg_0 = ::core::mem::ManuallyDrop::new(__arg_0);
                let __invoker: unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut ::callables_golden::Point,
                    *mut core::ffi::c_void,
                ) -> () = unsafe { ::core::mem::transmute(f.invoker()) };
                let mut __ret_storage =
                    ::core::mem::MaybeUninit::<::callables_golden::Point>::uninit();
                unsafe {
                    __invoker(
                        f.data(),
                        &mut *__arg_0 as *mut _,
                        __ret_storage.as_mut_ptr() as *mut _,
                    );
                    __ret_storage.assume_init()
                }
            },
            pt,
        );
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_upoint_uto_uint(
    f: ::bridge_rust::FnRefPayload,
    pt: *mut ::callables_golden::Point,
) -> i32 {
    unsafe {
        let pt = pt.read();
        ::callables_golden::call_point_to_int(
            move |__arg_0: ::callables_golden::Point| {
                let mut __arg_0 = ::core::mem::ManuallyDrop::new(__arg_0);
                let __invoker: unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut ::callables_golden::Point,
                ) -> i32 = unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe { __invoker(f.data(), &mut *__arg_0 as *mut _) }
            },
            pt,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_upoint_uvoid(
    f: ::bridge_rust::FnRefPayload,
    pt: *mut ::callables_golden::Point,
) -> () {
    unsafe {
        let pt = pt.read();
        ::callables_golden::call_point_void(
            move |__arg_0: ::callables_golden::Point| {
                let mut __arg_0 = ::core::mem::ManuallyDrop::new(__arg_0);
                let __invoker: unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut ::callables_golden::Point,
                ) -> () = unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe {
                    __invoker(f.data(), &mut *__arg_0 as *mut _);
                }
            },
            pt,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_utwo_uargs(
    f: ::bridge_rust::FnRefPayload,
    a: i32,
    b: i32,
) -> i32 {
    unsafe {
        ::callables_golden::call_two_args(
            move |__arg_0: i32, __arg_1: i32| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, i32, i32) -> i32 =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe { __invoker(f.data(), __arg_0, __arg_1) }
            },
            a,
            b,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_utwo_upoints(
    f: ::bridge_rust::FnRefPayload,
    a: *mut ::callables_golden::Point,
    b: *mut ::callables_golden::Point,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let a = a.read();
        let b = b.read();
        let __rs_return_value = ::callables_golden::call_two_points(
            move |__arg_0: ::callables_golden::Point, __arg_1: ::callables_golden::Point| {
                let mut __arg_0 = ::core::mem::ManuallyDrop::new(__arg_0);
                let mut __arg_1 = ::core::mem::ManuallyDrop::new(__arg_1);
                let __invoker: unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut ::callables_golden::Point,
                    *mut ::callables_golden::Point,
                    *mut core::ffi::c_void,
                ) -> () = unsafe { ::core::mem::transmute(f.invoker()) };
                let mut __ret_storage =
                    ::core::mem::MaybeUninit::<::callables_golden::Point>::uninit();
                unsafe {
                    __invoker(
                        f.data(),
                        &mut *__arg_0 as *mut _,
                        &mut *__arg_1 as *mut _,
                        __ret_storage.as_mut_ptr() as *mut _,
                    );
                    __ret_storage.assume_init()
                }
            },
            a,
            b,
        );
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uvoid(f: ::bridge_rust::FnRefPayload, x: i32) -> () {
    unsafe {
        ::callables_golden::call_void(
            move |__arg_0: i32| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> () =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe {
                    __invoker(f.data(), __arg_0);
                }
            },
            x,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uvoid_umut(f: ::bridge_rust::FnRefPayload, x: i32) -> () {
    unsafe {
        ::callables_golden::call_void_mut(
            move |__arg_0: i32| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> () =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe {
                    __invoker(f.data(), __arg_0);
                }
            },
            x,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uvoid_uonce(f: ::bridge_rust::FnRefPayload, x: i32) -> () {
    unsafe {
        ::callables_golden::call_void_once(
            move |__arg_0: i32| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, i32) -> () =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe {
                    __invoker(f.data(), __arg_0);
                }
            },
            x,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uwith_uhrtb_ustr(
    f: ::bridge_rust::FnRefPayload,
    s: &'static str,
) -> i32 {
    unsafe {
        ::callables_golden::call_with_hrtb_str(
            move |__arg_0: &str| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, &str) -> i32 =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe { __invoker(f.data(), __arg_0) }
            },
            s,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uwith_uhrtb_ustr_uto_ustr(
    f: ::bridge_rust::FnRefPayload,
    s: &'static str,
) -> usize {
    unsafe {
        ::callables_golden::call_with_hrtb_str_to_str(
            move |__arg_0: &str| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, &str) -> &str =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe { __invoker(f.data(), __arg_0) }
            },
            s,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uwith_umovable_udrop(
    f: ::bridge_rust::FnRefPayload,
    x: i32,
) -> () {
    unsafe {
        ::callables_golden::call_with_movable_drop(
            move |__arg_0: ::callables_golden::CppMovableDrop| {
                let mut __arg_0 = ::core::mem::ManuallyDrop::new(__arg_0);
                let __invoker: unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut ::callables_golden::CppMovableDrop,
                ) -> () = unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe {
                    __invoker(f.data(), &mut *__arg_0 as *mut _);
                }
            },
            x,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uwith_unon_umovable_uref(
    f: ::bridge_rust::FnRefPayload,
    x: &'static ::callables_golden::NonCppMovable,
) -> i32 {
    unsafe {
        ::callables_golden::call_with_non_movable_ref(
            move |__arg_0: &::callables_golden::NonCppMovable| {
                let __invoker: unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    &::callables_golden::NonCppMovable,
                ) -> () = unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe {
                    __invoker(f.data(), __arg_0);
                }
            },
            x,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uwith_upoint(
    f: ::bridge_rust::FnRefPayload,
    pt: *mut ::callables_golden::Point,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let pt = pt.read();
        let __rs_return_value = ::callables_golden::call_with_point(
            move |__arg_0: ::callables_golden::Point| {
                let mut __arg_0 = ::core::mem::ManuallyDrop::new(__arg_0);
                let __invoker: unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut ::callables_golden::Point,
                    *mut core::ffi::c_void,
                ) -> () = unsafe { ::core::mem::transmute(f.invoker()) };
                let mut __ret_storage =
                    ::core::mem::MaybeUninit::<::callables_golden::Point>::uninit();
                unsafe {
                    __invoker(
                        f.data(),
                        &mut *__arg_0 as *mut _,
                        __ret_storage.as_mut_ptr() as *mut _,
                    );
                    __ret_storage.assume_init()
                }
            },
            pt,
        );
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uwith_ustr(
    f: ::bridge_rust::FnRefPayload,
    s: &'static str,
) -> i32 {
    unsafe {
        ::callables_golden::call_with_str(
            move |__arg_0: &str| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, &str) -> i32 =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe { __invoker(f.data(), __arg_0) }
            },
            s,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uwith_ustr_uto_ustr(
    f: ::bridge_rust::FnRefPayload,
    s: &'static str,
) -> &'static str {
    unsafe {
        ::callables_golden::call_with_str_to_str(
            move |__arg_0: &str| {
                let __invoker: unsafe extern "C" fn(*mut core::ffi::c_void, &str) -> &str =
                    unsafe { ::core::mem::transmute(f.invoker()) };
                unsafe { __invoker(f.data(), __arg_0) }
            },
            s,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_call_uwith_utuple_uoption(f: ::bridge_rust::FnPayload) -> i32 {
    unsafe {
        ::callables_golden::call_with_tuple_option(::alloc::boxed::Box::new(
            move |__arg_0: (i32, ::core::option::Option<i32>)| {
                let mut __arg_0 = ::core::mem::ManuallyDrop::new(__arg_0);
                let __invoker: unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    *mut (i32, ::core::option::Option<i32>),
                    *mut core::ffi::c_void,
                ) -> () = unsafe { ::core::mem::transmute(f.invoker()) };
                let mut __ret_storage =
                    ::core::mem::MaybeUninit::<(i32, ::core::option::Option<i32>)>::uninit();
                unsafe {
                    __invoker(
                        f.data(),
                        &mut *__arg_0 as *mut _,
                        __ret_storage.as_mut_ptr() as *mut _,
                    );
                    __ret_storage.assume_init()
                }
            },
        ))
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_u_x00000028i32_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x00000029(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <(i32, ::core::option::Option<i32>) as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!((i32, ::core::option::Option<i32>,), 0) == 0);
const _: () = assert!(::core::mem::offset_of!((i32, ::core::option::Option<i32>,), 1) == 4);

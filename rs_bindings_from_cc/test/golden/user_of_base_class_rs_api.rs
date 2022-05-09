// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ::std as rust_std;
use memoffset_unstable_const::offset_of;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// The same as Derived from inheritance.h, but in a different build target.
///
/// This tests inheritance across library boundaries.
///
/// TODO(b/216195042): Correctly namespace base classes in generated Rust code.
#[repr(C, align(8))]
pub struct Derived2 {
    __non_field_data: [rust_std::mem::MaybeUninit<u8>; 20],
    pub derived_1: u8,
}
forward_declare::unsafe_define!(forward_declare::symbol!("Derived2"), crate::Derived2);

impl !Unpin for Derived2 {}

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=15
// Error while generating bindings for item 'Derived2::Derived2':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=15
// Error while generating bindings for item 'Derived2::Derived2':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=15
// Error while generating bindings for item 'Derived2::Derived2':
// Parameter #0 is not supported: Unsupported type 'struct Derived2 &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=15
// Error while generating bindings for item 'Derived2::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=15
// Error while generating bindings for item 'Derived2::operator=':
// Parameter #0 is not supported: Unsupported type 'struct Derived2 &&': Unsupported type: && without lifetime

unsafe impl oops::Inherits<inheritance_cc::Base0> for Derived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::Base0 {
        detail::__crubit_dynamic_upcast__Derived2__to__Base0(derived)
    }
}
unsafe impl oops::Inherits<inheritance_cc::Base1> for Derived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::Base1 {
        (derived as *const _ as *const u8).offset(8) as *const inheritance_cc::Base1
    }
}
unsafe impl oops::Inherits<inheritance_cc::Base2> for Derived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::Base2 {
        (derived as *const _ as *const u8).offset(18) as *const inheritance_cc::Base2
    }
}

#[repr(C, align(8))]
pub struct VirtualDerived2 {
    __non_field_data: [rust_std::mem::MaybeUninit<u8>; 32],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("VirtualDerived2"),
    crate::VirtualDerived2
);

impl !Unpin for VirtualDerived2 {}

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=19
// Error while generating bindings for item 'VirtualDerived2::VirtualDerived2':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=19
// Error while generating bindings for item 'VirtualDerived2::VirtualDerived2':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=19
// Error while generating bindings for item 'VirtualDerived2::VirtualDerived2':
// Parameter #0 is not supported: Unsupported type 'class VirtualDerived2 &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=19
// Error while generating bindings for item 'VirtualDerived2::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/user_of_base_class.h;l=19
// Error while generating bindings for item 'VirtualDerived2::operator=':
// Parameter #0 is not supported: Unsupported type 'class VirtualDerived2 &&': Unsupported type: && without lifetime

unsafe impl oops::Inherits<inheritance_cc::VirtualBase1> for VirtualDerived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::VirtualBase1 {
        detail::__crubit_dynamic_upcast__VirtualDerived2__to__VirtualBase1(derived)
    }
}
unsafe impl oops::Inherits<inheritance_cc::Base1> for VirtualDerived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::Base1 {
        detail::__crubit_dynamic_upcast__VirtualDerived2__to__Base1(derived)
    }
}
unsafe impl oops::Inherits<inheritance_cc::VirtualBase2> for VirtualDerived2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const inheritance_cc::VirtualBase2 {
        detail::__crubit_dynamic_upcast__VirtualDerived2__to__VirtualBase2(derived)
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_BASE_CLASS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub fn __crubit_dynamic_upcast__Derived2__to__Base0(
            from: *const Derived2,
        ) -> *const inheritance_cc::Base0;
        pub fn __crubit_dynamic_upcast__VirtualDerived2__to__VirtualBase1(
            from: *const VirtualDerived2,
        ) -> *const inheritance_cc::VirtualBase1;
        pub fn __crubit_dynamic_upcast__VirtualDerived2__to__Base1(
            from: *const VirtualDerived2,
        ) -> *const inheritance_cc::Base1;
        pub fn __crubit_dynamic_upcast__VirtualDerived2__to__VirtualBase2(
            from: *const VirtualDerived2,
        ) -> *const inheritance_cc::VirtualBase2;
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::Derived2>() == 24usize);
const _: () = assert!(rust_std::mem::align_of::<crate::Derived2>() == 8usize);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Derived2: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Derived2: Drop);
};
const _: () = assert!(offset_of!(crate::Derived2, derived_1) * 8 == 160usize);

const _: () = assert!(rust_std::mem::size_of::<crate::VirtualDerived2>() == 32usize);
const _: () = assert!(rust_std::mem::align_of::<crate::VirtualDerived2>() == 8usize);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::VirtualDerived2: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::VirtualDerived2: Drop);
};

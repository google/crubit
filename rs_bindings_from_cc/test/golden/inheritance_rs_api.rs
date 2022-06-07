// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:inheritance_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

use ::std as rust_std;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Using classes to force these to be non-POD.
/// In the Itanium ABI, the tail padding of POD types cannot be reused by other
/// objects, even if the POD type is potentially-overlapping.
#[ctor::recursively_pinned]
#[repr(C)]
pub struct Base0 {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("Base0"), crate::Base0);

// rs_bindings_from_cc/test/golden/inheritance.h;l=13
// Error while generating bindings for item 'Base0::Base0':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=13
// Error while generating bindings for item 'Base0::Base0':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=13
// Error while generating bindings for item 'Base0::Base0':
// Parameter #0 is not supported: Unsupported type 'class Base0 &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/inheritance.h;l=13
// Error while generating bindings for item 'Base0::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=13
// Error while generating bindings for item 'Base0::operator=':
// Parameter #0 is not supported: Unsupported type 'class Base0 &&': Unsupported type: && without lifetime

#[ctor::recursively_pinned]
#[repr(C, align(8))]
pub struct Base1 {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b1_1_: [crate::rust_std::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b1_2_: [crate::rust_std::mem::MaybeUninit<u8>; 8],
}
forward_declare::unsafe_define!(forward_declare::symbol!("Base1"), crate::Base1);

// rs_bindings_from_cc/test/golden/inheritance.h;l=14
// Error while generating bindings for item 'Base1::Base1':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=14
// Error while generating bindings for item 'Base1::Base1':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=14
// Error while generating bindings for item 'Base1::Base1':
// Parameter #0 is not supported: Unsupported type 'class Base1 &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/inheritance.h;l=14
// Error while generating bindings for item 'Base1::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=14
// Error while generating bindings for item 'Base1::operator=':
// Parameter #0 is not supported: Unsupported type 'class Base1 &&': Unsupported type: && without lifetime

#[ctor::recursively_pinned]
#[repr(C, align(2))]
pub struct Base2 {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b2_1_: [crate::rust_std::mem::MaybeUninit<u8>; 2],
}
forward_declare::unsafe_define!(forward_declare::symbol!("Base2"), crate::Base2);

// rs_bindings_from_cc/test/golden/inheritance.h;l=19
// Error while generating bindings for item 'Base2::Base2':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=19
// Error while generating bindings for item 'Base2::Base2':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=19
// Error while generating bindings for item 'Base2::Base2':
// Parameter #0 is not supported: Unsupported type 'class Base2 &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/inheritance.h;l=19
// Error while generating bindings for item 'Base2::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=19
// Error while generating bindings for item 'Base2::operator=':
// Parameter #0 is not supported: Unsupported type 'class Base2 &&': Unsupported type: && without lifetime

#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct Derived {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 12],
    pub derived_1: u8,
}
forward_declare::unsafe_define!(forward_declare::symbol!("Derived"), crate::Derived);

// rs_bindings_from_cc/test/golden/inheritance.h;l=23
// Error while generating bindings for item 'Derived::Derived':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=23
// Error while generating bindings for item 'Derived::Derived':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=23
// Error while generating bindings for item 'Derived::Derived':
// Parameter #0 is not supported: Unsupported type 'struct Derived &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/inheritance.h;l=23
// Error while generating bindings for item 'Derived::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=23
// Error while generating bindings for item 'Derived::operator=':
// Parameter #0 is not supported: Unsupported type 'struct Derived &&': Unsupported type: && without lifetime

unsafe impl oops::Inherits<crate::Base0> for Derived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base0 {
        (derived as *const _ as *const u8).offset(0) as *const crate::Base0
    }
}
unsafe impl oops::Inherits<crate::Base1> for Derived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base1 {
        (derived as *const _ as *const u8).offset(0) as *const crate::Base1
    }
}
unsafe impl oops::Inherits<crate::Base2> for Derived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base2 {
        (derived as *const _ as *const u8).offset(10) as *const crate::Base2
    }
}

#[ctor::recursively_pinned]
#[repr(C, align(8))]
pub struct VirtualBase1 {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 24],
}
forward_declare::unsafe_define!(forward_declare::symbol!("VirtualBase1"), crate::VirtualBase1);

// rs_bindings_from_cc/test/golden/inheritance.h;l=27
// Error while generating bindings for item 'VirtualBase1::VirtualBase1':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=27
// Error while generating bindings for item 'VirtualBase1::VirtualBase1':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=27
// Error while generating bindings for item 'VirtualBase1::VirtualBase1':
// Parameter #0 is not supported: Unsupported type 'class VirtualBase1 &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/inheritance.h;l=27
// Error while generating bindings for item 'VirtualBase1::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=27
// Error while generating bindings for item 'VirtualBase1::operator=':
// Parameter #0 is not supported: Unsupported type 'class VirtualBase1 &&': Unsupported type: && without lifetime

unsafe impl oops::Inherits<crate::Base1> for VirtualBase1 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base1 {
        crate::detail::__crubit_dynamic_upcast__VirtualBase1__to__Base1(derived)
    }
}

#[ctor::recursively_pinned]
#[repr(C, align(8))]
pub struct VirtualBase2 {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 24],
}
forward_declare::unsafe_define!(forward_declare::symbol!("VirtualBase2"), crate::VirtualBase2);

// rs_bindings_from_cc/test/golden/inheritance.h;l=28
// Error while generating bindings for item 'VirtualBase2::VirtualBase2':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=28
// Error while generating bindings for item 'VirtualBase2::VirtualBase2':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=28
// Error while generating bindings for item 'VirtualBase2::VirtualBase2':
// Parameter #0 is not supported: Unsupported type 'class VirtualBase2 &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/inheritance.h;l=28
// Error while generating bindings for item 'VirtualBase2::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=28
// Error while generating bindings for item 'VirtualBase2::operator=':
// Parameter #0 is not supported: Unsupported type 'class VirtualBase2 &&': Unsupported type: && without lifetime

unsafe impl oops::Inherits<crate::Base1> for VirtualBase2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base1 {
        crate::detail::__crubit_dynamic_upcast__VirtualBase2__to__Base1(derived)
    }
}

#[ctor::recursively_pinned]
#[repr(C, align(8))]
pub struct VirtualDerived {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 32],
}
forward_declare::unsafe_define!(forward_declare::symbol!("VirtualDerived"), crate::VirtualDerived);

// rs_bindings_from_cc/test/golden/inheritance.h;l=29
// Error while generating bindings for item 'VirtualDerived::VirtualDerived':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=29
// Error while generating bindings for item 'VirtualDerived::VirtualDerived':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=29
// Error while generating bindings for item 'VirtualDerived::VirtualDerived':
// Parameter #0 is not supported: Unsupported type 'class VirtualDerived &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/inheritance.h;l=29
// Error while generating bindings for item 'VirtualDerived::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/inheritance.h;l=29
// Error while generating bindings for item 'VirtualDerived::operator=':
// Parameter #0 is not supported: Unsupported type 'class VirtualDerived &&': Unsupported type: && without lifetime

unsafe impl oops::Inherits<crate::VirtualBase1> for VirtualDerived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::VirtualBase1 {
        crate::detail::__crubit_dynamic_upcast__VirtualDerived__to__VirtualBase1(derived)
    }
}
unsafe impl oops::Inherits<crate::Base1> for VirtualDerived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base1 {
        crate::detail::__crubit_dynamic_upcast__VirtualDerived__to__Base1(derived)
    }
}
unsafe impl oops::Inherits<crate::VirtualBase2> for VirtualDerived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::VirtualBase2 {
        crate::detail::__crubit_dynamic_upcast__VirtualDerived__to__VirtualBase2(derived)
    }
}

// rs_bindings_from_cc/test/golden/inheritance.h;l=32
// Error while generating bindings for item 'MyAbstractClass':
// Abstract classes are not supported yet

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_INHERITANCE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub fn __crubit_dynamic_upcast__VirtualBase1__to__Base1(
            from: *const VirtualBase1,
        ) -> *const crate::Base1;
        pub fn __crubit_dynamic_upcast__VirtualBase2__to__Base1(
            from: *const VirtualBase2,
        ) -> *const crate::Base1;
        pub fn __crubit_dynamic_upcast__VirtualDerived__to__VirtualBase1(
            from: *const VirtualDerived,
        ) -> *const crate::VirtualBase1;
        pub fn __crubit_dynamic_upcast__VirtualDerived__to__Base1(
            from: *const VirtualDerived,
        ) -> *const crate::Base1;
        pub fn __crubit_dynamic_upcast__VirtualDerived__to__VirtualBase2(
            from: *const VirtualDerived,
        ) -> *const crate::VirtualBase2;
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::Base0>() == 1);
const _: () = assert!(rust_std::mem::align_of::<crate::Base0>() == 1);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Base0: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Base0: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<crate::Base1>() == 16);
const _: () = assert!(rust_std::mem::align_of::<crate::Base1>() == 8);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Base1: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Base1: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Base1, b1_1_) == 0);
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Base1, b1_2_) == 8);

const _: () = assert!(rust_std::mem::size_of::<crate::Base2>() == 2);
const _: () = assert!(rust_std::mem::align_of::<crate::Base2>() == 2);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Base2: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Base2: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Base2, b2_1_) == 0);

const _: () = assert!(rust_std::mem::size_of::<crate::Derived>() == 16);
const _: () = assert!(rust_std::mem::align_of::<crate::Derived>() == 8);
const _: () = {
    static_assertions::assert_impl_all!(crate::Derived: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Derived: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::Derived: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::Derived, derived_1) == 12);

const _: () = assert!(rust_std::mem::size_of::<crate::VirtualBase1>() == 24);
const _: () = assert!(rust_std::mem::align_of::<crate::VirtualBase1>() == 8);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::VirtualBase1: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::VirtualBase1: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<crate::VirtualBase2>() == 24);
const _: () = assert!(rust_std::mem::align_of::<crate::VirtualBase2>() == 8);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::VirtualBase2: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::VirtualBase2: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<crate::VirtualDerived>() == 32);
const _: () = assert!(rust_std::mem::align_of::<crate::VirtualDerived>() == 8);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::VirtualDerived: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::VirtualDerived: Drop);
};

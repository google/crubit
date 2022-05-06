// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:inheritance_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ::std as rust_std;
use memoffset_unstable_const::offset_of;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Using classes to force these to be non-POD.
/// In the Itanium ABI, the tail padding of POD types cannot be reused by other
/// objects, even if the POD type is potentially-overlapping.
#[repr(C)]
pub struct Base0 {
    __non_field_data: [rust_std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("Base0"), Base0);

impl !Unpin for Base0 {}

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

#[repr(C)]
pub struct Base1 {
    b1_1_: i64,
    b1_2_: u8,
}
forward_declare::unsafe_define!(forward_declare::symbol!("Base1"), Base1);

impl !Unpin for Base1 {}

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

#[repr(C)]
pub struct Base2 {
    b2_1_: i16,
}
forward_declare::unsafe_define!(forward_declare::symbol!("Base2"), Base2);

impl !Unpin for Base2 {}

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
    __non_field_data: [rust_std::mem::MaybeUninit<u8>; 12],
    pub derived_1: u8,
}
forward_declare::unsafe_define!(forward_declare::symbol!("Derived"), Derived);

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

unsafe impl oops::Inherits<Base0> for Derived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const Base0 {
        (derived as *const _ as *const u8).offset(0) as *const Base0
    }
}
unsafe impl oops::Inherits<Base1> for Derived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const Base1 {
        (derived as *const _ as *const u8).offset(0) as *const Base1
    }
}
unsafe impl oops::Inherits<Base2> for Derived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const Base2 {
        (derived as *const _ as *const u8).offset(10) as *const Base2
    }
}

#[repr(C, align(8))]
pub struct VirtualBase1 {
    __non_field_data: [rust_std::mem::MaybeUninit<u8>; 24],
}
forward_declare::unsafe_define!(forward_declare::symbol!("VirtualBase1"), VirtualBase1);

impl !Unpin for VirtualBase1 {}

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

unsafe impl oops::Inherits<Base1> for VirtualBase1 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const Base1 {
        detail::__crubit_dynamic_upcast__VirtualBase1__to__Base1(derived)
    }
}

#[repr(C, align(8))]
pub struct VirtualBase2 {
    __non_field_data: [rust_std::mem::MaybeUninit<u8>; 24],
}
forward_declare::unsafe_define!(forward_declare::symbol!("VirtualBase2"), VirtualBase2);

impl !Unpin for VirtualBase2 {}

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

unsafe impl oops::Inherits<Base1> for VirtualBase2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const Base1 {
        detail::__crubit_dynamic_upcast__VirtualBase2__to__Base1(derived)
    }
}

#[repr(C, align(8))]
pub struct VirtualDerived {
    __non_field_data: [rust_std::mem::MaybeUninit<u8>; 32],
}
forward_declare::unsafe_define!(forward_declare::symbol!("VirtualDerived"), VirtualDerived);

impl !Unpin for VirtualDerived {}

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

unsafe impl oops::Inherits<VirtualBase1> for VirtualDerived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const VirtualBase1 {
        detail::__crubit_dynamic_upcast__VirtualDerived__to__VirtualBase1(derived)
    }
}
unsafe impl oops::Inherits<Base1> for VirtualDerived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const Base1 {
        detail::__crubit_dynamic_upcast__VirtualDerived__to__Base1(derived)
    }
}
unsafe impl oops::Inherits<VirtualBase2> for VirtualDerived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const VirtualBase2 {
        detail::__crubit_dynamic_upcast__VirtualDerived__to__VirtualBase2(derived)
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_INHERITANCE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub fn __crubit_dynamic_upcast__VirtualBase1__to__Base1(
            from: *const VirtualBase1,
        ) -> *const Base1;
        pub fn __crubit_dynamic_upcast__VirtualBase2__to__Base1(
            from: *const VirtualBase2,
        ) -> *const Base1;
        pub fn __crubit_dynamic_upcast__VirtualDerived__to__VirtualBase1(
            from: *const VirtualDerived,
        ) -> *const VirtualBase1;
        pub fn __crubit_dynamic_upcast__VirtualDerived__to__Base1(
            from: *const VirtualDerived,
        ) -> *const Base1;
        pub fn __crubit_dynamic_upcast__VirtualDerived__to__VirtualBase2(
            from: *const VirtualDerived,
        ) -> *const VirtualBase2;
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<Base0>() == 1usize);
const _: () = assert!(rust_std::mem::align_of::<Base0>() == 1usize);
const _: () = {
    static_assertions::assert_not_impl_all!(Base0: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(Base0: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<Base1>() == 16usize);
const _: () = assert!(rust_std::mem::align_of::<Base1>() == 8usize);
const _: () = {
    static_assertions::assert_not_impl_all!(Base1: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(Base1: Drop);
};
const _: () = assert!(offset_of!(Base1, b1_1_) * 8 == 0usize);
const _: () = assert!(offset_of!(Base1, b1_2_) * 8 == 64usize);

const _: () = assert!(rust_std::mem::size_of::<Base2>() == 2usize);
const _: () = assert!(rust_std::mem::align_of::<Base2>() == 2usize);
const _: () = {
    static_assertions::assert_not_impl_all!(Base2: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(Base2: Drop);
};
const _: () = assert!(offset_of!(Base2, b2_1_) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<Derived>() == 16usize);
const _: () = assert!(rust_std::mem::align_of::<Derived>() == 8usize);
const _: () = {
    static_assertions::assert_impl_all!(Derived: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(Derived: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(Derived: Drop);
};
const _: () = assert!(offset_of!(Derived, derived_1) * 8 == 96usize);

const _: () = assert!(rust_std::mem::size_of::<VirtualBase1>() == 24usize);
const _: () = assert!(rust_std::mem::align_of::<VirtualBase1>() == 8usize);
const _: () = {
    static_assertions::assert_not_impl_all!(VirtualBase1: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(VirtualBase1: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<VirtualBase2>() == 24usize);
const _: () = assert!(rust_std::mem::align_of::<VirtualBase2>() == 8usize);
const _: () = {
    static_assertions::assert_not_impl_all!(VirtualBase2: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(VirtualBase2: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<VirtualDerived>() == 32usize);
const _: () = assert!(rust_std::mem::align_of::<VirtualDerived>() == 8usize);
const _: () = {
    static_assertions::assert_not_impl_all!(VirtualDerived: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(VirtualDerived: Drop);
};

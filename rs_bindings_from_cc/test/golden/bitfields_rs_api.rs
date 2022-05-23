// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:bitfields_cc
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

#[ctor::recursively_pinned]
#[repr(C, align(4))]
pub struct WithBitfields {
    // f1 : 2 bits
    __bitfields0: [crate::rust_std::mem::MaybeUninit<u8>; 1],
    pub f2: i32,
    // f3 : 4 bits
    // f4 : 8 bits
    //  : 45 bits
    __bitfields2: [crate::rust_std::mem::MaybeUninit<u8>; 10],
    pub f5: i32,
    // f6 : 23 bits
    __bitfields4: [crate::rust_std::mem::MaybeUninit<u8>; 3],
    f7: [crate::rust_std::mem::MaybeUninit<u8>; 1],
    // f8 : 2 bits
    __bitfields6: [crate::rust_std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("WithBitfields"), crate::WithBitfields);
impl WithBitfields {
    pub fn f7(&self) -> &u8 {
        unsafe { &*(&self.f7 as *const _ as *const u8) }
    }
}

// rs_bindings_from_cc/test/golden/bitfields.h;l=8
// Error while generating bindings for item 'WithBitfields::WithBitfields':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/bitfields.h;l=8
// Error while generating bindings for item 'WithBitfields::WithBitfields':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/bitfields.h;l=8
// Error while generating bindings for item 'WithBitfields::WithBitfields':
// Parameter #0 is not supported: Unsupported type 'struct WithBitfields &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/bitfields.h;l=8
// Error while generating bindings for item 'WithBitfields::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/bitfields.h;l=8
// Error while generating bindings for item 'WithBitfields::operator=':
// Parameter #0 is not supported: Unsupported type 'struct WithBitfields &&': Unsupported type: && without lifetime

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BITFIELDS_H_

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::WithBitfields>() == 32);
const _: () = assert!(rust_std::mem::align_of::<crate::WithBitfields>() == 4);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::WithBitfields: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::WithBitfields: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::WithBitfields, f2) == 4);
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::WithBitfields, f5) == 20);
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::WithBitfields, f7) == 27);

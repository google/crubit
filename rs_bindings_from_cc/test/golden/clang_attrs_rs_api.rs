// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:clang_attrs_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(64))]
///CRUBIT_ANNOTATE: cpp_type=HasCustomAlignment
pub struct HasCustomAlignment {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 64],
}
impl !Send for HasCustomAlignment {}
impl !Sync for HasCustomAlignment {}
unsafe impl ::cxx::ExternType for HasCustomAlignment {
    type Id = ::cxx::type_id!("HasCustomAlignment");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("HasCustomAlignment"),
    crate::HasCustomAlignment
);

impl Default for HasCustomAlignment {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18HasCustomAlignmentC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=HasFieldWithCustomAlignment
pub struct HasFieldWithCustomAlignment {
    pub field: crate::HasCustomAlignment,
}
impl !Send for HasFieldWithCustomAlignment {}
impl !Sync for HasFieldWithCustomAlignment {}
unsafe impl ::cxx::ExternType for HasFieldWithCustomAlignment {
    type Id = ::cxx::type_id!("HasFieldWithCustomAlignment");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("HasFieldWithCustomAlignment"),
    crate::HasFieldWithCustomAlignment
);

impl Default for HasFieldWithCustomAlignment {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN27HasFieldWithCustomAlignmentC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(64))]
///CRUBIT_ANNOTATE: cpp_type=InheritsFromBaseWithCustomAlignment
pub struct InheritsFromBaseWithCustomAlignment {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 64],
}
impl !Send for InheritsFromBaseWithCustomAlignment {}
impl !Sync for InheritsFromBaseWithCustomAlignment {}
unsafe impl ::cxx::ExternType for InheritsFromBaseWithCustomAlignment {
    type Id = ::cxx::type_id!("InheritsFromBaseWithCustomAlignment");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("InheritsFromBaseWithCustomAlignment"),
    crate::InheritsFromBaseWithCustomAlignment
);

impl Default for InheritsFromBaseWithCustomAlignment {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

unsafe impl oops::Inherits<crate::HasCustomAlignment>
    for crate::InheritsFromBaseWithCustomAlignment
{
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::HasCustomAlignment {
        (derived as *const _ as *const u8).offset(0) as *const crate::HasCustomAlignment
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(64))]
///CRUBIT_ANNOTATE: cpp_type=HasCustomAlignmentWithGnuAttr
pub struct HasCustomAlignmentWithGnuAttr {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 64],
}
impl !Send for HasCustomAlignmentWithGnuAttr {}
impl !Sync for HasCustomAlignmentWithGnuAttr {}
unsafe impl ::cxx::ExternType for HasCustomAlignmentWithGnuAttr {
    type Id = ::cxx::type_id!("HasCustomAlignmentWithGnuAttr");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("HasCustomAlignmentWithGnuAttr"),
    crate::HasCustomAlignmentWithGnuAttr
);

impl Default for HasCustomAlignmentWithGnuAttr {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

pub mod template_with_preferred_name {
    // Error while generating bindings for class 'template_with_preferred_name::SomeTemplate':
    // Class templates are not supported yet

    /// Based on `llvm/include/c++/v1/__fwd/string_view.h` - mimics
    /// definition of the `string_view` type alias.
    pub type SpecializedTypeAlias =
        crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE;

    // Based on `llvm/include/c++/v1/string_view` - mimics definition of
    // `basic_string_view` class template (focusing on the attributes related to the
    // preferred name).
}

// namespace template_with_preferred_name

/// Based on `llvm/include/c++/v1/__fwd/string_view.h` - mimics
/// forward declaration of `basic_string_view` class template.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=template_with_preferred_name :: SomeTemplate < int >
pub struct __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE {}
impl !Sync for __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("template_with_preferred_name :: SomeTemplate < int >"),
    crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE
);

impl Default for __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl __CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE {
    #[inline(always)]
    pub fn foo<'a>(&'a mut self) -> ::core::ffi::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiE3fooEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc(self)
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN18HasCustomAlignmentC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN27HasFieldWithCustomAlignmentC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN35InheritsFromBaseWithCustomAlignmentC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN29HasCustomAlignmentWithGnuAttrC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN28template_with_preferred_name12SomeTemplateIiE3fooEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3aclang_5fattrs_5fcc<
            'a,
        >(
            __this: &'a mut crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
        ) -> ::core::ffi::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::HasCustomAlignment>() == 64);
    assert!(::core::mem::align_of::<crate::HasCustomAlignment>() == 64);
    static_assertions::assert_impl_all!(crate::HasCustomAlignment: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::HasCustomAlignment: Drop);

    assert!(::core::mem::size_of::<crate::HasFieldWithCustomAlignment>() == 64);
    assert!(::core::mem::align_of::<crate::HasFieldWithCustomAlignment>() == 64);
    static_assertions::assert_impl_all!(crate::HasFieldWithCustomAlignment: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::HasFieldWithCustomAlignment: Drop);
    assert!(::core::mem::offset_of!(crate::HasFieldWithCustomAlignment, field) == 0);
    assert!(::core::mem::size_of::<crate::InheritsFromBaseWithCustomAlignment>() == 64);
    assert!(::core::mem::align_of::<crate::InheritsFromBaseWithCustomAlignment>() == 64);
    static_assertions::assert_impl_all!(crate::InheritsFromBaseWithCustomAlignment: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::InheritsFromBaseWithCustomAlignment: Drop);

    assert!(::core::mem::size_of::<crate::HasCustomAlignmentWithGnuAttr>() == 64);
    assert!(::core::mem::align_of::<crate::HasCustomAlignmentWithGnuAttr>() == 64);
    static_assertions::assert_impl_all!(crate::HasCustomAlignmentWithGnuAttr: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::HasCustomAlignmentWithGnuAttr: Drop);

    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
        >() == 1
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE,
        >() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE: Drop);
};

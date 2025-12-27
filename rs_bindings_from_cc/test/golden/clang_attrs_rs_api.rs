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

// Error while generating bindings for constructor 'HasCustomAlignment::HasCustomAlignment':
// Can't generate bindings for HasCustomAlignment::HasCustomAlignment, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasCustomAlignment::HasCustomAlignment (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'HasCustomAlignment::HasCustomAlignment':
// Can't generate bindings for HasCustomAlignment::HasCustomAlignment, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasCustomAlignment::HasCustomAlignment (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'HasCustomAlignment::operator=':
// Can't generate bindings for HasCustomAlignment::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasCustomAlignment::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasCustomAlignment::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'HasCustomAlignment::operator=':
// Can't generate bindings for HasCustomAlignment::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasCustomAlignment::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasCustomAlignment::operator= (the type of __param_0 (parameter #1): references are not supported)

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

// Error while generating bindings for constructor 'HasFieldWithCustomAlignment::HasFieldWithCustomAlignment':
// Can't generate bindings for HasFieldWithCustomAlignment::HasFieldWithCustomAlignment, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasFieldWithCustomAlignment::HasFieldWithCustomAlignment (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'HasFieldWithCustomAlignment::HasFieldWithCustomAlignment':
// Can't generate bindings for HasFieldWithCustomAlignment::HasFieldWithCustomAlignment, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasFieldWithCustomAlignment::HasFieldWithCustomAlignment (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'HasFieldWithCustomAlignment::operator=':
// Can't generate bindings for HasFieldWithCustomAlignment::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasFieldWithCustomAlignment::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasFieldWithCustomAlignment::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'HasFieldWithCustomAlignment::operator=':
// Can't generate bindings for HasFieldWithCustomAlignment::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasFieldWithCustomAlignment::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasFieldWithCustomAlignment::operator= (the type of __param_0 (parameter #1): references are not supported)

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

// Error while generating bindings for constructor 'InheritsFromBaseWithCustomAlignment::InheritsFromBaseWithCustomAlignment':
// Can't generate bindings for InheritsFromBaseWithCustomAlignment::InheritsFromBaseWithCustomAlignment, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for InheritsFromBaseWithCustomAlignment::InheritsFromBaseWithCustomAlignment (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'InheritsFromBaseWithCustomAlignment::InheritsFromBaseWithCustomAlignment':
// Can't generate bindings for InheritsFromBaseWithCustomAlignment::InheritsFromBaseWithCustomAlignment, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for InheritsFromBaseWithCustomAlignment::InheritsFromBaseWithCustomAlignment (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'InheritsFromBaseWithCustomAlignment::operator=':
// Can't generate bindings for InheritsFromBaseWithCustomAlignment::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for InheritsFromBaseWithCustomAlignment::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for InheritsFromBaseWithCustomAlignment::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'InheritsFromBaseWithCustomAlignment::operator=':
// Can't generate bindings for InheritsFromBaseWithCustomAlignment::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for InheritsFromBaseWithCustomAlignment::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for InheritsFromBaseWithCustomAlignment::operator= (the type of __param_0 (parameter #1): references are not supported)

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

// Error while generating bindings for constructor 'HasCustomAlignmentWithGnuAttr::HasCustomAlignmentWithGnuAttr':
// Can't generate bindings for HasCustomAlignmentWithGnuAttr::HasCustomAlignmentWithGnuAttr, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasCustomAlignmentWithGnuAttr::HasCustomAlignmentWithGnuAttr (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'HasCustomAlignmentWithGnuAttr::HasCustomAlignmentWithGnuAttr':
// Can't generate bindings for HasCustomAlignmentWithGnuAttr::HasCustomAlignmentWithGnuAttr, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasCustomAlignmentWithGnuAttr::HasCustomAlignmentWithGnuAttr (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'HasCustomAlignmentWithGnuAttr::operator=':
// Can't generate bindings for HasCustomAlignmentWithGnuAttr::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasCustomAlignmentWithGnuAttr::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasCustomAlignmentWithGnuAttr::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'HasCustomAlignmentWithGnuAttr::operator=':
// Can't generate bindings for HasCustomAlignmentWithGnuAttr::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasCustomAlignmentWithGnuAttr::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:experimental] for HasCustomAlignmentWithGnuAttr::operator= (the type of __param_0 (parameter #1): references are not supported)

pub mod template_with_preferred_name { // Error while generating bindings for class 'template_with_preferred_name::SomeTemplate':
                                       // Class templates are not supported yet

    // Error while generating bindings for type alias 'template_with_preferred_name::SpecializedTypeAlias':
    // Can't generate bindings for template_with_preferred_name::SpecializedTypeAlias, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:wrapper] for template_with_preferred_name::SpecializedTypeAlias (error: Can't generate bindings for template_with_preferred_name::SomeTemplate<int>, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:wrapper] for template_with_preferred_name::SomeTemplate<int> (crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE is a template instantiation)
    // //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:wrapper] for template_with_preferred_name::SomeTemplate<int> (crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE is a template instantiation))

    // Based on `llvm/include/c++/v1/string_view` - mimics definition of
    // `basic_string_view` class template (focusing on the attributes related to the
    // preferred name).
}

// namespace template_with_preferred_name

// Error while generating bindings for struct 'template_with_preferred_name::SomeTemplate<int>':
// Can't generate bindings for template_with_preferred_name::SomeTemplate<int>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:wrapper] for template_with_preferred_name::SomeTemplate<int> (crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE is a template instantiation)
// //rs_bindings_from_cc/test/golden:clang_attrs_cc needs [//features:wrapper] for template_with_preferred_name::SomeTemplate<int> (crate::__CcTemplateInstN28template_with_preferred_name12SomeTemplateIiEE is a template instantiation)

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
};

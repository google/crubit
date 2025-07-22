// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:comment_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// File comment

// TODO(b/202933018): Re-enable once namespaces are supported
// namespace ns {
// a

/// Foo
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Foo
pub struct Foo {
    /// A field
    pub i: ::core::ffi::c_int,
    /// Another field
    pub j: ::core::ffi::c_int,
}
impl !Send for Foo {}
impl !Sync for Foo {}
unsafe impl ::cxx::ExternType for Foo {
    type Id = ::cxx::type_id!("Foo");
    type Kind = ::cxx::kind::Trivial;
}

// Error while generating bindings for constructor 'Foo::Foo':
// Can't generate bindings for Foo::Foo, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Foo::Foo (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for constructor 'Foo::Foo':
// Can't generate bindings for Foo::Foo, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Foo::Foo (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Foo::Foo (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'Foo::Foo':
// Can't generate bindings for Foo::Foo, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Foo::Foo (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Foo::Foo (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Foo::operator=':
// Can't generate bindings for Foo::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Foo::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Foo::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Foo::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Foo::operator=':
// Can't generate bindings for Foo::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Foo::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Foo::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Foo::operator= (the type of __param_0 (parameter #1): references are not supported)

// TODO(rosica): This comment appears near fields of a struct, and
// is currently generated below the struct definiton on the Rust side.

// TODO(rosica): This comment appears between fields of a struct, and
// is currently generated below the struct definiton on the Rust side.

// TODO(rosica): This comment appears near fields of a struct, and
// is currently generated below the struct definiton on the Rust side.

// b

// }  // namespace ns

// c

/// foo
#[inline(always)]
pub fn foo() {
    unsafe { crate::detail::__rust_thunk___Z3foov() }
}

/// Bar
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Bar
pub struct Bar {
    pub i: ::core::ffi::c_int,
}
impl !Send for Bar {}
impl !Sync for Bar {}
unsafe impl ::cxx::ExternType for Bar {
    type Id = ::cxx::type_id!("Bar");
    type Kind = ::cxx::kind::Trivial;
}

// Error while generating bindings for constructor 'Bar::Bar':
// Can't generate bindings for Bar::Bar, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Bar::Bar (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for constructor 'Bar::Bar':
// Can't generate bindings for Bar::Bar, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Bar::Bar (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Bar::Bar (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'Bar::Bar':
// Can't generate bindings for Bar::Bar, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Bar::Bar (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Bar::Bar (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Bar::operator=':
// Can't generate bindings for Bar::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Bar::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Bar::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Bar::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Bar::operator=':
// Can't generate bindings for Bar::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Bar::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Bar::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for Bar::operator= (the type of __param_0 (parameter #1): references are not supported)

/// d
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=HasNoComments
pub struct HasNoComments {
    pub i: ::core::ffi::c_int,
}
impl !Send for HasNoComments {}
impl !Sync for HasNoComments {}
unsafe impl ::cxx::ExternType for HasNoComments {
    type Id = ::cxx::type_id!("HasNoComments");
    type Kind = ::cxx::kind::Trivial;
}

// Error while generating bindings for constructor 'HasNoComments::HasNoComments':
// Can't generate bindings for HasNoComments::HasNoComments, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for HasNoComments::HasNoComments (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for constructor 'HasNoComments::HasNoComments':
// Can't generate bindings for HasNoComments::HasNoComments, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for HasNoComments::HasNoComments (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for HasNoComments::HasNoComments (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'HasNoComments::HasNoComments':
// Can't generate bindings for HasNoComments::HasNoComments, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for HasNoComments::HasNoComments (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for HasNoComments::HasNoComments (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'HasNoComments::operator=':
// Can't generate bindings for HasNoComments::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for HasNoComments::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for HasNoComments::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for HasNoComments::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'HasNoComments::operator=':
// Can't generate bindings for HasNoComments::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for HasNoComments::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for HasNoComments::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:comment_cc needs [//features:experimental] for HasNoComments::operator= (the type of __param_0 (parameter #1): references are not supported)

// e

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z3foov();
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Foo>() == 8);
    assert!(::core::mem::align_of::<crate::Foo>() == 4);
    static_assertions::assert_impl_all!(crate::Foo: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Foo: Drop);
    assert!(::core::mem::offset_of!(crate::Foo, i) == 0);
    assert!(::core::mem::offset_of!(crate::Foo, j) == 4);
    assert!(::core::mem::size_of::<crate::Bar>() == 4);
    assert!(::core::mem::align_of::<crate::Bar>() == 4);
    static_assertions::assert_impl_all!(crate::Bar: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Bar: Drop);
    assert!(::core::mem::offset_of!(crate::Bar, i) == 0);
    assert!(::core::mem::size_of::<crate::HasNoComments>() == 4);
    assert!(::core::mem::align_of::<crate::HasNoComments>() == 4);
    static_assertions::assert_impl_all!(crate::HasNoComments: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::HasNoComments: Drop);
    assert!(::core::mem::offset_of!(crate::HasNoComments, i) == 0);
};

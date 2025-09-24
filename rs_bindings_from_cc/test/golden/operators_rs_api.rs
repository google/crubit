// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:operators_cc

#![rustfmt::skip]
#![feature(
    allocator_api,
    cfg_sanitize,
    custom_inner_attributes,
    impl_trait_in_assoc_type,
    negative_impls
)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=AddableConstMember
pub struct AddableConstMember {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for AddableConstMember {}
impl !Sync for AddableConstMember {}
unsafe impl ::cxx::ExternType for AddableConstMember {
    type Id = ::cxx::type_id!("AddableConstMember");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddableConstMember {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableConstMemberC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AddableConstMember::AddableConstMember':
// Can't generate bindings for AddableConstMember::AddableConstMember, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableConstMember::AddableConstMember (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AddableConstMember::AddableConstMember':
// Can't generate bindings for AddableConstMember::AddableConstMember, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableConstMember::AddableConstMember (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableConstMember::operator=':
// Can't generate bindings for AddableConstMember::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableConstMember::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableConstMember::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableConstMember::operator=':
// Can't generate bindings for AddableConstMember::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableConstMember::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableConstMember::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableConstMember::operator+':
// Can't generate bindings for AddableConstMember::operator+, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableConstMember::operator+ (the type of rhs (parameter #1): references are not supported)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=AddableNonConstMember
pub struct AddableNonConstMember {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for AddableNonConstMember {}
impl !Sync for AddableNonConstMember {}
unsafe impl ::cxx::ExternType for AddableNonConstMember {
    type Id = ::cxx::type_id!("AddableNonConstMember");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddableNonConstMember {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21AddableNonConstMemberC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AddableNonConstMember::AddableNonConstMember':
// Can't generate bindings for AddableNonConstMember::AddableNonConstMember, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableNonConstMember::AddableNonConstMember (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AddableNonConstMember::AddableNonConstMember':
// Can't generate bindings for AddableNonConstMember::AddableNonConstMember, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableNonConstMember::AddableNonConstMember (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableNonConstMember::operator=':
// Can't generate bindings for AddableNonConstMember::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableNonConstMember::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableNonConstMember::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableNonConstMember::operator=':
// Can't generate bindings for AddableNonConstMember::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableNonConstMember::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableNonConstMember::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableNonConstMember::operator+':
// Can't generate bindings for AddableNonConstMember::operator+, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableNonConstMember::operator+ (the type of rhs (parameter #1): references are not supported)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=AddableFriend
pub struct AddableFriend {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for AddableFriend {}
impl !Sync for AddableFriend {}
unsafe impl ::cxx::ExternType for AddableFriend {
    type Id = ::cxx::type_id!("AddableFriend");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddableFriend {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13AddableFriendC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AddableFriend::AddableFriend':
// Can't generate bindings for AddableFriend::AddableFriend, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFriend::AddableFriend (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AddableFriend::AddableFriend':
// Can't generate bindings for AddableFriend::AddableFriend, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFriend::AddableFriend (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableFriend::operator=':
// Can't generate bindings for AddableFriend::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFriend::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFriend::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableFriend::operator=':
// Can't generate bindings for AddableFriend::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFriend::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFriend::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'operator+':
// Can't generate bindings for operator+, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for operator+ (the type of lhs (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for operator+ (the type of rhs (parameter #1): references are not supported)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddableFreeByConstRef
pub struct AddableFreeByConstRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddableFreeByConstRef {}
impl !Sync for AddableFreeByConstRef {}
unsafe impl ::cxx::ExternType for AddableFreeByConstRef {
    type Id = ::cxx::type_id!("AddableFreeByConstRef");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddableFreeByConstRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21AddableFreeByConstRefC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AddableFreeByConstRef::AddableFreeByConstRef':
// Can't generate bindings for AddableFreeByConstRef::AddableFreeByConstRef, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByConstRef::AddableFreeByConstRef (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AddableFreeByConstRef::AddableFreeByConstRef':
// Can't generate bindings for AddableFreeByConstRef::AddableFreeByConstRef, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByConstRef::AddableFreeByConstRef (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableFreeByConstRef::operator=':
// Can't generate bindings for AddableFreeByConstRef::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByConstRef::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByConstRef::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableFreeByConstRef::operator=':
// Can't generate bindings for AddableFreeByConstRef::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByConstRef::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByConstRef::operator= (the type of __param_0 (parameter #1): references are not supported)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddableFreeByMutRef
pub struct AddableFreeByMutRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddableFreeByMutRef {}
impl !Sync for AddableFreeByMutRef {}
unsafe impl ::cxx::ExternType for AddableFreeByMutRef {
    type Id = ::cxx::type_id!("AddableFreeByMutRef");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddableFreeByMutRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN19AddableFreeByMutRefC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AddableFreeByMutRef::AddableFreeByMutRef':
// Can't generate bindings for AddableFreeByMutRef::AddableFreeByMutRef, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByMutRef::AddableFreeByMutRef (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AddableFreeByMutRef::AddableFreeByMutRef':
// Can't generate bindings for AddableFreeByMutRef::AddableFreeByMutRef, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByMutRef::AddableFreeByMutRef (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableFreeByMutRef::operator=':
// Can't generate bindings for AddableFreeByMutRef::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByMutRef::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByMutRef::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableFreeByMutRef::operator=':
// Can't generate bindings for AddableFreeByMutRef::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByMutRef::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByMutRef::operator= (the type of __param_0 (parameter #1): references are not supported)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddableFreeByValue
pub struct AddableFreeByValue {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddableFreeByValue {}
impl !Sync for AddableFreeByValue {}
unsafe impl ::cxx::ExternType for AddableFreeByValue {
    type Id = ::cxx::type_id!("AddableFreeByValue");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddableFreeByValue {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableFreeByValueC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AddableFreeByValue::AddableFreeByValue':
// Can't generate bindings for AddableFreeByValue::AddableFreeByValue, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByValue::AddableFreeByValue (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AddableFreeByValue::AddableFreeByValue':
// Can't generate bindings for AddableFreeByValue::AddableFreeByValue, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByValue::AddableFreeByValue (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableFreeByValue::operator=':
// Can't generate bindings for AddableFreeByValue::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByValue::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByValue::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableFreeByValue::operator=':
// Can't generate bindings for AddableFreeByValue::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByValue::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByValue::operator= (the type of __param_0 (parameter #1): references are not supported)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddableFreeByRValueRef
pub struct AddableFreeByRValueRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddableFreeByRValueRef {}
impl !Sync for AddableFreeByRValueRef {}
unsafe impl ::cxx::ExternType for AddableFreeByRValueRef {
    type Id = ::cxx::type_id!("AddableFreeByRValueRef");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddableFreeByRValueRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN22AddableFreeByRValueRefC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AddableFreeByRValueRef::AddableFreeByRValueRef':
// Can't generate bindings for AddableFreeByRValueRef::AddableFreeByRValueRef, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByRValueRef::AddableFreeByRValueRef (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AddableFreeByRValueRef::AddableFreeByRValueRef':
// Can't generate bindings for AddableFreeByRValueRef::AddableFreeByRValueRef, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByRValueRef::AddableFreeByRValueRef (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableFreeByRValueRef::operator=':
// Can't generate bindings for AddableFreeByRValueRef::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByRValueRef::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByRValueRef::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableFreeByRValueRef::operator=':
// Can't generate bindings for AddableFreeByRValueRef::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByRValueRef::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableFreeByRValueRef::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'operator+':
// Can't generate bindings for operator+, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for operator+ (the type of lhs (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for operator+ (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'operator+':
// Can't generate bindings for operator+, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for operator+ (the type of lhs (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for operator+ (the type of rhs (parameter #1): references are not supported)

impl ::core::ops::Add<Self> for crate::AddableFreeByValue {
    type Output = crate::AddableFreeByValue;
    #[inline(always)]
    fn add(mut self, mut rhs: Self) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___Zpl18AddableFreeByValueS_(
                &raw mut __return as *mut ::core::ffi::c_void,
                &mut self,
                &mut rhs,
            );
            __return.assume_init()
        }
    }
}

// Error while generating bindings for function 'operator+':
// Rvalue reference types are not yet supported as first parameter of operators (b/219826128)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Overloaded
pub struct Overloaded {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Overloaded {}
impl !Sync for Overloaded {}
unsafe impl ::cxx::ExternType for Overloaded {
    type Id = ::cxx::type_id!("Overloaded");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for Overloaded {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10OverloadedC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'Overloaded::Overloaded':
// Can't generate bindings for Overloaded::Overloaded, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for Overloaded::Overloaded (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'Overloaded::Overloaded':
// Can't generate bindings for Overloaded::Overloaded, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for Overloaded::Overloaded (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Overloaded::operator=':
// Can't generate bindings for Overloaded::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for Overloaded::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for Overloaded::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'Overloaded::operator=':
// Can't generate bindings for Overloaded::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for Overloaded::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for Overloaded::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'operator+':
// Can't generate bindings for operator+, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for operator+ (the type of lhs (parameter #0): references are not supported)

// Error while generating bindings for function 'operator+':
// Can't generate bindings for operator+, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for operator+ (the type of lhs (parameter #0): references are not supported)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=IncompatibleLHS
pub struct IncompatibleLHS {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for IncompatibleLHS {}
impl !Sync for IncompatibleLHS {}
unsafe impl ::cxx::ExternType for IncompatibleLHS {
    type Id = ::cxx::type_id!("IncompatibleLHS");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for IncompatibleLHS {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15IncompatibleLHSC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'IncompatibleLHS::IncompatibleLHS':
// Can't generate bindings for IncompatibleLHS::IncompatibleLHS, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for IncompatibleLHS::IncompatibleLHS (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'IncompatibleLHS::IncompatibleLHS':
// Can't generate bindings for IncompatibleLHS::IncompatibleLHS, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for IncompatibleLHS::IncompatibleLHS (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'IncompatibleLHS::operator=':
// Can't generate bindings for IncompatibleLHS::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for IncompatibleLHS::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for IncompatibleLHS::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'IncompatibleLHS::operator=':
// Can't generate bindings for IncompatibleLHS::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for IncompatibleLHS::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for IncompatibleLHS::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'operator+':
// Non-record-nor-reference operator parameters are not yet supported, found ::core::ffi::c_int

// Error while generating bindings for function 'operator+':
// Expected first operator parameter to be a record or incomplete record, found ::core::ffi::c_int

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=AddableReturnsVoid
pub struct AddableReturnsVoid {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for AddableReturnsVoid {}
impl !Sync for AddableReturnsVoid {}
unsafe impl ::cxx::ExternType for AddableReturnsVoid {
    type Id = ::cxx::type_id!("AddableReturnsVoid");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddableReturnsVoid {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableReturnsVoidC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AddableReturnsVoid::AddableReturnsVoid':
// Can't generate bindings for AddableReturnsVoid::AddableReturnsVoid, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableReturnsVoid::AddableReturnsVoid (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AddableReturnsVoid::AddableReturnsVoid':
// Can't generate bindings for AddableReturnsVoid::AddableReturnsVoid, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableReturnsVoid::AddableReturnsVoid (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableReturnsVoid::operator=':
// Can't generate bindings for AddableReturnsVoid::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableReturnsVoid::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableReturnsVoid::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableReturnsVoid::operator=':
// Can't generate bindings for AddableReturnsVoid::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableReturnsVoid::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableReturnsVoid::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableReturnsVoid::operator+':
// Can't generate bindings for AddableReturnsVoid::operator+, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableReturnsVoid::operator+ (the type of rhs (parameter #1): references are not supported)

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=AddableConstMemberNonunpin
pub struct AddableConstMemberNonunpin {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for AddableConstMemberNonunpin {}
impl !Sync for AddableConstMemberNonunpin {}
unsafe impl ::cxx::ExternType for AddableConstMemberNonunpin {
    type Id = ::cxx::type_id!("AddableConstMemberNonunpin");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for AddableConstMemberNonunpin {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinC1Ev(
                    dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

// Error while generating bindings for constructor 'AddableConstMemberNonunpin::AddableConstMemberNonunpin':
// Can't generate bindings for AddableConstMemberNonunpin::AddableConstMemberNonunpin, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableConstMemberNonunpin::AddableConstMemberNonunpin (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableConstMemberNonunpin::operator=':
// Can't generate bindings for AddableConstMemberNonunpin::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableConstMemberNonunpin::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableConstMemberNonunpin::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddableConstMemberNonunpin::operator+':
// Can't generate bindings for AddableConstMemberNonunpin::operator+, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddableConstMemberNonunpin::operator+ (the type of rhs (parameter #1): references are not supported)

impl ::ctor::PinnedDrop for AddableConstMemberNonunpin {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinD1Ev(self)
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignMemberInt
pub struct AddAssignMemberInt {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignMemberInt {}
impl !Sync for AddAssignMemberInt {}
unsafe impl ::cxx::ExternType for AddAssignMemberInt {
    type Id = ::cxx::type_id!("AddAssignMemberInt");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddAssignMemberInt {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddAssignMemberIntC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AddAssignMemberInt::AddAssignMemberInt':
// Can't generate bindings for AddAssignMemberInt::AddAssignMemberInt, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignMemberInt::AddAssignMemberInt (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AddAssignMemberInt::AddAssignMemberInt':
// Can't generate bindings for AddAssignMemberInt::AddAssignMemberInt, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignMemberInt::AddAssignMemberInt (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignMemberInt::operator=':
// Can't generate bindings for AddAssignMemberInt::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignMemberInt::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignMemberInt::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignMemberInt::operator=':
// Can't generate bindings for AddAssignMemberInt::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignMemberInt::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignMemberInt::operator= (the type of __param_0 (parameter #1): references are not supported)

impl ::core::ops::AddAssign<::core::ffi::c_int> for AddAssignMemberInt {
    #[inline(always)]
    fn add_assign<'a>(&'a mut self, rhs: ::core::ffi::c_int) {
        unsafe {
            crate::detail::__rust_thunk___ZN18AddAssignMemberIntpLEi(self, rhs);
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignMemberByConstRef
pub struct AddAssignMemberByConstRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignMemberByConstRef {}
impl !Sync for AddAssignMemberByConstRef {}
unsafe impl ::cxx::ExternType for AddAssignMemberByConstRef {
    type Id = ::cxx::type_id!("AddAssignMemberByConstRef");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddAssignMemberByConstRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignMemberByConstRefC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AddAssignMemberByConstRef::AddAssignMemberByConstRef':
// Can't generate bindings for AddAssignMemberByConstRef::AddAssignMemberByConstRef, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignMemberByConstRef::AddAssignMemberByConstRef (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AddAssignMemberByConstRef::AddAssignMemberByConstRef':
// Can't generate bindings for AddAssignMemberByConstRef::AddAssignMemberByConstRef, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignMemberByConstRef::AddAssignMemberByConstRef (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignMemberByConstRef::operator=':
// Can't generate bindings for AddAssignMemberByConstRef::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignMemberByConstRef::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignMemberByConstRef::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignMemberByConstRef::operator=':
// Can't generate bindings for AddAssignMemberByConstRef::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignMemberByConstRef::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignMemberByConstRef::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignMemberByConstRef::operator+=':
// Can't generate bindings for AddAssignMemberByConstRef::operator+=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignMemberByConstRef::operator+= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignMemberByConstRef::operator+= (the type of rhs (parameter #1): references are not supported)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignFreeByConstRef
pub struct AddAssignFreeByConstRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignFreeByConstRef {}
impl !Sync for AddAssignFreeByConstRef {}
unsafe impl ::cxx::ExternType for AddAssignFreeByConstRef {
    type Id = ::cxx::type_id!("AddAssignFreeByConstRef");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddAssignFreeByConstRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23AddAssignFreeByConstRefC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AddAssignFreeByConstRef::AddAssignFreeByConstRef':
// Can't generate bindings for AddAssignFreeByConstRef::AddAssignFreeByConstRef, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFreeByConstRef::AddAssignFreeByConstRef (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AddAssignFreeByConstRef::AddAssignFreeByConstRef':
// Can't generate bindings for AddAssignFreeByConstRef::AddAssignFreeByConstRef, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFreeByConstRef::AddAssignFreeByConstRef (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignFreeByConstRef::operator=':
// Can't generate bindings for AddAssignFreeByConstRef::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFreeByConstRef::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFreeByConstRef::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignFreeByConstRef::operator=':
// Can't generate bindings for AddAssignFreeByConstRef::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFreeByConstRef::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFreeByConstRef::operator= (the type of __param_0 (parameter #1): references are not supported)

impl ::core::ops::AddAssign<&Self> for crate::AddAssignFreeByConstRef {
    #[inline(always)]
    fn add_assign(&mut self, rhs: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZpLR23AddAssignFreeByConstRefRKS_(self, rhs);
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignFreeByValue
pub struct AddAssignFreeByValue {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignFreeByValue {}
impl !Sync for AddAssignFreeByValue {}
unsafe impl ::cxx::ExternType for AddAssignFreeByValue {
    type Id = ::cxx::type_id!("AddAssignFreeByValue");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddAssignFreeByValue {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20AddAssignFreeByValueC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AddAssignFreeByValue::AddAssignFreeByValue':
// Can't generate bindings for AddAssignFreeByValue::AddAssignFreeByValue, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFreeByValue::AddAssignFreeByValue (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AddAssignFreeByValue::AddAssignFreeByValue':
// Can't generate bindings for AddAssignFreeByValue::AddAssignFreeByValue, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFreeByValue::AddAssignFreeByValue (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignFreeByValue::operator=':
// Can't generate bindings for AddAssignFreeByValue::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFreeByValue::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFreeByValue::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignFreeByValue::operator=':
// Can't generate bindings for AddAssignFreeByValue::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFreeByValue::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFreeByValue::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'operator+=':
// Can't generate bindings for operator+=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for operator+= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for operator+= (the type of lhs (parameter #0): references are not supported)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignFriendByConstRef
pub struct AddAssignFriendByConstRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignFriendByConstRef {}
impl !Sync for AddAssignFriendByConstRef {}
unsafe impl ::cxx::ExternType for AddAssignFriendByConstRef {
    type Id = ::cxx::type_id!("AddAssignFriendByConstRef");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddAssignFriendByConstRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignFriendByConstRefC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AddAssignFriendByConstRef::AddAssignFriendByConstRef':
// Can't generate bindings for AddAssignFriendByConstRef::AddAssignFriendByConstRef, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFriendByConstRef::AddAssignFriendByConstRef (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AddAssignFriendByConstRef::AddAssignFriendByConstRef':
// Can't generate bindings for AddAssignFriendByConstRef::AddAssignFriendByConstRef, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFriendByConstRef::AddAssignFriendByConstRef (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignFriendByConstRef::operator=':
// Can't generate bindings for AddAssignFriendByConstRef::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFriendByConstRef::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFriendByConstRef::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignFriendByConstRef::operator=':
// Can't generate bindings for AddAssignFriendByConstRef::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFriendByConstRef::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFriendByConstRef::operator= (the type of __param_0 (parameter #1): references are not supported)

impl ::core::ops::AddAssign<&Self> for crate::AddAssignFriendByConstRef {
    #[inline(always)]
    fn add_assign(&mut self, rhs: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZpLR25AddAssignFriendByConstRefRKS_(self, rhs);
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignFriendByValue
pub struct AddAssignFriendByValue {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignFriendByValue {}
impl !Sync for AddAssignFriendByValue {}
unsafe impl ::cxx::ExternType for AddAssignFriendByValue {
    type Id = ::cxx::type_id!("AddAssignFriendByValue");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddAssignFriendByValue {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN22AddAssignFriendByValueC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AddAssignFriendByValue::AddAssignFriendByValue':
// Can't generate bindings for AddAssignFriendByValue::AddAssignFriendByValue, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFriendByValue::AddAssignFriendByValue (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AddAssignFriendByValue::AddAssignFriendByValue':
// Can't generate bindings for AddAssignFriendByValue::AddAssignFriendByValue, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFriendByValue::AddAssignFriendByValue (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignFriendByValue::operator=':
// Can't generate bindings for AddAssignFriendByValue::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFriendByValue::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFriendByValue::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignFriendByValue::operator=':
// Can't generate bindings for AddAssignFriendByValue::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFriendByValue::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignFriendByValue::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'operator+=':
// Can't generate bindings for operator+=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for operator+= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for operator+= (the type of lhs (parameter #0): references are not supported)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignProhibitedConstMember
pub struct AddAssignProhibitedConstMember {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignProhibitedConstMember {}
impl !Sync for AddAssignProhibitedConstMember {}
unsafe impl ::cxx::ExternType for AddAssignProhibitedConstMember {
    type Id = ::cxx::type_id!("AddAssignProhibitedConstMember");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddAssignProhibitedConstMember {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN30AddAssignProhibitedConstMemberC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AddAssignProhibitedConstMember::AddAssignProhibitedConstMember':
// Can't generate bindings for AddAssignProhibitedConstMember::AddAssignProhibitedConstMember, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignProhibitedConstMember::AddAssignProhibitedConstMember (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AddAssignProhibitedConstMember::AddAssignProhibitedConstMember':
// Can't generate bindings for AddAssignProhibitedConstMember::AddAssignProhibitedConstMember, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignProhibitedConstMember::AddAssignProhibitedConstMember (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignProhibitedConstMember::operator=':
// Can't generate bindings for AddAssignProhibitedConstMember::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignProhibitedConstMember::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignProhibitedConstMember::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignProhibitedConstMember::operator=':
// Can't generate bindings for AddAssignProhibitedConstMember::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignProhibitedConstMember::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignProhibitedConstMember::operator= (the type of __param_0 (parameter #1): references are not supported)

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nCompound assignment with const left-hand side is not supported, found &'a crate::AddAssignProhibitedConstMember"
)]
pub trait BindingFailedFor_ZNK30AddAssignProhibitedConstMemberpLEi {}
impl<'error> ::core::ops::AddAssign<::core::ffi::c_int> for AddAssignProhibitedConstMember
where
    &'error (): BindingFailedFor_ZNK30AddAssignProhibitedConstMemberpLEi,
{
    #[inline(always)]
    fn add_assign<'a>(&'a mut self, rhs: ::core::ffi::c_int) {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=AddAssignProhibitedFriendConstLhs
pub struct AddAssignProhibitedFriendConstLhs {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for AddAssignProhibitedFriendConstLhs {}
impl !Sync for AddAssignProhibitedFriendConstLhs {}
unsafe impl ::cxx::ExternType for AddAssignProhibitedFriendConstLhs {
    type Id = ::cxx::type_id!("AddAssignProhibitedFriendConstLhs");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for AddAssignProhibitedFriendConstLhs {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN33AddAssignProhibitedFriendConstLhsC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AddAssignProhibitedFriendConstLhs::AddAssignProhibitedFriendConstLhs':
// Can't generate bindings for AddAssignProhibitedFriendConstLhs::AddAssignProhibitedFriendConstLhs, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignProhibitedFriendConstLhs::AddAssignProhibitedFriendConstLhs (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AddAssignProhibitedFriendConstLhs::AddAssignProhibitedFriendConstLhs':
// Can't generate bindings for AddAssignProhibitedFriendConstLhs::AddAssignProhibitedFriendConstLhs, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignProhibitedFriendConstLhs::AddAssignProhibitedFriendConstLhs (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignProhibitedFriendConstLhs::operator=':
// Can't generate bindings for AddAssignProhibitedFriendConstLhs::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignProhibitedFriendConstLhs::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignProhibitedFriendConstLhs::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AddAssignProhibitedFriendConstLhs::operator=':
// Can't generate bindings for AddAssignProhibitedFriendConstLhs::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignProhibitedFriendConstLhs::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for AddAssignProhibitedFriendConstLhs::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'operator+=':
// Can't generate bindings for operator+=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for operator+= (the type of lhs (parameter #0): references are not supported)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=ManyOperators
pub struct ManyOperators {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for ManyOperators {}
impl !Sync for ManyOperators {}
unsafe impl ::cxx::ExternType for ManyOperators {
    type Id = ::cxx::type_id!("ManyOperators");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for ManyOperators {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'ManyOperators::ManyOperators':
// Can't generate bindings for ManyOperators::ManyOperators, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::ManyOperators (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'ManyOperators::ManyOperators':
// Can't generate bindings for ManyOperators::ManyOperators, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::ManyOperators (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator=':
// Can't generate bindings for ManyOperators::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator=':
// Can't generate bindings for ManyOperators::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator= (the type of __param_0 (parameter #1): references are not supported)

impl ManyOperators {
    #[inline(always)]
    pub fn unary_plus<'a>(&'a self) -> crate::ManyOperators {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorspsEv(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
            );
            __return.assume_init()
        }
    }
}

impl<'a> ::core::ops::Neg for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsngEv(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
            );
            __return.assume_init()
        }
    }
}

impl<'a> ::core::ops::Not for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn not(self) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ManyOperators>::uninit();
            crate::detail::__rust_thunk___ZNK13ManyOperatorsntEv(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
            );
            __return.assume_init()
        }
    }
}

// Error while generating bindings for function 'ManyOperators::operator~':
// Bindings for this kind of operator (operator ~ with 1 parameter(s)) are not supported

// Error while generating bindings for function 'ManyOperators::operator+':
// Can't generate bindings for ManyOperators::operator+, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator+ (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator-':
// Can't generate bindings for ManyOperators::operator-, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator- (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator*':
// Can't generate bindings for ManyOperators::operator*, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator* (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator/':
// Can't generate bindings for ManyOperators::operator/, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator/ (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator%':
// Can't generate bindings for ManyOperators::operator%, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator% (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator&':
// Can't generate bindings for ManyOperators::operator&, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator& (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator|':
// Can't generate bindings for ManyOperators::operator|, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator| (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator^':
// Can't generate bindings for ManyOperators::operator^, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator^ (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator<<':
// Can't generate bindings for ManyOperators::operator<<, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator<< (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator>>':
// Can't generate bindings for ManyOperators::operator>>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator>> (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator+=':
// Can't generate bindings for ManyOperators::operator+=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator+= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator+= (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator-=':
// Can't generate bindings for ManyOperators::operator-=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator-= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator-= (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator*=':
// Can't generate bindings for ManyOperators::operator*=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator*= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator*= (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator/=':
// Can't generate bindings for ManyOperators::operator/=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator/= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator/= (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator%=':
// Can't generate bindings for ManyOperators::operator%=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator%= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator%= (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator&=':
// Can't generate bindings for ManyOperators::operator&=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator&= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator&= (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator|=':
// Can't generate bindings for ManyOperators::operator|=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator|= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator|= (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator^=':
// Can't generate bindings for ManyOperators::operator^=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator^= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator^= (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator<<=':
// Can't generate bindings for ManyOperators::operator<<=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator<<= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator<<= (the type of rhs (parameter #1): references are not supported)

// Error while generating bindings for function 'ManyOperators::operator>>=':
// Can't generate bindings for ManyOperators::operator>>=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator>>= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:operators_cc needs [//features:experimental] for ManyOperators::operator>>= (the type of rhs (parameter #1): references are not supported)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN18AddableConstMemberC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21AddableNonConstMemberC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13AddableFriendC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN21AddableFreeByConstRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19AddableFreeByMutRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18AddableFreeByValueC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN22AddableFreeByRValueRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___Zpl18AddableFreeByValueS_(
            __return: *mut ::core::ffi::c_void,
            lhs: &mut crate::AddableFreeByValue,
            rhs: &mut crate::AddableFreeByValue,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10OverloadedC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN15IncompatibleLHSC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18AddableReturnsVoidC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN26AddableConstMemberNonunpinC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN26AddableConstMemberNonunpinD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::AddableConstMemberNonunpin>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18AddAssignMemberIntC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZN18AddAssignMemberIntpLEi"]
        pub(crate) unsafe fn __rust_thunk___ZN18AddAssignMemberIntpLEi<'a>(
            __this: &'a mut crate::AddAssignMemberInt,
            rhs: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN25AddAssignMemberByConstRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23AddAssignFreeByConstRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZpLR23AddAssignFreeByConstRefRKS_"]
        pub(crate) unsafe fn __rust_thunk___ZpLR23AddAssignFreeByConstRefRKS_<'__return_lifetime>(
            lhs: &mut crate::AddAssignFreeByConstRef,
            rhs: &crate::AddAssignFreeByConstRef,
        ) -> &'__return_lifetime mut crate::AddAssignFreeByConstRef;
        pub(crate) unsafe fn __rust_thunk___ZN20AddAssignFreeByValueC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN25AddAssignFriendByConstRefC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZpLR25AddAssignFriendByConstRefRKS_"]
        pub(crate) unsafe fn __rust_thunk___ZpLR25AddAssignFriendByConstRefRKS_<
            '__return_lifetime,
        >(
            lhs: &mut crate::AddAssignFriendByConstRef,
            rhs: &crate::AddAssignFriendByConstRef,
        ) -> &'__return_lifetime mut crate::AddAssignFriendByConstRef;
        pub(crate) unsafe fn __rust_thunk___ZN22AddAssignFriendByValueC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN30AddAssignProhibitedConstMemberC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN33AddAssignProhibitedFriendConstLhsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13ManyOperatorsC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorspsEv<'a>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsngEv<'a>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::ManyOperators,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK13ManyOperatorsntEv<'a>(
            __return: *mut ::core::ffi::c_void,
            __this: &'a crate::ManyOperators,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::AddableConstMember>() == 4);
    assert!(::core::mem::align_of::<crate::AddableConstMember>() == 4);
    static_assertions::assert_impl_all!(crate::AddableConstMember: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddableConstMember: Drop);
    assert!(::core::mem::offset_of!(crate::AddableConstMember, field_) == 0);
    assert!(::core::mem::size_of::<crate::AddableNonConstMember>() == 4);
    assert!(::core::mem::align_of::<crate::AddableNonConstMember>() == 4);
    static_assertions::assert_impl_all!(crate::AddableNonConstMember: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddableNonConstMember: Drop);
    assert!(::core::mem::offset_of!(crate::AddableNonConstMember, field_) == 0);
    assert!(::core::mem::size_of::<crate::AddableFriend>() == 4);
    assert!(::core::mem::align_of::<crate::AddableFriend>() == 4);
    static_assertions::assert_impl_all!(crate::AddableFriend: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddableFriend: Drop);
    assert!(::core::mem::offset_of!(crate::AddableFriend, field_) == 0);
    assert!(::core::mem::size_of::<crate::AddableFreeByConstRef>() == 1);
    assert!(::core::mem::align_of::<crate::AddableFreeByConstRef>() == 1);
    static_assertions::assert_impl_all!(crate::AddableFreeByConstRef: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddableFreeByConstRef: Drop);

    assert!(::core::mem::size_of::<crate::AddableFreeByMutRef>() == 1);
    assert!(::core::mem::align_of::<crate::AddableFreeByMutRef>() == 1);
    static_assertions::assert_impl_all!(crate::AddableFreeByMutRef: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddableFreeByMutRef: Drop);

    assert!(::core::mem::size_of::<crate::AddableFreeByValue>() == 1);
    assert!(::core::mem::align_of::<crate::AddableFreeByValue>() == 1);
    static_assertions::assert_impl_all!(crate::AddableFreeByValue: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddableFreeByValue: Drop);

    assert!(::core::mem::size_of::<crate::AddableFreeByRValueRef>() == 1);
    assert!(::core::mem::align_of::<crate::AddableFreeByRValueRef>() == 1);
    static_assertions::assert_impl_all!(crate::AddableFreeByRValueRef: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddableFreeByRValueRef: Drop);

    assert!(::core::mem::size_of::<crate::Overloaded>() == 1);
    assert!(::core::mem::align_of::<crate::Overloaded>() == 1);
    static_assertions::assert_impl_all!(crate::Overloaded: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Overloaded: Drop);

    assert!(::core::mem::size_of::<crate::IncompatibleLHS>() == 1);
    assert!(::core::mem::align_of::<crate::IncompatibleLHS>() == 1);
    static_assertions::assert_impl_all!(crate::IncompatibleLHS: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::IncompatibleLHS: Drop);

    assert!(::core::mem::size_of::<crate::AddableReturnsVoid>() == 4);
    assert!(::core::mem::align_of::<crate::AddableReturnsVoid>() == 4);
    static_assertions::assert_impl_all!(crate::AddableReturnsVoid: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddableReturnsVoid: Drop);
    assert!(::core::mem::offset_of!(crate::AddableReturnsVoid, field_) == 0);
    assert!(::core::mem::size_of::<crate::AddableConstMemberNonunpin>() == 4);
    assert!(::core::mem::align_of::<crate::AddableConstMemberNonunpin>() == 4);
    static_assertions::assert_impl_all!(crate::AddableConstMemberNonunpin: Drop);
    static_assertions::assert_not_impl_any!(crate::AddableConstMemberNonunpin: Copy);
    assert!(::core::mem::offset_of!(crate::AddableConstMemberNonunpin, field_) == 0);
    assert!(::core::mem::size_of::<crate::AddAssignMemberInt>() == 1);
    assert!(::core::mem::align_of::<crate::AddAssignMemberInt>() == 1);
    static_assertions::assert_impl_all!(crate::AddAssignMemberInt: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddAssignMemberInt: Drop);

    assert!(::core::mem::size_of::<crate::AddAssignMemberByConstRef>() == 1);
    assert!(::core::mem::align_of::<crate::AddAssignMemberByConstRef>() == 1);
    static_assertions::assert_impl_all!(crate::AddAssignMemberByConstRef: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddAssignMemberByConstRef: Drop);

    assert!(::core::mem::size_of::<crate::AddAssignFreeByConstRef>() == 1);
    assert!(::core::mem::align_of::<crate::AddAssignFreeByConstRef>() == 1);
    static_assertions::assert_impl_all!(crate::AddAssignFreeByConstRef: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddAssignFreeByConstRef: Drop);

    assert!(::core::mem::size_of::<crate::AddAssignFreeByValue>() == 1);
    assert!(::core::mem::align_of::<crate::AddAssignFreeByValue>() == 1);
    static_assertions::assert_impl_all!(crate::AddAssignFreeByValue: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddAssignFreeByValue: Drop);

    assert!(::core::mem::size_of::<crate::AddAssignFriendByConstRef>() == 1);
    assert!(::core::mem::align_of::<crate::AddAssignFriendByConstRef>() == 1);
    static_assertions::assert_impl_all!(crate::AddAssignFriendByConstRef: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddAssignFriendByConstRef: Drop);

    assert!(::core::mem::size_of::<crate::AddAssignFriendByValue>() == 1);
    assert!(::core::mem::align_of::<crate::AddAssignFriendByValue>() == 1);
    static_assertions::assert_impl_all!(crate::AddAssignFriendByValue: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddAssignFriendByValue: Drop);

    assert!(::core::mem::size_of::<crate::AddAssignProhibitedConstMember>() == 1);
    assert!(::core::mem::align_of::<crate::AddAssignProhibitedConstMember>() == 1);
    static_assertions::assert_impl_all!(crate::AddAssignProhibitedConstMember: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddAssignProhibitedConstMember: Drop);

    assert!(::core::mem::size_of::<crate::AddAssignProhibitedFriendConstLhs>() == 1);
    assert!(::core::mem::align_of::<crate::AddAssignProhibitedFriendConstLhs>() == 1);
    static_assertions::assert_impl_all!(crate::AddAssignProhibitedFriendConstLhs: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AddAssignProhibitedFriendConstLhs: Drop);

    assert!(::core::mem::size_of::<crate::ManyOperators>() == 1);
    assert!(::core::mem::align_of::<crate::ManyOperators>() == 1);
    static_assertions::assert_impl_all!(crate::ManyOperators: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ManyOperators: Drop);
};

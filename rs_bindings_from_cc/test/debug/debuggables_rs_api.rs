// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/debug:debuggables

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "10Debuggable"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Debuggable
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Debuggable {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
unsafe impl ::cxx::ExternType for Debuggable {
    type Id = ::cxx::type_id!("Debuggable");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Debug for Debuggable {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("Debuggable").finish()
    }
}
forward_declare::unsafe_define!(forward_declare::symbol!("Debuggable"), crate::Debuggable);

impl Default for Debuggable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10DebuggableC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub type AliasedDebuggable = crate::Debuggable;

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "6OptOut"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=OptOut
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct OptOut {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
unsafe impl ::cxx::ExternType for OptOut {
    type Id = ::cxx::type_id!("OptOut");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("OptOut"), crate::OptOut);

impl Default for OptOut {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN6OptOutC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[cfi_encoding = "7NonCopy"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NonCopy
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct NonCopy {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
unsafe impl ::cxx::ExternType for NonCopy {
    type Id = ::cxx::type_id!("NonCopy");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Debug for NonCopy {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("NonCopy").finish()
    }
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonCopy"), crate::NonCopy);

impl Default for NonCopy {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN7NonCopyC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for NonCopy {
    #[inline(always)]
    fn from(args: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut __param_0 = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN7NonCopyC1EOS_(&raw mut tmp as *mut _, __param_0);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for NonCopy {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for NonCopy {
    #[inline(always)]
    fn unpin_assign<'__this>(&'__this mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN7NonCopyaSEOS_(self, __param_0);
        }
    }
}

#[cfi_encoding = "16HasMutableFields"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=HasMutableFields
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct HasMutableFields {
    pub i: ::core::cell::Cell<::ffi_11::c_int>,
    pub non_copy: ::core::cell::Cell<crate::NonCopy>,
}
unsafe impl ::cxx::ExternType for HasMutableFields {
    type Id = ::cxx::type_id!("HasMutableFields");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Debug for HasMutableFields {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("HasMutableFields").field("i", &self.i).finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("HasMutableFields"),
    crate::HasMutableFields
);

impl Default for HasMutableFields {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16HasMutableFieldsC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for HasMutableFields {
    #[inline(always)]
    fn from(args: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut __param_0 = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16HasMutableFieldsC1EOS_(
                &raw mut tmp as *mut _,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for HasMutableFields {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for HasMutableFields {
    #[inline(always)]
    fn unpin_assign<'__this>(&'__this mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN16HasMutableFieldsaSEOS_(self, __param_0);
        }
    }
}

// error: class `Templated` could not be bound
//   Class templates are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst9TemplatedILb1EE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Templated < true >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct TemplatedOptIn {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl ::core::fmt::Debug for TemplatedOptIn {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("Templated<true>").finish()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("Templated < true >"),
    crate::TemplatedOptIn
);

impl Default for TemplatedOptIn {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__7aa9ce1a__ZN9TemplatedILb1EEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst9TemplatedILb0EE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Templated < false >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct TemplatedOptOut {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("Templated < false >"),
    crate::TemplatedOptOut
);

impl Default for TemplatedOptOut {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__7aa9ce1a__ZN9TemplatedILb0EEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[cfi_encoding = "4Enum"]
///CRUBIT_ANNOTATE: cpp_type=Enum
pub struct Enum(::ffi_11::c_int);
impl Enum {
    pub const kOne: Enum = Enum(::ffi_11::new_c_int(1));
}
impl From<::ffi_11::c_int> for Enum {
    fn from(value: ::ffi_11::c_int) -> Enum {
        Enum(value)
    }
}
impl From<Enum> for ::ffi_11::c_int {
    fn from(value: Enum) -> ::ffi_11::c_int {
        value.0
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * The callee does not read an incorrect field out of the union.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "5Union"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Union
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub union Union {
    pub i: ::ffi_11::c_int,
    pub d: f64,
}
unsafe impl ::cxx::ExternType for Union {
    type Id = ::cxx::type_id!("Union");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Debug for Union {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("Union").finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(forward_declare::symbol!("Union"), crate::Union);

impl Default for Union {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5UnionC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `pointer`: raw pointer
///   * `named_union`: unsafe struct or union
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "10Exhaustive"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Exhaustive
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Exhaustive {
    pub primitive: ::ffi_11::c_int,
    pub pointer: *const ::ffi_11::c_void,
    pub function: Option<extern "C" fn()>,
    pub an_enum: crate::Enum,
    pub named_union: crate::Union,
    pub debuggable: crate::Debuggable,
    pub aliased_debuggable: crate::AliasedDebuggable,
    pub rust_debuggable: ::existing_rust_debuggable::ExistingRustDebuggable,
}
unsafe impl ::cxx::ExternType for Exhaustive {
    type Id = ::cxx::type_id!("Exhaustive");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Debug for Exhaustive {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter
            .debug_struct("Exhaustive")
            .field("primitive", &self.primitive)
            .field("pointer", &self.pointer)
            .field("function", &self.function)
            .field("an_enum", &self.an_enum)
            .field("named_union", &self.named_union)
            .field("debuggable", &self.debuggable)
            .field("aliased_debuggable", &self.aliased_debuggable)
            .field("rust_debuggable", &self.rust_debuggable)
            .finish()
    }
}
forward_declare::unsafe_define!(forward_declare::symbol!("Exhaustive"), crate::Exhaustive);

impl Default for Exhaustive {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10ExhaustiveC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "12RustKeywords"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=RustKeywords
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct RustKeywords {
    pub r#type: crate::rust_keywords::Type,
    pub r#use: crate::rust_keywords::Use,
    pub r#yield: f64,
}
unsafe impl ::cxx::ExternType for RustKeywords {
    type Id = ::cxx::type_id!("RustKeywords");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Debug for RustKeywords {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter
            .debug_struct("RustKeywords")
            .field("type", &self.r#type)
            .field("use", &self.r#use)
            .field("yield", &self.r#yield)
            .finish()
    }
}
forward_declare::unsafe_define!(forward_declare::symbol!("RustKeywords"), crate::RustKeywords);

impl Default for RustKeywords {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12RustKeywordsC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod rust_keywords {
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
    #[cfi_encoding = "N12RustKeywords4TypeE"]
    ///CRUBIT_ANNOTATE: cpp_type=RustKeywords :: Type
    pub struct Type(::ffi_11::c_int);
    impl Type {
        pub const kCereal: Type = Type(::ffi_11::new_c_int(0));
        pub const kFiber: Type = Type(::ffi_11::new_c_int(1));
    }
    impl From<::ffi_11::c_int> for Type {
        fn from(value: ::ffi_11::c_int) -> Type {
            Type(value)
        }
    }
    impl From<Type> for ::ffi_11::c_int {
        fn from(value: Type) -> ::ffi_11::c_int {
            value.0
        }
    }

    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
    #[cfi_encoding = "N12RustKeywords3UseE"]
    ///CRUBIT_ANNOTATE: cpp_type=RustKeywords :: Use
    pub struct Use(::ffi_11::c_int);
    impl Use {
        pub const kToEat: Use = Use(::ffi_11::new_c_int(0));
        pub const kToWear: Use = Use(::ffi_11::new_c_int(1));
        pub const kToSleep: Use = Use(::ffi_11::new_c_int(2));
    }
    impl From<::ffi_11::c_int> for Use {
        fn from(value: ::ffi_11::c_int) -> Use {
            Use(value)
        }
    }
    impl From<Use> for ::ffi_11::c_int {
        fn from(value: Use) -> ::ffi_11::c_int {
            value.0
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "16HasNonDebuggable"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=HasNonDebuggable
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct HasNonDebuggable {
    pub debuggable: crate::Debuggable,
    pub non_debuggable: crate::OptOut,
}
unsafe impl ::cxx::ExternType for HasNonDebuggable {
    type Id = ::cxx::type_id!("HasNonDebuggable");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Debug for HasNonDebuggable {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter
            .debug_struct("HasNonDebuggable")
            .field("debuggable", &self.debuggable)
            .finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("HasNonDebuggable"),
    crate::HasNonDebuggable
);

impl Default for HasNonDebuggable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16HasNonDebuggableC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * unnamed field: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'HasAnonymousUnion::(anonymous union at ./rs_bindings_from_cc/test/debug/debuggables.h:70:3)': No generated bindings found for ''
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "17HasAnonymousUnion"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=HasAnonymousUnion
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct HasAnonymousUnion {
    pub tag: crate::has_anonymous_union::Tag,
    __padding1: [::core::mem::MaybeUninit<u8>; 4],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'HasAnonymousUnion::(anonymous union at ./rs_bindings_from_cc/test/debug/debuggables.h:70:3)': No generated bindings found for ''
    pub(crate) __unnamed_field1: [::core::mem::MaybeUninit<u8>; 8],
}
/// Cannot infer `Send` because:
/// * field `__unnamed_field1` is represented as opaque bytes
impl !Send for HasAnonymousUnion {}
/// Cannot infer `Sync` because:
/// * field `__unnamed_field1` is represented as opaque bytes
impl !Sync for HasAnonymousUnion {}
unsafe impl ::cxx::ExternType for HasAnonymousUnion {
    type Id = ::cxx::type_id!("HasAnonymousUnion");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Debug for HasAnonymousUnion {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("HasAnonymousUnion").field("tag", &self.tag).finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("HasAnonymousUnion"),
    crate::HasAnonymousUnion
);

impl Default for HasAnonymousUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17HasAnonymousUnionC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod has_anonymous_union {
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
    #[cfi_encoding = "N17HasAnonymousUnion3TagE"]
    ///CRUBIT_ANNOTATE: cpp_type=HasAnonymousUnion :: Tag
    pub struct Tag(::ffi_11::c_int);
    impl Tag {
        pub const kUninit: Tag = Tag(::ffi_11::new_c_int(0));
        pub const kInt: Tag = Tag(::ffi_11::new_c_int(1));
        pub const kDouble: Tag = Tag(::ffi_11::new_c_int(2));
    }
    impl From<::ffi_11::c_int> for Tag {
        fn from(value: ::ffi_11::c_int) -> Tag {
            Tag(value)
        }
    }
    impl From<Tag> for ::ffi_11::c_int {
        fn from(value: Tag) -> ::ffi_11::c_int {
            value.0
        }
    }
}

/// Not Rust-movable: type is polymorphic
#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "8Abstract"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=Abstract
pub struct Abstract {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
/// Cannot infer `Send` because:
/// * non-field data (such as a vtable pointer) is represented as opaque bytes
impl !Send for Abstract {}
/// Cannot infer `Sync` because:
/// * non-field data (such as a vtable pointer) is represented as opaque bytes
impl !Sync for Abstract {}
unsafe impl ::cxx::ExternType for Abstract {
    type Id = ::cxx::type_id!("Abstract");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Abstract"), crate::Abstract);

impl ::ctor::PinnedDrop for Abstract {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN8AbstractD1Ev(self) }
    }
}

unsafe impl ::operator::Delete for crate::Abstract {
    #[inline(always)]
    unsafe fn delete(p: *mut Self) {
        unsafe {
            crate::detail::__crubit_operator_delete__8Abstract___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdebug_3adebuggables(p);
        }
    }
}

/// Not Rust-movable: type is polymorphic
#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "13AbstractOptIn"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=AbstractOptIn
pub struct AbstractOptIn {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
/// Cannot infer `Send` because:
/// * non-field data (such as a vtable pointer) is represented as opaque bytes
impl !Send for AbstractOptIn {}
/// Cannot infer `Sync` because:
/// * non-field data (such as a vtable pointer) is represented as opaque bytes
impl !Sync for AbstractOptIn {}
unsafe impl ::cxx::ExternType for AbstractOptIn {
    type Id = ::cxx::type_id!("AbstractOptIn");
    type Kind = ::cxx::kind::Opaque;
}
impl ::core::fmt::Debug for AbstractOptIn {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("AbstractOptIn").finish()
    }
}
forward_declare::unsafe_define!(forward_declare::symbol!("AbstractOptIn"), crate::AbstractOptIn);

impl ::ctor::PinnedDrop for AbstractOptIn {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN13AbstractOptInD1Ev(self) }
    }
}

unsafe impl ::operator::Delete for crate::AbstractOptIn {
    #[inline(always)]
    unsafe fn delete(p: *mut Self) {
        unsafe {
            crate::detail::__crubit_operator_delete__13AbstractOptIn___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdebug_3adebuggables(p);
        }
    }
}

#[inline(always)]
pub fn abstract_opt_in_instance() -> *const crate::AbstractOptIn {
    unsafe { crate::detail::__rust_thunk___Z24abstract_opt_in_instancev() }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "15ArrayDebuggable"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=ArrayDebuggable
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct ArrayDebuggable {
    pub large: [::ffi_11::c_char; 64],
    pub matrix: [[::ffi_11::c_int; 3]; 2],
}
unsafe impl ::cxx::ExternType for ArrayDebuggable {
    type Id = ::cxx::type_id!("ArrayDebuggable");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Debug for ArrayDebuggable {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter
            .debug_struct("ArrayDebuggable")
            .field("large", &self.large)
            .field("matrix", &self.matrix)
            .finish()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("ArrayDebuggable"),
    crate::ArrayDebuggable
);

impl Default for ArrayDebuggable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15ArrayDebuggableC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `ptrs`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'int * _Nullable[2]': C-style arrays of non-primitive, non-enum types are not yet supported (b/575922365)
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "26MixedOpaqueArrayDebuggable"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=MixedOpaqueArrayDebuggable
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct MixedOpaqueArrayDebuggable {
    pub header: ::ffi_11::c_int,
    pub arr: [::ffi_11::c_int; 2],
    __padding2: [::core::mem::MaybeUninit<u8>; 4],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'int * _Nullable[2]': C-style arrays of non-primitive, non-enum types are not yet supported (b/575922365)
    pub(crate) ptrs: [::core::mem::MaybeUninit<u8>; 16],
}
/// Cannot infer `Send` because:
/// * field `ptrs` is represented as opaque bytes
impl !Send for MixedOpaqueArrayDebuggable {}
/// Cannot infer `Sync` because:
/// * field `ptrs` is represented as opaque bytes
impl !Sync for MixedOpaqueArrayDebuggable {}
unsafe impl ::cxx::ExternType for MixedOpaqueArrayDebuggable {
    type Id = ::cxx::type_id!("MixedOpaqueArrayDebuggable");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Debug for MixedOpaqueArrayDebuggable {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter
            .debug_struct("MixedOpaqueArrayDebuggable")
            .field("header", &self.header)
            .field("arr", &self.arr)
            .finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MixedOpaqueArrayDebuggable"),
    crate::MixedOpaqueArrayDebuggable
);

impl Default for MixedOpaqueArrayDebuggable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN26MixedOpaqueArrayDebuggableC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[cfi_encoding = "17UnscopedDebugEnum"]
///CRUBIT_ANNOTATE: cpp_type=UnscopedDebugEnum
pub struct UnscopedDebugEnum(::ffi_11::c_uint);
impl UnscopedDebugEnum {
    pub const kFirst: UnscopedDebugEnum = UnscopedDebugEnum(::ffi_11::new_c_uint(10));
    pub const kSecond: UnscopedDebugEnum = UnscopedDebugEnum(::ffi_11::new_c_uint(20));
}
impl From<::ffi_11::c_uint> for UnscopedDebugEnum {
    fn from(value: ::ffi_11::c_uint) -> UnscopedDebugEnum {
        UnscopedDebugEnum(value)
    }
}
impl From<UnscopedDebugEnum> for ::ffi_11::c_uint {
    fn from(value: UnscopedDebugEnum) -> ::ffi_11::c_uint {
        value.0
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "19EnumArrayDebuggable"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=EnumArrayDebuggable
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct EnumArrayDebuggable {
    pub unscoped: [crate::UnscopedDebugEnum; 2],
    pub scoped: [crate::Enum; 2],
    pub matrix: [[crate::Enum; 2]; 2],
}
unsafe impl ::cxx::ExternType for EnumArrayDebuggable {
    type Id = ::cxx::type_id!("EnumArrayDebuggable");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Debug for EnumArrayDebuggable {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter
            .debug_struct("EnumArrayDebuggable")
            .field("unscoped", &self.unscoped)
            .field("scoped", &self.scoped)
            .field("matrix", &self.matrix)
            .finish()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("EnumArrayDebuggable"),
    crate::EnumArrayDebuggable
);

impl Default for EnumArrayDebuggable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN19EnumArrayDebuggableC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// Class template instantiations that aren't named by a C++ type alias.
pub mod zz_template_instantiations {
    #[doc(hidden)]
    pub type __CcTemplateInst9TemplatedILb1EE = crate::TemplatedOptIn;

    #[doc(hidden)]
    pub type __CcTemplateInst9TemplatedILb0EE = crate::TemplatedOptOut;
}
mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN10DebuggableC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN6OptOutC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN7NonCopyC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN7NonCopyC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::NonCopy>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN7NonCopyaSEOS_<'__this>(
            __this: &'__this mut crate::NonCopy,
            __param_0: ::ctor::RvalueReference<'_, crate::NonCopy>,
        ) -> &'__this mut crate::NonCopy;
        pub(crate) unsafe fn __rust_thunk___ZN16HasMutableFieldsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16HasMutableFieldsC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::HasMutableFields>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16HasMutableFieldsaSEOS_<'__this>(
            __this: &'__this mut crate::HasMutableFields,
            __param_0: ::ctor::RvalueReference<'_, crate::HasMutableFields>,
        ) -> &'__this mut crate::HasMutableFields;
        pub(crate) unsafe fn __rust_thunk__7aa9ce1a__ZN9TemplatedILb1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__7aa9ce1a__ZN9TemplatedILb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN5UnionC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN10ExhaustiveC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN12RustKeywordsC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN16HasNonDebuggableC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN17HasAnonymousUnionC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN8AbstractD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::Abstract>,
        );
        pub(crate) unsafe fn __crubit_operator_delete__8Abstract___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdebug_3adebuggables(
            ptr: *mut crate::Abstract,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13AbstractOptInD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::AbstractOptIn>,
        );
        pub(crate) unsafe fn __crubit_operator_delete__13AbstractOptIn___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdebug_3adebuggables(
            ptr: *mut crate::AbstractOptIn,
        );
        pub(crate) unsafe fn __rust_thunk___Z24abstract_opt_in_instancev(
        ) -> *const crate::AbstractOptIn;
        pub(crate) unsafe fn __rust_thunk___ZN15ArrayDebuggableC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN26MixedOpaqueArrayDebuggableC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19EnumArrayDebuggableC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Debuggable>() == 1);
    assert!(::core::mem::align_of::<crate::Debuggable>() == 1);
    static_assertions::assert_impl_all!(crate::Debuggable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Debuggable: Drop);

    assert!(::core::mem::size_of::<crate::OptOut>() == 1);
    assert!(::core::mem::align_of::<crate::OptOut>() == 1);
    static_assertions::assert_impl_all!(crate::OptOut: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::OptOut: Drop);

    assert!(::core::mem::size_of::<crate::NonCopy>() == 1);
    assert!(::core::mem::align_of::<crate::NonCopy>() == 1);
    static_assertions::assert_not_impl_any!(crate::NonCopy: Copy,Drop);

    assert!(::core::mem::size_of::<crate::HasMutableFields>() == 8);
    assert!(::core::mem::align_of::<crate::HasMutableFields>() == 4);
    static_assertions::assert_not_impl_any!(crate::HasMutableFields: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::HasMutableFields, i) == 0);
    assert!(::core::mem::offset_of!(crate::HasMutableFields, non_copy) == 4);
    assert!(::core::mem::size_of::<crate::TemplatedOptIn>() == 1);
    assert!(::core::mem::align_of::<crate::TemplatedOptIn>() == 1);
    static_assertions::assert_impl_all!(crate::TemplatedOptIn: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TemplatedOptIn: Drop);

    assert!(::core::mem::size_of::<crate::TemplatedOptOut>() == 1);
    assert!(::core::mem::align_of::<crate::TemplatedOptOut>() == 1);
    static_assertions::assert_impl_all!(crate::TemplatedOptOut: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TemplatedOptOut: Drop);

    assert!(::core::mem::size_of::<crate::Union>() == 8);
    assert!(::core::mem::align_of::<crate::Union>() == 8);
    static_assertions::assert_impl_all!(crate::Union: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Union: Drop);
    assert!(::core::mem::offset_of!(crate::Union, i) == 0);
    assert!(::core::mem::offset_of!(crate::Union, d) == 0);
    static_assertions::assert_impl_all!(::ffi_11::c_int: Copy);
    static_assertions::assert_impl_all!(f64: Copy);
    assert!(::core::mem::size_of::<crate::Exhaustive>() == 48);
    assert!(::core::mem::align_of::<crate::Exhaustive>() == 8);
    static_assertions::assert_impl_all!(crate::Exhaustive: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Exhaustive: Drop);
    assert!(::core::mem::offset_of!(crate::Exhaustive, primitive) == 0);
    assert!(::core::mem::offset_of!(crate::Exhaustive, pointer) == 8);
    assert!(::core::mem::offset_of!(crate::Exhaustive, function) == 16);
    assert!(::core::mem::offset_of!(crate::Exhaustive, an_enum) == 24);
    assert!(::core::mem::offset_of!(crate::Exhaustive, named_union) == 32);
    assert!(::core::mem::offset_of!(crate::Exhaustive, debuggable) == 40);
    assert!(::core::mem::offset_of!(crate::Exhaustive, aliased_debuggable) == 41);
    assert!(::core::mem::offset_of!(crate::Exhaustive, rust_debuggable) == 44);
    assert!(::core::mem::size_of::<crate::RustKeywords>() == 16);
    assert!(::core::mem::align_of::<crate::RustKeywords>() == 8);
    static_assertions::assert_impl_all!(crate::RustKeywords: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::RustKeywords: Drop);
    assert!(::core::mem::offset_of!(crate::RustKeywords, r#type) == 0);
    assert!(::core::mem::offset_of!(crate::RustKeywords, r#use) == 4);
    assert!(::core::mem::offset_of!(crate::RustKeywords, r#yield) == 8);
    assert!(::core::mem::size_of::<crate::HasNonDebuggable>() == 2);
    assert!(::core::mem::align_of::<crate::HasNonDebuggable>() == 1);
    static_assertions::assert_impl_all!(crate::HasNonDebuggable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::HasNonDebuggable: Drop);
    assert!(::core::mem::offset_of!(crate::HasNonDebuggable, debuggable) == 0);
    assert!(::core::mem::offset_of!(crate::HasNonDebuggable, non_debuggable) == 1);
    assert!(::core::mem::size_of::<crate::HasAnonymousUnion>() == 16);
    assert!(::core::mem::align_of::<crate::HasAnonymousUnion>() == 8);
    static_assertions::assert_impl_all!(crate::HasAnonymousUnion: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::HasAnonymousUnion: Drop);
    assert!(::core::mem::offset_of!(crate::HasAnonymousUnion, tag) == 0);
    assert!(::core::mem::offset_of!(crate::HasAnonymousUnion, __unnamed_field1) == 8);
    assert!(::core::mem::size_of::<crate::Abstract>() == 8);
    assert!(::core::mem::align_of::<crate::Abstract>() == 8);
    static_assertions::assert_impl_all!(crate::Abstract: Drop);
    static_assertions::assert_not_impl_any!(crate::Abstract: Copy);

    assert!(::core::mem::size_of::<crate::AbstractOptIn>() == 8);
    assert!(::core::mem::align_of::<crate::AbstractOptIn>() == 8);
    static_assertions::assert_impl_all!(crate::AbstractOptIn: Drop);
    static_assertions::assert_not_impl_any!(crate::AbstractOptIn: Copy);

    assert!(::core::mem::size_of::<crate::ArrayDebuggable>() == 88);
    assert!(::core::mem::align_of::<crate::ArrayDebuggable>() == 4);
    static_assertions::assert_impl_all!(crate::ArrayDebuggable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ArrayDebuggable: Drop);
    assert!(::core::mem::offset_of!(crate::ArrayDebuggable, large) == 0);
    assert!(::core::mem::offset_of!(crate::ArrayDebuggable, matrix) == 64);
    assert!(::core::mem::size_of::<crate::MixedOpaqueArrayDebuggable>() == 32);
    assert!(::core::mem::align_of::<crate::MixedOpaqueArrayDebuggable>() == 8);
    static_assertions::assert_impl_all!(crate::MixedOpaqueArrayDebuggable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::MixedOpaqueArrayDebuggable: Drop);
    assert!(::core::mem::offset_of!(crate::MixedOpaqueArrayDebuggable, header) == 0);
    assert!(::core::mem::offset_of!(crate::MixedOpaqueArrayDebuggable, arr) == 4);
    assert!(::core::mem::offset_of!(crate::MixedOpaqueArrayDebuggable, ptrs) == 16);
    assert!(::core::mem::size_of::<crate::EnumArrayDebuggable>() == 32);
    assert!(::core::mem::align_of::<crate::EnumArrayDebuggable>() == 4);
    static_assertions::assert_impl_all!(crate::EnumArrayDebuggable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::EnumArrayDebuggable: Drop);
    assert!(::core::mem::offset_of!(crate::EnumArrayDebuggable, unscoped) == 0);
    assert!(::core::mem::offset_of!(crate::EnumArrayDebuggable, scoped) == 8);
    assert!(::core::mem::offset_of!(crate::EnumArrayDebuggable, matrix) == 16);
};

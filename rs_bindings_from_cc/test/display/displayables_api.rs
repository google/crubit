// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/display:displayables
// Features: fmt, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=15
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=CanAbslStringify
pub struct CanAbslStringify {
    pub value: ::string_view::absl::string_view,
}
impl !Send for CanAbslStringify {}
impl !Sync for CanAbslStringify {}
unsafe impl ::cxx::ExternType for CanAbslStringify {
    type Id = ::cxx::type_id!("CanAbslStringify");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Display for CanAbslStringify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut f = ::lossy_formatter::LossyFormatter::new(f);
        if unsafe {
            crate::detail::__crubit_fmt__16CanAbslStringify___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(self,&mut f)
        } {
            ::core::result::Result::Ok(())
        } else {
            ::core::result::Result::Err(::core::fmt::Error)
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=15
impl Default for CanAbslStringify {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16CanAbslStringifyC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=24
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=CanAbslStringifyByFill
pub struct CanAbslStringifyByFill {
    pub count: usize,
    pub ch: ::ffi_11::c_char,
}
impl !Send for CanAbslStringifyByFill {}
impl !Sync for CanAbslStringifyByFill {}
unsafe impl ::cxx::ExternType for CanAbslStringifyByFill {
    type Id = ::cxx::type_id!("CanAbslStringifyByFill");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Display for CanAbslStringifyByFill {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut f = ::lossy_formatter::LossyFormatter::new(f);
        if unsafe {
            crate::detail::__crubit_fmt__22CanAbslStringifyByFill___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(self,&mut f)
        } {
            ::core::result::Result::Ok(())
        } else {
            ::core::result::Result::Err(::core::fmt::Error)
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=24
impl Default for CanAbslStringifyByFill {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN22CanAbslStringifyByFillC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=34
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=CanAbslStringifyByFormat
pub struct CanAbslStringifyByFormat {
    pub value: ::string_view::absl::string_view,
}
impl !Send for CanAbslStringifyByFormat {}
impl !Sync for CanAbslStringifyByFormat {}
unsafe impl ::cxx::ExternType for CanAbslStringifyByFormat {
    type Id = ::cxx::type_id!("CanAbslStringifyByFormat");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Display for CanAbslStringifyByFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut f = ::lossy_formatter::LossyFormatter::new(f);
        if unsafe {
            crate::detail::__crubit_fmt__24CanAbslStringifyByFormat___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(self,&mut f)
        } {
            ::core::result::Result::Ok(())
        } else {
            ::core::result::Result::Err(::core::fmt::Error)
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=34
impl Default for CanAbslStringifyByFormat {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN24CanAbslStringifyByFormatC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=43
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=CanOstream
pub struct CanOstream {
    pub value: ::string_view::absl::string_view,
}
impl !Send for CanOstream {}
impl !Sync for CanOstream {}
unsafe impl ::cxx::ExternType for CanOstream {
    type Id = ::cxx::type_id!("CanOstream");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Display for CanOstream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut f = ::lossy_formatter::LossyFormatter::new(f);
        if unsafe {
            crate::detail::__crubit_fmt__10CanOstream___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(self,&mut f)
        } {
            ::core::result::Result::Ok(())
        } else {
            ::core::result::Result::Err(::core::fmt::Error)
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=43
impl Default for CanOstream {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10CanOstreamC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=46
// Error while generating bindings for function 'operator<<':
// Can't generate bindings for operator<<, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for operator<< (return type: error: Can't generate bindings for std::basic_ostream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_ostream<char, std::char_traits<char>> (crate::__CcTemplateInstNSt3__u13basic_ostreamIcNS_11char_traitsIcEEEE is a template instantiation))
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for operator<< (the type of out (parameter #0): error: Can't generate bindings for std::basic_ostream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_ostream<char, std::char_traits<char>> (crate::__CcTemplateInstNSt3__u13basic_ostreamIcNS_11char_traitsIcEEEE is a template instantiation))

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=51
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=CanAbslStringifyAndOstream
pub struct CanAbslStringifyAndOstream {
    pub stringify: ::string_view::absl::string_view,
    pub ostream: ::string_view::absl::string_view,
}
impl !Send for CanAbslStringifyAndOstream {}
impl !Sync for CanAbslStringifyAndOstream {}
unsafe impl ::cxx::ExternType for CanAbslStringifyAndOstream {
    type Id = ::cxx::type_id!("CanAbslStringifyAndOstream");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Display for CanAbslStringifyAndOstream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut f = ::lossy_formatter::LossyFormatter::new(f);
        if unsafe {
            crate::detail::__crubit_fmt__26CanAbslStringifyAndOstream___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(self,&mut f)
        } {
            ::core::result::Result::Ok(())
        } else {
            ::core::result::Result::Err(::core::fmt::Error)
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=51
impl Default for CanAbslStringifyAndOstream {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN26CanAbslStringifyAndOstreamC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=61
// Error while generating bindings for function 'operator<<':
// Can't generate bindings for operator<<, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for operator<< (return type: error: Can't generate bindings for std::basic_ostream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_ostream<char, std::char_traits<char>> (crate::__CcTemplateInstNSt3__u13basic_ostreamIcNS_11char_traitsIcEEEE is a template instantiation))
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for operator<< (the type of out (parameter #0): error: Can't generate bindings for std::basic_ostream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_ostream<char, std::char_traits<char>> (crate::__CcTemplateInstNSt3__u13basic_ostreamIcNS_11char_traitsIcEEEE is a template instantiation))

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=DisplayableEnum
pub struct DisplayableEnum(::ffi_11::c_int);
impl DisplayableEnum {
    pub const kKnown: DisplayableEnum = DisplayableEnum(::ffi_11::new_c_int(1));
}
impl From<::ffi_11::c_int> for DisplayableEnum {
    fn from(value: ::ffi_11::c_int) -> DisplayableEnum {
        DisplayableEnum(value)
    }
}
impl From<DisplayableEnum> for ::ffi_11::c_int {
    fn from(value: DisplayableEnum) -> ::ffi_11::c_int {
        value.0
    }
}
impl ::core::fmt::Display for DisplayableEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut f = ::lossy_formatter::LossyFormatter::new(f);
        if unsafe {
            crate::detail::__crubit_fmt__DisplayableEnum___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(self,&mut f)
        } {
            ::core::result::Result::Ok(())
        } else {
            ::core::result::Result::Err(::core::fmt::Error)
        }
    }
}

// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=70
// Error while generating bindings for function 'AbslStringify':
// Function templates are not supported yet

// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=82
// Error while generating bindings for class 'Templated':
// Class templates are not supported yet

// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=86
// Error while generating bindings for function 'AbslStringify':
// Function templates are not supported yet

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=91
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NotDisplayable
pub struct NotDisplayable {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NotDisplayable {}
impl !Sync for NotDisplayable {}
unsafe impl ::cxx::ExternType for NotDisplayable {
    type Id = ::cxx::type_id!("NotDisplayable");
    type Kind = ::cxx::kind::Trivial;
}

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=91
impl Default for NotDisplayable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14NotDisplayableC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=92
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=TemplatedStringView
pub struct TemplatedStringView {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 16],
}
impl !Send for TemplatedStringView {}
impl !Sync for TemplatedStringView {}
unsafe impl ::cxx::ExternType for TemplatedStringView {
    type Id = ::cxx::type_id!("TemplatedStringView");
    type Kind = ::cxx::kind::Trivial;
}
impl ::core::fmt::Display for TemplatedStringView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut f = ::lossy_formatter::LossyFormatter::new(f);
        if unsafe {
            crate::detail::__crubit_fmt__19TemplatedStringView___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(self,&mut f)
        } {
            ::core::result::Result::Ok(())
        } else {
            ::core::result::Result::Err(::core::fmt::Error)
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=93
impl From<::string_view::absl::string_view> for TemplatedStringView {
    #[inline(always)]
    fn from(args: ::string_view::absl::string_view) -> Self {
        let mut v = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN19TemplatedStringViewC1ENSt3__u17basic_string_viewIcNS0_11char_traitsIcEEEE(&raw mut tmp as*mut _,&mut v);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::string_view::absl::string_view> for TemplatedStringView {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::string_view::absl::string_view) -> Self::CtorType {
        <Self as From<::string_view::absl::string_view>>::from(args)
    }
}

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=95
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TemplatedNotDisplayable
pub struct TemplatedNotDisplayable {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for TemplatedNotDisplayable {}
impl !Sync for TemplatedNotDisplayable {}
unsafe impl ::cxx::ExternType for TemplatedNotDisplayable {
    type Id = ::cxx::type_id!("TemplatedNotDisplayable");
    type Kind = ::cxx::kind::Trivial;
}

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=96
impl Default for TemplatedNotDisplayable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23TemplatedNotDisplayableC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=99
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DisplayInRust
pub struct DisplayInRust {
    pub cc_value: ::string_view::absl::string_view,
    pub rust_value: ::string_view::absl::string_view,
}
impl !Send for DisplayInRust {}
impl !Sync for DisplayInRust {}
unsafe impl ::cxx::ExternType for DisplayInRust {
    type Id = ::cxx::type_id!("DisplayInRust");
    type Kind = ::cxx::kind::Trivial;
}

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=99
impl Default for DisplayInRust {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13DisplayInRustC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::integral_constant<bool, false> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::integral_constant<bool, true> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator.h;l=62
// Error while generating bindings for class 'std::allocator<char32_t>':
// Can't generate bindings for std::allocator<char32_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::allocator<char32_t> (crate::__CcTemplateInstNSt3__u9allocatorIDiEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator.h;l=62
// Error while generating bindings for class 'std::allocator<char16_t>':
// Can't generate bindings for std::allocator<char16_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::allocator<char16_t> (crate::__CcTemplateInstNSt3__u9allocatorIDsEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator.h;l=62
// Error while generating bindings for class 'std::allocator<char>':
// Can't generate bindings for std::allocator<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::allocator<char> (crate::__CcTemplateInstNSt3__u9allocatorIcEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator.h;l=62
// Error while generating bindings for class 'std::allocator<wchar_t>':
// Can't generate bindings for std::allocator<wchar_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::allocator<wchar_t> (crate::__CcTemplateInstNSt3__u9allocatorIwEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory_resource/polymorphic_allocator.h;l=45
// Error while generating bindings for class 'std::pmr::polymorphic_allocator<char32_t>':
// Can't generate bindings for std::pmr::polymorphic_allocator<char32_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::pmr::polymorphic_allocator<char32_t> (crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory_resource/polymorphic_allocator.h;l=45
// Error while generating bindings for class 'std::pmr::polymorphic_allocator<char16_t>':
// Can't generate bindings for std::pmr::polymorphic_allocator<char16_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::pmr::polymorphic_allocator<char16_t> (crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory_resource/polymorphic_allocator.h;l=45
// Error while generating bindings for class 'std::pmr::polymorphic_allocator<char>':
// Can't generate bindings for std::pmr::polymorphic_allocator<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::pmr::polymorphic_allocator<char> (crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/string;l=734
// Error while generating bindings for class 'std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>>':
// Can't generate bindings for std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>> (crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/string;l=734
// Error while generating bindings for class 'std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>>':
// Can't generate bindings for std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>> (crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/string;l=734
// Error while generating bindings for class 'std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>>':
// Can't generate bindings for std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>> (crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/string;l=734
// Error while generating bindings for class 'std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>>':
// Can't generate bindings for std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>> (crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'char8_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'char8_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// Error while generating bindings for struct 'std::__type_identity<std::chrono::duration<int, std::ratio<2629746L, 1L>>>':
// Can't generate bindings for std::__type_identity<std::chrono::duration<int, std::ratio<2629746L, 1L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__type_identity<std::chrono::duration<int, std::ratio<2629746L, 1L>>> (crate::__CcTemplateInstNSt3__u15__type_identityINS_6chrono8durationIiNS_5ratioILl2629746ELl1EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// Error while generating bindings for struct 'std::__type_identity<std::chrono::duration<int, std::ratio<31556952L, 1L>>>':
// Can't generate bindings for std::__type_identity<std::chrono::duration<int, std::ratio<31556952L, 1L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__type_identity<std::chrono::duration<int, std::ratio<31556952L, 1L>>> (crate::__CcTemplateInstNSt3__u15__type_identityINS_6chrono8durationIiNS_5ratioILl31556952ELl1EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// Error while generating bindings for struct 'std::__type_identity<std::chrono::duration<int, std::ratio<604800L, 1L>>>':
// Can't generate bindings for std::__type_identity<std::chrono::duration<int, std::ratio<604800L, 1L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__type_identity<std::chrono::duration<int, std::ratio<604800L, 1L>>> (crate::__CcTemplateInstNSt3__u15__type_identityINS_6chrono8durationIiNS_5ratioILl604800ELl1EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// Error while generating bindings for struct 'std::__type_identity<std::chrono::duration<int, std::ratio<86400L, 1L>>>':
// Can't generate bindings for std::__type_identity<std::chrono::duration<int, std::ratio<86400L, 1L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__type_identity<std::chrono::duration<int, std::ratio<86400L, 1L>>> (crate::__CcTemplateInstNSt3__u15__type_identityINS_6chrono8durationIiNS_5ratioILl86400ELl1EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// Error while generating bindings for struct 'std::__type_identity<std::chrono::duration<long, std::ratio<1L, 1000000000000000L>>>':
// Can't generate bindings for std::__type_identity<std::chrono::duration<long, std::ratio<1L, 1000000000000000L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__type_identity<std::chrono::duration<long, std::ratio<1L, 1000000000000000L>>> (crate::__CcTemplateInstNSt3__u15__type_identityINS_6chrono8durationIlNS_5ratioILl1ELl1000000000000000EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// Error while generating bindings for struct 'std::__type_identity<std::chrono::duration<long, std::ratio<1L, 1L>>>':
// Can't generate bindings for std::__type_identity<std::chrono::duration<long, std::ratio<1L, 1L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__type_identity<std::chrono::duration<long, std::ratio<1L, 1L>>> (crate::__CcTemplateInstNSt3__u15__type_identityINS_6chrono8durationIlNS_5ratioILl1ELl1EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// Error while generating bindings for struct 'std::__type_identity<std::chrono::duration<long, std::ratio<3600L, 1L>>>':
// Can't generate bindings for std::__type_identity<std::chrono::duration<long, std::ratio<3600L, 1L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__type_identity<std::chrono::duration<long, std::ratio<3600L, 1L>>> (crate::__CcTemplateInstNSt3__u15__type_identityINS_6chrono8durationIlNS_5ratioILl3600ELl1EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// Error while generating bindings for struct 'std::__type_identity<std::chrono::duration<long, std::ratio<60L, 1L>>>':
// Can't generate bindings for std::__type_identity<std::chrono::duration<long, std::ratio<60L, 1L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__type_identity<std::chrono::duration<long, std::ratio<60L, 1L>>> (crate::__CcTemplateInstNSt3__u15__type_identityINS_6chrono8durationIlNS_5ratioILl60ELl1EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// Error while generating bindings for struct 'std::__type_identity<std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>>':
// Can't generate bindings for std::__type_identity<std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__type_identity<std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>> (crate::__CcTemplateInstNSt3__u15__type_identityINS_6chrono8durationInNS_5ratioILl1ELl1000000000EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// Error while generating bindings for struct 'std::__type_identity<std::chrono::duration<long long, std::ratio<1L, 1000000000L>>>':
// Can't generate bindings for std::__type_identity<std::chrono::duration<long long, std::ratio<1L, 1000000000L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__type_identity<std::chrono::duration<long long, std::ratio<1L, 1000000000L>>> (crate::__CcTemplateInstNSt3__u15__type_identityINS_6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// Error while generating bindings for struct 'std::__type_identity<std::chrono::duration<long long, std::ratio<1L, 1000000L>>>':
// Can't generate bindings for std::__type_identity<std::chrono::duration<long long, std::ratio<1L, 1000000L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__type_identity<std::chrono::duration<long long, std::ratio<1L, 1000000L>>> (crate::__CcTemplateInstNSt3__u15__type_identityINS_6chrono8durationIxNS_5ratioILl1ELl1000000EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// Error while generating bindings for struct 'std::__type_identity<std::chrono::duration<long long, std::ratio<1L, 1000L>>>':
// Can't generate bindings for std::__type_identity<std::chrono::duration<long long, std::ratio<1L, 1000L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__type_identity<std::chrono::duration<long long, std::ratio<1L, 1000L>>> (crate::__CcTemplateInstNSt3__u15__type_identityINS_6chrono8durationIxNS_5ratioILl1ELl1000EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// Error while generating bindings for struct 'std::__type_identity<std::chrono::duration<long long, std::ratio<1L, 1L>>>':
// Can't generate bindings for std::__type_identity<std::chrono::duration<long long, std::ratio<1L, 1L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__type_identity<std::chrono::duration<long long, std::ratio<1L, 1L>>> (crate::__CcTemplateInstNSt3__u15__type_identityINS_6chrono8durationIxNS_5ratioILl1ELl1EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// Error while generating bindings for struct 'std::iterator_traits<char32_t *>':
// Can't generate bindings for std::iterator_traits<char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::iterator_traits<char32_t *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPDiEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// Error while generating bindings for struct 'std::iterator_traits<char16_t *>':
// Can't generate bindings for std::iterator_traits<char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::iterator_traits<char16_t *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPDsEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// Error while generating bindings for struct 'std::iterator_traits<const char32_t *>':
// Can't generate bindings for std::iterator_traits<const char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::iterator_traits<const char32_t *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPKDiEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// Error while generating bindings for struct 'std::iterator_traits<const char16_t *>':
// Can't generate bindings for std::iterator_traits<const char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::iterator_traits<const char16_t *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPKDsEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// Error while generating bindings for struct 'std::iterator_traits<const char *>':
// Can't generate bindings for std::iterator_traits<const char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::iterator_traits<const char *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPKcEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// Error while generating bindings for struct 'std::iterator_traits<char *>':
// Can't generate bindings for std::iterator_traits<char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::iterator_traits<char *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPcEE is a template instantiation)

// Error while generating bindings for class 'std::basic_filebuf<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_filebuf<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_filebuf<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_filebuf<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_filebuf<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_filebuf<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_ifstream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_ifstream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_ifstream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ifstream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_ifstream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_ifstream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_ofstream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_ofstream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_ofstream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ofstream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_ofstream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_ofstream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_fstream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_fstream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_fstream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_fstream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_fstream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_fstream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Generated from: nowhere/llvm/src/libcxx/include/istream;l=1177
// Error while generating bindings for class 'std::basic_iostream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_iostream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_iostream<wchar_t, std::char_traits<wchar_t>> (crate::__CcTemplateInstNSt3__u14basic_iostreamIwNS_11char_traitsIwEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/sstream;l=345
// Error while generating bindings for class 'std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>':
// Can't generate bindings for std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>> (crate::__CcTemplateInstNSt3__u15basic_stringbufIwNS_11char_traitsIwEENS_9allocatorIwEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/sstream;l=867
// Error while generating bindings for class 'std::basic_istringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>':
// Can't generate bindings for std::basic_istringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_istringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>> (crate::__CcTemplateInstNSt3__u19basic_istringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/sstream;l=1005
// Error while generating bindings for class 'std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>':
// Can't generate bindings for std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>> (crate::__CcTemplateInstNSt3__u19basic_ostringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/sstream;l=1145
// Error while generating bindings for class 'std::basic_stringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>':
// Can't generate bindings for std::basic_stringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_stringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>> (crate::__CcTemplateInstNSt3__u18basic_stringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__ios/fpos.h;l=23
// Error while generating bindings for class 'std::fpos<__mbstate_t>':
// Can't generate bindings for std::fpos<__mbstate_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::fpos<__mbstate_t> (crate::__CcTemplateInstNSt3__u4fposI11__mbstate_tEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/pointer_traits.h;l=110
// Error while generating bindings for struct 'std::pointer_traits<char32_t *>':
// Can't generate bindings for std::pointer_traits<char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::pointer_traits<char32_t *> (crate::__CcTemplateInstNSt3__u14pointer_traitsIPDiEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/pointer_traits.h;l=110
// Error while generating bindings for struct 'std::pointer_traits<char16_t *>':
// Can't generate bindings for std::pointer_traits<char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::pointer_traits<char16_t *> (crate::__CcTemplateInstNSt3__u14pointer_traitsIPDsEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/pointer_traits.h;l=110
// Error while generating bindings for struct 'std::pointer_traits<char *>':
// Can't generate bindings for std::pointer_traits<char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::pointer_traits<char *> (crate::__CcTemplateInstNSt3__u14pointer_traitsIPcEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/initializer_list;l=62
// Error while generating bindings for class 'std::initializer_list<char32_t>':
// Can't generate bindings for std::initializer_list<char32_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::initializer_list<char32_t> (crate::__CcTemplateInstSt16initializer_listIDiE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/initializer_list;l=62
// Error while generating bindings for class 'std::initializer_list<char16_t>':
// Can't generate bindings for std::initializer_list<char16_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::initializer_list<char16_t> (crate::__CcTemplateInstSt16initializer_listIDsE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/initializer_list;l=62
// Error while generating bindings for class 'std::initializer_list<char>':
// Can't generate bindings for std::initializer_list<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::initializer_list<char> (crate::__CcTemplateInstSt16initializer_listIcE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=81
// Error while generating bindings for struct 'std::char_traits<char>':
// Can't generate bindings for std::char_traits<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::char_traits<char> (crate::__CcTemplateInstNSt3__u11char_traitsIcEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=180
// Error while generating bindings for struct 'std::__char_traits_base<char32_t, unsigned int, 4294967295U>':
// Can't generate bindings for std::__char_traits_base<char32_t, unsigned int, 4294967295U>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__char_traits_base<char32_t, unsigned int, 4294967295U> (crate::__CcTemplateInstNSt3__u18__char_traits_baseIDijLj4294967295EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=180
// Error while generating bindings for struct 'std::__char_traits_base<char16_t, unsigned short, (unsigned short)65535>':
// Can't generate bindings for std::__char_traits_base<char16_t, unsigned short, (unsigned short)65535>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__char_traits_base<char16_t, unsigned short, (unsigned short)65535> (crate::__CcTemplateInstNSt3__u18__char_traits_baseIDstLt65535EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=180
// Error while generating bindings for struct 'std::__char_traits_base<char8_t, unsigned int, 4294967295U>':
// Can't generate bindings for std::__char_traits_base<char8_t, unsigned int, 4294967295U>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__char_traits_base<char8_t, unsigned int, 4294967295U> (crate::__CcTemplateInstNSt3__u18__char_traits_baseIDujLj4294967295EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=180
// Error while generating bindings for struct 'std::__char_traits_base<wchar_t, unsigned int, 4294967295U>':
// Can't generate bindings for std::__char_traits_base<wchar_t, unsigned int, 4294967295U>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__char_traits_base<wchar_t, unsigned int, 4294967295U> (crate::__CcTemplateInstNSt3__u18__char_traits_baseIwjLj4294967295EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=247
// Error while generating bindings for struct 'std::char_traits<wchar_t>':
// Can't generate bindings for std::char_traits<wchar_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::char_traits<wchar_t> (crate::__CcTemplateInstNSt3__u11char_traitsIwEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=270
// Error while generating bindings for struct 'std::char_traits<char8_t>':
// Can't generate bindings for std::char_traits<char8_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::char_traits<char8_t> (crate::__CcTemplateInstNSt3__u11char_traitsIDuEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=289
// Error while generating bindings for struct 'std::char_traits<char16_t>':
// Can't generate bindings for std::char_traits<char16_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::char_traits<char16_t> (crate::__CcTemplateInstNSt3__u11char_traitsIDsEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=324
// Error while generating bindings for struct 'std::char_traits<char32_t>':
// Can't generate bindings for std::char_traits<char32_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::char_traits<char32_t> (crate::__CcTemplateInstNSt3__u11char_traitsIDiEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<char32_t *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<char32_t *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<char32_t *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<char16_t *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<char16_t *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<char16_t *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<const char32_t *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<const char32_t *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<const char32_t *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<const char16_t *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<const char16_t *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<const char16_t *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<const char *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<const char *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<const char *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<char *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<char *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<char *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<const char32_t *>':
// Can't generate bindings for std::reverse_iterator<const char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::reverse_iterator<const char32_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<const char16_t *>':
// Can't generate bindings for std::reverse_iterator<const char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::reverse_iterator<const char16_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<const char8_t *>':
// Can't generate bindings for std::reverse_iterator<const char8_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::reverse_iterator<const char8_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<const void *const *>':
// Can't generate bindings for std::reverse_iterator<const void *const *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::reverse_iterator<const void *const *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKPKvEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<const char *>':
// Can't generate bindings for std::reverse_iterator<const char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::reverse_iterator<const char *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// Error while generating bindings for class 'std::reverse_iterator<const wchar_t *>':
// Can't generate bindings for std::reverse_iterator<const wchar_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::reverse_iterator<const wchar_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// Error while generating bindings for class 'std::__wrap_iter<char32_t *>':
// Can't generate bindings for std::__wrap_iter<char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__wrap_iter<char32_t *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// Error while generating bindings for class 'std::__wrap_iter<char16_t *>':
// Can't generate bindings for std::__wrap_iter<char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__wrap_iter<char16_t *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// Error while generating bindings for class 'std::__wrap_iter<const char32_t *>':
// Can't generate bindings for std::__wrap_iter<const char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__wrap_iter<const char32_t *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// Error while generating bindings for class 'std::__wrap_iter<const char16_t *>':
// Can't generate bindings for std::__wrap_iter<const char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__wrap_iter<const char16_t *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// Error while generating bindings for class 'std::__wrap_iter<const char *>':
// Can't generate bindings for std::__wrap_iter<const char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__wrap_iter<const char *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// Error while generating bindings for class 'std::__wrap_iter<char *>':
// Can't generate bindings for std::__wrap_iter<char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__wrap_iter<char *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=97
// Error while generating bindings for struct 'std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char32_t>, char32_t *, void>':
// Can't generate bindings for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char32_t>, char32_t *, void>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char32_t>, char32_t *, void> (crate::__CcTemplateInstNSt3__u30__alloc_traits_difference_typeINS_3pmr21polymorphic_allocatorIDiEEPDivEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=97
// Error while generating bindings for struct 'std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char16_t>, char16_t *, void>':
// Can't generate bindings for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char16_t>, char16_t *, void>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char16_t>, char16_t *, void> (crate::__CcTemplateInstNSt3__u30__alloc_traits_difference_typeINS_3pmr21polymorphic_allocatorIDsEEPDsvEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=97
// Error while generating bindings for struct 'std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char>, char *, void>':
// Can't generate bindings for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char>, char *, void>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char>, char *, void> (crate::__CcTemplateInstNSt3__u30__alloc_traits_difference_typeINS_3pmr21polymorphic_allocatorIcEEPcvEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=233
// Error while generating bindings for struct 'std::__allocator_traits_base<std::pmr::polymorphic_allocator<char32_t>>':
// Can't generate bindings for std::__allocator_traits_base<std::pmr::polymorphic_allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__allocator_traits_base<std::pmr::polymorphic_allocator<char32_t>> (crate::__CcTemplateInstNSt3__u23__allocator_traits_baseINS_3pmr21polymorphic_allocatorIDiEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=233
// Error while generating bindings for struct 'std::__allocator_traits_base<std::pmr::polymorphic_allocator<char16_t>>':
// Can't generate bindings for std::__allocator_traits_base<std::pmr::polymorphic_allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__allocator_traits_base<std::pmr::polymorphic_allocator<char16_t>> (crate::__CcTemplateInstNSt3__u23__allocator_traits_baseINS_3pmr21polymorphic_allocatorIDsEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=233
// Error while generating bindings for struct 'std::__allocator_traits_base<std::pmr::polymorphic_allocator<char>>':
// Can't generate bindings for std::__allocator_traits_base<std::pmr::polymorphic_allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__allocator_traits_base<std::pmr::polymorphic_allocator<char>> (crate::__CcTemplateInstNSt3__u23__allocator_traits_baseINS_3pmr21polymorphic_allocatorIcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=265
// Error while generating bindings for struct 'std::__allocator_traits_base<std::allocator<char32_t>>':
// Can't generate bindings for std::__allocator_traits_base<std::allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__allocator_traits_base<std::allocator<char32_t>> (crate::__CcTemplateInstNSt3__u23__allocator_traits_baseINS_9allocatorIDiEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=265
// Error while generating bindings for struct 'std::__allocator_traits_base<std::allocator<char16_t>>':
// Can't generate bindings for std::__allocator_traits_base<std::allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__allocator_traits_base<std::allocator<char16_t>> (crate::__CcTemplateInstNSt3__u23__allocator_traits_baseINS_9allocatorIDsEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=265
// Error while generating bindings for struct 'std::__allocator_traits_base<std::allocator<char>>':
// Can't generate bindings for std::__allocator_traits_base<std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__allocator_traits_base<std::allocator<char>> (crate::__CcTemplateInstNSt3__u23__allocator_traits_baseINS_9allocatorIcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// Error while generating bindings for struct 'std::allocator_traits<std::pmr::polymorphic_allocator<char32_t>>':
// Can't generate bindings for std::allocator_traits<std::pmr::polymorphic_allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::allocator_traits<std::pmr::polymorphic_allocator<char32_t>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_3pmr21polymorphic_allocatorIDiEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// Error while generating bindings for struct 'std::allocator_traits<std::pmr::polymorphic_allocator<char16_t>>':
// Can't generate bindings for std::allocator_traits<std::pmr::polymorphic_allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::allocator_traits<std::pmr::polymorphic_allocator<char16_t>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_3pmr21polymorphic_allocatorIDsEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// Error while generating bindings for struct 'std::allocator_traits<std::pmr::polymorphic_allocator<char>>':
// Can't generate bindings for std::allocator_traits<std::pmr::polymorphic_allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::allocator_traits<std::pmr::polymorphic_allocator<char>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_3pmr21polymorphic_allocatorIcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// Error while generating bindings for struct 'std::allocator_traits<std::allocator<char32_t>>':
// Can't generate bindings for std::allocator_traits<std::allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::allocator_traits<std::allocator<char32_t>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_9allocatorIDiEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// Error while generating bindings for struct 'std::allocator_traits<std::allocator<char16_t>>':
// Can't generate bindings for std::allocator_traits<std::allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::allocator_traits<std::allocator<char16_t>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_9allocatorIDsEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// Error while generating bindings for struct 'std::allocator_traits<std::allocator<char>>':
// Can't generate bindings for std::allocator_traits<std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::allocator_traits<std::allocator<char>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_9allocatorIcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocate_at_least.h;l=23
// Error while generating bindings for struct 'std::__allocation_result<char32_t *, unsigned long>':
// Can't generate bindings for std::__allocation_result<char32_t *, unsigned long>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__allocation_result<char32_t *, unsigned long> (crate::__CcTemplateInstNSt3__u19__allocation_resultIPDimEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocate_at_least.h;l=23
// Error while generating bindings for struct 'std::__allocation_result<char16_t *, unsigned long>':
// Can't generate bindings for std::__allocation_result<char16_t *, unsigned long>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__allocation_result<char16_t *, unsigned long> (crate::__CcTemplateInstNSt3__u19__allocation_resultIPDsmEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocate_at_least.h;l=23
// Error while generating bindings for struct 'std::__allocation_result<char *, unsigned long>':
// Can't generate bindings for std::__allocation_result<char *, unsigned long>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__allocation_result<char *, unsigned long> (crate::__CcTemplateInstNSt3__u19__allocation_resultIPcmEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
// Error while generating bindings for class 'std::basic_string_view<char32_t, std::char_traits<char32_t>>':
// Can't generate bindings for std::basic_string_view<char32_t, std::char_traits<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_string_view<char32_t, std::char_traits<char32_t>> (crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
// Error while generating bindings for class 'std::basic_string_view<char16_t, std::char_traits<char16_t>>':
// Can't generate bindings for std::basic_string_view<char16_t, std::char_traits<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_string_view<char16_t, std::char_traits<char16_t>> (crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
// Error while generating bindings for class 'std::basic_string_view<char8_t, std::char_traits<char8_t>>':
// Can't generate bindings for std::basic_string_view<char8_t, std::char_traits<char8_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_string_view<char8_t, std::char_traits<char8_t>> (crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE is a template instantiation)

/// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: basic_string_view < wchar_t , std :: char_traits < wchar_t >>
pub struct __CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE {}
impl !Sync for __CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!(
        "std :: basic_string_view < wchar_t , std :: char_traits < wchar_t >>"
    ),
    crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE
);

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=290
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::traits_type':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::traits_type, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::traits_type (error: Can't generate bindings for std::char_traits<wchar_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::char_traits<wchar_t> (crate::__CcTemplateInstNSt3__u11char_traitsIwEE is a template instantiation))

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=291
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type due to missing bindings for its dependency: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=292
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::pointer':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::pointer due to missing bindings for its dependency: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=293
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_pointer':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_pointer due to missing bindings for its dependency: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=294
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reference':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reference due to missing bindings for its dependency: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=295
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reference':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reference due to missing bindings for its dependency: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=301
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_iterator':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_iterator due to missing bindings for its dependency: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=303
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::iterator':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::iterator due to missing bindings for its dependency: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=304
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator (error: Can't generate bindings for std::reverse_iterator<const wchar_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::reverse_iterator<const wchar_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE is a template instantiation))

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=305
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reverse_iterator':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reverse_iterator, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reverse_iterator (error: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator)

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=306
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::size_type':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::size_type, because it is unsupported: b/485949049: type definitions nested inside templated records are not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=307
// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::difference_type':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::difference_type, because it is unsupported: b/485949049: type definitions nested inside templated records are not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=308
// Error while generating bindings for global variable 'std::basic_string_view<wchar_t>::npos':
// static data members are not supported

// Generated from: nowhere/llvm/src/libcxx/include/__config;l=154
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=320
// Error while generating bindings for constructor 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>> (b/248542210: template instantiation of member function cannot reliably get bindings)

// Generated from: nowhere/llvm/src/libcxx/include/__configuration/attributes.h;l=86
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=322
// Error while generating bindings for constructor 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>> (b/248542210: template instantiation of member function cannot reliably get bindings)

// Generated from: nowhere/llvm/src/libcxx/include/__configuration/attributes.h;l=86
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=324
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::operator=':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::operator= (b/248542210: template instantiation of member function cannot reliably get bindings)

// Generated from: nowhere/llvm/src/libcxx/include/__config;l=154
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=326
// Error while generating bindings for constructor 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=342
// Error while generating bindings for function 'std::basic_string_view<wchar_t>::basic_string_view':
// Function templates are not supported yet

// Generated from: nowhere/llvm/src/libcxx/include/__config;l=154
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=361
// Error while generating bindings for constructor 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=370
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::begin':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=372
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::end':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=374
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::cbegin':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=382
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::cend':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=390
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rbegin':
// Cannot use an error type by value: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=395
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rend':
// Cannot use an error type by value: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=399
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::crbegin':
// Cannot use an error type by value: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=404
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::crend':
// Cannot use an error type by value: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=409
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::size':
// Return type is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=411
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::length':
// Return type is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=413
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::max_size':
// Return type is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=417
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::empty':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::empty, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::empty ([[nodiscard]] attribute)

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=420
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::operator[]':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=425
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::at':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=429
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::front':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=433
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::back':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=437
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::data':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__config;l=339
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=440
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::remove_prefix':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/__config;l=339
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=446
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::remove_suffix':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/__config;l=339
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=451
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::swap':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::swap, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::swap (b/248542210: template instantiation of member function cannot reliably get bindings)

// Generated from: nowhere/llvm/src/libcxx/include/__configuration/attributes.h;l=86
// Expanded at: nowhere/llvm/src/libcxx/include/string_view;l=461
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::copy':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=470
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::substr':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=486
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare ([[nodiscard]] attribute)

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=494
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=499
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #3 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #4 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=504
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=509
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=514
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #3 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=521
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=526
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=531
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=538
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=546
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rfind':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=551
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rfind':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=556
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rfind':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=563
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rfind':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=571
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_of':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=577
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_of':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=582
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=589
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=597
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_of':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=603
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_of':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=608
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=615
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=623
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_not_of':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=629
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_not_of':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=634
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_not_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=641
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_not_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=649
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_not_of':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=655
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_not_of':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=660
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_not_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=667
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_not_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=675
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with ([[nodiscard]] attribute)

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=679
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with':
// Parameter #0 is not supported: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=683
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with':
// Parameter #0 is not supported: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=688
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with ([[nodiscard]] attribute)

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=692
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with':
// Parameter #0 is not supported: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=696
// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with':
// Parameter #0 is not supported: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<char32_t, std::__cxx_atomic_base_impl<char32_t>>':
// Can't generate bindings for std::__cxx_atomic_impl<char32_t, std::__cxx_atomic_base_impl<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<char32_t, std::__cxx_atomic_base_impl<char32_t>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIDiNS_22__cxx_atomic_base_implIDiEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<char16_t, std::__cxx_atomic_base_impl<char16_t>>':
// Can't generate bindings for std::__cxx_atomic_impl<char16_t, std::__cxx_atomic_base_impl<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<char16_t, std::__cxx_atomic_base_impl<char16_t>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIDsNS_22__cxx_atomic_base_implIDsEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<char8_t, std::__cxx_atomic_base_impl<char8_t>>':
// Can't generate bindings for std::__cxx_atomic_impl<char8_t, std::__cxx_atomic_base_impl<char8_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<char8_t, std::__cxx_atomic_base_impl<char8_t>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIDuNS_22__cxx_atomic_base_implIDuEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<absl::base_internal::PerThreadSynch::State, std::__cxx_atomic_base_impl<absl::base_internal::PerThreadSynch::State>>':
// Can't generate bindings for std::__cxx_atomic_impl<absl::base_internal::PerThreadSynch::State, std::__cxx_atomic_base_impl<absl::base_internal::PerThreadSynch::State>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<absl::base_internal::PerThreadSynch::State, std::__cxx_atomic_base_impl<absl::base_internal::PerThreadSynch::State>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIN4absl13base_internal14PerThreadSynch5StateENS_22__cxx_atomic_base_implIS4_EEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<absl::base_internal::ThreadIdentity::WaitState, std::__cxx_atomic_base_impl<absl::base_internal::ThreadIdentity::WaitState>>':
// Can't generate bindings for std::__cxx_atomic_impl<absl::base_internal::ThreadIdentity::WaitState, std::__cxx_atomic_base_impl<absl::base_internal::ThreadIdentity::WaitState>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<absl::base_internal::ThreadIdentity::WaitState, std::__cxx_atomic_base_impl<absl::base_internal::ThreadIdentity::WaitState>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIN4absl13base_internal14ThreadIdentity9WaitStateENS_22__cxx_atomic_base_implIS4_EEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<base::scheduling::Schedulable *, std::__cxx_atomic_base_impl<base::scheduling::Schedulable *>>':
// Can't generate bindings for std::__cxx_atomic_impl<base::scheduling::Schedulable *, std::__cxx_atomic_base_impl<base::scheduling::Schedulable *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<base::scheduling::Schedulable *, std::__cxx_atomic_base_impl<base::scheduling::Schedulable *>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIPN4base10scheduling11SchedulableENS_22__cxx_atomic_base_implIS4_EEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<signed char, std::__cxx_atomic_base_impl<signed char>>':
// Can't generate bindings for std::__cxx_atomic_impl<signed char, std::__cxx_atomic_base_impl<signed char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<signed char, std::__cxx_atomic_base_impl<signed char>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIaNS_22__cxx_atomic_base_implIaEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<bool, std::__cxx_atomic_base_impl<bool>>':
// Can't generate bindings for std::__cxx_atomic_impl<bool, std::__cxx_atomic_base_impl<bool>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<bool, std::__cxx_atomic_base_impl<bool>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIbNS_22__cxx_atomic_base_implIbEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<char, std::__cxx_atomic_base_impl<char>>':
// Can't generate bindings for std::__cxx_atomic_impl<char, std::__cxx_atomic_base_impl<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<char, std::__cxx_atomic_base_impl<char>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIcNS_22__cxx_atomic_base_implIcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<unsigned char, std::__cxx_atomic_base_impl<unsigned char>>':
// Can't generate bindings for std::__cxx_atomic_impl<unsigned char, std::__cxx_atomic_base_impl<unsigned char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<unsigned char, std::__cxx_atomic_base_impl<unsigned char>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIhNS_22__cxx_atomic_base_implIhEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<int, std::__cxx_atomic_base_impl<int>>':
// Can't generate bindings for std::__cxx_atomic_impl<int, std::__cxx_atomic_base_impl<int>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<int, std::__cxx_atomic_base_impl<int>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIiNS_22__cxx_atomic_base_implIiEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<unsigned int, std::__cxx_atomic_base_impl<unsigned int>>':
// Can't generate bindings for std::__cxx_atomic_impl<unsigned int, std::__cxx_atomic_base_impl<unsigned int>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<unsigned int, std::__cxx_atomic_base_impl<unsigned int>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIjNS_22__cxx_atomic_base_implIjEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<long, std::__cxx_atomic_base_impl<long>>':
// Can't generate bindings for std::__cxx_atomic_impl<long, std::__cxx_atomic_base_impl<long>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<long, std::__cxx_atomic_base_impl<long>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIlNS_22__cxx_atomic_base_implIlEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<unsigned long, std::__cxx_atomic_base_impl<unsigned long>>':
// Can't generate bindings for std::__cxx_atomic_impl<unsigned long, std::__cxx_atomic_base_impl<unsigned long>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<unsigned long, std::__cxx_atomic_base_impl<unsigned long>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implImNS_22__cxx_atomic_base_implImEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<short, std::__cxx_atomic_base_impl<short>>':
// Can't generate bindings for std::__cxx_atomic_impl<short, std::__cxx_atomic_base_impl<short>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<short, std::__cxx_atomic_base_impl<short>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIsNS_22__cxx_atomic_base_implIsEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<unsigned short, std::__cxx_atomic_base_impl<unsigned short>>':
// Can't generate bindings for std::__cxx_atomic_impl<unsigned short, std::__cxx_atomic_base_impl<unsigned short>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<unsigned short, std::__cxx_atomic_base_impl<unsigned short>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implItNS_22__cxx_atomic_base_implItEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<wchar_t, std::__cxx_atomic_base_impl<wchar_t>>':
// Can't generate bindings for std::__cxx_atomic_impl<wchar_t, std::__cxx_atomic_base_impl<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<wchar_t, std::__cxx_atomic_base_impl<wchar_t>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIwNS_22__cxx_atomic_base_implIwEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<long long, std::__cxx_atomic_base_impl<long long>>':
// Can't generate bindings for std::__cxx_atomic_impl<long long, std::__cxx_atomic_base_impl<long long>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<long long, std::__cxx_atomic_base_impl<long long>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIxNS_22__cxx_atomic_base_implIxEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// Error while generating bindings for struct 'std::__cxx_atomic_impl<unsigned long long, std::__cxx_atomic_base_impl<unsigned long long>>':
// Can't generate bindings for std::__cxx_atomic_impl<unsigned long long, std::__cxx_atomic_base_impl<unsigned long long>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__cxx_atomic_impl<unsigned long long, std::__cxx_atomic_base_impl<unsigned long long>> (crate::__CcTemplateInstNSt3__u17__cxx_atomic_implIyNS_22__cxx_atomic_base_implIyEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<1000000000000000000L, 1L>':
// Can't generate bindings for std::ratio<1000000000000000000L, 1L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<1000000000000000000L, 1L> (crate::__CcTemplateInstNSt3__u5ratioILl1000000000000000000ELl1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<1000000000000000L, 1L>':
// Can't generate bindings for std::ratio<1000000000000000L, 1L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<1000000000000000L, 1L> (crate::__CcTemplateInstNSt3__u5ratioILl1000000000000000ELl1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<1000000000000L, 1L>':
// Can't generate bindings for std::ratio<1000000000000L, 1L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<1000000000000L, 1L> (crate::__CcTemplateInstNSt3__u5ratioILl1000000000000ELl1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<1000000000L, 1L>':
// Can't generate bindings for std::ratio<1000000000L, 1L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<1000000000L, 1L> (crate::__CcTemplateInstNSt3__u5ratioILl1000000000ELl1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<1000000L, 1L>':
// Can't generate bindings for std::ratio<1000000L, 1L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<1000000L, 1L> (crate::__CcTemplateInstNSt3__u5ratioILl1000000ELl1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<1000L, 1L>':
// Can't generate bindings for std::ratio<1000L, 1L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<1000L, 1L> (crate::__CcTemplateInstNSt3__u5ratioILl1000ELl1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<100L, 1L>':
// Can't generate bindings for std::ratio<100L, 1L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<100L, 1L> (crate::__CcTemplateInstNSt3__u5ratioILl100ELl1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<10L, 1L>':
// Can't generate bindings for std::ratio<10L, 1L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<10L, 1L> (crate::__CcTemplateInstNSt3__u5ratioILl10ELl1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<1L, 1000000000000000000L>':
// Can't generate bindings for std::ratio<1L, 1000000000000000000L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<1L, 1000000000000000000L> (crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000000EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<1L, 1000000000000000L>':
// Can't generate bindings for std::ratio<1L, 1000000000000000L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<1L, 1000000000000000L> (crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<1L, 1000000000000L>':
// Can't generate bindings for std::ratio<1L, 1000000000000L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<1L, 1000000000000L> (crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<1L, 1000000000L>':
// Can't generate bindings for std::ratio<1L, 1000000000L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<1L, 1000000000L> (crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<1L, 1000000L>':
// Can't generate bindings for std::ratio<1L, 1000000L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<1L, 1000000L> (crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<1L, 1000L>':
// Can't generate bindings for std::ratio<1L, 1000L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<1L, 1000L> (crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<1L, 100L>':
// Can't generate bindings for std::ratio<1L, 100L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<1L, 100L> (crate::__CcTemplateInstNSt3__u5ratioILl1ELl100EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<1L, 10L>':
// Can't generate bindings for std::ratio<1L, 10L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<1L, 10L> (crate::__CcTemplateInstNSt3__u5ratioILl1ELl10EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<1L, 1L>':
// Can't generate bindings for std::ratio<1L, 1L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<1L, 1L> (crate::__CcTemplateInstNSt3__u5ratioILl1ELl1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<2629746L, 1L>':
// Can't generate bindings for std::ratio<2629746L, 1L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<2629746L, 1L> (crate::__CcTemplateInstNSt3__u5ratioILl2629746ELl1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<31556952L, 1L>':
// Can't generate bindings for std::ratio<31556952L, 1L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<31556952L, 1L> (crate::__CcTemplateInstNSt3__u5ratioILl31556952ELl1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<3600L, 1L>':
// Can't generate bindings for std::ratio<3600L, 1L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<3600L, 1L> (crate::__CcTemplateInstNSt3__u5ratioILl3600ELl1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<604800L, 1L>':
// Can't generate bindings for std::ratio<604800L, 1L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<604800L, 1L> (crate::__CcTemplateInstNSt3__u5ratioILl604800ELl1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<60L, 1L>':
// Can't generate bindings for std::ratio<60L, 1L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<60L, 1L> (crate::__CcTemplateInstNSt3__u5ratioILl60ELl1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// Error while generating bindings for class 'std::ratio<86400L, 1L>':
// Can't generate bindings for std::ratio<86400L, 1L>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ratio<86400L, 1L> (crate::__CcTemplateInstNSt3__u5ratioILl86400ELl1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// Error while generating bindings for class 'std::chrono::duration<int, std::ratio<2629746L, 1L>>':
// Can't generate bindings for std::chrono::duration<int, std::ratio<2629746L, 1L>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::duration<int, std::ratio<2629746L, 1L>> (crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl2629746ELl1EEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// Error while generating bindings for class 'std::chrono::duration<int, std::ratio<31556952L, 1L>>':
// Can't generate bindings for std::chrono::duration<int, std::ratio<31556952L, 1L>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::duration<int, std::ratio<31556952L, 1L>> (crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl31556952ELl1EEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// Error while generating bindings for class 'std::chrono::duration<int, std::ratio<604800L, 1L>>':
// Can't generate bindings for std::chrono::duration<int, std::ratio<604800L, 1L>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::duration<int, std::ratio<604800L, 1L>> (crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl604800ELl1EEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// Error while generating bindings for class 'std::chrono::duration<int, std::ratio<86400L, 1L>>':
// Can't generate bindings for std::chrono::duration<int, std::ratio<86400L, 1L>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::duration<int, std::ratio<86400L, 1L>> (crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// Error while generating bindings for class 'std::chrono::duration<long, std::ratio<1L, 1000000000000000L>>':
// Can't generate bindings for std::chrono::duration<long, std::ratio<1L, 1000000000000000L>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::duration<long, std::ratio<1L, 1000000000000000L>> (crate::__CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl1ELl1000000000000000EEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// Error while generating bindings for class 'std::chrono::duration<long, std::ratio<1L, 1L>>':
// Can't generate bindings for std::chrono::duration<long, std::ratio<1L, 1L>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::duration<long, std::ratio<1L, 1L>> (crate::__CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl1ELl1EEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// Error while generating bindings for class 'std::chrono::duration<long, std::ratio<3600L, 1L>>':
// Can't generate bindings for std::chrono::duration<long, std::ratio<3600L, 1L>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::duration<long, std::ratio<3600L, 1L>> (crate::__CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl3600ELl1EEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// Error while generating bindings for class 'std::chrono::duration<long, std::ratio<60L, 1L>>':
// Can't generate bindings for std::chrono::duration<long, std::ratio<60L, 1L>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::duration<long, std::ratio<60L, 1L>> (crate::__CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl60ELl1EEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// Error while generating bindings for class 'std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>':
// Can't generate bindings for std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::duration<__int128, std::ratio<1L, 1000000000L>> (crate::__CcTemplateInstNSt3__u6chrono8durationInNS_5ratioILl1ELl1000000000EEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// Error while generating bindings for class 'std::chrono::duration<long long, std::ratio<1L, 1000000000L>>':
// Can't generate bindings for std::chrono::duration<long long, std::ratio<1L, 1000000000L>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::duration<long long, std::ratio<1L, 1000000000L>> (crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// Error while generating bindings for class 'std::chrono::duration<long long, std::ratio<1L, 1000000L>>':
// Can't generate bindings for std::chrono::duration<long long, std::ratio<1L, 1000000L>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::duration<long long, std::ratio<1L, 1000000L>> (crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// Error while generating bindings for class 'std::chrono::duration<long long, std::ratio<1L, 1000L>>':
// Can't generate bindings for std::chrono::duration<long long, std::ratio<1L, 1000L>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::duration<long long, std::ratio<1L, 1000L>> (crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000EEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// Error while generating bindings for class 'std::chrono::duration<long long, std::ratio<1L, 1L>>':
// Can't generate bindings for std::chrono::duration<long long, std::ratio<1L, 1L>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::duration<long long, std::ratio<1L, 1L>> (crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/time_point.h;l=36
// Error while generating bindings for class 'std::chrono::time_point<std::chrono::steady_clock, std::chrono::duration<long long, std::ratio<1L, 1000000000L>>>':
// Can't generate bindings for std::chrono::time_point<std::chrono::steady_clock, std::chrono::duration<long long, std::ratio<1L, 1000000000L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::time_point<std::chrono::steady_clock, std::chrono::duration<long long, std::ratio<1L, 1000000000L>>> (crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/time_point.h;l=36
// Error while generating bindings for class 'std::chrono::time_point<std::chrono::system_clock, std::chrono::duration<int, std::ratio<86400L, 1L>>>':
// Can't generate bindings for std::chrono::time_point<std::chrono::system_clock, std::chrono::duration<int, std::ratio<86400L, 1L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::time_point<std::chrono::system_clock, std::chrono::duration<int, std::ratio<86400L, 1L>>> (crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/time_point.h;l=36
// Error while generating bindings for class 'std::chrono::time_point<std::chrono::system_clock, std::chrono::duration<long long, std::ratio<1L, 1000000L>>>':
// Can't generate bindings for std::chrono::time_point<std::chrono::system_clock, std::chrono::duration<long long, std::ratio<1L, 1000000L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::time_point<std::chrono::system_clock, std::chrono::duration<long long, std::ratio<1L, 1000000L>>> (crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/time_point.h;l=36
// Error while generating bindings for class 'std::chrono::time_point<std::chrono::system_clock, std::chrono::duration<long long, std::ratio<1L, 1L>>>':
// Can't generate bindings for std::chrono::time_point<std::chrono::system_clock, std::chrono::duration<long long, std::ratio<1L, 1L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::time_point<std::chrono::system_clock, std::chrono::duration<long long, std::ratio<1L, 1L>>> (crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/time_point.h;l=36
// Error while generating bindings for class 'std::chrono::time_point<std::chrono::local_t, std::chrono::duration<int, std::ratio<86400L, 1L>>>':
// Can't generate bindings for std::chrono::time_point<std::chrono::local_t, std::chrono::duration<int, std::ratio<86400L, 1L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::time_point<std::chrono::local_t, std::chrono::duration<int, std::ratio<86400L, 1L>>> (crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_7local_tENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/time_point.h;l=36
// Error while generating bindings for class 'std::chrono::time_point<std::chrono::local_t, std::chrono::duration<long long, std::ratio<1L, 1L>>>':
// Can't generate bindings for std::chrono::time_point<std::chrono::local_t, std::chrono::duration<long long, std::ratio<1L, 1L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::time_point<std::chrono::local_t, std::chrono::duration<long long, std::ratio<1L, 1L>>> (crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_7local_tENS0_8durationIxNS_5ratioILl1ELl1EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/time_point.h;l=36
// Error while generating bindings for class 'std::chrono::time_point<std::filesystem::_FilesystemClock, std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>>':
// Can't generate bindings for std::chrono::time_point<std::filesystem::_FilesystemClock, std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::chrono::time_point<std::filesystem::_FilesystemClock, std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>> (crate::__CcTemplateInstNSt3__u6chrono10time_pointINS_10filesystem16_FilesystemClockENS0_8durationInNS_5ratioILl1ELl1000000000EEEEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<char32_t, false>':
// Can't generate bindings for std::__atomic_base<char32_t, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<char32_t, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<char16_t, false>':
// Can't generate bindings for std::__atomic_base<char16_t, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<char16_t, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<char8_t, false>':
// Can't generate bindings for std::__atomic_base<char8_t, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<char8_t, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<absl::base_internal::PerThreadSynch::State, false>':
// Can't generate bindings for std::__atomic_base<absl::base_internal::PerThreadSynch::State, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<absl::base_internal::PerThreadSynch::State, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIN4absl13base_internal14PerThreadSynch5StateELb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState, false>':
// Can't generate bindings for std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIN4absl13base_internal14ThreadIdentity9WaitStateELb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<base::scheduling::Schedulable *, false>':
// Can't generate bindings for std::__atomic_base<base::scheduling::Schedulable *, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<base::scheduling::Schedulable *, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIPN4base10scheduling11SchedulableELb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<signed char, false>':
// Can't generate bindings for std::__atomic_base<signed char, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<signed char, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<bool, false>':
// Can't generate bindings for std::__atomic_base<bool, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<bool, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<char, false>':
// Can't generate bindings for std::__atomic_base<char, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<char, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<unsigned char, false>':
// Can't generate bindings for std::__atomic_base<unsigned char, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<unsigned char, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<int, false>':
// Can't generate bindings for std::__atomic_base<int, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<int, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<unsigned int, false>':
// Can't generate bindings for std::__atomic_base<unsigned int, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<unsigned int, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<long, false>':
// Can't generate bindings for std::__atomic_base<long, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<long, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<unsigned long, false>':
// Can't generate bindings for std::__atomic_base<unsigned long, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<unsigned long, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseImLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<short, false>':
// Can't generate bindings for std::__atomic_base<short, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<short, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<unsigned short, false>':
// Can't generate bindings for std::__atomic_base<unsigned short, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<unsigned short, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseItLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<wchar_t, false>':
// Can't generate bindings for std::__atomic_base<wchar_t, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<wchar_t, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<long long, false>':
// Can't generate bindings for std::__atomic_base<long long, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<long long, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// Error while generating bindings for struct 'std::__atomic_base<unsigned long long, false>':
// Can't generate bindings for std::__atomic_base<unsigned long long, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<unsigned long long, false> (crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// Error while generating bindings for struct 'std::__atomic_base<char32_t, true>':
// Can't generate bindings for std::__atomic_base<char32_t, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<char32_t, true> (crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// Error while generating bindings for struct 'std::__atomic_base<char16_t, true>':
// Can't generate bindings for std::__atomic_base<char16_t, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<char16_t, true> (crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// Error while generating bindings for struct 'std::__atomic_base<char8_t, true>':
// Can't generate bindings for std::__atomic_base<char8_t, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<char8_t, true> (crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// Error while generating bindings for struct 'std::__atomic_base<signed char, true>':
// Can't generate bindings for std::__atomic_base<signed char, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<signed char, true> (crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// Error while generating bindings for struct 'std::__atomic_base<char, true>':
// Can't generate bindings for std::__atomic_base<char, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<char, true> (crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// Error while generating bindings for struct 'std::__atomic_base<unsigned char, true>':
// Can't generate bindings for std::__atomic_base<unsigned char, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<unsigned char, true> (crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// Error while generating bindings for struct 'std::__atomic_base<int, true>':
// Can't generate bindings for std::__atomic_base<int, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<int, true> (crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// Error while generating bindings for struct 'std::__atomic_base<unsigned int, true>':
// Can't generate bindings for std::__atomic_base<unsigned int, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<unsigned int, true> (crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// Error while generating bindings for struct 'std::__atomic_base<long, true>':
// Can't generate bindings for std::__atomic_base<long, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<long, true> (crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// Error while generating bindings for struct 'std::__atomic_base<unsigned long, true>':
// Can't generate bindings for std::__atomic_base<unsigned long, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<unsigned long, true> (crate::__CcTemplateInstNSt3__u13__atomic_baseImLb1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// Error while generating bindings for struct 'std::__atomic_base<short, true>':
// Can't generate bindings for std::__atomic_base<short, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<short, true> (crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// Error while generating bindings for struct 'std::__atomic_base<unsigned short, true>':
// Can't generate bindings for std::__atomic_base<unsigned short, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<unsigned short, true> (crate::__CcTemplateInstNSt3__u13__atomic_baseItLb1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// Error while generating bindings for struct 'std::__atomic_base<wchar_t, true>':
// Can't generate bindings for std::__atomic_base<wchar_t, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<wchar_t, true> (crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// Error while generating bindings for struct 'std::__atomic_base<long long, true>':
// Can't generate bindings for std::__atomic_base<long long, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<long long, true> (crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// Error while generating bindings for struct 'std::__atomic_base<unsigned long long, true>':
// Can't generate bindings for std::__atomic_base<unsigned long long, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__atomic_base<unsigned long long, true> (crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<char32_t>':
// Can't generate bindings for std::atomic<char32_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<char32_t> (crate::__CcTemplateInstNSt3__u6atomicIDiEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<char16_t>':
// Can't generate bindings for std::atomic<char16_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<char16_t> (crate::__CcTemplateInstNSt3__u6atomicIDsEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<char8_t>':
// Can't generate bindings for std::atomic<char8_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<char8_t> (crate::__CcTemplateInstNSt3__u6atomicIDuEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<absl::base_internal::PerThreadSynch::State>':
// Can't generate bindings for std::atomic<absl::base_internal::PerThreadSynch::State>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<absl::base_internal::PerThreadSynch::State> (crate::__CcTemplateInstNSt3__u6atomicIN4absl13base_internal14PerThreadSynch5StateEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<absl::base_internal::ThreadIdentity::WaitState>':
// Can't generate bindings for std::atomic<absl::base_internal::ThreadIdentity::WaitState>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<absl::base_internal::ThreadIdentity::WaitState> (crate::__CcTemplateInstNSt3__u6atomicIN4absl13base_internal14ThreadIdentity9WaitStateEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<signed char>':
// Can't generate bindings for std::atomic<signed char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<signed char> (crate::__CcTemplateInstNSt3__u6atomicIaEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<bool>':
// Can't generate bindings for std::atomic<bool>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<bool> (crate::__CcTemplateInstNSt3__u6atomicIbEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<char>':
// Can't generate bindings for std::atomic<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<char> (crate::__CcTemplateInstNSt3__u6atomicIcEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<unsigned char>':
// Can't generate bindings for std::atomic<unsigned char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<unsigned char> (crate::__CcTemplateInstNSt3__u6atomicIhEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<int>':
// Can't generate bindings for std::atomic<int>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<int> (crate::__CcTemplateInstNSt3__u6atomicIiEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<unsigned int>':
// Can't generate bindings for std::atomic<unsigned int>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<unsigned int> (crate::__CcTemplateInstNSt3__u6atomicIjEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<long>':
// Can't generate bindings for std::atomic<long>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<long> (crate::__CcTemplateInstNSt3__u6atomicIlEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<unsigned long>':
// Can't generate bindings for std::atomic<unsigned long>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<unsigned long> (crate::__CcTemplateInstNSt3__u6atomicImEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<short>':
// Can't generate bindings for std::atomic<short>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<short> (crate::__CcTemplateInstNSt3__u6atomicIsEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<unsigned short>':
// Can't generate bindings for std::atomic<unsigned short>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<unsigned short> (crate::__CcTemplateInstNSt3__u6atomicItEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<wchar_t>':
// Can't generate bindings for std::atomic<wchar_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<wchar_t> (crate::__CcTemplateInstNSt3__u6atomicIwEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<long long>':
// Can't generate bindings for std::atomic<long long>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<long long> (crate::__CcTemplateInstNSt3__u6atomicIxEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// Error while generating bindings for struct 'std::atomic<unsigned long long>':
// Can't generate bindings for std::atomic<unsigned long long>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<unsigned long long> (crate::__CcTemplateInstNSt3__u6atomicIyEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=267
// Error while generating bindings for struct 'std::atomic<base::scheduling::Schedulable *>':
// Can't generate bindings for std::atomic<base::scheduling::Schedulable *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::atomic<base::scheduling::Schedulable *> (crate::__CcTemplateInstNSt3__u6atomicIPN4base10scheduling11SchedulableEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/ios.h;l=24
// Error while generating bindings for class 'std::basic_ios<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_ios<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_ios<char, std::char_traits<char>> (crate::__CcTemplateInstNSt3__u9basic_iosIcNS_11char_traitsIcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/ios.h;l=24
// Error while generating bindings for class 'std::basic_ios<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_ios<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_ios<wchar_t, std::char_traits<wchar_t>> (crate::__CcTemplateInstNSt3__u9basic_iosIwNS_11char_traitsIwEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/streambuf.h;l=22
// Error while generating bindings for class 'std::basic_streambuf<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_streambuf<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_streambuf<char, std::char_traits<char>> (crate::__CcTemplateInstNSt3__u15basic_streambufIcNS_11char_traitsIcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/streambuf.h;l=22
// Error while generating bindings for class 'std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_streambuf<wchar_t, std::char_traits<wchar_t>> (crate::__CcTemplateInstNSt3__u15basic_streambufIwNS_11char_traitsIwEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/ostream.h;l=22
// Error while generating bindings for class 'std::basic_ostream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_ostream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_ostream<char, std::char_traits<char>> (crate::__CcTemplateInstNSt3__u13basic_ostreamIcNS_11char_traitsIcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/ostream.h;l=22
// Error while generating bindings for class 'std::basic_ostream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_ostream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_ostream<wchar_t, std::char_traits<wchar_t>> (crate::__CcTemplateInstNSt3__u13basic_ostreamIwNS_11char_traitsIwEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_context.h;l=70
// Error while generating bindings for class 'std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<char>>, char>':
// Can't generate bindings for std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<char>>, char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<char>>, char> (crate::__CcTemplateInstNSt3__u20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_context.h;l=70
// Error while generating bindings for class 'std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, wchar_t>':
// Can't generate bindings for std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, wchar_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, wchar_t> (crate::__CcTemplateInstNSt3__u20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/istream.h;l=22
// Error while generating bindings for class 'std::basic_istream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_istream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_istream<char, std::char_traits<char>> (crate::__CcTemplateInstNSt3__u13basic_istreamIcNS_11char_traitsIcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/istream.h;l=22
// Error while generating bindings for class 'std::basic_istream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_istream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_istream<wchar_t, std::char_traits<wchar_t>> (crate::__CcTemplateInstNSt3__u13basic_istreamIwNS_11char_traitsIwEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/istream.h;l=25
// Error while generating bindings for class 'std::basic_iostream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_iostream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_iostream<char, std::char_traits<char>> (crate::__CcTemplateInstNSt3__u14basic_iostreamIcNS_11char_traitsIcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/sstream.h;l=23
// Error while generating bindings for class 'std::basic_stringbuf<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_stringbuf<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_stringbuf<char, std::char_traits<char>, std::allocator<char>> (crate::__CcTemplateInstNSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/sstream.h;l=30
// Error while generating bindings for class 'std::basic_stringstream<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_stringstream<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_stringstream<char, std::char_traits<char>, std::allocator<char>> (crate::__CcTemplateInstNSt3__u18basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/sstream.h;l=28
// Error while generating bindings for class 'std::basic_ostringstream<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_ostringstream<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_ostringstream<char, std::char_traits<char>, std::allocator<char>> (crate::__CcTemplateInstNSt3__u19basic_ostringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/sstream.h;l=26
// Error while generating bindings for class 'std::basic_istringstream<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_istringstream<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_istringstream<char, std::char_traits<char>, std::allocator<char>> (crate::__CcTemplateInstNSt3__u19basic_istringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// Error while generating bindings for struct 'std::placeholders::__ph<10>':
// Can't generate bindings for std::placeholders::__ph<10>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::placeholders::__ph<10> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi10EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// Error while generating bindings for struct 'std::placeholders::__ph<1>':
// Can't generate bindings for std::placeholders::__ph<1>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::placeholders::__ph<1> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// Error while generating bindings for struct 'std::placeholders::__ph<2>':
// Can't generate bindings for std::placeholders::__ph<2>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::placeholders::__ph<2> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi2EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// Error while generating bindings for struct 'std::placeholders::__ph<3>':
// Can't generate bindings for std::placeholders::__ph<3>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::placeholders::__ph<3> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi3EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// Error while generating bindings for struct 'std::placeholders::__ph<4>':
// Can't generate bindings for std::placeholders::__ph<4>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::placeholders::__ph<4> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi4EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// Error while generating bindings for struct 'std::placeholders::__ph<5>':
// Can't generate bindings for std::placeholders::__ph<5>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::placeholders::__ph<5> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi5EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// Error while generating bindings for struct 'std::placeholders::__ph<6>':
// Can't generate bindings for std::placeholders::__ph<6>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::placeholders::__ph<6> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi6EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// Error while generating bindings for struct 'std::placeholders::__ph<7>':
// Can't generate bindings for std::placeholders::__ph<7>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::placeholders::__ph<7> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi7EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// Error while generating bindings for struct 'std::placeholders::__ph<8>':
// Can't generate bindings for std::placeholders::__ph<8>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::placeholders::__ph<8> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi8EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// Error while generating bindings for struct 'std::placeholders::__ph<9>':
// Can't generate bindings for std::placeholders::__ph<9>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::placeholders::__ph<9> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi9EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/back_insert_iterator.h;l=30
// Error while generating bindings for class 'std::back_insert_iterator<std::__format::__output_buffer<char>>':
// Can't generate bindings for std::back_insert_iterator<std::__format::__output_buffer<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::back_insert_iterator<std::__format::__output_buffer<char>> (crate::__CcTemplateInstNSt3__u20back_insert_iteratorINS_8__format15__output_bufferIcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/back_insert_iterator.h;l=30
// Error while generating bindings for class 'std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>':
// Can't generate bindings for std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::back_insert_iterator<std::__format::__output_buffer<wchar_t>> (crate::__CcTemplateInstNSt3__u20back_insert_iteratorINS_8__format15__output_bufferIwEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__ranges/elements_view.h;l=393
// Error while generating bindings for struct 'std::ranges::views::__elements::__fn<0UL>':
// Can't generate bindings for std::ranges::views::__elements::__fn<0UL>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ranges::views::__elements::__fn<0UL> (crate::__CcTemplateInstNSt3__u6ranges5views10__elements4__fnILm0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__ranges/elements_view.h;l=393
// Error while generating bindings for struct 'std::ranges::views::__elements::__fn<1UL>':
// Can't generate bindings for std::ranges::views::__elements::__fn<1UL>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::ranges::views::__elements::__fn<1UL> (crate::__CcTemplateInstNSt3__u6ranges5views10__elements4__fnILm1EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_parse_context.h;l=27
// Error while generating bindings for class 'std::basic_format_parse_context<char>':
// Can't generate bindings for std::basic_format_parse_context<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_format_parse_context<char> (crate::__CcTemplateInstNSt3__u26basic_format_parse_contextIcEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_parse_context.h;l=27
// Error while generating bindings for class 'std::basic_format_parse_context<wchar_t>':
// Can't generate bindings for std::basic_format_parse_context<wchar_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_format_parse_context<wchar_t> (crate::__CcTemplateInstNSt3__u26basic_format_parse_contextIwEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__format/buffer.h;l=182
// Error while generating bindings for class 'std::__format::__output_buffer<char>':
// Can't generate bindings for std::__format::__output_buffer<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__format::__output_buffer<char> (crate::__CcTemplateInstNSt3__u8__format15__output_bufferIcEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__format/buffer.h;l=182
// Error while generating bindings for class 'std::__format::__output_buffer<wchar_t>':
// Can't generate bindings for std::__format::__output_buffer<wchar_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__format::__output_buffer<wchar_t> (crate::__CcTemplateInstNSt3__u8__format15__output_bufferIwEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_arg.h;l=210
// Error while generating bindings for class 'std::__basic_format_arg_value<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>':
// Can't generate bindings for std::__basic_format_arg_value<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__basic_format_arg_value<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<char>>, char>> (crate::__CcTemplateInstNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_arg.h;l=210
// Error while generating bindings for class 'std::__basic_format_arg_value<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, wchar_t>>':
// Can't generate bindings for std::__basic_format_arg_value<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::__basic_format_arg_value<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, wchar_t>> (crate::__CcTemplateInstNSt3__u24__basic_format_arg_valueINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_arg.h;l=280
// Error while generating bindings for class 'std::basic_format_arg<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>':
// Can't generate bindings for std::basic_format_arg<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_format_arg<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<char>>, char>> (crate::__CcTemplateInstNSt3__u16basic_format_argINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_arg.h;l=280
// Error while generating bindings for class 'std::basic_format_arg<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, wchar_t>>':
// Can't generate bindings for std::basic_format_arg<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_format_arg<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, wchar_t>> (crate::__CcTemplateInstNSt3__u16basic_format_argINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_args.h;l=29
// Error while generating bindings for class 'std::basic_format_args<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>':
// Can't generate bindings for std::basic_format_args<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_format_args<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<char>>, char>> (crate::__CcTemplateInstNSt3__u17basic_format_argsINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIcEEEEcEEEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_args.h;l=29
// Error while generating bindings for class 'std::basic_format_args<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, wchar_t>>':
// Can't generate bindings for std::basic_format_args<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/display:displayables needs [//features:wrapper] for std::basic_format_args<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, wchar_t>> (crate::__CcTemplateInstNSt3__u17basic_format_argsINS_20basic_format_contextINS_20back_insert_iteratorINS_8__format15__output_bufferIwEEEEwEEEE is a template instantiation)

#[path = "rs_bindings_from_cc/test/display/displayables.rs"]
mod __crubit_mod_0;
#[allow(unused_imports)]
pub use __crubit_mod_0::*;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN16CanAbslStringifyC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __crubit_fmt__16CanAbslStringify___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(
            value: &crate::CanAbslStringify,
            formatter: &mut ::lossy_formatter::LossyFormatter,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk___ZN22CanAbslStringifyByFillC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __crubit_fmt__22CanAbslStringifyByFill___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(
            value: &crate::CanAbslStringifyByFill,
            formatter: &mut ::lossy_formatter::LossyFormatter,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk___ZN24CanAbslStringifyByFormatC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __crubit_fmt__24CanAbslStringifyByFormat___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(
            value: &crate::CanAbslStringifyByFormat,
            formatter: &mut ::lossy_formatter::LossyFormatter,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk___ZN10CanOstreamC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __crubit_fmt__10CanOstream___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(
            value: &crate::CanOstream,
            formatter: &mut ::lossy_formatter::LossyFormatter,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk___ZN26CanAbslStringifyAndOstreamC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __crubit_fmt__26CanAbslStringifyAndOstream___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(
            value: &crate::CanAbslStringifyAndOstream,
            formatter: &mut ::lossy_formatter::LossyFormatter,
        ) -> bool;
        pub(crate) unsafe fn __crubit_fmt__DisplayableEnum___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(
            value: &crate::DisplayableEnum,
            formatter: &mut ::lossy_formatter::LossyFormatter,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk___ZN14NotDisplayableC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19TemplatedStringViewC1ENSt3__u17basic_string_viewIcNS0_11char_traitsIcEEEE(
            __this: *mut ::core::ffi::c_void,
            v: &mut ::string_view::absl::string_view,
        );
        pub(crate) unsafe fn __crubit_fmt__19TemplatedStringView___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fdisplay_3adisplayables(
            value: &crate::TemplatedStringView,
            formatter: &mut ::lossy_formatter::LossyFormatter,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk___ZN23TemplatedNotDisplayableC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13DisplayInRustC1Ev(__this: *mut ::core::ffi::c_void);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::CanAbslStringify>() == 16);
    assert!(::core::mem::align_of::<crate::CanAbslStringify>() == 8);
    static_assertions::assert_impl_all!(crate::CanAbslStringify: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::CanAbslStringify: Drop);
    assert!(::core::mem::offset_of!(crate::CanAbslStringify, value) == 0);
    assert!(::core::mem::size_of::<crate::CanAbslStringifyByFill>() == 16);
    assert!(::core::mem::align_of::<crate::CanAbslStringifyByFill>() == 8);
    static_assertions::assert_impl_all!(crate::CanAbslStringifyByFill: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::CanAbslStringifyByFill: Drop);
    assert!(::core::mem::offset_of!(crate::CanAbslStringifyByFill, count) == 0);
    assert!(::core::mem::offset_of!(crate::CanAbslStringifyByFill, ch) == 8);
    assert!(::core::mem::size_of::<crate::CanAbslStringifyByFormat>() == 16);
    assert!(::core::mem::align_of::<crate::CanAbslStringifyByFormat>() == 8);
    static_assertions::assert_impl_all!(crate::CanAbslStringifyByFormat: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::CanAbslStringifyByFormat: Drop);
    assert!(::core::mem::offset_of!(crate::CanAbslStringifyByFormat, value) == 0);
    assert!(::core::mem::size_of::<crate::CanOstream>() == 16);
    assert!(::core::mem::align_of::<crate::CanOstream>() == 8);
    static_assertions::assert_impl_all!(crate::CanOstream: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::CanOstream: Drop);
    assert!(::core::mem::offset_of!(crate::CanOstream, value) == 0);
    assert!(::core::mem::size_of::<crate::CanAbslStringifyAndOstream>() == 32);
    assert!(::core::mem::align_of::<crate::CanAbslStringifyAndOstream>() == 8);
    static_assertions::assert_impl_all!(crate::CanAbslStringifyAndOstream: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::CanAbslStringifyAndOstream: Drop);
    assert!(::core::mem::offset_of!(crate::CanAbslStringifyAndOstream, stringify) == 0);
    assert!(::core::mem::offset_of!(crate::CanAbslStringifyAndOstream, ostream) == 16);
    assert!(::core::mem::size_of::<crate::NotDisplayable>() == 1);
    assert!(::core::mem::align_of::<crate::NotDisplayable>() == 1);
    static_assertions::assert_impl_all!(crate::NotDisplayable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NotDisplayable: Drop);

    assert!(::core::mem::size_of::<crate::TemplatedStringView>() == 16);
    assert!(::core::mem::align_of::<crate::TemplatedStringView>() == 8);
    static_assertions::assert_impl_all!(crate::TemplatedStringView: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TemplatedStringView: Drop);

    assert!(::core::mem::size_of::<crate::TemplatedNotDisplayable>() == 1);
    assert!(::core::mem::align_of::<crate::TemplatedNotDisplayable>() == 1);
    static_assertions::assert_impl_all!(crate::TemplatedNotDisplayable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TemplatedNotDisplayable: Drop);

    assert!(::core::mem::size_of::<crate::DisplayInRust>() == 32);
    assert!(::core::mem::align_of::<crate::DisplayInRust>() == 8);
    static_assertions::assert_impl_all!(crate::DisplayInRust: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::DisplayInRust: Drop);
    assert!(::core::mem::offset_of!(crate::DisplayInRust, cc_value) == 0);
    assert!(::core::mem::offset_of!(crate::DisplayInRust, rust_value) == 16);
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE,
            __size_
        ) == 8
    );
};

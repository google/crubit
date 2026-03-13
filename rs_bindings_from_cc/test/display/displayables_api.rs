// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/display:displayables
// Features: fmt, supported, types

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=15
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=CanAbslStringify
pub struct CanAbslStringify {
    pub value: ::cc_std::std::__u::raw_string_view,
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
    pub value: ::cc_std::std::__u::raw_string_view,
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
    pub value: ::cc_std::std::__u::raw_string_view,
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
// error: function `operator<<` could not be bound
//   Unsupported return type: template instantiation is not yet supported
//   Unsupported parameter #0 (out): template instantiation is not yet supported

/// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=51
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=CanAbslStringifyAndOstream
pub struct CanAbslStringifyAndOstream {
    pub stringify: ::cc_std::std::__u::raw_string_view,
    pub ostream: ::cc_std::std::__u::raw_string_view,
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
// error: function `operator<<` could not be bound
//   Unsupported return type: template instantiation is not yet supported
//   Unsupported parameter #0 (out): template instantiation is not yet supported

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
// error: function `AbslStringify` could not be bound
//   Function templates are not yet supported

// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=82
// error: class `Templated` could not be bound
//   Class templates are not yet supported

// Generated from: rs_bindings_from_cc/test/display/displayables.h;l=86
// error: function `AbslStringify` could not be bound
//   Function templates are not yet supported

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
impl From<::cc_std::std::__u::raw_string_view> for TemplatedStringView {
    #[inline(always)]
    fn from(args: ::cc_std::std::__u::raw_string_view) -> Self {
        let mut v = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN19TemplatedStringViewC1ENSt3__u17basic_string_viewIcNS0_11char_traitsIcEEEE(&raw mut tmp as*mut _,&mut v);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::cc_std::std::__u::raw_string_view> for TemplatedStringView {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::cc_std::std::__u::raw_string_view) -> Self::CtorType {
        <Self as From<::cc_std::std::__u::raw_string_view>>::from(args)
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
    pub cc_value: ::cc_std::std::__u::raw_string_view,
    pub rust_value: ::cc_std::std::__u::raw_string_view,
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
// error: struct `std::integral_constant<bool, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// error: struct `std::integral_constant<bool, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator.h;l=62
// error: class `std::allocator<char32_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator.h;l=62
// error: class `std::allocator<char16_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator.h;l=62
// error: class `std::allocator<char>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator.h;l=62
// error: class `std::allocator<wchar_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory_resource/polymorphic_allocator.h;l=45
// error: class `std::pmr::polymorphic_allocator<char32_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory_resource/polymorphic_allocator.h;l=45
// error: class `std::pmr::polymorphic_allocator<char16_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory_resource/polymorphic_allocator.h;l=45
// error: class `std::pmr::polymorphic_allocator<char>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string;l=734
// error: class `std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string;l=734
// error: class `std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string;l=734
// error: class `std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string;l=734
// error: class `std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// error: class `std::basic_string` could not be bound
//   Unsupported type 'char8_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// error: class `std::basic_string` could not be bound
//   Unsupported type 'char8_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// error: class `std::basic_string` could not be bound
//   Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/string.h;l=42
// error: class `std::basic_string` could not be bound
//   Unsupported type 'wchar_t': Unsupported builtin type

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// error: struct `std::__type_identity<std::chrono::duration<int, std::ratio<2629746L, 1L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// error: struct `std::__type_identity<std::chrono::duration<int, std::ratio<31556952L, 1L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// error: struct `std::__type_identity<std::chrono::duration<int, std::ratio<604800L, 1L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// error: struct `std::__type_identity<std::chrono::duration<int, std::ratio<86400L, 1L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// error: struct `std::__type_identity<std::chrono::duration<long, std::ratio<1L, 1000000000000000L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// error: struct `std::__type_identity<std::chrono::duration<long, std::ratio<1L, 1L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// error: struct `std::__type_identity<std::chrono::duration<long, std::ratio<3600L, 1L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// error: struct `std::__type_identity<std::chrono::duration<long, std::ratio<60L, 1L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// error: struct `std::__type_identity<std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// error: struct `std::__type_identity<std::chrono::duration<long long, std::ratio<1L, 1000000000L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// error: struct `std::__type_identity<std::chrono::duration<long long, std::ratio<1L, 1000000L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// error: struct `std::__type_identity<std::chrono::duration<long long, std::ratio<1L, 1000L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/type_identity.h;l=21
// error: struct `std::__type_identity<std::chrono::duration<long long, std::ratio<1L, 1L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// error: struct `std::iterator_traits<char32_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// error: struct `std::iterator_traits<char16_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// error: struct `std::iterator_traits<const char32_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// error: struct `std::iterator_traits<const char16_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// error: struct `std::iterator_traits<const char *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/iterator_traits.h;l=340
// error: struct `std::iterator_traits<char *>` could not be bound
//   template instantiation is not yet supported

// error: class `std::basic_filebuf<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_filebuf<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ifstream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ifstream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ofstream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ofstream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_fstream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_fstream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// Generated from: nowhere/llvm/src/libcxx/include/istream;l=1177
// error: class `std::basic_iostream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/sstream;l=345
// error: class `std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/sstream;l=867
// error: class `std::basic_istringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/sstream;l=1005
// error: class `std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/sstream;l=1145
// error: class `std::basic_stringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__ios/fpos.h;l=23
// error: class `std::fpos<__mbstate_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/pointer_traits.h;l=110
// error: struct `std::pointer_traits<char32_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/pointer_traits.h;l=110
// error: struct `std::pointer_traits<char16_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/pointer_traits.h;l=110
// error: struct `std::pointer_traits<char *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/initializer_list;l=62
// error: class `std::initializer_list<char32_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/initializer_list;l=62
// error: class `std::initializer_list<char16_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/initializer_list;l=62
// error: class `std::initializer_list<char>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=81
// error: struct `std::char_traits<char>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=180
// error: struct `std::__char_traits_base<char32_t, unsigned int, 4294967295U>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=180
// error: struct `std::__char_traits_base<char16_t, unsigned short, (unsigned short)65535>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=180
// error: struct `std::__char_traits_base<char8_t, unsigned int, 4294967295U>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=180
// error: struct `std::__char_traits_base<wchar_t, unsigned int, 4294967295U>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=247
// error: struct `std::char_traits<wchar_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=270
// error: struct `std::char_traits<char8_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=289
// error: struct `std::char_traits<char16_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__string/char_traits.h;l=324
// error: struct `std::char_traits<char32_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<std::__wrap_iter<char32_t *>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<std::__wrap_iter<char16_t *>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<std::__wrap_iter<const char32_t *>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<std::__wrap_iter<const char16_t *>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<std::__wrap_iter<const char *>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<std::__wrap_iter<char *>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<const char32_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<const char16_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<const char8_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<const void *const *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<const char *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/reverse_iterator.h;l=41
// error: class `std::reverse_iterator<const wchar_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// error: class `std::__wrap_iter<char32_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// error: class `std::__wrap_iter<char16_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// error: class `std::__wrap_iter<const char32_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// error: class `std::__wrap_iter<const char16_t *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// error: class `std::__wrap_iter<const char *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/wrap_iter.h;l=35
// error: class `std::__wrap_iter<char *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=97
// error: struct `std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char32_t>, char32_t *, void>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=97
// error: struct `std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char16_t>, char16_t *, void>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=97
// error: struct `std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char>, char *, void>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=233
// error: struct `std::__allocator_traits_base<std::pmr::polymorphic_allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=233
// error: struct `std::__allocator_traits_base<std::pmr::polymorphic_allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=233
// error: struct `std::__allocator_traits_base<std::pmr::polymorphic_allocator<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=265
// error: struct `std::__allocator_traits_base<std::allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=265
// error: struct `std::__allocator_traits_base<std::allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=265
// error: struct `std::__allocator_traits_base<std::allocator<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// error: struct `std::allocator_traits<std::pmr::polymorphic_allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// error: struct `std::allocator_traits<std::pmr::polymorphic_allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// error: struct `std::allocator_traits<std::pmr::polymorphic_allocator<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// error: struct `std::allocator_traits<std::allocator<char32_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// error: struct `std::allocator_traits<std::allocator<char16_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocator_traits.h;l=297
// error: struct `std::allocator_traits<std::allocator<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocate_at_least.h;l=23
// error: struct `std::__allocation_result<char32_t *, unsigned long>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocate_at_least.h;l=23
// error: struct `std::__allocation_result<char16_t *, unsigned long>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__memory/allocate_at_least.h;l=23
// error: struct `std::__allocation_result<char *, unsigned long>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
// error: class `std::basic_string_view<char32_t, std::char_traits<char32_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
// error: class `std::basic_string_view<char16_t, std::char_traits<char16_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/string_view;l=287
// error: class `std::basic_string_view<char8_t, std::char_traits<char8_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<char32_t, std::__cxx_atomic_base_impl<char32_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<char16_t, std::__cxx_atomic_base_impl<char16_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<char8_t, std::__cxx_atomic_base_impl<char8_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<absl::base_internal::PerThreadSynch::State, std::__cxx_atomic_base_impl<absl::base_internal::PerThreadSynch::State>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<absl::base_internal::ThreadIdentity::WaitState, std::__cxx_atomic_base_impl<absl::base_internal::ThreadIdentity::WaitState>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<base::scheduling::Schedulable *, std::__cxx_atomic_base_impl<base::scheduling::Schedulable *>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<signed char, std::__cxx_atomic_base_impl<signed char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<bool, std::__cxx_atomic_base_impl<bool>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<char, std::__cxx_atomic_base_impl<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<unsigned char, std::__cxx_atomic_base_impl<unsigned char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<int, std::__cxx_atomic_base_impl<int>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<unsigned int, std::__cxx_atomic_base_impl<unsigned int>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<long, std::__cxx_atomic_base_impl<long>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<unsigned long, std::__cxx_atomic_base_impl<unsigned long>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<short, std::__cxx_atomic_base_impl<short>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<unsigned short, std::__cxx_atomic_base_impl<unsigned short>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<wchar_t, std::__cxx_atomic_base_impl<wchar_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<long long, std::__cxx_atomic_base_impl<long long>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/support.h;l=114
// error: struct `std::__cxx_atomic_impl<unsigned long long, std::__cxx_atomic_base_impl<unsigned long long>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<1000000000000000000L, 1L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<1000000000000000L, 1L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<1000000000000L, 1L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<1000000000L, 1L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<1000000L, 1L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<1000L, 1L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<100L, 1L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<10L, 1L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<1L, 1000000000000000000L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<1L, 1000000000000000L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<1L, 1000000000000L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<1L, 1000000000L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<1L, 1000000L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<1L, 1000L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<1L, 100L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<1L, 10L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<1L, 1L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<2629746L, 1L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<31556952L, 1L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<3600L, 1L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<604800L, 1L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<60L, 1L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/ratio;l=232
// error: class `std::ratio<86400L, 1L>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// error: class `std::chrono::duration<int, std::ratio<2629746L, 1L>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// error: class `std::chrono::duration<int, std::ratio<31556952L, 1L>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// error: class `std::chrono::duration<int, std::ratio<604800L, 1L>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// error: class `std::chrono::duration<int, std::ratio<86400L, 1L>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// error: class `std::chrono::duration<long, std::ratio<1L, 1000000000000000L>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// error: class `std::chrono::duration<long, std::ratio<1L, 1L>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// error: class `std::chrono::duration<long, std::ratio<3600L, 1L>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// error: class `std::chrono::duration<long, std::ratio<60L, 1L>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// error: class `std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// error: class `std::chrono::duration<long long, std::ratio<1L, 1000000000L>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// error: class `std::chrono::duration<long long, std::ratio<1L, 1000000L>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// error: class `std::chrono::duration<long long, std::ratio<1L, 1000L>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/duration.h;l=166
// error: class `std::chrono::duration<long long, std::ratio<1L, 1L>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/time_point.h;l=36
// error: class `std::chrono::time_point<std::chrono::steady_clock, std::chrono::duration<long long, std::ratio<1L, 1000000000L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/time_point.h;l=36
// error: class `std::chrono::time_point<std::chrono::system_clock, std::chrono::duration<int, std::ratio<86400L, 1L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/time_point.h;l=36
// error: class `std::chrono::time_point<std::chrono::system_clock, std::chrono::duration<long long, std::ratio<1L, 1000000L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/time_point.h;l=36
// error: class `std::chrono::time_point<std::chrono::system_clock, std::chrono::duration<long long, std::ratio<1L, 1L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/time_point.h;l=36
// error: class `std::chrono::time_point<std::chrono::local_t, std::chrono::duration<int, std::ratio<86400L, 1L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/time_point.h;l=36
// error: class `std::chrono::time_point<std::chrono::local_t, std::chrono::duration<long long, std::ratio<1L, 1L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__chrono/time_point.h;l=36
// error: class `std::chrono::time_point<std::filesystem::_FilesystemClock, std::chrono::duration<__int128, std::ratio<1L, 1000000000L>>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<char32_t, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<char16_t, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<char8_t, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<absl::base_internal::PerThreadSynch::State, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<absl::base_internal::ThreadIdentity::WaitState, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<base::scheduling::Schedulable *, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<signed char, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<bool, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<char, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<unsigned char, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<int, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<unsigned int, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<long, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<unsigned long, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<short, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<unsigned short, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<wchar_t, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<long long, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=40
// error: struct `std::__atomic_base<unsigned long long, false>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// error: struct `std::__atomic_base<char32_t, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// error: struct `std::__atomic_base<char16_t, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// error: struct `std::__atomic_base<char8_t, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// error: struct `std::__atomic_base<signed char, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// error: struct `std::__atomic_base<char, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// error: struct `std::__atomic_base<unsigned char, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// error: struct `std::__atomic_base<int, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// error: struct `std::__atomic_base<unsigned int, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// error: struct `std::__atomic_base<long, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// error: struct `std::__atomic_base<unsigned long, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// error: struct `std::__atomic_base<short, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// error: struct `std::__atomic_base<unsigned short, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// error: struct `std::__atomic_base<wchar_t, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// error: struct `std::__atomic_base<long long, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=142
// error: struct `std::__atomic_base<unsigned long long, true>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<char32_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<char16_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<char8_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<absl::base_internal::PerThreadSynch::State>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<absl::base_internal::ThreadIdentity::WaitState>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<signed char>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<bool>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<char>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<unsigned char>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<int>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<unsigned int>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<long>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<unsigned long>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<short>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<unsigned short>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<wchar_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<long long>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=240
// error: struct `std::atomic<unsigned long long>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__atomic/atomic.h;l=267
// error: struct `std::atomic<base::scheduling::Schedulable *>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/ios.h;l=24
// error: class `std::basic_ios<char, std::char_traits<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/ios.h;l=24
// error: class `std::basic_ios<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/streambuf.h;l=22
// error: class `std::basic_streambuf<char, std::char_traits<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/streambuf.h;l=22
// error: class `std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/ostream.h;l=22
// error: class `std::basic_ostream<char, std::char_traits<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/ostream.h;l=22
// error: class `std::basic_ostream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_context.h;l=70
// error: class `std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<char>>, char>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_context.h;l=70
// error: class `std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, wchar_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/istream.h;l=22
// error: class `std::basic_istream<char, std::char_traits<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/istream.h;l=22
// error: class `std::basic_istream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/istream.h;l=25
// error: class `std::basic_iostream<char, std::char_traits<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/sstream.h;l=23
// error: class `std::basic_stringbuf<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/sstream.h;l=30
// error: class `std::basic_stringstream<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/sstream.h;l=28
// error: class `std::basic_ostringstream<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__fwd/sstream.h;l=26
// error: class `std::basic_istringstream<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<10>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<1>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<2>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<3>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<4>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<5>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<6>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<7>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<8>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__functional/bind.h;l=51
// error: struct `std::placeholders::__ph<9>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/back_insert_iterator.h;l=30
// error: class `std::back_insert_iterator<std::__format::__output_buffer<char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__iterator/back_insert_iterator.h;l=30
// error: class `std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__ranges/elements_view.h;l=393
// error: struct `std::ranges::views::__elements::__fn<0UL>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__ranges/elements_view.h;l=393
// error: struct `std::ranges::views::__elements::__fn<1UL>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_parse_context.h;l=27
// error: class `std::basic_format_parse_context<char>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_parse_context.h;l=27
// error: class `std::basic_format_parse_context<wchar_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__format/buffer.h;l=182
// error: class `std::__format::__output_buffer<char>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__format/buffer.h;l=182
// error: class `std::__format::__output_buffer<wchar_t>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_arg.h;l=210
// error: class `std::__basic_format_arg_value<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_arg.h;l=210
// error: class `std::__basic_format_arg_value<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, wchar_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_arg.h;l=280
// error: class `std::basic_format_arg<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_arg.h;l=280
// error: class `std::basic_format_arg<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, wchar_t>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_args.h;l=29
// error: class `std::basic_format_args<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<char>>, char>>` could not be bound
//   template instantiation is not yet supported

// Generated from: nowhere/llvm/src/libcxx/include/__format/format_args.h;l=29
// error: class `std::basic_format_args<std::basic_format_context<std::back_insert_iterator<std::__format::__output_buffer<wchar_t>>, wchar_t>>` could not be bound
//   template instantiation is not yet supported

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
            v: &mut ::cc_std::std::__u::raw_string_view,
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
};

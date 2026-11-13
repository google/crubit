// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub mod test_format_bridged_func_arg_by_pointer {
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=CppType const*")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_to_rust_converter=cpp_pointer_to_rust_struct")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: rust_to_cpp_converter=rust_struct_to_cpp_pointer")]
    #[repr(transparent)]
    pub struct RustTypeView {
        pub cpp_type: *const core::ffi::c_void,
    }

    #[unsafe(no_mangle)]
    pub fn test_format_bridged_func_arg_by_pointer(_: RustTypeView) {}
}

pub mod test_format_bridged_func_arg_by_value {
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=cpp_ns::CppType")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_to_rust_converter=convert_cpp_to_rust_type")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: rust_to_cpp_converter=convert_rust_to_cpp_type")]
    pub struct RustType {
        pub x: i32,
    }

    #[unsafe(no_mangle)]
    pub fn test_format_bridged_func_arg_by_value(_a: RustType) {}
}

pub mod test_format_bridged_return_type_by_pointer {
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=CppType*")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: rust_to_cpp_converter=rust_struct_to_cpp_pointer")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_to_rust_converter=cpp_pointer_to_rust_struct")]
    pub struct RustTypeOwned {
        pub cpp_type: *const core::ffi::c_void,
    }

    #[unsafe(no_mangle)]
    pub fn test_format_bridged_return_type_by_pointer() -> RustTypeOwned {
        todo!()
    }
}

pub mod test_format_bridged_return_type_by_value {
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=cpp_ns::CppType")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: rust_to_cpp_converter=rust_to_cpp_converter")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_to_rust_converter=cpp_to_rust_converter")]
    pub struct RustType {
        pub x: i32,
    }

    #[unsafe(no_mangle)]
    pub fn test_format_bridged_return_type_by_value() -> RustType {
        RustType { x: 10 }
    }
}

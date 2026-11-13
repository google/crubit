// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub mod test_format_func_arg_pointer_like {
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=const CppType*")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h")]
    #[repr(transparent)]
    pub struct RustTypeView {
        pub cpp_type: *const core::ffi::c_void,
    }

    #[unsafe(no_mangle)]
    pub fn test_format_func_arg_pointer_like(_: RustTypeView) {}
}

pub mod test_format_return_type_pointer_like {
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=CppType*")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: include_path=cpp_ns/cpp_type.h")]
    #[repr(transparent)]
    pub struct RustTypeOwned {
        pub cpp_type: *mut core::ffi::c_void,
    }

    #[unsafe(no_mangle)]
    pub fn test_format_return_type_pointer_like() -> RustTypeOwned {
        todo!()
    }
}

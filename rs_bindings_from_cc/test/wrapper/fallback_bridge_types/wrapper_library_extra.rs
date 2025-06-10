// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn get_string() -> cc_std::std::string {
    // SAFETY: reads the string pointer during its lifetime.
    unsafe { crate::CopyString(crate::GetGlobalString()) }
}

pub fn copy_string(x: &cc_std::std::string) -> cc_std::std::string {
    use forward_declare::CppCast;
    // SAFETY: reads the string pointer during its lifetime.
    unsafe { crate::CopyString(x.cpp_cast()) }
}

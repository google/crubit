// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use googletest::prelude::*;

    #[gtest]
    fn test_invoke_memcpy() {
        use void_pointers::invoke_memcpy;

        let src = [42u8; 256];
        let mut dst = [0u8; 256];
        let result = unsafe {
            invoke_memcpy(
                dst.as_mut_ptr() as *mut core::ffi::c_void,
                src.as_ptr() as *const core::ffi::c_void,
                dst.len(),
            )
        };

        assert_eq!(result, dst.as_mut_ptr() as *mut core::ffi::c_void);
        assert_eq!(src, dst);
    }
}

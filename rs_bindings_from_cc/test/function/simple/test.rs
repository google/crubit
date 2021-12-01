// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    #[test]
    fn test_return_value() {
        use simple_functions::return_value;
        assert_eq!(return_value(), 42);
    }

    #[test]
    fn test_return_pointer() {
        use simple_functions::return_pointer;
        unsafe {
            assert_eq!(*return_pointer(), 42);
        }
    }

    #[test]
    fn test_take_pointer() {
        use simple_functions::take_pointer;
        let mut i: i32 = 0;
        take_pointer(&mut i);
        assert_eq!(i, 42);
    }
}

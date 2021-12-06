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
    fn test_return_reference() {
        use simple_functions::return_reference;
        unsafe {
            assert_eq!(*return_reference(), 42);
        }
    }

    #[test]
    fn test_take_pointer() {
        use simple_functions::take_pointer;
        take_pointer(None);
        let mut i: i32 = 0;
        take_pointer(Some(&mut i));
        assert_eq!(i, 42);
    }

    #[test]
    fn test_take_reference() {
        use simple_functions::take_reference;
        let mut i: i32 = 0;
        take_reference(&mut i);
        assert_eq!(i, 42);
    }

    #[test]
    fn test_forward_pointer() {
        use simple_functions::forward_pointer;
        assert_eq!(forward_pointer(None), None);
        let i: i32 = 42;
        assert_eq!(*forward_pointer(Some(&i)).unwrap(), 42);
    }

    #[test]
    fn test_forward_reference() {
        use simple_functions::forward_reference;
        let i: i32 = 42;
        assert_eq!(*forward_reference(&i), 42);
    }
}

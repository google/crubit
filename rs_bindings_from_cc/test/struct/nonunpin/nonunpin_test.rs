// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//#[cfg(test)]
mod tests {
    use ctor::CtorNew as _;
    use nonunpin::Nonunpin;

    /// When a value is constructed in-place, it is initialized, has the correct
    /// address.
    #[test]
    fn test_ctor() {
        ctor::emplace! {
            let mut x = Nonunpin::ctor_new(42);
        }
        assert_eq!(x.value(), 42);
        assert_eq!(x.addr(), &*x as *const _ as usize);
    }

    #[test]
    fn test_copy() {
        ctor::emplace! {
            let x = Nonunpin::ctor_new(42);
            let y = ctor::copy(&*x);
        }

        assert_eq!(x.value(), 42);
        assert_eq!(y.value(), 42);

        assert_eq!(x.addr(), &*x as *const _ as usize);
        assert_eq!(y.addr(), &*y as *const _ as usize);
    }

    #[test]
    fn test_methods() {
        ctor::emplace! {
            let mut x = Nonunpin::ctor_new(42);
        }
        x.as_mut().set_value(24);
        assert_eq!(x.value(), 24);
    }
}

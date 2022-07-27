// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use ctor::{ctor, ConstRvalueReference, RvalueReference};
    use ctor::{Assign as _, CtorNew as _};
    use nonunpin::{Nonmovable, Nonunpin, NonunpinStruct, ReturnsNonmovable};
    use std::pin::Pin;

    /// When a value is constructed in-place, it is initialized, has the correct
    /// address.
    #[test]
    fn test_onearg_ctor() {
        ctor::emplace! {
            let mut x = Nonunpin::ctor_new(42);
        }
        assert_eq!(x.value(), 42);
        assert_eq!(x.addr(), &*x as *const _ as usize);
    }

    #[test]
    fn test_default_ctor() {
        ctor::emplace! {
            let mut x = Nonunpin::ctor_new(());
        }
        assert_eq!(x.value(), 0);
        assert_eq!(x.addr(), &*x as *const _ as usize);
    }

    #[test]
    fn test_move_construct() {
        ctor::emplace! {
            let mut x = Nonunpin::ctor_new(42);
            let mut y = ctor::mov(x.as_mut());
        }

        assert_eq!(x.value(), 0); // moved-from
        assert_eq!(y.value(), 42); // moved-to

        assert_eq!(x.addr(), &*x as *const _ as usize);
        assert_eq!(y.addr(), &*y as *const _ as usize);
    }

    #[test]
    fn test_move_assign() {
        ctor::emplace! {
            let mut x = Nonunpin::ctor_new(42);
            let mut y = Nonunpin::ctor_new(8);
        }

        y.as_mut().assign(ctor::mov(x.as_mut()));

        assert_eq!(x.value(), 0); // moved-from
        assert_eq!(y.value(), 42); // moved-to

        assert_eq!(x.addr(), &*x as *const _ as usize);
        assert_eq!(y.addr(), &*y as *const _ as usize);
    }

    #[test]
    fn test_copy_construct() {
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
    fn test_copy_assign() {
        ctor::emplace! {
            let x = Nonunpin::ctor_new(42);
            let mut y = Nonunpin::ctor_new(8);
        }
        y.as_mut().assign(&*x);

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

    /// Test that the struct can be returned and passed as all the reference
    /// types, and passed by value.
    #[test]
    fn test_ref() {
        ctor::emplace! {
            let mut x = Nonunpin::ctor_new(42);
        }
        {
            let x: Pin<&mut Nonunpin> = x.as_mut().AsMutRef();
            assert_eq!(nonunpin::GetValueFromMutRef(x), 42);
        }
        {
            let x: &Nonunpin = x.AsConstRef();
            assert_eq!(nonunpin::GetValueFromConstRef(x), 42);
        }
        {
            let x: RvalueReference<Nonunpin> = x.as_mut().AsRvalueRef();
            assert_eq!(nonunpin::GetValueFromRvalueRef(x), 42);
        }
        {
            let x: ConstRvalueReference<Nonunpin> = x.AsConstRvalueRef();
            assert_eq!(nonunpin::GetValueFromConstRvalueRef(x), 42);
        }
        {
            assert_eq!(nonunpin::GetValueFromValue(ctor::copy(&*x)), 42);
            assert_eq!(nonunpin::GetValueFromValue(ctor::mov(x)), 42);
        }
    }

    #[test]
    fn test_aggregate() {
        ctor::emplace! {
            let mut x = ctor!(NonunpinStruct {value: 42});
        }
        assert_eq!(x.value, 42);
        {
            // Read/write via a pin-projection.
            let mut x = x.as_mut().project_pin();
            assert_eq!(*x.value, 42);
            *x.value = 0;
            assert_eq!(*x.value, 0);
        }
        assert_eq!(x.value, 0);
    }

    #[test]
    fn test_return_by_value() {
        ctor::emplace! {
            let x = Nonunpin::ctor_new(42);
            let y = x.AsValue();
        }

        assert_eq!(x.value(), 42);
        assert_eq!(y.value(), 42);

        assert_eq!(x.addr(), &*x as *const _ as usize);
        assert_eq!(y.addr(), &*y as *const _ as usize);
    }

    #[test]
    fn test_nonmovable_ctor() {
        ctor::emplace! {
            let x = Nonmovable::ctor_new(());
        }
        assert_eq!(x.addr, &*x as *const _ as usize);
    }

    /// Thanks to C++17 prvalue semantics, we can in fact return a non-movable
    /// type by value.
    #[test]
    fn test_nonmovable_return_value() {
        ctor::emplace! {
            let x = ReturnsNonmovable();
        }
        assert_eq!(x.addr, &*x as *const _ as usize);
    }
}

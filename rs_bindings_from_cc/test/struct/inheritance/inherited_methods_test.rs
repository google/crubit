// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::gtest;
use inherited_methods::{Base, Derived};

#[gtest]
fn test_inherits_has_bindings() {
    let _base = Base::default();
    let _derived = Derived::default();
    assert!(_base.has_bindings());
    assert!(_derived.has_bindings());
}

/// Static member functions have no `this` parameter, so a parameter naming the
/// base class stays a base-class parameter even when the function is reached
/// through the derived class. This mirrors C++, where `Derived::f(Base*)` can
/// be called with a pointer to either `Base` or `Derived`.
#[gtest]
fn test_inherited_static_methods_take_base_arguments() {
    let mut base = Base::default();
    let mut derived = Derived::default();

    Derived::static_no_params();

    // Both a `Base` and an upcast `Derived` are accepted, just as in C++.
    unsafe {
        Derived::static_ptr_param(&raw mut base);
        Derived::static_two_ptr_params(&raw mut base, &raw mut base);
        assert_eq!(Derived::static_ptr_return(&raw mut base), &raw mut base);

        let derived_as_base: *mut Base = oops::Upcast::<*mut Base>::upcast(&raw mut derived);
        Derived::static_ptr_param(derived_as_base);
    }

    Derived::static_ref_param(&base);
    Derived::static_value_param(base);
}

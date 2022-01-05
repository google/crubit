// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[macro_use]
extern crate static_assertions;

#[cfg(test)]
mod tests {
    use constructors::*;

    #[test]
    fn test_user_provided_constructors() {
        assert_impl_all!(StructWithUserProvidedConstructors: Default);
        let s: StructWithUserProvidedConstructors = Default::default();
        assert_eq!(42, s.int_field);

        // TODO(lukasza): Implement and test a user-defined copy constructor / impl
        // Clone.

        // Trivial-ABI structs implement the Copy trait, even if they have user-defined
        // constructors.
        assert_impl_all!(StructWithUserProvidedConstructors: Copy);

        assert_impl_all!(StructWithUserProvidedConstructors: From<i32>);
        let i: StructWithUserProvidedConstructors = 123.into();
        assert_eq!(123, i.int_field);
    }

    #[test]
    fn test_deleted_constructors() {
        assert_not_impl_all!(StructWithDeletedConstructors: Clone);
        assert_not_impl_all!(StructWithDeletedConstructors: Copy);
        assert_not_impl_all!(StructWithDeletedConstructors: Default);
        assert_not_impl_all!(StructWithDeletedConstructors: From<i32>);
    }

    #[test]
    fn test_private_constructors() {
        assert_not_impl_all!(StructWithPrivateConstructors: Clone);
        assert_not_impl_all!(StructWithPrivateConstructors: Copy);
        assert_not_impl_all!(StructWithPrivateConstructors: Default);
        assert_not_impl_all!(StructWithPrivateConstructors: From<i32>);
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_explicitly_defaulted_constructors() {
        assert_impl_all!(StructWithExplicitlyDefaultedConstructors: Default);
        let s: StructWithExplicitlyDefaultedConstructors = Default::default();
        assert_eq!(0, s.field_with_no_initializer); // Using `MaybeUninit<T>::zeroed()`.
        assert_eq!(123, s.field_with_explicit_initializer);

        // In some scenarios the bindings generator may be able to ask Rust to
        // `#[derive(Clone)]` (e.g. when the C++ constructor has been
        // implicitly or explicitly `=default`-ed + when Rust can mimic how C++
        // would copy/clone all the fields). Therefore, the test assertions
        // below may mostly be testing/exercising how Rust derives Clone.  This
        // should be okay.
        assert_impl_all!(StructWithExplicitlyDefaultedConstructors: Clone);
        let s2 = s.clone();
        assert_eq!(0, s2.field_with_no_initializer);
        assert_eq!(123, s2.field_with_explicit_initializer);

        assert_impl_all!(StructWithExplicitlyDefaultedConstructors: Copy);
    }

    #[test]
    fn test_nontrivial_struct() {
        // Non-trivial types cannot be copied.
        assert_not_impl_all!(NonTrivialStructWithConstructors: Copy);

        // Non-trivial types cannot be constructed by-value, despite having default
        // constructor, copy constructor, and constructor taking an int.
        assert_not_impl_all!(NonTrivialStructWithConstructors: Clone);
        assert_not_impl_all!(NonTrivialStructWithConstructors: Default);
        assert_not_impl_all!(NonTrivialStructWithConstructors: From<i32>);

        // TODO(b/200067242): Support constructing non-trivially-relocatable
        // types. See also <internal link>.
    }
}

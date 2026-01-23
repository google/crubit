// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

// pathological shadowed names: shadow important modules that the macros use.
mod std {}
mod forward_declare {}

#[gtest]
fn test_conversions() {
    use ::forward_declare::CppCast as _; // test becomes too verbose otherwise.
    struct MyType;
    type MyTypeSymbol = ::forward_declare::symbol!("X");
    ::forward_declare::unsafe_define!(MyTypeSymbol, MyType);

    let mut complete = MyType;
    ::forward_declare::forward_declare!(MyTypeIncomplete = MyTypeSymbol);

    fn ptr_location(x: impl ::std::ops::Deref) -> usize {
        &*x as *const _ as *const u8 as usize
    }

    let loc = ptr_location(&complete);

    // & -> &
    {
        let incomplete_ref: &MyTypeIncomplete = (&complete).cpp_cast();
        let complete_ref: &MyType = incomplete_ref.cpp_cast();
        assert_eq!(ptr_location(incomplete_ref), ptr_location(complete_ref));
    }

    // Pin<&> <-> Pin<&>
    {
        let incomplete_pin_ref: ::std::pin::Pin<&MyTypeIncomplete> =
            ::std::pin::Pin::new(&complete).cpp_cast();
        let complete_pin_ref: ::std::pin::Pin<&MyType> = incomplete_pin_ref.cpp_cast();
        assert_eq!(ptr_location(incomplete_pin_ref), loc);
        assert_eq!(ptr_location(complete_pin_ref), loc);
        let complete_unpinned_ref: &MyType = incomplete_pin_ref.cpp_cast();
        assert_eq!(ptr_location(complete_unpinned_ref), loc);
    }

    // Pin<&mut> <-> Pin<&mut>
    {
        let incomplete_pin_mut: ::std::pin::Pin<&mut MyTypeIncomplete> =
            ::std::pin::Pin::new(&mut complete).cpp_cast();
        assert_eq!(ptr_location(&*incomplete_pin_mut), loc);
        let complete_pin_mut: ::std::pin::Pin<&mut MyType> = incomplete_pin_mut.cpp_cast();
        assert_eq!(ptr_location(complete_pin_mut), loc);
    }

    {
        // &mut -> Pin<&mut>
        let mut incomplete_pin_mut: ::std::pin::Pin<&mut MyTypeIncomplete> =
            (&mut complete).cpp_cast();
        assert_eq!(ptr_location(&*incomplete_pin_mut), loc);
        // Pin<&mut> -> &mut
        {
            let complete_unpinned_mut: &mut MyType = incomplete_pin_mut.as_mut().cpp_cast();
            assert_eq!(ptr_location(complete_unpinned_mut), loc);
        }
        // Pin<&mut> -> &
        {
            let complete_unpinned_ref: &MyType = incomplete_pin_mut.as_ref().cpp_cast();
            assert_eq!(ptr_location(complete_unpinned_ref), loc);
        }
    }

    /// Typeless location&length info for a slice.
    fn slice_location<T>(slice: &[T]) -> (usize, usize) {
        (slice.as_ptr() as usize, slice.len())
    }

    // Vec<&> <-> Vec<&>
    {
        let complete_vec: Vec<&MyType> = vec![&complete];
        let loc = slice_location(&complete_vec);

        let incomplete_vec: Vec<&MyTypeIncomplete> = complete_vec.cpp_cast();
        assert_eq!(slice_location(&incomplete_vec), loc);
        let complete_vec: Vec<&MyType> = incomplete_vec.cpp_cast();
        assert_eq!(slice_location(&complete_vec), loc);
    }

    // &[&] <-> &[&]
    {
        let complete_vec: Vec<&MyType> = vec![&complete];
        let complete_slice: &[&MyType] = complete_vec.as_slice();
        let loc = slice_location(complete_slice);

        let incomplete_slice: &[&MyTypeIncomplete] = complete_slice.cpp_cast();
        assert_eq!(slice_location(incomplete_slice), loc);
        let complete_slice: &[&MyType] = incomplete_slice.cpp_cast();
        assert_eq!(slice_location(complete_slice), loc);
    }

    // [&; N] <-> [&; N]
    {
        let complete_array: [&MyType; 2] = [&complete, &complete];
        let incomplete_array: [&MyTypeIncomplete; 2] = complete_array.cpp_cast();
        // TODO(jeanpierreda, lukasza): Avoid copying the array to a different memory location
        // (maybe by tweaking `cpp_cast()` to use `std::mem::transmute`
        // instead of `std::mem::transmute_copy` when the input and output types
        // are both `Sized`).  Once that is done, we should be able to add
        // asserts that say:
        //
        //      let loc = slice_location(&complete_array);
        //      ...
        //      assert_eq!(slice_location(&incomplete_array), loc)
        //      ...
        //      assert_eq!(slice_location(&complete_array), loc)
        let _complete_array: [&MyType; 2] = incomplete_array.cpp_cast();
    }
}

/// You should be able to call unsafe_define!() twice (on different types) in
/// the same scope.
#[gtest]
fn test_hygiene() {
    #[allow(dead_code)]
    struct MyType1;
    #[allow(dead_code)]
    type MyTypeSymbol1 = ::forward_declare::symbol!("X1");
    ::forward_declare::unsafe_define!(MyTypeSymbol1, MyType1);

    #[allow(dead_code)]
    struct MyType2;
    #[allow(dead_code)]
    type MyTypeSymbol2 = ::forward_declare::symbol!("X2");
    ::forward_declare::unsafe_define!(MyTypeSymbol2, MyType2);
}

/// Suppose a library used to define its API using an incomplete type, but
/// changed to using a complete type?
/// This test verifies that callers continue to work as normal.
///
/// (The reverse direction, fundamentally, is a lot less likely to work in
/// idiomatic code.)
#[gtest]
fn test_formerly_incomplete() {
    use ::forward_declare::CppCast as _; // test becomes too verbose otherwise.
    struct MyType;
    ::forward_declare::unsafe_define!(::forward_declare::symbol!("X"), MyType);

    mod callee {
        ::forward_declare::forward_declare!(pub MyType = ::forward_declare::symbol!("X"));
    }
    mod caller {
        ::forward_declare::forward_declare!(pub MyType = ::forward_declare::symbol!("X"));
    }
    fn takes_incomplete(_: &callee::MyType) {}
    fn takes_complete(_: &MyType) {}

    // calls which previously were converting a complete type to incomplete type are
    // now converting a complete type to itself -- not great, but still compiles
    // and works.
    let x = MyType;
    let x = &x;
    takes_incomplete(x.cpp_cast()); // before
    takes_complete(x.cpp_cast()); // after

    // Calls which previously were converting an incomplete type to an incomplete
    // will also continue to work. In fact, this is required, since different crates
    // will define different incomplete types.
    let x: &caller::MyType = x.cpp_cast();
    takes_incomplete(x.cpp_cast()); // before
    takes_complete(x.cpp_cast()); // after

    // However, if you passed an incomplete type in without calling
    // .cpp_cast(), that will no longer work.
    // takes_incomplete(x);  // COMPILATION ERROR
    // takes_complete(x);  // COMPILATION ERROR

    // Symmetrically, you can also convert complete types to incomplete if all
    // callers call .cpp_cast(), but this is much less reasonable a
    // requirement.
}

/// In C++, you can define a type as so:
/// template<typename T> struct Vector {T* x; size_t length;};
/// and it can be passed by value, even for forward-declared T.
/// How would this look in Rust?
///
/// The aim of this test is to establish that we can define such a Vector that
/// supports conversion for complete/incomplete T, though being Rust, it cannot
/// be passed by *value*, only by *reference*.
///
/// We can then add an additional trait bound on all methods, `where T :
/// Complete`. This turns out to be easier, for silly typing reasons, than
/// defining a whole new vector type for incomplete T.
#[gtest]
fn test_vector_alike() {
    use ::forward_declare::{
        forward_declare, internal::CppType, symbol, unsafe_define, Complete, CppCast,
    };
    struct MyComplete;
    unsafe_define!(symbol!("T"), MyComplete);
    forward_declare!(MyIncomplete = symbol!("T"));

    /// An equivalent to Vector from the function comment, which is a compound
    /// type that supports conversion.
    struct Vector<T: ?Sized>(*mut T, usize);
    unsafe impl<T: ?Sized> CppType for Vector<T>
    where
        T: CppType,
    {
        type Name = (Vector<()>, T::Name);
    }

    /// Methods on `Vector` that don't require complete `T`
    impl<T: ?Sized> Vector<T> {
        fn len(&self) -> usize {
            self.1
        }
    }
    /// Methods on Vector that require complete `T`.
    impl<T: Complete> Vector<T> {
        fn back(&self) -> Option<&T> {
            if self.len() == 0 {
                return None;
            }
            unimplemented!("An *actual* implementation is not important for this test");
        }
    }

    fn expects_incomplete(_: &Vector<MyIncomplete>) {}
    fn expects_complete(_: &Vector<MyComplete>) {}

    let complete = &Vector(0 as *mut MyComplete, 0);
    let incomplete: &Vector<MyIncomplete> = complete.cpp_cast();

    expects_incomplete(complete.cpp_cast());
    expects_incomplete(incomplete.cpp_cast());
    expects_complete(incomplete.cpp_cast());
    expects_complete(complete.cpp_cast());

    assert!(complete.back().is_none()); // works fine
                                        // incomplete.back() // compilation error due to unsatisfied trait bounds
                                        // (`!Complete`)
}

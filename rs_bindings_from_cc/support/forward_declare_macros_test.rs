// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// pathological shadowed names: shadow important modules that the macros use.
mod std {}
mod forward_declare {}

mod test_is_same_0 {
    type _Expected = ::forward_declare::Symbol<()>;
    fn _is_same(x: _Expected) -> ::forward_declare::symbol!("") {
        x
    }
}

mod test_is_same_1 {
    type _Expected = ::forward_declare::Symbol<(::forward_declare::C<'x'>,)>;
    fn _is_same(x: _Expected) -> ::forward_declare::symbol!("x") {
        x
    }
}

mod test_is_same_3 {
    type _Expected = ::forward_declare::Symbol<(
        ::forward_declare::C<'f'>,
        ::forward_declare::C<'o'>,
        ::forward_declare::C<'o'>,
    )>;
    fn _is_same(x: _Expected) -> ::forward_declare::symbol!("foo") {
        x
    }
}

#[test]
fn test_conversions() {
    use ::forward_declare::IncompleteCast as _; // test becomes too verbose otherwise.
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
        let incomplete_ref: &MyTypeIncomplete = (&complete).incomplete_cast();
        let complete_ref: &MyType = incomplete_ref.incomplete_cast();
        assert_eq!(ptr_location(incomplete_ref), ptr_location(complete_ref));
    }

    // Pin<&> <-> Pin<&>
    {
        let incomplete_pin_ref: ::std::pin::Pin<&MyTypeIncomplete> =
            ::std::pin::Pin::new(&complete).incomplete_cast();
        let complete_pin_ref: ::std::pin::Pin<&MyType> = incomplete_pin_ref.incomplete_cast();
        assert_eq!(ptr_location(incomplete_pin_ref), loc);
        assert_eq!(ptr_location(complete_pin_ref), loc);
        let complete_unpinned_ref: &MyType = incomplete_pin_ref.incomplete_cast();
        assert_eq!(ptr_location(complete_unpinned_ref), loc);
    }

    // &mut -> &mut
    {
        let incomplete_mut: &mut MyTypeIncomplete = (&mut complete).incomplete_cast();
        assert_eq!(ptr_location(&*incomplete_mut), loc);
        let complete_mut: &mut MyType = incomplete_mut.incomplete_cast();
        assert_eq!(ptr_location(complete_mut), loc);
    }

    // Pin<&mut> <-> Pin<&mut>
    {
        let incomplete_pin_mut: ::std::pin::Pin<&mut MyTypeIncomplete> =
            ::std::pin::Pin::new(&mut complete).incomplete_cast();
        assert_eq!(ptr_location(&*incomplete_pin_mut), loc);
        let complete_pin_mut: ::std::pin::Pin<&mut MyType> = incomplete_pin_mut.incomplete_cast();
        assert_eq!(ptr_location(complete_pin_mut), loc);
    }

    {
        // &mut -> Pin<&mut>
        let mut incomplete_pin_mut: ::std::pin::Pin<&mut MyTypeIncomplete> =
            (&mut complete).incomplete_cast();
        assert_eq!(ptr_location(&*incomplete_pin_mut), loc);
        // Pin<&mut> -> &mut
        {
            let complete_unpinned_mut: &mut MyType = incomplete_pin_mut.as_mut().incomplete_cast();
            assert_eq!(ptr_location(complete_unpinned_mut), loc);
        }
        // Pin<&mut> -> &
        {
            let complete_unpinned_ref: &MyType = incomplete_pin_mut.as_ref().incomplete_cast();
            assert_eq!(ptr_location(complete_unpinned_ref), loc);
        }
    }

    /// Typeless location&length info for a slice.
    fn slice_location<T>(slice: &[T]) -> (usize, usize) {
        (slice.as_ptr() as usize, slice.len())
    }

    {
        let complete_vec: Vec<&MyType> = vec![&complete];
        let loc = slice_location(&complete_vec);

        let incomplete_vec: Vec<&MyTypeIncomplete> = complete_vec.incomplete_cast();
        assert_eq!(slice_location(&incomplete_vec), loc);
        let complete_vec: Vec<&MyType> = incomplete_vec.incomplete_cast();
        assert_eq!(slice_location(&complete_vec), loc);
    }
}

/// You should be able to call unsafe_define!() twice (on different types) in
/// the same scope.
#[test]
fn test_hygiene() {
    struct MyType1;
    type MyTypeSymbol1 = ::forward_declare::symbol!("X1");
    ::forward_declare::unsafe_define!(MyTypeSymbol1, MyType1);

    struct MyType2;
    type MyTypeSymbol2 = ::forward_declare::symbol!("X2");
    ::forward_declare::unsafe_define!(MyTypeSymbol2, MyType2);
}

/// Suppose a library used to define its API using an incomplete type, but
/// changed to using a complete type?
/// This test verifies that callers continue to work as normal.
///
/// (The reverse direction, fundamentally, is a lot less likely to work in
/// idiomatic code.)
#[test]
fn test_formerly_incomplete() {
    use ::forward_declare::IncompleteCast as _; // test becomes too verbose otherwise.
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
    takes_incomplete(x.incomplete_cast()); // before
    takes_complete(x.incomplete_cast()); // after

    // calls which previously were, in a fit of paranoia, converting an incomplete
    // type to itself will also continue to work.
    // We wish to show this because this demonstrates that the fit of paranoia is,
    // in fact, a good practice when dealing with incomplete types: do not
    // assume the things you call will continue to use incomplete types forever!
    // TODO(jeanpierreda): *require* that callers use .incomplete_cast(), e.g. by making
    // these two different Incomplete types (e.g. Incomplete<SymbolName,
    // CurrentCrate>)
    let x: &caller::MyType = x.incomplete_cast();
    takes_incomplete(x.incomplete_cast()); // before
    takes_complete(x.incomplete_cast()); // after

    // However, if you passed an incomplete type in without calling
    // .incomplete_cast(), that will no longer work.
    // takes_incomplete(x);  // COMPILATION ERROR
    // takes_complete(x);  // COMPILATION ERROR

    // Symmetrically, you can also convert complete types to incomplete if all
    // callers call .incomplete_cast(), but this is much less reasonable a
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
#[test]
fn test_vector_alike() {
    use ::forward_declare::{
        forward_declare, symbol, unsafe_define, Complete, IncompleteCast, IncompleteTransmute,
    };
    struct MyComplete;
    unsafe_define!(symbol!("T"), MyComplete);
    forward_declare!(MyIncomplete = symbol!("T"));

    /// An equivalent to Vector from the function comment, which is a compound
    /// type that supports conversion.
    struct Vector<T: ?Sized>(*mut T, usize);
    unsafe impl<T: ?Sized, U: ?Sized> IncompleteTransmute<Vector<U>> for Vector<T> where
        T: IncompleteTransmute<U>
    {
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
    let incomplete: &Vector<MyIncomplete> = complete.incomplete_cast();

    expects_incomplete(complete.incomplete_cast());
    expects_incomplete(incomplete.incomplete_cast());
    expects_complete(incomplete.incomplete_cast());
    expects_complete(complete.incomplete_cast());

    assert!(complete.back().is_none()); // works fine
    // incomplete.back() // compilation error due to unsatisfied trait bounds
    // (`!Complete`)
}

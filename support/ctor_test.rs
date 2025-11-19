// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(negative_impls)]

//! Unit tests for the `ctor` crate.

use ctor::{
    copy, ctor, emplace, mov, try_emplace, Assign, Ctor, CtorNew, Emplace, FnCtor,
    ManuallyDropCtor, Reconstruct, RecursivelyPinned, RvalueReference, Slot, UnreachableCtor,
};
use googletest::gtest;
use std::cell::RefCell;
use std::convert::Infallible;
use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

/// Only really need one test for the new super-let syntax, as it uses the same
/// building blocks as the old syntax.
#[gtest]
fn test_emplace_super_let() {
    let x = emplace!(u32::ctor_new(()));
    assert_eq!(*x, 0);
}

#[gtest]
fn test_copy_rust_type() {
    let x: u32 = 42;
    let mut y = Box::emplace(copy(&x));
    assert_eq!(x, 42);
    assert_eq!(*y, 42);

    // Can also mutate:
    let x = 50;
    y.as_mut().assign(&x);
    assert_eq!(x, 50);
    assert_eq!(*y, 50);

    // Can also mutate:
    let x = 100;
    y.as_mut().assign(copy(&x));
    assert_eq!(x, 100);
    assert_eq!(*y, 100);
}

#[gtest]
fn test_copy_smart_ptr() {
    let x = Box::new(42_u32);
    let y = emplace!(copy(x));
    // x is no longer in scope, as it was moved into the Copy.
    assert_eq!(*y, 42);
}

/// Tests that the assigned variables have the correct type.
#[gtest]
fn test_emplace_type() {
    let x: u32 = 42;
    let foo = emplace!(copy(&x));
    let _foo: Pin<&mut u32> = foo; // type checks OK
}

#[gtest]
fn test_emplace_macro() {
    let x: u32 = 42;
    let foo = emplace!(copy(&x));
    assert_eq!(*foo, 42);
}

#[gtest]
fn test_try_emplace_returns_ok() {
    struct OkCtor;
    impl !Unpin for OkCtor {}
    // SAFETY: unconditionally initializes dest.
    unsafe impl Ctor for OkCtor {
        type Output = i32;
        type Error = String;
        unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Self::Error> {
            unsafe { dest.write(42) };
            Ok(())
        }
    }

    let x: Result<Pin<&mut i32>, String> = try_emplace!(OkCtor);
    assert_eq!(x, Ok(Pin::new(&mut 42)));
}

#[gtest]
fn test_try_emplace_returns_err() {
    struct ErrCtor;
    impl !Unpin for ErrCtor {}
    // SAFETY: unconditionally returns error.
    unsafe impl Ctor for ErrCtor {
        type Output = i32;
        type Error = String;
        unsafe fn ctor(self, _: *mut Self::Output) -> Result<(), Self::Error> {
            Err("fooey".to_string())
        }
    }

    let x: Result<Pin<&mut i32>, String> = try_emplace!(ErrCtor);
    assert_eq!(x, Err("fooey".to_string()));
}

#[gtest]
fn test_emplace_trait() {
    let x: Pin<Box<u32>> = Box::emplace(u32::ctor_new(()));
    assert_eq!(*x, 0);

    let x: Pin<Rc<u32>> = Rc::emplace(u32::ctor_new(()));
    assert_eq!(*x, 0);

    let x: Pin<Arc<u32>> = Arc::emplace(u32::ctor_new(()));
    assert_eq!(*x, 0);
}

#[gtest]
fn test_emplace_mut() {
    let x: u32 = 42;
    let mut foo = emplace!(copy(&x));
    assert_eq!(*foo, 42);
    *foo = 0;
    assert_eq!(*foo, 0);
}

#[gtest]
fn test_emplace_multi() {
    let x: u32 = 42;
    let foo = emplace!(copy(&x));
    let bar = emplace!(copy(&*foo));
    assert_eq!(*foo, 42);
    assert_eq!(*bar, 42);
}

#[gtest]
fn test_emplace_type_syntax() {
    let x: u32 = 42;
    let mut foo: Pin<&mut u32> = emplace!(copy(&x));
    let bar: Pin<&mut u32> = emplace!(copy(&x));
    assert_eq!(*foo, 42);
    *foo = 0;
    assert_eq!(*foo, 0);
    assert_eq!(*bar, 42);
}

#[gtest]
fn test_ctor_macro() {
    struct MyStruct {
        x: u32,
        y: u32,
    }
    unsafe impl RecursivelyPinned for MyStruct {
        type CtorInitializedFields = Self;
    }
    let my_struct = emplace!(ctor!(MyStruct { x: 4, y: copy(&2) }));
    assert_eq!(my_struct.x, 4);
    assert_eq!(my_struct.y, 2);

    // test that trailing commas compile:
    #[rustfmt::skip]
    let _ = ctor!(MyStruct {
        x: 0,
        y: 0,
    });
}

#[gtest]
fn test_ctor_macro_generic_struct() {
    struct MyStruct<T> {
        x: T,
        y: T,
    }

    unsafe impl<T> RecursivelyPinned for MyStruct<T> {
        type CtorInitializedFields = Self;
    }

    let my_struct = emplace!(ctor!(MyStruct<u32> {
        x: 4,
        y: 2,
    }));

    assert_eq!(my_struct.x, 4);
    assert_eq!(my_struct.y, 2);
}

#[gtest]
fn test_ctor_macro_unit_struct() {
    struct MyStruct;
    unsafe impl RecursivelyPinned for MyStruct {
        type CtorInitializedFields = Self;
    }
    let _my_struct = emplace!(ctor!(MyStruct));
    let _my_struct = emplace!(ctor!(MyStruct {}));
}

#[gtest]
fn test_ctor_macro_named_tuple_struct() {
    struct MyStruct(u32, u32);
    unsafe impl RecursivelyPinned for MyStruct {
        type CtorInitializedFields = Self;
    }
    let my_struct = emplace!(ctor!(MyStruct { 0: 4, 1: copy(&2) }));
    assert_eq!(my_struct.0, 4);
    assert_eq!(my_struct.1, 2);
}

#[gtest]
fn test_ctor_macro_tuple_struct() {
    struct MyStruct(u32, u32);
    unsafe impl RecursivelyPinned for MyStruct {
        type CtorInitializedFields = Self;
    }
    let my_struct = emplace!(ctor!(MyStruct(4, copy(&2))));
    assert_eq!(my_struct.0, 4);
    assert_eq!(my_struct.1, 2);
}

#[gtest]
fn test_ctor_macro_manuallydrop_struct() {
    struct MyStruct {
        /// Invariant: must be initialized before drop.
        x: ManuallyDrop<Vec<u32>>,
        y: u64,
    }
    impl Drop for MyStruct {
        fn drop(&mut self) {
            // Don't leak memory, it breaks tests. :)
            // SAFETY: x is initialized before drop, below.
            unsafe { ManuallyDrop::drop(&mut self.x) }
        }
    }

    unsafe impl RecursivelyPinned for MyStruct {
        type CtorInitializedFields = Self;
    }
    let my_struct =
        emplace!(ctor!(MyStruct { x: unsafe { ManuallyDropCtor::new(vec![42]) }, y: 0 }));
    assert_eq!(&*my_struct.x, &vec![42]);
    assert_eq!(my_struct.y, 0);
}

#[gtest]
fn test_ctor_macro_union() {
    union MyUnion {
        x: ManuallyDrop<Vec<u32>>,
        y: u64,
    }
    unsafe impl RecursivelyPinned for MyUnion {
        type CtorInitializedFields = Self;
    }
    let mut my_union = emplace!(ctor!(MyUnion { x: unsafe { ManuallyDropCtor::new(vec![42]) } }));
    assert_eq!(unsafe { &*my_union.x }, &vec![42]);

    // And now write to the union.

    // First, should drop myunion.x.
    unsafe { ManuallyDrop::drop(&mut Pin::into_inner_unchecked(my_union.as_mut()).x) }
    // Second, we can overwrite.
    my_union.as_mut().reconstruct(ctor!(MyUnion { y: 24 }));
    assert_eq!(unsafe { my_union.y }, 24);
}

#[gtest]
fn test_ctor_macro_nested_struct() {
    mod nested {
        pub struct MyStruct {
            pub x: u32,
            pub y: u32,
        }
        unsafe impl crate::RecursivelyPinned for MyStruct {
            type CtorInitializedFields = Self;
        }
    }
    let my_struct = emplace!(ctor!(nested::MyStruct { x: 4, y: copy(&2) }));
    assert_eq!(my_struct.x, 4);
    assert_eq!(my_struct.y, 2);
}

#[gtest]
fn test_ctor_macro_nested_tuple_struct() {
    mod nested {
        pub struct MyStruct(pub u32, pub u32);
        unsafe impl crate::RecursivelyPinned for MyStruct {
            type CtorInitializedFields = Self;
        }
    }
    let my_struct = emplace!(ctor!(nested::MyStruct(4, copy(&2))));
    assert_eq!(my_struct.0, 4);
    assert_eq!(my_struct.1, 2);
}

/// Test that the ctor macro safety check doesn't rely on the struct not
/// implementing Drop.
#[gtest]
fn test_ctor_macro_drop_struct() {
    struct MyStruct {
        x: String,
    }
    unsafe impl RecursivelyPinned for MyStruct {
        type CtorInitializedFields = Self;
    }
    impl Drop for MyStruct {
        fn drop(&mut self) {}
    }

    let _my_struct = emplace!(ctor!(MyStruct { x: "".to_string() }));
}

/// Struct which sets a bool to true when dropped.
///
/// We use Mutex so as to allow for mutating state in a way Rust believes is
/// unwind-safe.
struct DropNotify<'a>(&'a Mutex<bool>);

impl Drop for DropNotify<'_> {
    fn drop(&mut self) {
        *self.0.lock().unwrap() = true;
    }
}

/// Ctor which constructs a DropNotify but panics before completing
/// construction.
///
/// This can be used to test both that a drop occurs (for e.g. previously
/// constructed DropNotify objects), as well as that one does not:
/// because it in fact does fully initialize the thing it is meant to
/// construct, it is not UB to then call drop on it -- even though it would
/// be UB for almost any other Ctor.
#[must_use]
struct PanicCtor<'a>(DropNotify<'a>);

// SAFETY: unconditionally initializes dest.
unsafe impl<'a> Ctor for PanicCtor<'a> {
    type Output = DropNotify<'a>;
    type Error = Infallible;
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Infallible> {
        self.0.ctor(dest)?;
        panic!();
    }
}

impl !Unpin for PanicCtor<'_> {}

/// Tests that drop() is called when the Ctor doesn't panic.
#[gtest]
fn test_emplace_drop() {
    let is_dropped = Mutex::new(false);
    {
        let _my_drop_notify = emplace!(DropNotify(&is_dropped));
    }
    assert!(*is_dropped.lock().unwrap());
}

/// Tests that when a panic occurs during emplace!{}, the uninitialized
/// value is not dropped.
#[gtest]
fn test_emplace_no_drop_on_panic() {
    let is_dropped = Mutex::new(false);
    let panic_result = std::panic::catch_unwind(|| {
        let _my_drop_notify = emplace!(PanicCtor(DropNotify(&is_dropped)));
    });
    assert!(panic_result.is_err());
    assert!(!*is_dropped.lock().unwrap());
}

/// Tests that when a panic occurs during initialization of a struct with
/// ctor!, the initialized fields are dropped, and the uninitialized
/// fields are not.
#[gtest]
fn test_ctor_macro_drop() {
    struct MyStruct<'a> {
        x: DropNotify<'a>,
        y: DropNotify<'a>,
    }
    unsafe impl RecursivelyPinned for MyStruct<'_> {
        type CtorInitializedFields = Self;
    }

    let x_dropped = Mutex::new(false);
    let y_dropped = Mutex::new(false);
    let panic_result = std::panic::catch_unwind(|| {
        let _my_struct = emplace!(ctor!(MyStruct {
            x: DropNotify(&x_dropped),
            y: PanicCtor(DropNotify(&y_dropped))
        }));
    });
    assert!(panic_result.is_err());
    assert!(*x_dropped.lock().unwrap());
    assert!(!*y_dropped.lock().unwrap());
}

#[gtest]
fn test_ctor_initialized_fields_struct() {
    pub struct CtorOnly {
        pub field: i32,
        _must_construct_using_ctor: [(); 0],
    }

    pub struct CtorOnlyPubFields {
        #[allow(unused)]
        pub field: i32,
    }

    unsafe impl RecursivelyPinned for CtorOnly {
        type CtorInitializedFields = CtorOnlyPubFields;
    }

    // Fails to compile: did not specify _must_construct_using_ctor, and cannot
    // (outside this crate) because it is private
    // let x = CtorOnly {field: 3};

    let x = emplace!(ctor!(CtorOnly { field: 3 }));
    assert_eq!(x.field, 3);
}

#[gtest]
fn ctor_initialized_fields_tuple_struct() {
    pub struct CtorOnly(pub i32, [(); 0]);
    pub struct CtorOnlyPubFields(#[allow(unused)] i32);

    unsafe impl RecursivelyPinned for CtorOnly {
        type CtorInitializedFields = CtorOnlyPubFields;
    }

    // Fails to compile: did not specify field 1, and cannot (outside this crate)
    // because it is private
    // let x = CtorOnly(3);

    let x = emplace!(ctor!(CtorOnly(3)));
    assert_eq!(x.0, 3);
}

/// logs calls to the constructors, drop.
struct DropCtorLogger<'a> {
    log: &'a RefCell<Vec<&'static str>>,
}

impl Drop for DropCtorLogger<'_> {
    fn drop(&mut self) {
        self.log.borrow_mut().push("drop");
    }
}

struct LoggingCtor<'a> {
    log: &'a RefCell<Vec<&'static str>>,
    ctor_message: &'static str,
}

// SAFETY: unconditionally initializes dest.
unsafe impl<'a> Ctor for LoggingCtor<'a> {
    type Output = DropCtorLogger<'a>;
    type Error = Infallible;
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Infallible> {
        self.log.borrow_mut().push(self.ctor_message);
        dest.write(DropCtorLogger { log: self.log });
        Ok(())
    }
}
impl !Unpin for LoggingCtor<'_> {}

impl<'a> CtorNew<&DropCtorLogger<'a>> for DropCtorLogger<'a> {
    type CtorType = LoggingCtor<'a>;
    type Error = Infallible;
    fn ctor_new(src: &DropCtorLogger<'a>) -> Self::CtorType {
        LoggingCtor { log: &src.log, ctor_message: "copy ctor" }
    }
}

impl<'a> CtorNew<RvalueReference<'_, DropCtorLogger<'a>>> for DropCtorLogger<'a> {
    type CtorType = LoggingCtor<'a>;
    type Error = Infallible;
    fn ctor_new(src: RvalueReference<'_, DropCtorLogger<'a>>) -> Self::CtorType {
        LoggingCtor { log: &src.0.log, ctor_message: "move ctor" }
    }
}

/// Tests the ctor/drop order for copy-constructible Unpin types: ctor comes
/// before drop.
#[gtest]
fn test_copy_ctor_drop_order() {
    let log = RefCell::new(vec![]);
    let log = &log;

    let notify_tester = emplace!(DropCtorLogger { log });
    let new_value = emplace!(DropCtorLogger { log });
    notify_tester.assign(&*new_value);
    assert_eq!(*log.borrow(), vec!["copy ctor", "drop"]);
}

/// Tests the ctor/drop order for move-constructible Unpin types: ctor comes
/// before drop.
#[gtest]
fn test_move_ctor_drop_order() {
    let log = RefCell::new(vec![]);
    let log = &log;

    let notify_tester = emplace!(DropCtorLogger { log });
    let new_value = emplace!(DropCtorLogger { log });
    notify_tester.assign(mov!(new_value));
    assert_eq!(*log.borrow(), vec!["move ctor", "drop"]);
}

fn takes_rvalue_reference<T>(_: RvalueReference<T>) {}
/// Non-obvious fact: you can mov() an owned reference type! Moving anything
/// also performs a rust move, but the resulting rvalue reference is
/// still valid for a temporary's lifetime.
#[gtest]
fn test_mov_box() {
    struct S;
    let x: Pin<Box<S>> = Box::pin(S);
    takes_rvalue_reference(mov!(x));
    // let _x = x; // fails to compile: x is moved!
}

#[gtest]
fn test_mov_mut_ref_to_unpin() {
    takes_rvalue_reference(mov!(&mut 1));
}

#[gtest]
fn test_mov_pinned_mut_ref() {
    let x = &mut 2;
    let pinned_mut_ref = Pin::new(x);
    takes_rvalue_reference(mov!(pinned_mut_ref));
}

#[gtest]
fn test_ctor_then() {
    let x = emplace!(40.ctor_then(|mut y| {
        *y += 2;
        Ok(())
    }));
    assert_eq!(*x, 42);
}

#[gtest]
fn test_ctor_map_err_on_ok_not_called() {
    let r: Result<Pin<&mut i32>, ()> = try_emplace!(40.ctor_map_err(|_| panic!()));
    assert_eq!(r, Ok(Pin::new(&mut 40)));
}

#[gtest]
fn test_ctor_map_err_on_err_transforms_error() {
    let res: Result<Pin<&mut i32>, String> = try_emplace!(10
        .ctor_make_fallible()
        .ctor_then(|_| Err(15))
        .ctor_map_err(|e| format!("oops: {e}")));
    assert_eq!(res, Err("oops: 15".to_string()));
}

#[gtest]
fn test_ctor_or_else_on_ok_not_called() {
    Box::emplace(10.ctor_or_else(|_| -> i32 { panic!() }));
}

#[gtest]
fn test_ctor_or_else_on_err_can_recover_value() {
    let res: Pin<Box<i32>> =
        Box::emplace(10.ctor_make_fallible().ctor_then(|_| Err(15)).ctor_or_else(|_| 22));
    assert_eq!(res, Box::pin(22));
}

/// Test that a slot can be created in a temporary.
#[gtest]
fn test_slot_temporary() {
    assert_eq!(*emplace!(Slot::new(42)).as_opt().unwrap(), 42);
}

/// Test that a slot can be created in a local.
#[gtest]
fn test_slot_local() {
    let slot = emplace!(Slot::new(42));
    assert_eq!(*slot.as_opt().unwrap(), 42);
}

/// Shows the use of Slot to implement a "slotted return value", similar to
/// moveit.
#[gtest]
fn test_slotted_return_value() {
    // TODO(jeanpierreda): delete this, use doctests when doctests work.
    fn foo(slot: Pin<&mut Slot<u32>>) -> Pin<&mut u32> {
        slot.replace(42)
    }

    let slot = emplace!(Slot::uninit());
    let rv = foo(slot);
    assert_eq!(*rv, 42);
}

/// Shows the use of Slot to implement output parameters.
#[gtest]
fn test_slotted_output_parameter() {
    // TODO(jeanpierreda): delete this, use doctests when doctests work.
    fn foo(slot: Pin<&mut Slot<u32>>) {
        slot.replace(42);
    }

    let mut slot = emplace!(Slot::uninit());
    foo(slot.as_mut());
    assert_eq!(*slot.as_opt().unwrap(), 42);
}

#[gtest]
fn test_ctor_trait_captures() {
    fn adder<'a, 'b>(
        x: &'a i32,
        y: &'b i32,
    ) -> impl Ctor<Output = i32, Error = Infallible> + use<'a, 'b> {
        FnCtor::new(|dest: *mut i32| {
            // SAFETY: dest is valid and uninitialized.
            unsafe {
                dest.write(*x + *y);
            }
        })
    }

    let sum = emplace!(adder(&40, &2));
    assert_eq!(*sum, 42);
}

/// You should be able to at least _spell_ a `Ctor` for a !Sized type.
#[gtest]
fn test_ctor_dst() {
    pub fn _foo() -> impl Ctor<Output = [i32], Error = Infallible> {
        panic!("can't actually implement this for slices");
        #[allow(unreachable_code)]
        UnreachableCtor::new()
    }
}

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(negative_impls, vec_into_raw_parts, extern_types)]
//! This crate defines facilities that imitate C++ forward declarations in Rust.
//!
//! Given this C++:
//!
//! ```c++
//! namespace foo {
//! class Foo;
//! }  // namespace foo
//!
//! void MyFunction(foo::Foo& foo) {
//!   // can pass Foo around, but cannot access members, copy it, delete it, etc.
//!   some_other_library::OtherFunction(foo);
//! }
//! ```
//!
//! We can translate it to this equivalent Rust:
//!
//! ```
//! forward_declare!(pub Foo = symbol!("foo::Foo"));
//! fn MyFunction(foo: &mut Foo) {
//!   some_other_library::OtherFunction(foo.incomplete_cast());
//! }
//! ```
//!
//! On the reverse end, to **define** a type which may be forward-declared in
//! other crates, use the `unsafe_define!` macro:
//!
//! ```
//! pub struct Foo { ... }
//! unsafe_define!(symbol!("foo::Foo"), Foo);
//! ```
//!
//! ## Background
//!
//! The C++ forward declaration of a type `class Foo;` declares the name `Foo`
//! to be a type, but does not provide details such size and contents; such type
//! declaration is said to be incomplete. This is used in C++ to speed up build
//! performance (by omitting the real definition from the dependencies) and to
//! break up dependency cycles (by having one or both sides of the cycle
//! forward-declare pieces from the other side).
//!
//! In the general case, forward declarations in existing C++ code cannot be
//! removed without substantial changes. For example, a library can
//! forward-declare a class that is only defined in the binary. So a forward
//! declaration might not even have a single defining type, but one for
//! every binary that uses the library!
//!
//! Rust has close-enough equivalents to forward-declaration for constants and
//! functions, but not for types. This crate can be used wherever forward
//! declarations are used in C++, for those cases where forward declarations
//! cannot be avoided.
//!
//! ## Supporting Forward Declaration
//!
//! For each type you wish to allow to be forward declared by others, give it a
//! globally unique name such as `"foo::YourType"`, and call
//! `unsafe_define!(symbol!("foo::YourType"), YourType)`.
//!
//! This defines `IncompleteCast<&Incomplete<Name, _>>` and vice versa, for all
//! of the following common pointer types:
//!
//!   * `& _`
//!   * `&mut _`
//!   * `Pin<& _>`
//!   * `Pin<&mut _>`
//!   * `* const _`
//!   * `* mut _`
//!
//! `Box`, `Rc`, and `Arc` are excluded, because they require knowledge of the
//! `Drop` impl, and incomplete types do not know how to drop.
//!
//! Additionally, a `Pin` can be constructed from a mut reference and vice
//! versa, if the non-pinned referent is `Unpin`.
//!
//! ## Forward Declaring
//!
//! To forward declare a type which supports forward declarations, use the
//! `forward_declare` macro.
//!
//! ```
//! forward_declare!(pub YourType = symbol!("foo::YourType"));
//! ```
//!
//! This creates a public alias `pub YourType =
//! Incomplete<symbol!("foo::YourType"), ???>;`, where `???` is some type unique
//! to this scope. In particular, this means that different calls
//! to `forward_declare` produce different (but otherwise interchangeable)
//! types.
//!
//! ## Passing around values
//!
//! Compound data types implement the `IncompleteCast` trait, which allows for
//! converting between complete and incomplete types. To convert from complete
//! to incomplete, incomplete to complete, or incomplete to incomplete when
//! crossing crate boundaries, use `.incomplete_cast()` to perform
//! the conversion.
//!
//! ```
//! forward_declare!(pub Foo = symbol!("foo::Foo"));
//!
//! fn takes_incomplete(foo: &mut Foo) {
//!   some_other_library::other_function(foo.incomplete_cast());
//! }
//!
//! fn takes_complete(foo: &mut RealFoo) {
//!   takes_incomplete(foo.incomplete_cast());
//! }
//! ```
//!
//! ## Crossing Crate Boundaries
//!
//! **Warning:** Users of incomplete types from other crates should not assume
//! that currently-incomplete types stay incomplete forever; code owners should
//! feel free to replace an incomplete type with a complete type.
//!
//! In Rust, unlike C++, the incomplete type and incomplete type are different
//! types. It is in theory a backwards-incompatible change to change from
//! incomplete to complete. Callers must try as much as possible to ease that
//! transition, since forward declarations are expected to be removed over time.
//!
//! This is partially enforced by the type system: every forward declaration of
//! a type is unique, and every crate should have its own forward declaration,
//! not reusing the forward declaration of another crate. As a result, you must
//! call `incomplete_cast()` when crossing crate boundaries, and failure to do
//! so is an error:
//!
//!
//! ```compile_fail
//! # mod other_crate {
//! #   forward_declare!(pub ForwardDeclared = symbol!("foo::Type"));
//! #   pub fn function(x: &ForwardDeclared) {panic!()}
//! # }
//!
//! forward_declare!(pub ForwardDeclared = symbol!("foo::Type"));
//! let x: &ForwardDeclared = unimplemented!();
//! other_crate::function(x);  // ERROR
//! ```
//!
//! If that example were valid, then removing a forward declaration and
//! replacing it with the real type would break the caller, requiring a change
//! to call `incomplete_cast()`. But since we always require the caller to call`
//! incomplete_cast()` *anyway*, even if it already has an incomplete
//! type, `other_crate::function` is free to remove the forward declaration
//! without breaking this caller.
//!
//! ```no_run
//! # mod other_crate {
//! #   forward_declare!(pub ForwardDeclared = symbol!("foo::Type"));
//! #   pub fn function(x: &ForwardDeclared) {panic!()}
//! # }
//! #
//! # forward_declare!(pub ForwardDeclared = symbol!("foo::Type"));
//! # let x: &ForwardDeclared = unimplemented!();
//! other_crate::function(x.incomplete_cast());  // Works whether `function` takes incomplete or complete type.
//! ```

pub use forward_declare_proc_macros::*;

/// `Symbol` type, equivalent to const &'static str values, but usable in
/// generics in stable Rust.
///
/// Every symbol is a tuple of `C`: for example, `symbol!("xy")` is
/// `Symbol((C<'x'>, C<'y'>))`
#[repr(C)]
pub struct Symbol<T>(std::marker::PhantomData<T>);

/// A character in a symbol string.
#[repr(C)]
pub struct C<const CHAR: char>;

extern "C" {
    /// Adding an `Unsized` field to your type makes it completely unsized (not
    /// just dynamically-sized).
    type Unsized;
}

/// A struct representing an "Incomplete Type" for `Name`
///
/// For any given name, there may be multiple "incomplete types" for that name
/// -- for example, `Incomplete<symbol!("T"), crate_1::T1>` is a different type
/// from `Incomplete<symbol!("T"), T2>`. This is intentional: in public
/// interfaces, an incomplete type is allowed to be changed to the complete type
/// without breaking callers.
///
/// TODO(jeanpierreda): make rust think this is ffi-safe. (PhantomData...)
#[repr(C)]
pub struct Incomplete<Name, Declarer>(std::marker::PhantomData<(Name, Declarer)>, Unsized);

// Ensure this is not `Unpin`, `Send`, or `Sync`. This is discussed somewhat in:
// https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs

/// Incomplete types can't be copied by value, because we don't know their true
/// size.
impl<Name, Declarer> !Unpin for Incomplete<Name, Declarer> {}
/// Similarly, we don't know if they are Send or Sync.
impl<Name, Declarer> !Send for Incomplete<Name, Declarer> {}
impl<Name, Declarer> !Sync for Incomplete<Name, Declarer> {}

/// Marker trait for complete types.
///
/// This is automatically implemented for anything `!Unpin`, but all other types
/// need to implement it by hand. Sorry, Rust just doesn't allow the kind of
/// negative reasoning we'd need to make this better.
///
/// In particular, this is *not* implemented for `Incomplete<...>`.
pub trait Complete {}

/// We know this to be true because `Incomplete<...>` is `!Unpin`.
///
/// Ideally we'd get a better blanket impl, but this very nearly covers
/// everything. Most `Unpin` types are going to be wrappers around C++ types,
/// which can auto-generate a `Complete` impl, and the rest are unlikely to
/// matter.
impl<T: Unpin> Complete for T {}

/// Types that implement the `CcType` trait with the same `Name` can be safely
/// transmuted between each other, because they either provide bindings for the
/// same C++ type, or point/refer to the same C++ type, or contain the same C++
/// type.
///
/// Even though this trait is public, implementations of this trait should only
/// be provided by Crubit itself.  Crubit implements `CcType` for the following
/// cases (the first 2 cases require the trait to be public):
///
/// - Structs and unions defined in the generated Rust bindings (i.e. in
///   `..._rs_api.rs` files)
/// - Structs and unions forward-declared in the generated Rust bindings (i.e.
///   in `..._rs_api.rs`    files) using the `forward_declare!` macro and
///   aliased to `Incomplete<Name, Declarer>`.
/// - Blanket `impl`s are provided for references, pointers, etc - e.g. if `T`
///   belongs to a `CcType` for a given `Name`, then `&'a T` belongs to a
///   sibling transmutability equivalence class.
#[doc(hidden)]
pub unsafe trait CcType {
    /// `Name` helps Rust type system to identify the given trasmutability
    /// equivalence class.
    ///
    /// In the first two scenarios above, the `Name` is formed using the
    /// `symbol!` macro, based on the fully-qualified name of the imported
    /// C++ type.
    ///
    /// In other scenarios, the `Name` is formed using an arbitrary convention
    /// that is sufficient to guarantee non-overlapping names (e.g. see the
    /// private `RefName` type alias below).
    type Name;
}

/// All forward declarations represented by `Incomplete<Name, ...>` form a
/// transmutability equivalence class (together with the complete definition(s)
/// - see the `unsafe_define!` macro below).
///
/// # Safety notes
///
/// We alias references to arbitrary `T` using references to `Incomplete`. This
/// is OK, because of the design of `Incomplete`:
///
/// - layout: `Incomplete` has no fields of size > 0, so it does not alias `T`
///   incompatibly.
///
/// - provenance: while in general it is not valid to access memory
///   **neighboring** that of a type    (e.g. one can't use &vec[0] to access
///   vec[1]), in this case, we are using    `feature(extern_types)`, which is a
///   DST which must grant access to the following memory or    else it would be
///   useless. (This type of access is the reason the feature exists).
unsafe impl<Name, Declarer> CcType for Incomplete<Name, Declarer> {
    type Name = Name;
}

/// `unsafe_define!(symbol!("..."), T)` claims that `T` is the unique definition
/// of the type identified by that symbol, and implements the conversions for
/// pointers to and from the corresponding `Incomplete` pointers.
///
/// Safety: if more than one type claim to be the definition, then they may be
/// transmuted into each other in safe code. The safety requirements for
/// `std::mem::transmute` apply.
// Each `rustfmt` invocation would indent the `type Name...` line further and further to the right.
#[rustfmt::skip]
#[macro_export]
macro_rules! unsafe_define {
    // TODO(jeanpierreda): support generic complete type (e.g. `symbol!("xyz") <-> Foo<T> where T : Bar`)
    ($Name:ty, $Complete:ty) => {
        unsafe impl $crate::CcType for $Complete {
            type Name = $Name;
        }
    };
}

// Below, we have ?Sized everywhere -- but we're only dealing with thin pointers
// and pointers to things containing Unsized (also thin, but thin unsized
// pointers), so it's safe to cast through usize or similar.

/// If `T` can be transmuted into `U`, then one can also transmute between
/// `*const T`, and `* const U`.
unsafe impl<T: ?Sized> CcType for *const T
where
    T: CcType,
{
    type Name = (*const (), T::Name);
}

/// If `T` can be transmuted into `U`, then one can also transmute between `*mut
/// T`, and `* mut U`.
unsafe impl<T: ?Sized> CcType for *mut T
where
    T: CcType,
{
    type Name = (*mut (), T::Name);
}

/// If `T` can be transmuted into `U`, then one can also transmute between `&'a
/// T`, `&'a U`, `Pin<&'a T>`, and `Pin<&'a U>`.  The corresponding CcType uses
/// `RefName` as the `Name` of the transmutability equivalence class.
///
/// # Safety
///
/// 1. `&T` => `&U`: Pointers to transmutable things are themselves
/// transmutable.
///
/// 2. `&T` => `Pin<&T>`: `Pin` is `repr(transparent)` and `Pin::new` is
/// safe.
///
/// 3. `Pin<&T>` => `&T`: `Pin` is `repr(transparent)` and `Pin::get_ref` is
/// safe.
///
/// 4. `Pin<&T>` => `Pin<&U>`: This combines transmutes 3, 1, and 2.
///
/// 5. `&T` => `Pin<&U>`: This combines transmutes 1 and 2.
///
/// 6. `Pin<&T>` => `&U`: This combines transmutes 4 and 3.
///
/// # Transmuting to the unpinned variant
///
/// We allow transmuting to the unpinned variant, when known to be safe.
///
/// ## Motivation
///
/// Consider what happens if a forward declaration in C++ is removed in favor of
/// the complete type, where the library containing that declaration was wrapped
/// in Rust. If the type is trivially relocatable, and we automatically generate
/// C++ bindings which do not use `Pin` except for non-trivially-relocatable
/// types, this would be a backwards incompatible change. So, we should
/// also support unpinning during the cast.
///
/// ## Limited scope of impls
///
/// Note: ideally we'd blanket impl the conversion to allow unpinning any smart
/// pointer type, and not only mut references, but this causes coherence issues,
/// because `Pin` itself is a smart pointer. I could not think of a workaround
/// that did not require specialization, other than to hardcode the set of
/// pointer types supported.
///
/// Since we're hardcoding a set of pointer types above *anyway*, we just need
/// to keep the list of unpinnable types in sync with the list of "pointers to
/// transmutable things" above.
mod ref_transmutability {
    use super::CcType;
    use std::pin::Pin;

    type RefName<'a, Name> = (&'a (), Name);

    /// Safety: See the doc comment for the `ref_transmutability` module.
    unsafe impl<'a, T: ?Sized> CcType for &'a T
    where
        T: CcType,
    {
        type Name = RefName<'a, T::Name>;
    }

    /// Safety: See the doc comment for the `ref_transmutability` module.
    unsafe impl<'a, T: ?Sized> CcType for Pin<&'a T>
    where
        T: CcType,
    {
        type Name = RefName<'a, T::Name>;
    }
}

/// If `T` can be transmuted into `U`, then one can also transmute
/// 1) between `&'a mut T` and `&'a mut U`
/// 2) between `Pin<&'a mut T>` and `Pin<&'a mut U>`
/// 3) from `&'a mut T` into `Pin<&'a mut T>` or into `Pin<&'a mut U>`
///
/// Additionally, if `T` and `U` are `Unpin`, then one can also transmute
/// 4) from `Pin<&'a mut T>` into `&'a mut T` or into `&'a mut U`
///
/// The corresponding CcType uses `MutRefName` as the `Name` of the
/// transmutability equivalence class.
///
/// # Safety
///
/// The list below is very similar to the safety notes for `RefName`, except
/// that the 3rd item below also needs to talk about the `T: Unpin` constraint.
///
/// 1. `&mut T` => `&mut U`: Pointers to transmutable things are themselves
/// transmutable.
///
/// 2. `&mut T` => `Pin<&mut T>`: `Pin` is `repr(transparent)` and `Pin::new`
///    is safe.
///
/// 3. `Pin<&mut T>` => `&mut T`: `Pin` is `repr(transparent)` and
/// `Pin::into_inner` is safe    and allowed for `T: Unpin`.
///
/// 4. `Pin<&mut T>` => `Pin<&mut U>`: This combines transmutes 3, 1, and 2.
///
/// 5. `&mut T` => `Pin<&mut U>`: This combines transmutes 1 and 2.
///
/// 6. `Pin<&mut T>` => `&mut U`: This combines transmutes 4 and 3.
///
/// # Transmuting to the unpinned variant
///
/// Transmuting to the unpinned variant is described in more details in the doc
/// comment of `RefName`.
mod mut_ref_transmutability {
    use super::CcType;
    use std::pin::Pin;

    type MutRefName<'a, Name> = (&'a mut (), Name);

    /// Safety: See the doc comment for the `mut_ref_transmutability` module.
    unsafe impl<'a, T: ?Sized> CcType for &'a mut T
    where
        T: CcType,
        T: Unpin, // See safety notes for MutRefName, item 3
    {
        type Name = MutRefName<'a, T::Name>;
    }

    /// Safety: See the doc comment for the `mut_ref_transmutability` module.
    unsafe impl<'a, T: ?Sized> CcType for Pin<&'a mut T>
    where
        T: CcType,
    {
        type Name = MutRefName<'a, T::Name>;
    }
}

/// Like `Into<T>`, but for completeness conversions.
pub trait IncompleteCast<T> {
    fn incomplete_cast(self) -> T;
}

impl<T, U> IncompleteCast<U> for T
where
    T: CcType,
    U: CcType<Name = T::Name>,
{
    fn incomplete_cast(self) -> U {
        assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<U>());
        let x = std::mem::ManuallyDrop::new(self);
        unsafe { std::mem::transmute_copy(&*x) }
    }
}

// TODO(jeanpierreda): Slices of references.

impl<'a, T: ?Sized, U: ?Sized> IncompleteCast<Vec<&'a U>> for Vec<&'a T>
where
    T: CcType,
    U: CcType<Name = T::Name>,
{
    fn incomplete_cast(self) -> Vec<&'a U> {
        let (p, len, capacity) = self.into_raw_parts();
        unsafe { Vec::from_raw_parts(p as *mut &'a U, len, capacity) }
    }
}

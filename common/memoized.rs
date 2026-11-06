// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Memoization of functions, in the style of Salsa.
//!
//! See the documentation for `query_group!` for more details.
//!
//! # Why not Salsa?
//!
//! Salsa is not compatible with rustc: types like `Ty<'tcx>`, which are not
//! `'static`, cannot be used as function parameters or return values in Salsa.
//! See https://github.com/salsa-rs/salsa/issues/424.
//!
//! Since Salsa is off the table, this module offers a very simplified
//! alternative, which implements _only_ memoization, as that is all we need.
//!
//! # Differences with Salsa
//!
//! * Supports non-`'static` types.
//! * Supports `#[return_ref]` (`-> &T`, `-> &[T]`, `-> Option<&T>`, `-> Result<&T>`, etc.) to store
//!   owned values once in the database and return references without cloning on lookup.
//! * Statically checks via [`CheapClone`] that query arguments and by-value inputs/outputs are
//!   `Copy` or `O(1)` to clone.
//! * Syntactic differences when initially setting up the trait and database.
//! * Immutable input and no support for recomputation given mutated inputs.
//! * Correspondingly, no requirement that the *return* types implement `Eq` or `Hash`.
//! * Supports `#[break_cycles_with = <default-value>]`, which generates a function that returns
//!   <default-value> if a cycle is detected.
//! * Uses function pointers in the interior of a concrete type, instead of `dyn Trait`.
//!
//! There are more substantial differences with Salsa 2022 - this was written
//! based on Salsa 0.16. We don't need to match exactly the API, but the
//! differences are kept relatively small so as to support eventually either
//! going back to Salsa, or evolving towards something closer to what
//! Salsa implements.
//!
//! # Design Choices & Alternatives
//!
//! * **`#[return_ref]` vs. `Rc<T>` / External Arena**: Annotating a query or input with
//!   `#[return_ref]` stores the owned value once in the database and projects borrowed views (`&T`,
//!   `&[T]`, `Option<&T>`, `Option<&str>`, `Result<&T, E>`, `Result<Option<&T>, E>`) tied to the
//!   `&'db self` borrow. Unlike wrapping every return type in `Rc<T>`, callers work with idiomatic
//!   slices/references and can use `?` directly on `Result<&T, E>` (cloning only the `CheapClone`
//!   error `E` on the cold error path).
//! * **`NonNull<Return>` Heap Storage (`RefMemoizationTable`)**: Safe Rust cannot return `&'a T`
//!   from a `RefCell<HashMap<Args, T>>` because the `cell::Ref` guard cannot outlive the call and
//!   table resizing moves values. Storing `Box<Return>` directly inside the `HashMap` would also be
//!   unsound under Stacked Borrows / Tree Borrows because moving a `Box` on `HashMap` resize retags
//!   its `Unique` (`noalias`) pointer and invalidates live `&'a Return` borrows held by outer query
//!   frames. Converting each computed value via `Box::into_raw` into `NonNull<Return>` relinquishes
//!   `Box`'s `Unique` tag before any shared reference is created; the table and an append-only
//!   `RefCell<Vec<NonNull<Return>>>` cleanup list move only raw pointers until `Drop`.
//! * **Type-Level `#[return_ref]` Projection via Higher-Ranked Function Pointers**: Inside
//!   `macro_rules!`, `$return_type:ty` is an opaque type token that may contain elided lifetimes
//!   (e.g., `Result<&CcSnippet<'tcx>>`). Encoding the signature as `ByRef<fn(&()) -> *mut $ty>`
//!   leverages Rust's built-in lifetime elision to bind any elided return lifetime to the higher-
//!   ranked parameter (`for<'a> fn(&'a ()) -> *mut ...`), allowing `QueryMode` and `RefProject` to
//!   map the borrowed return type to its owned storage type without proc-macros or TT munchers.
//! * **Static Enforcement via `CheapClone` + Autoref Specialization**: Because Rust coherence
//!   forbids a blanket `impl<T: Copy> CheapClone for T` alongside `impl<T> CheapClone for Rc<T>`,
//!   `__assert_cheap_clone!` uses autoref specialization on `CheapCloneCheck<T>` so that any `Copy`
//!   type (as well as `Result<T, E>` and `Option<Result<T, E>>` for `T: Copy, E: CheapClone`) is
//!   accepted automatically without boilerplate, while non-`Copy` types must implement
//!   [`CheapClone`] or use `#[return_ref]`.

/// Marker trait for types whose `Clone::clone` is `O(1)` and does not heap-allocate.
///
/// Memoized query arguments and by-value inputs/outputs must be `Copy` or `CheapClone`; annotate
/// with `#[return_ref]` (`-> &T`, `-> Option<&T>`, `-> Result<&T>`) to borrow from the database.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not known to be cheaply cloneable (`Copy` or `memoized::CheapClone`)",
    note = "annotate with `#[return_ref]` (`-> &T`, `-> Option<&T>`, or `-> Result<&T>`) so the database stores `{Self}` once, or implement `Copy`/`memoized::CheapClone` if cloning is O(1)"
)]
pub trait CheapClone: Clone {}

macro_rules! impl_cheap_clone {
    ($($ty:ty),*) => { $(impl CheapClone for $ty {})* };
    (@u $T:ident: $($ty:ty),*) => { $(impl<$T: ?Sized> CheapClone for $ty {})* };
    (@t $(($($p:ident),+)),+) => { $(impl<$($p: CheapClone),+> CheapClone for ($($p,)+) {})+ };
}
impl_cheap_clone!((), bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_cheap_clone!(f32, f64);
impl_cheap_clone!(@u T: &T, *const T, *mut T, core::ptr::NonNull<T>, core::marker::PhantomData<T>);
impl_cheap_clone!(@u T: std::rc::Rc<T>, std::rc::Weak<T>, std::sync::Arc<T>, std::sync::Weak<T>);
impl_cheap_clone!(@t (A), (A, B), (A, B, C), (A, B, C, D), (A, B, C, D, E), (A, B, C, D, E, F));
impl_cheap_clone!(@t (A, B, C, D, E, F, G), (A, B, C, D, E, F, G, H));
impl<T: CheapClone, const N: usize> CheapClone for [T; N] {}
impl<T: CheapClone> CheapClone for Option<T> {}
impl<T: CheapClone, E: CheapClone> CheapClone for Result<T, E> {}

#[doc(hidden)]
#[macro_export]
macro_rules! __mode {
    ([return_ref], $ty:ty) => { $crate::internal::ByRef<fn(&()) -> *mut $ty> };
    ([], $ty:ty) => { $crate::internal::ByValue<$ty> };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __on_cycle {
    ($fn_name:ident, $default:expr) => {
        $default
    };
    ($fn_name:ident) => {
        panic!("Cycle detected: '{}' depends on its own return value", stringify!($fn_name))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __assert_cheap_clone {
    ($($ty:ty),* $(,)?) => {
        if false {
            use $crate::internal::cheap_clone_traits::*;
            $((&&&$crate::internal::CheapCloneCheck::<$ty>::NEW).__assert_cheap_clone();)*
        }
    };
}

/// `query_group!` defines a collection of memoized functions, and the shared
/// inputs that all of those functions can access.
///
/// These functions are defined as a trait, as well as a concrete type that
/// stores the inputs and memoized values, and implements that trait.
///
/// The structure of an invocation is always as follows:
///
/// ```
/// query_group! {
///   // QueryGroup name is the name of the memoized query object, including
///   // all relevant state.
///   struct QueryGroup {
///     // First, all shared inputs are specified, in order.
///     //
///     // These are available to all of the memoized functions, by calling `db.some_input()`, etc.,
///     // in order to access the input value. In other words, they form a sort of immutable global
///     // state, useful to pass immutable configuration values to _all_ of the memoized functions.
///     //
///     // By-value inputs are immutable and must be `Copy` or `CheapClone` (or annotated with
///     // `#[return_ref]` to borrow `&InputType` from the database without cloning). Because they
///     // are immutable and global, they do not need to be `Eq` or `Hash`, and they are not
///     // compared when memoizing functions.
///     //
///     // As a rule of thumb, anything which is a constant for the lifetime of an execution, but
///     // is not an actual compile time constant, should be an input.
///
///     #[input]
///     /// Doc comment goes after `#[input]`.
///     fn some_input(&self) -> InputType;
///     //...
///
///     // After all of the inputs, the actual memoized functions are specified.
///     //
///     // Memoized functions are pure functions, which take any number of (`Clone+Eq+Hash`,
///     // `Copy` or `CheapClone`) parameters, and either return a `Copy`/`CheapClone` value or are
///     // annotated with `#[return_ref]` (`-> &T`, `-> Option<&T>`, `-> Result<&T>`) so the owned
///     // value is stored once in the database.
///     //
///     // Each of these must be implemented in the _same module_ as a top-level function, with
///     // the same function signature (returning the owned value when `#[return_ref]` is used).
///     //
///     // The functions can call methods on the `&QueryGroup` self argument to access both
///     // the shared inputs, as well to call other memoized functions.
///     //
///     // Inputs to the computation which change from call to call should be specified as function
///     // parameters. Inputs which never change can be either `#[input]` functions, or actual
///     // program globals.
///     //
///     // When called on the `&QueryGroup`, the functions will be memoized, and the return
///     // value will be cached, automatically.
///     //
///     // Some functions may need to gracefully handle cycles, in which case they should be
///     // annotated with `#[break_cycles_with = <default_value>]`. This will generate a function
///     // that returns <default_value> if a cycle is detected, but will _not_ cache the result.
///
///     #[break_cycles_with = ReturnType::default()]
///     /// Doc comment goes after `#[break_cycles_with = ..]`.
///     fn may_be_cyclic(&self, arg: ArgType) -> ReturnType;
///
///     /// Doc comment goes here.
///     fn some_function(&self, arg: ArgType) -> ReturnType;
///
///     #[return_ref]
///     /// Doc comment goes after `#[return_ref]`.
///     fn some_ref_function(&self, arg: ArgType) -> &ExpensiveReturnType;
///   }
/// }
///
/// // The non-memoized implementation of the memoized functions
/// fn may_by_cyclic(db: &QueryGroup, arg: ArgType) -> ReturnType {
///   // ...
/// }
///
/// fn some_function(db: &QueryGroup, arg: ArgType) -> ReturnType {
///   // ...
/// }
/// ```
///
/// A new instance of `QueryGroup` can be created by using `QueryGroup::new(input,
/// values, here)`. It will implement the trait defined above: the `#[input]`
/// functions will return the corresponding values passed in to `QueryGroup::new`,
/// and the memoized functions will call the corresponding top-level functions.
///
/// For example, above, one could run `let db =
/// QueryGroup::new(InputType::default())`.
///
/// Now, if you call `db.some_function(...)`, it will either return a cached
/// value (if one is present), or else execute `some_function(&db)`, and cache
/// and return the result. A direct call to `some_function` (instead of
/// `db.some_function`) is not directly memoized.
///
/// Because the results are saved, it's very important that the functions are
/// pure and have no side-effects. In particular, internal mutability on the
/// inputs and arguments is best avoided.
///
/// Important notes:
///
/// * In the trait definition, all `#[input]` functions must be declared before all (non-`#[input]`)
///   memoized functions. The order of the input functions is the order of their parameters in
///   `new()`.
/// * Every (non-`#[input]`) trait method _must_ have a matching function passed to `new()`.
/// * Since all trait methods are memoized, their arguments must be `Clone`, `Eq`, and `Hash`.
///
/// # Non-`'static` types
///
/// The trait and struct can accept a lifetime parameter, but the syntax is a
/// little trickier:
///
/// ```
/// query_group! {
///   struct QueryGroup<'a> {
///     #[input]
///     fn some_input(&self) -> &'a InputType;
///     fn some_function(&self, arg: &'a ArgType) -> &'a ReturnType;
///   }
/// }
///
/// fn<'a> some_function(db: &QueryGroup<'a>, arg: &'a ArgType) -> &'a ReturnType {
///   // ...
/// }
/// ```
///
/// Note that under the hood, `QueryGroup` will bedefined as something
/// like:
///
/// ```
/// struct QueryGroup<'a> {...}
/// ```
///
/// And so you may need to specify the lifetime in some uses.
#[macro_export]
macro_rules! query_group {
    (
        $vis:vis struct $database_struct:ident $(<$($type_param:tt),*>)? {
            $(
                // TODO(jeanpierreda): Ideally would allow putting the doc-comment first,
                // but this causes parsing ambiguity.
                #[input]
                $(#[return_ref $(@$in_rr:tt)?])?
                $(#[doc = $input_doc:literal])*
                fn $input_function:ident(&self $(,)?) -> $input_type:ty;
            )*
            $(
                // TODO(jeanpierreda): Ideally would allow putting the doc-comment first,
                // but this causes parsing ambiguity.
                $(#[break_cycles_with = $break_cycles_default_value:expr])?
                $(#[return_ref $(@$fn_rr:tt)?])?
                $(#[doc = $function_doc:literal])*
                fn $function:ident(&self $(, $arg:ident : $arg_type:ty)* $(,)?) -> $return_type:ty;
            )*
        }
    ) => {
        // The database struct, which contains the lookup tables.
        $vis struct $database_struct $(<$($type_param),*>)? {
            pub __unwinding_cycles: ::core::cell::Cell<u32>,
            $($input_function: $crate::internal::Input<$crate::__mode!([$(return_ref $($in_rr)?)?], $input_type)>,)*
            $(
                $function: $crate::internal::FnAndTable<
                    fn(&Self, $($arg_type),*) -> $crate::internal::Stored<$crate::__mode!([$(return_ref $($fn_rr)?)?], $return_type)>,
                    ($($arg_type,)*),
                    $crate::__mode!([$(return_ref $($fn_rr)?)?], $return_type),
                >,
            )*
        }

        // ...and the methods.
        impl $(<$($type_param),*>)? $database_struct $(<$($type_param),*>)? {
            $(
                $(#[doc = $input_doc])*
                $vis fn $input_function(&self) -> $input_type {
                    self.$input_function.get()
                }
            )*
            $(
                $(#[doc = $function_doc])*
                $vis fn $function(&self, $($arg : $arg_type),*) -> $return_type {
                    self.$function
                        .table
                        .internal_memoized_call(
                            ($($arg,)*),
                            |($($arg,)*)| (self.$function.fn_ptr)(self, $($arg),*),
                            &self.__unwinding_cycles,
                        )
                        .unwrap_or_else(|| $crate::__on_cycle!($function $(, $break_cycles_default_value)?))
                }
            )*

            // And the new() function for initialization.
            #[allow(clippy::too_many_arguments)]
            $vis fn new(
                $($input_function: $crate::internal::Stored<$crate::__mode!([$(return_ref $($in_rr)?)?], $input_type)>,)*
                $($function: fn(&Self, $($arg_type),*) -> $crate::internal::Stored<$crate::__mode!([$(return_ref $($fn_rr)?)?], $return_type)>,)*
            ) -> Self {
                $crate::__assert_cheap_clone!(
                    $($crate::internal::CheapCheck<$crate::__mode!([$(return_ref $($in_rr)?)?], $input_type)>,)*
                    $($($arg_type,)* $crate::internal::CheapCheck<$crate::__mode!([$(return_ref $($fn_rr)?)?], $return_type)>,)*
                );
                Self {
                    __unwinding_cycles: ::core::cell::Cell::new(0),
                    $($input_function: $crate::internal::Input($input_function),)*
                    $($function: $crate::internal::FnAndTable { fn_ptr: $function, table: Default::default() },)*
                }
            }
        }
    };
}

#[doc(hidden)]
pub mod internal {
    use core::marker::PhantomData;
    use core::ptr::NonNull;
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;
    use std::hash::Hash;

    // Compile-time autoref specialization used by `__assert_cheap_clone!`.
    //
    // Rust's coherence and orphan rules prevent implementing both `impl<T: Copy> CheapClone for T`
    // and `impl<T> CheapClone for Rc<T>`, and also prevent implementing `CheapClone` directly on
    // external `Copy` types from `rustc_middle` (such as `DefId`, `Ty<'tcx>`, `CrateNum`).
    //
    // Instead, `__assert_cheap_clone!` calls
    // `(&&&CheapCloneCheck::<T>::NEW).__assert_cheap_clone()` inside a dead `if false { ... }`
    // block (so this has no runtime storage or execution cost). Because `__assert_cheap_clone(&
    // self)` takes `&self`, rustc's method resolution searches candidate receivers by dereferencing
    // one level at a time:
    //   1. `self = &&&CheapCloneCheck<T>` (`CheapCopy`: matches any `T: Copy`)
    //   2. `self = &&CheapCloneCheck<T>`  (`CheapResCopy`: matches `Result<T, E>` where `T: Copy,
    //      E: CheapClone`)
    //   3. `self = &CheapCloneCheck<T>`   (`CheapOptResCopy`: matches `Option<Result<T, E>>` where
    //      `T: Copy, E: CheapClone`)
    //   4. `self = CheapCloneCheck<T>`    (`Cheap`: matches any `T: CheapClone`, triggers the
    //      diagnostic on failure)
    pub struct CheapCloneCheck<T: ?Sized>(PhantomData<fn() -> *const T>);
    impl<T: ?Sized> CheapCloneCheck<T> {
        pub const NEW: Self = Self(PhantomData);
    }
    macro_rules! cheap_traits {
        ($($name:ident),*) => { $(pub trait $name { fn __assert_cheap_clone(&self) {} })* };
    }
    cheap_traits!(CheapCopy, CheapResCopy, CheapOptResCopy, Cheap);
    impl<T: Copy> CheapCopy for &&&CheapCloneCheck<T> {}
    impl<T: Copy, E: crate::CheapClone> CheapResCopy for &&CheapCloneCheck<Result<T, E>> {}
    impl<T: Copy, E: crate::CheapClone> CheapOptResCopy for &CheapCloneCheck<Option<Result<T, E>>> {}
    impl<T: crate::CheapClone> Cheap for CheapCloneCheck<T> {}
    pub mod cheap_clone_traits {
        pub use super::{Cheap as _, CheapCopy as _, CheapOptResCopy as _, CheapResCopy as _};
    }

    pub struct ByValue<T>(PhantomData<fn() -> T>);
    pub struct ByRef<Sig>(PhantomData<Sig>);

    pub trait QueryMode {
        type Stored;
        type CheapCheck;
        type Table<Args: Clone + Eq + Hash>: Default;
    }
    pub type Stored<M> = <M as QueryMode>::Stored;
    pub type CheapCheck<M> = <M as QueryMode>::CheapCheck;

    impl<T: Clone> QueryMode for ByValue<T> {
        type Stored = T;
        type CheapCheck = T;
        type Table<Args: Clone + Eq + Hash> = MemoizationTable<Args, T>;
    }
    pub trait RefProject<'a, Out> {
        fn project(&'a self) -> Out;
    }

    macro_rules! impl_return_ref {
        ($lt:lifetime: $(
            [$($p:ident $(: $b:ident)?),*] $out:ty => $stored:ty [$check:ty]
            |&$self:ident| $proj:expr;
        )*) => {$(
            impl<$($p),*> QueryMode for ByRef<for<$lt> fn(&$lt ()) -> *mut $out> {
                type Stored = $stored;
                type CheapCheck = $check;
                type Table<Args: Clone + Eq + Hash> = RefMemoizationTable<Args, $stored>;
            }
            impl<$lt $(, $p $(: $b)?)*> RefProject<$lt, $out> for $stored {
                fn project(&$lt $self) -> $out {
                    $proj
                }
            }
        )*};
    }
    impl_return_ref! { 'a:
        [T] &'a T => T [()] |&self| self;
        [T] &'a [T] => Vec<T> [()] |&self| self.as_slice();
        [T] Option<&'a T> => Option<T> [()] |&self| self.as_ref();
        [] Option<&'a str> => Option<std::rc::Rc<str>> [()] |&self| self.as_deref();
        [T, E: Clone] Result<&'a T, E> => Result<T, E> [Result<(), E>]
            |&self| self.as_ref().map_err(Clone::clone);
                [T, E: Clone] Result<Option<&'a T>, E> => Result<Option<T>, E> [Result<(), E>]
            |&self| self.as_ref().map(Option::as_ref).map_err(Clone::clone);
        [E] Result<(), &'a E> => Result<(), E> [()]
            |&self| match self { Ok(()) => Ok(()), Err(e) => Err(e) };
    }

    pub struct Input<Mode: QueryMode>(pub Mode::Stored);
    impl<T: Clone> Input<ByValue<T>> {
        // Have to be very careful to clone whatever the top level value is.
        // In particular, if it's a reference `&T`, clone the _reference_ to get another `&T`,
        // not the referent to get a `T`, like if we just cloned `self.$input_function`.
        pub fn get(&self) -> T {
            Clone::clone(&self.0)
        }
    }
    impl<Sig> Input<ByRef<Sig>>
    where
        ByRef<Sig>: QueryMode,
    {
        pub fn get<'a, Out>(&'a self) -> Out
        where
            Stored<ByRef<Sig>>: RefProject<'a, Out>,
        {
            self.0.project()
        }
    }

    pub struct FnAndTable<F, Args: Clone + Eq + Hash, Mode: QueryMode> {
        pub fn_ptr: F,
        pub table: Mode::Table<Args>,
    }

    #[derive(Copy, Clone, PartialEq, Eq)]
    enum FoundCycle {
        No,
        Yes,
    }

    pub struct MemoizationTable<Args, Return>
    where
        Args: Clone + Eq + Hash,
        Return: Clone,
    {
        memoized: RefCell<HashMap<Args, Return>>,
        active: RefCell<HashMap<Args, FoundCycle>>,
    }

    // Separate `impl` instead of `#[derive(Default)]` because the `derive` would
    // needlessly require that `Args` and `Return` also implement `Default`.
    impl<Args, Return> Default for MemoizationTable<Args, Return>
    where
        Args: Clone + Eq + Hash,
        Return: Clone,
    {
        fn default() -> Self {
            Self { memoized: RefCell::new(HashMap::new()), active: RefCell::new(HashMap::new()) }
        }
    }

    impl<Args, Return> MemoizationTable<Args, Return>
    where
        Args: Clone + Eq + Hash,
        Return: Clone,
    {
        pub fn internal_memoized_call<F>(
            &self,
            args: Args,
            f: F,
            unwinding_cycles: &Cell<u32>,
        ) -> Option<Return>
        where
            F: FnOnce(Args) -> Return,
        {
            if let Some(return_value) = self.memoized.borrow().get(&args) {
                return Some(return_value.clone());
            }
            if let Some(found_cycle) = self.active.borrow_mut().get_mut(&args) {
                // We're in a cycle.
                if *found_cycle == FoundCycle::No {
                    // Only increase the count if we haven't hit this cycle before.
                    unwinding_cycles.set(unwinding_cycles.get() + 1);
                }
                *found_cycle = FoundCycle::Yes;
                return None;
            }
            self.active.borrow_mut().insert(args.clone(), FoundCycle::No);
            let return_value = f(args.clone());
            let found_cycle = self.active.borrow_mut().remove(&args).expect(
                "Internal error: currently-active cycle detection args not found. \
                      Most likely this is because of a buggy Clone/Eq/Hash impl.",
            );

            if found_cycle == FoundCycle::Yes {
                // We did hit outselves in a cycle but now we've broken out of it.
                // If we hit ourselves multiple times, we were careful to only increment this
                // count once.
                unwinding_cycles.set(unwinding_cycles.get() - 1);
            }
            if unwinding_cycles.get() == 0 {
                // No cycles, we can safely cache the result knowing that we haven't depended on
                // any cycle default values.
                self.memoized.borrow_mut().insert(args, return_value.clone());
            }
            Some(return_value)
        }
    }

    // Technical justification for `unsafe` in `RefMemoizationTable`:
    // Safe Rust cannot return `&'a Return` tied to `&'a self` from an interior-mutable memoization
    // table (`RefCell<HashMap<Args, ...>>`) without keeping a `cell::Ref` borrow guard alive (which
    // panics on recursive or subsequent `borrow_mut()` calls) or cloning on every lookup.
    // Storing raw heap pointers in an append-only `Vec<NonNull<Return>>` avoids `Box` `Unique`
    // retagging across `Vec`/`HashMap` resize (which would invalidate references), and ensures heap
    // addresses stay valid and unmutated for `'a`. The `Drop::drop` implementation ensures we
    // satisfy `Box::from_raw` obligations safely. `Drop::drop`'s exclusive `&mut self` guarantees
    // no live `&'a self` (or projected `'a`) borrows remain, and because `RefMemoizationTable<Args,
    // Return>` implements `Drop` without `#[may_dangle]`, `Args` and `Return` strictly outlive
    // `self` so their `Drop` impls cannot hold `'a` borrows derived from `&'a self` either; and
    // if `Return::drop` panics while unwinding, `drain(..)` has already advanced past `ptr` and
    // safely leaks remaining allocations without double-freeing.
    pub struct RefMemoizationTable<Args: Clone + Eq + Hash, Return> {
        table: MemoizationTable<Args, NonNull<Return>>,
        allocated: RefCell<Vec<NonNull<Return>>>,
    }

    impl<Args: Clone + Eq + Hash, Return> Default for RefMemoizationTable<Args, Return> {
        fn default() -> Self {
            Self { table: MemoizationTable::default(), allocated: RefCell::new(Vec::new()) }
        }
    }

    impl<Args: Clone + Eq + Hash, Return> Drop for RefMemoizationTable<Args, Return> {
        fn drop(&mut self) {
            for ptr in self.allocated.get_mut().drain(..) {
                // SAFETY: We satisfy `Box::from_raw` obligations: every pointer in `self.allocated`
                // was created via `Box::into_raw` of a valid `Box` allocation in
                // `internal_memoized_call`. `Drop::drop`'s exclusive `&mut self` guarantees no live
                // `&'a self` (or projected `'a`) borrows remain, and because
                // `RefMemoizationTable<Args, Return>` implements `Drop` without
                // `#[may_dangle]`, `Args` and `Return` strictly outlive `self`
                // so their `Drop` impls cannot hold `'a` borrows derived from `&'a self` either. If
                // `Return::drop` panics while unwinding, `drain(..)` has already advanced past
                // `ptr` and safely leaks remaining allocations without
                // double-freeing.

                unsafe {
                    drop(Box::from_raw(ptr.as_ptr()));
                }
            }
        }
    }

    impl<Args: Clone + Eq + Hash, Return> RefMemoizationTable<Args, Return> {
        pub fn internal_memoized_call<'a, Out>(
            &'a self,
            args: Args,
            f: impl FnOnce(Args) -> Return,
            unwinding_cycles: &Cell<u32>,
        ) -> Option<Out>
        where
            Return: RefProject<'a, Out>,
        {
            let ptr = self.table.internal_memoized_call(
                args,
                |args| {
                    let ptr = NonNull::new(Box::into_raw(Box::new(f(args)))).unwrap();
                    self.allocated.borrow_mut().push(ptr);
                    ptr
                },
                unwinding_cycles,
            )?;
            // SAFETY: We satisfy `NonNull::as_ref` obligations: `ptr` was allocated via
            // `Box::into_raw` and pushed into `self.allocated`. The collection is append-only
            // until `self` drops, so the value lives at a stable, properly aligned, dereferenceable
            // heap address that stays valid and is not mutated for `'a` (except inside any
            // UnsafeCell contained in Return).
            //
            Some(unsafe { ptr.as_ref() }.project())
        }
    }
}

#[cfg(test)]
pub mod tests {
    use googletest::{assert_that, gtest, matchers::eq};
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::sync::Arc;

    #[gtest]
    fn test_basic_memoization() {
        crate::query_group! {
          pub struct Add10 {
            #[input]
            /// Tracker for how many times this function is called so we can check
            /// that memoization is indeed happening. This is just for testing;
            /// memoized functions in non-test code shouldn't have side effects,
            /// and inputs in non-test code shouldn't have internal mutability.
            fn call_counter(&self) -> Rc<Cell<i32>>;
            fn add10(&self, arg: i32) -> i32;
          }
        }
        fn add10(db: &Add10, arg: i32) -> i32 {
            db.call_counter().set(db.call_counter().get() + 1);
            arg + 10
        }
        let db = Add10::new(Rc::new(Cell::new(0)), add10);

        assert_eq!(db.add10(100), 110);
        assert_eq!(db.call_counter().get(), 1);

        assert_eq!(db.add10(100), 110);
        assert_eq!(db.call_counter().get(), 1);

        assert_eq!(db.add10(200), 210);
        assert_eq!(db.call_counter().get(), 2);
    }

    /// The raison d'etre of this module: memoization with an attached lifetime.
    ///
    /// This test is similar to test_basic_memoization, except that it accepts
    /// and returns references.
    #[gtest]
    fn test_nonstatic_memoization() {
        crate::query_group! {
          pub struct Add10<'a> {
            #[input]
            fn call_counter(&self) -> &'a Cell<i32>;
            fn add10(&self, arg: &'a i32) -> i32;
            // non-input function with 'a in return type
            fn identity(&self, arg: &'a i32) -> &'a i32;
          }
        }
        fn add10<'a>(db: &Add10<'a>, arg: &'a i32) -> i32 {
            db.call_counter().set(db.call_counter().get() + 1);
            *arg + 10
        }
        fn identity<'a>(db: &Add10<'a>, arg: &'a i32) -> &'a i32 {
            db.call_counter().set(db.call_counter().get() + 1);
            arg
        }
        let count = Cell::new(0);
        let db = Add10::new(&count, add10, identity);

        assert_eq!(db.add10(&100), 110);
        assert_eq!(count.get(), 1);

        assert_eq!(db.add10(&100), 110);
        assert_eq!(count.get(), 1);

        assert_eq!(db.add10(&200), 210);
        assert_eq!(count.get(), 2);

        assert_eq!(db.identity(&100), &100);
        assert_eq!(count.get(), 3);

        assert_eq!(db.identity(&100), &100);
        assert_eq!(count.get(), 3);
    }

    #[gtest]
    #[should_panic(expected = "Cycle detected: 'add10' depends on its own return value")]
    fn test_cycle() {
        crate::query_group! {
          pub struct Add10 {
            fn add10(&self, arg: i32) -> i32;
          }
        }
        fn add10(db: &Add10, arg: i32) -> i32 {
            db.add10(arg) // infinite recursion!
        }
        let db = Add10::new(add10);
        db.add10(1);
    }

    #[gtest]
    fn test_break_cycles_with_option() {
        crate::query_group! {
          pub struct Add10 {
            #[break_cycles_with = None]
            fn add10(&self, arg: i32) -> Option<i32>;
          }
        }
        fn add10(db: &Add10, arg: i32) -> Option<i32> {
            db.add10(arg)
        }
        let db = Add10::new(add10);
        assert_eq!(db.add10(1), None);
    }

    #[gtest]
    fn test_break_cycles_with_sentinel() {
        crate::query_group! {
          pub struct Add10 {
            #[break_cycles_with = -1]
            fn add10(&self, arg: i32) -> i32;
          }
        }
        fn add10(db: &Add10, arg: i32) -> i32 {
            db.add10(arg)
        }
        let db = Add10::new(add10);
        assert_eq!(db.add10(1), -1);
    }

    #[gtest]
    fn test_calls_in_cycle_are_not_memoized() {
        crate::query_group! {
          pub struct Table {
            #[input]
            fn logging(&self) -> Rc<RefCell<Vec<String>>>;

            #[input]
            fn records(&self) -> &'static [Record];

            #[break_cycles_with = false]
            fn is_unsafe(&self, name: &'static str) -> bool;

            fn record(&self, name: &'static str) -> Record;
          }
        }

        #[derive(Copy, Clone)]
        struct Record {
            name: &'static str,
            is_unsafe: bool,
            fields: &'static [&'static str],
        }

        // Returns whether or not a record is unsafe, checking recursively.
        fn is_unsafe(db: &Table, name: &'static str) -> bool {
            let record = db.record(name);
            let outcome =
                record.is_unsafe || record.fields.iter().any(|&field| db.is_unsafe(field));
            db.logging().borrow_mut().push(format!("is_unsafe({name}) = {outcome}"));
            outcome
        }

        // Helper function so we can refer to records by name instead of by index.
        fn record(db: &Table, name: &'static str) -> Record {
            *db.records().iter().find(|record| record.name == name).expect("Record not found")
        }

        let logging = Rc::default();

        let db = Table::new(
            Rc::clone(&logging),
            &[
                Record { name: "A", is_unsafe: false, fields: &["B", "Unsafe"] },
                Record { name: "B", is_unsafe: false, fields: &["A"] },
                Record { name: "Unsafe", is_unsafe: true, fields: &[] },
            ],
            is_unsafe,
            record,
        );
        // When checking if A is unsafe, it will first ask B, which will try to ask A
        // again, defaulting to false. So B says "I guess I'm safe", but _doesn't_
        // memoize that result. A will then see that it has Unsafe which is unsafe, so A
        // will memoize itself as unsafe. But when we go to ask B if it's unsafe now, it
        // will have correctly _not_ memoized that it's safe, and so it will ask
        // A again, which will again say "I am unsafe", and so B will correctly memoize
        // that it's unsafe.
        assert!(db.is_unsafe("A"));
        assert!(db.is_unsafe("B"));
        assert_eq!(
            logging.borrow().clone(),
            vec![
                "is_unsafe(B) = false".to_string(), // this is the cycle-default value
                "is_unsafe(Unsafe) = true".to_string(),
                "is_unsafe(A) = true".to_string(),
                "is_unsafe(B) = true".to_string(), // as we can see, the default wasn't memoized
            ]
        );
    }

    #[gtest]
    fn test_finite_recursion() {
        crate::query_group! {
          pub struct Add10 {
            #[input]
            fn call_counter(&self) -> Rc<Cell<i32>>;
            fn add10(&self, arg: i32) -> i32;
          }
        }
        fn add10(db: &Add10, arg: i32) -> i32 {
            db.call_counter().set(db.call_counter().get() + 1);
            if (arg % 10) != 0 {
                db.add10(arg - 1) + 1 // Some recursion, but not infinite!
            } else {
                arg + 10
            }
        }
        let db = Add10::new(Rc::new(Cell::new(0)), add10);

        assert_eq!(db.add10(100), 110);
        assert_eq!(db.call_counter().get(), 1);

        assert_eq!(db.add10(100), 110);
        assert_eq!(db.call_counter().get(), 1);

        assert_eq!(db.add10(205), 215);
        assert_eq!(db.call_counter().get(), 7);

        assert_eq!(db.add10(205), 215);
        assert_eq!(db.call_counter().get(), 7);
    }

    /// As an edge case (which perhaps isn't optimized well[^1]), you can even
    /// memoize a function which accepts no additional arguments, as a way
    /// of running a fixed computation at most once.
    ///
    /// [^1]: Since it has no inputs at all, we could store it as a bare `Option<ReturnType>`, but
    /// instead we use `HashMap<(), ReturnType>`. Well, good enough.
    #[gtest]
    fn test_argless() {
        crate::query_group! {
          pub struct Argless {
            #[input]
            fn call_counter(&self) -> Rc<Cell<i32>>;
            fn argless_function(&self) -> Rc<i32>;
          }
        }
        fn argless_function(db: &Argless) -> Rc<i32> {
            db.call_counter().set(db.call_counter().get() + 1);
            Rc::new(0)
        }
        let db = Argless::new(Rc::new(Cell::new(0)), argless_function);

        assert_eq!(db.call_counter().get(), 0);
        let argless_return = db.argless_function();
        assert_eq!(db.call_counter().get(), 1);
        let argless_return_2 = db.argless_function();
        assert_eq!(db.call_counter().get(), 1);
        assert!(Rc::ptr_eq(&argless_return, &argless_return_2));
    }

    // Deliberately does NOT implement `Clone`, verifying `#[return_ref]` never clones values.
    #[derive(Debug, PartialEq, Eq)]
    struct NonCloneValue<'a>(&'a str, HashMap<i32, usize>, Arc<()>);

    #[gtest]
    fn test_return_ref_memoization_reallocation_and_cycles() {
        type CustomResult<T, E = i32> = core::result::Result<T, E>;
        crate::query_group! {
            pub struct RefDb<'a> {
                #[input]
                fn calls(&self) -> Rc<Cell<i32>>;
                #[input]
                #[return_ref]
                fn cfg(&self) -> &NonCloneValue<'a>;
                #[break_cycles_with = false]
                fn check(&self, name: &'static str) -> bool;
                #[return_ref]
                /// Doc comment after `#[return_ref]`.
                fn chain(&self, depth: usize) -> &NonCloneValue<'a>;
                #[return_ref]
                fn opt(&self, key: i32) -> Option<&NonCloneValue<'a>>;
                #[return_ref]
                fn res(&self, key: i32) -> CustomResult<&NonCloneValue<'a>>;
                #[return_ref]
                fn details(&self, name: &'static str) -> &NonCloneValue<'a>;
                #[return_ref]
                fn slc(&self) -> &[i32];
                #[return_ref]
                fn opt_str(&self) -> Option<&str>;
                #[return_ref]
                fn res_opt_ref(&self, k: i32) -> Result<Option<&NonCloneValue<'a>>, i32>;
                #[return_ref]
                fn res_err_ref(&self, k: i32) -> Result<(), &NonCloneValue<'a>>;
            }
        }
        fn make<'a>(db: &RefDb<'a>, count: bool, k: i32, v: usize) -> NonCloneValue<'a> {
            db.calls().set(db.calls().get() + i32::from(count));
            NonCloneValue(db.cfg().0, HashMap::from([(k, v)]), Arc::clone(&db.cfg().2))
        }
        let (tracker, label) = (Arc::new(()), String::from("borrowed"));
        {
            let db = RefDb::new(
                Rc::new(Cell::new(0)),
                NonCloneValue(&label, HashMap::from([(0, 1)]), Arc::clone(&tracker)),
                |db, n| match n {
                    "A" => db.check("B") || db.check("U"),
                    "B" => db.details("B").1[&0] != 0,
                    _ => n == "U",
                },
                |db, d| make(db, false, 0, if d == 0 { 0 } else { db.chain(d - 1).1[&0] + d }),
                |db, k| (k > 0).then(|| make(db, true, k, k as usize)),
                |db, k| if k < 0 { Err(k) } else { Ok(make(db, true, k, k as usize)) },
                |db, n| make(db, true, 0, usize::from(n == "B" && db.check("A"))),
                |_db| vec![1, 2, 3],
                |_db| Some(Rc::from("hello")),
                |db, k| {
                    if k == 0 {
                        Err(0)
                    } else {
                        Ok((k > 0).then(|| make(db, true, k, k as usize)))
                    }
                },
                |db, k| if k > 0 { Ok(()) } else { Err(make(db, true, k, (-k) as usize)) },
            );
            // Hold `first` across >1024 nested insertions that reallocate `allocated` and
            // `memoized`.
            let first = db.chain(0);
            for i in 1..=2000 {
                db.opt(i);
            }
            assert_that!(db.chain(64).1[&0], eq(64 * 65 / 2));
            assert_that!(core::ptr::eq(first, db.chain(0)), eq(true));
            assert_that!(db.res_err_ref(10), eq(Ok(())));
            assert_that!(db.res_err_ref(-10).is_err(), eq(true));
            assert_that!(
                core::ptr::eq(db.res_err_ref(-10).unwrap_err(), db.res_err_ref(-10).unwrap_err()),
                eq(true)
            );
            assert_that!(core::ptr::eq(db.opt(8).unwrap(), db.opt(8).unwrap()), eq(true));
            assert_that!(core::ptr::eq(db.res(7).unwrap(), db.res(7).unwrap()), eq(true));
            assert_that!((db.opt(0), db.res(-5), db.calls().get()), eq((None, Err(-5), 2002)));
            assert_that!(db.check("A"), eq(true));
            let b = db.details("B");
            assert_that!((b.1[&0], core::ptr::eq(b, db.details("B"))), eq((1, true)));
            assert_that!(db.slc(), eq(&[1, 2, 3]));
            assert_that!(db.opt_str(), eq(Some("hello")));
            assert_that!(core::ptr::eq(db.opt_str().unwrap(), db.opt_str().unwrap()), eq(true));
            assert_that!(db.res_opt_ref(0), eq(Err(0)));
            assert_that!(db.res_opt_ref(-1), eq(Ok(None)));
            assert_that!(
                core::ptr::eq(
                    db.res_opt_ref(9).unwrap().unwrap(),
                    db.res_opt_ref(9).unwrap().unwrap()
                ),
                eq(true)
            );
        }
        assert_that!(Arc::strong_count(&tracker), eq(1));
    }
}

impl CheapClone for arc_anyhow::Error {}
